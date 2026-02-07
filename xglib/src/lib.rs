use id3::{ErrorKind as IdErrorKind, Result, Tag, TagLike};

#[cxx::bridge(namespace = "XGLib")]
pub mod ffi {
	extern "Rust" {
		type Media;

		#[cxx_name = "readAudioFile"]
		fn read_audio_file(filepath: &str) -> Result<Box<Media>>;

		fn title(self: &Media) -> String;
		fn album(self: &Media) -> String;
	}
}

struct Media(Option<Tag>);

fn read_audio_file(filepath: &str) -> Result<Box<Media>> {
	match Tag::read_from_path(filepath) {
		Ok(tag) => {
			dbg!(tag.title());
			Ok(Box::new(Media(Some(tag))))
		}
		Err(id3::Error {
			kind: IdErrorKind::NoTag,
			..
		}) => Ok(Box::new(Media(None))),
		Err(e) => Err(e),
	}
}

impl Media {
	fn title(&self) -> String {
		self.0
			.as_ref()
			.and_then(|s| s.title())
			.map(String::from)
			.unwrap_or_default()
	}

	fn album(&self) -> String {
		self.0
			.as_ref()
			.and_then(|s| s.album())
			.map(String::from)
			.unwrap_or_default()
	}
}
