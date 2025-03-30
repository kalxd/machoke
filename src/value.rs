use futures::channel::mpsc;
use gtk::{
	gdk_pixbuf::{Pixbuf, PixbufLoader},
	prelude::PixbufLoaderExt,
	MessageType,
};
use std::{ops::Deref, path::PathBuf};

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

	pub const fn as_str(&self) -> &'static str {
		match self {
			Self::Jpg => "jpeg",
			_ => "png",
		}
	}

	fn from_mime_type(lit: &str) -> Self {
		match lit {
			"mime/png" => Self::Png,
			_ => Self::Jpg,
		}
	}
}

pub struct SlimImage {
	raw_data: Vec<u8>,
	mime_type: CoverMimeType,
}

impl From<&id3::frame::Picture> for SlimImage {
	fn from(value: &id3::frame::Picture) -> Self {
		let raw_data = value.data.clone();
		let mime_type = CoverMimeType::from_mime_type(&value.mime_type);

		Self {
			raw_data,
			mime_type,
		}
	}
}

impl SlimImage {
	pub fn to_pixbuf(&self) -> Option<Pixbuf> {
		let loader = PixbufLoader::new();
		loader.write(&self.raw_data).ok()?;
		loader.close().ok()?;
		loader.pixbuf()
	}
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
