use id3::{Result, Tag};

#[cxx::bridge(namespace = "XGLib")]
pub mod ffi {
	extern "Rust" {
		type Media;

		#[cxx_name = "sayHello"]
		fn say_hello();

		#[cxx_name = "readAudioFile"]
		fn read_audio_file(filepath: &str) -> Result<Box<Media>>;
	}
}

struct Media(Tag);

fn read_audio_file(filepath: &str) -> Result<Box<Media>> {
	println!("fuck!!!!");
	let tag = Tag::read_from_path(filepath)?;
	Ok(Box::new(Media(tag)))
}

fn say_hello() {
	println!("hello world");
}
