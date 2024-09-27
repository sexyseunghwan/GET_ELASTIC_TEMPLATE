use crate::common::*;

use crate::repository::es_repository::*;

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct EsHelper {
    cluster_name: String,
    mon_es_pool: Vec<EsObj>
}

impl EsHelper {
    
    /* 
        EsHelper의 생성자 -> Elasticsearch cluster connection 정보객체를 생성해줌.
    */
    pub fn new(cluster_name: &str, hosts: Vec<String>, es_id: &str, es_pw: &str) -> Result<Self, anyhow::Error> {
        
        let mut mon_es_clients: Vec<EsObj> = Vec::new();
        
        for url in hosts {
                
            let parse_url = if es_id.is_empty() || es_pw.is_empty() {
                format!("http://{}", url)
            } else {
                format!("http://{}:{}@{}", es_id, es_pw, url)
            };
            
            let es_url = Url::parse(&parse_url)?;
            let conn_pool = SingleNodeConnectionPool::new(es_url);
            let transport = TransportBuilder::new(conn_pool)
                .timeout(Duration::new(5,0))
                .build()?;
            
            mon_es_clients.push(EsObj::new(url, Elasticsearch::new(transport)));
        }
        
        Ok(EsHelper{cluster_name: cluster_name.to_string(), mon_es_pool: mon_es_clients})
    }
    
    
    /*

    */
    pub async fn get_cluster_mustache_template_infos(&self) -> Result<Value, anyhow::Error> {

        for es_obj in self.mon_es_pool.iter() {

            match es_obj.get_mustache_template_infos().await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    error!("{:?}", err);      
                    continue;
                }
            }   
        }
        
        Err(anyhow!("[Elasticsearch Error][get_cluster_mustache_template_infos()] All Elasticsearch connections failed"))
    }    
}
