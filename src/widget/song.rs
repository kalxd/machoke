use std::cell::Ref;
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};

use gtk::{prelude::*, Box as GtkBox, Frame};

use super::{cover::CoverWidget, form::MetaForm};
use crate::emitter::{EmitEvent, Emitter};
use crate::value::{AppState, CoverMimeType};

pub struct SongWidget {
	pub cover: CoverWidget,
	pub form: MetaForm,
	pub layout: GtkBox,
	mime_type: Rc<RefCell<Option<CoverMimeType>>>,
}

impl SongWidget {
	pub fn new(tx: Emitter) -> Self {
		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.sensitive(false)
			.spacing(10)
			.build();

		let cover = CoverWidget::new(tx.clone());
		let frame = Frame::builder().label("封面").build();
		frame.add(&cover.layout);
		layout.pack_start(&frame, false, false, 10);

		let form = MetaForm::new();
		let frame = Frame::builder().label("详情").build();
		frame.add(&form.layout);
		layout.pack_start(&frame, false, false, 10);

		let widget = Self {
			cover,
			form,
			layout,
			mime_type: Default::default(),
		};

		widget.connect_signal(tx);

		widget
	}

	fn connect_signal(&self, tx: Emitter) {
		self.cover.connect_change_cover({
			let tx = tx.clone();
			move |path| tx.send(EmitEvent::ChangeCover(path))
		});

		self.cover.connect_remove_cover({
			let tx = tx.clone();
			move || tx.send(EmitEvent::RemoveCover)
		});
	}

	pub fn hide_something(&self) {
		self.cover.hide_something();
	}

	pub fn update(&self, state: &AppState) {
		self.layout.set_sensitive(true);
		self.cover.update_with_tag(state);
		self.form.update(state);

		let pic = state
			.front_cover()
			.map(|pic| CoverMimeType::from_mime_type(&pic.mime_type));
		self.mime_type.replace(pic);
	}

	pub fn get_data(&self) -> (Ref<Option<CoverMimeType>>, Option<Vec<u8>>) {
		let mime_type = self.mime_type.borrow();
		let pic_data = self.cover.get_pixbuf_bytes();
		(mime_type, pic_data)
	}

	pub fn change_cover(&self, path: &PathBuf) {
		self.cover.update_cover_from_path(path);

		let mime_type = CoverMimeType::from_path(path);
		self.mime_type.replace(Some(mime_type));
	}

	pub fn remove_cover(&self) {
		self.cover.remove_cover();
		self.mime_type.replace(None);
	}
}
