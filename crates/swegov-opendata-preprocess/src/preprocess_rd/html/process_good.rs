use exn::{Exn, ResultExt};
use minidom_extension::{
    attrib_query::attrib_equals,
    elem_is_empty,
    minidom::{
        quick_xml::{events::Event, Reader},
        Element,
    },
};

use crate::preprocess_rd::html::{
    extract_elem, extract_list, extract_page, extract_page_id_from_attributes, extract_paragraph,
    extract_paragraph_implicit_end, extract_paragraph_or_list, extract_table, process_div_bad,
    process_rd_lista, unescape, unquoted_qttribute, ParseHtmlState, ProcessDivError, Unexpected,
    UnexpectedTag,
};

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ProcessHtmlGoodError {
    #[error("process html failed at pos {0}")]
    AtPosition(u64),
    #[error("process html failed")]
    Failure,
    #[error("process html failed because of {0}")]
    Unexpected(Unexpected),
    #[error("process html failed: {msg}")]
    WithMsg { msg: String },
}

impl ProcessHtmlGoodError {
    pub fn unexpected_start_tag<S: Into<String>>(pos: u64, tag: &[u8], context: S) -> Self {
        Self::Unexpected(Unexpected::Start(UnexpectedTag::new(
            pos,
            tag,
            context.into(),
        )))
    }
    pub fn unexpected_empty_tag<S: Into<String>>(pos: u64, tag: &[u8], context: S) -> Self {
        Self::Unexpected(Unexpected::Empty(UnexpectedTag::new(
            pos,
            tag,
            context.into(),
        )))
    }
    pub fn unexpected_end_tag<S: Into<String>>(pos: u64, tag: &[u8], context: S) -> Self {
        Self::Unexpected(Unexpected::End(UnexpectedTag::new(
            pos,
            tag,
            context.into(),
        )))
    }
    pub fn with_msg<S: Into<String>>(msg: S) -> Self {
        Self::WithMsg { msg: msg.into() }
    }
}

