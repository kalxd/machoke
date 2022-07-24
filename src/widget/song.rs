use std::cell::Ref;
use std::path::Path;
use std::{cell::RefCell, rc::Rc};

use gtk::{prelude::*, Box as GtkBox, Frame};
use id3::{Tag, TagLike};

use super::{cover::CoverWidget, form::MetaForm};
use crate::emitter::Emitter;
use crate::value::{AppState, CoverMimeType};

pub struct SongWidget {
	pub cover: CoverWidget,
	pub form: MetaForm,
	pub layout: GtkBox,
	mime_type: Rc<RefCell<Option<CoverMimeType>>>,
	tx: Rc<Emitter>,
}

impl SongWidget {
	pub fn new(tx: Rc<Emitter>) -> Self {
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

		Self {
			cover,
			form,
			layout,
			mime_type: Default::default(),
			tx,
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

	/*
	pub fn save_file(&self) {
		if let Some(SongMetaData { filepath, tag }) = self.data.borrow_mut().as_mut() {
			self.save_to(tag, filepath);
		}
	}
	 */

	fn save_to<T: AsRef<Path>>(&self, tag: &mut Tag, path: T) {
		let pic_bytes = self.cover.get_pixbuf_bytes();
		let state = self.form.form_data();

		tag.pictures().for_each(|p| {
			dbg!(&p.mime_type);
		});

		if let Some(bytes) = pic_bytes {
			let pic = id3::frame::Picture {
				mime_type: "image/png".into(),
				picture_type: id3::frame::PictureType::CoverFront,
				description: "".into(),
				data: bytes,
			};

			dbg!("saving covers!!!");
			tag.add_frame(pic);
		} else {
			tag.remove_picture_by_type(id3::frame::PictureType::CoverFront);
		}

		tag.set_title(state.title);
		tag.set_artist(state.artist);
		tag.set_album(state.album);
		tag.set_genre(state.genre);

		let version = tag.version();
		let result = tag
			.write_to_path(path, version)
			.map(|_| String::from("保存成功！"));
		self.tx.alert(result);
	}
}
