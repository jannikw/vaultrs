use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadEntityResponse {
    pub creation_time: String,
    pub disabled: bool,
    pub id: String,
    pub last_update_time: String,
    pub metadata: Option<HashMap<String, String>>,
    pub name: String,
    pub aliases: Vec<ReadEntityAliasResponse>,
    pub policies: Vec<String>,
    pub direct_group_ids: Vec<String>,
    pub group_ids: Vec<String>,
    pub inherited_group_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadEntityAliasResponse {
    pub canonical_id: String,
    pub id: String,
    pub creation_time: String,
    pub last_update_time: String,
    pub local: bool,
    pub metadata: Option<HashMap<String, String>>,
    pub custom_metadata: Option<HashMap<String, String>>,
    pub mount_accessor: String,
    pub mount_path: String,
    pub mount_type: String,
    pub name: String,
}
