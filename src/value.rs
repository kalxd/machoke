use futures::channel::mpsc;
use gtk::MessageType;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};

type AlertMessageBox = (MessageType, String);

pub enum EventAction {
	OpenAudio(PathBuf),
	Alert(AlertMessageBox),
	Close,
}

pub enum CoverMimeType {
	Png,
	Jpg,
}

impl CoverMimeType {
	pub const fn as_mime_type(&self) -> &'static str {
		match self {
			Self::Jpg => "image/jpeg",
			Self::Png => "image/png",
		}
	}
}

pub fn read_picture_from_path<P: AsRef<Path>>(path: P) -> std::io::Result<id3::frame::Picture> {
	let file_ext = {
		if path.as_ref().extension().and_then(|s| s.to_str()) == Some("png") {
			CoverMimeType::Png
		} else {
			CoverMimeType::Jpg
		}
	};

	let content = fs::read(path)?;

	Ok(id3::frame::Picture {
		description: String::from(""),
		data: content,
		mime_type: file_ext.as_mime_type().to_string(),
		picture_type: id3::frame::PictureType::CoverFront,
	})
}

pub struct ParseBox {
	pub audio_tag: id3::Tag,
	pub audio_src: PathBuf,
}

impl ParseBox {
	pub fn parse_from_path(path: PathBuf) -> id3::Result<(Self, Option<AlertMessageBox>)> {
		match id3::Tag::read_from_path(&path) {
			Ok(tag) => Ok((
				Self {
					audio_src: path,
					audio_tag: tag,
				},
				None,
			)),
			Err(e) if e.partial_tag.is_none() => Ok((
				Self {
					audio_src: path,
					audio_tag: id3::Tag::default(),
				},
				Some((
					MessageType::Warning,
					String::from("无法解析tag，我亲自为你生成一个！"),
				)),
			)),
			Err(e) => Err(e),
		}
	}

	pub fn front_cover(&self) -> Option<&id3::frame::Picture> {
		self.audio_tag
			.pictures()
			.find(|p| p.picture_type == id3::frame::PictureType::CoverFront)
	}
}

#[derive(Clone)]
pub struct EventSender(mpsc::Sender<EventAction>);

impl EventSender {
	pub fn send(&self, action: EventAction) {
		self.0.clone().start_send(action).unwrap();
	}

	pub fn alert<S: ToString>(&self, t: MessageType, msg: S) {
		self.send(EventAction::Alert((t, msg.to_string())))
	}

	pub fn warn<S: ToString>(&self, msg: S) {
		self.alert(MessageType::Warning, msg)
	}

	pub fn error<S: ToString>(&self, msg: S) {
		self.alert(MessageType::Error, msg)
	}
}

impl Deref for EventSender {
	type Target = mpsc::Sender<EventAction>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub fn channel() -> (EventSender, mpsc::Receiver<EventAction>) {
	let (a, b) = mpsc::channel(10);
	(EventSender(a), b)
}
