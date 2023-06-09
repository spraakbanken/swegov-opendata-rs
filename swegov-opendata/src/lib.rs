pub mod date_formats;
pub mod try_parse;

use crate::try_parse::{deserialize_tryparse_from_string, TryParse};
use deserx::DeXml;
use quick_xml::events::BytesEnd;
use serde_with::serde_as;
use serde_with::DisplayFromStr;

use serde_aux::field_attributes::deserialize_number_from_string;
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentStatusPage {
    pub dokumentstatus: DokumentStatus,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename = "dokumentstatus")]
pub struct DokumentStatus {
    pub dokument: Dokument,
    pub dokuppgift: DokUppgift,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokbilaga: Option<DokBilaga>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dokument {
    pub dok_id: String,
    hangar_id: String,
    pub rm: String,
    beteckning: String,
    pub typ: String,
    subtyp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    doktyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    typrubrik: Option<String>,
    dokumentnamn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    debattnamn: Option<String>,
    tempbeteckning: String,
    organ: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mottagare: Option<String>,
    // #[serde(deserialize_with = "deserialize_tryparse_from_string")]
    // nummer: TryParse<u64>,
    nummer: String,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    slutnummer: String,
    #[serde(with = "date_formats::swe_date_format")]
    datum: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format")]
    publicerad: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format")]
    systemdatum: NaiveDateTime,
    titel: String,
    subtitel: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    htmlformat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relaterat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dokument_url_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dokument_url_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dokumentstatus_url_xml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    utskottsforslag_url_xml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
}

impl Dokument {
    pub fn dok_id(&self) -> &str {
        &self.dok_id
    }
    pub fn rm(&self) -> &str {
        &self.rm
    }
    pub fn datum(&self) -> NaiveDate {
        self.datum.date()
    }
    pub fn titel(&self) -> &str {
        &self.titel
    }
    pub fn organ(&self) -> &str {
        &self.organ
    }
    pub fn html(&self) -> Option<&str> {
        self.html.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokUppgift {
    uppgift: Vec<Uppgift>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokBilaga {
    bilaga: Vec<Bilaga>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uppgift {
    #[serde(skip_serializing_if = "Option::is_none")]
    dok_id: Option<String>,
    kod: String,
    namn: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "date_formats::option_swe_date_format",
        default
    )]
    systemdatum: Option<NaiveDateTime>,
    text: Option<String>,
}

use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentListaPage {
    dokumentlista: DokumentLista,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, rename = "dokumentlista")]
pub struct DokumentLista {
    #[serde(rename = "@dDt")]
    d_dt: String,
    #[serde(rename = "@dPre")]
    d_pre: String,
    #[serde(rename = "@dR")]
    d_r: String,
    #[serde(rename = "@dSol")]
    d_sol: String,
    // #[serde(rename = "@datum", with = "date_formats::swe_date_format")]
    // datum: NaiveDateTime,
    #[serde(rename = "@ms")]
    ms: String,
    #[serde(rename = "@nasta_sida")]
    pub nasta_sida: String,
    #[serde(rename = "@q")]
    pub q: String,
    // #[serde(rename = "@sida")]
    // #[serde_as(as = "DisplayFromStr")]
    // sida: u64,
    // #[serde(rename = "@sidor")]
    // #[serde_as(as = "DisplayFromStr")]
    // sidor: u64,
    // #[serde(rename = "@traff_fran")]
    // #[serde_as(as = "DisplayFromStr")]
    // traff_fran: u64,
    // #[serde(rename = "@traff_till")]
    // #[serde_as(as = "DisplayFromStr")]
    // traff_till: u64,
    // #[serde(rename = "@traffar")]
    // #[serde_as(as = "DisplayFromStr")]
    // traffar: u64,
    // #[serde(rename = "@varning")]
    // varning: Option<String>,
    // #[serde(rename = "@version")]
    // version: String,
    pub dokument: Vec<DokumentListaDokument>,
    // facettlista: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(deny_unknown_fields)]
pub struct DokumentListaDokument {
    // ardometyp: String,
    // audio: String,
    // avdelning: String,
    // beredningsdag: String,
    // beslutad: String,
    // beslutsdag: String,
    // beteckning: String,
    // database: String,
    // datum: NaiveDate,
    // debatt: Option<String>,
    // debattdag: String,
    // debattgrupp: String,
    // debattnamn: String,
    // debattsekunder: String,
    pub dok_id: String,
    // dokintressent: Option<String>,
    // doktyp: String,
    // dokument_url_html: String,
    // dokument_url_text: String,
    // dokumentformat: String,
    // dokumentnamn: String,
    // domain: String,
    // #[serde(deserialize_with = "deserialize_null_default")]
    // filbilaga: FilBilaga,
    // id: String,
    // inlamnad: String,
    // justeringsdag: String,
    // kall_id: String,
    // kalla: String,
    // klockslag: String,
    // lang: String,
    // motionstid: String,
    // notis: String,
    // notisrubrik: String,
    // nummer: TryParse<u64>,
    // organ: String,
    // plats: String,
    // // #[serde(with = "date_formats::option_swe_date_format")]
    // /// TODO this field can contain date (2018-03-07) and datetime (2016-02-11 15:28:15)
    // publicerad: String,
    // rddata: Option<String>,
    // rdrest: Option<String>,
    // relaterat_id: String,
    // relurl: String,
    // reservationer: String,
    // rm: String,
    // score: String,
    // slutdatum: String,
    // sokdata: SokData,
    // status: String,
    // struktur: String,
    // subtyp: String,
    // summary: String,
    // #[serde(with = "date_formats::swe_date_format")]
    // systemdatum: NaiveDateTime,
    // tempbeteckning: String,
    // tilldelat: String,
    // titel: String,
    // #[serde_as(as = "DisplayFromStr")]
    // traff: u64,
    // typ: String,
    // undertitel: String,
    // url: String,
    // video: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct FilBilaga {
    #[serde(skip_serializing_if = "Option::is_none")]
    fil: Option<Fil>,
}

impl Default for FilBilaga {
    fn default() -> Self {
        Self { fil: None }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fil {
    typ: String,
    namn: String,
    storlek: u64,
    url: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SokData {
    brodsmula: String,
    parti_epost: String,
    parti_kod: String,
    parti_logotyp_img_alt: String,
    parti_logotyp_img_id: String,
    parti_logotyp_img_url: String,
    parti_mandat: String,
    parti_namn: String,
    parti_telefon: String,
    parti_website_namn: String,
    parti_website_url: String,
    soktyp: String,
    statusrad: NaiveDate,
    titel: String,
    undertitel: String,
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

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

impl DeXml for DokumentLista {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "dokumentlista")
    }
    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &quick_xml::events::BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        // #[serde(rename = "@dDt")]
        let d_dt: String = DeXml::deserialize_xml_from_attribute(start, "dDt")?;
        let d_pre: String = DeXml::deserialize_xml_from_attribute(start, "dPre")?;
        let d_r: String = DeXml::deserialize_xml_from_attribute(start, "dR")?;
        let d_sol: String = DeXml::deserialize_xml_from_attribute(start, "dSol")?;

        // #[serde(rename = "@datum", with = "date_formats::swe_date_format")]
        // datum: NaiveDateTime,
        let ms: String = DeXml::deserialize_xml_from_attribute(start, "ms")?;
        let nasta_sida: String = DeXml::deserialize_xml_from_attribute(start, "nasta_sida")?;
        // #[serde(rename = "@q")]
        let q: String = DeXml::deserialize_xml_from_attribute(start, "q")?;
        // #[serde(rename = "@sida")]
        // #[serde_as(as = "DisplayFromStr")]
        // sida: u64,
        // #[serde(rename = "@sidor")]
        // #[serde_as(as = "DisplayFromStr")]
        // sidor: u64,
        // #[serde(rename = "@traff_fran")]
        // #[serde_as(as = "DisplayFromStr")]
        // traff_fran: u64,
        // #[serde(rename = "@traff_till")]
        // #[serde_as(as = "DisplayFromStr")]
        // traff_till: u64,
        // #[serde(rename = "@traffar")]
        // #[serde_as(as = "DisplayFromStr")]
        // traffar: u64,
        // #[serde(rename = "@varning")]
        // varning: Option<String>,
        // #[serde(rename = "@version")]
        // version: String,
        let dokument: Vec<DokumentListaDokument> = DeXml::deserialize_xml_from_body(reader, start)?;
        // facettlista: Option<String>,
        Ok(Self {
            d_dt,
            d_pre,
            d_r,
            d_sol,
            ms,
            nasta_sida,
            q,
            dokument,
        })
    }
    fn deserialize_xml_from_body_with_end<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &quick_xml::events::BytesStart,
        expected_end: quick_xml::events::BytesEnd,
    ) -> Result<Self, quick_xml::Error> {
        // #[serde(rename = "@dDt")]
        let d_dt: String = DeXml::deserialize_xml_from_attribute(start, "dDt")?;
        let d_pre: String = DeXml::deserialize_xml_from_attribute(start, "dPre")?;
        let d_r: String = DeXml::deserialize_xml_from_attribute(start, "dR")?;
        let d_sol: String = DeXml::deserialize_xml_from_attribute(start, "dSol")?;

        // #[serde(rename = "@datum", with = "date_formats::swe_date_format")]
        // datum: NaiveDateTime,
        let ms: String = DeXml::deserialize_xml_from_attribute(start, "ms")?;
        let nasta_sida: String = DeXml::deserialize_xml_from_attribute(start, "nasta_sida")?;
        // #[serde(rename = "@q")]
        let q: String = DeXml::deserialize_xml_from_attribute(start, "q")?;
        // #[serde(rename = "@sida")]
        // #[serde_as(as = "DisplayFromStr")]
        // sida: u64,
        // #[serde(rename = "@sidor")]
        // #[serde_as(as = "DisplayFromStr")]
        // sidor: u64,
        // #[serde(rename = "@traff_fran")]
        // #[serde_as(as = "DisplayFromStr")]
        // traff_fran: u64,
        // #[serde(rename = "@traff_till")]
        // #[serde_as(as = "DisplayFromStr")]
        // traff_till: u64,
        // #[serde(rename = "@traffar")]
        // #[serde_as(as = "DisplayFromStr")]
        // traffar: u64,
        // #[serde(rename = "@varning")]
        // varning: Option<String>,
        // #[serde(rename = "@version")]
        // version: String,
        let dokument: Vec<DokumentListaDokument> =
            deser_xml_from_flat_vec_with_tag(reader, "dokument", &expected_end)?;
        // facettlista: Option<String>,
        Ok(Self {
            d_dt,
            d_pre,
            d_r,
            d_sol,
            ms,
            nasta_sida,
            q,
            dokument,
        })
    }
}

impl DeXml for DokumentListaDokument {
    fn deserialize_xml<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
    ) -> Result<Self, quick_xml::Error> {
        Self::deserialize_xml_from_tag(reader, "dokument")
    }

