use clap::Parser;
use refocus::*;
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// show config file path
    #[arg(short, long, default_value_t = false)]
    config: bool,

    /// show all host groups
    #[arg(long, default_value_t = false)]
    groups: bool,

    /// select host group
    #[arg(short, long, default_value_t = String::from(""))]
    group: String,

    /// filter host groups
    #[arg(short, long, default_value_t = String::from(""))]
    filter: String,

    /// add hostname to group
    #[arg(short, long, default_value_t = String::from(""))]
    add: String,

    /// delete hostname
    #[arg(short, long, default_value_t = String::from(""))]
    delete: String,

    /// toogle hostgroup(s)
    #[arg(short, long, default_value_t = String::from(""))]
    toggle: String,

    /// execute refocus
    #[arg(short, long, default_value_t = true)]
    execute: bool,
}

fn main() {
    let args = Args::parse();

    if args.config {
        match get_config_file_dir() {
            Ok(path) => {
                println!("Config file path: {:?}", path);
            }
            Err(e) => {
                eprintln!("Failed to get config file path: {}", e);
                process::exit(1);
            }
        }
        return;
    }

    if args.groups {
        match read_hostname_groups_config() {
            Ok(groups) => {
                groups
                    .iter()
                    .filter(|group| {
                        args.filter.is_empty() || group.name.to_lowercase().contains(&args.filter)
                    })
                    .for_each(|group| println!("{}", group));
            }
            Err(e) => {
                eprintln!("Failed to read hostname groups config: {}", e);
                process::exit(1);
            }
        }
        return;
    }

    if !args.add.is_empty() && args.group.is_empty() {
        eprintln!("Cannot add hostname to group without specifying group");
        process::exit(1);
    }

    if !args.toggle.is_empty() {
        match read_hostname_groups_config() {
            Ok(mut groups) => {
                let toggle_groups = split_args(&args.toggle);

                for toggle_group in &toggle_groups {
                    for group in groups.iter_mut() {
                        if group.name.to_lowercase() == toggle_group.to_lowercase() {
                            group.disabled = Some(!group.disabled.unwrap_or(false));
                            println!("Toggled group: {}. Disabled: {}", group.name, group.disabled.unwrap_or(false));
                        }
                    }
                }

                if overwrite_config_file(&groups).is_err() {
                    eprintln!("Failed to update hostname groups config");
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to read hostname groups config: {}", e);
                process::exit(1);
            }
        }
    }

    if !args.group.is_empty() && !args.add.is_empty() {
        match read_hostname_groups_config() {
            Ok(mut groups) => {
                let new_hostnames = split_args(
                        &args.add
                        .trim()
                    )
                    .into_iter()
                    // .replace(' ', "")
                    .filter(|hostname| hostname.contains('.'))
                    .collect::<Vec<String>>();

                if let Some(group) = groups
                    .iter_mut()
                    .find(|group| group.name.to_lowercase() == args.group.to_lowercase())
                {
                    group.hostnames.extend(new_hostnames);
                } else {
                    groups.push(HostnameGroup::new(args.group, new_hostnames));
                }

                if overwrite_config_file(&groups).is_err() {
                    eprintln!("Failed to update hostname groups config");
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to read hostname groups config: {}", e);
                process::exit(1);
            }
        }
    }

    if !args.delete.is_empty() {
        match read_hostname_groups_config() {
            Ok(mut groups) => {
                let delete_hostnames: Vec<String> = split_args(&args.delete);

                for delete_hostname in delete_hostnames {
                    for group in groups.iter_mut() {
                        group
                            .hostnames
                            .retain(|hostname| hostname.to_lowercase() != delete_hostname);
                    }
                }

                if overwrite_config_file(&groups).is_err() {
                    eprintln!("Failed to update hostname groups config");
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to read hostname groups config: {}", e);
                process::exit(1);
            }
        }
    }

    if args.execute {
        let result = create_tmp_hosts_file()
            .and(generate_new_hosts_file())
            .and(copy_to_etc());

        match result {
            Ok(_) => {
                println!("Refocus ran successful");
            }
            Err(e) => {
                eprintln!("Failed to execute refocus: {}", e);
                process::exit(1);
            }
        }
    }
}
