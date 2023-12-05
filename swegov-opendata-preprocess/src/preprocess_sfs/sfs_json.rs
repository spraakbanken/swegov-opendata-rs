use std::{fs, io::Write};

use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use minidom::{
    quick_xml::{events::Event, Reader, Writer},
    Element,
};
use minidom_extension::minidom;
use once_cell::sync::Lazy;
use regex::Regex;
use swegov_opendata::{DokumentStatus, DokumentStatusPage};

use super::shared::attrib_equals;
use crate::shared::clean_element;

use super::SfsPreprocessError;

mod sfs_div_dok;
mod sfs_standard;

pub fn preprocess_json(source: &str) -> error_stack::Result<Vec<u8>, SfsPreprocessError> {
    let DokumentStatusPage {
        dokumentstatus:
            DokumentStatus {
                dokument,
                dokbilaga: _,
                dokuppgift,
            },
    } = serde_json::from_str(source).change_context(SfsPreprocessError::Json)?;

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

    // text attributes
    for (name, value) in [
        ("hangar_id", &dokument.hangar_id),
        ("rm", &dokument.rm),
        ("beteckning", &dokument.beteckning),
        ("dokumentnamn", &dokument.dokumentnamn),
        ("typ", &dokument.typ),
        ("subtyp", &dokument.subtyp),
        ("organ", &dokument.organ),
        ("nummer", &dokument.nummer),
        ("slutnummer", &dokument.slutnummer),
        ("titel", &dokument.titel),
        ("status", &dokument.status),
    ] {
        textelem.set_attr(name, value.replace("\r\n", " "));
    }
    for (name, value) in [
        ("subtitel", &dokument.subtitel),
        ("tempbeteckning", &dokument.tempbeteckning),
    ] {
        if !value.is_empty() {
            textelem.set_attr(name, value.replace("\r\n", " "));
        }
    }
    for (name, value) in [
        ("publicerad", &dokument.publicerad),
        ("systemdatum", &dokument.systemdatum),
        ("datum", &dokument.datum),
    ] {
        textelem.set_attr(name, value.to_string());
    }
    if let Some(upphavd_str) = dokuppgift.get_by_kod("upphavd") {
        let (upphavd_at, _remaining) =
            NaiveDate::parse_and_remainder(&upphavd_str, "%Y-%m-%d").unwrap();
        textelem.set_attr("upphavd", upphavd_at.to_string());
    }
    if let Some(upphnr) = dokuppgift.get_by_kod("upphnr") {
        textelem.set_attr("upphnr", upphnr);
    }

    if dokument.html.is_empty() {
        return Err(Report::new(SfsPreprocessError::HtmlFieldIsEmpty));
    } else {
        process_html(&dokument.html, &mut textelem)?;
    }
    if !(textelem.has_child("p", "") || textelem.has_child("page", "")) {
        tracing::error!(docelem = ?docelem, textelem = ?textelem, "no p or page");
        todo!("handle no p/page");
    }
    let textelem = clean_element(&textelem); //.expect("Cleaning should work");
    if !(textelem.has_child("p", "") || textelem.has_child("page", "")) {
        tracing::warn!(docelem = ?docelem, textelem = ?textelem, "document contains no text");
    }

    // Add text as child to dokument
    docelem.append_child(textelem);

    // Serialize dokument
    let mut result = Vec::new();
    let mut writer = Writer::new_with_indent(&mut result, b' ', 2);
    docelem
        .to_writer(&mut writer)
        .change_context(SfsPreprocessError::Write)?;
    Ok(result)
}

fn process_html(
    contents: &str,
    textelem: &mut Element,
) -> error_stack::Result<(), SfsPreprocessError> {
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

    let contents = contents.replace("\"", r#"""#);
    let contents = contents.replace("\r\n", " ");
    let contents = contents.replace('&', "&amp;");

    let contents = NON_TAG.replace_all(&contents, "&lt;${1}&gt;");
    let contents = ASTERIX_RIGHT_ANGLE.replace_all(&contents, "*&gt;");
    let contents = DOUBLE_ANGLES.replace_all(&contents, "« $1 »");
    let contents = DOUBLE_LEFT_ANGLES.replace_all(&contents, "&lt;${1}");
    let contents = LEFT_ANGLE_NON_TAG.replace_all(&contents, "&lt;${1}");
    let contents = UNEXPECTED_BANG.replace_all(&contents, "&lt;!${1}");
    // let contents = contents.replace("<<", "«");
    // let contents = contents.replace(">>", "»");
    let contents = contents.replace("<t>", " ");
    // let contents = contents.replace("-<", "-&lt;");

    let _ = fs::File::create("assets/contents.html")
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
    let mut reader = Reader::from_str(&contents);
    loop {
        match reader.read_event() {
            Err(e) => {
                return Err(Report::new(SfsPreprocessError::XmlParsingError {
                    pos: reader.buffer_position(),
                    err: e,
                }))
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
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process_html() -> error_stack::Result<(), SfsPreprocessError> {
        let src = "<div><p><a name=\"S1\"></a></p><a class=\"paragraf\" name=\"P1\"><b>1 §</b></a>   Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har<br />\r\n   1. inlösts enligt 56 a § arbetsmarknadskungörelsen (1966:368),<br />\r\n   2. avstyckats från jordbruksfastighet genom åtgärder i samband med jordbrukets rationalisering.<h3 name=\"overgang\"><a name=\"overgang\">Övergångsbestämmelser</a></h3>\r\n1985:458<p><a name=\"P11S2\"></a></p>\r\n\r\nDenna förordning träder i kraft den 1 juli 1985.</div>";
        let mut actual_elem = Element::bare("text", "");
        process_html(src, &mut actual_elem)?;
        let actual_elem = clean_element(&actual_elem);
        let mut actual = Vec::new();
        actual_elem
            .write_to(&mut actual)
            .change_context_lazy(|| SfsPreprocessError::Internal(format!("writing actual")))?;
        let expected = r#"<text xmlns=""><p>1 § Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har<br/> 1. inlösts enligt 56 a § arbetsmarknadskungörelsen (1966:368),<br/> 2. avstyckats från jordbruksfastighet genom åtgärder i samband med jordbrukets rationalisering.</p><p>Övergångsbestämmelser</p><p> 1985:458</p><p> Denna förordning träder i kraft den 1 juli 1985.</p></text>"#;
        let actual_str = String::from_utf8_lossy(&actual);
        assert_eq!(actual_str, expected);
        Ok(())
    }
}
