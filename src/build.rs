use std::ops::Deref;
use anyhow::anyhow;
use lazy_static::lazy_static;
use Error::ThreadEvents;
use Install::setting::local_config::SUPER_URL;
use Static::Events;
use View::{Colour, Information, ViewDrive};
use crate::test::test_get_db;

const OUT_LOG: &str = "事件";
lazy_static! {
    pub static ref EVENTS: Information<String,Vec<String>>=Information{};
}

pub async fn init() -> Events<()> {
    if db_build().await? {} else {}
    Ok(())
}

//生成表
async fn db_build() -> Events<bool> {
    let xe = test_get_db().await?;
    return match xe.is_empty() {
        true => { Ok(true) }
        false => {
            match Colour::view_container("Postgres数据表损坏是否新建？") {
                Ok(e) => {
                    match e {
                        true => {
                            let mut xr = vec![];
                            for e in xe {
                                xr.push(format!("{}", SUPER_URL.deref().load().postgres.connect_rab_execute(e).await?));
                            }
                            println!("{}", Colour::Monitoring.table(Information { list: vec![OUT_LOG.to_string()], data: vec![xr] }));
                            Ok(true)
                        }
                        false => { Err(ThreadEvents::UnknownError(anyhow!("安全退出"))) }
                    }
                }
                Err(e) => { Err(ThreadEvents::IoError(e)) }
            }
        }
    };
}