pub fn process_html_good(
    reader: &mut Reader<&[u8]>,
) -> Result<Vec<Element>, Exn<ProcessHtmlGoodError>> {
    let make_error = || ProcessHtmlGoodError::Failure;

    let mut state = ParseHtmlState::Start;
    let mut unmatched_end_tags: Vec<String> = Vec::new();
    let mut res = Vec::new();

    loop {
        match reader
            .read_event()
            .or_raise(|| ProcessHtmlGoodError::AtPosition(reader.error_position()))?
        {
            Event::Empty(e) => {
                if let ParseHtmlState::Skip { tag: _ } = state {
                    continue;
                }
                match e.name().as_ref() {
                    b"br" | b"BR" | b"hr" | b"v" | b"a" | b"A" => (),
                    _ => {
                        exn::bail!(ProcessHtmlGoodError::unexpected_empty_tag(
                            reader.error_position(),
                            e.name().as_ref(),
                            "process_html",
                        ))
                    }
                }
            }
            Event::Start(e) => {
                if let ParseHtmlState::Skip { tag: _ } = state {
                    continue;
                }
                match e.name().as_ref() {
                    b"body" | b"BODY" | b"html" | b"HTML" => (),
                    b"div" | b"DIV" => {
                        if let Some(id) = extract_page_id_from_attributes(e.attributes()) {
                            let page = extract_page(reader, id).or_raise(make_error)?;
                            res.push(page);
                            state = ParseHtmlState::Start;
                        } else {
                            let elem = process_div(reader).or_raise(make_error)?;
                            for child in elem.children() {
                                res.push(child.clone());
                            }
                        }
                    }
                    b"hr" | b"link" | b"LINK" | b"label" | b"font" | b"FONT" => (),
                    b"h1" => {
                        if let ParseHtmlState::Paragraph(elem) = state {
                            res.push(elem);
                            state = ParseHtmlState::Start;
                        }
                        let (elem, end_tag) =
                            extract_paragraph_implicit_end(reader, e.name().as_ref())
                                // .or_raise(make_error)?,
                                .or_raise(make_error)?;
                        res.push(elem);
                        if let Some(end_tag) = end_tag {
                            unmatched_end_tags
                                .push(String::from_utf8_lossy(e.name().as_ref()).to_string());
                            let elem = extract_paragraph(reader, end_tag.as_bytes())
                                .or_raise(make_error)?;
                            res.push(elem);
                        }
                    }
                    b"pre" | b"p" | b"P" | b"h2" | b"h3" | b"h4" | b"h5" | b"h6" | b"li" => {
                        if let ParseHtmlState::Paragraph(elem) = state {
                            res.push(elem);
                            state = ParseHtmlState::Start;
                        }
                        res.push(
                            extract_paragraph(reader, e.name().as_ref()).or_raise(make_error)?,
                            // .or_raise(|| {
                            //     ProcessHtmlGoodError::with_msg(format!(
                            //         "textelem = {:?}",
                            //         textelem
                            //     ))
                            // })?,
                        );
                    }
                    b"head" | b"HEAD" | b"style" | b"STYLE" => {
                        state = ParseHtmlState::Skip {
                            tag: e.name().as_ref().to_vec(),
                        }
                    }
                    b"table" | b"TABLE" => {
                        let paragraphs = extract_table(reader).or_raise(make_error)?;
                        for p in paragraphs {
                            res.push(p);
                        }
                    }
                    b"br" | b"BR" => {
                        if let ParseHtmlState::Paragraph(elem) = &mut state {
                            elem.append_child(Element::bare("br", ""));
                        }
                    }
                    b"ol" | b"ul" => {
                        let paragraphs =
                            extract_list(reader, e.name().as_ref()).or_raise(make_error)?;
                        for p in paragraphs {
                            res.push(p);
                        }
                    }
                    // _ => todo!("handle Start({:?})", e),
                    b"span" if attrib_equals(&e, b"class", b"rd_lista") => {
                        let mut textelem = Element::bare("p", "");
                        process_rd_lista(reader, &mut textelem).or_raise(make_error)?;
                        res.push(textelem);
                    }
                    b"span" if attrib_equals(&e, b"class", b"DatumRad") => {
                        for elem in extract_paragraph_or_list(reader, e.name().as_ref())
                            .or_raise(make_error)?
                        {
                            res.push(elem);
                        }
                    }
                    b"span" => {
                        let p =
                            extract_paragraph(reader, e.name().as_ref()).or_raise(make_error)?;
                        if !elem_is_empty(&p) {
                            res.push(p);
                        }
                    }
                    b"UL" | b"DIR" | b"OL" => {
                        if let ParseHtmlState::Paragraph(elem) = state {
                            res.push(elem);
                            state = ParseHtmlState::Start;
                        }
                        for elem in extract_paragraph_or_list(reader, e.name().as_ref())
                            .or_raise(make_error)?
                        {
                            res.push(elem);
                        }
                    }
                    b"b" | b"B" | b"i" | b"I" => {
                        if let ParseHtmlState::Paragraph(ref mut elem) = &mut state {
                            elem.append_child(
                                extract_elem(reader, e.name().as_ref()).or_raise(make_error)?,
                            );
                            continue;
                        }
                        if let ParseHtmlState::Start = state {
                            let mut p = Element::bare("p", "");
                            p.append_child(
                                extract_elem(reader, e.name().as_ref()).or_raise(make_error)?,
                            );
                            state = ParseHtmlState::Paragraph(p);
                        }
                    }
                    _ => {
                        exn::bail!(ProcessHtmlGoodError::unexpected_start_tag(
                            reader.error_position(),
                            e.name().as_ref(),
                            "process_html_good".to_string(),
                        ));
                    }
                }
            }
            Event::Text(text) => match &mut state {
                ParseHtmlState::Skip { tag: _ } => (),
                ParseHtmlState::Paragraph(p) => {
                    let text = unescape(&text);
                    p.append_text_node(text);
                }
                _ => {
                    let text = unescape(&text);
                    if text.trim().is_empty() {
                        continue;
                    }
                    let mut p = Element::bare("p", "");
                    p.append_text_node(text);
                    state = ParseHtmlState::Paragraph(p);
                }
            },
            Event::End(e) => {
                if let ParseHtmlState::Skip { ref tag } = state {
                    if e.name().as_ref() == tag {
                        state = ParseHtmlState::Start;
                    }
                    continue;
                }
                match e.name().as_ref() {
                    b"style" | b"label" | b"body" | b"BODY" | b"html" | b"HTML" | b"font"
                    | b"FONT" => (),
                    b"b" | b"B" | b"i" => {
                        if let ParseHtmlState::Paragraph(_p) = &state {}
                        exn::bail!(ProcessHtmlGoodError::unexpected_end_tag(
                            reader.error_position(),
                            e.name().as_ref(),
                            format!("process_html, End={:?}", e),
                        ));
                    }
                    // skip errounues </I>
                    b"I" | b"span" | b"li" | b"ol" | b"em" => (),
                    _ => {
                        if !unmatched_end_tags.is_empty() && e.name().as_ref() == b"div" {
                            continue;
                        }
                        let found_end = if let Some(last) = unmatched_end_tags.last() {
                            last.as_bytes() == e.name().as_ref()
                        } else {
                            false
                        };
                        if found_end {
                            unmatched_end_tags.pop();
                            continue;
                        }

                        exn::bail!(ProcessHtmlGoodError::unexpected_end_tag(
                            reader.error_position(),
                            e.name().as_ref(),
                            format!(
                                "state={:?}, unmatched_end_tags={:?}",
                                state, unmatched_end_tags
                            ),
                        ))
                    }
                }
            }
            Event::Eof => break,
            Event::Comment(_) => (),
            Event::DocType(e) => {
                // let text = e.unescape().unwrap();
                // if text.contains("html1") {
                //     process_html1(reader, textelem)?;
                // } else if text.contains("html4") {
                //     process_html4(reader, textelem)?;
                // }
                todo!("handle DocType={:?}", e);
            }
            Event::Decl(_) => (),
            e => todo!("handle {:?}", e),
        }
    }
    if let ParseHtmlState::Paragraph(elem) = state {
        res.push(elem);
    }
    Ok(res)
}

