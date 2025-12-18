use chrono::NaiveDate;
use xml::{EventReader, ParserConfig};
use yaserde::YaDeserialize;

use crate::date_formats::SweDateTime;

use super::*;

fn dataset_anforande_202324() -> DataSet {
    DataSet {
        namn: "anforande".into(),
        typ: "anforande".into(),
        samling: "anforande-202324".into(),
        rm: "2023/24".into(),
        filnamn: "anforande-202324.csv.zip".into(),
        storlek_bytes: 213476,
        format: DataFormat::Csv,
        filformat: FilFormat::Zip,
        uppdaterad: SweDateTime::from(
            NaiveDate::from_ymd_opt(2024, 9, 26)
                .unwrap()
                .and_hms_opt(2, 1, 45)
                .unwrap(),
        ),
        url: "/dataset/anforande/anforande-202324.csv.zip".into(),
        description: Some("Anföranden som ledamöter hållit i kammaren.".into()),
        beskrivning: Some("Anföranden för riksmöte 2023/24".into()),
        upplysning: upplysning_anforande_202334().into(),
    }
}
fn dataset_anforande_202223() -> DataSet {
    DataSet {
        namn: "anforande".into(),
        typ: "anforande".into(),
        samling: "anforande-202223".into(),
        rm: "2022/23".into(),
        filnamn: "anforande-202223.csvt.zip".into(),
        storlek_bytes: 9428358,
        format: DataFormat::CsvT,
        filformat: FilFormat::Zip,
        uppdaterad: SweDateTime::from(
            NaiveDate::from_ymd_opt(2024, 09, 26)
                .unwrap()
                .and_hms_opt(02, 00, 27)
                .unwrap(),
        ),
        url: "/dataset/anforande/anforande-202223.csvt.zip".into(),
        description: Some("Anföranden som ledamöter hållit i kammaren.".into()),
        beskrivning: None,
        upplysning: None,
    }
}
fn dataset_bet_1971_1979() -> DataSet {
    DataSet {
        namn: "dokument".into(),
        typ: "bet".into(),
        samling: "bet-1971-1979".into(),
        rm: "1971-1979".into(),
        filnamn: "bet-1971-1979.xml.zip".into(),
        storlek_bytes: 43958334,
        format: DataFormat::Xml,
        filformat: FilFormat::Zip,
        uppdaterad:  SweDateTime::from(
                          NaiveDate::from_ymd_opt(2015, 04, 27)
            .unwrap()
            .and_hms_opt(03, 59, 11)
            .unwrap()),
        url: "/dataset/dokument/bet-1971-1979.xml.zip".into(),
        description:
        Some("Utskottens betänkanden och utlåtanden, inklusive rksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet. I vissa årgångar finns även debatten med i formaten JSON, SQL och XML.".into())
        ,
        beskrivning: None,
        upplysning: Some(upplysning_bet_1971_1979())

    }
}
fn upplysning_bet_1971_1979() -> Upplysning {
    Upplysning {
        upplysning: "Saknade dokument: ".into(),
        year_comment: YearCommentMap {
            year_comments: vec![
                YearComment {
                    year: "1975/76".into(),
                    comment: "alla".into(),
                },
                YearComment {
                    year: "1976/77".into(),
                    comment: "FiU,NU".into(),
                },
            ],
        },
    }
}

fn upplysning_anforande_202334() -> Upplysning {
    Upplysning {
        upplysning: "Samtliga anföranden saknas. Vi arbetar på att åtgärda problemet (2024-04-22)."
            .into(),
        year_comment: Default::default(),
    }
}

#[test]
fn dataset_serialize_deserialize_xml() -> anyhow::Result<()> {
    let value = dataset_bet_1971_1979();

    let buffer = yaserde::ser::to_string(&value).unwrap();

    insta::assert_snapshot!("dataset_ser_xml", buffer.as_str());

    let mut xml_reader = yaserde::de::Deserializer::new(EventReader::new_with_config(
        buffer.as_bytes(),
        ParserConfig::new().trim_whitespace(false),
    ));
    let actual = DataSet::deserialize(&mut xml_reader).unwrap();
    // let actual: DataSet = yaserde::de::from_str(&buffer).unwrap();
    similar_asserts::assert_eq!(actual, value);
    Ok(())
}

#[test]
fn upplysning_ser_de_xml() -> anyhow::Result<()> {
    let value = upplysning_bet_1971_1979();
    let buffer = yaserde::ser::to_string(&value).unwrap();

    insta::assert_snapshot!("upplysning_ser_xml", buffer.as_str());

    let mut xml_reader = yaserde::de::Deserializer::new(EventReader::new_with_config(
        buffer.as_bytes(),
        ParserConfig::new().trim_whitespace(false),
    ));
    let actual = Upplysning::deserialize(&mut xml_reader).unwrap();
    similar_asserts::assert_eq!(actual, value);
    Ok(())
}

#[test]
fn datalista_de_ser_xml() -> anyhow::Result<()> {
    let value = DatasetLista {
        dataset: vec![
            dataset_anforande_202324(),
            dataset_anforande_202223(),
            dataset_bet_1971_1979(),
        ],
    };

    let buffer = yaserde::ser::to_string(&value).unwrap();

    insta::assert_snapshot!("datalista_ser_xml", buffer.as_str());

    let mut xml_reader = yaserde::de::Deserializer::new(EventReader::new_with_config(
        buffer.as_bytes(),
        ParserConfig::new().trim_whitespace(false),
    ));
    let actual = DatasetLista::deserialize(&mut xml_reader).unwrap();
    similar_asserts::assert_eq!(actual, value);
    Ok(())
}

#[test]
fn datalista_de_ser_json() -> anyhow::Result<()> {
    let value = DatasetLista {
        dataset: vec![
            dataset_anforande_202324(),
            dataset_anforande_202223(),
            dataset_bet_1971_1979(),
        ],
    };

    let buffer = serde_json::to_string(&value)?;

    insta::assert_snapshot!("datalista_ser_json", buffer.as_str());

    let actual: DatasetLista = serde_json::from_str(&buffer)?;
    similar_asserts::assert_eq!(actual, value);
    Ok(())
}
