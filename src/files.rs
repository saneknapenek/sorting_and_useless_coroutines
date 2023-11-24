use std::fs::File;
use std::io::{Read, Write, Error, Seek};
use std::os::unix::fs::FileExt;


pub fn sreadf(path: &str) -> Result<Vec<u32>, Error> {
    let mut file = File::open(path)?;
    let mut strbuf = String::new();
    file.read_to_string(&mut strbuf)?;
    Ok(strbuf.split_whitespace().map(|e| e.parse::<u32>().unwrap()).collect())
}

pub fn swritef(path: &str, data: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn swrite_to_end(path: &str, data: &str) -> Result<(), Error> {
    let mut file = File::options().append(true).open(&path)?;
    let size = file.seek(std::io::SeekFrom::End(0))?;
    file.write_at(" ".as_bytes(), size)?;
    file.write_at(data.as_bytes(), size)?;
    Ok(())
}
