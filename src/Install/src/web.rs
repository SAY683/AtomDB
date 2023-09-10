use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};
use Static::{Alexia, Events};
use Static::alex::Overmaster;
use Static::base::FutureEx;
use crate::setting::database_config::{Database, Service};
use crate::setting::local_config::{SUPER_DLR_URL, SUPER_URL};
use crate::sql_url::OrmEX;
use crate::web::tet::{test};

#[derive(Copy, Clone, Reflect, Debug)]
pub struct Websocket;

impl Alexia<Websocket> for Websocket {
    fn event() -> Vec<FutureEx<'static, Overmaster, Events<Websocket>>> {
        vec![FutureEx::AsyncTraitSimple(Box::pin(async {
            web().await?.await?;
            Ok(Websocket)
        })), FutureEx::AsyncTraitSimple(Box::pin(async {
            test().await?.await?;
            Ok(Websocket)
        }))]
    }
}

pub async fn web() -> Events<Server> {
    Ok(HttpServer::new(|| {
        App::new().route(SUPER_DLR_URL.load().path.as_str(), web::get().to(index))
    }).bind(SUPER_DLR_URL.load().port)?.run())
}

async fn index() -> impl Responder {
    let mut eg = SUPER_URL.load().postgres.connect_bdc().await.unwrap();
    let xe = Database::select_all(&mut eg).await.unwrap();
    let xr = Service::select_all(&mut eg).await.unwrap();
    let mut med: Vec<MysqlESR> = vec![];
    xe.into_iter().for_each(|e| {
        let r = xr.clone().into_iter().find(|x| { &x.uuid == &e.uuid }).unwrap_or(Service::default());
        med.push(MysqlESR {
            name: e.name,
            port: format!("{}/{}", e.port, r.name),
            logs: r.logs,
        })
    });
    HttpResponse::Ok().json(med)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MysqlESR {
    pub name: String,
    pub port: String,
    pub logs: Option<String>,
}


pub mod tet {
    use std::fs;
    use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder};
    use actix_web::dev::Server;
    use Static::Events;
    use crate::LOCAL_BIN_WEB;
    use crate::setting::local_config::SUPER_DLR_URL;

    pub async fn test() -> Events<Server> {
        Ok(HttpServer::new(|| {
            App::new().service(test_web).service(test_web_re)
        }).workers(2).bind(SUPER_DLR_URL.load().port)?.run())
    }

    #[get("/test")]
    pub async fn test_web() -> impl Responder {
        HttpResponse::Ok().body(fs::read_to_string(LOCAL_BIN_WEB.as_path()).unwrap())
    }

    #[get("/test_response")]
    pub async fn test_web_re(req: HttpRequest) -> impl Responder {
        HttpResponse::Ok().body(format!("{:?}", req.head()))
    }
}

