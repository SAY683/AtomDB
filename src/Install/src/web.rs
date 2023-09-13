use std::ops::Deref;
use actix_files::NamedFile;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use bevy_reflect::Reflect;
use rayon::prelude::*;
use redis::Commands;
use serde::{Deserialize, Serialize};
use Static::{Alexia, Events};
use Static::alex::Overmaster;
use Static::base::FutureEx;
use crate::rei::Response;
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
    let mut x = SUPER_URL.load().postgres.connect_bdc().await.unwrap();
    let erx = Database::select_all(&mut x).await?;
    let mut sup = HttpServer::new(|| {
        let mut ddc = App::new();
        ddc = ddc.route(SUPER_DLR_URL.load().path.as_str(), web::get().to(index));
        //ddc = ddc.service(download);
        ddc
    });
    for e in erx {
        sup = sup.bind(e.port).unwrap();
    }
    Ok(sup.bind(SUPER_DLR_URL.load().port)?.run())
}

// #[get("/{filename}")]
async fn download(filename: String) -> impl Responder {
    let file_path = format!("/{}", filename); // 你的文件路径，这里只是一个例子
    let mut eg = SUPER_URL.load().postgres.connect_bdc().await.unwrap();
    let erx = Database::select_name(&mut eg, &filename).await.unwrap();
    let er = Service::select_name(&mut eg, &filename).await.unwrap();
    for erx in erx {
        er.into_par_iter().for_each(|e| {
            if erx.uuid == e.uuid {

            }
        });
    }
    match NamedFile::open(file_path) {
        Ok(file) => HttpResponse::Ok().content_type("application/octet-stream").body(""),
        Err(_) => HttpResponse::NotFound().body("File not found"),
    }
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
    let mut med = vec![];
    let mut nc = SUPER_URL.deref().load().redis.redis_connection_async().await.unwrap();
    for e in xe.into_iter() {
        let xx = nc.get::<_, String>(e.uuid).unwrap();
        med.push(serde_json::from_str::<Response>(xx.as_str()).unwrap())
    }
    HttpResponse::Ok().json(med)
}