use minidom_extension::minidom::{
    quick_xml::{
        events::{attributes::Attributes, Event},
        Reader,
    },
    Element, Node,
};

#[cfg(test)]
mod tests;

pub fn process_html(contents: &str, textelem: &mut Element) {
    let contents_processed = contents.replace("\r\n", " ");
    let contents_processed = contents_processed.replace("STYLEREF Kantrubrik \\* MERGEFORMAT", "");
    let contents_processed = contents_processed.replace("\u{a0}", "");
    let contents_processed = contents_processed.replace("&nbsp;", " ");

    let mut reader = Reader::from_str(&contents_processed);
    let mut state = ParseHtmlState::Start;

    loop {
        println!("state = {:?}", state);
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::Empty(e)) => {
                match state {
                    ParseHtmlState::Skip { tag: _ } => continue,
                    _ => (),
                }
                match e.name().as_ref() {
                    b"br" | b"BR" => (),
                    _ => todo!("handle Empty({:?}), state={:?}", e, state),
                }
            }
            Ok(Event::Start(e)) => {
                match state {
                    ParseHtmlState::Skip { tag: _ } => continue,
                    _ => (),
                }
                match e.name().as_ref() {
                    b"body" | b"BODY" => (),
                    b"div" => process_div(&mut reader, textelem),
                    b"hr" => (),
                    b"h1" | b"pre" | b"p" | b"h2" | b"h3" | b"h4" => {
                        textelem.append_child(extract_paragraph(&mut reader, e.name().as_ref()));
                    }
                    b"head" | b"HEAD" | b"style" | b"STYLE" => {
                        state = ParseHtmlState::Skip {
                            tag: e.name().as_ref().to_vec(),
                        }
                    }
                    b"table" | b"TABLE" => {
                        let paragraphs = extract_table(&mut reader);
                        for p in paragraphs {
                            textelem.append_child(p);
                        }
                    }
                    b"br" | b"BR" => {
                        if let ParseHtmlState::Paragraph(elem) = &mut state {
                            elem.append_child(Element::bare("br", ""));
                        }
                    }
                    _ => todo!("handle Start({:?})", e),
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
                    b"style" => (),
                    _ => todo!("handle {:?}", e),
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::Comment(_)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    if let ParseHtmlState::Paragraph(elem) = state {
        textelem.append_child(elem);
    }
}

fn process_div(reader: &mut Reader<&[u8]>, textelem: &mut Element) {
    let mut state = ParseHtmlState::Start;
    reader.check_end_names(false);
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
                        let mut page = extract_page(reader);
                        page.set_attr("id", id);
                        textelem.append_child(page);
                        state = ParseHtmlState::Start;
                    } else {
                        div_count += 1;
                    }
                }
                b"p" | b"P" | b"h2" | b"H2" | b"h4" => {
                    textelem.append_child(extract_paragraph(reader, e.name().as_ref()));
                }
                b"table" | b"TABLE" => {
                    let paragraphs = extract_table(reader);
                    for p in paragraphs {
                        textelem.append_child(p);
                    }
                }
                b"noter"
                | b"hanvisning"
                | b"textovervagande"
                | b"rubriksarskiltyttrande"
                | b"yttrandebilaga" => (),
                _ => todo!("handle Start({:?})", e),
            },
            Ok(Event::Text(_t)) => match state {
                ParseHtmlState::Skip { tag: _ } => continue,
                // _ => {
                //     if t.as_ref() == b"\r\n" {
                //         continue;
                //     }
                //     todo!("handle text '{:?}'", t);
                // }
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
                    | b"yttrandebilaga" => (),
                    _ => todo!("handle End({:?})", e),
                }
            }
            Ok(Event::Empty(_e)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    reader.check_end_names(true);
}

#[derive(Debug, Clone, PartialEq)]
enum ParseHtmlState {
    Start,
    // ExtractPage,
    // ExtractMetadataFoundKey { key: Cow<'a, str> },
    // Dokument,
    Paragraph(Element),
    Skip { tag: Vec<u8> },
}

#[derive(Debug, Clone, PartialEq)]
enum ParsePageState {
    Start,
    // ExtractPage,
    // ExtractMetadataFoundKey { key: Cow<'a, str> },
    // Dokument,
    Paragraph,
    Skip { tag: Vec<u8> },
}

