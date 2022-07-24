//! 发送者
use gtk::glib;

use std::ops::Deref;

pub enum EmitEvent {
	/// 打开新音频
	OpenTag(id3::Tag),
	/// 保存信息到音频
	Save,
	/// 交互信息
	Alert(Result<String, String>),
}

pub enum CoverMimeType {
	PNG,
	JPEG,
}

impl AsRef<str> for CoverMimeType {
	fn as_ref(&self) -> &str {
		match self {
			CoverMimeType::PNG => "image/png",
			_ => "image/jpeg",
		}
	}
}

impl ToString for CoverMimeType {
	fn to_string(&self) -> String {
		self.as_ref().into()
	}
}

pub struct Emitter(glib::Sender<EmitEvent>);

impl Emitter {
	pub fn new(tx: glib::Sender<EmitEvent>) -> Self {
		Self(tx)
	}

	pub fn error<S: ToString>(&self, msg: S) {
		self.0.send(EmitEvent::Alert(Err(msg.to_string()))).unwrap();
	}

	pub fn info<S: ToString>(&self, msg: S) {
		self.0.send(EmitEvent::Alert(Ok(msg.to_string()))).unwrap();
	}

	pub fn alert<S1: ToString, S2: ToString>(&self, msg: Result<S1, S2>) {
		let s = msg.map(|s| s.to_string()).map_err(|e| e.to_string());
		self.0.send(EmitEvent::Alert(s)).unwrap();
	}

	pub fn send(&self, event: EmitEvent) {
		self.0.send(event).unwrap();
	}
}

impl Deref for Emitter {
	type Target = glib::Sender<EmitEvent>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
