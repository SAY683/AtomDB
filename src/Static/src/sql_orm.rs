use deadpool_redis::{Config, Pool, Runtime};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, ExecResult, QueryResult, Statement};

use crate::Events;
use crate::static_array::Archive;

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
	async fn redis_connection(&self) -> Config {
		Config::from_url(self.url())
	}
	async fn redis_pool(&self) -> Events<Pool> {
		Ok(self.redis_connection().await.create_pool(Some(Runtime::Tokio1))?)
	}
}