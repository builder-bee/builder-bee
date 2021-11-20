#[cfg(test)]

mod dependency_test {

	use anyhow::Result;
	use bbee::maven::resolver::{Dependency, find_dependency};
	use pretty_assertions::assert_eq;

	#[tokio::test]
	async fn check_jitpack_success() -> Result<()> {
		let dependency = Dependency {
			group_id: String::from("com.github.Minestom"),
			artifact_id: String::from("Minestom"),
			version: String::from("master-SNAPSHOT")
		};
		
		let respositories = vec!["https://jitpack.io/"];
		let result = find_dependency(&dependency, &respositories).await.ok_or_else(|| anyhow::anyhow!("No result"))?;

		assert_eq!(result, "https://jitpack.io/com/github/Minestom/Minestom/master-SNAPSHOT/Minestom-master-SNAPSHOT.jar");

		Ok(())
	}
}
