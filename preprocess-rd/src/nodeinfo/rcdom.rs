use html5ever::rcdom::{self, NodeData};

pub fn dbg_rcdom_node(node: &rcdom::Handle) -> String {
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            let attrs_str: Vec<String> = if attrs.borrow().len() > 0 {
                attrs
                    .borrow()
                    .iter()
                    .map(|a| format!("{}='{}'", a.name.local, a.value))
                    .collect()
            } else {
                vec![]
            };
            format!(
                "Element {{ name = {}, attrs = {:?} }}",
                name.local, attrs_str
            )
        }
        NodeData::Text { contents } => {
            format!("Text '{:?}'", contents)
        }
        NodeData::Comment { contents } => {
            format!("Comment '{:?}'", contents)
        }
        _ => todo!("handle"),
    }
}

pub fn rcdom_collect_texts(
    node_iter: impl Iterator<Item = rcdom::Handle>,
    separator: &str,
) -> String {
    let mut text_contents = String::new();

    for child in node_iter {
        match &child.data {
            NodeData::Text { contents } => {
                if !text_contents.is_empty() {
                    text_contents.push_str(separator);
                }
                text_contents.push_str(contents.borrow().as_ref());
            }
            _ => (),
        }
    }
    text_contents
}
pub fn rcdom_text_len(node: &rcdom::Handle) -> usize {
    match &node.data {
        NodeData::Text { contents } => {
            // eprintln!("Text '{:?}'", contents);
            contents.borrow().len()
        }
        NodeData::Element {
            name: _,
            attrs: _,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            // eprintln!("Element <{}>", name.local);
            node.children
                .borrow()
                .iter()
                .map(|node| rcdom_text_len(node))
                .sum()
        }
        _ => 0,
    }
}
