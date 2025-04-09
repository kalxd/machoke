use gtk::{
	gdk_pixbuf::Pixbuf,
	glib::{StaticType, Type, Value},
	prelude::{GtkListStoreExt, GtkListStoreExtManual, TreeModelExt},
	ListStore,
};
use std::ops::Deref;

use crate::value::{scale_picture, CoverMimeType};

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
			.filter_map(|i| self.0.iter_nth_child(None, i))
			.filter_map(|iter| self.0.value(&iter, 0).get::<'_, String>().ok())
			.any(|s| s == text);

		if !is_contains {
			let iter = self.0.append();
			self.0.set_value(&iter, 0, &Value::from(text));
		}
	}
}

impl Deref for CompletionStore {
	type Target = ListStore;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub struct HistoryStore(ListStore);

impl HistoryStore {
	pub fn new() -> Self {
		let store = ListStore::new(&[
			Type::STRING,
			Pixbuf::static_type(),
			Pixbuf::static_type(),
			Type::STRING,
		]);
		Self(store)
	}

	pub fn add_item(&self, key: &str, pic: &id3::frame::Picture) {
		let is_exist = (0..self.0.iter_n_children(None))
			.filter_map(|i| self.0.iter_nth_child(None, i))
			.filter_map(|iter| self.0.value(&iter, 0).get::<'_, String>().ok())
			.any(|s| s == key);

		if is_exist {
			return;
		}

		if let Some((raw_pixbuf, scale_pixbuf)) = scale_picture(pic, 64) {
			let iter = self.0.prepend();
			self.0.set(
				&iter,
				&[
					(0, &key),
					(1, &scale_pixbuf),
					(2, &raw_pixbuf),
					(3, &pic.mime_type),
				],
			)
		}
	}
}

impl Deref for HistoryStore {
	type Target = ListStore;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
