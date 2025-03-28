use super::store::{CompletionStore, MultiLineModel, MultiLineObject};
use gtk::{
	glib::{self, clone, Cast},
	prelude::{
		BoxExt, ButtonExt, ContainerExt, EntryCompletionExt, EntryExt, ListBoxExt, ListBoxRowExt,
		WidgetExt,
	},
	Box as GtkBox, Button, Entry, EntryCompletion, Image, ListBox, ListBoxRow,
};
use std::{borrow::Borrow, ops::Deref};

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

pub struct MultiLine {
	pub layout: GtkBox,
	list_box: ListBox,
	model: MultiLineModel,
}

impl MultiLine {
	pub fn new() -> Self {
		let model = MultiLineModel::new();
		let store = CompletionStore::new();

		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.spacing(10)
			.build();

		let list_box = ListBox::builder()
			.selection_mode(gtk::SelectionMode::Single)
			.build();
		list_box.bind_model(Some(&model), {
			let list_box = list_box.clone();
			let model = model.clone();
			let store = store.clone();
			move |item| {
				let obj = item.downcast_ref::<MultiLineObject>().unwrap();
				let layout = ListBoxRow::new();

				let hlayout = GtkBox::builder().spacing(10).build();
				layout.add(&hlayout);

				let entry = CompletionEntry::new(store.clone());
				entry.set_text(&obj.text());
				hlayout.pack_start(&*entry, true, true, 0);

				let remove_btn = Button::builder()
					.image(&Image::builder().icon_name("list-list").build())
					.tooltip_text("删除该列")
					.build();
				hlayout.pack_start(&remove_btn, false, false, 0);
				remove_btn.connect_clicked(clone!(@weak list_box, @weak model => move |_| {
					let sel = list_box.selected_row();
					if let Some(sel) = dbg!(sel) {
						let index = sel.index();
						model.remove_row(index as usize);
					}
				}));

				layout.show_all();
				layout.upcast()
			}
		});
		layout.pack_start(&list_box, false, true, 0);

		let blank_entry = CompletionEntry::new(store);
		layout.pack_start(&*blank_entry, false, true, 0);

		let add_btn = Button::builder()
			.image(&Image::builder().icon_name("list-add").build())
			.tooltip_text("添加新一列内容")
			.build();
		layout.pack_start(&add_btn, false, true, 0);
		add_btn.connect_clicked({
			let model = model.clone();
			let black_entry = blank_entry.clone();
			move |_| {
				let text = black_entry.text();
				let text = text.trim();
				if !text.is_empty() {
					let blank = MultiLineObject::new(text);
					model.add_row(blank);
				}
				black_entry.set_text("");
				black_entry.grab_focus();
			}
		});

		Self {
			list_box,
			model,
			layout,
		}
	}
}
