#[derive(Debug)]
pub struct Dependency {
    pub artifact_id: String,
    pub group_id: String,
    pub version: String
}

impl Clone for Dependency {
    fn clone(&self) -> Self {
        Dependency {
            artifact_id: self.artifact_id.clone(),
            group_id: self.group_id.clone(),
            version: self.version.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.artifact_id = source.artifact_id.clone();
        self.group_id = source.group_id.clone();
        self.version = source.version.clone();
    }
}
