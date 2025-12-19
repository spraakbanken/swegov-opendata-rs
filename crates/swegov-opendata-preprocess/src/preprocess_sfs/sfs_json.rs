use std::{fs, io::Write};

use chrono::NaiveDate;
use minidom::{
    quick_xml::{events::Event, Reader, Writer},
    Element,
};
use minidom_extension::{attrib_query::attrib_equals, minidom};
use once_cell::sync::Lazy;
use regex::Regex;
use swegov_opendata::{DokumentStatus, DokumentStatusPage};

use crate::shared::{clean_element, is_segreg};

use super::SfsPreprocessError;

mod sfs_div_dok;
mod sfs_standard;

pub fn preprocess_json(source: &str) -> Result<Vec<u8>, SfsPreprocessError> {
    let DokumentStatusPage {
        dokumentstatus:
            DokumentStatus {
                dokument,
                dokuppgift,
                ..
            },
    } = serde_json::from_str(source)?;

    // Build dokument
    let mut docelem = Element::builder("dokument", "")
        .attr("dok_id", &dokument.dok_id)
        .build();
    for (attr, value_opt) in [
        ("dokument_url_text", &dokument.dokument_url_text),
        ("dokument_url_html", &dokument.dokument_url_html),
        ("dokumentstatus_url_xml", &dokument.dokumentstatus_url_xml),
    ] {
        if let Some(value) = value_opt {
            docelem.set_attr(attr, value);
        }
    }

    // Build textelem
    let mut textelem = Element::builder("text", "")
        .attr("datatyp", "huvuddokument")
        .build();
    textelem.set_attr("segreg", is_segreg(source).to_string());
    // text attributes
    for (name, value) in [
        // ("hangar_id", &dokument.hangar_id),
        ("rm", &dokument.rm),
        // ("beteckning", &dokument.beteckning),
        ("dokumentnamn", &dokument.dokumentnamn),
        ("typ", &dokument.typ),
        // ("subtyp", &dokument.subtyp),
        // ("organ", &dokument.organ),
        ("nummer", &dokument.nummer),
        ("slutnummer", &dokument.slutnummer),
        ("title", &dokument.titel),
        // ("status", &dokument.status),
    ] {
        textelem.set_attr(name, value.replace("\r\n", " "));
    }
    for (name, value_opt) in [
        ("hangar_id", &dokument.hangar_id),
        // ("rm", &dokument.rm),
        ("beteckning", &dokument.beteckning),
        // ("dokumentnamn", &dokument.dokumentnamn),
        // ("typ", &dokument.typ),
        ("subtyp", &dokument.subtyp),
        ("organ", &dokument.organ),
        // ("nummer", &dokument.nummer),
        // ("slutnummer", &dokument.slutnummer),
        // ("title", &dokument.titel),
        ("status", &dokument.status),
    ] {
        textelem.set_attr(
            name,
            value_opt
                .as_ref()
                .map(|s| s.replace("\r\n", " "))
                .as_deref()
                .unwrap_or(""),
        );
    }
    for (name, value_opt) in [
        ("subtitle", &dokument.subtitel),
        ("tempbeteckning", &dokument.tempbeteckning),
    ] {
        if let Some(value) = value_opt {
            if !value.is_empty() {
                textelem.set_attr(name, value.replace("\r\n", " "));
            }
        }
    }
    for (name, value_opt) in [
        ("publicerad", &dokument.publicerad),
        // ("systemdatum", &dokument.systemdatum),
        // ("datum", &dokument.datum),
    ] {
        if let Some(value) = value_opt {
            textelem.set_attr(name, value.to_string());
        }
    }
    for (name, value) in [
        // ("publicerad", &dokument.publicerad),
        ("systemdatum", &dokument.systemdatum),
        ("datum", &dokument.datum),
    ] {
        textelem.set_attr(name, value.to_string());
    }
    if let Some(dokuppgift) = &dokuppgift {
        if let Some(upphavd_str) = dokuppgift.get_by_kod("upphavd") {
            let (upphavd_at, _remaining) =
                NaiveDate::parse_and_remainder(upphavd_str, "%Y-%m-%d").unwrap();
            textelem.set_attr("upphavd", upphavd_at.to_string());
        }
        if let Some(upphnr) = dokuppgift.get_by_kod("upphnr") {
            textelem.set_attr("upphnr", upphnr);
        }
    }

    if let Some(html) = dokument.html() {
        process_html(html, &mut textelem)?;
    } else {
        return Err(SfsPreprocessError::HtmlFieldIsEmpty);
    }
    if !(textelem.has_child("p", "") || textelem.has_child("page", "")) {
        tracing::error!(docelem = ?docelem, textelem = ?textelem, "no p or page");
        todo!("handle no p/page");
    }
    dbg!(&textelem);
    let textelem = clean_element(&textelem); //.expect("Cleaning should work");
    if !(textelem.has_child("p", "") || textelem.has_child("page", "")) {
        tracing::warn!(docelem = ?docelem, textelem = ?textelem, "document contains no text");
    }

    // Add text as child to dokument
    docelem.append_child(textelem);

    // Serialize dokument
    let mut result = Vec::new();
    let mut writer = Writer::new_with_indent(&mut result, b' ', 2);
    docelem.to_writer(&mut writer)?;
    Ok(result)
}

