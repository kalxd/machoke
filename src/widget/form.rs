use gtk::{
	prelude::*, Align, Box as GtkBox, Button, Entry, EntryCompletion, Image, Label, ListStore,
	Orientation, SizeGroup, SizeGroupMode,
};
use id3::TagLike;

use crate::value::{AppState, MetaFormData, FAV_SPACING};

const GENRE: &[&'static str] = &[
	"袁派", "王派", "傅派", "戚派", "金派", "张派", "吕派", "徐派", "范派", "陆派", "毕派", "尹派",
];

struct GenreStore(ListStore);

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

struct MultiEntryRow {
	entry: Entry,
	btn: Button,
	layout: GtkBox,
}

impl MultiEntryRow {
	fn new() -> Self {
		let layout = GtkBox::new(Orientation::Horizontal, 0);
		let entry = Entry::new();
		let btn = Button::builder()
			.image(&Image::builder().icon_name("list-remove").build())
			.build();

		layout.pack_start(&entry, true, true, 0);
		layout.pack_end(&btn, false, false, 0);

		Self { entry, btn, layout }
	}

	fn set_text<S: AsRef<str>>(&self, text: S) {
		self.entry.set_text(text.as_ref());
	}
}

// 多行文本
struct MultiEntry {
	entry: Entry,
	entry_list: Vec<MultiEntryRow>,
	add_btn: Button,
	layout: GtkBox,
}

impl MultiEntry {
	fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.halign(Align::Start)
			.build();

		let entry = Entry::new();
		let entry_list = vec![];
		let add_btn = Button::builder()
			.image(&Image::builder().icon_name("list-add").build())
			.tooltip_text("添加一列新内容")
			.build();

		layout.pack_start(&entry, true, true, 0);
		layout.pack_start(&add_btn, false, false, 0);

		Self {
			entry,
			entry_list,
			add_btn,
			layout,
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
			.margin(FAV_SPACING)
			.build();
		let size_group = SizeGroup::new(SizeGroupMode::Horizontal);

		Self { size_group, layout }
	}

	fn add_row(&self, label: &str) -> Entry {
		let row_layout = GtkBox::new(Orientation::Horizontal, FAV_SPACING);
		let label = Label::new(Some(label));
		row_layout.pack_start(&label, false, false, 0);
		self.size_group.add_widget(&label);

		let entry = Entry::new();
		row_layout.pack_start(&entry, true, true, 0);

		self.layout.pack_start(&row_layout, false, true, 10);

		return entry;
	}

	fn add_multi_entry(&self, label: &str) -> MultiEntry {
		let row_layout = GtkBox::new(Orientation::Horizontal, FAV_SPACING);

		let label = Label::builder()
			.label(label)
			.valign(Align::Start)
			.margin_top(FAV_SPACING)
			.build();
		row_layout.pack_start(&label, false, false, 0);
		self.size_group.add_widget(&label);

		let mutil_entry = MultiEntry::new();
		row_layout.pack_start(&mutil_entry.layout, true, true, 0);

		self.layout.pack_start(&row_layout, true, true, 0);
		return mutil_entry;
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

		{
			form_row.add_multi_entry("我的流派");
		}

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