    fn deserialize_xml_from_body<R: std::io::BufRead>(
        reader: &mut quick_xml::NsReader<R>,
        start: &quick_xml::events::BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let traff: String = DeXml::deserialize_xml_from_tag(reader, "traff")?;
        let domain: String = DeXml::deserialize_xml_from_tag(reader, "domain")?;
        let database: String = DeXml::deserialize_xml_from_tag(reader, "database")?;
        let datum: String = DeXml::deserialize_xml_from_tag(reader, "datum")?;
        let id: String = DeXml::deserialize_xml_from_tag(reader, "id")?;
        let rdrest: String = DeXml::deserialize_xml_from_tag(reader, "rdrest")?;
        let slutdatum: String = DeXml::deserialize_xml_from_tag(reader, "slutdatum")?;
        let dok_id: String = DeXml::deserialize_xml_from_tag(reader, "dok_id").unwrap();
        Ok(Self { dok_id })
    }
}

pub fn deser_xml_from_flat_vec_with_tag<R: std::io::BufRead, T>(
    reader: &mut quick_xml::NsReader<R>,
    tag: &str,
    expected_end: &BytesEnd,
) -> Result<Vec<T>, quick_xml::Error>
where
    T: DeXml,
{
    use quick_xml::events::Event;
    let mut _vec = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Empty(start) => {}
            Event::Start(start) if start.name().as_ref() == tag.as_bytes() => {
                let _t: T = DeXml::deserialize_xml_from_body(reader, &start)?;
                _vec.push(_t);
            }
            evt => todo!("handle {:?}", evt),
        }
    }
    Ok(_vec)
}
