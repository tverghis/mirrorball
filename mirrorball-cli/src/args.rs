use std::path::PathBuf;

use lexopt::{
    Arg::{Long, Short, Value},
    Parser, ValueExt as _,
};

const USAGE: &str = r#"USAGE: mirb -s SERVER_ADDRESS -d REMOTE_DESTINATION FILE

Send FILE to REMOTE_DESTINATION on the host specified by SERVER_ADDRESS.

OPTIONS:
  -s, --server-address SERVER_ADDRESS
      The address of the remote host running mirrorball-server.

  -d, --destination REMOTE_DESTINATION
      The absolute path to the destination on the remote host's
      filesystem to which FILE should be copied. Intermediate
      directories will be created if they do not already exist.

  -h, --help
      Print this message and exit.

EXAMPLES:
Copy photos.tgz to the host at "/foo/bar/photos.tar.gz":
  mirb -s http://my-server.example.com -d "/foo/bar/" photos.tar.gz"#;

#[derive(Debug)]
pub struct Args {
    server_address: String,
    destination: String,
    file: PathBuf,
}

impl Args {
    pub fn parse_from_env() -> anyhow::Result<Self> {
        let mut server_address = None;
        let mut destination = None;
        let mut file = None;

        let mut arg_parser = Parser::from_env();

        while let Some(arg) = arg_parser.next()? {
            match arg {
                Short('s') | Long("server-address") if server_address.is_none() => {
                    server_address = Some(arg_parser.value()?.string()?)
                }
                Short('d') | Long("destination") if destination.is_none() => {
                    destination = Some(arg_parser.value()?.string()?)
                }
                Value(f) if file.is_none() => file = Some(PathBuf::from(f)),
                Short('h') | Long("help") => print_usage_and_exit(0),
                _ => print_usage_and_exit(1),
            }
        }

        if server_address.is_none() || destination.is_none() || file.is_none() {
            print_usage_and_exit(1);
        }

        Ok(Self {
            server_address: server_address.unwrap(),
            destination: destination.unwrap(),
            file: file.unwrap(),
        })
    }
}

fn print_usage_and_exit(exit_code: i32) {
    if exit_code == 0 {
        println!("{USAGE}");
    } else {
        eprintln!("{USAGE}");
    }

    std::process::exit(exit_code);
}
