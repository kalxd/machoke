//! 发送者
use std::path::PathBuf;

use gtk::glib;

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
	Alert(Result<String, String>),
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
