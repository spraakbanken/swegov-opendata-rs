use itertools::Itertools;
use minidom::{Element, Node};
use minidom_extension::{elem_is_empty, minidom};
use once_cell::sync::Lazy;
use regex::Regex;

pub mod io_ext;

pub fn clean_element(elem: &Element) -> Element {
    // let new_elem = elem.clone();
    let mut elem_builder = Element::builder(elem.name(), elem.ns());
    for (name, value) in elem.attrs() {
        elem_builder = elem_builder.attr(name, value);
    }
    let mut new_elem = elem_builder.build();
    clean_nodes(&mut new_elem, elem);
    // if elem.name() == "p" {
    //     // let mut text = minidom_collect_texts(elem);
    //     clean_text(&mut text);
    //     if text.is_empty() {
    //         return None;
    //     } else {
    //         elem_builder = elem_builder.append(minidom::Node::Text(text));
    //     }
    // }
    // println!("clean_element: new_elem={:#?}", new_elem);
    new_elem
}

pub fn clean_texts(_elem: &mut Element) {}

fn clean_nodes(new_elem: &mut Element, elem: &Element) {
    for node in elem.nodes() {
        match node {
            Node::Text(contents) => {
                let mut text = contents.clone();
                text = clean_text(&text);
                if !text.is_empty() {
                    if new_elem.nodes().len() > 0 {
                        new_elem.append_text_node(" ".to_string());
                    }
                    new_elem.append_text_node(text);
                }
                // dbg!(&new_elem);
            }
            Node::Element(c_elem) => {
                let mut elem_builder = Element::builder(c_elem.name(), c_elem.ns());
                for (name, value) in c_elem.attrs() {
                    elem_builder = elem_builder.attr(name, value);
                }
                let mut new_child_elem = elem_builder.build();
                clean_nodes(&mut new_child_elem, c_elem);
                if new_child_elem.name() == "br" || !elem_is_empty(&new_child_elem) {
                    new_elem.append_child(new_child_elem);
                }
            } //todo!("handle {:?}", c_elem),
        }
    }
}

pub fn clean_text(text: &str) -> String {
    let text = text.replace('\u{AD}', "");
    Itertools::intersperse(text.split_whitespace(), " ").collect()
    // text.split_whitespace()
    //     // .split(char::is_whitespace)
    //     .intersperse(" ")
    //     // .filter(|part| !part.trim().is_empty())
    //     .collect()
}

pub fn is_segreg(s: &str) -> bool {
    static SEGREG: Lazy<Regex> = Lazy::new(|| Regex::new(r"[Ss][Ee][Gg][Rr][Ee][Gg]").unwrap());
    SEGREG.is_match(s)
}

#[cfg(test)]
mod tests;
