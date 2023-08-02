pub use rayon::prelude::*;
use tokio::main;

use Static::Events;
use View::{Colour, ViewDrive};

///# 发布时[Install::NTS]=true;测试时保持
/// sea-orm-cli generate entity -u postgresql://root:683S@y683@localhost:5432/atomic -o ./src/Install/src/tables
///#
#[main]
pub async fn main() -> Events<()> {
	Colour::select_funz_column(&vec!["12", "24"]).unwrap();
	Ok(())
}
