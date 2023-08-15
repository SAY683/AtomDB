use crate::setting::{database_config::DATABASE_BUILD_DIR};
use crate::setting::database_config::SERVICE_BUILD_DIR;

///# 查询数据库
const INQUIRE_BUILD_DIR_SERVER: &str = "select tablename from pg_tables where tablename ='service'";
const INQUIRE_BUILD_DIR_DATABASE: &str = "select tablename from pg_tables where tablename='database'";

///# 初始数据库必修
pub const JUDGEMENT: [(&str, &str); 2] = [(INQUIRE_BUILD_DIR_DATABASE, DATABASE_BUILD_DIR), (INQUIRE_BUILD_DIR_SERVER, SERVICE_BUILD_DIR)];

pub mod database_config {
    use rbatis::{crud, impl_select};
    use sea_orm::prelude::{DateTime, Json};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    ///# 创建结构
    pub const SERVICE_BUILD_DIR: &str = r#"
    create table service
(
    "Uuid"        uuid not null
        constraint service_pk
            primary key
        constraint "service_database_Uuid_fk"
            references database ("Uuid")
            on update cascade on delete restrict,
    "ServicePort" inet,
    "Name"        text,
    "Framework"   json,
    constraint service_pk2
        unique ("Name", "ServicePort")
);
    "#;
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DatabaseConfig {
        pub name: String,
        pub uuid: Uuid,
        pub time: Option<DateTime>,
        pub hash: Option<String>,
    }
    crud!(DatabaseConfig{});

    ///# 创建结构
    pub const DATABASE_BUILD_DIR: &str = r#"
    create table database
(
    "Name" text not null,
    "Uuid" uuid not null
        unique,
    "Time" timestamp,
    "Hash" text,
    constraint database_pk
        primary key ("Name", "Uuid"),
    constraint database_pk2
        unique ("Time", "Hash")
);
  "#;
    #[derive(Clone, Debug, PartialEq,  Eq, Serialize, Deserialize)]
    pub struct ServiceConfig {
        pub uuid: Uuid,
        pub service_port: Option<String>,
        pub name: Option<String>,
        pub framework: Option<Json>,
    }
    crud!(ServiceConfig{});
    impl_select!(ServiceConfig{select_name(name:String) -> Option => "`where name = #{name}`"});
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