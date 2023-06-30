use std::ops::Deref;

use sea_orm::EntityTrait;

use Install::setting::local_config::SUPER_URL;
use Install::tables::prelude::Database;
use Static::Events;
use Static::sql_orm::OrmEX;

///# 构建
pub async fn build_graph_table_postgres() -> Events<()> {
	let x = SUPER_URL.deref().load().postgres.connect().await?;
	let x86_64 = Database::find().all(&x).await?;
	x86_64.into_iter().for_each(|e|{
		println!("{:?}",e);
	});
	Ok(())
}