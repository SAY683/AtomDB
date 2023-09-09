use std::fs;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use bevy_reflect::Reflect;
use Static::{Alexia, Events};
use Static::alex::Overmaster;
use Static::base::FutureEx;
use crate::LOCAL_BIN_WEB;
use crate::setting::local_config::SUPER_DLR_URL;

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

pub async fn test() -> Events<Server> {
    Ok(HttpServer::new(|| {
        App::new().service(test_web)
    }).bind(SUPER_DLR_URL.load().port)?.run())
}

#[get("test")]
pub async fn test_web() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string(LOCAL_BIN_WEB.as_path()).unwrap())
}


async fn index() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string(LOCAL_BIN_WEB.as_path()).unwrap())
}