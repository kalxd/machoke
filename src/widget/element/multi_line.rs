use super::store::CompletionStore;
use gtk::{
	prelude::{BoxExt, ButtonExt, ContainerExt, EntryCompletionExt, EntryExt, WidgetExt},
	Box as GtkBox, Button, Entry, EntryCompletion, Image,
};
use std::{cell::RefCell, ops::Deref, rc::Rc};

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

	pub fn set_store_text(&self, text: &str) {
		self.store.set_text(text);
	}
}

impl Deref for CompletionEntry {
	type Target = Entry;

	fn deref(&self) -> &Self::Target {
		&self.entry
	}
}

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

		add_btn.connect_clicked({
			let store = store.clone();
			let layout = layout.clone();
			let entry_list = entry_list.clone();
			move |_| {
				let row = MultiLineRow::new(&store);
				layout.pack_start(&row.layout, false, false, 0);
				row.layout.show_all();
				row.entry.grab_focus();

				let xs = entry_list.clone();

				row.connect_click_remove({
					let layout = layout.clone();
					let row_layout = row.layout.clone();
					let xs = entry_list.clone();
					move || {
						layout.remove(&row_layout);
						xs.borrow_mut()
							.retain(|item: &MultiLineRow| item.layout != row_layout);
					}
				});

				xs.borrow_mut().push(row);
			}
		});

		Self {
			layout,
			store,
			add_entry,
			entry_list,
		}
	}
}