fn process_div(reader: &mut Reader<&[u8]>) -> Result<Element, Exn<ProcessDivError>> {
    let make_error = || ProcessDivError::Failure;
    let mut state = ParseHtmlState::Start;
    let mut div_count = 1;
    let mut curr_elem_opt: Option<Element> = None;
    let mut elem = Element::bare("p", "");
    let textelem = &mut elem;
    loop {
        match reader
            .read_event()
            .or_raise(|| ProcessDivError::AtPosition(reader.error_position()))?
        {
            Event::Start(e) => match e.name().as_ref() {
                b"style" | b"STYLE" | b"img" | b"IMG" | b"script" | b"SCRIPT" => {
                    state = ParseHtmlState::Skip {
                        tag: e.name().as_ref().to_vec(),
                    }
                }
                b"div" | b"DIV" => {
                    if let Some(id) = extract_page_id_from_attributes(e.attributes()) {
                        if let Some(curr_elem) = curr_elem_opt.take() {
                            textelem.append_child(curr_elem);
                        }
                        let page = extract_page(reader, id).or_raise(make_error)?;
                        textelem.append_child(page);
                        state = ParseHtmlState::Start;
                    } else {
                        div_count += 1;
                    }
                }
                b"p" | b"P" | b"h1" | b"h2" | b"H2" | b"h3" | b"h4" | b"h5" | b"h6" | b"li"
                | b"o:p" => {
                    if let Some(curr_elem) = curr_elem_opt.take() {
                        textelem.append_child(curr_elem);
                    }
                    let elems = extract_paragraph_or_list(reader, e.name().as_ref())
                        .or_raise(make_error)?;
                    for elem in elems {
                        textelem.append_child(elem);
                    }
                }
                b"b" | b"B" | b"i" | b"I" | b"pre" | b"PRE" | b"strong" => {
                    if let Some(curr_elem) = curr_elem_opt.take() {
                        textelem.append_child(curr_elem);
                    }
                    let e = extract_elem(reader, e.name().as_ref()).or_raise(make_error)?;
                    textelem.append_child(e);
                }
                b"br" | b"BR" | b"hr" => {
                    if let Some(curr_elem) = curr_elem_opt.take() {
                        textelem.append_child(curr_elem);
                    }
                    textelem.append_child(Element::bare("br", ""));
                }
                b"table" | b"TABLE" => {
                    if let Some(curr_elem) = curr_elem_opt.take() {
                        textelem.append_child(curr_elem);
                    }
                    let paragraphs = extract_table(reader).or_raise(make_error)?;
                    // println!("process_div: extract_table: {paragraphs:?}");
                    for p in paragraphs {
                        textelem.append_child(p);
                    }
                }
                b"ol" | b"ul" => {
                    if let Some(curr_elem) = curr_elem_opt.take() {
                        textelem.append_child(curr_elem);
                    }
                    let paragraphs =
                        extract_list(reader, e.name().as_ref()).or_raise(make_error)?;
                    for p in paragraphs {
                        textelem.append_child(p);
                    }
                }
                b"body" | b"BODY" => (),
                b"noter"
                | b"hanvisning"
                | b"textovervagande"
                | b"rubriksarskiltyttrande"
                | b"yttrandebilaga" => (),
                b"span" | b"a" | b"font" | b"td" | b"tr" => (),
                _ => {
                    exn::bail!(ProcessDivError::unexpected_start_tag(
                        reader.error_position(),
                        e.name().as_ref(),
                        "process_div",
                    ))
                }
            },
            Event::Text(text) => {
                if let ParseHtmlState::Skip { tag: _ } = state {
                    continue;
                }
                let unescaped_text = unescape(&text);
                if unescaped_text.trim().is_empty() {
                    continue;
                }
                if let Some(curr_elem) = &mut curr_elem_opt {
                    curr_elem.append_text_node(unescaped_text);
                } else {
                    let mut elem = Element::bare("p", "");
                    elem.append_text_node(unescaped_text);
                    curr_elem_opt = Some(elem);
                }
            }
            Event::End(e) => {
                if let ParseHtmlState::Skip { ref tag } = state {
                    if e.name().as_ref() == tag {
                        state = ParseHtmlState::Start;
                    }
                    continue;
                }
                match e.name().as_ref() {
                    b"div" | b"DIV" => {
                        div_count -= 1;
                        if div_count == 0 {
                            break;
                        }
                    }
                    b"noter"
                    | b"textovervagande"
                    | b"rubriksarskiltyttrande"
                    | b"yttrandebilaga"
                    | b"td"
                    | b"tr"
                    | b"table" => (),
                    b"o:p" => (),
                    b"span" | b"a" | b"font" | b"p" => (),
                    _ => {
                        exn::bail!(ProcessDivError::unexpected_end_tag(
                            reader.error_position(),
                            e.name().as_ref(),
                            "process_div",
                        ))
                    }
                }
            }
            Event::Empty(_e) => (),
            Event::Eof => break,
            e => todo!("handle {:?}", e),
        }
    }
    if let Some(curr_elem) = curr_elem_opt.take() {
        elem.append_child(curr_elem);
    }
    Ok(elem)
}
