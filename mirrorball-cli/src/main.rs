use crate::args::Command;

mod api;
mod args;

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

fn main() -> anyhow::Result<()> {
    let cmd = match Command::parse_from_env() {
        Err(e) => {
            eprintln!("Error parsing arguments: {e}.\n");
            print_usage();
            std::process::exit(1);
        }
        Ok(args) => args,
    };

    match cmd {
        Command::Help => print_usage(),
        Command::TransferFile(args) => api::transfer_file(args)?,
    };

    Ok(())
}

fn print_usage() {
    println!("{USAGE}");
}
