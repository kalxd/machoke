//! 发送者
use std::path::PathBuf;

use gtk::{glib, MessageType};

pub enum EmitEvent {
	/// 打开新音频
	OpenTag(PathBuf),
	/// 保存信息到音频
	Save,
	/// 更改封面
	ChangeCover(PathBuf),
	/// 移除封面
	RemoveCover,
	/// 交互信息
	Alert((MessageType, String)),
}

pub struct Emitter(glib::Sender<EmitEvent>);

impl Emitter {
	pub fn new(tx: glib::Sender<EmitEvent>) -> Self {
		Self(tx)
	}

	pub fn info<S: ToString>(&self, msg: S) {
		self.0
			.send(EmitEvent::Alert((MessageType::Info, msg.to_string())))
			.unwrap();
	}

	pub fn error<S: ToString>(&self, msg: S) {
		self.0
			.send(EmitEvent::Alert((MessageType::Error, msg.to_string())))
			.unwrap();
	}

	pub fn warn<S: ToString>(&self, msg: S) {
		self.0
			.send(EmitEvent::Alert((MessageType::Warning, msg.to_string())))
			.unwrap();
	}

	pub fn alert<S1: ToString, S2: ToString>(&self, msg: Result<S1, S2>) {
		match msg {
			Ok(s) => self.info(s),
			Err(e) => self.error(e),
		}
	}

	pub fn send(&self, event: EmitEvent) {
		self.0.send(event).unwrap();
	}
}
