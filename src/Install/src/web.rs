use std::fs;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use bevy_reflect::Reflect;
use Static::{Alexia, Events};
use crate::LOCAL_BIN_WEB;
use crate::setting::local_config::SUPER_DLR_URL;

#[derive(Copy, Clone, Reflect, Debug)]
pub struct Websocket;

pub trait WebInfo {}

pub async fn web() -> Events<Server> {
    Ok(HttpServer::new(|| {
        App::new().route(SUPER_DLR_URL.load().path.as_str(), web::get().to(index))
    }).bind(SUPER_DLR_URL.load().port)?.run())
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string(LOCAL_BIN_WEB.as_path()).unwrap())
}