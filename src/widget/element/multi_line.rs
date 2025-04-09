use super::store::CompletionStore;
use gtk::{
	glib::GString,
	prelude::{BoxExt, ButtonExt, ContainerExt, EntryCompletionExt, EntryExt, WidgetExt},
	Box as GtkBox, Button, Entry, EntryCompletion, Image,
};
use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct CompletionEntry {
	entry: Entry,
	store: CompletionStore,
}

impl CompletionEntry {
	pub fn new(store: CompletionStore) -> Self {
		let comp = EntryCompletion::builder()
			.model(&*store)
			.minimum_key_length(0)
			.popup_completion(true)
			.build();
		comp.set_text_column(0);

		let entry = Entry::new();
		entry.set_completion(Some(&comp));

		Self { entry, store }
	}

	pub fn set_text(&self, text: &str) {
		self.entry.set_text(text);
		self.store.set_text(text);
	}
}

impl Deref for CompletionEntry {
	type Target = Entry;

	fn deref(&self) -> &Self::Target {
		&self.entry
	}
}

#[derive(Clone)]
struct MultiLineRow {
	layout: GtkBox,
	entry: CompletionEntry,
	remove_btn: Button,
}

impl MultiLineRow {
	fn new(store: &CompletionStore) -> Self {
		let layout = GtkBox::builder().spacing(10).build();

		let entry = CompletionEntry::new(store.clone());
		layout.pack_start(&*entry, true, true, 0);

		let remove_btn = Button::builder()
			.image(&Image::builder().icon_name("list-remove").build())
			.tooltip_text("删除该列")
			.build();
		layout.pack_start(&remove_btn, false, false, 0);

		Self {
			entry,
			remove_btn,
			layout,
		}
	}

	fn connect_click_remove<F>(&self, f: F)
	where
		F: Fn() + 'static,
	{
		self.remove_btn.connect_clicked(move |_| f());
	}
}

#[derive(Clone)]
pub struct MultiLine {
	pub layout: GtkBox,
	store: CompletionStore,
	add_entry: CompletionEntry,
	entry_list: Rc<RefCell<Vec<MultiLineRow>>>,
}

impl MultiLine {
	pub fn new() -> Self {
		let store = CompletionStore::new();

		let entry_list = Rc::new(RefCell::new(Vec::default()));

		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.spacing(10)
			.build();

		let add_entry = CompletionEntry::new(store.clone());
		layout.pack_start(&*add_entry, false, false, 0);

		let add_btn = Button::builder()
			.image(&Image::builder().icon_name("list-add").build())
			.tooltip_text("添加一列")
			.build();
		layout.pack_end(&add_btn, false, false, 0);

		let myself = Self {
			layout,
			store,
			add_entry,
			entry_list,
		};

		add_btn.connect_clicked({
			let myself = myself.clone();
			move |_| {
				let row = myself.add_row("");
				row.entry.grab_focus();
			}
		});

		myself
	}

	fn add_row<S: AsRef<str>>(&self, text: S) -> MultiLineRow {
		let row = MultiLineRow::new(&self.store);
		self.layout.pack_start(&row.layout, false, false, 0);
		row.layout.show_all();
		row.entry.set_text(text.as_ref());

		let xs = self.entry_list.clone();

		row.connect_click_remove({
			let layout = self.layout.clone();
			let row_layout = row.layout.clone();
			let xs = self.entry_list.clone();
			move || {
				layout.remove(&row_layout);
				xs.borrow_mut()
					.retain(|item: &MultiLineRow| item.layout != row_layout);
			}
		});

		xs.borrow_mut().push(row.clone());

		row
	}

	fn clear(&self) {
		for row in self.entry_list.borrow().iter() {
			self.layout.remove(&row.layout);
		}

		self.entry_list.borrow_mut().clear();
	}

	pub fn set_text<S: AsRef<str>>(&self, text: &[S]) {
		self.clear();

		if let Some((h, xs)) = text.split_first() {
			self.add_entry.set_text(&h.as_ref());

			for x in xs {
				self.add_row(x.as_ref());
			}
		} else {
			self.add_entry.set_text("");
		}
	}

	pub fn text(&self) -> Vec<GString> {
		let entry_list = self.entry_list.borrow();
		let mut iter = vec![self.add_entry.text()];
		let line_iter = entry_list.iter().map(|row| row.entry.text());

		iter.extend(line_iter);

		iter.into_iter()
			.filter_map(|s| if s.trim().is_empty() { None } else { Some(s) })
			.collect()
	}
}
