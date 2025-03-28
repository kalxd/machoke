use super::store::MultiLineModel;
use gtk::ListBox;

pub struct MultiLine {
	list_box: ListBox,
	model: MultiLineModel,
}

impl MultiLine {
	pub fn new() -> Self {
		let model = MultiLineModel::new();

		let list_box = ListBox::new();

		Self { list_box, model }
	}
}
