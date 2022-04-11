use gtk::{prelude::*, Box as GtkBox, FileChooserWidget, FileFilter, Frame, Image, Orientation};

use std::ops::Deref;

pub struct FileWidget {
	file_chooser: FileChooserWidget,
}

impl FileWidget {
	pub fn new() -> Self {
		let filter = FileFilter::new();
		filter.add_mime_type("audio/*");

		let file_chooser = FileChooserWidget::builder()
			.select_multiple(false)
			.filter(&filter)
			.build();
		Self { file_chooser }
	}
}

impl Deref for FileWidget {
	type Target = FileChooserWidget;

	fn deref(&self) -> &Self::Target {
		&self.file_chooser
	}
}

pub struct SongForm {
	pub layout: Frame,
}

impl SongForm {
	pub fn new() -> Self {
		let layout = Frame::builder().label("歌曲信息").build();

		let main_layout = GtkBox::builder().orientation(Orientation::Vertical).build();

		let cover_layout = Frame::builder().label("封面").build();
		let cover_item = Image::new();
		cover_layout.add(&cover_item);

		main_layout.pack_start(&cover_layout, false, false, 0);
		layout.add(&main_layout);

		Self { layout }
	}
}
