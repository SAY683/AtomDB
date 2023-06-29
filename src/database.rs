use std::ops::Deref;

use Install::setting::{BUILD_DIR, STRING_BUILD_SCALAR_DIR};
use Install::setting::local_config::SUPER_URL;
use Install::sql_url::PostgresUlr;
use Static::Events;
use Static::sql_orm::OrmEX;
use Static::static_array::Archive;

///# 构建
pub async fn build_graph_table_postgres() -> Events<()> {
	let _ = PostgresUlr::default().run_one(BUILD_DIR).await;
	SUPER_URL.deref().load().postgres.run(Archive(STRING_BUILD_SCALAR_DIR)).await;
	Ok(())
}