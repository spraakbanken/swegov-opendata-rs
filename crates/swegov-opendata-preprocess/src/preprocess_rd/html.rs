use core::fmt;
use std::borrow::Cow;

use minidom_extension::{
    attrib_query::attrib_equals,
    minidom::{
        quick_xml::{
            events::{
                attributes::{AttrError, Attributes},
                BytesText, Event,
            },
            Reader,
        },
        Element, Node,
    },
};
use once_cell::sync::Lazy;
use regex::Regex;

#[cfg(test)]
mod tests;

fn remove_cdata<'a>(text: &'a str) -> Cow<'a, str> {
    static CDATA: Lazy<Regex> = Lazy::new(|| Regex::new(r"<!.+?>").unwrap());
    CDATA.replace_all(text, "")
}
pub fn process_html(contents: &str, textelem: &mut Element) -> Result<(), ProcessHtmlError> {
    println!("process_html");
    let contents_processed = contents.replace("\r\n", " ");
    let contents_processed = contents_processed.replace("STYLEREF Kantrubrik \\* MERGEFORMAT", "");
    let contents_processed = contents_processed.replace("\u{a0}", "");
    let contents_processed = contents_processed.replace("&nbsp;", " ");
    let contents_processed = contents_processed.replace("&amp;ouml;", "ö");
    let contents_processed = contents_processed.replace("&amp;auml;", "ä");
    let contents_processed = contents_processed.replace("&amp;aring;", "å");
    let contents_processed = contents_processed.replace("&aring;", "å");
    let contents_processed = contents_processed.replace("&auml;", "ä");
    let contents_processed = contents_processed.replace("&ouml;", "ö");
    let contents_processed = contents_processed.replace("&Aring;", "Å");
    let contents_processed = contents_processed.replace("&Auml;", "Ä");
    let contents_processed = contents_processed.replace("&Ouml;", "Ö");
    let contents_processed = remove_cdata(&contents_processed);

    let mut reader = Reader::from_str(&contents_processed);
    reader.check_end_names(false);

    let mut state = ParseHtmlState::Start;

    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::Empty(e)) => {
                match state {
                    ParseHtmlState::Skip { tag: _ } => continue,
                    _ => (),
                }
                match e.name().as_ref() {
                    b"br" | b"BR" | b"hr" | b"v" => (),
                    _ => {
                        return Err(ProcessHtmlError::unexpected_empty_tag(
                            reader.buffer_position(),
                            e.name().as_ref(),
                            "process_html",
                        ))
                    }
                }
            }
            Ok(Event::Start(e)) => {
                match state {
                    ParseHtmlState::Skip { tag: _ } => continue,
                    _ => (),
                }
                match e.name().as_ref() {
                    b"body" | b"BODY" | b"html" | b"HTML" => (),
                    b"div" | b"DIV" => {
                        if unquoted_qttribute(e.attributes()) {
                            process_div_bad(&mut reader, textelem)?;
                            dbg!(&textelem);
                        } else if let Some(id) = extract_page_id_from_attributes(e.attributes()) {
                            let page = extract_page(&mut reader, id)?;
                            textelem.append_child(page);
                            state = ParseHtmlState::Start;
                        } else {
                            process_div(&mut reader, textelem)?;
                        }
                    }
                    b"hr" | b"link" | b"LINK" | b"label" | b"font" | b"FONT" => (),
                    b"h1" | b"pre" | b"p" | b"P" | b"h2" | b"h3" | b"h4" => {
                        if let ParseHtmlState::Paragraph(elem) = state {
                            textelem.append_child(elem);
                            state = ParseHtmlState::Start;
                        }
                        textelem.append_child(extract_paragraph(&mut reader, e.name().as_ref())?);
                    }
                    b"head" | b"HEAD" | b"style" | b"STYLE" => {
                        state = ParseHtmlState::Skip {
                            tag: e.name().as_ref().to_vec(),
                        }
                    }
                    b"table" | b"TABLE" => {
                        let paragraphs = extract_table(&mut reader)?;
                        for p in paragraphs {
                            textelem.append_child(p);
                        }
                    }
                    b"br" | b"BR" => {
                        if let ParseHtmlState::Paragraph(elem) = &mut state {
                            elem.append_child(Element::bare("br", ""));
                        }
                    }
                    b"ol" | b"ul" => {
                        let paragraphs = extract_list(&mut reader, e.name().as_ref())?;
                        for p in paragraphs {
                            textelem.append_child(p);
                        }
                    }
                    // _ => todo!("handle Start({:?})", e),
                    b"span" if attrib_equals(&e, b"class", b"rd_lista") => {
                        process_rd_lista(&mut reader, textelem)?;
                    }
                    b"span" if attrib_equals(&e, b"class", b"DatumRad") => {
                        let elem = extract_paragraph(&mut reader, e.name().as_ref())?;
                        textelem.append_child(elem);
                    }
                    b"UL" => {
                        if let ParseHtmlState::Paragraph(elem) = state {
                            textelem.append_child(elem);
                            state = ParseHtmlState::Start;
                        }
                        textelem.append_child(extract_paragraph(&mut reader, e.name().as_ref())?);
                    }
                    b"b" | b"B" | b"i" | b"I" => {
                        if let ParseHtmlState::Paragraph(ref mut elem) = &mut state {
                            elem.append_child(extract_elem(&mut reader, e.name().as_ref()));
                            continue;
                        }
                        if let ParseHtmlState::Start = state {
                            let mut p = Element::bare("p", "");
                            p.append_child(extract_elem(&mut reader, e.name().as_ref()));
                            state = ParseHtmlState::Paragraph(p);
                        }
                    }
                    _ => {
                        return Err(ProcessHtmlError::unexpected_start_tag(
                            reader.buffer_position(),
                            e.name().as_ref(),
                            "process_html",
                        ));
                    }
                }
            }
            Ok(Event::Text(text)) => match &mut state {
                ParseHtmlState::Skip { tag: _ } => (),
                ParseHtmlState::Paragraph(p) => {
                    let text = text.unescape().unwrap();
                    p.append_text_node(text);
                }
                _ => {
                    let text = text.unescape().unwrap();
                    if text.trim().is_empty() {
                        continue;
                    }
                    let mut p = Element::bare("p", "");
                    p.append_text_node(text);
                    state = ParseHtmlState::Paragraph(p);
                }
            },
            Ok(Event::End(e)) => {
                match state {
                    ParseHtmlState::Skip { ref tag } => {
                        if e.name().as_ref() == tag {
                            state = ParseHtmlState::Start;
                        }
                        continue;
                    }
                    _ => (),
                }
                match e.name().as_ref() {
                    b"style" | b"label" | b"body" | b"BODY" | b"html" | b"HTML" | b"font"
                    | b"FONT" => (),
                    b"b" | b"B" | b"i" | b"I" => {
                        if let ParseHtmlState::Paragraph(p) = &state {
                            dbg!(&p);
                        }
                        todo!("handle End={:?}", e);
                    }
                    _ => todo!("handle {:?}", e),
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::Comment(_)) => (),
            Ok(Event::DocType(e)) => {
                // let text = e.unescape().unwrap();
                // if text.contains("html1") {
                //     process_html1(&mut reader, textelem)?;
                // } else if text.contains("html4") {
                //     process_html4(&mut reader, textelem)?;
                // }
                todo!("handle DocType={:?}", e);
            }
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    if let ParseHtmlState::Paragraph(elem) = state {
        textelem.append_child(elem);
    }
    Ok(())
}
const IGNORED_TAG_PREFIXES: &[&'static [u8]] = &[b"v:", b"w:", b"o:"];
fn process_div_bad(
    reader: &mut Reader<&[u8]>,
    textelem: &mut Element,
) -> Result<(), ProcessHtmlError> {
    println!("process_div_bad");
    let mut div_count = 1;
    let mut p_count = 0;
    let mut curr_p: Option<Element> = None;
    let mut stack: Vec<Element> = Vec::new();
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::Text(text)) => {
                dbg!(&text);
                let text = unescape(&text);
                if !text.trim().is_empty() {
                    // if let Some(p) = curr_p.as_mut() {
                    //     p.append_text_node(text.clone());
                    // } else {
                    //     todo!("handle non-empty text");
                    // }
                    if let Some(e) = stack.last_mut() {
                        e.append_text_node(text);
                    } else {
                        todo!("handle non-empty text");
                    }
                }
            }
            Ok(Event::Start(e)) => {
                dbg!(&e);
                match e.name().as_ref() {
                    b"div" => div_count += 1,
                    b"table" | b"tr" | b"td" | b"font" | b"img" | b"xml" => (),
                    b"span" => (),
                    b"p" | b"h1" | b"h2" | b"h3" | b"h4" => {
                        if p_count == 0 {
                            // if let Some(p) = curr_p.take() {
                            //     textelem.append_child(p);
                            // }
                            // curr_p = Some(Element::bare("p", ""));
                            stack.push(Element::bare("p", ""));
                        }
                        p_count += 1;
                    }
                    b"a" => (),
                    b"b" | b"i" => stack.push(Element::bare(
                        String::from_utf8_lossy(e.name().as_ref()).to_lowercase(),
                        "",
                    )),
                    b"br" => {
                        if let Some(p) = stack.last_mut() {
                            p.append_child(Element::bare("br", ""));
                        }
                    }
                    tag if IGNORED_TAG_PREFIXES.contains(&&tag[0..2]) => (),
                    _ => todo!("handle Start={:?}", e),
                }
            }
            Ok(Event::End(e)) => match dbg!(&e).name().as_ref() {
                b"div" => {
                    div_count -= 1;
                    if div_count == 0 {
                        break;
                    }
                }
                b"table" | b"tr" | b"td" | b"font" | b"img" | b"xml" => (),
                b"span" => (),
                b"a" => (),
                b"p" | b"h1" | b"h2" | b"h3" | b"h4" => {
                    p_count -= 1;
                    // if p_count == 0 {
                    //     if let Some(p) = curr_p.take() {
                    //         textelem.append_child(p);
                    //     }
                    // }
                }
                b"b" | b"i" => {
                    if let Some(b) = stack.pop() {
                        if let Some(p) = stack.last_mut() {
                            p.append_child(b);
                        }
                    }
                }
                tag if IGNORED_TAG_PREFIXES.contains(&&tag[0..2]) => (),
                _ => todo!("handle End={:?}", e),
            },
            Ok(Event::Empty(e)) => match dbg!(&e).name().as_ref() {
                tag if IGNORED_TAG_PREFIXES.contains(&&tag[0..2]) => (),
                _ => todo!("handle Empty={:?}", e),
            },
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    for elem in stack {
        textelem.append_child(elem);
    }
    Ok(())
}
fn process_html4(
    reader: &mut Reader<&[u8]>,
    textelem: &mut Element,
) -> Result<(), ProcessHtmlError> {
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    Ok(())
}
fn process_div(reader: &mut Reader<&[u8]>, textelem: &mut Element) -> Result<(), ProcessHtmlError> {
    let mut state = ParseHtmlState::Start;
    let mut div_count = 1;
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"style" | b"STYLE" | b"img" | b"IMG" => {
                    state = ParseHtmlState::Skip {
                        tag: e.name().as_ref().to_vec(),
                    }
                }
                b"div" | b"DIV" => {
                    if let Some(id) = extract_page_id_from_attributes(e.attributes()) {
                        let page = extract_page(reader, id)?;
                        textelem.append_child(page);
                        state = ParseHtmlState::Start;
                    } else {
                        div_count += 1;
                    }
                }
                b"p" | b"P" | b"h1" | b"h2" | b"H2" | b"h3" | b"h4" => {
                    textelem.append_child(extract_paragraph(reader, e.name().as_ref())?);
                }
                b"table" | b"TABLE" => {
                    let paragraphs = extract_table(reader)?;
                    println!("process_div: extract_table: {paragraphs:?}");
                    for p in paragraphs {
                        textelem.append_child(p);
                    }
                }
                b"ol" | b"ul" => {
                    let paragraphs = extract_list(reader, e.name().as_ref())?;
                    for p in paragraphs {
                        textelem.append_child(p);
                    }
                }
                b"noter"
                | b"hanvisning"
                | b"textovervagande"
                | b"rubriksarskiltyttrande"
                | b"yttrandebilaga" => (),
                b"o:p" => (),
                b"span" | b"a" | b"font" | b"td" | b"tr" => (),
                _ => {
                    return Err(ProcessHtmlError::unexpected_start_tag(
                        reader.buffer_position(),
                        e.name().as_ref(),
                        "process_div",
                    ))
                }
            },
            Ok(Event::Text(_t)) => match state {
                ParseHtmlState::Skip { tag: _ } => continue,
                _ => (),
            },
            Ok(Event::End(e)) => {
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
                        return Err(ProcessHtmlError::unexpected_end_tag(
                            reader.buffer_position(),
                            e.name().as_ref(),
                            "process_div",
                        ))
                    }
                }
            }
            Ok(Event::Empty(_e)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum ParseHtmlState {
    Start,
    Paragraph(Element),
    Skip { tag: Vec<u8> },
}

fn extract_paragraph(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<Element, ProcessHtmlError> {
    let mut elem = Element::bare("p", "");
    let mut curr_node: Option<Node> = None;
    let mut just_seen_span = false;
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle error {:?}", e),
            Ok(Event::Text(text)) => {
                let text = unescape(&text).to_string();
                match curr_node {
                    None => curr_node = Some(Node::Text(text)),
                    Some(Node::Element(e)) => {
                        elem.append_child(e);
                        curr_node = Some(Node::Text(text));
                    }
                    Some(Node::Text(t)) => {
                        curr_node = Some(Node::Text(format!("{}{}", t, text)));
                    }
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                e_tag if e_tag == tag => break,
                b"a" | b"A" | b"p" | b"P" | b"notreferens" | b"hanvisning" | b"kant" | b"h4"
                | b"font" | b"div" | b"pre" | b"xml" => just_seen_span = false,
                b"span" | b"SPAN" => just_seen_span = true,
                // Handle errornous </NOBR> in at least one document
                b"nobr" | b"NOBR" => just_seen_span = false,
                tag if &tag[0..2] == b"o:" => just_seen_span = false,
                tag if &tag[0..2] == b"v:" => just_seen_span = false,
                _ => {
                    return Err(ProcessHtmlError::unexpected_end_tag(
                        reader.buffer_position(),
                        e.name().as_ref(),
                        "extract_paragraph",
                    ))
                }
            },

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"nobr" | b"NOBR" | b"em" | b"EM" | b"sup" | b"i" | b"b" => {
                    just_seen_span = false;
                    if let Some(node) = curr_node.take() {
                        elem.append_node(node);
                    }
                    let e = extract_elem(reader, e.name().as_ref());
                    elem.append_child(e);
                }
                b"br" | b"BR" => {
                    just_seen_span = false;
                    if let Some(node) = curr_node.take() {
                        elem.append_node(node);
                    }
                    elem.append_child(Element::bare("br", ""));
                }
                b"a" | b"A" | b"notreferens" => (),
                b"span" | b"SPAN" => {
                    if just_seen_span {
                        curr_node.as_mut().map(|node| match node {
                            Node::Text(t) => t.push_str(" "),
                            _ => (),
                        });
                    }
                }
                b"p" | b"P" | b"hanvisning" | b"kant" | b"h4" | b"font" | b"." | b"div"
                | b"pre" | b"INGENBILD" | b"img" => (),
                b"table" | b"tr" | b"td" | b"xml" => (),
                tag if &tag[0..2] == b"o:" => (),
                tag if &tag[0..2] == b"v:" => (),
                _ => {
                    return Err(ProcessHtmlError::unexpected_start_tag(
                        reader.buffer_position(),
                        e.name().as_ref(),
                        "extract_paragraph",
                    ));
                }
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"br" | b"BR" => {
                    just_seen_span = false;
                    if let Some(node) = curr_node.take() {
                        elem.append_node(node);
                    }
                    elem.append_child(Element::bare("br", ""));
                }
                b"p" | b"P" | b"a" | b"A" | b"img" | b"IMG" => (),
                b"w:wrap" | b"o:lock" => (),
                tag if &tag[0..2] == b"v:" => (),
                _ => {
                    return Err(ProcessHtmlError::unexpected_empty_tag(
                        reader.buffer_position(),
                        e.name().as_ref(),
                        "extract_paragraph",
                    ))
                }
            },
            Ok(Event::Comment(_)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    if let Some(node) = curr_node.take() {
        elem.append_node(node);
    }
    Ok(elem)
}

fn process_rd_lista(
    reader: &mut Reader<&[u8]>,
    textelem: &mut Element,
) -> Result<(), ProcessHtmlError> {
    let mut curr_elem = Some(Element::bare("p", ""));
    let mut span_count = 1;
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle error={:?}", e),
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"br" => {
                    if let Some(elem) = curr_elem.as_mut() {
                        elem.append_child(Element::bare("br", ""));
                    } else {
                        todo!("handle curr_elem={:?} start={:?}", curr_elem, e);
                    }
                }
                b"a" | b"div" => {
                    if let Some(elem) = curr_elem.take() {
                        textelem.append_child(elem);
                    }
                    curr_elem = Some(extract_paragraph(reader, e.name().as_ref())?);
                }
                b"span" => span_count += 1,
                _ => todo!("handle Start={:?}", e),
            },
            Ok(Event::Text(text)) => {
                if let Some(elem) = curr_elem.as_mut() {
                    elem.append_text_node(unescape(&text));
                } else {
                    todo!("handle curr_elem={:?} text={:?}", curr_elem, text);
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"span" | b"SPAN" => {
                    span_count -= 1;
                    if span_count == 0 {
                        break;
                    }
                }
                _ => todo!("handle End={:?}", e),
            },
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    Ok(())
}
fn extract_list(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<Vec<Element>, ProcessHtmlError> {
    let mut items = Vec::new();
    let mut curr_item = None;
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle error={:?}", e),
            Ok(Event::Start(e)) => match e.name().as_ref() {
                // b"li" => list.push(extract_paragraph(reader, e.name().as_ref())?),
                b"li" => {
                    if let Some(item) = curr_item.take() {
                        items.push(item);
                    }
                    curr_item = Some(Element::bare("p", ""));
                }
                b"br" => {
                    if let Some(item) = curr_item.as_mut() {
                        item.append_child(Element::bare("br", ""));
                    }
                }
                b"b" | b"i" => {
                    if let Some(item) = curr_item.as_mut() {
                        let e = extract_elem(reader, e.name().as_ref());
                        item.append_child(e);
                    }
                }
                b"span" | b"p" => (),
                _ => todo!("handle Start({:?})", e),
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                e_tag if e_tag == tag => break,
                b"span" | b"p" => (),
                b"li" => {
                    if let Some(item) = curr_item.take() {
                        items.push(item);
                    }
                }
                _ => todo!("handle End({:?})", e),
            },
            Ok(Event::Text(text)) => {
                let text = text.unescape().unwrap();
                if let Some(item) = curr_item.as_mut() {
                    item.append_text_node(text);
                } else if !text.trim().is_empty() {
                    todo!("handle text outside of li");
                }
            }
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    if let Some(item) = curr_item.take() {
        items.push(item);
    }
    Ok(items)
}
fn extract_page(reader: &mut Reader<&[u8]>, id: String) -> Result<Element, ProcessHtmlError> {
    let mut elem = Element::bare("page", "");
    elem.set_attr("id", &id);
    let mut curr_child: Option<Element> = None;
    let mut div_count = 1;
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"div" | b"DIV" => {
                    div_count -= 1;
                    if div_count == 0 {
                        break;
                    }
                }
                b"img" | b"IMG" => (),
                b"table" | b"TABLE" => (),
                _ => todo!("handle {:?}", e),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"div" | b"DIV" => div_count += 1,
                b"img" | b"IMG" | b"ingenbild" | b"INGENBILD" => (),
                b"table" | b"TABLE" => {
                    let paragraphs = extract_table(reader)?;
                    if let Some(child) = curr_child.take() {
                        elem.append_child(child);
                    }
                    for p in paragraphs {
                        elem.append_child(p);
                    }
                }
                b"p" | b"P" | b"span" | b"SPAN" | b"a" | b"A" => {
                    if let Some(child) = curr_child.take() {
                        elem.append_child(child);
                    }
                    curr_child = Some(extract_paragraph(reader, e.name().as_ref())?);
                }
                b"nobr" | b"NOBR" => {
                    if let Some(child) = curr_child.as_mut() {
                        let e = extract_elem(reader, e.name().as_ref());
                        child.append_child(e);
                    }
                }
                _ => todo!("handle Start({:?})", e),
            },
            Ok(Event::Text(_text)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    if let Some(child) = curr_child.take() {
        elem.append_child(child);
    }
    Ok(elem)
}

fn unquoted_qttribute(attrs: Attributes) -> bool {
    for attr in attrs {
        if let Err(AttrError::UnquotedValue(_)) = attr {
            return true;
        }
    }
    false
}
fn extract_page_id_from_attributes(attrs: Attributes) -> Option<String> {
    for attr in attrs {
        let attr = match attr {
            Ok(attr) => attr,
            Err(err) => {
                tracing::warn!("Error reading attribute: {:?}, skipping ..", err);
                continue;
            }
        };
        if attr.key.as_ref() == b"id" {
            if let Some(id) = attr.value.strip_prefix(b"page_") {
                return Some(String::from_utf8(id.to_vec()).expect("valid utf8"));
            }
        }
    }
    None
}

fn extract_elem(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Element {
    let mut elem = Element::bare(String::from_utf8_lossy(tag).to_lowercase(), "");
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::Text(text)) => {
                elem.append_text_node(unescape(&text));
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                e_tag if e_tag == tag => break,
                b"a" | b"A" | b"span" | b"SPAN" | b"o:p" | b"font" | b"FONT" => (),
                _ => todo!("handle End({:?})", e),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"a" | b"A" | b"span" | b"SPAN" | b"font" | b"FONT" | b"o:p" => (),
                b"br" => {
                    elem.append_child(Element::bare("br", ""));
                }
                _ => todo!(
                    "handle Start({:?}), tag={}",
                    e,
                    String::from_utf8_lossy(tag)
                ),
            },
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    elem
}

fn unescape<'a>(text: &'a BytesText) -> Cow<'a, str> {
    match text.unescape() {
        Ok(text) => text,
        Err(e) => {
            let bad_text = String::from_utf8_lossy(text.as_ref());
            tracing::warn!("Unescape error for '{}': {:?}; Using as is...", bad_text, e);
            bad_text
        }
    }
}
fn extract_href_from_attributes(attrs: Attributes) -> Option<String> {
    for attr in attrs {
        let attr = attr.expect("a valid attribute");
        if attr.key.as_ref() == b"href" {
            return Some(String::from_utf8(attr.value.to_vec()).expect("valid utf8"));
        }
    }
    None
}

fn extract_table(reader: &mut Reader<&[u8]>) -> Result<Vec<Element>, ProcessHtmlError> {
    let mut table = Vec::new();
    let curr_elem: Option<Element> = None;
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"table" | b"TABLE" => break,
                b"tbody" | b"TBODY" => (),
                b"tr" | b"TR" => (),
                b"thead" => (),
                b"colgroup" => (),
                _ => todo!("handle End({:?})", e),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"a" | b"A" => {
                    let mut p = extract_paragraph(reader, e.name().as_ref())?;
                    if let Some(href) = extract_href_from_attributes(e.attributes()) {
                        p.set_attr("link", href);
                    }
                    table.push(p);
                }
                b"tbody" | b"TBODY" => (),
                b"thead" => (),
                b"tr" | b"TR" => (),
                b"td" | b"TD" | b"th" | b"TH" => {
                    table.push(extract_paragraph(reader, e.name().as_ref())?);
                }
                b"span" | b"SPAN" => (),
                b"colgroup" => (),
                _ => todo!("handle Start({:?})", e),
            },
            Ok(Event::Text(_text)) => (),
            Ok(Event::Empty(_e)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    Ok(table)
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ProcessHtmlError {
    #[error("Unexpected start {0}")]
    UnexpectedStartTag(UnexpectedTag),
    #[error("Unexpected empty {0}")]
    UnexpectedEmptyTag(UnexpectedTag),
    #[error("Unexpected end {0}")]
    UnexpectedEndTag(UnexpectedTag),
}

impl ProcessHtmlError {
    pub fn unexpected_start_tag(pos: usize, tag: &[u8], context: &str) -> Self {
        Self::UnexpectedStartTag(UnexpectedTag::new(pos, tag, context))
    }
    pub fn unexpected_empty_tag(pos: usize, tag: &[u8], context: &str) -> Self {
        Self::UnexpectedEmptyTag(UnexpectedTag::new(pos, tag, context))
    }
    pub fn unexpected_end_tag(pos: usize, tag: &[u8], context: &str) -> Self {
        Self::UnexpectedEndTag(UnexpectedTag::new(pos, tag, context))
    }
}

#[derive(Debug)]
pub struct UnexpectedTag {
    pos: usize,
    tag: String,
    context: String,
}

impl UnexpectedTag {
    pub fn new(pos: usize, tag: &[u8], context: &str) -> Self {
        Self {
            pos,
            tag: String::from_utf8_lossy(tag).to_string(),
            context: context.to_string(),
        }
    }
}

impl fmt::Display for UnexpectedTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "tag='{}' at pos={} in {}",
            self.tag, self.pos, self.context
        ))
    }
}
