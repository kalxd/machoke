use std::{cell::RefCell, rc::Rc};

use gtk::{prelude::*, Box as GtkBox, Button, Entry, Image, Label, SizeGroup};
use id3::Tag;

const COVER_SIZE: i32 = 128;

pub struct FormState {
	title: Option<String>,
	artist: Option<String>,
	album: Option<String>,
	genere: Option<String>,
}

impl Default for FormState {
	fn default() -> Self {
		Self {
			title: None,
			artist: None,
			album: None,
			genere: None,
		}
	}
}

pub struct AppState {
	tag: Option<Tag>,
	form_state: Rc<RefCell<FormState>>,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			tag: None,
			form_state: Default::default(),
		}
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
}
