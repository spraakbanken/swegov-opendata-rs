use std::io::BufReader;
use std::path::PathBuf;
use std::{fs, io::Read};

use minidom_extension::asserts::assert_elem_equal_with_cleaning;
use minidom_extension::minidom::{quick_xml::reader::Reader, Element};
use swegov_opendata_preprocess::preprocess_sfs::{build_sparv_source, sfs_json};
use swegov_opendata_preprocess::shared::{clean_element, clean_text};
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
fn test_build_sparv_source_sfs_1976() -> PreprocessResult<()> {
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
    let actual_file = fs::File::open(actual_path).map_err(|error| {
        PreprocessError::custom(format!(
            "failed to read actual from '{}': {:?}",
            actual_path, error
        ))
    })?;
    let reader = BufReader::new(actual_file);
    let mut reader = Reader::from_reader(reader);
    let actual = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("failed to read actual: {:?}", error)))?;

    let example1_expected_path = "assets/sfs-1976-257.expected.xml";
    let example1_expected_file = fs::File::open(example1_expected_path).map_err(|error| {
        PreprocessError::CouldNotReadFile {
            path: example1_expected_path.into(),
            error,
        }
    })?;
    let reader = BufReader::new(example1_expected_file);
    let mut reader = Reader::from_reader(reader);
    let expected = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("Failed read expecting: {:?}", error)))?;

    assert_elem_equal_with_cleaning(&actual, &expected, &clean_text);
    Ok(())
}

#[test]
fn test_build_sparv_source_sfs_1994() -> PreprocessResult<()> {
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
    let actual_file = fs::File::open(actual_path).map_err(|error| {
        PreprocessError::custom(format!(
            "failed to read actual from '{}': {:?}",
            actual_path, error
        ))
    })?;
    let reader = BufReader::new(actual_file);
    let mut reader = Reader::from_reader(reader);
    let actual = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("failed to read actual: {:?}", error)))?;

    let example1_expected_path = "assets/sfs-1994-448.expected.xml";
    let example1_expected_file = fs::File::open(example1_expected_path).map_err(|error| {
        PreprocessError::CouldNotReadFile {
            path: example1_expected_path.into(),
            error,
        }
    })?;
    let reader = BufReader::new(example1_expected_file);
    let mut reader = Reader::from_reader(reader);
    let expected = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("Failed read expecting: {:?}", error)))?;

    assert_elem_equal_with_cleaning(&actual, &expected, &clean_text);
    Ok(())
}

#[test]
fn test_build_sparv_source_cks6riksg() -> PreprocessResult<()> {
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
    let actual_file = fs::File::open(actual_path).map_err(|error| {
        PreprocessError::custom(format!(
            "failed to read actual from '{}': {:?}",
            actual_path, error
        ))
    })?;
    let reader = BufReader::new(actual_file);
    let mut reader = Reader::from_reader(reader);
    let actual = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("failed to read actual: {:?}", error)))?;

    let example1_expected_path = "assets/cks6riksg.expected.xml";
    let example1_expected_file = fs::File::open(example1_expected_path).map_err(|error| {
        PreprocessError::CouldNotReadFile {
            path: example1_expected_path.into(),
            error,
        }
    })?;
    let reader = BufReader::new(example1_expected_file);
    let mut reader = Reader::from_reader(reader);
    let expected = Element::from_reader(&mut reader)
        .map_err(|error| PreprocessError::custom(format!("Failed read expecting: {:?}", error)))?;
    let expected = clean_element(&expected);
    assert_elem_equal_with_cleaning(&actual, &expected, &clean_text);
    Ok(())
}
