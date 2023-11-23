use minidom::{Element, ElementBuilder, Node};
use minidom_extension::{elem_is_empty, minidom};

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

pub fn clean_texts(elem: &mut Element) {}

fn clean_nodes(new_elem: &mut Element, elem: &Element) {
    for node in elem.nodes() {
        match node {
            Node::Text(contents) => {
                let mut text = contents.clone();
                clean_text(&mut text);
                if !text.is_empty() {
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
const SPACES: [char; 2] = [' ', '\u{00A0}'];
pub fn clean_text(text: &mut String) {
    text.truncate(text.trim_end().len());
    // dbg!(&text);
    *text = text.replace('\u{00A0}', " ");
    let mut prev = 'x';
    text.retain(|ch| {
        if ['\n', '\u{00AD}'].contains(&ch) {
            return false;
        }
        let consecutive_space = ch == ' ' && prev == ' ';
        prev = ch;
        !consecutive_space
    });
    // dbg!(&text);
    // if text.is_empty() && orig_len > 0 {
    //     *text = " ".into();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let mut s =
            "1 §  Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av
        egnahemsfastighet som har"
                .into();
        clean_text(&mut s);
        assert_eq!(
            s,
            "1 § Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har"
        )
    }
}
