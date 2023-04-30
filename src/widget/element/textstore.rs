use std::ops::Deref;

use gtk::glib::{types::Type, value::Value};
use gtk::prelude::{
	EntryCompletionExt, EntryExt, GtkListStoreExt, GtkListStoreExtManual, TreeModelExt,
};
use gtk::{Entry, EntryCompletion, ListStore};

pub struct TextStore(ListStore);

impl TextStore {
	pub fn new() -> Self {
		let store = ListStore::new(&[Type::STRING]);

		Self(store)
	}

	pub fn new_entry(&self) -> Entry {
		let entry_completion = EntryCompletion::builder()
			.model(&self.0)
			.minimum_key_length(0)
			.popup_completion(true)
			.build();
		entry_completion.set_text_column(0);

		let entry = Entry::new();
		entry.set_completion(Some(&entry_completion));

		entry
	}

	pub fn set_text<S: AsRef<str>>(&self, text: S) {
		let text = text.as_ref();
		if text.trim().is_empty() {
			return;
		}

		let is_contains = (0..self.0.iter_n_children(None))
			.map(|i| self.0.iter_nth_child(None, i))
			.map(|miter| miter.and_then(|iter| self.0.value(&iter, 0).get::<'_, String>().ok()))
			.any(|ma| ma.map(|a| a == text).unwrap_or(false));

		if !is_contains {
			let iter = self.0.append();
			self.0.set_value(&iter, 0, &Value::from(text));
		}
	}

	pub fn set_text_list<S: AsRef<str>>(&self, texts: &[S]) {
		for text in texts {
			self.set_text(text);
		}
	}
}

impl Deref for TextStore {
	type Target = ListStore;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Default for TextStore {
	fn default() -> Self {
		Self::new()
	}
}
