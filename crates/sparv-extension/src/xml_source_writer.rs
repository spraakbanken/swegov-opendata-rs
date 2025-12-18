use std::{io, path::Path};

use super::error::SparvError;
use fs_err as fs;

pub struct XmlSourceWriter<'a> {
    target_dir: &'a Path,
    counter: usize,
    result: Vec<Vec<u8>>,
    total_size: usize,
    stub: Option<&'a str>,
}

impl<'a> XmlSourceWriter<'a> {
    const MAX_SIZE: usize = 10 * 1024 * 1024;

    pub fn new(target_dir: &'a Path) -> Self {
        Self {
            target_dir,
            counter: 1,
            result: Vec::new(),
            total_size: 0,
            stub: None,
        }
    }

    pub fn with_target_and_counter(target_dir: &'a Path, counter: usize) -> Self {
        Self {
            target_dir,
            counter,
            result: Vec::default(),
            total_size: 0,
            stub: None,
        }
    }

    pub fn set_stub(&mut self, stub: Option<&'a str>) {
        self.stub = stub;
    }

    pub fn write(&mut self, xmlstring: Vec<u8>) -> Result<(), SparvError> {
        if xmlstring.is_empty() {
            return Ok(());
        }
        let this_size = xmlstring.len();

        // If adding the latest result would lead to the file size going over the limit, save
        if self.total_size + this_size > Self::MAX_SIZE {
            self.write_xml(&self.result, &self.target_dir.join(self.current_filename()))?;
            self.total_size = 0;
            self.result.clear();
            self.counter += 1;
        }
        self.result.push(xmlstring);
        self.total_size += this_size;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), SparvError> {
        if !self.result.is_empty() {
            self.write_xml(&self.result, &self.target_dir.join(self.current_filename()))?;
            self.result.clear();
        }
        Ok(())
    }

    pub fn current_filename(&self) -> String {
        let output_stub = self.output_stub();
        format!("{}-{}.xml", output_stub, self.counter)
    }

    fn output_stub(&self) -> &str {
        self.stub.unwrap_or_else(|| {
            self.target_dir
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("no-stub")
        })
    }
    fn write_xml(&self, texts: &[Vec<u8>], xmlpath: &Path) -> Result<(), SparvError> {
        use std::io::Write;
        if texts.is_empty() {
            tracing::debug!("no texts to writer, skipping the write");
            return Ok(());
        }
        let corpus_source_dir = xmlpath.parent().unwrap();
        fs::create_dir_all(corpus_source_dir).map_err(|source| {
            SparvError::CouldNotCreateFolder {
                path: xmlpath.display().to_string(),
                source,
            }
        })?;
        let xmlfile =
            fs::File::create(xmlpath).map_err(|source| SparvError::CouldNotCreateFile {
                path: xmlpath.display().to_string(),
                source,
            })?;
        let mut writer = io::BufWriter::new(xmlfile);
        writer
            .write(b"<file xmlns=\"\">\n")
            .map_err(|source| SparvError::CouldNotWriteToFile {
                path: xmlpath.display().to_string(),
                source,
            })?;
        for text in texts {
            writer
                .write(text)
                .map_err(|source| SparvError::CouldNotWriteToFile {
                    path: xmlpath.display().to_string(),
                    source,
                })?;
            writer
                .write(b"\n")
                .map_err(|source| SparvError::CouldNotWriteToFile {
                    path: xmlpath.display().to_string(),
                    source,
                })?;
        }
        writer
            .write(b"\n</file>")
            .map_err(|source| SparvError::CouldNotWriteToFile {
                path: xmlpath.display().to_string(),
                source,
            })?;
        Ok(())
    }
}
