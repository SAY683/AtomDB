use std::fmt::Display;

use dialoguer::Input;

use crate::ViewDrive;

///# 数据输入
#[derive(Debug, Copy, Clone)]
pub struct InputStream<EF: Display, ER: IntoIterator<Item = EF>> {
	pub view: ER,
}

impl<EF: Display, ER: IntoIterator<Item = EF> + 'static> ViewDrive for InputStream<EF, ER> {
	type Frame = Input<'static, ER>;
	fn view(&self) -> Self::Frame {
		todo!()
	}
}
