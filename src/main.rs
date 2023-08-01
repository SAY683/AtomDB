pub use rayon::prelude::*;
use tokio::main;

use Static::Events;
use View::{Colour, Information, ViewDrive};

use crate::database::build_graph_table_postgres;

pub mod database;

///# 发布时[Install::NTS]=true;测试时保持
/// sea-orm-cli generate entity -u postgresql://root:683S@y683@localhost:5432/atomic -o ./src/Install/src/tables
///#
#[main]
pub async fn main() -> Events<()> {
	build_graph_table_postgres().await?;
	println!("{}", Colour::Function.table(Information { list: ["12"], data: [["12"]] }));
	Ok(())
}
