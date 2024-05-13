use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use futures::channel::mpsc::Sender;

use gtk::MessageType;

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

#[derive(Clone)]
pub struct Emitter(Arc<Mutex<Sender<EmitEvent>>>);

impl Emitter {
	pub fn new(tx: Sender<EmitEvent>) -> Self {
		Self(Arc::new(Mutex::new(tx)))
	}

	pub fn info<S: ToString>(&self, msg: S) {
		self.0
			.lock()
			.unwrap()
			.try_send(EmitEvent::Alert((MessageType::Info, msg.to_string())))
			.unwrap();
	}

	pub fn error<S: ToString>(&self, msg: S) {
		self.0
			.lock()
			.unwrap()
			.try_send(EmitEvent::Alert((MessageType::Error, msg.to_string())))
			.unwrap();
	}

	pub fn warn<S: ToString>(&self, msg: S) {
		self.0
			.lock()
			.unwrap()
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
		self.0.lock().unwrap().try_send(event).unwrap();
	}
}
