#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]

extern crate core;

use std::env::current_dir;
use std::env::current_exe;
use std::path::PathBuf;

use anyhow::Result;
use lazy_static::lazy_static;

pub mod system;
pub mod io;
pub mod setting;
pub mod sql_url;
pub mod tables;


///# 发布
pub const NTS: bool = false;
///# 文件
const DATA: &str = "Data";
const DATA_DB: &str = "Atomic";

const BIN: &str = "Bin";
const SYSTEM_FILE: &str = "atom.toml";
const SYSTEM_DIR_FILE: &str = "atomic.toml";

const LOGS: &str = "Logs";
const LOGS1: &str = "atomic.logs";
//文件路径
lazy_static! {
    //log位置
    pub static ref LOCAL_BIN_LOGS: PathBuf = {
        let mut x = PathBuf::new();
        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
        x.push(LOGS);
        x.push(LOGS1);
        x
    };
    //设置位置
    pub static ref LOCAL_BIN_FILR: PathBuf = {
        let mut x = PathBuf::new();
        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
        x.push(BIN);
        x.push(SYSTEM_FILE);
        x
    };
       pub static ref LOCAL_BIN_DIR_FILR: PathBuf = {
        let mut x = PathBuf::new();
        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
        x.push(BIN);
        x.push(SYSTEM_DIR_FILE);
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
