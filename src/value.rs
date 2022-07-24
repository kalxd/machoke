use std::path::{Path, PathBuf};

use gtk::glib::GString;
use id3::frame::{Picture, PictureType};

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
