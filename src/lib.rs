use serde::{Serialize, Deserialize};

pub type HostnameGroups = Vec<HostnameGroup>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostnameGroup {
    pub name: String,
    pub hostnames: Vec<String>,
}