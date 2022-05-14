use gtk::{glib::GString, prelude::*, Box as GtkBox, Button, Entry, Image, Label, SizeGroup};
use id3::Tag;

use crate::t::AudioTag;

const COVER_SIZE: i32 = 128;

fn entry_text(entry: &Entry) -> Option<GString> {
	let text = entry.text();
	Some(text.trim())
		.filter(|s| s.is_empty())
		.map(GString::from)
}

pub struct FormState {
	title: Option<GString>,
	artist: Option<GString>,
	album: Option<GString>,
	genre: Option<GString>,
}

impl Default for FormState {
	fn default() -> Self {
		Self {
			title: None,
			artist: None,
			album: None,
			genre: None,
		}
	}
}

pub struct AppState {
	tag: Option<Tag>,
}

impl Default for AppState {
	fn default() -> Self {
		Self { tag: None }
	}
}

pub struct CoverWidget {
	image: Image,
	pub layout: GtkBox,
}

impl CoverWidget {
	pub fn new() -> Self {
		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Horizontal)
			.build();

		let change_btn = Button::builder().sensitive(false).label("更换封面").build();
		let remove_btn = Button::builder().sensitive(false).label("删除封面").build();
		let btn_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();
		btn_layout.pack_start(&change_btn, false, false, 0);
		btn_layout.pack_end(&remove_btn, false, false, 0);

		layout.pack_start(&image, false, false, 0);
		layout.pack_start(&btn_layout, false, false, 0);

		Self { image, layout }
	}
}

#[derive(Clone)]
pub struct FormWidget {
	title_entry: Entry,
	artist_entry: Entry,
	album_entry: Entry,
	genre_entry: Entry,
	pub layout: GtkBox,
}

impl FormWidget {
	pub fn new() -> Self {
		let layout = GtkBox::builder()
			.margin(10)
			.orientation(gtk::Orientation::Vertical)
			.spacing(10)
			.build();
		let size_group = SizeGroup::new(gtk::SizeGroupMode::Horizontal);

		let row_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(10)
			.build();
		let title_label = Label::new(Some("歌曲名称"));
		let title_entry = Entry::new();
		size_group.add_widget(&title_label);
		row_layout.pack_start(&title_label, false, false, 0);
		row_layout.pack_start(&title_entry, true, true, 0);
		layout.pack_start(&row_layout, false, false, 0);

		let row_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(10)
			.build();
		let artist_label = Label::new(Some("艺术家"));
		let artist_entry = Entry::new();
		size_group.add_widget(&artist_label);
		row_layout.pack_start(&artist_label, false, false, 0);
		row_layout.pack_start(&artist_entry, true, true, 0);
		layout.pack_start(&row_layout, false, false, 0);

		let row_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(10)
			.build();
		let album_label = Label::new(Some("专辑"));
		let album_entry = Entry::new();
		size_group.add_widget(&album_label);
		row_layout.pack_start(&album_label, false, false, 0);
		row_layout.pack_start(&album_entry, true, true, 0);
		layout.pack_start(&row_layout, false, false, 0);

		let row_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(10)
			.build();

		let genre_label = Label::new(Some("流派"));
		let genre_entry = Entry::new();
		size_group.add_widget(&genre_label);
		row_layout.pack_start(&genre_label, false, false, 0);
		row_layout.pack_start(&genre_entry, true, true, 0);
		layout.pack_start(&row_layout, false, false, 0);

		Self {
			title_entry,
			artist_entry,
			album_entry,
			genre_entry,
			layout,
		}
	}

	pub fn set_form_state(&self, tag: &AudioTag) -> &Self {
		self.title_entry
			.set_text(tag.title.as_deref().unwrap_or_default());
		self.artist_entry
			.set_text(tag.artist.as_deref().unwrap_or_default());
		self.album_entry
			.set_text(tag.album.as_deref().unwrap_or_default());
		self.genre_entry
			.set_text(tag.genre.as_deref().unwrap_or_default());
		return &self;
	}

	pub fn get_form_state(&self) -> FormState {
		let title = entry_text(&self.title_entry);
		let artist = entry_text(&self.artist_entry);
		let album = entry_text(&self.album_entry);
		let genre = entry_text(&self.genre_entry);

		FormState {
			title,
			artist,
			album,
			genre,
		}
	}
}
