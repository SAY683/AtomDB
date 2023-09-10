use std::ops::Deref;
use deadpool_redis::redis::AsyncCommands;
use Static::Events;
use crate::setting::database_config::{Database, Service};
use crate::setting::local_config::SUPER_URL;
use crate::sql_url::OrmEX;

pub async fn build_redis() -> Events<()> {
    let xd = SUPER_URL.deref().load().redis.redis_pool().await?;
    let mut eg = SUPER_URL.deref().load().postgres.connect_bdc().await?;
    let _xe = Database::select_all(&mut eg).await?;
    let _xr = Service::select_all(&mut eg).await?;
    let mut cmd = xd.get().await?;
    Ok(())
}