use std::ops::Deref;
use anyhow::anyhow;
use Error::ThreadEvents;
use Install::setting::local_config::SUPER_URL;
use Static::Events;
use View::{Colour, Information, ViewDrive};
use crate::test::test_get_db;

pub async fn init() -> Events<()> {
    db_build().await?;
    Ok(())
}

//生成表
async fn db_build() -> Events<()> {
    let x = test_get_db().await?;
    match x.is_empty() {
        true => {}
        false => {
            match Colour::view_container("Postgres数据表损坏是否新建？") {
                Ok(e) => {
                    match e {
                        true => {
                            let mut xr = vec![];
                            for e in x {
                                xr.push(format!("{}", SUPER_URL.deref().load().postgres.connect_rab_execute(e).await?));
                            }
                            println!("{}", Colour::Monitoring.table(Information { list: vec!["结果".to_string()], data: vec![xr] }));
                        }
                        false => { return Err(ThreadEvents::UnknownError(anyhow!("安全退出"))); }
                    }
                }
                Err(e) => { return Err(ThreadEvents::IoError(e)); }
            }
        }
    }
    Ok(())
}