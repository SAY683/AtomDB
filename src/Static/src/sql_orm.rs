use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, ExecResult, QueryResult, Statement};

use crate::Events;
use crate::static_array::Archive;

pub trait OrmEX {
	fn url(&self) -> String;
	async fn connect(&self) -> Events<DatabaseConnection> {
		Ok(Database::connect(self.url()).await?)
	}
	async fn run_all<const LK: usize>(&self, event: Archive<&str, LK>) -> Events<Vec<Vec<QueryResult>>> {
		let db = self.connect().await?;
		let mut x = vec![];
		event.0.into_iter().for_each(|e| {
			x.push(block_on(db.query_all(Statement::from_string(db.get_database_backend(), e.to_string()))).unwrap());
		});
		db.close().await?;
		Ok(x)
	}
	async fn run_execute<const LK: usize>(&self, event: Archive<&str, LK>) -> Events<Vec<ExecResult>> {
		let db = self.connect().await?;
		let mut x = vec![];
		event.0.into_iter().for_each(|e| {
			x.push(block_on(db.execute(Statement::from_string(db.get_database_backend(), e.to_string()))).unwrap());
		});
		db.close().await?;
		Ok(x)
	}
}