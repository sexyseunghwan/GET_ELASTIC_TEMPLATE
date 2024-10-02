use crate::common::*;

use crate::service::es_service::*;

use crate::utils_modules::io_utils::*;


/*
    test
*/
pub async fn get_templates_name(es_conn: &EsHelper) -> Result<(), anyhow::Error> {

    // ES 쿼리 던지기
    let res = es_conn.get_cluster_mustache_template_infos().await?;

    // 응답 json 파싱 && 파일쓰기
    if let Some(stored_scripts) = res["metadata"]["stored_scripts"].as_object() {
        for template_name in stored_scripts.keys() {
            write_to_file("./datas/result.txt", template_name)?;
        }
    }

    Ok(())
}