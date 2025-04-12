use std::{ops::Deref, path::Path};

use gtk::{
	prelude::{BoxExt, ButtonExt, InfoBarExt, LabelExt, WidgetExt},
	Button, InfoBar, Label, MessageType,
};

pub struct AlertBar {
	bar: InfoBar,
	label: Label,
}

impl AlertBar {
	pub fn new() -> Self {
		let bar = InfoBar::builder()
			.show_close_button(true)
			.visible(false)
			.build();

		bar.connect_response(|infobar, _| {
			infobar.hide();
		});

		let label = Label::new(None);
		bar.content_area().pack_start(&label, false, false, 0);

		Self { bar, label }
	}

	pub fn show<T: AsRef<str>>(&self, typ: MessageType, msg: T) {
		self.bar.set_message_type(typ);
		self.label.set_text(msg.as_ref());
		self.bar.show();
	}
}

impl Deref for AlertBar {
	type Target = InfoBar;

	fn deref(&self) -> &Self::Target {
		&self.bar
	}
}

struct ParseAudioInfo<'a> {
	title: Option<&'a str>,
	author: Option<&'a str>,
}

fn parse_audio_info<'a, P: AsRef<Path> + ?Sized + 'a>(path: &'a P) -> ParseAudioInfo<'a> {
	if let Some(filename) = path.as_ref().file_name().and_then(|s| s.to_str()) {
		let mut cs = filename.char_indices();

		const LEFT_P: char = '《';
		const LEFT_LEN: usize = LEFT_P.len_utf8();
		const RIGHT_P: char = '》';
		const RIGHT_LEN: usize = RIGHT_P.len_utf8();

		let left_title = cs.find(|(_, c)| *c == LEFT_P).map(|(i, _)| i);
		let right_title = cs.find(|(_, c)| *c == RIGHT_P).map(|(i, _)| i);
		let dot = cs.find(|(_, c)| *c == '.').map(|(i, _)| i);

		let title: Option<&str> = if let Some((left, right)) = left_title.zip(right_title) {
			Some(&filename[left + LEFT_LEN..right])
		} else {
			Some(&filename)
		};

		let author: Option<&str> = if let Some(right) = right_title {
			let xs = {
				if let Some(dot) = dot {
					&filename[right + RIGHT_LEN..dot]
				} else {
					&filename[right + RIGHT_LEN..]
				}
			};

			Some(xs).filter(|s| !s.is_empty())
		} else {
			None
		};

		ParseAudioInfo { title, author }
	} else {
		ParseAudioInfo {
			title: None,
			author: None,
		}
	}
}

pub struct PathBar {
	bar: InfoBar,
	btn: Button,
	label: Label,
}

impl PathBar {
	pub fn new() -> Self {
		let bar = InfoBar::builder()
			.show_close_button(false)
			.message_type(MessageType::Other)
			.build();
		let label = Label::new(None);
		bar.content_area().pack_start(&label, true, true, 0);

		let set_btn = Button::builder().label("应用预设信息").build();
		bar.content_area().pack_end(&set_btn, false, false, 0);

		Self {
			bar,
			label,
			btn: set_btn,
		}
	}

	pub fn set_text(&self, text: &str) {
		self.label.set_text(text);
	}

	pub fn connect_apply<F>(&self, f: F)
	where
		F: Fn((Option<String>, Option<String>)) + 'static,
	{
		self.btn.connect_clicked({
			let label = self.label.clone();
			move |_| {
				let text = label.text();
				let a = parse_audio_info(text.as_str());
				f((a.title.map(String::from), a.author.map(String::from)))
			}
		});
	}
}

impl Deref for PathBar {
	type Target = InfoBar;

	fn deref(&self) -> &Self::Target {
		&self.bar
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_normal_path() {
		let a = parse_audio_info("/a/b");
		assert_eq!(Some("b"), a.title);
		assert_eq!(None, a.author);

		let a = parse_audio_info("/a/a《今》");
		assert_eq!(Some("今"), a.title);
		assert_eq!(None, a.author);

		let a = parse_audio_info("/a/从今《不一足》今天下.mp3");
		assert_eq!(Some("不一足"), a.title);
		assert_eq!(Some("今天下"), a.author);
	}
}
