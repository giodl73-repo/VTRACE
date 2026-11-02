use std::io::{self, Read};
use vtrace_package_inspector::{evaluate, REQUEST_BYTES};

fn main() -> io::Result<()> {
    let mut bytes = Vec::new();
    io::stdin()
        .take(REQUEST_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    let request = if bytes.len() > REQUEST_BYTES {
        // Admission precedes decoding; a bounded prefix can split valid UTF-8.
        " ".repeat(REQUEST_BYTES + 1)
    } else {
        String::from_utf8(bytes)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?
    };
    println!("{}", evaluate(&request));
    Ok(())
}
