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

		let response = if let Ok(x) = reqwest::get(&path).await { x } else { continue; };

		if response.status() == StatusCode::OK {
			let jar_path = format!("{}/{}-{}.jar", path, dependency.artifact_id, dependency.version);
			return Option::Some(jar_path);
		}
	}

	Option::None
}