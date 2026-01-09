use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathParamsModel {
    pub id: String,
}
