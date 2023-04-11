use gtk::{
	glib::{object::IsA, GString},
	prelude::*,
	Align, Box as GtkBox, Button, Entry, EntryCompletion, Image, Label, Orientation, SizeGroup,
	SizeGroupMode, Widget,
};
use id3::TagLike;

use std::cell::RefCell;
use std::rc::Rc;

use super::entryc::EntryC;
use crate::value::{AppState, MetaFormData, FAV_SPACING};

struct MultiEntryRow {
	entry: Entry,
	btn: Button,
	layout: GtkBox,
}

impl MultiEntryRow {
	fn new(entry_completion: Option<&EntryCompletion>) -> Self {
		let layout = GtkBox::new(Orientation::Horizontal, FAV_SPACING);
		let entry = Entry::new();
		entry.set_completion(entry_completion);
		let btn = Button::builder()
			.image(&Image::builder().icon_name("list-remove").build())
			.tooltip_text("删除该列")
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
	entry_list: Rc<RefCell<Vec<MultiEntryRow>>>,
	entry_completion: EntryCompletion,
	layout: GtkBox,
}

impl MultiEntry {
	fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.build();

		let entry_completion = EntryCompletion::new();
		let entry = Entry::new();
		entry.set_completion(Some(&entry_completion));

		let entry_list = Rc::new(RefCell::new(vec![]));
		let add_btn = Button::builder()
			.image(&Image::builder().icon_name("list-add").build())
			.tooltip_text("添加一列新内容")
			.build();

		layout.pack_start(&entry, true, true, 0);
		layout.pack_end(&add_btn, false, false, 0);

		{
			// 添加新的一列
			add_btn.connect_clicked({
				let entry_list = entry_list.clone();
				let entry_completion = entry_completion.clone();
				let layout = layout.clone();
				move |_| {
					let row = MultiEntryRow::new(Some(&entry_completion));
					layout.pack_start(&row.layout, false, false, 0);
					row.layout.show_all();

					row.btn.connect_clicked({
						let layout = layout.clone();
						let row_layout = row.layout.clone();
						let entry_list = entry_list.clone();
						move |_| {
							layout.remove(&row_layout);
							let mut xs = entry_list.borrow_mut();
							xs.retain(|x: &MultiEntryRow| x.layout != row_layout);
						}
					});

					let mut xs = entry_list.borrow_mut();
					xs.push(row);
				}
			});
		}

		Self {
			entry,
			entry_list,
			entry_completion,
			layout,
		}
	}

	fn reset_row(&self) {
		for row in self.entry_list.take() {
			self.layout.remove(&row.layout);
		}
	}

	fn add_row<S: AsRef<str>>(&self, text: S) {
		let layout = self.layout.clone();

		let row = MultiEntryRow::new(Some(&self.entry_completion));
		row.set_text(text);
		let row_layout = row.layout.clone();
		layout.pack_start(&row.layout, false, false, 0);
		layout.show_all();

		row.btn.connect_clicked({
			let entry_list = self.entry_list.clone();
			move |_| {
				layout.remove(&row_layout);
				let mut xs = entry_list.borrow_mut();
				xs.retain(|x| x.layout != row_layout);
			}
		});

		let mut xs = self.entry_list.borrow_mut();
		xs.push(row);
	}

	fn set_text_list<S: AsRef<str>>(&self, xs: &[S]) {
		self.reset_row();
		if let Some((h, xs)) = xs.split_first() {
			self.entry.set_text(h.as_ref());
			xs.iter().for_each(|x| self.add_row(x));
		} else {
			self.entry.set_text("");
		}
	}

	fn get_text_list(&self) -> Vec<GString> {
		let mut xs = vec![];
		xs.push(self.entry.text());

		let mut ys: Vec<GString> = self
			.entry_list
			.borrow()
			.iter()
			.map(|row| row.entry.text())
			.filter(|text| !text.as_str().trim().is_empty())
			.collect();

		xs.append(&mut ys);

		return xs;
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

	fn add_row_with(&self, label: &str, w: &impl IsA<Widget>) {
		let row_layout = GtkBox::new(Orientation::Horizontal, FAV_SPACING);
		let label = Label::new(Some(label));
		row_layout.pack_start(&label, false, false, 0);
		self.size_group.add_widget(&label);

		row_layout.pack_start(w, true, true, 0);
		self.layout.pack_start(&row_layout, false, true, 10);
	}

	fn add_row(&self, label: &str) -> Entry {
		let entry = Entry::new();
		self.add_row_with(label, &entry);
		return entry;
	}

	fn add_row_entryc(&self, label: &str) -> EntryC {
		let entryc = EntryC::new();
		self.add_row_with(label, entryc.as_ref());
		return entryc;
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
		return mutil_entry;
	}
}

pub struct MetaForm {
	pub layout: GtkBox,
	title_entry: Entry,
	artist_entry: MultiEntry,
	album_entry: EntryC,
	genre_entry: MultiEntry,
}

impl MetaForm {
	pub fn new() -> Self {
		let layout = GtkBox::builder().orientation(Orientation::Vertical).build();

		let form_row = FormRow::new();

		let title_entry = form_row.add_row("标题");
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
			.set_text_list(&state.tag.artists().unwrap_or(vec![]));
		self.album_entry.set_text(state.tag.album().unwrap_or(""));
		self.genre_entry
			.set_text_list(&state.tag.genres().unwrap_or(vec![]));
	}

	pub fn form_data(&self) -> MetaFormData {
		let title = self.title_entry.text();
		let artist = self.artist_entry.get_text_list();
		let album = self.album_entry.as_ref().text();
		let genre = self.genre_entry.get_text_list();

		MetaFormData {
			title,
			artist,
			album,
			genre,
		}
	}
}
