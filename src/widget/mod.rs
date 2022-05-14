use std::path::PathBuf;

use gtk::{
	gdk_pixbuf::{Pixbuf, PixbufLoader},
	glib::GString,
	prelude::*,
	Box as GtkBox, Button, Entry, FileChooserDialog, FileFilter, Image, Label, ResponseType,
	SizeGroup,
};

use crate::t::{AudioBaseInfo, FormState};

const COVER_SIZE: i32 = 128;

fn entry_text(entry: &Entry) -> Option<GString> {
	let text = entry.text();
	Some(text.trim())
		.filter(|s| !s.is_empty())
		.map(GString::from)
}

#[derive(Clone)]
pub struct CoverWidget {
	image: Image,
	change_btn: Button,
	remove_btn: Button,
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

		let s = Self {
			image,
			change_btn,
			remove_btn,
			layout,
		};

		return s;
	}

	pub fn update_cover(&self, tag: &AudioBaseInfo) -> &Self {
		if let Some(pic) = &tag.cover {
			let loader = PixbufLoader::new();
			unsafe {
				loader.write(&pic.data).unwrap_unchecked();
				loader.close().unwrap_unchecked();
			}
			let pixbuf = loader.pixbuf().and_then(|pixbuf| {
				pixbuf.scale_simple(COVER_SIZE, COVER_SIZE, gtk::gdk_pixbuf::InterpType::Nearest)
			});
			self.image.set_pixbuf(pixbuf.as_ref());
			self.active();
		} else {
			self.image.clear();
			self.inactive();
		}
		return &self;
	}

	pub fn connect_cover_changed<F: Fn(PathBuf) + 'static>(&self, f: F) {
		let image = self.image.clone();

		self.change_btn.connect_clicked(move |_| {
			let filter = FileFilter::new();
			filter.add_mime_type("image/*");
			let dialog = FileChooserDialog::builder()
				.title("选择一张能看的封面！")
				.select_multiple(false)
				.filter(&filter)
				.action(gtk::FileChooserAction::Open)
				.create_folders(false)
				.preview_widget_active(true)
				.build();

			dialog.add_buttons(&[("不好", ResponseType::Cancel), ("好", ResponseType::Ok)]);
			match dialog.run() {
				gtk::ResponseType::Ok => {
					if let Some(path) = dialog.filename() {
						let pixbuf = unsafe { Pixbuf::from_file(&path).unwrap_unchecked() };
						let pixbuf = pixbuf.scale_simple(
							COVER_SIZE,
							COVER_SIZE,
							gtk::gdk_pixbuf::InterpType::Nearest,
						);
						image.set_pixbuf(pixbuf.as_ref());
						f(path);
					}
				}
				_ => {}
			}

			dialog.emit_close();
		});
	}

	pub fn connect_cover_remove<F: Fn() + 'static>(&self, f: F) {
		let image = self.image.clone();
		self.remove_btn.connect_clicked(move |_| {
			image.clear();
			f();
		});
	}

	fn active(&self) {
		self.remove_btn.set_sensitive(true);
		self.change_btn.set_sensitive(true);
	}

	fn inactive(&self) {
		self.remove_btn.set_sensitive(false);
		self.change_btn.set_sensitive(false);
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

	pub fn set_form_state(&self, tag: &AudioBaseInfo) -> &Self {
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
