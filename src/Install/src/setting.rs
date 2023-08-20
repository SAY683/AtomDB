use crate::setting::{database_config::DATABASE_BUILD_DIR};

///# 查询数据库[冻结]
// const INQUIRE_BUILD_DIR_SERVER: &str = r#"
// select tablename from pg_tables where tablename ='service'"#;
const INQUIRE_BUILD_DIR_DATABASE: &str = r#"
select tablename from pg_tables where tablename='database'"#;
// const INQUIRE_BUILD_DIR_TYPER: &str = r#"
// select typname
// from pg_type
// where typname = 'modes'"#;
// const INQUIRE_BUILD_DIR_TYPER2: &str = r#"
// select typname
// from pg_type
// where typname = 'key_value'"#;
// const INQUIRE_BUILD_DIR_TYPER3: &str = r#"
// select typname
// from pg_type
// where typname = 'modes_table';"#;

///# 初始数据库必修
pub const JUDGEMENT: [(&str, &str); 1] = [
    // (INQUIRE_BUILD_DIR_TYPER, TYPE_EME),
    // (INQUIRE_BUILD_DIR_TYPER2, TYPE_EME2),
    // (INQUIRE_BUILD_DIR_TYPER3, TYPE_EME3)
    (INQUIRE_BUILD_DIR_DATABASE, DATABASE_BUILD_DIR),
    //(INQUIRE_BUILD_DIR_SERVER, SERVICE_BUILD_DIR),
];

pub mod database_config {
    use rbatis::{crud};
    use serde::{Deserialize, Serialize};

    pub const TYPE_EME: &str = r#"
    create type modes as enum ('HASH', 'MAP', 'CACHE');"#;
    pub const TYPE_EME2: &str = r#"
    create type key_value as
(
    name text,
    hash text
);"#;
    pub const TYPE_EME3: &str = r#"
    create type modes_table as
(
    name text,
    mode modes
);"#;

    pub const DATABASE_BUILD_DIR: &str = r#"
    create table database
(
    uuid text not null
        constraint database_pk
            primary key,
    name text not null,
    hash text not null,
    port text not null,
    constraint database_pk2
        unique (name, hash, port)
);"#;
    ///# 创建结构 [冻结]
    pub const SERVICE_BUILD_DIR: &str = r#"
    create table service
(
    uuid text  not null
        constraint service_pk
            primary key
        constraint service_database_uuid_fk
            references database,
    name text  not null,
    port character  not null,
    logs xml,
    mode text not null,
    constraint service_pk2
        unique (name, port)
);"#;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Database {
        pub uuid: String,
        pub name: String,
        pub hash: String,
        pub port: String,
    }
    crud!(Database{});

    ///# 创建结构
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Test {
        pub name: String,
        pub tels: String,
    }
    crud!(Test{});
}

pub mod local_config {
    use std::net::{SocketAddr};
    use std::path::PathBuf;
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
        pub view: bool,
        pub apl: PathBuf,
    }

    impl InstallUtils for LocalConfigToml {}

    impl Toml for LocalConfigToml {}
}