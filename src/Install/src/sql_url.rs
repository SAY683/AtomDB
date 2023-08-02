use std::ops::Deref;

use deadpool_redis::{Config as Conf, Pool, Runtime};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, ExecResult, QueryResult, Statement};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Config, Connection, NoTls, Socket};
use tokio_postgres::tls::NoTlsStream;

use Static::Events;
use Static::static_array::Archive;

use crate::setting::local_config::SUPER_URL;

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

pub trait OrmEX {
	fn url(&self) -> String;
	//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
	async fn default_connect(e: String) -> Events<DatabaseConnection> {
		Ok(Database::connect(e).await?)
	}
	async fn connect(&self) -> Events<DatabaseConnection> {
		Ok(Database::connect(self.url()).await?)
	}
	async fn run_all<const LK: usize>(&self, event: Archive<&str, LK>) -> Events<Vec<Vec<QueryResult>>> {
		let db = self.connect().await?;
		let mut x = vec![];
		for e in event.into_iter() {
			x.push(db.query_all(Statement::from_string(db.get_database_backend(), e.to_string())).await?);
		}
		db.close().await?;
		Ok(x)
	}
	async fn run_execute<const LK: usize>(&self, event: Archive<&str, LK>) -> Events<Vec<ExecResult>> {
		let db = self.connect().await?;
		let mut x = vec![];
		for e in event.into_iter() {
			x.push(db.execute(Statement::from_string(db.get_database_backend(), e.to_string())).await?);
		}
		db.close().await?;
		Ok(x)
	}
	//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
	async fn redis_connection(&self) -> Conf {
		Conf::from_url(self.url())
	}
	async fn redis_pool(&self) -> Events<Pool> {
		Ok(self.redis_connection().await.create_pool(Some(Runtime::Tokio1))?)
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

impl PostgresUlr {
	pub async fn config(&self) -> (Client, Connection<Socket, NoTlsStream>) {
		let mut config = Config::new();
		config.host(self.host.as_str());
		config.user(self.name.as_str());
		config.password(self.password.as_str());
		config.dbname(self.database.as_str());
		config.port({
			if let Some(ref port) = self.port {
				port.parse().unwrap()
			} else {
				"5432".parse().unwrap()
			}
		});
		config.connect(NoTls).await.unwrap()
	}
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