use fs_err as fs;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

use minidom_extension::minidom::{quick_xml::reader::Reader, Element};
use swegov_opendata_preprocess::preprocess_sfs::{build_sparv_source, sfs_json};
use swegov_opendata_preprocess::{PreprocessError, PreprocessResult};

#[test]
fn test_preprocess_sfs_json() -> PreprocessResult<()> {
    // Arrange
    let example1_source_path = [
        env!("CARGO_MANIFEST_DIR"),
        "assets",
        "sfs-1976",
        "sfs-1976-257.json",
    ]
    .iter()
    .collect::<PathBuf>();
    let mut example1_source = String::new();
    let mut example1_source_file = fs::File::open(&example1_source_path).map_err(|error| {
        PreprocessError::CouldNotReadFile {
            path: example1_source_path.clone(),
            error,
        }
    })?;
    example1_source_file.read_to_string(&mut example1_source)?;

    // Act
    let actual = sfs_json::preprocess_json(&example1_source).map_err(|error| {
        PreprocessError::SfsPreprocessError {
            path: example1_source_path.clone(),
            error,
        }
    })?;

    // Assert
    let mut reader = Reader::from_reader(actual.as_slice());
    let _actual = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("Failed reading actual: {:?}", error)))?;

    let example1_expected_path = "assets/sfs-1976-257.expected.xml";
    let example1_expected_file = fs::File::open(example1_expected_path).map_err(|error| {
        PreprocessError::CouldNotReadFile {
            path: example1_expected_path.into(),
            error,
        }
    })?;
    let reader = BufReader::new(example1_expected_file);
    let mut reader = Reader::from_reader(reader);
    let _expected = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("Failed read expecting: {}", error)))?;

    // assert_elem_equal(&actual, &expected);
    Ok(())
}

#[test]
fn test_build_sparv_source_sfs_1976() -> anyhow::Result<()> {
    // Arrange
    let assets_path = [env!("CARGO_MANIFEST_DIR"), "assets"]
        .iter()
        .collect::<PathBuf>();
    let example1_source_path = assets_path.join("sfs-1976");
    let corpus_source_dir = assets_path.join("gen").join("sfs-1976");

    // Act
    build_sparv_source(&example1_source_path, &corpus_source_dir)?;

    // Assert
    let actual_path = "assets/gen/sfs-1976/sfs-1976-1.xml";
    let actual_content = fs::read_to_string(actual_path)?;
    insta::assert_snapshot!(actual_content);

    Ok(())
}

#[test]
fn test_build_sparv_source_sfs_1994() -> anyhow::Result<()> {
    // Arrange
    let assets_path = [env!("CARGO_MANIFEST_DIR"), "assets"]
        .iter()
        .collect::<PathBuf>();
    let example1_source_path = assets_path.join("sfs-1994");
    let corpus_source_dir = assets_path.join("gen").join("sfs-1994");

    // Act
    build_sparv_source(&example1_source_path, &corpus_source_dir)?;

    // Assert
    let actual_path = "assets/gen/sfs-1994/sfs-1994-1.xml";
    let actual_content = fs::read_to_string(actual_path)?;

    insta::assert_snapshot!(actual_content);

    Ok(())
}

#[test]
fn test_build_sparv_source_cks6riksg() -> anyhow::Result<()> {
    // Arrange
    let assets_path = [env!("CARGO_MANIFEST_DIR"), "assets"]
        .iter()
        .collect::<PathBuf>();
    let example1_source_path = assets_path.join("cks6riksg");
    let corpus_source_dir = assets_path.join("gen").join("cks6riksg");

    // Act
    build_sparv_source(&example1_source_path, &corpus_source_dir)?;

    // Assert
    let actual_path = "assets/gen/cks6riksg/cks6riksg-1.xml";
    let actual_content = fs::read_to_string(actual_path)?;

    insta::assert_snapshot!(actual_content);

    Ok(())
}