fn process_h1(reader: &mut Reader<&[u8]>, textelem: &mut Element) {
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
}

fn extract_paragraph(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Element {
    let mut elem = Element::bare("p", "");
    let mut curr_node: Option<Node> = None;
    let mut just_seen_span = false;
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle error {:?}", e),
            Ok(Event::Text(text)) => {
                let text = text.unescape().unwrap().to_string();
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
                b"a" | b"A" | b"p" | b"P" | b"notreferens" | b"hanvisning" | b"kant" | b"h4" => {
                    just_seen_span = false
                }
                b"span" | b"SPAN" => just_seen_span = true,

                _ => todo!("handle End({:?})", e),
            },

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"nobr" | b"NOBR" | b"em" | b"EM" | b"sup" => {
                    just_seen_span = false;
                    if let Some(node) = curr_node.take() {
                        elem.append_node(node);
                    }
                    let e = extract_elem(reader, e.name().as_ref());
                    elem.append_child(e);
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
                b"p" | b"P" | b"hanvisning" | b"kant" | b"h4" => (),
                _ => todo!("handle Start({:?})", e),
            },
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"br" | b"BR" => {
                    just_seen_span = false;
                    if let Some(node) = curr_node.take() {
                        elem.append_node(node);
                    }
                    elem.append_child(Element::bare("br", ""));
                }
                b"p" | b"P" | b"a" | b"A" => (),
                _ => todo!("handle Empty({:?})", e),
            },
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    if let Some(node) = curr_node.take() {
        elem.append_node(node);
    }
    elem
}

