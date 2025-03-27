use std::path::PathBuf;

pub enum EventAction {
	/// 打开音频。
	OpenFile(PathBuf),
}
