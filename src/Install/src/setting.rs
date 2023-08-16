use crate::setting::{database_config::DATABASE_BUILD_DIR};
use crate::setting::database_config::{SERVICE_BUILD_DIR, TYPE_EME};

///# 查询数据库
const INQUIRE_BUILD_DIR_SERVER: &str = "select tablename from pg_tables where tablename ='service'";
const INQUIRE_BUILD_DIR_DATABASE: &str = "select tablename from pg_tables where tablename='database'";
const INQUIRE_BUILD_DIR_TYPER: &str = r#"
select t.typname as "modes", pg_catalog.format_type(t.oid, NULL)
from pg_type t
where t.typname = 'modes';"#;

///# 初始数据库必修
pub const JUDGEMENT: [(&str, &str); 3] = [(INQUIRE_BUILD_DIR_DATABASE, DATABASE_BUILD_DIR), (INQUIRE_BUILD_DIR_SERVER, SERVICE_BUILD_DIR), (INQUIRE_BUILD_DIR_TYPER, TYPE_EME)];

pub mod database_config {
    use rbatis::{crud};
    use sea_orm::prelude::{DateTime, Json};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use crate::tables::sea_orm_active_enums::Modes;

    pub const TYPE_EME: &str = r#"create type modes as enum ('HASH', 'MAP', 'CACHE');"#;
    pub const DATABASE_BUILD_DIR: &str = r#"
    create table database
(
    name text not null,
    uuid uuid not null
        primary key,
    hash text not null
        unique,
    time timestamp
)
    using ???
  "#;
    ///# 创建结构
    pub const SERVICE_BUILD_DIR: &str = r#"
    create table service
(
    uuid  uuid  not null
        primary key
        constraint service_database_uuid_fk
            references database
            on update cascade on delete restrict,
    name  text  not null,
    port  inet  not null,
    frame json,
    mode  modes not null
)
    using ???;
    "#;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Database {
        pub name: String,
        pub uuid: Uuid,
        pub hash: String,
        pub time: Option<DateTime>,
    }
    crud!(Database{});

    ///# 创建结构
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Service {
        pub uuid: Uuid,
        pub name: String,
        pub port: String,
        pub frame: Option<Json>,
        pub mode: Modes,
    }
    crud!(Service{});
}

pub mod local_config {
    use std::net::{SocketAddr};
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
    pub struct LocalConfigToml {
        pub port: SocketAddr,
    }

    impl InstallUtils for LocalConfigToml {}

    impl Toml for LocalConfigToml {}
}