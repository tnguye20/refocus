# Refocus

## Description
A simple program written in Rust to manipulate `etc/hosts` and redirect specific URLs to `localhost`, effectively blocking access to them.

## Usage
```sh
Usage: refocus [OPTIONS]

Options:
  -c, --config           show config file path
      --groups           show all host groups
  -g, --group <GROUP>    select host group [default: ]
  -f, --filter <FILTER>  filter host groups [default: ]
  -a, --add <ADD>        add hostname to group [default: ]
  -d, --delete <DELETE>  delete hostname [default: ]
  -t, --toggle <TOGGLE>  toogle hostgroup(s) [default: ]
  -e, --execute          execute refocus
  -h, --help             Print help
  -V, --version          Print version
```

## Build
Clone the repo and run 
```sh
cargo build
```

## Deploy
Run `build` script