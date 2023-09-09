use std::fs;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
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
            // websocket().await?;
            Ok(Websocket)
        }))]
    }
}

pub trait WebInfo {}

pub async fn web() -> Events<()> {
    HttpServer::new(|| {
        App::new().route(SUPER_DLR_URL.load().path.as_str(), web::get().to(index))
    }).bind(SUPER_DLR_URL.load().port)?
        .run()
        .await?;
    Ok(())
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string(LOCAL_BIN_WEB.as_path()).unwrap())
}