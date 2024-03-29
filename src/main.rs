use std::{error::Error, fs::read_to_string};

static HOSTS_FILE_PATH: &str = "/tmp/hosts";

fn read_hosts() -> Result<String, Box<dyn Error>>
{
    let hosts_content = read_to_string(HOSTS_FILE_PATH)?;
    Ok(hosts_content)
}

fn main() {
    let hosts = read_hosts().unwrap();
    println!("{}", hosts);
}
