use std::{cell::RefCell, path::PathBuf, rc::Rc};

use gtk::ListStore;
use gtk::{glib::GString, prelude::*};
use id3::{
	frame::{Picture, PictureType},
	Tag, TagLike,
};

pub const GENRE_LIST: &[&'static str] = &[
	"袁派", "傅派", "王派", "戚派", "金派", "吕派", "张派", "范派", "徐派", "毕派", "陆派", "尹派",
];

/// 我想要从一张音频中拿到哪些信息
#[derive(Debug)]
pub struct AudioBaseInfo {
	pub cover: Option<Picture>,
	pub artist: Option<String>,
	pub album: Option<String>,
	pub title: Option<String>,
	pub genre: Option<String>,
}

impl From<&Tag> for AudioBaseInfo {
	fn from(tag: &Tag) -> Self {
		let pic = tag
			.pictures()
			.find(|p| p.picture_type == PictureType::CoverFront);

		let artist = tag.artist().map(Into::into);
		let album = tag.album().map(Into::into);
		let title = tag.title().map(Into::into);
		let genre = tag.genre().map(Into::into);

		Self {
			cover: pic.cloned(),
			artist,
			album,
			title,
			genre,
		}
	}
}

pub struct FormState {
	pub title: Option<GString>,
	pub artist: Option<GString>,
	pub album: Option<GString>,
	pub genre: Option<GString>,
}

/// 顶级状态，一个字：强！
#[derive(Clone, Debug)]
pub struct AppState {
	pub tag: Rc<RefCell<Option<Tag>>>,
	pub audio_path: Rc<RefCell<Option<PathBuf>>>,
	pub new_cover_path: Rc<RefCell<Option<PathBuf>>>,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			tag: Default::default(),
			audio_path: Default::default(),
			new_cover_path: Default::default(),
		}
	}
}

impl AppState {
	pub fn save(&self, form_state: &FormState) -> Result<(), String> {
		match self.tag.borrow_mut().as_mut() {
			None => return Ok(()),
			Some(tag) => {
				if let Some(title) = &form_state.title {
					tag.set_title(title.as_str());
				}

				if let Some(artist) = &form_state.artist {
					tag.set_artist(artist.as_str());
				}

				if let Some(album) = &form_state.album {
					tag.set_album(album.as_str());
				}

				if let Some(genre) = &form_state.genre {
					tag.set_genre(genre.as_str());
				}

				if let Some(cover_path) = self.new_cover_path.borrow().as_ref() {
					let data =
						std::fs::read(cover_path).map_err(|e| String::from(format!("{}", e)))?;
					tag.add_frame(Picture {
						mime_type: "image/png".into(),
						picture_type: PictureType::CoverFront,
						description: "".into(),
						data,
					});
				}

				if let Some(save_path) = self.audio_path.borrow().as_ref() {
					let version = tag.version();
					tag.write_to_path(save_path, version)
						.map_err::<String, _>(|e| format!("{}", e).into())?;
				}

				Ok(())
			}
		}
	}
}

#[derive(Clone)]
pub struct GenreStore {
	pub store: ListStore,
}

impl GenreStore {
	pub fn new() -> Self {
		let store = ListStore::new(&[gtk::glib::types::Type::STRING]);

		GENRE_LIST.iter().for_each(|name| {
			let iter = store.append();
			store.set(&iter, &[(0, name)]);
		});

		Self { store }
	}
}
