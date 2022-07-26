use std::cell::Ref;
use std::{cell::RefCell, rc::Rc};

use gtk::{prelude::*, Box as GtkBox, Frame};

use super::{cover::CoverWidget, form::MetaForm};
use crate::emitter::Emitter;
use crate::value::{AppState, CoverMimeType};

pub struct SongWidget {
	pub cover: CoverWidget,
	pub form: MetaForm,
	pub layout: GtkBox,
	mime_type: Rc<RefCell<Option<CoverMimeType>>>,
}

impl SongWidget {
	pub fn new(tx: Rc<Emitter>) -> Self {
		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.sensitive(false)
			.spacing(10)
			.build();

		let cover = CoverWidget::new(tx);
		let frame = Frame::builder().label("封面").build();
		frame.add(&cover.layout);
		layout.pack_start(&frame, false, false, 10);

		let form = MetaForm::new();
		let frame = Frame::builder().label("详情").build();
		frame.add(&form.layout);
		layout.pack_start(&frame, false, false, 10);

		Self {
			cover,
			form,
			layout,
			mime_type: Default::default(),
		}
	}

	pub fn hide_something(&self) {
		self.cover.hide_something();
	}

	pub fn update(&self, state: &AppState) {
		self.layout.set_sensitive(true);
		self.cover.update_with_tag(&state);
		self.form.update(&state);

		let pic = state
			.front_cover()
			.map(|pic| CoverMimeType::from_mime_type(&pic.mime_type));
		self.mime_type.replace(pic);
	}

	pub fn get_data<'a>(&'a self) -> (Ref<Option<CoverMimeType>>, Option<Vec<u8>>) {
		let mime_type = self.mime_type.borrow();
		let pic_data = self.cover.get_pixbuf_bytes();
		(mime_type, pic_data)
	}

	pub fn remove_cover(&self) {
		self.cover.remove_cover();
		self.mime_type.replace(None);
	}
}
