use std::path::PathBuf;

pub struct ParseBox {
	pub audio_tag: id3::Tag,
	pub audio_src: PathBuf,
}

impl ParseBox {
	pub fn parse_from_path(path: PathBuf) -> id3::Result<ParseBox> {
		match id3::Tag::read_from_path(&path) {
			Ok(tag) => Ok(Self {
				audio_src: path,
				audio_tag: tag,
			}),
			Err(e) => Err(e),
		}
	}
}
