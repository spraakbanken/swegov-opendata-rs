use minidom::{
    quick_xml::{events::Event, Reader},
    Element,
};
use minidom_extension::{attrib_query::attrib_equals, elem_is_empty, minidom};

use super::SfsPreprocessError;

pub fn process_html_sfs_div_dok(
    reader: &mut Reader<&[u8]>,
    textelem: &mut Element,
) -> Result<(), SfsPreprocessError> {
    let mut state = ParseHtmlState::Start;
    let mut page_nr = 1;
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
                b"style" => state = ParseHtmlState::Skip { tag: b"style" },
                b"div" if attrib_equals(&e, b"class", b"pageWrap") => {
                    state = ParseHtmlState::ExtractPage;
                }
                b"div" if attrib_equals(&e, b"class", b"sida") => {
                    let mut page = extract_page(reader)?;
                    page.set_attr("id", page_nr);
                    page_nr += 1;
                    textelem.append_child(page);
                    state = ParseHtmlState::Start;
                }
                _ => todo!("handle {:?} state={:?}", e, state),
            },
            Ok(Event::Text(_content)) => {
                if let ParseHtmlState::Skip { tag: _ } = state {
                    continue;
                }
            }
            Ok(Event::End(e)) => {
                if let ParseHtmlState::Skip { tag } = state {
                    if e.name().as_ref() == tag {
                        state = ParseHtmlState::Start;
                    }
                    continue;
                }
            }
            Ok(Event::Comment(_)) => continue,
            Ok(e) => todo!("handle {:?} state={:?}", e, state),
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum ParseHtmlState {
    Start,
    ExtractPage,
    // ExtractMetadataFoundKey { key: Cow<'a, str> },
    // Dokument,
    // Paragraph,
    Skip { tag: &'static [u8] },
}

pub fn extract_page(reader: &mut Reader<&[u8]>) -> Result<Element, SfsPreprocessError> {
    reader.check_end_names(false);
    let mut elem = Element::bare("page", "");
    let mut curr: Option<Element> = Some(Element::bare("p", ""));
    let mut state = ExtractPageState::InSida;
    loop {
        match reader.read_event() {
            Err(err) => match state {
                ExtractPageState::ParsingBadString => {
                    dbg!("handling err", &err, &state);
                    state = ExtractPageState::Start;
                    continue;
                }
                _ => todo!(
                    "handle err {:?} state={:?}, pos={}",
                    err,
                    state,
                    reader.buffer_position()
                ),
            },
            Ok(Event::Eof) => {
                break;
            }
            Ok(Event::Start(e)) => {
                if let ExtractPageState::Skip { tag: _ } = &state {
                    continue;
                }
                match e.name().as_ref() {
                    b"div" if attrib_equals(&e, b"class", b"block") => {
                        state = ExtractPageState::InBlock;
                        continue;
                    }
                    b"p" => {
                        if let Some(p) = curr.take() {
                            if !elem_is_empty(&p) {
                                elem.append_child(p);
                            }
                        }
                        curr = Some(Element::bare("p", ""));
                        state = ExtractPageState::InParagraph;
                    }
                    b"i" => {
                        if let Some(p) = &mut curr {
                            p.append_text_node("<i>");
                        }
                    }
                    b"span" => continue,
                    b"table" => {
                        if let Some(p) = curr.take() {
                            if !elem_is_empty(&p) {
                                elem.append_child(p);
                            }
                        }
                        let table = Element::builder("table", "")
                            .attr("class", "removed")
                            .build();
                        elem.append_child(table);
                        curr = Some(Element::bare("p", ""));
                        state = ExtractPageState::Skip { tag: b"table" };
                    }
                    should_be_text => {
                        tracing::warn!(
                            event = ?e,
                            ?state,
                            pos = reader.buffer_position(),
                            "handling unknown Start",
                        );
                        let mut raw_text = vec![b'<'];
                        raw_text.extend(should_be_text);
                        raw_text.extend(e.attributes_raw());
                        let text = String::from_utf8_lossy(&raw_text).to_string();
                        tracing::warn!(text, ?state, "text from unknown tag");
                        if let Some(curr_elem) = &mut curr {
                            curr_elem.append_text_node(text);
                        } else {
                            todo!("handle curr empty state={:?}", state);
                        }
                        state = ExtractPageState::ParsingBadString;
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                if let ExtractPageState::Skip { tag: _ } = state {
                    continue;
                }
                match e.name().as_ref() {
                    b"br" => {
                        if let Some(curr_elem) = &mut curr {
                            curr_elem.append_child(Element::bare("br", ""));
                        }
                    }
                    b"p" => continue,
                    _ => todo!(
                        "handle Empty: {:?} state={:?} pos={:?}",
                        e,
                        state,
                        reader.buffer_position()
                    ),
                }
            }
            Ok(Event::Text(content)) => {
                if let ExtractPageState::Skip { tag: _ } = state {
                    continue;
                }
                let text = match content.unescape() {
                    Ok(text) => text.to_string(),
                    Err(err) => {
                        tracing::error!(error = ?err, "making text of error");
                        let err_content = content.into_inner();
                        match String::from_utf8(err_content.to_vec()) {
                            Ok(text) => text,
                            Err(err) => {
                                return Err(SfsPreprocessError::XmlFromUtf8Error {
                                    pos: reader.buffer_position(),
                                    err,
                                });
                            }
                        }
                    }
                };

                if let Some(curr_elem) = &mut curr {
                    curr_elem.append_text_node(text);
                } else {
                    todo!("handle curr empty state={:?}", state);
                }
            }
            Ok(Event::End(e)) => {
                if let ExtractPageState::Skip { tag } = state {
                    if e.name().as_ref() == tag {
                        state = ExtractPageState::InBlock;
                    }
                    continue;
                }
                match e.name().as_ref() {
                    b"div" => {
                        state = match state {
                            ExtractPageState::InBlock => ExtractPageState::InSida,
                            ExtractPageState::InSida => ExtractPageState::InPageWrap,
                            ExtractPageState::InPageWrap => {
                                break;
                            }
                            state => state,
                        };
                    }
                    b"p" => {
                        state = ExtractPageState::InBlock;
                        continue;
                    }
                    b"span" => continue,
                    _ => todo!("handle End {:?} state={:?}", e, state),
                }
            }
            Ok(Event::Comment(_e)) => {
                continue;
            }
            Ok(e) => todo!("handle {:?} state={:?}", e, state),
        }
    }
    if let Some(p) = curr.take() {
        if !elem_is_empty(&p) {
            elem.append_child(p);
        }
    }
    reader.check_end_names(true);

    Ok(elem)
}

#[derive(Debug, Clone, PartialEq)]
enum ExtractPageState {
    Start,
    // ExtractPage,
    // ExtractMetadataFoundKey { key: Cow<'a, str> },
    // Dokument,
    // Paragraph,
    InBlock,
    InSida,
    InPageWrap,
    InParagraph,
    Skip { tag: &'static [u8] },
    ParsingBadString,
}
