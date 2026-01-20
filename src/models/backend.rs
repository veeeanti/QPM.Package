use schemars::JsonSchema;
use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, PartialEq, Eq)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "The package version")]
pub struct PackageVersion {
    #[schemars(description = "The unique identifier of the package.")]
    pub id: String,

    #[schemars(description = "The version of the package.")]
    pub version: Version,
}
