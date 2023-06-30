use std::ops::Deref;

use sea_orm::{DatabaseBackend, DatabaseConnection, EntityTrait, IntoMockRow, MockDatabase};
use serde::{Deserialize, Serialize};

use Static::Events;
use Static::sql_orm::OrmEX;

use crate::setting::local_config::SUPER_URL;
use crate::tables::prelude::Database;

//use crate::entities::prelude::*;
/////# 数据库
//pub enum Table {
//	Server(Service),
//	Settings(Database)
//}

pub trait Url {
	fn build_url(&self) -> String;
	///# 数据库切换
	fn build_url_database(&self, e: &str) -> String {
		let et = self.build_url();
		let et = et.rsplitn(2, "/").collect::<Vec<_>>();
		let mut xx = String::from(et[1]);
		xx.push_str("/");
		xx.push_str(e);
		xx
	}
}

///# Mysql_Ulr
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MysqlUlr {
	pub start: bool,
	pub name: String,
	pub password: String,
	pub host: String,
	pub port: Option<String>,
	pub database: String,
}

impl Default for MysqlUlr {
	fn default() -> Self {
		MysqlUlr {
			database: DEFAULT_BUILD_DIR_POSTGRES.to_string(),
			..SUPER_URL.deref().load().mysql.clone()
		}
	}
}

impl Url for MysqlUlr {
	fn build_url(&self) -> String {
		format!(
			"mysql://{}:{}@{}:{}/{}",
			self.name.as_str(),
			self.password.as_str(),
			self.host.as_str(),
			{
				if let Some(ref port) = self.port {
					port.as_ref()
				} else {
					"3306"
				}
			},
			self.database
		)
	}
}

///# Redis_Ulr
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RedisUlr {
	pub start: bool,
	pub name: Option<String>,
	pub password: Option<String>,
	pub host: String,
	pub port: Option<String>,
	pub database: String,
}

impl Url for RedisUlr {
	///#产生
	///#redis://[<username>][:<password>@]<hostname>[:port][/<db>]
	fn build_url(&self) -> String {
		if self.name.is_some() || self.password.is_some() {
			format!(
				"redis://{}:{}@{}:{}/{}",
				self.name.as_ref().unwrap().as_str(),
				self.password.as_ref().unwrap().as_str(),
				self.host.as_str(),
				{
					if let Some(ref port) = self.port {
						port.as_ref()
					} else {
						"6379"
					}
				},
				self.database.as_str()
			)
		} else {
			format!("redis://{}:{}", self.host.as_str(), {
				if let Some(ref port) = self.port {
					port.as_ref()
				} else {
					"6379"
				}
			})
		}
	}
}

impl OrmEX for RedisUlr {
	fn url(&self) -> String {
		self.build_url()
	}
}

///# Postgres_Ulr
///# jdbc:postgresql://root:root@localhost:5432/postgres
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostgresUlr {
	pub start: bool,
	pub name: String,
	pub password: String,
	pub port: Option<String>,
	pub host: String,
	pub database: String,
}

impl Default for PostgresUlr {
	fn default() -> Self {
		PostgresUlr {
			database: DEFAULT_BUILD_DIR_POSTGRES.to_string(),
			..SUPER_URL.deref().load().postgres.clone()
		}
	}
}

impl OrmEX for PostgresUlr {
	fn url(&self) -> String {
		self.build_url()
	}
}

pub mod tables {}

pub trait SqlCommand {
	///# 默认数据库
	async fn default_psql<T, I, II>(e: II) -> Events<DatabaseConnection> where T: IntoMockRow
	, I: IntoIterator<Item = T>, II: IntoIterator<Item = I>, {
		Ok(MockDatabase::new(DatabaseBackend::Postgres)
			.append_query_results(e)
			.into_connection())
	}
	async fn default_myql<T, I, II>(e: II) -> Events<DatabaseConnection> where T: IntoMockRow
	, I: IntoIterator<Item = T>, II: IntoIterator<Item = I>, {
		Ok(MockDatabase::new(DatabaseBackend::MySql)
			.append_query_results(e)
			.into_connection())
	}
	async fn url_query() -> String;
}

impl Database {}

///# 默认数据库
pub const DEFAULT_BUILD_DIR_POSTGRES: &str = "postgres";
pub const DEFAULT_BUILD_DIR_MARIADB: &str = "localhost";

impl Url for PostgresUlr {
	fn build_url(&self) -> String {
		format!(
			"postgresql://{}:{}@{}:{}/{}",
			self.name.as_str(),
			self.password.as_str(),
			self.host.as_str(),
			{
				if let Some(ref port) = self.port {
					port.as_ref()
				} else {
					"5432"
				}
			},
			self.database,
		)
	}
}