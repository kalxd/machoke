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

use gtk::{gio, glib};

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
