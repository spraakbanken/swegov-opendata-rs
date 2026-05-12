use exn::Exn;
use exn::ResultExt;
use fs_err as fs;
use preprocess_sfs::preprocess_sfs::sfs_json::PreprocessJsonResponse;
use preprocess_sfs::preprocess_sfs::SfsSparvSourceOptions;
use std::io::BufReader;
use std::path::PathBuf;

use minidom_extension::minidom::{quick_xml::reader::Reader, Element};
use preprocess_sfs::preprocess_sfs::{build_sparv_source, sfs_json};

#[derive(Debug, thiserror::Error)]
#[error("Test failed")]
pub struct TestError;

#[test]
fn test_preprocess_sfs_json() -> Result<(), Exn<TestError>> {
    // Arrange
    let example1_source_path = "assets/sfs-1976/sfs-1976-257.json";
    let example1_source = fs::read_to_string(example1_source_path).or_raise(|| TestError)?;

    // Act
    let actual = match sfs_json::preprocess_json(&example1_source).or_raise(|| TestError)? {
        PreprocessJsonResponse::Actual(xmlstring) => xmlstring,
        PreprocessJsonResponse::Former(xmlstring) => xmlstring,
    };

    // Assert
    let mut reader = Reader::from_reader(actual.as_slice());
    let _actual = Element::from_reader(&mut reader).or_raise(|| TestError)?;

    let example1_expected_path = "assets/sfs-1976-257.expected.xml";
    let example1_expected_file = fs::File::open(example1_expected_path).or_raise(|| TestError)?;
    let reader = BufReader::new(example1_expected_file);
    let mut reader = Reader::from_reader(reader);
    let _expected = Element::from_reader(&mut reader).or_raise(|| TestError)?;

    // assert_elem_equal(&actual, &expected);
    Ok(())
}

#[test]
fn test_build_sparv_source_sfs_1976() -> Result<(), Exn<TestError>> {
    // Arrange
    let assets_path = PathBuf::from("assets");
    let example1_source_path = assets_path.join("sfs-1976");
    let corpus_source_dir = assets_path.join("gen").join("sfs-1976");

    // Act
    build_sparv_source(
        &example1_source_path,
        SfsSparvSourceOptions {
            source_dir_actual: &corpus_source_dir,
            source_dir_former: &corpus_source_dir,
        },
    )
    .or_raise(|| TestError)?;

    // Assert
    let actual_path = "assets/gen/sfs-1976/sfs-1976-1.xml";
    let actual_content = fs::read_to_string(actual_path).or_raise(|| TestError)?;
    insta::assert_snapshot!(actual_content);

    Ok(())
}

#[test]
fn test_build_sparv_source_sfs_1994() -> Result<(), Exn<TestError>> {
    // Arrange
    let assets_path = PathBuf::from("assets");
    let example1_source_path = assets_path.join("sfs-1994");
    let corpus_source_dir = assets_path.join("gen").join("sfs-1994");

    // Act
    build_sparv_source(
        &example1_source_path,
        SfsSparvSourceOptions {
            source_dir_actual: &corpus_source_dir,
            source_dir_former: &corpus_source_dir,
        },
    )
    .or_raise(|| TestError)?;

    // Assert
    let actual_path = "assets/gen/sfs-1994/sfs-1994-1.xml";
    let actual_content = fs::read_to_string(actual_path).or_raise(|| TestError)?;

    insta::assert_snapshot!(actual_content);

    Ok(())
}

#[test]
fn test_build_sparv_source_cks6riksg() -> Result<(), Exn<TestError>> {
    // Arrange
    let assets_path = PathBuf::from("assets");
    let example1_source_path = assets_path.join("cks6riksg");
    let corpus_source_dir = assets_path.join("gen").join("cks6riksg");

    // Act
    build_sparv_source(
        &example1_source_path,
        SfsSparvSourceOptions {
            source_dir_actual: &corpus_source_dir,
            source_dir_former: &corpus_source_dir,
        },
    )
    .or_raise(|| TestError)?;

    // Assert
    let actual_path = "assets/gen/cks6riksg/cks6riksg-1.xml";
    let actual_content = fs::read_to_string(actual_path).or_raise(|| TestError)?;

    insta::assert_snapshot!(actual_content);

    Ok(())
}
