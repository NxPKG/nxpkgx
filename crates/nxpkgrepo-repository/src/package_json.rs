use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use nxpkgpath::{AbsoluteSystemPath, RelativeUnixPathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "nxpkg", skip_serializing_if = "Option::is_none")]
    pub legacy_nxpkg_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scripts: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolutions: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnpm: Option<PnpmConfig>,
    // Unstructured fields kept for round trip capabilities
    #[serde(flatten)]
    pub other: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PnpmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patched_dependencies: Option<BTreeMap<String, RelativeUnixPathBuf>>,
    // Unstructured config options kept for round trip capabilities
    #[serde(flatten)]
    pub other: BTreeMap<String, Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to read package.json: {0}")]
    Io(#[from] std::io::Error),
    #[error("unable to parse package.json: {0}")]
    Json(#[from] serde_json::Error),
}

impl PackageJson {
    pub fn load(path: &AbsoluteSystemPath) -> Result<PackageJson, Error> {
        let contents = std::fs::read_to_string(path)?;
        let package_json: PackageJson = serde_json::from_str(&contents)?;
        Ok(package_json)
    }

    // Utility method for easy construction of package.json during testing
    pub fn from_value(value: serde_json::Value) -> Result<PackageJson, Error> {
        let package_json: PackageJson = serde_json::from_value(value)?;
        Ok(package_json)
    }

    pub fn all_dependencies(&self) -> impl Iterator<Item = (&String, &String)> + '_ {
        self.dev_dependencies
            .iter()
            .flatten()
            .chain(self.optional_dependencies.iter().flatten())
            .chain(self.dependencies.iter().flatten())
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use test_case::test_case;

    use super::*;

    #[test_case(json!({"name": "foo", "random-field": true}) ; "additional fields kept during round trip")]
    #[test_case(json!({"name": "foo", "resolutions": {"foo": "1.0.0"}}) ; "berry resolutions")]
    #[test_case(json!({"name": "foo", "pnpm": {"patchedDependencies": {"some-pkg": "./patchfile"}, "another-field": 1}}) ; "pnpm")]
    #[test_case(json!({"name": "foo", "pnpm": {"another-field": 1}}) ; "pnpm without patches")]
    fn test_roundtrip(json: Value) {
        let package_json: PackageJson = serde_json::from_value(json.clone()).unwrap();
        let actual = serde_json::to_value(package_json).unwrap();
        assert_eq!(actual, json);
    }

    #[test]
    fn test_legacy_nxpkg_config() -> Result<()> {
        let contents = r#"{"nxpkg": {}}"#;
        let package_json = serde_json::from_str::<PackageJson>(contents)?;

        assert!(package_json.legacy_nxpkg_config.is_some());

        let contents = r#"{"nxpkg": { "globalDependencies": [".env"] } }"#;
        let package_json = serde_json::from_str::<PackageJson>(contents)?;

        assert!(package_json.legacy_nxpkg_config.is_some());

        Ok(())
    }
}
