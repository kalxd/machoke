mod row_object_impl {
	use gtk::{glib, prelude::*, subclass::prelude::*};
	use std::cell::RefCell;

	#[derive(Default, glib::Properties)]
	#[properties(wrapper_type = super::MultiLineObject)]
	pub struct InnerRowObject {
		#[property(get, set)]
		text: RefCell<String>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for InnerRowObject {
		const NAME: &str = "RowItem";
		type Type = super::MultiLineObject;
	}

	#[glib::derived_properties]
	impl ObjectImpl for InnerRowObject {}
}

mod model_imp {
	use gtk::{
		gio,
		glib::{self, Cast, StaticType},
		subclass::prelude::{ListModelImpl, ObjectImpl, ObjectSubclass},
	};
	use std::cell::RefCell;

	#[derive(Default, Debug)]
	pub struct InnerModel(pub RefCell<Vec<super::MultiLineObject>>);

	#[glib::object_subclass]
	impl ObjectSubclass for InnerModel {
		const NAME: &str = "Model";
		type Type = super::MultiLineModel;
		type ParentType = glib::Object;
		type Interfaces = (gio::ListModel,);
	}

	impl ObjectImpl for InnerModel {}

	impl ListModelImpl for InnerModel {
		fn item_type(&self) -> glib::Type {
			super::MultiLineObject::static_type()
		}

		fn n_items(&self) -> u32 {
			self.0.borrow().len() as u32
		}

		fn item(&self, position: u32) -> Option<glib::Object> {
			let item = self.0.borrow().get(position as usize)?.clone();
			Some(item.upcast())
		}
	}
}

use std::ops::Deref;

use gtk::{
	gio,
	glib::{self, Type, Value},
	prelude::{GtkListStoreExt, GtkListStoreExtManual, TreeModelExt},
	ListStore,
};

glib::wrapper! {
	pub struct MultiLineObject(ObjectSubclass<row_object_impl::InnerRowObject>);
}

impl MultiLineObject {
	fn new(value: &str) -> Self {
		glib::Object::builder().property("text", value).build()
	}
}

glib::wrapper! {
	pub struct MultiLineModel(ObjectSubclass<model_imp::InnerModel>) @implements gio::ListModel;
}

impl MultiLineModel {
	pub(super) fn new() -> Self {
		glib::Object::new()
	}
}

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
