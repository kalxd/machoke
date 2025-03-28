use std::ops::Deref;

use super::store::{CompletionStore, MultiLineModel};
use gtk::{
	prelude::{EntryCompletionExt, EntryExt},
	Entry, EntryCompletion, ListBox,
};

pub struct CompletionEntry {
	entry: Entry,
	store: CompletionStore,
}

impl CompletionEntry {
	pub fn new() -> Self {
		let store = CompletionStore::new();

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
	list_box: ListBox,
	model: MultiLineModel,
}

impl MultiLine {
	pub fn new() -> Self {
		let model = MultiLineModel::new();

		let list_box = ListBox::new();

		Self { list_box, model }
	}
}
