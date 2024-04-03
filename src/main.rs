use refocus::*;
use std::process;

fn main() {
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
