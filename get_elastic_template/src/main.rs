/*
Author      : Seunghwan Shin 
Create date : 2024-10-04 
Description : Elasticsearch cluster 에 존재하는 mustache 템플릿 리스트를 뽑아주는 기능.
    
History     : 2023-10-04 Seunghwan Shin       # first create
*/ 


mod common;
mod controller;
mod model;
mod repository;
mod service;
mod utils_modules;

use crate::controller::main_controller::*;


#[tokio::main]
async fn main() { main_controller().await; }