use crate::common::*;

#[async_trait]
pub trait EsRepository {
    async fn get_mustache_template_infos(&self) -> Result<Value, anyhow::Error>;
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct EsRepositoryPub {
    pub cluster_name: String,
    pub es_clients: Vec<EsClient>,
}


#[derive(Debug, Getters, Clone, new)]
pub(crate) struct EsClient {
    host: String,
    es_conn: Elasticsearch
}



impl EsRepositoryPub {
    
    pub fn new(cluster_name: &str, hosts: Vec<String>, es_id: &str, es_pw: &str) -> Result<Self, anyhow::Error> {

        let mut es_clients: Vec<EsClient> = Vec::new();

        for host in hosts {
    
            let parse_url = format!("http://{}:{}@{}", es_id, es_pw, host);
            
            let es_url = Url::parse(&parse_url)?;
            let conn_pool = SingleNodeConnectionPool::new(es_url);
            let transport = TransportBuilder::new(conn_pool)
                .timeout(Duration::new(5,0))
                .build()?;
            
            let elastic_conn = Elasticsearch::new(transport);
            let es_client = EsClient::new(host, elastic_conn);
            es_clients.push(es_client);
        }   
        
        Ok(EsRepositoryPub{cluster_name: cluster_name.to_string(), es_clients})
    }
    
    
    // Common logic: common node failure handling and node selection
    async fn execute_on_any_node<F, Fut>(&self, operation: F) -> Result<Response, anyhow::Error>
    where
        F: Fn(Elasticsearch) -> Fut + Send + Sync,
        Fut: Future<Output = Result<Response, anyhow::Error>> + Send,
    {
        let mut last_error = None;
    
        // StdRng를 사용하여 Send 트레잇 문제 해결
        let mut rng = StdRng::from_entropy(); // 랜덤 시드로 생성
        
        // 클라이언트 목록을 셔플
        let mut shuffled_clients: Vec<EsClient> = self.es_clients.clone();
        shuffled_clients.shuffle(&mut rng); // StdRng를 사용하여 셔플
        
        // 셔플된 클라이언트들에 대해 순차적으로 operation 수행
        for es_client in shuffled_clients {
            
            let es_conn: Elasticsearch = es_client.es_conn;

            match operation(es_conn).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    last_error = Some(err);
                }
            }
        }
        
        // 모든 노드에서 실패했을 경우 에러 반환
        Err(anyhow::anyhow!(
            "All Elasticsearch nodes failed. Last error: {:?}",
            last_error
        ))
    }

}




#[async_trait]
impl EsRepository for EsRepositoryPub {
    
    
    /*
        mustache template 정보를 쿼리해주는 함수  
    */
    async fn get_mustache_template_infos(&self) -> Result<Value, anyhow::Error> {

        let response = self.execute_on_any_node(|es_client| async move {

            let response = 
                es_client
                .cluster()
                .state(ClusterStateParts::Metric(&["metadata"]))
                .filter_path(&["metadata.stored_scripts"])
                .send()
                .await?;
            
            Ok(response)

        })
        .await?;
        

        if response.status_code().is_success() {
            let response_body: Value = response.json::<Value>().await?;
            Ok(response_body)
        } else {
            Err(anyhow!("[ERROR][get_cluster_template_infos()]"))
        }
    }

}