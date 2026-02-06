use id3::{ErrorKind as IdErrorKind, Result, Tag, TagLike};

#[cxx::bridge(namespace = "XGLib")]
pub mod ffi {
	extern "Rust" {
		type Media;

		#[cxx_name = "readAudioFile"]
		fn read_audio_file(filepath: &str) -> Result<*mut Media>;

		#[cxx_name = "readMediaTitle"]
		unsafe fn read_media_title(media: *mut Media) -> String;
	}
}

struct Media(Tag);

fn read_audio_file(filepath: &str) -> Result<*mut Media> {
	match Tag::read_from_path(filepath) {
		Ok(tag) => {
			let b = Box::new(Media(tag));
			Ok(Box::into_raw(b))
		}
		Err(id3::Error {
			kind: IdErrorKind::NoTag,
			..
		}) => Ok(std::ptr::null_mut()),
		Err(e) => Err(e),
	}
}

fn read_media_title(media: *mut Media) -> String {
	let media = unsafe { Box::from_raw(media) };
	media.0.title().map(String::from).unwrap_or_default()
}
