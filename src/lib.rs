use directories::ProjectDirs;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use core::fmt;
use std::{
    error::Error,
    fs::{self, read_to_string, File},
    io::{ErrorKind, Read},
    path::{Path, PathBuf},
};

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

impl fmt::Display for HostnameGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padded_hostnames: Vec<String> = self.hostnames.iter()
            .map(|hostname| "  ".to_string() + hostname)
            .collect();
        write!(f, "Group: {}\n{}\n", self.name, padded_hostnames.join("\n"))
    }
}

impl std::default::Default for HostnameGroup {
    fn default() -> Self {
        Self {
            name: String::from("Social Media"),
            hostnames: vec![
                "facebook.com".to_string(),
                "instagram.com".to_string(),
                "twitter.com".to_string(),
                "linkedin.com".to_string(),
            ],
        }
    }
}

pub fn create_tmp_hosts_file() -> Result<(), Box<dyn Error>> {
    let result =
        read_to_string(HOSTS_FILE_PATH).and_then(|content| fs::write(TMP_HOSTS_FILE_PATH, content));
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to create tmp hosts file".into()),
    }
}

pub fn get_config_file_dir() -> Result<PathBuf, Box<dyn Error>> {
    let project_dirs = ProjectDirs::from("rs", APPNAME, APPNAME).unwrap();
    let path = project_dirs
        .config_dir()
        .join(Path::new(HOSTNAME_GROUPS_CONFIG_FILENAME));
    Ok(path)
}

pub fn read_hosts() -> Result<String, Box<dyn Error>> {
    match read_to_string(HOSTS_FILE_PATH) {
        Ok(hosts_content) => Ok(hosts_content),
        Err(_) => Err("Failed to read hosts file".into()),
    }
}

pub fn read_hostname_groups_config() -> Result<HostnameGroups, Box<dyn Error>> {
    let path = get_config_file_dir()?;

    match File::open(&path) {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            let hostname_groups = serde_json::from_str(&buf)?;
            Ok(hostname_groups)
        }
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            let parent = path.parent().unwrap();
            fs::create_dir_all(parent)?;
            let hostname_groups = vec![HostnameGroup::default()];
            let content = serde_json::to_string_pretty(&hostname_groups)?;
            fs::write(path, content)?;
            Ok(hostname_groups)
        }
        Err(e) => Err(e.into()),
    }
}

pub fn construct_refocus_line(hostgroups: &HostnameGroups) -> String {
    info!("Generating Refocus hosts line");
    let mut refocus_line = "127.0.0.1 ".to_owned() + HOSTNAME_ANCHOR + " ";
    for group in hostgroups {
        for hostname in &group.hostnames {
            refocus_line.push_str(hostname);
            refocus_line.push(' ');
            if !hostname.contains("www.") {
                let w = "www.".to_string() + hostname;
                refocus_line.push_str(&w);
                refocus_line.push(' ');
            }
        }
    }
    trace!("Refocus line: {:?}", refocus_line);
    refocus_line
}

pub fn copy_to_etc() -> Result<(), Box<dyn Error>> {
    let out = std::process::Command::new("sudo")
        .arg("cp")
        .arg(TMP_HOSTS_FILE_PATH)
        .arg(HOSTS_FILE_PATH)
        .output()?;
    if !out.status.success() {
        return Err("Failed to copy to /etc/hosts".into());
    }
    Ok(())
}

pub fn generate_new_hosts_file() -> Result<(), Box<dyn Error>> {
    let mut hosts_content = read_hosts()?;
    let hostgroups = read_hostname_groups_config()?;

    if hosts_content.contains(HOSTNAME_ANCHOR) {
        info!("Refocus Line found. Finding line to regenerate");
        let mut new_hosts_content = String::from("");
        for line in hosts_content.lines() {
            if line.contains(HOSTNAME_ANCHOR) {
                new_hosts_content.push_str(&construct_refocus_line(&hostgroups));
                new_hosts_content.push('\n');
            } else {
                new_hosts_content.push_str(line);
                new_hosts_content.push('\n');
            }
        }
        trace!("New hosts content: \n {:?}", new_hosts_content);

        fs::write(TMP_HOSTS_FILE_PATH, new_hosts_content)?;
    } else {
        info!("Refocus Line not found. Creating one and appending at end of file");
        hosts_content.push_str(&construct_refocus_line(&hostgroups));
        trace!("New hosts content: \n {:?}", hosts_content);

        fs::write(TMP_HOSTS_FILE_PATH, hosts_content)?;
    }

    Ok(())
}

pub fn overwrite_config_file(hostgroups: &HostnameGroups) -> Result<(), Box<dyn Error>> {
    let path = get_config_file_dir()?;
    let content = serde_json::to_string_pretty(&hostgroups)?;
    fs::write(path, content)?;
    Ok(())
}