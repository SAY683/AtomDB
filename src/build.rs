use std::ops::Deref;
use anyhow::anyhow;
use Error::ThreadEvents;
use Install::io::DiskWrite;
use Install::setting::local_config::{SUPER_URL};
use Static::{Alexia, Events};
use View::{Colour, Information, ViewDrive};
use crate::build::log::{log_info, ORD1, ORD2, OUT_LOG, OUT_LOG_1};
use crate::test::test_get_db;

pub async fn init() -> Events<()> {
    if db_build().await? {
        log_info();
        'life: loop {
            let index = vec![ORD1, ORD2];
            match index[Colour::select_func_column(&index, OUT_LOG_1).unwrap()] {
                ORD1 => {
                    //写入
                    DiskWrite::run(DiskWrite::aggregation()).await?;
                }
                ORD2 => {
                    //结束
                    break 'life;
                }
                e => {
                    println!("[{}]不存在", e);
                }
            }
        }
    } else {
        return Err(ThreadEvents::UnknownError(anyhow!("安全退出")));
    }
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
                                xr.push(vec![format!("{}", SUPER_URL.deref().load().postgres.connect_rab_execute(e).await?)]);
                            }
                            println!("{}", Colour::Monitoring.table(Information { list: vec![OUT_LOG.to_string()], data: xr }));
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

pub mod log {
    use std::ops::Deref;
    use Install::setting::local_config::SUPER_DLR_URL;
    use View::{Colour, Information, ViewDrive};

    pub const OUT_LOG: &str = "事件";
    pub const OUT_LOG_1: &str = "菜单";
    pub const ORD1: &str = "写入";
    pub const ORD2: &str = "结束";

    ///# 开始显示
    pub fn log_info() {
        println!("{}", Colour::Output.table(Information { list: vec!["AtomicDB".to_string()], data: vec![vec![format!("基本端口{}", SUPER_DLR_URL.deref().load().port.to_string())]] }))
    }

    ///# 结束显示
    pub fn log_info_stop() {
        println!("{}", Colour::Monitoring.table(Information { list: vec!["AtomicDB".to_string()], data: vec![vec![format!("事务结束")]] }))
    }
}