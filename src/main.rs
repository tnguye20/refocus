use std::{fs, process};
use log::{info, trace};
use refocus::*;

fn main() {

    info!("Copy from {} to {}", HOSTS_FILE_PATH, TMP_HOSTS_FILE_PATH);

    if let Err(_) = create_tmp_hosts_file()
    {
        eprintln!("Insufficient permission to backup {}", HOSTS_FILE_PATH);
        process::exit(1);
    }
    let mut hosts_content = read_hosts().unwrap();
    let hostgroups = read_hostname_groups_config().unwrap();

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

        fs::write(TMP_HOSTS_FILE_PATH, new_hosts_content).expect("Unable to write hosts file");
    }
    else
    {
        info!("Refocus Line not found. Creating one and appending at end of file");
        hosts_content.push_str(&construct_refocus_line(&hostgroups));
        trace!("New hosts content: \n {:?}", hosts_content);

        fs::write(TMP_HOSTS_FILE_PATH, hosts_content).expect("Unable to write hosts file");
    }

    if let Err(_) = fs::copy(TMP_HOSTS_FILE_PATH, HOSTS_FILE_PATH)
    {
        eprintln!("Insufficient permission to overwrite {}", HOSTS_FILE_PATH);
        process::exit(1);
    }
}
