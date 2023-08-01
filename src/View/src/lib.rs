#![feature(read_buf)]

use std::fmt::{Display, Formatter};

use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use dialoguer::{Input, Password, Select};
use dialoguer::console::Term;
use indicatif::ProgressBar;

pub mod db_view;

/*
单独模块不接受其他库
*/

///#风格
pub enum Colour {
	///错误
	Error,
	///输出
	Output,
	///命令
	Order,
	///监控
	Monitoring,
	///函数
	Function,
}

impl Display for Colour {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Colour::Error => { write!(f, "ERROR") }
			Colour::Output => { write!(f, "OUTPUT") }
			Colour::Order => { write!(f, "ORDER") }
			Colour::Monitoring => { write!(f, "MONITORING") }
			Colour::Function => { write!(f, "FUNCTION") }
		}
	}
}

///#数据
pub struct Information<EF: Display, ER: IntoIterator<Item = EF>, GL: IntoIterator<Item = ER>> {
	///列表
	pub list: ER,
	///数据
	pub data: GL,
}

///# 显示
pub trait ViewDrive {
	type Frame;
	const SIGNAL: u64 = 100;
	fn view(&self) -> Self::Frame;
	fn table<EF: Display, ER: IntoIterator<Item = EF>, GL: IntoIterator<Item = ER>>(
		&self,
		_: Information<EF, ER, GL>,
	) -> Table where Self: Display {
		println!("{}", self);
		Table::new()
	}
	///# 进度条
	fn view_column() -> ProgressBar {
		ProgressBar::new(Self::SIGNAL)
	}
	///# 输入
	fn input_column(position: Vec<String>) -> std::io::Result<String> {
		let mut os = Input::<String>::new();
		position.into_iter().for_each(|e| { os.with_prompt(e); });
		os.interact()
	}
	///# 选择输入
	fn select_column(position: Vec<&str>, rows: Option<Vec<&str>>) -> std::io::Result<usize> {
		let mut os = Select::new();
		os.items(&position);
		if let Some(ro) = rows {
			ro.into_iter().for_each(|e| {
				os.with_prompt(e);
			})
		}
		os.default(0);
		os.interact_on_opt(&Term::buffered_stderr())?;
		os.interact()
	}
	///# 密码
	fn password_column(position: Vec<&str>) -> std::io::Result<String> {
		let mut password = Password::new();
		position.into_iter().for_each(|e| {
			password.with_prompt(e);
		});
		password.interact()
	}
}

impl ViewDrive for Colour {
	type Frame = Frames;
	fn view(&self) -> Self::Frame {
		match self {
			Colour::Error => Frames {
				text: Attribute::Italic,
				frames: Color::DarkRed,
			},
			Colour::Output => Frames {
				text: Attribute::Bold,
				frames: Color::DarkGreen,
			},
			Colour::Order => Frames {
				text: Attribute::RapidBlink,
				frames: Color::DarkYellow,
			},
			Colour::Monitoring => Frames {
				text: Attribute::Underlined,
				frames: Color::DarkCyan,
			},
			Colour::Function => Frames {
				text: Attribute::Reverse,
				frames: Color::DarkGrey,
			},
		}
	}
	fn table<EF: Display, ER: IntoIterator<Item = EF>, GL: IntoIterator<Item = ER>>(
		&self,
		e: Information<EF, ER, GL>,
	) -> Table {
		let i = Colour::view(self);
		let mut table = Table::new();
		table
			.load_preset(UTF8_FULL)
			.apply_modifier(UTF8_ROUND_CORNERS)
			.set_content_arrangement(ContentArrangement::Dynamic);
		table.set_header(
			e.list
				.into_iter()
				.map(|x| Cell::new(x).add_attribute(i.text).fg(i.frames))
				.collect::<Vec<_>>(),
		);
		e.data.into_iter().for_each(|x| {
			table.add_row(
				x.into_iter()
					.map(|x| Cell::new(x).add_attribute(i.text).fg(i.frames))
					.collect::<Vec<_>>(),
			);
		});
		table
	}
}

///#画面数据
pub struct Frames {
	//文本
	text: Attribute,
	//单元格前景色
	frames: Color,
}
