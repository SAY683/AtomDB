use std::ops::Deref;
use deadpool_redis::redis::AsyncCommands;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use Static::Events;
use crate::setting::database_config::{Database, Service};
use crate::setting::local_config::{SUPER_DLR_URL, SUPER_URL};
use crate::sql_url::OrmEX;
use crate::system::{InstallUtils, Json};

pub async fn build_redis() -> Events<()> {
    let mut eg = SUPER_URL.deref().load().postgres.connect_bdc().await?;
    let xe = Database::select_all(&mut eg).await?;
    let time = SUPER_DLR_URL.load().time.clone();
    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    let xd = SUPER_URL.deref().load().redis.redis_pool().await?;
    let mut cmd = xd.get().await?;
    for i in xe {
        let xv = Service::select_all(&mut eg).await?.into_par_iter().find_any(|ie| {
            ie.uuid == i.uuid
        }).unwrap_or(Service::default());
        cmd.set_ex::<_, _, bool>(i.uuid, serde_json::to_string(&Response {
            name: xv.name,
            hash: i.hash,
            mode: xv.mode,
        }).unwrap(), time.clone()).await.unwrap();
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub name: String,
    pub hash: String,
    pub mode: String,
}

impl InstallUtils for Response {}

impl Json for Response {}