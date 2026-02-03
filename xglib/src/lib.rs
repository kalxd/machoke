use id3::{Result, Tag};

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
	println!("fuck!!!!");
	let tag = Tag::read_from_path(filepath)?;
	let value = Box::new(Media(tag));
	Ok(Box::into_raw(value))
}

fn say_hello() {
	println!("hello world");
}
