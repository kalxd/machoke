use gtk::glib::{types::Type, value::Value};
use gtk::prelude::{
	EntryCompletionExt, EntryExt, GtkListStoreExt, GtkListStoreExtManual, TreeModelExt,
};
use gtk::{Entry, EntryCompletion, ListStore};

use std::convert::AsRef;

pub struct EntryC {
	entry: Entry,
	store: ListStore,
}

impl EntryC {
	pub fn new() -> Self {
		let store = ListStore::new(&[Type::STRING]);
		let entry_completion = EntryCompletion::builder().model(&store).build();
		entry_completion.set_text_column(0);
		entry_completion.set_minimum_key_length(0);

		let entry = Entry::new();
		entry.set_completion(Some(&entry_completion));

		Self { entry, store }
	}

	pub fn set_text(&self, text: &str) {
		self.entry.set_text(text);

		let is_contains = (0..self.store.iter_n_children(None))
			.map(|i| self.store.iter_nth_child(None, i))
			.map(|miter| miter.and_then(|iter| self.store.value(&iter, 0).get::<'_, String>().ok()))
			.any(|ma| ma.map(|a| a == text).unwrap_or(false));

		if !is_contains {
			let iter = self.store.append();
			self.store.set_value(&iter, 0, &Value::from(text));
		}
	}
}

impl AsRef<Entry> for EntryC {
	fn as_ref(&self) -> &Entry {
		&self.entry
	}
}
