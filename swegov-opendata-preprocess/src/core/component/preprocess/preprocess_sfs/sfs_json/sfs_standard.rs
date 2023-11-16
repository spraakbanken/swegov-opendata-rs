use std::borrow::Cow;

use error_stack::Report;
use minidom::{
    quick_xml::{events::Event, Reader},
    Element, Node,
};

use crate::{
    core::component::preprocess::preprocess_sfs::SfsPreprocessError,
    nodeinfo::minidom::elem_is_empty,
};

#[tracing::instrument(skip(reader, textelem))]
pub fn process_html_sfs_standard(
    reader: &mut Reader<&[u8]>,
    textelem: &mut Element,
) -> error_stack::Result<(), SfsPreprocessError> {
    let mut state = ParseHtmlState::Start;
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
                if let ParseHtmlState::Skip { tag: _ } = &state {
                    tracing::trace!(start = ?e,"skipping");
                    continue;
                }
                tracing::trace!(start = ?e);
                state = match e.name().as_ref() {
                    b"style" => ParseHtmlState::Skip { tag: b"style" },
                    b"b" => {
                        if let ParseHtmlState::Start = state {
                            ParseHtmlState::ExtractMetadata
                        } else {
                            state
                        }
                    }
                    b"a" => match state {
                        ParseHtmlState::ExtractMetadataFoundKey { key } => {
                            let mut new_state = None;
                            for attr in e.attributes() {
                                let attr =
                                    attr.map_err(|err| SfsPreprocessError::XmlParsingAttrError {
                                        pos: reader.buffer_position(),
                                        err: err,
                                    })?;
                                if attr.key.local_name().as_ref() == b"href" {
                                    textelem.set_attr(
                                        key.as_ref().to_lowercase(),
                                        attr.unescape_value().unwrap().as_ref(),
                                    );
                                    new_state = Some(ParseHtmlState::ExtractMetadata);
                                }
                            }
                            new_state
                                .unwrap_or_else(|| ParseHtmlState::ExtractMetadataFoundKey { key })
                        }
                        ParseHtmlState::Paragraph => {
                            continue;
                        }
                        _ => todo!(),
                    },
                    b"div" => match state {
                        ParseHtmlState::ExtractMetadata | ParseHtmlState::Start => {
                            if e.attributes()
                                .find(|attr| {
                                    attr.as_ref()
                                        .map(|attr| {
                                            attr.key.local_name().as_ref() == b"class"
                                                && attr.value.as_ref() == b"sfstoc"
                                        })
                                        .is_ok()
                                })
                                .is_some()
                            {
                                ParseHtmlState::Skip { tag: b"div" }
                            } else {
                                let paragraphs = extract_paragraphs(reader)?;
                                // todo!("handle paragraphs={:?}", paragraphs);
                                for paragraph in paragraphs {
                                    textelem.append_child(paragraph);
                                }
                                ParseHtmlState::Dokument
                            }
                        }
                        _ => todo!(),
                    },
                    b"p" => match state {
                        ParseHtmlState::Dokument | ParseHtmlState::Start => {
                            let paragraphs = extract_paragraphs(reader)?;
                            // todo!("handle paragraphs={:?}", paragraphs);
                            for paragraph in paragraphs {
                                textelem.append_child(paragraph);
                            }
                            ParseHtmlState::Paragraph
                        }
                        _ => todo!("handle {:?} for state={:?}", e, state),
                    },
                    _ => todo!("handle {:?}", e),
                }
            }
            Ok(Event::Text(content)) => {
                match state {
                    ParseHtmlState::Start | ParseHtmlState::Skip { tag: _ } => continue,
                    _ => (),
                }
                let content_text =
                    content
                        .unescape()
                        .map_err(|err| SfsPreprocessError::XmlParsingError {
                            pos: reader.buffer_position(),
                            err,
                        })?;
                // .change_context_lazy(|| SfsPreprocessError::Xml())?;
                tracing::trace!(text = ?content_text);
                match state {
                    ParseHtmlState::ExtractMetadata => {
                        if ["Ändringsregister", "Källa"].contains(&content_text.as_ref()) {
                            state = ParseHtmlState::ExtractMetadataFoundKey { key: content_text };
                        }
                    }
                    ParseHtmlState::ExtractMetadataFoundKey { key: _ } => continue,
                    _ => todo!("handle content_text='{}'", content_text),
                }
                // todo!("handle content_text='{}'", content_text);
            }
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
                match e.name().as_ref() {
                    b"style" => state = ParseHtmlState::Start,
                    b"a" => match state {
                        ParseHtmlState::ExtractMetadata => continue,
                        ParseHtmlState::Paragraph => continue,

                        _ => todo!(),
                    },
                    b"b" => match state {
                        ParseHtmlState::ExtractMetadata
                        | ParseHtmlState::ExtractMetadataFoundKey { key: _ } => continue,
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
            Ok(Event::Empty(e)) => {
                tracing::trace!(empty = ?e);
                match e.name().as_ref() {
                    b"p" => {
                        let paragraphs = extract_paragraphs(reader)?;
                        // todo!("handle paragraphs={:?}", paragraphs);
                        for paragraph in paragraphs {
                            textelem.append_child(paragraph);
                        }
                        state = ParseHtmlState::Paragraph
                    }
                    _ => continue,
                }
            }
            Ok(Event::Comment(_)) => continue,
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum ParseHtmlState<'a> {
    Start,
    ExtractMetadata,
    ExtractMetadataFoundKey { key: Cow<'a, str> },
    Dokument,
    Paragraph,
    Skip { tag: &'static [u8] },
}

#[tracing::instrument(skip(reader))]
pub fn extract_paragraphs(
    reader: &mut Reader<&[u8]>,
) -> error_stack::Result<Vec<Element>, SfsPreprocessError> {
    tracing::trace!("extracting paragraphs");
    let mut state = ParagraphsState::Start;
    let mut paragraphs = Vec::new();
    let mut curr = Some(Element::bare("p", ""));
    loop {
        tracing::trace!(state = ?state);
        match reader.read_event() {
            Err(err) => {
                tracing::error!(error= ?err, pos = reader.buffer_position(), "handle err ");
                todo!("handle err {:?}, pos={}", err, reader.buffer_position());
            }
            Ok(Event::Start(e)) => {
                tracing::trace!(start = ?e);
                match e.name().as_ref() {
                    b"a" => continue,
                    b"p" => {
                        if let Some(elem) = curr.take() {
                            paragraphs.push(elem);
                        }
                        curr = Some(Element::bare("p", ""));
                    }
                    b"pre" => continue,
                    b"b" | b"i" => continue,
                    b"h1" | b"h2" | b"h3" | b"h4" => {
                        if let Some(elem) = curr.take() {
                            // dbg!(&elem);
                            if !elem_is_empty(&elem) {
                                paragraphs.push(elem);
                            }
                        }
                        curr = Some(Element::bare("p", ""));
                    }
                    _ => todo!("handle Start {:?} ", e),
                }
            }
            Ok(Event::End(e)) => {
                tracing::trace!(end = ?e);
                match e.name().as_ref() {
                    b"a" => continue,
                    b"p" => match state {
                        ParagraphsState::Start => continue,
                    },
                    b"pre" => continue,
                    b"div" => break,
                    b"b" | b"i" => continue,
                    b"h1" | b"h2" | b"h3" | b"h4" => {
                        if let Some(elem) = curr.take() {
                            paragraphs.push(elem);
                        }
                        curr = Some(Element::bare("p", ""));
                    }
                    _ => todo!("handle End {:?} ", e),
                }
            }
            Ok(Event::Text(content)) => {
                tracing::trace!(content = ?content);
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
                tracing::trace!(text = ?text);
                if let Some(elem) = &mut curr {
                    elem.append_text_node(text);
                }
            }
            Ok(Event::Empty(e)) => {
                tracing::trace!(empty = ?e);
                match e.name().as_ref() {
                    b"br" => {
                        if let Some(elem) = &mut curr {
                            elem.append_child(Element::bare("br", ""));
                        }
                    }
                    b"p" => continue,
                    _ => todo!("handle Empty: {:?}", e),
                }
            }
            Ok(Event::Eof) => break,
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    if let Some(elem) = curr.take() {
        if !elem_is_empty(&elem) {
            paragraphs.push(elem);
        }
    }
    Ok(paragraphs)
}

#[derive(Debug, Clone, PartialEq)]
enum ParagraphsState {
    Start,
}
