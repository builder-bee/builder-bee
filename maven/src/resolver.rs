use reqwest::StatusCode;

pub struct Dependency {
	pub group_id: String,
	pub artifact_id: String,
	pub version: String,
}

/// Removes a hanging slash from the end of a repository URL.
#[must_use]
pub fn correct_repository(url: &str) -> String {
	if url.ends_with('/') {
		url[..url.len() - 1].to_string()
	} else {
		url.to_owned()
	}
}

#[must_use]
pub fn correct_repositories(repositores: &Vec<String>) -> Vec<String> {
	repositores.iter().map(|repository| correct_repository(repository)).collect::<Vec<String>>()
}


pub async fn find_dependency(dependency: &Dependency, repositories: &Vec<String>) -> Option<String> {
	let mut iter = repositories.iter();

	while let Some(repository) = iter.next() {
		let path = format!(
			"{}/{}/{}/{}",
			repository,
			dependency.group_id.replace(".", "/"),
			dependency.artifact_id,
			dependency.version
		);

		let response = if let Ok(x) = reqwest::get(&path).await { x } else { continue; };

		if response.status() == StatusCode::OK {
			let jar_path = format!("{}/{}-{}.jar", path, dependency.artifact_id, dependency.version);
			return Option::Some(jar_path);
		}
	}

	Option::None
}