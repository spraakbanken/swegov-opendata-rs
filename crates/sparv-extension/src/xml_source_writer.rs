use std::{fs, io, path::Path};

use error_stack::ResultExt;

use super::error::SparvError;

pub struct XmlSourceWriter<'a> {
    target_dir: &'a Path,
    counter: usize,
    result: Vec<Vec<u8>>,
    total_size: usize,
}

impl<'a> XmlSourceWriter<'a> {
    const MAX_SIZE: usize = 10 * 1024 * 1024;

    pub fn new(target_dir: &'a Path) -> Self {
        Self {
            target_dir,
            counter: 1,
            result: Vec::new(),
            total_size: 0,
        }
    }

    pub fn with_target_and_counter(target_dir: &'a Path, counter: usize) -> Self {
        Self {
            target_dir,
            counter,
            result: Vec::default(),
            total_size: 0,
        }
    }

    pub fn write(&mut self, xmlstring: Vec<u8>) -> error_stack::Result<(), SparvError> {
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

    pub fn flush(&mut self) -> error_stack::Result<(), SparvError> {
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
        self.target_dir
            .file_name()
            .map(|s| s.to_str())
            .flatten()
            .unwrap_or("no-stub")
    }
    fn write_xml(&self, texts: &[Vec<u8>], xmlpath: &Path) -> error_stack::Result<(), SparvError> {
        use std::io::Write;
        let corpus_source_dir = xmlpath.parent().unwrap();
        fs::create_dir_all(corpus_source_dir).change_context(SparvError)?;
        let xmlfile = fs::File::create(xmlpath).change_context(SparvError)?;
        let mut writer = io::BufWriter::new(xmlfile);
        writer
            .write(b"<file xmlns=\"\">\n")
            .change_context(SparvError)?;
        for text in texts {
            writer.write(text).change_context(SparvError)?;
            writer.write(b"\n").change_context(SparvError)?;
        }
        writer.write(b"\n</file>").change_context(SparvError)?;
        Ok(())
    }
}
