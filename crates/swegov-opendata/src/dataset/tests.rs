use chrono::NaiveDate;
use deserx::{DeXml, SerXml};

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
        uppdaterad: NaiveDate::from_ymd_opt(2024, 09, 26)
            .unwrap()
            .and_hms_opt(02, 01, 45)
            .unwrap(),
        url: "/dataset/anforande/anforande-202324.csv.zip".into(),
        description: "Anföranden som ledamöter hållit i kammaren.".into(),
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
        uppdaterad: NaiveDate::from_ymd_opt(2024, 09, 26)
            .unwrap()
            .and_hms_opt(02, 00, 27)
            .unwrap(),
        url: "/dataset/anforande/anforande-202223.csvt.zip".into(),
        description: "Anföranden som ledamöter hållit i kammaren.".into(),
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
        uppdaterad: NaiveDate::from_ymd_opt(2015, 04, 27)
            .unwrap()
            .and_hms_opt(03, 59, 11)
            .unwrap(),
        url: "/dataset/dokument/bet-1971-1979.xml.zip".into(),
        description:
        "Utskottens betänkanden och utlåtanden, inklusive rksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet. I vissa årgångar finns även debatten med i formaten JSON, SQL och XML.".into(),
        beskrivning: None,
        upplysning: Some(upplysning_bet_1971_1979())

    }
}
fn upplysning_bet_1971_1979() -> Upplysning {
    Upplysning {
        upplysning: "Saknade dokument: ".into(),
        year_comment: [
            ("1975/76".into(), "alla".into()),
            ("1976/77".into(), "FiU,NU".into()),
        ]
        .into(),
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
fn dataset_serialize_deserialize() -> anyhow::Result<()> {
    let value = dataset_bet_1971_1979();

    let mut buffer = Vec::new();
    let mut writer = quick_xml::Writer::new(&mut buffer);
    value.serialize_xml(&mut writer)?;
    let buffer = String::from_utf8(buffer)?;
    dbg!(&buffer);
    let s_expected = "<dataset><namn>dokument</namn><typ>bet</typ><samling>bet-1971-1979</samling><rm>1971-1979</rm><filnamn>bet-1971-1979.xml.zip</filnamn><storlek>43958334</storlek><format>xml</format><filformat>zip</filformat><uppdaterad>2015-04-27 03:59:11</uppdaterad><url>/dataset/dokument/bet-1971-1979.xml.zip</url><description>Utskottens betänkanden och utlåtanden, inklusive rksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet. I vissa årgångar finns även debatten med i formaten JSON, SQL och XML.</description><beskrivning/><upplysning>Saknade dokument: <br/>1975/76: alla<br/>1976/77: FiU,NU</upplysning></dataset>";
    similar_asserts::assert_eq!(buffer, s_expected);

    // let mut reader = quick_xml::Reader::from_str(s_expected);
    let mut reader = quick_xml::NsReader::from_str(&buffer);
    let actual = DataSet::deserialize_xml(&mut reader)?;
    // let actual = DataSet = quick_xml::de::from_str(s_expected)?;
    dbg!(&actual);
    similar_asserts::assert_eq!(actual, value);
    Ok(())
}

#[test]
fn upplysning_ser_de() -> anyhow::Result<()> {
    let value = upplysning_bet_1971_1979();
    let mut buffer = Vec::new();
    let mut writer = quick_xml::Writer::new(&mut buffer);
    value.serialize_xml(&mut writer)?;
    let buffer = String::from_utf8(buffer)?;
    // let s = quick_xml::se::to_string_with_root("upplysning", &value)?;
    // dbg!(&s);
    dbg!(&buffer);
    let se_expected =
        "<upplysning>Saknade dokument: <br/>1975/76: alla<br/>1976/77: FiU,NU</upplysning>";
    similar_asserts::assert_eq!(buffer, se_expected);
    let mut reader = quick_xml::NsReader::from_str(&buffer);
    let actual = Upplysning::deserialize_xml(&mut reader)?;
    similar_asserts::assert_eq!(actual, value);
    Ok(())
}

#[test]
fn datalista_de_ser() -> anyhow::Result<()> {
    let value = DatasetLista {
        dataset: vec![
            dataset_anforande_202324(),
            dataset_anforande_202223(),
            dataset_bet_1971_1979(),
        ],
    };

    let mut buffer = Vec::new();
    let mut writer = quick_xml::Writer::new(&mut buffer);
    value.serialize_xml(&mut writer)?;
    let buffer = String::from_utf8(buffer)?;
    let se_expected = "<datasetlista>\
        <dataset>\
        <namn>anforande</namn>\
        <typ>anforande</typ><samling>anforande-202324</samling><rm>2023/24</rm><filnamn>anforande-202324.csv.zip</filnamn><storlek>213476</storlek><format>csv</format><filformat>zip</filformat><uppdaterad>2024-09-26 02:01:45</uppdaterad><url>/dataset/anforande/anforande-202324.csv.zip</url><description>Anföranden som ledamöter hållit i kammaren.</description><beskrivning>Anföranden för riksmöte 2023/24</beskrivning><upplysning>Samtliga anföranden saknas. Vi arbetar på att åtgärda problemet (2024-04-22).</upplysning></dataset><dataset>\
    <namn>anforande</namn>\
    <typ>anforande</typ>\
    <samling>anforande-202223</samling>\
    <rm>2022/23</rm>\
    <filnamn>anforande-202223.csvt.zip</filnamn>\
    <storlek>9428358</storlek>\
    <format>csvt</format>\
    <filformat>zip</filformat>\
    <uppdaterad>2024-09-26 02:00:27</uppdaterad>\
    <url>/dataset/anforande/anforande-202223.csvt.zip</url>\
    <description>Anföranden som ledamöter hållit i kammaren.</description>\
    <beskrivning/>\
    <upplysning/>\
    </dataset>\
    <dataset><namn>dokument</namn><typ>bet</typ><samling>bet-1971-1979</samling><rm>1971-1979</rm><filnamn>bet-1971-1979.xml.zip</filnamn><storlek>43958334</storlek><format>xml</format><filformat>zip</filformat><uppdaterad>2015-04-27 03:59:11</uppdaterad><url>/dataset/dokument/bet-1971-1979.xml.zip</url><description>Utskottens betänkanden och utlåtanden, inklusive rksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet. I vissa årgångar finns även debatten med i formaten JSON, SQL och XML.</description><beskrivning/><upplysning>Saknade dokument: <br/>1975/76: alla<br/>1976/77: FiU,NU</upplysning></dataset></datasetlista>";
    similar_asserts::assert_eq!(buffer, se_expected);

    let mut reader = quick_xml::NsReader::from_str(&buffer);
    let actual = DatasetLista::deserialize_xml(&mut reader)?;
    similar_asserts::assert_eq!(actual, value);
    Ok(())
}
