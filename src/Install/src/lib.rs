#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]

use std::env::current_dir;
use std::env::current_exe;
use std::path::PathBuf;

use anyhow::Result;
use lazy_static::lazy_static;

pub mod system;
pub mod io;
pub mod setting;
pub mod sql_url;
//pub mod entities;


///# 发布
pub const NTS: bool = false;
///# 文件
const DATA: &str = "Data";
const DATA_DB: &str = "Atomic";

const BIN: &str = "Bin";
const SYSTEM_FILE: &str = "atom.toml";
//文件路径
lazy_static! {
    //设置位置
    pub static ref LOCAL_BIN_FILR: PathBuf = {
        let mut x = PathBuf::new();
        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
        x.push(BIN);
        x.push(SYSTEM_FILE);
        x
    };
    //默认存储位置
    pub static ref LOCAL_DEF_DB: PathBuf = {
        let mut x = PathBuf::new();
        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
        x.push(DATA);
        x.push(DATA_DB);
        x
    };
    //我的位置
    pub static ref LOCAL_PATH: Result<PathBuf> = Ok(if NTS {
        let mut et = current_exe()?;
        et.pop();
        et
    } else {
        current_dir()?
    });
}
