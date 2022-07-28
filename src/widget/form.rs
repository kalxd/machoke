use gtk::{
	prelude::*, Box as GtkBox, Entry, EntryCompletion, Label, ListStore, Orientation, SizeGroup,
	SizeGroupMode,
};
use id3::TagLike;

use crate::value::{AppState, MetaFormData};

const GENRE: &[&'static str] = &[
	"袁派", "王派", "傅派", "戚派", "金派", "张派", "吕派", "徐派", "范派", "陆派", "毕派", "尹派",
];

struct GenreStore(pub ListStore);

impl GenreStore {
	fn new() -> Self {
		let store = ListStore::new(&[gtk::glib::types::Type::STRING]);

		GENRE.iter().for_each(|name| {
			let iter = store.append();
			store.set(&iter, &[(0, name)]);
		});

		Self(store)
	}
}

struct FormRow {
	size_group: SizeGroup,
	layout: GtkBox,
}

impl FormRow {
	fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.margin(10)
			.build();
		let size_group = SizeGroup::new(SizeGroupMode::Horizontal);

		Self { size_group, layout }
	}

	fn add_row(&self, label: &str) -> Entry {
		let row_layout = GtkBox::new(Orientation::Horizontal, 10);
		let label = Label::new(Some(label));
		row_layout.pack_start(&label, false, false, 0);
		self.size_group.add_widget(&label);

		let entry = Entry::new();
		row_layout.pack_start(&entry, true, true, 0);

		self.layout.pack_start(&row_layout, false, true, 10);

		return entry;
	}
}

pub struct MetaForm {
	pub layout: GtkBox,
	title_entry: Entry,
	artist_entry: Entry,
	album_entry: Entry,
	genre_entry: Entry,
}

impl MetaForm {
	pub fn new() -> Self {
		let layout = GtkBox::builder().orientation(Orientation::Vertical).build();

		let form_row = FormRow::new();

		let title_entry = form_row.add_row("标题");
		let artist_entry = form_row.add_row("艺术家");
		let album_entry = form_row.add_row("专辑");
		let genre_entry = form_row.add_row("流派");

		let genre_store = GenreStore::new();
		let genre_completion = EntryCompletion::builder()
			.model(&genre_store.0)
			.minimum_key_length(0)
			.build();
		genre_entry.set_completion(Some(&genre_completion));
		genre_completion.set_text_column(0);

		layout.add(&form_row.layout);

		Self {
			layout,
			title_entry,
			artist_entry,
			album_entry,
			genre_entry,
		}
	}

	pub fn update(&self, state: &AppState) {
		self.title_entry.set_text(state.tag.title().unwrap_or(""));
		self.artist_entry.set_text(state.tag.artist().unwrap_or(""));
		self.album_entry.set_text(state.tag.album().unwrap_or(""));
		self.genre_entry.set_text(state.tag.genre().unwrap_or(""));
	}

	pub fn form_data(&self) -> MetaFormData {
		let title = self.title_entry.text();
		let artist = self.artist_entry.text();
		let album = self.album_entry.text();
		let genre = self.genre_entry.text();

		MetaFormData {
			title,
			artist,
			album,
			genre,
		}
	}
}
