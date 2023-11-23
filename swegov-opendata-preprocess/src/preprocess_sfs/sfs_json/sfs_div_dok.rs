use std::borrow::Cow;

use error_stack::{Report, ResultExt};
use minidom::{
    quick_xml::{events::Event, Reader},
    Element,
};
use minidom_extension::{elem_is_empty, minidom};

use super::SfsPreprocessError;
use crate::preprocess_sfs::shared::attrib_equals;

#[tracing::instrument(skip(reader, textelem))]
pub fn process_html_sfs_div_dok(
    reader: &mut Reader<&[u8]>,
    textelem: &mut Element,
) -> error_stack::Result<(), SfsPreprocessError> {
    tracing::trace!("processing div-dok");
    let mut state = ParseHtmlState::Start;
    let mut page_nr = 1;
    loop {
        tracing::trace!(state = ?state);
        match reader.read_event() {
            Err(e) => {
                return Err(Report::new(SfsPreprocessError::XmlParsingError {
                    pos: reader.buffer_position(),
                    err: e,
                }))
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                tracing::trace!(start = ?e);
                match e.name().as_ref() {
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
                }
            }
            Ok(Event::Text(content)) => match state {
                ParseHtmlState::Skip { tag: _ } => continue,
                _ => (),
            },
            Ok(Event::End(e)) => {
                tracing::trace!(end = ?e);
                if let ParseHtmlState::Skip { ref tag } = state {
                    if e.name().as_ref() == *tag {
                        state = ParseHtmlState::Start;
                    }
                    tracing::trace!(end = ?e, "skipping");
                    continue;
                }
                tracing::trace!(end = ?e);
            }
            Ok(Event::Comment(_)) => continue,
            Ok(e) => todo!("handle {:?} state={:?}", e, state),
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum ParseHtmlState<'a> {
    Start,
    ExtractPage,
    ExtractMetadataFoundKey { key: Cow<'a, str> },
    Dokument,
    Paragraph,
    Skip { tag: &'static [u8] },
}

#[tracing::instrument(skip(reader))]
pub fn extract_page(
    reader: &mut Reader<&[u8]>,
) -> error_stack::Result<Element, SfsPreprocessError> {
    tracing::trace!("extracting paragraphs");
    reader.check_end_names(false);
    let mut elem = Element::bare("page", "");
    let mut curr: Option<Element> = Some(Element::bare("p", ""));
    let mut state = ExtractPageState::InSida;
    loop {
        tracing::trace!(state = ?state);
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
                // dbg!("EOF", &state);
                break;
            }
            Ok(Event::Start(e)) => {
                if let ExtractPageState::Skip { tag: _ } = &state {
                    tracing::trace!(start = ?e,"skipping");
                    continue;
                }
                tracing::trace!(start = ?e);
                match e.name().as_ref() {
                    b"div" if attrib_equals(&e, b"class", b"block") => {
                        state = ExtractPageState::InBlock;
                        continue;
                    }
                    b"p" => {
                        // if let ExtractPageState::InBlock = &state {
                        //     println!("skipping {:?}", curr);
                        // } else {
                        if let Some(p) = curr.take() {
                            // dbg!(&p);
                            if !elem_is_empty(&p) {
                                // dbg!(&p);
                                elem.append_child(p);
                            }
                        }
                        // }
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
                        println!(
                            "handling unknown Start {:?} state={:?} pos={}",
                            e,
                            state,
                            reader.buffer_position()
                        );
                        tracing::warn!(
                            event = ?e,
                            ?state,
                            pos = reader.buffer_position(),
                            "handling unknown Start",
                        );
                        let mut raw_text = vec![b'<'];
                        raw_text.extend(should_be_text);
                        raw_text.extend(e.attributes_raw());
                        let mut text = String::from_utf8_lossy(&raw_text).to_string();
                        dbg!(&text);
                        // text = text.replace("</span", " ");
                        tracing::warn!(text, ?state, "text from unknown tag");
                        panic!("bad xml");
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
                // dbg!("Empty", &e, &state);
                match state {
                    ExtractPageState::Skip { tag: _ } => continue,
                    _ => (),
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
                match state {
                    ExtractPageState::Skip { tag: _ } => {
                        // dbg!(&content, "skipping", &state, reader.buffer_position());
                        continue;
                    }
                    _ => (),
                }
                let text = match content.unescape() {
                    Ok(text) => text.to_string(),
                    Err(err) => {
                        tracing::error!(error = ?err, "making text of error");
                        let err_content = content.into_inner();
                        match String::from_utf8(err_content.to_vec()) {
                            Ok(text) => text,
                            Err(err) => {
                                return Err(Report::new(SfsPreprocessError::XmlFromUtf8Error {
                                    pos: reader.buffer_position(),
                                    err,
                                }));
                            }
                        }
                    }
                };
                // .map_err(|err| SfsPreprocessError::XmlParsingError {
                //     pos: reader.buffer_position(),
                //     err,
                // })?;
                tracing::trace!(text = ?text);

                if let Some(curr_elem) = &mut curr {
                    curr_elem.append_text_node(text);
                } else {
                    todo!("handle curr empty state={:?}", state);
                }
            }
            Ok(Event::End(e)) => {
                if let ExtractPageState::Skip { ref tag } = state {
                    if e.name().as_ref() == *tag {
                        state = ExtractPageState::InBlock;
                    }
                    tracing::trace!(end = ?e, "skipping");
                    // dbg!(&e, "skipping", &state, reader.buffer_position());
                    continue;
                }
                tracing::trace!(end = ?e, state = ?state);
                match e.name().as_ref() {
                    b"div" => {
                        state = match state {
                            ExtractPageState::InBlock => ExtractPageState::InSida,
                            ExtractPageState::InSida => ExtractPageState::InPageWrap,
                            ExtractPageState::InPageWrap => {
                                tracing::trace!("returning");
                                break;
                            }
                            state => state,
                        };
                        // if let ExtractPageState::InPageWrap = state {
                        //     tracing::trace!("returning");
                        //     break;
                        // }
                    }
                    // b"p" => {
                    //     if let Some(p) = curr.take() {
                    //         if !elem_is_empty(&p) {
                    //             elem.append_child(p);
                    //         }
                    //         curr = Some(Element::bare("p", ""));
                    //     }
                    // }
                    b"p" => {
                        state = ExtractPageState::InBlock;
                        continue;
                    }
                    b"span" => continue,
                    _ => todo!("handle End {:?} state={:?}", e, state),
                }
            }
            Ok(Event::Comment(e)) => {
                tracing::trace!(comment = ?e,"skipping");
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

fn into_line_and_column(reader: &mut Reader<&[u8]>) -> (usize, usize) {
    let end_pos = reader.buffer_position();
    let mut cursor = reader.get_ref();
    let s = String::from_utf8(
        cursor /*[0..end_pos]*/
            .to_vec(),
    )
    .expect("can't make a string");
    let mut line = 1;
    let mut column = 0;
    for c in s.chars() {
        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    (line, column)
}