fn extract_page(reader: &mut Reader<&[u8]>) -> Element {
    let mut elem = Element::bare("page", "");
    let mut curr_child: Option<Element> = None;
    let mut state = ParsePageState::Start;
    let mut div_count = 1;
    loop {
        println!(
            "extract_page: state = {:?}, elem = {:?}, curr_child={:?}",
            state, elem, curr_child
        );
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
                // b"nobr" | b"NOBR" => (),
                b"table" | b"TABLE" => (),
                // b"p" | b"P" => match state {
                //     ParsePageState::Paragraph => state = ParsePageState::Start,
                //     _ => (),
                // },
                // b"span" | b"SPAN" => match state {
                //     ParsePageState::Paragraph => (),
                //     _ => todo!("handle span in state={:?}", state),
                // },
                _ => todo!("handle {:?}", e),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"div" | b"DIV" => {
                    div_count += 1;
                    state = ParsePageState::Skip {
                        tag: e.name().as_ref().to_vec(),
                    }
                }
                b"img" | b"IMG" => (),
                b"table" | b"TABLE" => {
                    let paragraphs = extract_table(reader);
                    if let Some(child) = curr_child.take() {
                        elem.append_child(child);
                    }
                    for p in paragraphs {
                        elem.append_child(p);
                    }
                }
                b"p" | b"P" => {
                    if let Some(child) = curr_child.take() {
                        elem.append_child(child);
                    }
                    curr_child = Some(extract_paragraph(reader, e.name().as_ref()));
                    state = ParsePageState::Paragraph;
                }
                // b"nobr" | b"NOBR" => match state {
                //     ParsePageState::Paragraph => {
                //         let e = extract_elem(reader, e.name().as_ref());
                //         curr_child.as_mut().map(|c| c.append_child(e));
                //     }
                //     _ => (),
                // },
                // b"span" | b"SPAN" => match state {
                //     ParsePageState::Paragraph => (),
                //     _ => todo!("handle span in state={:?}", state),
                // },
                _ => todo!("handle {:?}", e),
            },
            // Ok(Event::Text(text)) => match state {
            //     ParsePageState::Paragraph => {
            //         curr_child.as_mut().map(|c| {
            //             c.append_text_node(text.unescape().expect("valid utf8").to_string())
            //         });
            //     }
            //     _ => (),
            // },
            Ok(Event::Text(_text)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    elem
}

fn extract_page_id_from_attributes(attrs: Attributes) -> Option<String> {
    for attr in attrs {
        let attr = attr.expect("a valid attribute");
        if attr.key.as_ref() == b"id" {
            if let Some(id) = attr.value.strip_prefix(b"page_") {
                return Some(String::from_utf8(id.to_vec()).expect("valid utf8"));
            }
        }
    }
    None
    // if e.attributes().any(|attr| {
    //     let attr = attr.expect("valid attribute");
    //     attr.key.as_ref() == b"id" && attr.value.starts_with(b"page_")
    // })
}

fn extract_elem(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Element {
    let mut elem = Element::bare(String::from_utf8_lossy(tag).to_lowercase(), "");
    loop {
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::Text(text)) => elem.append_text_node(text.unescape().unwrap()),
            Ok(Event::End(e)) => match e.name().as_ref() {
                e_tag if e_tag == tag => break,
                _ => todo!("handle {:?}", e),
            },
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    elem
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

#[derive(Debug, Clone, PartialEq)]
enum ParseTableState {
    Start,
    // ExtractPage,
    // ExtractMetadataFoundKey { key: Cow<'a, str> },
    // Dokument,
    Paragraph,
    Skip { tag: Vec<u8> },
}

fn extract_table(reader: &mut Reader<&[u8]>) -> Vec<Element> {
    let mut table = Vec::new();
    let mut elem = Element::bare("page", "");
    let mut curr_elem: Option<Element> = None;
    let mut state = ParseTableState::Start;
    loop {
        println!(
            "extract_table: state={:?}, table={:?},curr_elem={:?}",
            state, table, curr_elem
        );
        match reader.read_event() {
            Err(e) => todo!("handle err {:?}", e),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"a" | b"A" => match state {
                    ParseTableState::Paragraph => {
                        if let Some(elem) = curr_elem.take() {
                            table.push(elem);
                        }
                        state = ParseTableState::Start;
                    }
                    _ => todo!("handle bad state"),
                },
                // b"div" | b"DIV" => (),
                // b"img" | b"IMG" => (),
                // b"nobr" | b"NOBR" => (),
                b"span" | b"SPAN" => (),
                b"table" | b"TABLE" => break,
                b"tbody" | b"TBODY" => (),
                b"td" | b"TD" => match state {
                    ParseTableState::Paragraph => {
                        if let Some(elem) = curr_elem.take() {
                            table.push(elem);
                        }
                        state = ParseTableState::Start;
                    }
                    ParseTableState::Start => (),
                    _ => todo!("handle bad state"),
                },
                b"tr" | b"TR" => (),
                b"thead" => (),
                // b"p" | b"P" => match state {
                //     ParseTableState::Paragraph => state = ParsePageState::Start,
                //     _ => (),
                // },
                _ => todo!("handle End({:?})", e),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"a" | b"A" => {
                    // if let Some(elem) = curr_elem.take() {
                    //     table.push(elem);
                    // }
                    let mut p = extract_paragraph(reader, e.name().as_ref());
                    if let Some(href) = extract_href_from_attributes(e.attributes()) {
                        p.set_attr("link", href);
                    }
                    table.push(p);
                    // curr_elem = Some(p);
                    // state = ParseTableState::Paragraph;
                }
                b"tbody" | b"TBODY" => (),
                b"thead" => (),
                b"tr" | b"TR" => (),
                b"td" | b"TD" | b"th" | b"TH" => {
                    // if let Some(elem) = curr_elem.take() {
                    //     table.push(elem);
                    // }
                    // curr_elem = Some(Element::bare("p", ""));
                    table.push(extract_paragraph(reader, e.name().as_ref()));
                    // state = ParseTableState::Paragraph;
                }
                b"span" | b"SPAN" => (),
                // b"img" | b"IMG" => (),
                // b"table" | b"TABLE" => {
                //     let paragraphs = extract_table(reader);
                // }
                // b"p" | b"P" => {
                //     if let Some(child) = curr_child.take() {
                //         elem.append_child(child);
                //     }
                //     curr_child = Some(Element::bare("p", ""));
                //     state = ParsePageState::Paragraph;
                // }
                // b"nobr" | b"NOBR" => (),
                _ => todo!("handle Start({:?})", e),
            },
            Ok(Event::Text(text)) => match state {
                ParseTableState::Paragraph => {
                    curr_elem
                        .as_mut()
                        .map(|e| e.append_text_node(text.unescape().unwrap()));
                }
                ParseTableState::Start => (),
                _ => todo!(
                    "handle text='{}' state={:?}",
                    text.unescape().unwrap().as_ref(),
                    state
                ),
            },
            Ok(Event::Empty(_e)) => (),
            Ok(e) => todo!("handle {:?}", e),
        }
    }
    table
}
