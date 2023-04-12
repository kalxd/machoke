use gtk::prelude::{EntryCompletionExt, EntryExt};
use gtk::{Entry, EntryCompletion};

use super::textstore::TextStore;

pub struct EntryC {
	entry: Entry,
	store: TextStore,
}

impl EntryC {
	pub fn new() -> Self {
		let store = TextStore::new();
		let entry_completion = EntryCompletion::builder()
			.model(store.as_ref())
			.minimum_key_length(0)
			.build();
		entry_completion.set_text_column(0);

		let entry = Entry::new();
		entry.set_completion(Some(&entry_completion));

		Self { entry, store }
	}

	pub fn set_text(&self, text: &str) {
		self.entry.set_text(text);
		self.store.set_text(text);
	}
}

impl AsRef<Entry> for EntryC {
	fn as_ref(&self) -> &Entry {
		&self.entry
	}
}
