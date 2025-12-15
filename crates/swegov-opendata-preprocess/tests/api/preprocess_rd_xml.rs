// #[test]
// fn test_preprocess_xml() -> PreprocessResult<()> {
//     let example1_source_path = [env!("CARGO_MANIFEST_DIR"), "assets", "example1.xml"]
//         .iter()
//         .collect::<PathBuf>();
//     let mut example1_source = String::new();
//     let mut example1_source_file = fs::File::open(&example1_source_path).map_err(|error| {
//         PreprocessError::CouldNotReadFile {
//             path: example1_source_path.clone(),
//             error,
//         }
//     })?;
//     example1_source_file.read_to_string(&mut example1_source)?;

//     let actual = preprocess_xml(&example1_source, example1_source_path.to_string_lossy()).map_err(
//         |error| PreprocessError::XmlError {
//             path: example1_source_path.display().to_string(),
//             error,
//         },
//     )?;
//     let mut reader = Reader::from_reader(actual.as_slice());
//     let actual = Element::from_reader(&mut reader)
//         .map_err(|error| PreprocessError::custom(format!("Failed reading actual: {:?}", error)))?;

//     let example1_expected_path = "assets/example1.expected.xml";
//     let example1_expected_file = fs::File::open(example1_expected_path).map_err(|error| {
//         PreprocessError::CouldNotReadFile {
//             path: example1_expected_path.into(),
//             error,
//         }
//     })?;
//     let reader = BufReader::new(example1_expected_file);
//     let mut reader = Reader::from_reader(reader);
//     let expected = Element::from_reader(&mut reader)
//         .map_err(|error| PreprocessError::custom(format!("Failed read expecting: {:?}", error)))?;

//     assert_elem_equal_with_cleaning(&actual, &expected, &clean_text);
//     Ok(())
// }
