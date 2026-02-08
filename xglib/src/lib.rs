use id3::{ErrorKind as IdErrorKind, Result, Tag, TagLike};

#[cxx::bridge(namespace = "XGLib")]
pub mod ffi {
	#[derive(Debug)]
	enum CoverMime {
		Jpg,
		Png,
		None,
	}

	#[derive(Debug)]
	struct CoverTuple {
		mime: CoverMime,
		data: Vec<u8>,
	}

	#[derive(Debug)]
	struct SaveTagData {
		title: String,
		artists: Vec<String>,
		album: String,
	}

	extern "Rust" {
		type Media;

		#[cxx_name = "readAudioFile"]
		fn read_audio_file(filepath: &str) -> Result<Box<Media>>;

		fn front_cover(self: &Media) -> Box<CoverTuple>;
		fn title(self: &Media) -> String;
		fn artists(self: &Media) -> Vec<String>;
		fn album(self: &Media) -> String;
		fn genres(self: &Media) -> Vec<String>;
	}
}

struct Media(Option<Tag>);

fn read_audio_file(filepath: &str) -> Result<Box<Media>> {
	match Tag::read_from_path(filepath) {
		Ok(tag) => Ok(Box::new(Media(Some(tag)))),
		Err(id3::Error {
			kind: IdErrorKind::NoTag,
			..
		}) => Ok(Box::new(Media(None))),
		Err(e) => Err(e),
	}
}

impl Media {
	fn pick_front_cover(&self) -> Option<ffi::CoverTuple> {
		let tag = self.0.as_ref()?;

		tag.pictures()
			.filter_map(|pic| {
				if pic.picture_type != id3::frame::PictureType::CoverFront {
					return None;
				}

				if !["image/jpeg", "image/png"].contains(&pic.mime_type.as_str()) {
					return None;
				}

				let mime = match pic.mime_type.as_str() {
					"image/png" => ffi::CoverMime::Png,
					_ => ffi::CoverMime::Jpg,
				};

				Some(ffi::CoverTuple {
					mime,
					data: pic.data.clone(),
				})
			})
			.next()
	}

	fn front_cover(&self) -> Box<ffi::CoverTuple> {
		let a = self.pick_front_cover().unwrap_or_else(|| ffi::CoverTuple {
			mime: ffi::CoverMime::None,
			data: Vec::default(),
		});

		dbg!(&a.mime);

		Box::new(a)
	}

	fn title(&self) -> String {
		self.0
			.as_ref()
			.and_then(|s| s.title())
			.map(String::from)
			.unwrap_or_default()
	}

	fn artists(&self) -> Vec<String> {
		self.0
			.as_ref()
			.and_then(|s| s.artists())
			.map(|xs| xs.into_iter().map(String::from).collect())
			.unwrap_or_default()
	}

	fn album(&self) -> String {
		self.0
			.as_ref()
			.and_then(|s| s.album())
			.map(String::from)
			.unwrap_or_default()
	}

	fn genres(&self) -> Vec<String> {
		self.0
			.as_ref()
			.and_then(|s| s.genres())
			.map(|xs| xs.into_iter().map(String::from).collect())
			.unwrap_or_default()
	}
}
