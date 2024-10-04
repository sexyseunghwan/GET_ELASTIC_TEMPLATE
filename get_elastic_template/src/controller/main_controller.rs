use core::panic;

use crate::common::*;

use crate::utils_modules::init_utils::*;
use crate::utils_modules::logger_utils::*;

use crate::repository::es_repository::*;

use crate::service::template_search_service::*;

/*
    메인컨트롤러 영역
*/
pub async fn main_controller() {
    
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
    
    let mut template_services: Vec<TemplateService> = Vec::new();
    
    for es_repo in es_infos_vec {
        let tmpl_service = TemplateService::new(es_repo);
        template_services.push(tmpl_service);
    }
    

    /* 
        Elasticsearch Cluster 가 여러개일 경우
        각 cluster 별로 비동기적으로 처리함.
    */
    let futures = template_services.iter().map(|tmpl_service| {
        let service: TemplateService = tmpl_service.clone();
        async move {                
            service.get_templates_name().await /* 실제 작업을 담당하는 함수 */
        }
    });
    
    let results = join_all(futures).await;

    for result in results {
        match result {
            Ok(_) => {
                info!("Template processed successfully");
            }
            Err(e) => {
                error!("Error processing template: {:?}", e);
            }
        }
    }
}