use std::{fs::File, io::Read, num::NonZeroU64};

use anyhow::{anyhow, bail};
use mirrorball_api::{CreateUploadRequest, CreateUploadResponse};
use sha2::{Digest, Sha256};

use crate::args::TransferFileArgs;
use crate::chunks;

pub fn transfer_file(args: TransferFileArgs) -> anyhow::Result<()> {
    let mut file = File::open(args.file)?;

    let mut bytes = Vec::new();
    let bytes_read = file.read_to_end(&mut bytes)?;

    if bytes_read == 0 {
        bail!("read 0 bytes from file");
    }

    let chunk_size = chunks::chunk_size(bytes_read as u64) as usize;

    let chunk_hashes: Vec<_> = bytes
        .chunks(chunk_size)
        .map(Sha256::digest)
        .map(|d| byte_slice_to_hex_string(&d))
        .collect();

    let req = CreateUploadRequest {
        file_size: NonZeroU64::new(bytes.len() as u64)
            .ok_or_else(|| anyhow!("file size is not a positive non-zero integer"))?,
        destination: args.destination,
        chunk_hashes,
    };

    let endpoint = format!("{}/api/v1/uploads/upload_request", args.server_address);

    let _ = ureq::post(endpoint)
        .send_json(req)?
        .body_mut()
        .read_json::<CreateUploadResponse>()?;

    Ok(())
}

// copied from mirrorball-server
fn byte_slice_to_hex_string(input: &[u8]) -> String {
    use std::fmt::Write;

    // Each byte is represented as 2 hex characters: 0A, 2C, FF, etc.
    // So we need 2 * the number of input bytes as the capacity of the output String.
    let mut output = String::with_capacity(input.len() * 2);

    for byte in input {
        // Writing a byte to a String should never fail
        write!(&mut output, "{byte:02x}").unwrap()
    }

    output
}
