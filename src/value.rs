use std::path::{Path, PathBuf};

use gtk::{
	glib::{uri_unescape_string, GString},
	SelectionData,
};
use id3::{
	frame::{Picture, PictureType},
	Content, TagLike,
};

/// 我们最爱的间距。
pub const FAV_SPACING: i32 = 10;

pub struct MetaFormData {
	pub title: GString,
	pub artist: Vec<GString>,
	pub album: GString,
	pub genre: Vec<GString>,
}

pub enum CoverMimeType {
	Png,
	Jpeg,
}

impl CoverMimeType {
	pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
		path.as_ref()
			.extension()
			.filter(|s| s == &"png")
			.map(|_| CoverMimeType::Png)
			.unwrap_or(CoverMimeType::Jpeg)
	}

	pub fn from_mime_type<S: AsRef<str>>(t: S) -> Self {
		match t.as_ref() {
			"mine/png" => CoverMimeType::Png,
			_ => CoverMimeType::Jpeg,
		}
	}
}

impl AsRef<str> for CoverMimeType {
	fn as_ref(&self) -> &str {
		match self {
			CoverMimeType::Png => "image/png",
			_ => "image/jpeg",
		}
	}
}

impl std::fmt::Display for CoverMimeType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}

/// 最终在保存的数据
pub struct SaveData<'a> {
	/// 所有文本记录的信息
	pub base: MetaFormData,
	/// 封面
	pub cover: Option<(&'a CoverMimeType, Vec<u8>)>,
}

/// 全局状态
pub struct AppState {
	pub tag: id3::Tag,
	pub audio_path: PathBuf,
}

impl AppState {
	pub fn front_cover(&self) -> Option<&Picture> {
		self.tag
			.pictures()
			.find(|p| p.picture_type == PictureType::CoverFront)
	}

	pub fn save<'a>(&'a mut self, data: SaveData<'a>) -> id3::Result<()> {
		if let Some((mime_type, pic_data)) = data.cover {
			let pic = id3::frame::Picture {
				mime_type: mime_type.to_string(),
				picture_type: PictureType::CoverFront,
				description: String::from(""),
				data: pic_data,
			};

			self.tag.add_frame(pic);
		} else {
			self.tag.remove_picture_by_type(PictureType::CoverFront);
		}

		self.tag.set_title(data.base.title);
		{
			let c = Content::new_text_values(data.base.artist);
			self.tag.set_artist(c.text().unwrap_or(""));
		}
		self.tag.set_album(data.base.album);
		{
			let c = Content::new_text_values(data.base.genre);
			self.tag.set_genre(c.text().unwrap_or(""));
		}

		let version = self.tag.version();

		self.tag.write_to_path(&self.audio_path, version)
	}
}

pub struct AppStateBox(pub (Option<String>, AppState));

impl TryFrom<PathBuf> for AppStateBox {
	type Error = id3::Error;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		match id3::Tag::read_from_path(&path) {
			Ok(tag) => Ok(Self((
				None,
				AppState {
					tag,
					audio_path: path,
				},
			))),
			Err(e) => {
				if e.partial_tag.is_none() {
					// 无法解析出tag
					Ok(Self((
						Some("无法解析tag，我亲自为你生成一个！".into()),
						AppState {
							tag: Default::default(),
							audio_path: path,
						},
					)))
				} else {
					Err(e)
				}
			}
		}
	}
}

pub fn get_drag_drop_filepath(sel: &SelectionData) -> Option<PathBuf> {
	let uris = sel.uris();
	let file = uris.first()?;
	let path = uri_unescape_string(file, None::<&str>)?;
	let path = Path::new(path.as_str().strip_prefix("file://")?);
	dbg!(&path);
	if path.exists() && path.extension()? == "mp3" {
		Some(path.into())
	} else {
		None
	}
}
