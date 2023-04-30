use gtk::{
	glib::object::IsA, prelude::*, Align, Box as GtkBox, Label, Orientation, SizeGroup,
	SizeGroupMode, Widget,
};
use id3::TagLike;

use super::element::entry::{EntryC, MultiEntry};
use crate::value::{AppState, MetaFormData, FAV_SPACING};

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

	fn add_row_with(&self, label: &str, w: &impl IsA<Widget>) {
		let row_layout = GtkBox::new(Orientation::Horizontal, FAV_SPACING);
		let label = Label::new(Some(label));
		row_layout.pack_start(&label, false, false, 0);
		self.size_group.add_widget(&label);

		row_layout.pack_start(w, true, true, 0);
		self.layout.pack_start(&row_layout, false, true, 10);
	}

	fn add_row_entryc(&self, label: &str) -> EntryC {
		let entryc = EntryC::new();
		self.add_row_with(label, &*entryc);
		entryc
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

		self.layout.pack_start(&row_layout, false, true, 0);
		mutil_entry
	}
}

pub struct MetaForm {
	pub layout: GtkBox,
	title_entry: EntryC,
	artist_entry: MultiEntry,
	album_entry: EntryC,
	genre_entry: MultiEntry,
}

impl MetaForm {
	pub fn new() -> Self {
		let layout = GtkBox::builder().orientation(Orientation::Vertical).build();

		let form_row = FormRow::new();

		let title_entry = form_row.add_row_entryc("标题");
		let artist_entry = form_row.add_multi_entry("艺术家");
		let album_entry = form_row.add_row_entryc("专辑");
		let genre_entry = form_row.add_multi_entry("流派");

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
		self.artist_entry
			.set_text_list(&state.tag.artists().unwrap_or_default());
		self.album_entry.set_text(state.tag.album().unwrap_or(""));
		self.genre_entry
			.set_text_list(&state.tag.genres().unwrap_or_default());
	}

	pub fn form_data(&self) -> MetaFormData {
		let title = self.title_entry.text();
		let artist = self.artist_entry.get_text_list();
		let album = self.album_entry.text();
		let genre = self.genre_entry.get_text_list();

		MetaFormData {
			title,
			artist,
			album,
			genre,
		}
	}

	pub fn save_to_store(&self, data: &MetaFormData) {
		self.title_entry.store.set_text(&data.title);
		self.artist_entry.store.set_text_list(&data.artist);
		self.album_entry.store.set_text(&data.album);
		self.genre_entry.store.set_text_list(&data.genre);
	}
}
