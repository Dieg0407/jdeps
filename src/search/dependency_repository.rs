use std::error::Error;

use serde_json::Value;

use crate::models::Dependency;

pub fn fetch_dependencies(artifact: &String) -> Result<Vec<Dependency>, Box<dyn Error>> {
    let url = format!("https://search.maven.org/solrsearch/select?q=a:{}&rows=100&wt=json", artifact);
    let result = ureq::get(&url)
        .call()?
        .into_string()?
        .parse_json()?;

    let result = &result["response"]["docs"];
    let mut dependencies = vec![];
    for depedency in result.as_array().unwrap_or(&vec![]) {
        dependencies.push(Dependency { 
            version: depedency["latestVersion"].to_string(),
            group_id: depedency["g"].to_string(),
            artifact_id: depedency["a"].to_string()
        });
    }

    Ok(dependencies)
}


trait ParseJson {
    fn parse_json(&self) -> Result<Value, Box<dyn Error>>;
}

impl ParseJson for String {
    fn parse_json(&self) -> Result<Value, Box<dyn Error>> {
        match serde_json::from_str(&self) {
            Ok(json) => Ok(json),
            Err(e) => Err(Box::new(e))
        }
    }
}

