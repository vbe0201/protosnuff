use ansi_term::Color::Red;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, String> {
    let path = path.as_ref();
    if !(path.exists() && path.is_file()) {
        return Err(
            format!("{:?} {}",
                    Red.paint(path.to_str().unwrap()),
                    Red.paint("is not a file or does not exist!"))
        );
    }

    let file = File::open(path).expect("File does not exist!");

    Ok(BufReader::new(file))
}

pub fn read_bytes<R>(reader: &mut R, bytes_to_read: u64) -> Vec<u8>
    where R: Read,
{
    let mut buffer = vec![];
    let mut chunk = reader.take(bytes_to_read);

    let amount = chunk.read_to_end(&mut buffer).expect("Didn't read enough!");
    assert_eq!(bytes_to_read as usize, amount);

    buffer
}
