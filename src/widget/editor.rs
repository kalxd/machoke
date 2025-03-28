use gtk::{
	glib::IsA,
	prelude::{BoxExt, ContainerExt, SizeGroupExt},
	Box as GtkBox, Button, ButtonBox, Frame, Label, Orientation, SizeGroup, Widget,
};
use id3::TagLike;

use crate::value::ParseBox;

use super::element::{cover, multi_line::CompletionEntry};

struct EditorRow {
	layout: GtkBox,
	size_group: SizeGroup,
}

impl EditorRow {
	fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.margin(10)
			.build();

		let size_group = SizeGroup::new(gtk::SizeGroupMode::Horizontal);

		Self { layout, size_group }
	}

	fn add_row_with(&self, label: &str, w: &impl IsA<Widget>) {
		let row_layout = GtkBox::new(Orientation::Horizontal, 10);
		let label = Label::new(Some(label));
		row_layout.pack_start(&label, false, false, 0);
		self.size_group.add_widget(&label);

		row_layout.pack_start(w, true, true, 0);
		self.layout.pack_start(&row_layout, false, true, 10);
	}

	fn add_row(&self, label: &str) -> CompletionEntry {
		let entry = CompletionEntry::new();
		self.add_row_with(label, &*entry);
		entry
	}
}

pub struct Editor {
	pub layout: GtkBox,

	title_line: CompletionEntry,
}

impl Editor {
	pub fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.build();

		let album_layout = GtkBox::builder().spacing(10).build();
		layout.pack_start(&album_layout, false, true, 0);

		let cur_album_frame = Frame::builder().label("封面").build();
		album_layout.pack_start(&cur_album_frame, true, true, 0);

		let cover = cover::Cover::new();
		cur_album_frame.set_child(Some(&cover.layout));

		let history_alubm_frame = Frame::builder().label("历史封面").margin_start(5).build();
		album_layout.pack_start(&history_alubm_frame, true, true, 0);

		let form_frame = Frame::builder().label("基础信息").build();
		layout.pack_start(&form_frame, true, true, 10);

		let form_row = EditorRow::new();
		form_frame.set_child(Some(&form_row.layout));

		let title_line = form_row.add_row("标题");
		form_row.add_row("other");

		let btn_box = ButtonBox::builder()
			.layout_style(gtk::ButtonBoxStyle::End)
			.spacing(10)
			.build();
		layout.pack_start(&btn_box, false, false, 0);

		let close_btn = Button::with_label("关闭");
		btn_box.add(&close_btn);

		let save_btn = Button::with_label("保存");
		btn_box.add(&save_btn);

		Self { layout, title_line }
	}

	pub fn update_state(&self, state: &ParseBox) {
		let title = state.audio_tag.title();
		self.title_line.set_text(title.unwrap_or_default());
	}
}
