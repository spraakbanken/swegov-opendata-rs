use std::borrow::Cow;

use minidom::quick_xml::events::Event;
use minidom::quick_xml::Reader;
use minidom::quick_xml::Writer;
use minidom::Element;
use minidom_extension::{minidom, minidom_collect_texts};

use crate::shared::clean_text;

use super::html::process_html;

/// Extract meta data and html from f.
#[tracing::instrument(skip(xml_string))]
pub fn preprocess_xml(xml_string: &str, filename: Cow<'_, str>) -> Result<Vec<u8>, XmlError> {
    // let tree = Soup::new(xml_string);

    // Create new element and build document
    let mut docelem = Element::builder("dokument", "").build();
    let mut textelem = Element::builder("text", "")
        .attr("datatyp", "huvuddokument")
        .build();

    //     docelem = etree.Element("dokument")
    //     textelem = etree.SubElement(docelem, "text")
    //     textelem.set("datatyp", "huvuddokument")
    // let search_dokument = tree.tag("dokument").find().unwrap();
    // if search_dokument.tag("html").find().is_none() {
    //     eprintln!("    WARNING: No html found in {filename}");
    // }
    let mut in_dokument = false;
    let mut in_html = false;
    let mut found_html = false;
    let mut doc_attr = None;
    let mut text_attr = None;
    let mut reader = Reader::from_str(xml_string);
    loop {
        match reader.read_event() {
            Err(e) => {
                return Err(XmlError::Read {
                    pos: reader.buffer_position(),
                    error: e,
                })
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"html" => {
                    tracing::trace!("found 'html'");
                    in_html = true;
                    found_html = true;
                }
                b"dokument" => {
                    tracing::trace!("found 'dokument'");
                    in_dokument = true;
                }
                b"dok_id"
                | b"dokumentstatus_url_xml"
                | b"dokument_url_text"
                | b"dokument_url_html" => {
                    doc_attr = Some(String::from_utf8(e.name().as_ref().to_vec()).unwrap());

                    tracing::trace!("found doc attr '{:?}'", doc_attr);
                }
                tag => {
                    text_attr = Some(String::from_utf8(tag.to_vec()).unwrap());
                }
            },
            Ok(Event::Text(e)) => {
                if in_html {
                    let html_string = match e.unescape() {
                        Ok(s) => s,
                        Err(err) => panic!("unescape failed: {:?}", err),
                    };
                    process_html(&html_string, &mut textelem);
                    // tracing::trace!("textelem = {:?}", textelem);
                } else if doc_attr.is_some() {
                    let name = doc_attr.take().unwrap();
                    let value = match e.unescape() {
                        Ok(s) => s,
                        Err(err) => {
                            return Err(XmlError::Read {
                                pos: reader.buffer_position(),
                                error: err,
                            })
                        }
                    };
                    let value = value.as_ref().trim();
                    if !value.is_empty() {
                        tracing::trace!("setting attribute at docelem {}='{}'", name, value);
                        docelem.set_attr(name, value);
                    }
                } else if text_attr.is_some() {
                    let name = text_attr.take().unwrap();
                    let value = match e.unescape() {
                        Ok(s) => s,
                        Err(err) => {
                            return Err(XmlError::Read {
                                pos: reader.buffer_position(),
                                error: err,
                            })
                        }
                    };
                    let value = value.as_ref().trim();
                    if !value.is_empty() {
                        tracing::trace!("setting attribute at textelem {}='{}'", name, value);
                        textelem.set_attr(name, value);
                    }
                }
            }
            Ok(Event::End(e)) => {
                if in_html && e.name().as_ref() == b"html" {
                    in_html = false;
                }
                if in_dokument && e.name().as_ref() == b"dokument" {
                    in_dokument = false;
                }
            }
            _ => (),
        }
    }
    if !found_html {
        tracing::warn!("    WARNING: No html found in {filename}");
    }
    dbg!("BEFORE", &textelem);
    let textelem = clean_element(&textelem).unwrap();
    dbg!("AFTER", &textelem);
    docelem.append_child(textelem);
    let mut result = Vec::new();
    let mut writer = Writer::new_with_indent(&mut result, b' ', 2);
    docelem
        .to_writer(&mut writer)
        .map_err(|error| XmlError::Write(error))?;
    Ok(result)
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum XmlError {
    #[error("Error reading xml at position {pos}: {error:?}")]
    Read {
        pos: usize,
        #[source]
        error: minidom::quick_xml::Error,
    },
    #[error("Error writing xml")]
    Write(#[source] minidom::Error),
}

pub fn clean_element(elem: &minidom::Element) -> Option<minidom::Element> {
    dbg!(&elem);
    let mut elem_builder = Element::builder(elem.name(), elem.ns());
    for (name, value) in elem.attrs() {
        elem_builder = elem_builder.attr(name, value);
    }
    if elem.name() == "p" {
        let mut text = minidom_collect_texts(elem);
        text = clean_text(&text);
        if text.is_empty() {
            return None;
        } else {
            elem_builder = elem_builder.append(minidom::Node::Text(text));
        }
    } else {
        let mut curr_node: Option<minidom::Node> = None;
        for node in elem.nodes() {
            let node = clean_node(node);
            if let Some(node) = node {
                if !node_is_empty(&node) {
                    match node {
                        minidom::Node::Element(child_elem) => {
                            if let Some(c_node) = curr_node.take() {
                                elem_builder = elem_builder.append(c_node);
                            }
                            if elem.name() == child_elem.name() && elem.ns() == child_elem.ns()
                                || child_elem.name() == "div"
                            {
                                for child_node in child_elem.nodes() {
                                    elem_builder = elem_builder.append(child_node.clone());
                                }
                            } else {
                                elem_builder =
                                    elem_builder.append(minidom::Node::Element(child_elem));
                            }
                        }
                        minidom::Node::Text(contents) => {
                            if let Some(c_node) = curr_node.take() {
                                match c_node {
                                    minidom::Node::Element(c_elem) => {
                                        elem_builder =
                                            elem_builder.append(minidom::Node::Element(c_elem));
                                        curr_node = Some(minidom::Node::Text(contents));
                                    }
                                    minidom::Node::Text(text) => {
                                        // println!("adding text to current text");
                                        curr_node =
                                            Some(minidom::Node::Text(format!("{text} {contents}")))
                                    }
                                }
                                // if let Some(_c_elem) = c_node.as_element() {
                                //     elem_builder = elem_builder.append(c_node);
                                //     curr_node = Some(minidom::Node::Text(contents));
                                // } else if let Some {}
                                // elem_builder = elem_builder.append(minidom::Node::Text(contents));
                            } else {
                                // println!("puting text to current text");
                                curr_node = Some(minidom::Node::Text(contents));
                            }
                        }
                    }
                }
            }
        }
        if let Some(node) = curr_node.take() {
            elem_builder = elem_builder.append(node);
        }
    }
    let new_elem = elem_builder.build();
    // println!("clean_element: new_elem={:#?}", new_elem);
    Some(new_elem)
}

pub fn clean_node(node: &minidom::Node) -> Option<minidom::Node> {
    // dbg!(&node);
    match node {
        minidom::Node::Text(contents) => {
            let text = contents.clone();
            // clean_text(&mut text);
            Some(minidom::Node::Text(text))
        }
        minidom::Node::Element(elem) => clean_element(elem).map(minidom::Node::Element),
    }
}

pub fn node_is_empty(node: &minidom::Node) -> bool {
    // dbg!(&node);
    match &node {
        minidom::Node::Element(elem) => {
            elem.children().count() == 0
                && elem.texts().filter(|text| !text.is_empty()).count() == 0
        }
        minidom::Node::Text(contents) => contents.trim().is_empty(),
    }
}

#[cfg(test)]
mod tests;
