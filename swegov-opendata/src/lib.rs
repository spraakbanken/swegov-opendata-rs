#[derive(Debug, Clone, serde::Deserialize)]
pub struct DokumentStatusPage {
    dokumentstatus: DokumentStatus,
}
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DokumentStatus {
    dokument: SfsDokument,
    dokuppgift: DokUppgift,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SfsDokument {
    dok_id: String,
    rm: String,
    beteckning: String,
    typ: String,
    subtyp: String,
    organ: String,
    nummer: String,
    slutnummer: String,
    #[serde(with = "my_date_format")]
    datum: NaiveDateTime,
    #[serde(with = "my_date_format")]
    publicerad: NaiveDateTime,
    #[serde(with = "my_date_format")]
    systemdatum: NaiveDateTime,
    titel: String,
    text: String,
    html: String,
    dokumentnamn: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DokUppgift {
    uppgift: Vec<Uppgift>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Uppgift {
    kod: String,
    namn: String,
    text: String,
}

use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentListaPage {
    dokumentlista: DokumentLista,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(deny_unknown_fields)]
pub struct DokumentLista {
    #[serde(rename = "@dDt")]
    d_dt: String,
    #[serde(rename = "@dPre")]
    d_pre: String,
    dokument: Vec<DokumentListaDokument>,
    #[serde(rename = "@nasta_sida")]
    nasta_sida: String,
    #[serde(rename = "@sida")]
    sida: String,
    #[serde(rename = "@q")]
    q: String,

    #[serde(rename = "@sidor")]
    sidor: String,
    #[serde(rename = "@traffar")]
    traffar: String,
    #[serde(rename = "@traff_fran")]
    traff_fran: String,
    #[serde(rename = "@traff_till")]
    traff_till: String,
    #[serde(rename = "@version")]
    version: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DokumentListaDokument {
    id: String,
    dok_id: String,
    traff: String,
    domain: String,
    database: String,
    datum: NaiveDate,
    #[serde(with = "my_date_format")]
    publicerad: NaiveDateTime,
    #[serde(with = "my_date_format")]
    systemdatum: NaiveDateTime,
}

mod my_date_format {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
