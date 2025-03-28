use gtk::{
	glib::{Type, Value},
	prelude::{GtkListStoreExt, GtkListStoreExtManual, TreeModelExt},
	ListStore,
};
use std::ops::Deref;

#[derive(Clone)]
pub struct CompletionStore(ListStore);

impl CompletionStore {
	pub fn new() -> Self {
		let store = ListStore::new(&[Type::STRING]);

		Self(store)
	}

	pub(super) fn set_text(&self, text: &str) {
		let text = text.trim();
		if text.is_empty() {
			return;
		}

		let is_contains = (0..self.0.iter_n_children(None))
			.map(|i| self.0.iter_nth_child(None, i))
			.map(|miter| miter.and_then(|iter| self.0.value(&iter, 0).get::<'_, String>().ok()))
			.any(|ma| ma.as_deref() == Some(text));

		if !is_contains {
			let iter = self.0.append();
			self.0.set_value(&iter, 0, &Value::from(text));
		}
	}

	pub(super) fn set_text_list(&self, text_list: &[&str]) {
		for s in text_list {
			self.set_text(s);
		}
	}
}

impl Deref for CompletionStore {
	type Target = ListStore;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
