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

#[cfg(test)]
pub mod asserts {
    use itertools::{EitherOrBoth, Itertools};
    use minidom::{Element, Node};

    pub fn assert_node_equal(left: &Node, right: &Node) {
        match (left, right) {
            (Node::Text(left_text), Node::Text(right_text)) => assert_eq!(left_text, right_text),
            (Node::Element(left_elem), Node::Element(right_elem)) => {
                assert_elem_equal(left_elem, right_elem)
            }
            (l, r) => assert_eq!(l, r),
        }
    }
    pub fn assert_elem_equal(left: &Element, right: &Element) {
        dbg!(left, right);

        assert_eq!(left.name(), right.name());
        assert_eq!(left.ns(), right.ns());
        assert!(left.attrs().eq(right.attrs()));

        for cmp in left.nodes().zip_longest(right.nodes()) {
            match cmp {
                EitherOrBoth::Both(node1, node2) => assert_node_equal(node1, node2),
                EitherOrBoth::Left(node1) => {
                    dbg!(node1);

                    panic!("left is longer '{:?}'", node1)
                }
                EitherOrBoth::Right(node2) => {
                    dbg!(node2);
                    panic!("right is longer '{:?}'", node2)
                }
            }
        }
        // assert!(left
        //     .nodes()
        //     .zip_longest(right.nodes())
        //     .all(|either_or_both| {
        //         match either_or_both {
        //             itertools::EitherOrBoth::Both(node1, node2) => node1 == node2,
        //             _ => false,
        //         }
        //     }));
    }
}
