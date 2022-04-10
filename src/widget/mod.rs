use gtk::{FileChooserWidget, FileFilter};

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
