//! 一堆常数

/// 目前支持的图片种类
pub enum ImageType {
	PNG,
	JPG,
}

impl ToString for ImageType {
	fn to_string(&self) -> String {
		match self {
			self::ImageType::PNG => "png".into(),
			self::ImageType::JPG => "jpg".into(),
		}
	}
}

impl ImageType {
	pub fn mine_type(&self) -> String {
		format!("image/{}", self.to_string())
	}
}
