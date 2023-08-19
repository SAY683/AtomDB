pub mod test;
mod build;

use fast_log::Config;
pub use rayon::prelude::*;
use tokio::main;
use uuid::fmt::Urn;
use uuid::Uuid;
use Install::io::{Disk, KVStore};
use Install::LOCAL_BIN_LOGS;
use Install::setting::database_config::{Database, Test};
use Install::sql_url::{OrmEX, PostgresUlr};
use Static::{Events};
use crate::build::init;
use crate::build::log::log_info_stop;

///# 发布时[Install::NTS]=true;测试时保持
///sea-orm-cli generate entity -u postgresql://postgres:683683say@localhost:5432/postgres -o ./src/Install/src/tables --with-serde both
#[main]
pub async fn main() -> Events<()> {
    fast_log::init(Config::new().file(LOCAL_BIN_LOGS.as_path().to_str().unwrap()).console())?;
    init().await?;
    let xx = Urn::from_uuid(Uuid::new_v4()).into_uuid();
    let mut x = PostgresUlr::default().connect_bdc().await?;
    let time = KVStore::<String, String>::io_time();
    Database::insert(&mut x, &Database{
        uuid: xx,
        name: "12".to_string(),
        hash: "32".to_string(),
        time: None,
    }).await?;
    log_info_stop();
    Ok(())
}
