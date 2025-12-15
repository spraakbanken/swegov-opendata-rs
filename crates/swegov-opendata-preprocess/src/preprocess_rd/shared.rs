use std::{fs, io, path::Path};

use crate::PreprocessError;

pub fn read_json_or_default<T>(path: &Path) -> Result<T, PreprocessError>
where
    T: Default + serde::de::DeserializeOwned,
{
    let json: T = match fs::File::open(path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            serde_json::from_reader(reader).map_err(|error| PreprocessError::CouldNotReadJson {
                path: path.to_path_buf(),
                error,
            })?
        }

        Err(_) => T::default(),
    };
    Ok(json)
}

pub fn write_json<T>(path: &Path, value: &T) -> Result<(), PreprocessError>
where
    T: serde::Serialize,
{
    let file = fs::File::create(path)?;
    let writer = io::BufWriter::new(file);
    serde_json::to_writer(writer, value).map_err(|error| PreprocessError::CouldNotWriteJson {
        path: path.to_path_buf(),
        error,
    })?;

    Ok(())
}
