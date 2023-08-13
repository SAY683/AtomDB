pub mod test;
mod build;

use arc_swap::access::{Access, DynAccess};
pub use rayon::prelude::*;
use tokio::main;
use Install::io::DiskWrite;
use Static::{Alexia, Events};
use crate::build::init;

///# 发布时[Install::NTS]=true;测试时保持
///sea-orm-cli generate entity -u postgresql://postgres:683683say@localhost:5432/postgres -o ./src/Install/src/tables --with-serde both
#[main]
pub async fn main() -> Events<()> {
    init().await?;
    DiskWrite::run(DiskWrite::aggregation()).await?;
    println!("OK!");
    Ok(())
}
