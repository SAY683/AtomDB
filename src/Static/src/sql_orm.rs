use futures::executor::block_on;
use rayon::prelude::*;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, ExecResult, Statement};
use tokio::runtime::Runtime;

use crate::Events;
use crate::static_array::Archive;

pub trait OrmEX {
	type Operate;
	fn url(&self) -> String;
	async fn connect(&self) -> Events<DatabaseConnection> {
		Ok(Database::connect(self.url()).await?)
	}
	///# 使用数据库
	async fn run_one(&self, sql: &str) -> Events<ExecResult> {
		let db = self.connect().await?;
		let opt = db.execute(Statement::from_string(
			db.get_database_backend(),
			sql.to_string(),
		))
			.await?;
		db.close().await?;
		Ok(opt)
	}
	async fn run<const LK: usize>(&self, event: Archive<&str, LK>) -> Events<()> {
		let db = self.connect().await?;
		event.0.into_par_iter().for_each(|e| {
			block_on(db.query_all(Statement::from_string(db.get_database_backend(), e.to_string()))).unwrap();
		});
		db.close().await?;
		Ok(())
	}
	async fn insert(&self, e: Self::Operate);
}