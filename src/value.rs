use std::path::{Path, PathBuf};

use gtk::glib::GString;
use id3::{
	frame::{Picture, PictureType},
	TagLike,
};

pub struct MetaFormData {
	pub title: GString,
	pub artist: GString,
	pub album: GString,
	pub genre: GString,
}

pub enum CoverMimeType {
	PNG,
	JPEG,
}

impl CoverMimeType {
	pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
		path.as_ref()
			.extension()
			.filter(|ext| ext == &"png")
			.map(|_| CoverMimeType::PNG)
			.unwrap_or(CoverMimeType::JPEG)
	}

	pub fn from_mime_type<S: AsRef<str>>(t: S) -> Self {
		match t.as_ref() {
			"mine/png" => CoverMimeType::PNG,
			_ => CoverMimeType::JPEG,
		}
	}
}

impl AsRef<str> for CoverMimeType {
	fn as_ref(&self) -> &str {
		match self {
			CoverMimeType::PNG => "image/png",
			_ => "image/jpeg",
		}
	}
}

impl ToString for CoverMimeType {
	fn to_string(&self) -> String {
		self.as_ref().into()
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
		self.tag.set_artist(data.base.artist);
		self.tag.set_album(data.base.album);
		self.tag.set_genre(data.base.genre);

		let version = self.tag.version();

		self.tag.write_to_path(&self.audio_path, version)
	}
}

impl TryFrom<PathBuf> for AppState {
	type Error = id3::Error;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		let tag = id3::Tag::read_from_path(&path)?;

		Ok(Self {
			tag,
			audio_path: path,
		})
	}
}
