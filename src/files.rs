use std::fs::File;
use std::io::{Read, Write, Error};



pub fn readf(path: &str) -> Result<Vec<u32>, Error> {
    let mut file = File::open(path)?;
    let mut strbuf = String::new();
    file.read_to_string(&mut strbuf)?;
    Ok(strbuf.split_whitespace().map(|e| e.parse::<u32>().unwrap()).collect())
}

pub fn writef(path: &str, data: Vec<u32>) -> Result<(), Error> {
    let mut file = File::create(path)?;
    let str_data: String = data.into_iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" ");
    file.write_all(str_data.as_bytes())?;
    Ok(())
}
