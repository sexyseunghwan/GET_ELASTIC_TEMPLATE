use core::panic;

use crate::common::*;

use crate::utils_modules::logger_utils::*;

use crate::repository::es_repository::*;

use crate::service::template_search_service::*;

pub struct MainController<S: TemplateService> {
    template_services: Vec<S>
}


impl<S: TemplateService> MainController<S> {

    pub fn new(template_services: Vec<S>) -> Self {
        Self {
            template_services,
        }
    }

    /*
    
    */
    pub async fn main_function(&self) {

        /* 
            Elasticsearch Cluster 가 여러개일 경우
            각 cluster 별로 비동기적으로 처리함.
        */
        let futures = self.template_services.iter().map(|tmpl_service| {
            
            let service = tmpl_service.clone();
            
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

}