use id3::{ErrorKind as IdErrorKind, Result, Tag};

#[cxx::bridge(namespace = "XGLib")]
pub mod ffi {
	extern "Rust" {
		type Media;

		#[cxx_name = "sayHello"]
		fn say_hello();

		#[cxx_name = "readAudioFile"]
		fn read_audio_file(filepath: &str) -> Result<*const Media>;
	}
}

struct Media(Tag);

fn read_audio_file(filepath: &str) -> Result<*const Media> {
	match Tag::read_from_path(filepath) {
		Ok(tag) => {
			let b = Box::new(Media(tag));
			Ok(Box::into_raw(b))
		}
		Err(id3::Error {
			kind: IdErrorKind::NoTag,
			..
		}) => Ok(std::ptr::null()),
		Err(e) => Err(e),
	}
}

fn say_hello() {
	println!("hello world");
}
