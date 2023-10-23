use std::io::BufReader;
use std::path::PathBuf;
use std::{fs, io::Read};

use error_stack::ResultExt;
use minidom::quick_xml::reader::Reader;
use minidom::Element;
use swegov_opendata_preprocess::nodeinfo::minidom::asserts::assert_elem_equal;
use swegov_opendata_preprocess::preprocess::sfs;
use swegov_opendata_preprocess::{PreprocessError, PreprocessResult};

#[test]
fn test_preprocess_sfs_json() -> PreprocessResult<()> {
    let example1_source_path = [env!("CARGO_MANIFEST_DIR"), "assets", "sfs-1976-257.json"]
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

    let actual = sfs::preprocess_json(&example1_source).change_context(PreprocessError)?;
    let mut reader = Reader::from_reader(actual.as_slice());
    let actual = Element::from_reader(&mut reader)
        .change_context(PreprocessError)
        .attach_printable("failed to read actual")?;

    let example1_expected_path = "assets/sfs-1976-257.expected.xml";
    let example1_expected_file = fs::File::open(example1_expected_path)
        .change_context(PreprocessError)
        .attach_printable_lazy(|| format!("{}", example1_expected_path))?;
    let reader = BufReader::new(example1_expected_file);
    let mut reader = Reader::from_reader(reader);
    let expected = Element::from_reader(&mut reader)
        .change_context(PreprocessError)
        .attach_printable("failed to read expected")?;

    assert_elem_equal(&actual, &expected);
    Ok(())
}
