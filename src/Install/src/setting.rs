use crate::setting::{database_config::DATABASE_BUILD_DIR};
use crate::setting::database_config::SERVICE_BUILD_DIR;

///# 查询数据库
const INQUIRE_BUILD_DIR_SERVER: &str = "select tablename from pg_tables where tablename ='service'";
const INQUIRE_BUILD_DIR_DATABASE: &str = "select tablename from pg_tables where tablename='database'";

///# 初始数据库必修
pub const JUDGEMENT: [(&str, &str); 2] = [(INQUIRE_BUILD_DIR_DATABASE, DATABASE_BUILD_DIR), (INQUIRE_BUILD_DIR_SERVER, SERVICE_BUILD_DIR)];

pub mod database_config {
    use rbatis::crud;
    use sea_orm::{IdenStatic};
    use serde::{Deserialize, Serialize};

    ///# 创建结构
    pub const SERVICE_BUILD_DIR: &str = r#"
    create table service
(
);
    "#;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Service {}
    crud!(Service{});


    ///# 创建结构
    pub const DATABASE_BUILD_DIR: &str = r#"
    create table database
(
    UUID   uuid
        constraint database_pk
            primary key,
    "Name" text,
    "Hash" text,
    "Time" timestamp,
    constraint database_pk2
        unique ("Name", "Hash")
);
  "#;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct DatabaseConfig {}
    crud!(DatabaseConfig{});
}

pub mod local_config {
    use arc_swap::ArcSwap;
    use once_cell::sync::Lazy;
    use serde::{Deserialize, Serialize};

    use crate::{LOCAL_BIN_DIR_FILR, LOCAL_BIN_FILR};
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

    ///+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    pub static SUPER_DLR_URL: Lazy<ArcSwap<LocalConfigToml>> = Lazy::new(|| { ArcSwap::from_pointee(LocalConfigToml::toml_build(LOCAL_BIN_DIR_FILR.as_path()).unwrap()) });

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct LocalConfigToml {}

    impl InstallUtils for LocalConfigToml {}

    impl Toml for LocalConfigToml {}
}