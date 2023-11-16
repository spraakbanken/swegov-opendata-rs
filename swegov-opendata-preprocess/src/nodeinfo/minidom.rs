use minidom::Node;

pub fn minidom_collect_texts(elem: &minidom::Element) -> String {
    let mut text = String::new();
    for child in elem.nodes() {
        match child {
            Node::Element(c_elem) => {
                let c_text = minidom_collect_texts(c_elem);
                text.push_str(&c_text);
            }
            Node::Text(contents) => text.push_str(&contents),
        }
    }
    text
}

pub fn minidom_text_len(node: &minidom::Node) -> usize {
    match &node {
        minidom::Node::Text(contents) => {
            // eprintln!("Text '{:?}'", contents);
            contents.len()
        }
        minidom::Node::Element(elem) => {
            // eprintln!("Element <{}>", elem.name());
            elem.nodes().map(|node| minidom_text_len(node)).sum()
        }
        _ => 0,
    }
}

pub fn elem_is_empty(elem: &minidom::Element) -> bool {
    // if elem.attrs().count() == 0 && elem.nodes().len() == 0 {
    //     return true;
    // }
    for node in elem.nodes() {
        match node {
            minidom::Node::Element(c_elem) => {
                if !elem_is_empty(c_elem) {
                    return false;
                }
            }
            minidom::Node::Text(contents) => {
                if !contents.is_empty() {
                    return false;
                }
            }
        }
    }
    true
}

pub mod asserts {
    use std::collections::BTreeMap;

    use itertools::{EitherOrBoth, Itertools};
    use minidom::{Element, Node};
    use pretty_assertions::assert_eq;

    use crate::preprocess::xml::clean_text;

    pub fn assert_node_equal(left: &Node, right: &Node) {
        match (left, right) {
            (Node::Text(left_text), Node::Text(right_text)) => {
                let mut left_text = left_text.replace('\n', " ");
                clean_text(&mut left_text);
                let mut right_text = right_text.replace('\n', " ");
                clean_text(&mut right_text);
                assert_eq!(&left_text, &right_text);
            }
            (Node::Element(left_elem), Node::Element(right_elem)) => {
                assert_elem_equal(left_elem, right_elem)
            }
            (Node::Text(left_text), Node::Element(right_elem)) => {
                dbg!(right_elem);
                panic!("left is Text({:?}) and right is an Element", left_text)
            }
            (Node::Element(left_elem), Node::Text(right_text)) => {
                dbg!(left_elem);
                panic!("right is Text({:?}) and left is an Element", right_text)
            }
        }
    }
    pub fn assert_elem_equal(left: &Element, right: &Element) {
        dbg!(left, right);

        assert_eq!(
            left.name(),
            right.name(),
            "tag of {:#?} and {:#?}",
            left,
            right
        );
        assert_eq!(left.ns(), right.ns());
        let left_attrs: BTreeMap<&str, &str> = left.attrs().collect();
        let right_attrs: BTreeMap<&str, &str> = right.attrs().collect();
        assert_eq!(left_attrs, right_attrs);

        //     "we are testing attrs of {:#?} and {:#?}",
        //     left,
        //     right
        // );

        // assert_eq!(
        //     left.nodes().len(),
        //     right.nodes().len(),
        //     "len of {:#?} and {:#?}",
        //     left,
        //     right
        // );

        for (i, val) in left
            .nodes()
            .filter(|node| match node {
                Node::Element(_) => true,
                Node::Text(c) => !c.trim().is_empty(),
            })
            .zip_longest(right.nodes().filter(|node| match node {
                Node::Element(_) => true,
                Node::Text(c) => !c.trim().is_empty(),
            }))
            .enumerate()
        {
            dbg!(i);
            match val {
                EitherOrBoth::Left(l1) => {
                    panic!("Left contains more nodes: among them node[{i}]= {:#?}", l1)
                }
                EitherOrBoth::Right(r1) => {
                    panic!("Right contains more nodes: among them node[{i}]= {:#?}, left={:#?}, right={:#?}", r1, left, right)
                }
                EitherOrBoth::Both(l1, r1) => assert_node_equal(l1, r1),
            }
        }
    }
}
