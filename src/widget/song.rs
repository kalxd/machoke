use std::path::Path;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use gtk::glib;
use gtk::{prelude::*, Box as GtkBox, Frame};
use id3::{Tag, TagLike};

use super::AppAction;
use super::{cover::CoverWidget, form::MetaForm};

struct SongMetaData {
	filepath: PathBuf,
	tag: id3::Tag,
}

pub struct SongWidget {
	pub cover: CoverWidget,
	pub form: MetaForm,
	pub layout: GtkBox,
	data: Rc<RefCell<Option<SongMetaData>>>,
	tx: Rc<glib::Sender<AppAction>>,
}

impl SongWidget {
	pub fn new(tx: Rc<glib::Sender<AppAction>>) -> Self {
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
			data: Default::default(),
			tx,
		}
	}

	pub fn hide_something(&self) {
		self.cover.hide_something();
	}

	pub fn update(&self, filepath: PathBuf, tag: id3::Tag) {
		self.layout.set_sensitive(true);
		self.cover.update(&tag);
		self.form.update(&tag);
		self.data.replace(Some(SongMetaData { filepath, tag }));
	}

	pub fn save_file(&self) {
		if let Some(SongMetaData { filepath, tag }) = self.data.borrow_mut().as_mut() {
			self.save_to(tag, filepath);
		}
	}

	fn save_to<T: AsRef<Path>>(&self, tag: &mut Tag, path: T) {
		let pic_bytes = self.cover.get_pixbuf_bytes();
		let state = self.form.state();

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
			.map(|_| "保存成功！".into())
			.map_err(|e| e.to_string());
		self.tx.send(AppAction::Alert(result)).unwrap();
	}
}
