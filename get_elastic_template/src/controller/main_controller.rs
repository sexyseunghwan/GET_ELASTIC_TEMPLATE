use core::panic;

use crate::common::*;

use crate::utils_modules::io_utils::*;

use crate::model::ClusterJson::*;

use crate::service::es_service::*;
use crate::service::template_search_service::*;

/*
    메인컨트롤러 영역--
*/
pub async fn main_controller() {
    
    // Elasticsearch 정보 관련 파일읽기
    let config: ClusterJson = match read_json_from_file::<ClusterJson>("./datas/server_info.json") {
        Ok(config) => config,
        Err(e) => {
            error!("{:?}", e);
            panic!("{:?}", e)
        }
    };
    
    // Elasticsearch connection 생성
    let elastic_conn = match EsHelper::new(
        &config.cluster_name,
        config.hosts.clone(),
        &config.es_id,
        &config.es_pw,
    ) {
        Ok(cluster_config) => cluster_config,
        Err(e) => {
            error!("{:?}", e);
            panic!("{:?}", e)
        } 
    };

    // 서비스 제공 
    match get_templates_name(&elastic_conn).await {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
            panic!("{:?}", e)
        } 
    }
    
}