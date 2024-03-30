use std::{error::Error, fs::{self, File, read_to_string}, io::{ErrorKind, Read}, path::{Path, PathBuf}};
use log::{info, trace};
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;

pub static APPNAME: &str = "refocus";
pub static HOSTS_FILE_PATH: &str = "/etc/hosts";
pub static TMP_HOSTS_FILE_PATH: &str = "/tmp/hosts";
pub static HOSTNAME_ANCHOR: &str = "refocus.dev";
pub static HOSTNAME_GROUPS_CONFIG_FILENAME: &str = "refocus_hostnames_group.json";

pub type HostnameGroups = Vec<HostnameGroup>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostnameGroup {
    pub name: String,
    pub hostnames: Vec<String>,
}

impl std::default::Default for HostnameGroup
{
    fn default() -> Self {
        Self { 
            name: String::from("Social Media"),
            hostnames: vec!["facebook.com".to_string(), "instagram.com".to_string(), "twitter.com".to_string(), "linkedin.com".to_string()]
        }
    }
}

pub fn create_tmp_hosts_file() -> Result<(), Box<dyn Error>>
{
    let hosts_content = read_to_string(HOSTS_FILE_PATH)?;
    fs::write(TMP_HOSTS_FILE_PATH, &hosts_content)?;
    Ok(())
}

pub fn get_config_file_dir() -> Result<PathBuf, Box<dyn Error>>
{
    let project_dirs = ProjectDirs::from("rs", APPNAME, APPNAME).unwrap();
    let path = project_dirs.config_dir().join(Path::new(HOSTNAME_GROUPS_CONFIG_FILENAME));
    Ok(path)
}

pub fn read_hosts() -> Result<String, Box<dyn Error>>
{
    let hosts_content = read_to_string(HOSTS_FILE_PATH)?;
    Ok(hosts_content)
}

pub fn read_hostname_groups_config() -> Result<HostnameGroups, Box<dyn Error>>
{
    let path = get_config_file_dir()?;

    println!("Config path: {:?}", path);

    match File::open(&path)
    {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            let hostname_groups = serde_json::from_str(&buf)?;
            Ok(hostname_groups)
        },
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            let parent = path.parent().unwrap();
            fs::create_dir_all(parent)?;
            let hostname_groups = vec![
                HostnameGroup::default()
            ];
            let content = serde_json::to_string_pretty(&hostname_groups)?;
            fs::write(path, content)?;
            Ok(hostname_groups)
        },
        Err(e) => Err(e.into())
    }
}
 
pub fn construct_refocus_line(hostgroups: &HostnameGroups) -> String
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
