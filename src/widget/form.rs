use gtk::{prelude::*, Box as GtkBox, Entry, Frame, Label, Orientation, SizeGroup, SizeGroupMode};
use id3::TagLike;

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
		row_layout.pack_end(&entry, true, true, 0);

		self.layout.pack_end(&row_layout, false, true, 10);

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

		let title_entry = form_row.add_row("歌手");
		let artist_entry = form_row.add_row("艺术家");
		let album_entry = form_row.add_row("专辑");
		let genre_entry = form_row.add_row("流派");

		layout.add(&form_row.layout);

		Self {
			layout,
			title_entry,
			artist_entry,
			album_entry,
			genre_entry,
		}
	}

	pub fn update(&self, tag: &id3::Tag) {
		self.title_entry.set_text(tag.title().unwrap_or(""));
		self.artist_entry.set_text(tag.artist().unwrap_or(""));
		self.album_entry.set_text(tag.album().unwrap_or(""));
		self.genre_entry.set_text(tag.genre().unwrap_or(""));
	}
}
