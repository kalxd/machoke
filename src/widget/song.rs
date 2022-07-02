use std::{cell::RefCell, path::PathBuf, rc::Rc};

use gtk::glib;
use gtk::{prelude::*, Box as GtkBox, Frame};

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
	tag: Rc<RefCell<Option<SongMetaData>>>,
}

impl SongWidget {
	pub fn new(tx: Rc<glib::Sender<AppAction>>) -> Self {
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
			tag: Default::default(),
		}
	}

	pub fn hide_something(&self) {
		self.cover.hide_something();
	}

	pub fn update(&self, filepath: PathBuf, tag: id3::Tag) {
		self.layout.set_sensitive(true);
		self.cover.update(&tag);
		self.form.update(&tag);
		self.tag.replace(Some(SongMetaData { filepath, tag }));
	}
}
