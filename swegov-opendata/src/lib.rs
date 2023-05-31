pub mod date_formats;

use serde_aux::field_attributes::deserialize_number_from_string;
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentStatusPage {
    dokumentstatus: DokumentStatus,
}
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentStatus {
    dokument: SfsDokument,
    dokuppgift: DokUppgift,
    dokumentuppgift: Option<DokUppgift>,
    dokbilaga: Option<DokBilaga>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SfsDokument {
    dok_id: String,
    hangar_id: String,
    rm: String,
    beteckning: String,
    typ: String,
    subtyp: String,
    doktyp: Option<String>,
    typrubrik: Option<String>,
    dokumentnamn: String,
    debattnamn: Option<String>,
    tempbeteckning: String,
    organ: String,
    mottagare: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    nummer: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    slutnummer: u64,
    #[serde(deserialize_with = "date_formats::deserialize_swe_date")]
    datum: NaiveDateTime,
    #[serde(deserialize_with = "date_formats::deserialize_swe_date")]
    publicerad: NaiveDateTime,
    #[serde(deserialize_with = "date_formats::deserialize_swe_date")]
    systemdatum: NaiveDateTime,
    titel: String,
    subtitel: String,
    status: String,
    htmlformat: Option<String>,
    relaterat_id: Option<String>,
    source: Option<String>,
    sourceid: Option<String>,
    dokument_url_text: Option<String>,
    dokument_url_html: Option<String>,
    dokumentstatus_url_xml: Option<String>,
    utskottsforslag_url_xml: Option<String>,
    text: Option<String>,
    html: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DokUppgift {
    uppgift: Vec<Uppgift>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DokBilaga {
    bilaga: Vec<Bilaga>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bilaga {
    dok_id: String,
    fil_url: String,
    filnamn: String,
    filstorlek: String,
    filtyp: String,
    subtitel: String,
    titel: String,
}
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Uppgift {
    dok_id: Option<String>,
    kod: String,
    namn: String,
    #[serde(with = "option_swe_date_format", default)]
    systemdatum: Option<NaiveDateTime>,
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
    #[serde(with = "swe_date_format")]
    publicerad: NaiveDateTime,
    #[serde(deserialize_with = "date_formats::deserialize_swe_date")]
    systemdatum: NaiveDateTime,
}

mod swe_date_format {
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

mod option_swe_date_format {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    // use crate::swe_date_format::deserialize;

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    // pub fn serialize<S>(date: Option<&NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    // where
    //     S: Serializer,
    // {
    //     let s = format!("{}", date.format(FORMAT));
    //     serializer.serialize_str(&s)
    // }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let option_s = Option::<String>::deserialize(deserializer)?;
        match option_s {
            None => Ok(None),
            Some(s) => {
                // match Option<String>::deserialize(deserializer)? {
                //     S
                // let s = String::deserialize(deserializer)?;
                let date =
                    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
        }
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
