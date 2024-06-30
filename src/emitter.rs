use futures::channel::mpsc::Sender;
use gtk::{gdk_pixbuf::Pixbuf, MessageType};
use std::path::PathBuf;

use crate::value::CoverMimeType;

pub enum EmitEvent {
	/// 打开新音频
	OpenTag(PathBuf),
	/// 保存信息到音频
	Save,
	/// 更改封面
	ChangeCover(PathBuf),
	/// 应用封面
	ApplyCover((Pixbuf, CoverMimeType)),
	/// 移除封面
	RemoveCover,
	/// 交互信息
	Alert((MessageType, String)),
}

#[derive(Clone)]
pub struct Emitter(Sender<EmitEvent>);

impl Emitter {
	pub fn new(tx: Sender<EmitEvent>) -> Self {
		Self(tx)
	}

	pub fn info<S: ToString>(&self, msg: S) {
		self.0
			.clone()
			.try_send(EmitEvent::Alert((MessageType::Info, msg.to_string())))
			.unwrap();
	}

	pub fn error<S: ToString>(&self, msg: S) {
		self.0
			.clone()
			.try_send(EmitEvent::Alert((MessageType::Error, msg.to_string())))
			.unwrap();
	}

	pub fn warn<S: ToString>(&self, msg: S) {
		self.0
			.clone()
			.try_send(EmitEvent::Alert((MessageType::Warning, msg.to_string())))
			.unwrap();
	}

	pub fn alert<S1: ToString, S2: ToString>(&self, msg: Result<S1, S2>) {
		match msg {
			Ok(s) => self.info(s),
			Err(e) => self.error(e),
		}
	}

	pub fn send(&self, event: EmitEvent) {
		self.0.clone().try_send(event).unwrap();
	}
}
