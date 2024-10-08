/*
Author      : Seunghwan Shin 
Create date : 2024-10-04 
Description : Elasticsearch cluster 에 존재하는 mustache 템플릿 리스트를 뽑아주는 기능.
    
History     : 2024-10-04 Seunghwan Shin       # first create
              2024-10-08 Seunghwan Shin       # 추상화 아키텍쳐로 변경.
*/ 


mod common;
use common::*;

mod controller;
use controller::main_controller::*;

mod model;
mod repository;
use repository::es_repository::*;

mod service;
use service::template_search_service::*;

mod utils_modules;
use utils_modules::logger_utils::*;

#[tokio::main]
async fn main() { 
    
    set_global_logger();
    info!("Start the template finder program");

    // 대상이 되는 Elasticsearch DB 커넥션 정보들
    let es_infos_vec: Vec<EsRepositoryPub> = match initialize_db_clients("./datas/server_info.json") {
        Ok(es_infos_vec) => es_infos_vec,
        Err(e) => {
            error!("{:?}", e);
            panic!("{:?}", e)
        }
    };

    // 서비스 의존주입.
    let mut services: Vec<TemplateServicePub<EsRepositoryPub>> = Vec::new();
    
    for cluster in es_infos_vec {
        let template_service = TemplateServicePub::new(cluster);
        services.push(template_service);
    }

    // 컨트롤러 의존주입.
    let controller = MainController::new(services);
    controller.main_function().await;
    

    
}