use id3::frame::Picture;

/// 我想要从一张音频中拿到哪些信息
#[derive(Debug)]
pub struct AudioTag {
	pub cover: Option<Picture>,
	pub artist: Option<String>,
	pub album: Option<String>,
	pub title: Option<String>,
	pub genre: Option<String>,
}