fn process_html(contents: &str, textelem: &mut Element) -> Result<(), SfsPreprocessError> {
    static DOUBLE_ANGLES: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"<<([\w\s]+)>>").expect("regex failed"));
    static NON_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"<(gr|t)?>").expect("regex failed"));
    static DOUBLE_LEFT_ANGLES: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"<([\w\s]?<[/\w, \(:]+)").expect("regex failed"));

    static LEFT_ANGLE_NON_TAG: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r#"<([jö:[:^alpha:]--/]|[bijlnrt] |be|ck|er|hl|i[iglt\("]|ln|nc|r[ijlnt]|s[s\d]|ui|[a-z/]?[A-Z»;,\.'-]|if )"#,
        )
        .expect("regex failed")
    });
    // static LEFT_ANGLE_NON_TAG: Lazy<Regex> = Lazy::new(|| {
    //     Regex::new(
    //         r"<([\dö\\: £?=\^’•\*«{_'■\)-]|[btrjil] |be|ck|er|i[gl]|nc|r[lnt]|s[s\d]|[a-z/]?[A-Z»;,\.]|if )",
    //     )
    //     .expect("regex failed")
    // });
    static ASTERIX_RIGHT_ANGLE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\*>").expect("regex failed"));
    static UNEXPECTED_BANG: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"<!([^-])").expect("regex failed"));

    let contents_processed = contents.replace("\\\"", r#"""#);
    let contents_processed = contents_processed.replace("\r\n", " ");
    let contents_processed = contents_processed.replace('&', "&amp;");

    let contents_processed = NON_TAG.replace_all(&contents_processed, "&lt;${1}&gt;");
    let contents_processed = ASTERIX_RIGHT_ANGLE.replace_all(&contents_processed, "*&gt;");
    let contents_processed = DOUBLE_ANGLES.replace_all(&contents_processed, "« $1 »");
    let contents_processed = DOUBLE_LEFT_ANGLES.replace_all(&contents_processed, "&lt;${1}");
    let contents_processed = LEFT_ANGLE_NON_TAG.replace_all(&contents_processed, "&lt;${1}");
    let contents_processed = UNEXPECTED_BANG.replace_all(&contents_processed, "&lt;!${1}");
    // let contents = contents.replace("<<", "«");
    // let contents = contents.replace(">>", "»");
    let contents_processed = contents_processed.replace("<t>", " ");
    // let contents = contents.replace("-<", "-&lt;");

    fs::File::create("assets/contents.html")
        .unwrap()
        .write_all(contents_processed.as_bytes())
        .unwrap();
    let mut reader = Reader::from_str(&contents_processed);
    loop {
        match reader.read_event() {
            Err(e) => {
                return Err(SfsPreprocessError::XmlParsingError {
                    pos: reader.buffer_position(),
                    err: e,
                })
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"div" if attrib_equals(&e, b"class", b"dok") => {
                    sfs_div_dok::process_html_sfs_div_dok(&mut reader, textelem)?;
                }
                _ => sfs_standard::process_html_sfs_standard(&mut reader, textelem)?,
            },
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
