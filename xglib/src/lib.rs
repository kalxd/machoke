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

		#[cxx_name = "saveAudioFile"]
		fn save_audio_file(media: &mut Media, value: SaveTagData) -> Result<()>;

		fn front_cover(self: &Media) -> Box<CoverTuple>;
		fn title(self: &Media) -> String;
		fn artists(self: &Media) -> Vec<String>;
		fn album(self: &Media) -> String;
		fn genres(self: &Media) -> Vec<String>;
	}
}

struct Media(Option<(Tag, std::path::PathBuf)>);

fn read_audio_file(filepath: &str) -> Result<Box<Media>> {
	let path = std::path::PathBuf::from(filepath);

	match Tag::read_from_path(&path) {
		Ok(tag) => Ok(Box::new(Media(Some((tag, path))))),
		Err(id3::Error {
			kind: IdErrorKind::NoTag,
			..
		}) => Ok(Box::new(Media(None))),
		Err(e) => Err(e),
	}
}

fn save_audio_file(media: &mut Media, value: ffi::SaveTagData) -> Result<()> {
	let (raw_tag, filepath) = media.0.as_mut().ok_or(id3::Error::new(
		id3::ErrorKind::InvalidInput,
		"无法保存未打开过的音频",
	))?;

	if !value.title.is_empty() {
		raw_tag.set_title(value.title);
	}

	raw_tag.write_to_path(filepath, raw_tag.version())
}

impl Media {
	fn pick_front_cover(&self) -> Option<ffi::CoverTuple> {
		let (tag, _) = self.0.as_ref()?;

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
			.and_then(|s| s.0.title())
			.map(String::from)
			.unwrap_or_default()
	}

	fn artists(&self) -> Vec<String> {
		self.0
			.as_ref()
			.and_then(|s| s.0.artists())
			.map(|xs| xs.into_iter().map(String::from).collect())
			.unwrap_or_default()
	}

	fn album(&self) -> String {
		self.0
			.as_ref()
			.and_then(|s| s.0.album())
			.map(String::from)
			.unwrap_or_default()
	}

	fn genres(&self) -> Vec<String> {
		self.0
			.as_ref()
			.and_then(|s| s.0.genres())
			.map(|xs| xs.into_iter().map(String::from).collect())
			.unwrap_or_default()
	}
}
