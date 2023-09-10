use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use bevy_reflect::Reflect;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use Static::{Alexia, Events};
use Static::alex::Overmaster;
use Static::base::FutureEx;
use crate::setting::database_config::{Database, Service};
use crate::setting::local_config::{SUPER_DLR_URL, SUPER_URL};
use crate::sql_url::OrmEX;

#[derive(Copy, Clone, Reflect, Debug)]
pub struct Websocket;

///# 链路 【未启用】
impl Alexia<Websocket> for Websocket {
    fn event() -> Vec<FutureEx<'static, Overmaster, Events<Websocket>>> {
        vec![FutureEx::AsyncTraitSimple(Box::pin(async {
            web().await?.await?;
            Ok(Websocket)
        })), FutureEx::AsyncTraitSimple(Box::pin(async {
            Ok(Websocket)
        }))]
    }
}

pub async fn web() -> Events<Server> {
    Ok(HttpServer::new(|| {
        App::new().route(SUPER_DLR_URL.load().path.as_str(), web::get().to(index))
    }).bind(SUPER_DLR_URL.load().port)?.run())
}

#[derive(Serialize, Deserialize, Debug)]
struct MysqlESR {
    name: String,
    port: String,
    logs: Option<String>,
}

async fn index() -> impl Responder {
    let mut eg = SUPER_URL.load().postgres.connect_bdc().await.unwrap();
    let xe = Database::select_all(&mut eg).await.unwrap();
    let mut med: Vec<MysqlESR> = vec![];
    for e in xe.into_iter() {
        let ra = Service::select_all(&mut eg).await.unwrap_or(vec![Service::default()]).into_par_iter().find_any(|x| {
            x.uuid == e.uuid
        }).unwrap_or(Service::default());
        med.push(MysqlESR {
            name: e.name,
            port: format!("{}/{}", e.port, ra.name),
            logs: ra.logs,
        })
    }
    HttpResponse::Ok().json(med)
}