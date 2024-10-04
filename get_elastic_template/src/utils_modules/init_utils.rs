use crate::common::*;

use crate::repository::es_repository::*;

use crate::model::ClusterConfig::*;

use crate::utils_modules::io_utils::*;

/* 
    Elasticsearch DB 초기화
*/
pub fn initialize_db_clients(es_info_path: &str) -> Result<Vec<EsRepositoryPub>, anyhow::Error> {

    let mut elastic_conn_vec: Vec<EsRepositoryPub> = Vec::new();

    let cluster_config: ClusterConfig = read_json_from_file::<ClusterConfig>(es_info_path)?;
    
    for config in &cluster_config.clusters {
        
        let es_helper = EsRepositoryPub::new(
            &config.cluster_name,
            config.hosts.clone(), 
            &config.es_id, 
            &config.es_pw)?;
        
        elastic_conn_vec.push(es_helper);
    }
    
    Ok(elastic_conn_vec)

}