pub mod test;
mod build;

use fast_log::Config;
pub use rayon::prelude::*;
use tokio::main;
use Install::LOCAL_BIN_LOGS;
use Static::{Events};
use crate::build::init;
use crate::build::log::log_info_stop;

///# 发布时[Install::NTS]=true;测试时保持
///sea-orm-cli generate entity -u postgresql://postgres:683683say@localhost:5432/postgres -o ./src/Install/src/tables --with-serde both
#[main]
pub async fn main() -> Events<()> {
    fast_log::init(Config::new().file(LOCAL_BIN_LOGS.as_path().to_str().unwrap()).console())?;
    init().await?;
    log_info_stop();
    Ok(())
}
