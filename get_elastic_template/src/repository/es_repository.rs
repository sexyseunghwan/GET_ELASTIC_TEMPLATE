use crate::common::*;

#[async_trait]
pub trait EsRepository {
    async fn get_mustache_template_infos(&self) -> Result<Value, anyhow::Error>;
}


#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct EsObj {
    pub es_host: String,
    pub es_pool: Elasticsearch
}

#[async_trait]
impl EsRepository for EsObj {
    
    
    /*
        test
    */
    async fn get_mustache_template_infos(&self) -> Result<Value, anyhow::Error> {
        
        let response = self.es_pool
            .cluster()
            .state(ClusterStateParts::Metric(&["metadata"]))
            .filter_path(&["metadata.stored_scripts"])
            .send()
            .await?;
        
        if response.status_code().is_success() {
            let response_body: Value = response.json::<Value>().await?;
            Ok(response_body)
        } else {
            Err(anyhow!("[ERROR][get_cluster_template_infos()]"))
        }
    }

}