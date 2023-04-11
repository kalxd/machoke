use gtk::glib::{types::Type, value::Value};
use gtk::prelude::{EntryCompletionExt, EntryExt, GtkListStoreExt, GtkListStoreExtManual};
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

		let iter = self.store.append();
		self.store.set_value(&iter, 0, &Value::from(text));
	}
}

impl AsRef<Entry> for EntryC {
	fn as_ref(&self) -> &Entry {
		&self.entry
	}
}
