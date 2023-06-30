use lazy_static::lazy_static;

use crate::setting::{database_config::DATABASE_BUILD_DIR, local_config::SUPER_URL, service_config::SERVICE_BUILD_DIR};
use crate::sql_url::{DEFAULT_BUILD_DIR_MARIADB, DEFAULT_BUILD_DIR_POSTGRES, Url};

///# 创建数据库
const INQUIRE_BUILD_DIR_SERVER: &str = "select tablename from pg_tables where tablename ='service';";
const INQUIRE_BUILD_DIR_DATABASE: &str = "select tablename from pg_tables where tablename='database';";

///# 初始数据库必修
pub const STRING_BUILD_SCALAR_DIR: [&str; 2] = [DATABASE_BUILD_DIR, SERVICE_BUILD_DIR];
pub const JUDGEMENT: [&str; 2] = [INQUIRE_BUILD_DIR_DATABASE, INQUIRE_BUILD_DIR_SERVER];
lazy_static! {
	//默认数据url
	pub static ref DEFAULT_URL:String={
		SUPER_URL.deref().load().postgres.build_url_database(DEFAULT_BUILD_DIR_POSTGRES)
	};
	pub static ref MYSQL_URL:String={
		SUPER_URL.deref().load().postgres.build_url_database(DEFAULT_BUILD_DIR_MARIADB)
	};
}
pub mod service_config {
	use rbatis::crud;
	use serde::{Deserialize, Serialize};
	
	///# 创建结构
	pub const SERVICE_BUILD_DIR: &str = "
	create table service
(
    id         serial
        constraint service_database_id_fk
            references database
        primary key,
    get_port   text,
    illustrate jsonb
);";
	
	#[derive(Serialize, Deserialize, Clone, Debug)]
	pub struct ServiceConfig {}
	crud!(ServiceConfig{});
}

pub mod database_config {
	use rbatis::crud;
	use serde::{Deserialize, Serialize};
	
	///# 创建结构
	pub const DATABASE_BUILD_DIR: &str = "
	create table database
(
    id               serial
        constraint database_pk
            primary key,
    architecture     text not null,
    storage_location text not null,
    query_statements text not null
);";
	
	#[derive(Serialize, Deserialize, Clone, Debug)]
	pub struct DatabaseConfig {}
	crud!(DatabaseConfig{});
}

pub mod local_config {
	use arc_swap::ArcSwap;
	use once_cell::sync::Lazy;
	use serde::{Deserialize, Serialize};
	
	use crate::LOCAL_BIN_FILR;
	use crate::sql_url::{MysqlUlr, PostgresUlr, RedisUlr};
	use crate::system::{InstallUtils, Toml};
	
	pub static SUPER_URL: Lazy<ArcSwap<LocalConfig>> = Lazy::new(|| { ArcSwap::from_pointee(LocalConfig::toml_build(LOCAL_BIN_FILR.as_path()).unwrap()) });
	
	#[derive(Serialize, Deserialize, Clone, Debug)]
	pub struct LocalConfig {
		pub postgres: PostgresUlr,
		pub mysql: MysqlUlr,
		pub redis: RedisUlr,
		pub mariadb: MysqlUlr,
	}
	
	impl InstallUtils for LocalConfig {}
	
	impl Toml for LocalConfig {}
}