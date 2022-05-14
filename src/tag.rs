use std::path::Path;

use id3::{frame::PictureType, Result, Tag, TagLike};

use crate::t::AudioTag;

pub fn read_audio_tag_from_path<P: AsRef<Path>>(path: P) -> Result<AudioTag> {
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
