use crate::common::*;


/*
    Json 파일을 읽어서 객체로 변환해주는 함수
*/
pub fn read_json_from_file<T: DeserializeOwned>(file_path: &str) -> Result<T, anyhow::Error> {
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = from_reader(reader)?;
    
    Ok(data)
}

/*
    파일에 쓰는 함수도 만들어줘야 한다.
*/
pub fn write_to_file(file_path: &str, contents: &str) -> Result<(), anyhow::Error> {

    let mut file = OpenOptions::new()
        .create(true)       // 파일이 없으면 새로 만듦
        .append(true)       // 기존 내용 뒤에 이어서 씀
        .open(file_path)?;                  // 파일 경로 지정

    // 파일에 내용을 쓰기
    writeln!(file, "{}", contents)?; // 개행을 추가하여 새 줄에 내용 쓰기

    Ok(())
}