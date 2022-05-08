use std::path::Path;

use id3::{
	frame::{Picture, PictureType},
	Result, Tag, TagLike,
};

#[derive(Debug)]
pub struct AudioTag {
	pub cover: Option<Picture>,
	pub artist: Option<String>,
	pub album: Option<String>,
	pub title: Option<String>,
	pub genre: Option<String>,
}

pub fn read_cover_from_path<P: AsRef<Path>>(path: P) -> Result<AudioTag> {
	let tag = Tag::read_from_path(path)?;
	let pic = tag
		.pictures()
		.find(|p| p.picture_type == PictureType::CoverFront);

	let artist = tag.artist().map(Into::into);
	let album = tag.album().map(Into::into);
	let title = tag.album().map(Into::into);
	let genre = tag.genre().map(Into::into);

	Ok(AudioTag {
		cover: pic.cloned(),
		artist,
		album,
		title,
		genre,
	})
}
