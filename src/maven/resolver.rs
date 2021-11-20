use std::ops::Index;

use reqwest::StatusCode;

pub struct Dependency {
	pub group_id: String,
	pub artifact_id: String,
	pub version: String,
}

pub async fn find_dependency(dependency: &Dependency, repositories: &Vec<&str>) -> Option<String> {
    let mut iter = repositories.iter();

    while let Some(repository) = iter.next() {
        let path = format!(
            "{}/{}/{}/{}",
            repository,
            dependency.group_id.replace(".", "/"),
            dependency.artifact_id,
            dependency.version
        );

        let response_result = reqwest::get(&path).await;

        match response_result {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    let response_url_path = response.url().path();
                    let last_slash_index = response_url_path.rfind("/")?;

                    let resulting_maven_path = &response_url_path[0..last_slash_index];
                    let resulting_maven_path_slash_index = resulting_maven_path.rfind("/")? + 1;

                    let real_maven_version = &resulting_maven_path[resulting_maven_path_slash_index..last_slash_index];
                    let jar_file_name = format!("{}-{}.jar", dependency.artifact_id, real_maven_version);

                    return Option::Some(format!("{}/{}", path, jar_file_name));
                }       
            },
            Err(_) => {
                continue;
            }
        };
    }

    Option::None
}