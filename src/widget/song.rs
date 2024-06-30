use gtk::{prelude::*, Box as GtkBox, Frame};

use super::{cover::CoverWidget, form::MetaForm};
use crate::emitter::{EmitEvent, Emitter};
use crate::value::AppState;

pub struct SongWidget {
	pub cover: CoverWidget,
	pub form: MetaForm,
	pub layout: GtkBox,
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
	}
}
