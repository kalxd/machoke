use gtk::{
	prelude::*, Box as GtkBox, Button, Entry, EntryCompletion, Image, Label, ListStore,
	Orientation, SizeGroup, SizeGroupMode,
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

// 单行的编辑器
struct MultiRowEntry {
	entry: Entry,
	remove_btn: Button,
	layout: GtkBox,
}

impl MultiRowEntry {
	fn new() -> Self {
		let layout = GtkBox::new(Orientation::Horizontal, 10);

		let entry = Entry::new();
		layout.pack_start(&entry, true, true, 0);

		let remove_btn = Button::builder()
			.image(&Image::builder().icon_name("list-remove").build())
			.build();
		layout.pack_start(&remove_btn, false, false, 0);

		Self {
			entry,
			remove_btn,
			layout,
		}
	}
}

// 多行编辑器
struct MultiEntry {
	entry_list: Vec<Entry>,
	layout: GtkBox,
}

impl MultiEntry {
	fn new() -> Self {
		let main_layout = GtkBox::builder()
			.spacing(10)
			.orientation(Orientation::Vertical)
			.build();

		Self {
			entry_list: vec![],
			layout: main_layout,
		}
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

		{
			// 测试用的
			let label = Label::new(Some("sb"));
			let row = GtkBox::builder()
				.spacing(10)
				.orientation(Orientation::Horizontal)
				.build();
			row.pack_start(&label, false, false, 0);

			let v = GtkBox::builder()
				.spacing(10)
				.orientation(Orientation::Vertical)
				.build();
			{
				let vl = GtkBox::builder()
					.spacing(10)
					.orientation(Orientation::Horizontal)
					.build();

				let entry = Entry::new();
				vl.pack_start(&entry, true, true, 0);

				let b = Button::with_label("+");
				vl.pack_start(&b, false, false, 0);

				v.pack_start(&vl, false, false, 0);
			}
			row.pack_start(&v, false, true, 0);

			layout.pack_start(&row, false, false, 10);
		}

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
