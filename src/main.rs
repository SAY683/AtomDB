pub mod test;
mod build;

pub use rayon::prelude::*;
use tokio::main;
use Static::{Events};
use crate::build::init;
use crate::build::log::log_info_stop;

///# 发布时[Install::NTS]=true;测试时保持
///sea-orm-cli generate entity -u postgresql://postgres:683683say@localhost:5432/postgres -o ./src/Install/src/tables --with-serde both
#[main]
pub async fn main() -> Events<()> {
    init().await?;
    log_info_stop();
    Ok(())
}
