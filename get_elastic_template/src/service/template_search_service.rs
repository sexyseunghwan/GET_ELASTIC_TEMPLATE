use crate::common::*;

use crate::utils_modules::io_utils::*;
use crate::utils_modules::time_utils::*;

use crate::repository::es_repository::*;

#[derive(Debug, Clone, new)]
pub struct TemplateService {
    es_conn: EsRepositoryPub
}

impl TemplateService {

    /*
        특정 Elasticsearch 클러스터에서 운영되는 모든 mustache 템플릿들을 파일에 써주는 함수
    */
    pub async fn get_templates_name(&self) -> Result<(), anyhow::Error> {

        // ES 쿼리 던지기
        let res = self.es_conn.get_mustache_template_infos().await?;

        // Cluster 이름
        let cluster_name = self.es_conn.cluster_name().as_str();
        
        let cur_datetime = get_current_kor_naive_datetime();
        let cur_datetime_str = get_str_from_naive_datetime(cur_datetime);

        // 경로 설정 및 폴더 생성 확인
        let folder_path = format!("./datas/result/{}", cluster_name);
        let file_name = format!("{}/{}.txt", folder_path, cur_datetime_str);
        
        // 폴더가 없는 경우 생성
        if !Path::new(&folder_path).exists() {
            std::fs::create_dir_all(&folder_path)?; // 경로에 맞는 모든 상위 폴더도 생성
        }

        // 응답 json 파싱 && 파일쓰기
        if let Some(stored_scripts) = res["metadata"]["stored_scripts"].as_object() {
            for template_name in stored_scripts.keys() {
                write_to_file(&file_name, template_name)?;
            }
        }
        
        Ok(())
    }

}

