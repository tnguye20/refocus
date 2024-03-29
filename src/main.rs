use std::{error::Error, fs::read_to_string};

use refocus::*;

static HOSTS_FILE_PATH: &str = "/tmp/hosts";
static HOSTS_GROUP_CONFIG_PATH: &str = "/tmp/refocus_host_group_config.json";

fn read_hosts() -> Result<String, Box<dyn Error>>
{
    let hosts_content = read_to_string(HOSTS_FILE_PATH)?;
    Ok(hosts_content)
}

fn read_hostname_group_config() -> Result<HostnameGroups, Box<dyn Error>>
{
    let hostgroup_config = read_to_string(HOSTS_GROUP_CONFIG_PATH)?;
    let hostgroups: HostnameGroups = serde_json::from_str(&hostgroup_config).unwrap();

    Ok(hostgroups)
}

fn main() {
    let _ = read_hosts().unwrap();
    let _ = read_hostname_group_config().unwrap();
}
