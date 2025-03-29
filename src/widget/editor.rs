use gtk::{
	glib::IsA,
	prelude::{BoxExt, ButtonExt, ContainerExt, SizeGroupExt},
	Box as GtkBox, Button, ButtonBox, Frame, Label, Orientation, SizeGroup, Widget,
};
use id3::TagLike;

use crate::value::{EventAction, EventSender, ParseBox};

use super::element::{
	cover,
	multi_line::{CompletionEntry, MultiLine},
	store::CompletionStore,
};

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
		let label = Label::builder()
			.label(label)
			.valign(gtk::Align::Start)
			.margin_top(8)
			.build();
		row_layout.pack_start(&label, false, false, 0);
		self.size_group.add_widget(&label);

		row_layout.pack_start(w, true, true, 0);
		self.layout.pack_start(&row_layout, false, true, 10);
	}

	fn add_row(&self, label: &str) -> CompletionEntry {
		let store = CompletionStore::new();
		let entry = CompletionEntry::new(store);
		self.add_row_with(label, &*entry);
		entry
	}

	fn add_multi_row(&self, label: &str) -> MultiLine {
		let multi_line = MultiLine::new();
		self.add_row_with(label, &multi_line.layout);
		multi_line
	}
}

pub struct Editor {
	pub layout: GtkBox,

	title_line: CompletionEntry,
	artist_line: MultiLine,
	album_line: CompletionEntry,
	genre_line: MultiLine,
}

impl Editor {
	pub fn new(tx: EventSender) -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.build();

		let cover_layout = GtkBox::builder().spacing(10).build();
		layout.pack_start(&cover_layout, false, true, 0);

		let cur_cover_frame = Frame::builder().label("封面").build();
		cover_layout.pack_start(&cur_cover_frame, true, true, 0);

		let cover = cover::Cover::new();
		cur_cover_frame.set_child(Some(&cover.layout));

		let history_cover_frame = Frame::builder().label("历史封面").build();
		cover_layout.pack_start(&history_cover_frame, true, true, 0);

		let form_frame = Frame::builder().label("基础信息").build();
		layout.pack_start(&form_frame, true, true, 10);

		let form_row = EditorRow::new();
		form_frame.set_child(Some(&form_row.layout));

		let title_line = form_row.add_row("标题");
		let artist_line = form_row.add_multi_row("艺术家");
		let album_line = form_row.add_row("专辑");
		let genre_line = form_row.add_multi_row("流派");

		let btn_box = ButtonBox::builder()
			.layout_style(gtk::ButtonBoxStyle::End)
			.spacing(10)
			.build();
		layout.pack_start(&btn_box, false, false, 0);

		let close_btn = Button::with_label("关闭");
		btn_box.add(&close_btn);
		close_btn.connect_clicked({
			let tx = tx.clone();
			move |_| tx.send(EventAction::Close)
		});

		let save_btn = Button::with_label("保存");
		btn_box.add(&save_btn);

		Self {
			layout,
			title_line,
			artist_line,
			album_line,
			genre_line,
		}
	}

	pub fn update_state(&self, state: &ParseBox) {
		let title = state.audio_tag.title();
		self.title_line.set_text(title.unwrap_or_default());

		let artist = state.audio_tag.artists();
		self.artist_line.set_text(&artist.unwrap_or_default());

		let album = state.audio_tag.album();
		self.album_line.set_text(album.unwrap_or_default());

		let genre = state.audio_tag.genres();
		self.genre_line.set_text(&genre.unwrap_or_default());
	}
}
