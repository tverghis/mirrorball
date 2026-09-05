use std::path::PathBuf;

use anyhow::bail;
use lexopt::{
    Arg::{Long, Short, Value},
    Parser, ValueExt as _,
};

#[derive(Debug)]
pub enum Command {
    Help,
    TransferFile(TransferFileArgs),
}

#[derive(Debug)]
pub struct TransferFileArgs {
    server_address: String,
    destination: String,
    file: PathBuf,
}

impl Command {
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
                Short('h') | Long("help") => return Ok(Command::Help),
                _ => bail!("failed to parse arguments"),
            }
        }

        if server_address.is_none() || destination.is_none() || file.is_none() {
            bail!("required arguments not supplied");
        }

        let args = TransferFileArgs {
            server_address: server_address.unwrap(),
            destination: destination.unwrap(),
            file: file.unwrap(),
        };

        Ok(Self::TransferFile(args))
    }
}
