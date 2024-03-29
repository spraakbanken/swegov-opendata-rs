use std::io::BufReader;
use std::path::PathBuf;
use std::{fs, io::Read};

use error_stack::ResultExt;
use minidom::quick_xml::reader::Reader;
use minidom::Element;
use minidom_extension::{asserts::assert_elem_equal_with_cleaning, minidom};
use swegov_opendata_preprocess::preprocess_rd::preprocess_xml;
use swegov_opendata_preprocess::shared::clean_text;
use swegov_opendata_preprocess::{PreprocessError, PreprocessResult};

#[test]
fn test_preprocess_xml() -> PreprocessResult<()> {
    let example1_source_path = [env!("CARGO_MANIFEST_DIR"), "assets", "example1.xml"]
        .iter()
        .collect::<PathBuf>();
    let mut example1_source = String::new();
    let mut example1_source_file = fs::File::open(&example1_source_path)
        .change_context(PreprocessError)
        .attach_printable_lazy(|| format!("Reading {}", example1_source_path.display()))?;
    example1_source_file
        .read_to_string(&mut example1_source)
        .change_context(PreprocessError)
        .attach_printable("failed to read source")?;

    let actual = preprocess_xml(&example1_source, example1_source_path.to_string_lossy())
        .change_context(PreprocessError)?;
    let mut reader = Reader::from_reader(actual.as_slice());
    let actual = Element::from_reader(&mut reader)
        .change_context(PreprocessError)
        .attach_printable("failed to read actual")?;

    let example1_expected_path = "assets/example1.expected.xml";
    let example1_expected_file = fs::File::open(example1_expected_path)
        .change_context(PreprocessError)
        .attach_printable_lazy(|| format!("{}", example1_expected_path))?;
    let reader = BufReader::new(example1_expected_file);
    let mut reader = Reader::from_reader(reader);
    let expected = Element::from_reader(&mut reader)
        .change_context(PreprocessError)
        .attach_printable("failed to read expected")?;

    assert_elem_equal_with_cleaning(&actual, &expected, &clean_text);
    Ok(())
}
