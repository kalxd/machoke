use super::store::{CompletionStore, MultiLineModel};
use gtk::{
	glib::{self, clone, Cast},
	prelude::{BoxExt, ContainerExt, EntryCompletionExt, EntryExt, ListBoxExt},
	Box as GtkBox, Button, Entry, EntryCompletion, Image, ListBox, ListBoxRow,
};
use std::ops::Deref;

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

		let list_box = ListBox::new();
		list_box.bind_model(
			Some(&model),
			clone!(@strong store => move |item| {
				let layout = ListBoxRow::new();

				let hlayout = GtkBox::builder()
					.spacing(10)
					.build();
				layout.add(&hlayout);

				let entry = CompletionEntry::new(store.clone());
				hlayout.pack_start(&*entry, true, true, 0);

				let remove_btn = Button::builder()
					.image(&Image::builder().icon_name("list-list").build())
					.tooltip_text("删除该列")
					.build();
				hlayout.pack_start(&remove_btn, false, false, 0);

				layout.upcast()
			}),
		);
		layout.pack_start(&list_box, false, true, 0);

		let black_entry = CompletionEntry::new(store);
		layout.pack_start(&*black_entry, false, true, 0);

		let add_btn = Button::builder()
			.image(&Image::builder().icon_name("list-add").build())
			.tooltip_text("添加新一列内容")
			.build();
		layout.pack_start(&add_btn, false, true, 0);

		Self {
			list_box,
			model,
			layout,
		}
	}
}
