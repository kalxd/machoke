use gtk::glib::{types::Type, value::Value};
use gtk::prelude::{GtkListStoreExt, GtkListStoreExtManual, TreeModelExt};
use gtk::ListStore;

pub struct TextStore(ListStore);

impl TextStore {
	pub fn new() -> Self {
		let store = ListStore::new(&[Type::STRING]);

		Self(store)
	}

	pub fn set_text(&self, text: &str) {
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
}

impl AsRef<ListStore> for TextStore {
	fn as_ref(&self) -> &ListStore {
		&self.0
	}
}
