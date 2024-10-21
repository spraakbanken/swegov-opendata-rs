use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use flate2::read::GzDecoder;

pub fn read_text(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();

    if path.extension().is_some_and(|ext| ext == "gz") {
        let mut gz = GzDecoder::new(&file);
        gz.read_to_string(&mut text)?;
    } else {
        file.read_to_string(&mut text)?;
    }
    Ok(text)
}

pub fn without_bom(s: &str) -> &str {
    if &s[0..3] == "\u{feff}" {
        &s[3..]
    } else {
        &s[..]
    }
}
