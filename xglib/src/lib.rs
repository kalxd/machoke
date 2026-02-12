use id3::{Content, Result, Tag, TagLike, frame};

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
		cover: CoverTuple,
		title: String,
		artists: Vec<String>,
		album: String,
		genres: Vec<String>,
	}

	extern "Rust" {
		type Media;
		type MediaPathInfo;

		#[cxx_name = "readAudioFile"]
		fn read_audio_file(filepath: &str) -> Box<Media>;

		#[cxx_name = "saveAudioFile"]
		fn save_audio_file(media: &mut Box<Media>, value: SaveTagData) -> Result<()>;

		fn front_cover(self: &Media) -> Box<CoverTuple>;
		fn title(self: &Media) -> String;
		fn artists(self: &Media) -> Vec<String>;
		fn album(self: &Media) -> String;
		fn genres(self: &Media) -> Vec<String>;

		#[cxx_name = "pathInfo"]
		fn path_info(self: &Media) -> Box<MediaPathInfo>;

		fn title(self: &MediaPathInfo) -> String;
		fn artist(self: &MediaPathInfo) -> String;
	}
}

struct Media((Tag, std::path::PathBuf));

struct PathInfo {
	title: String,
	artist: String,
}

struct MediaPathInfo(Option<PathInfo>);

fn read_audio_file(filepath: &str) -> Box<Media> {
	let path = std::path::PathBuf::from(filepath);

	match Tag::read_from_path(&path) {
		Ok(tag) => Box::new(Media((tag, path))),
		Err(_) => Box::new(Media((Tag::new(), path))),
	}
}

fn save_audio_file(media: &mut Box<Media>, value: ffi::SaveTagData) -> Result<()> {
	let (raw_tag, filepath) = &mut media.0;

	if value.cover.mime == ffi::CoverMime::None {
		raw_tag.remove_picture_by_type(frame::PictureType::CoverFront);
	} else {
		let mime = match value.cover.mime {
			ffi::CoverMime::Jpg => "image/jpeg",
			ffi::CoverMime::Png => "image/png",
			_ => panic!("不能保存mime为none的封面！"),
		};

		let pic = frame::Picture {
			description: String::default(),
			mime_type: mime.to_string(),
			picture_type: frame::PictureType::CoverFront,
			data: value.cover.data,
		};

		raw_tag.add_frame(pic);
	}

	if value.title.is_empty() {
		raw_tag.remove_title();
	} else {
		raw_tag.set_title(value.title);
	}

	if value.artists.is_empty() {
		raw_tag.remove_artist();
	} else {
		let c = Content::new_text_values(value.artists);
		raw_tag.set_artist(c.to_string());
	}

	if value.album.is_empty() {
		raw_tag.remove_album();
	} else {
		raw_tag.set_album(value.album);
	}

	if value.genres.is_empty() {
		raw_tag.remove_genre();
	} else {
		let c = Content::new_text_values(value.genres);
		raw_tag.set_genre(c.to_string());
	}

	raw_tag.write_to_path(filepath, raw_tag.version())
}

impl Media {
	fn pick_front_cover(&self) -> Option<ffi::CoverTuple> {
		let (tag, _) = &self.0;

		tag.pictures()
			.filter_map(|pic| {
				if pic.picture_type != frame::PictureType::CoverFront {
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

		Box::new(a)
	}

	fn title(&self) -> String {
		self.0.0.title().map(String::from).unwrap_or_default()
	}

	fn artists(&self) -> Vec<String> {
		self.0
			.0
			.artists()
			.map(|xs| xs.into_iter().map(String::from).collect())
			.unwrap_or_default()
	}

	fn album(&self) -> String {
		self.0.0.album().map(String::from).unwrap_or_default()
	}

	fn genres(&self) -> Vec<String> {
		self.0
			.0
			.genres()
			.map(|xs| xs.into_iter().map(String::from).collect())
			.unwrap_or_default()
	}

	fn path_info_simple(&self) -> Option<PathInfo> {
		let filename = self.0.1.file_prefix()?.to_str()?;

		match filename.strip_prefix('《') {
			Some(filename) => {
				let (title, artist) = filename.split_once('》')?;

				Some(PathInfo {
					title: String::from(title),
					artist: String::from(artist),
				})
			}

			None => Some(PathInfo {
				title: String::from(filename),
				artist: String::default(),
			}),
		}
	}

	fn path_info(&self) -> Box<MediaPathInfo> {
		Box::new(MediaPathInfo(self.path_info_simple()))
	}
}

impl MediaPathInfo {
	fn title(&self) -> String {
		self.0.as_ref().map(|s| s.title.clone()).unwrap_or_default()
	}

	fn artist(&self) -> String {
		self.0
			.as_ref()
			.map(|s| s.artist.clone())
			.unwrap_or_default()
	}
}
