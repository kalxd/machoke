use futures::channel::mpsc;
use gtk::gdk_pixbuf::{Pixbuf, PixbufLoader};
use gtk::glib::{uri_unescape_string, GString};
use gtk::prelude::PixbufLoaderExt;
use gtk::{MessageType, SelectionData};
use id3::TagLike;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};

type AlertMessageBox = (MessageType, String);

pub enum EventAction {
	OpenAudio(PathBuf),
	Alert(AlertMessageBox),
	Close,
	Save,
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

	pub const fn as_extension(&self) -> &'static str {
		match self {
			Self::Jpg => "jpg",
			Self::Png => "png",
		}
	}

	pub fn from_mime_type(t: &str) -> Self {
		match t {
			"image/jpeg" | "image/jpg" => Self::Jpg,
			_ => Self::Png,
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

pub fn scale_picture(pic: &id3::frame::Picture, size: i32) -> Option<(Pixbuf, Pixbuf)> {
	let loader = PixbufLoader::new();
	loader.write(&pic.data).ok()?;
	loader.close().ok()?;
	let raw_pixbuf = loader.pixbuf()?;
	let scale_pixbuf = raw_pixbuf.scale_simple(size, size, gtk::gdk_pixbuf::InterpType::Nearest)?;

	Some((raw_pixbuf, scale_pixbuf))
}

#[derive(Debug)]
pub struct SaveBox {
	pub title: GString,
	pub artist: Vec<GString>,
	pub album: GString,
	pub genre: Vec<GString>,
	pub picture: Option<id3::frame::Picture>,
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

	pub fn save(&mut self, state: SaveBox) -> id3::Result<()> {
		if let Some(pic) = state.picture.as_ref() {
			self.audio_tag.add_frame(pic.clone());
		} else {
			self.audio_tag
				.remove_picture_by_type(id3::frame::PictureType::CoverFront);
		}

		self.audio_tag.set_title(state.title);
		{
			let c = id3::Content::new_text_values(state.artist);
			self.audio_tag.set_artist(c.text().unwrap_or_default());
		}

		self.audio_tag.set_album(state.album);
		{
			let c = id3::Content::new_text_values(state.genre);
			self.audio_tag.set_genre(c.text().unwrap_or_default());
		}

		let v = self.audio_tag.version();
		self.audio_tag.write_to_path(&self.audio_src, v)
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

pub fn get_drag_drop_filepath(sel: &SelectionData) -> Option<PathBuf> {
	let uris = sel.uris();
	let file = uris.first()?;
	let path = uri_unescape_string(file, None::<&str>)?;
	let path = Path::new(path.as_str().strip_prefix("file://")?);
	if path.exists() && path.extension()? == "mp3" {
		Some(path.into())
	} else {
		None
	}
}
