use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use gtk::glib::GString;
use gtk::prelude::{BoxExt, ButtonExt, ContainerExt, EntryExt, WidgetExt};
use gtk::{Box as GtkBox, Button, Entry, Image, Orientation};

use super::textstore::TextStore;
use crate::value::FAV_SPACING;

pub struct EntryC {
	entry: Entry,
	pub store: TextStore,
}

impl EntryC {
	pub fn new() -> Self {
		let store = TextStore::new();
		let entry = store.new_entry();
		Self { entry, store }
	}

	pub fn set_text(&self, text: &str) {
		self.entry.set_text(text);
		self.store.set_text(text);
	}
}

impl Deref for EntryC {
	type Target = Entry;

	fn deref(&self) -> &Self::Target {
		&self.entry
	}
}

struct MultiEntryRow {
	entry: Entry,
	btn: Button,
	layout: GtkBox,
}

impl MultiEntryRow {
	fn new(store: Rc<TextStore>) -> Self {
		let layout = GtkBox::new(Orientation::Horizontal, FAV_SPACING);
		let entry = store.new_entry();
		let btn = Button::builder()
			.image(&Image::builder().icon_name("list-remove").build())
			.tooltip_text("删除该列")
			.build();

		layout.pack_start(&entry, true, true, 0);
		layout.pack_end(&btn, false, false, 0);

		Self { entry, btn, layout }
	}

	fn set_text<S: AsRef<str>>(&self, text: S) {
		self.entry.set_text(text.as_ref());
	}
}

// 多行文本
pub struct MultiEntry {
	entry: Entry,
	entry_list: Rc<RefCell<Vec<MultiEntryRow>>>,
	pub store: Rc<TextStore>,
	pub layout: GtkBox,
}

impl MultiEntry {
	pub fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.build();

		let store = Rc::new(TextStore::new());
		let entry = store.new_entry();

		let entry_list = Rc::new(RefCell::new(vec![]));
		let add_btn = Button::builder()
			.image(&Image::builder().icon_name("list-add").build())
			.tooltip_text("添加一列新内容")
			.build();

		layout.pack_start(&entry, true, true, 0);
		layout.pack_end(&add_btn, false, false, 0);

		{
			// 添加新的一列
			add_btn.connect_clicked({
				let entry_list = entry_list.clone();
				let store = store.clone();
				let layout = layout.clone();
				move |_| {
					let row = MultiEntryRow::new(store.clone());
					layout.pack_start(&row.layout, false, false, 0);
					row.layout.show_all();

					row.btn.connect_clicked({
						let layout = layout.clone();
						let row_layout = row.layout.clone();
						let entry_list = entry_list.clone();
						move |_| {
							layout.remove(&row_layout);
							let mut xs = entry_list.borrow_mut();
							xs.retain(|x: &MultiEntryRow| x.layout != row_layout);
						}
					});

					let mut xs = entry_list.borrow_mut();
					xs.push(row);
				}
			});
		}

		Self {
			entry,
			entry_list,
			store,
			layout,
		}
	}

	fn reset_row(&self) {
		for row in self.entry_list.take() {
			self.layout.remove(&row.layout);
		}
	}

	fn add_row<S: AsRef<str>>(&self, text: S) {
		let layout = self.layout.clone();

		let row = MultiEntryRow::new(self.store.clone());
		row.set_text(text);
		let row_layout = row.layout.clone();
		layout.pack_start(&row.layout, false, false, 0);
		layout.show_all();

		row.btn.connect_clicked({
			let entry_list = self.entry_list.clone();
			move |_| {
				layout.remove(&row_layout);
				let mut xs = entry_list.borrow_mut();
				xs.retain(|x| x.layout != row_layout);
			}
		});

		let mut xs = self.entry_list.borrow_mut();
		xs.push(row);
	}

	pub fn set_text_list<S: AsRef<str>>(&self, xs: &[S]) {
		self.reset_row();
		if let Some((h, xs)) = xs.split_first() {
			self.entry.set_text(h.as_ref());
			xs.iter().for_each(|x| self.add_row(x));
		} else {
			self.entry.set_text("");
		}

		self.store.set_text_list(xs);
	}

	pub fn get_text_list(&self) -> Vec<GString> {
		let mut xs = vec![];
		xs.push(self.entry.text());

		let mut ys: Vec<GString> = self
			.entry_list
			.borrow()
			.iter()
			.map(|row| row.entry.text())
			.filter(|text| !text.as_str().trim().is_empty())
			.collect();

		xs.append(&mut ys);

		return xs;
	}
}
