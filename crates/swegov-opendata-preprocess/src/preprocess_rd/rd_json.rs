use std::{collections::BTreeSet, fmt::Display, iter::Peekable};

use itertools::Itertools;
use minidom_extension::minidom::{quick_xml::Writer, Element, Error as MinidomError};
use swegov_opendata::{DataSet, DokumentStatusPageRef, DokumentStatusRef};

use crate::shared::{clean_element, io_ext};

use super::html::{process_html, ProcessHtmlError};

#[tracing::instrument(skip(source, metadata))]
pub fn preprocess_json(source: &str, metadata: &DataSet) -> Result<Vec<u8>, PreprocessJsonError> {
    let source = io_ext::without_bom(source);
    // tracing::trace!("source = {}", source);
    let DokumentStatusPageRef {
        dokumentstatus:
            DokumentStatusRef {
                dokument,
                dokbilaga,
                dokuppgift,
                dokintressent,
                debatt,
                dokforslag,
                dokreferens,
                dokaktivitet,
                dokmotforslag,
                dokutskottsforslag,
                webbmedia,
            },
    } = serde_json::from_str(source)?;

    // Build docelem
    let mut docelem = Element::builder("dokument", "")
        .attr("dok_id", dokument.dok_id)
        .build();

    for (attr, value_opt) in [
        ("dokument_url_text", dokument.dokument_url_text),
        ("dokument_url_html", dokument.dokument_url_html),
        ("dokumentstatus_url_xml", dokument.dokumentstatus_url_xml),
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
        // ("hangar_id", &dokument.hangar_id),
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
    if let Some(upplysning) = &metadata.upplysning {
        todo!("Handle upplysning={:?}", upplysning);
    }
    if let Some(dokintressent) = dokintressent {
        let mut intressenter = BTreeSet::new();
        for dok_int in dokintressent.intressent {
            intressenter.insert(dok_int);
        }
        let mut name_party = vec![];
        let mut name = vec![];
        let mut party = BTreeSet::new();
        let mut intressent_id = vec![];
        let mut roles = BTreeSet::new();
        let mut name_party_intressent_id_role = vec![];

        for dok_int in intressenter {
            name.push(dok_int.namn);
            party.insert(dok_int.partibet.unwrap_or(""));
            name_party.push(format!(
                "{} ({})",
                dok_int.namn,
                dok_int.partibet.unwrap_or("")
            ));
            intressent_id.push(dok_int.intressent_id);
            roles.insert(dok_int.roll);
            name_party_intressent_id_role.push(format!(
                "{} ({}), {}, {}",
                dok_int.namn,
                dok_int.partibet.unwrap_or(""),
                dok_int.intressent_id,
                dok_int.roll
            ));
        }

        textelem.set_attr(
            "intressent_namn_parti",
            format_multi_value(name_party.iter().peekable()),
        );
        textelem.set_attr(
            "intressent_namn",
            format_multi_value(name.iter().peekable()),
        );
        textelem.set_attr(
            "intressent_parti",
            format_multi_value(party.iter().peekable()),
        );
        textelem.set_attr(
            "intressent_id",
            format_multi_value(intressent_id.iter().peekable()),
        );
        textelem.set_attr(
            "intressent_namn_parti_id_roll",
            format_multi_value(name_party_intressent_id_role.iter().peekable()),
        );
    }
    if let Some(html) = dokument.html() {
        process_html(html, &mut textelem)?;
    } else {
        tracing::warn!("The html field is empty");
    }
    let textelem = clean_element(&textelem); //.expect("Cleaning should work");
    if !(textelem.has_child("p", "") || textelem.has_child("page", "")) {
        tracing::warn!(docelem = ?docelem, textelem = ?textelem, "document contains no text");
    }
    // Add textelem as child to docelem
    docelem.append_child(textelem);
    // dbg!(&dokintressent);

    // Add anforande as separate texts
    if let Some(debatt) = debatt {
        for anforande in debatt.anforande {
            let mut textelem = Element::builder("text", "")
                .attr("datatyp", "anforande")
                .build();
            for (name, value) in [
                ("beteckning", anforande.anf_beteckning),
                ("klockslag", anforande.anf_klockslag),
                ("rm", anforande.anf_rm),
                ("nummer", anforande.anf_nummer),
                ("anf_video_id", anforande.anf_video_id),
                ("klockslag", anforande.anf_klockslag),
                ("typ", anforande.anf_typ),
                ("parti", anforande.parti),
                ("talare", anforande.talare),
                ("video_id", anforande.video_id),
                ("video_url", anforande.video_url),
            ] {
                textelem.set_attr(name, value.trim());
            }
            for (name, value_opt) in [
                ("id", anforande.anf_id),
                ("debatt_id", anforande.debatt_id),
                ("debatt_typ", anforande.debatt_typ),
                ("debatt_titel", anforande.debatt_titel),
                ("dok_beteckning", anforande.dok_beteckning),
                ("dok_id", anforande.dok_id),
                ("dok_intressent", anforande.dok_intressent),
                ("intressent_id", anforande.intressent_id),
                ("kon", anforande.kon),
                ("parent_id", anforande.parent_id),
                ("voteringspunkt", anforande.voteringspunkt),
            ] {
                textelem.set_attr(name, value_opt.map(|s| s.trim()).unwrap_or(""));
            }
            textelem.set_attr("datum", anforande.datumtid.to_string());
            for (name, value_opt) in [("systemdatum", &anforande.systemdatum)] {
                if let Some(value) = value_opt {
                    textelem.set_attr(name, value.to_string());
                }
            }
            for (name, value) in [
                ("hangar_id", anforande.anf_hangar_id),
                ("sekunder", anforande.anf_sekunder),
            ] {
                textelem.set_attr(name, value.to_string());
            }
            if let Some(text) = &anforande.anf_text {
                process_html(text, &mut textelem)?;
            } else {
                tracing::warn!(anforande.anf_id, "The field 'anf_text' is empty");
            }
            let textelem = clean_element(&textelem);
            // Add textelem as child to docelem
            docelem.append_child(textelem);
        }
    }

    // Add forslag as separate texts
    if let Some(dokforslag) = dokforslag {
        for forslag in dokforslag.forslag {
            let mut textelem = Element::builder("text", "")
                .attr("datatyp", "forslag")
                .build();
            for (name, value) in [("nummer", forslag.nummer)] {
                textelem.set_attr(name, value.to_string());
            }
            for (name, value_opt) in [
                ("hangar_id", forslag.hangar_id),
                ("beteckning", forslag.beteckning),
                ("utskottet", forslag.utskottet),
                ("kammaren", forslag.kammaren),
                ("behandlas_i", forslag.behandlas_i),
                ("behandlas_i_punkt", forslag.behandlas_i_punkt),
                ("kammarbeslutstyp", forslag.kammarbeslutstyp),
                ("intressent", forslag.intressent),
                ("avsnitt", forslag.avsnitt),
                ("grundforfattning", forslag.grundforfattning),
                ("andringsforfattning", forslag.andringsforfattning),
            ] {
                textelem.set_attr(name, value_opt.map(|s| s.trim()).unwrap_or(""));
            }
            process_html(forslag.lydelse, &mut textelem)?;
            if let Some(text) = forslag.lydelse2 {
                process_html(&text, &mut textelem)?;
            }
            let textelem = clean_element(&textelem);
            // Add textelem as child to docelem
            docelem.append_child(textelem);
        }
    }

    // Add uppgift as separate texts
    if let Some(dokuppgift) = dokuppgift {
        for uppgift in dokuppgift.uppgift {
            let mut textelem = Element::builder("text", "")
                .attr("datatyp", "uppgift")
                .build();
            textelem.set_attr("kod", uppgift.kod.trim());
            textelem.set_attr("namn", uppgift.namn.trim());
            textelem.set_attr("dok_id", uppgift.dok_id.map(|s| s.trim()).unwrap_or(""));
            if let Some(systemdatum) = uppgift.systemdatum {
                textelem.set_attr("systemdatum", systemdatum.to_string());
            }
            if let Some(text) = uppgift.text {
                process_html(&text, &mut textelem)?;
            }
            let textelem = clean_element(&textelem);
            // Add textelem as child to docelem
            docelem.append_child(textelem);
        }
    }

    // Add utskottsforslag as separate texts
    if let Some(dokutskottsforslag) = dokutskottsforslag {
        for utskottsforslag in dokutskottsforslag.utskottsforslag {
            let mut textelem = Element::builder("text", "")
                .attr("datatyp", "utskottforslag")
                .build();
            for (name, value) in [
                ("punkt", utskottsforslag.punkt),
                ("motforslag_nummer", utskottsforslag.motforslag_nummer),
            ] {
                textelem.set_attr(name, value.to_string());
            }
            for (name, value) in [
                ("rm", utskottsforslag.rm),
                ("bet", utskottsforslag.bet),
                ("rubrik", utskottsforslag.rubrik),
            ] {
                textelem.set_attr(name, value.trim());
            }
            for (name, value_opt) in [
                ("beteckning", utskottsforslag.beteckning),
                ("beslut", utskottsforslag.beslut),
                ("beslutstyp", utskottsforslag.beslutstyp),
                ("beslutsregelkvot", utskottsforslag.beslutsregelkvot),
                ("beslutsregelparagraf", utskottsforslag.beslutsregelparagraf),
                ("vinnare", utskottsforslag.vinnare),
                ("voteringskrav", utskottsforslag.voteringskrav),
                ("votering_url_xml", utskottsforslag.votering_url_xml),
                (
                    "votering_ledamot_url_xml",
                    utskottsforslag.votering_ledamot_url_xml,
                ),
                ("punkttyp", utskottsforslag.punkttyp),
                // ("dok_intressent", utskottsforslag.dok_intressent),
                // ("intressent_id", utskottsforslag.intressent_id),
                // ("kon", utskottsforslag.kon),
                // ("parent_id", utskottsforslag.parent_id),
                // ("voteringspunkt", utskottsforslag.voteringspunkt),
            ] {
                textelem.set_attr(name, value_opt.map(|s| s.trim()).unwrap_or(""));
            }
            if let Some(motforslag_partier) = utskottsforslag.motforslag_partier {
                textelem.set_attr(
                    "motforslag_partier",
                    split_and_format_parties(motforslag_partier.as_ref()),
                );
            }
            if let Some(text) = utskottsforslag.forslag {
                process_html(&text, &mut textelem)?;
            }
            if let Some(text) = utskottsforslag.forslag_del2 {
                process_html(&text, &mut textelem)?;
            }
            if let Some(value) = &utskottsforslag.votering_sammanfattning_html {
                process_json_value(value, &mut textelem)?;
            }
            let textelem = clean_element(&textelem);
            // Add textelem as child to docelem
            docelem.append_child(textelem);
        }
    }
    // Add forslag as separate texts
    if let Some(dokmotforslag) = dokmotforslag {
        for motforslag in dokmotforslag.motforslag {
            let mut textelem = Element::builder("text", "")
                .attr("datatyp", "motforslag")
                .build();
            for (name, value) in [
                ("nummer", motforslag.nummer),
                ("utskottsforslag_punkt", motforslag.utskottsforslag_punkt),
            ] {
                textelem.set_attr(name, value.to_string());
            }
            for (name, value) in [("typ", motforslag.typ)] {
                textelem.set_attr(name, value.trim());
            }
            for (name, value_opt) in [("id", motforslag.id)] {
                textelem.set_attr(name, value_opt.map(|s| s.trim()).unwrap_or(""));
            }
            textelem.set_attr(
                "partier",
                split_and_format_parties(motforslag.partier.as_ref()),
            );
            if let Some(text) = motforslag.rubrik {
                process_html(&text, &mut textelem)?;
            }
            if let Some(text) = motforslag.forslag {
                process_html(text, &mut textelem)?;
            }
            let textelem = clean_element(&textelem);
            // Add textelem as child to docelem
            docelem.append_child(textelem);
        }
    }

    // Serialize dokument
    let mut result = Vec::new();
    let mut writer = Writer::new_with_indent(&mut result, b' ', 2);
    docelem.to_writer(&mut writer).map_err(|error| {
        tracing::error!("Error writing xml: {:?}", error);
        PreprocessJsonError::XmlWrite(error)
    })?;
    Ok(result)
}

#[inline]
fn format_multi_value<I>(mut iter: Peekable<I>) -> String
where
    I: Iterator,
    I::Item: Display,
{
    if iter.peek().is_none() {
        return String::from("|");
    }

    format!("|{}|", iter.join("|"))
}

fn split_and_format_parties(text: &str) -> String {
    let mut parties = BTreeSet::new();
    let parts = text.split('"');
    for part in parts {
        match part {
            "" | "," => (),
            x => {
                parties.insert(x.to_uppercase());
            }
        }
    }

    format_multi_value(parties.iter().peekable())
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum PreprocessJsonError {
    #[error("Error reading JSON")]
    JsonError(#[from] serde_json::Error),
    #[error("Failed write XML")]
    XmlWrite(#[source] MinidomError),
    #[error("Document contains no html")]
    #[diagnostic(severity(Warning))]
    HtmlFieldIsEmpty,
    #[error("Error processing HTML")]
    HtmlError(#[from] ProcessHtmlError),
}

fn process_json_value(
    value: &serde_json::Value,
    textelem: &mut Element,
) -> Result<(), ProcessHtmlError> {
    use serde_json::Value;
    // if let serde_json::Value::String(html) = value {
    //     process_html(html, textelem, &Cow::Borrowed(""));
    //     return;
    // }
    // todo!("handle {:?}", value)
    match value {
        Value::String(html) => process_html(html, textelem)?,
        Value::Object(obj) => {
            if let Some(table) = obj.get("table") {
                if let Some(table) = table.as_array() {
                    dbg!(table);
                    for a in table {
                        match a {
                            Value::Object(to) => match &to["tr"] {
                                Value::Array(rows) => {
                                    for row in rows {
                                        match row {
                                            Value::Object(r) => {
                                                if let Some(td) = r.get("td") {
                                                    match td {
                                                        Value::Object(td_obj) => {
                                                            dbg!(td_obj);
                                                            match &td_obj["h4"] {
                                                                Value::String(text) => {
                                                                    textelem
                                                                        .append_child(elem_p(text));
                                                                }
                                                                Value::Null => (),
                                                                x => todo!("handle {:?}", x),
                                                            }
                                                            if let Some(td_obj_p) = td_obj.get("p")
                                                            {
                                                                match td_obj_p {
                                                                    Value::String(text) => {
                                                                        textelem.append_child(
                                                                            elem_p(text),
                                                                        );
                                                                    }
                                                                    x => todo!("handle {:?}", x),
                                                                }
                                                            }
                                                        }
                                                        Value::Array(td_arr) => {
                                                            for td_v in td_arr {
                                                                match td_v {
                                                                    Value::String(text) => {
                                                                        textelem.append_child(
                                                                            elem_p(text),
                                                                        );
                                                                    }
                                                                    x => todo!("handle {:?}", x),
                                                                }
                                                            }
                                                        }
                                                        x => todo!("handle {:?}", x),
                                                    }
                                                }
                                                if let Some(th) = &r.get("th") {
                                                    match th {
                                                        Value::Array(th_arr) => {
                                                            for th_v in th_arr {
                                                                match th_v {
                                                                    Value::String(text) => {
                                                                        textelem.append_child(
                                                                            elem_p(text),
                                                                        );
                                                                    }
                                                                    x => todo!("handle {:?}", x),
                                                                }
                                                            }
                                                        }
                                                        x => todo!("handle {:?}", x),
                                                    }
                                                }
                                            }
                                            x => todo!("handle {:?}", x),
                                        }
                                    }
                                }
                                x => todo!("handle {:?}", x),
                            },
                            x => todo!("handle {:?}", x),
                        }
                    }
                } else if let Some(table) = table.as_object() {
                } else {
                    todo!("handle table={:?}", table);
                }
            }
        }
        x => todo!("handle {:?}", x),
    }
    Ok(())
}

fn elem_p(text: &str) -> Element {
    let mut p = Element::bare("p", "");
    p.append_text_node(text);
    p
}
