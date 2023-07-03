use std::ops::Deref;

use sea_orm::EntityTrait;

use Install::setting::local_config::SUPER_URL;
use Install::sql_url::{PostgresUlr, Url};
use Install::tables::prelude::*;
use Static::Events;
use Static::sql_orm::OrmEX;

///# 构建
pub async fn build_graph_table_postgres() -> Events<()> {
	let x = SUPER_URL.deref().load().postgres.connect().await?;
	let x86_64 = Database::find().all(&x).await?;
	for i in x86_64.into_iter() {
		let db = PostgresUlr::default_connect(SUPER_URL.deref().load().postgres.build_url_database(&i.architecture)).await?;
		println!("{:?}", i);
	}
	x.close().await?;
	Ok(())
}