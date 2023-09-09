pub mod test;
mod build;

use actix_rt::spawn;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
pub use rayon::prelude::*;
use tokio::main;
use Install::web::web;
use Static::{Alexia, Events};
use crate::build::{Burden};

///# 发布时[Install::NTS]=true;测试时保持
///sea-orm-cli generate entity -u postgresql://postgres:683683say@localhost:5432/postgres -o ./src/Install/src/tables --with-serde both
#[main]
pub async fn main() -> Events<()> {
    Burden::run(Burden::aggregation()).await?;
    Ok(())
}

pub static WEB_BUILD: Lazy<ArcSwap<bool>> = Lazy::new(|| {
    ArcSwap::from_pointee(false)
});