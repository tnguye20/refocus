use std::{error::Error, fs::{self, read_to_string}};
use log::{info, trace};
use refocus::*;

static HOSTS_FILE_PATH: &str = "/tmp/hosts";
static HOSTS_GROUP_CONFIG_PATH: &str = "/tmp/refocus_host_group_config.json";
static HOSTNAME_ANCHOR: &str = "refocus.dev";

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

fn construct_refocus_line(hostgroups: &HostnameGroups) -> String
{
    info!("Generating Refocus hosts line");
    let mut refocus_line = String::from("127.0.0.1 ".to_owned() + HOSTNAME_ANCHOR + " ");
    for group in hostgroups
    {
        for hostname in &group.hostnames
        {
            refocus_line.push_str(hostname);
            refocus_line.push_str(" ");
            if !hostname.contains("www.")
            {
                let w = "www.".to_string() + hostname;
                refocus_line.push_str(&w);
                refocus_line.push_str(" ");
            }
        }
    }
    refocus_line.push_str("\n");

    trace!("Refocus line: {:?}", refocus_line);
    refocus_line
}

fn main() {
    let mut hosts_content = read_hosts().unwrap();
    let hostgroups = read_hostname_group_config().unwrap();


    if hosts_content.contains(HOSTNAME_ANCHOR)
    {
        info!("Refocus Line found. Finding line to regenerate");
        let mut new_hosts_content = String::from("");
        for line in hosts_content.lines()
        {
            if line.contains(HOSTNAME_ANCHOR)
            {
                new_hosts_content.push_str(&construct_refocus_line(&hostgroups));
                new_hosts_content.push_str("\n");
            }
            else
            {
                new_hosts_content.push_str(line);
                new_hosts_content.push_str("\n");
            }
        }
        trace!("New hosts content: \n {:?}", new_hosts_content);

        fs::write(HOSTS_FILE_PATH, new_hosts_content).expect("Unable to write hosts file");
    }
    else
    {
        info!("Refocus Line not found. Creating one and appending at end of file");
        hosts_content.push_str(&construct_refocus_line(&hostgroups));
        trace!("New hosts content: \n {:?}", hosts_content);

        fs::write(HOSTS_FILE_PATH, hosts_content).expect("Unable to write hosts file");
    }
}
