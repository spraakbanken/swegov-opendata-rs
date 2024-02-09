use super::*;

mod clean {
    use minidom_extension::asserts::assert_elem_equal;

    use crate::shared::clean_text;

    use super::*;
    use minidom::Element;
    use rstest::rstest;

    #[rstest]
    #[case("example")]
    #[case("valid space")]
    fn clean_text_doesnt_touch_cleaned(#[case] given: &str) {
        let mut text: String = given.into();
        let expected = text.clone();
        clean_text(&mut text);
        assert_eq!(text, expected);
    }

    #[rstest]
    // #[case("example")]
    #[case(" initial space", "initial space")]
    #[case("  initial space", "initial space")]
    #[case("invalid   space", "invalid space")]
    #[case("invalid  space", "invalid space")]
    #[case("trailing space ", "trailing space")]
    #[case("trailing space  ", "trailing space")]
    #[case("  \u{a0}\u{a0}\u{a0}   ", "")]
    fn clean_text_cleans_text(#[case] given: &str, #[case] expected: &str) {
        let mut text: String = given.into();
        clean_text(&mut text);
        assert_eq!(text, expected);
    }

    #[rstest]
    #[case(r#"<p xmlns="">hi<p></p></p>"#, Some(r#"<p xmlns="">hi</p>"#))]
    #[case(r#"<p xmlns=""> <p></p></p>"#, None)]
    #[case(r#"<p xmlns="">hi<p> </p></p>"#, Some(r#"<p xmlns="">hi</p>"#))]
    #[case(r#"<p xmlns="">hi<p> <p> </p></p></p>"#, Some(r#"<p xmlns="">hi</p>"#))]
    #[case(
        r#"<p xmlns="">this is<p> a <p>sentence</p></p></p>"#,
        Some(r#"<p xmlns="">this is a sentence</p>"#)
    )]
    #[case(
        r#"<p xmlns="">
                <b><span>Civilutskottets betänkanden nr 13 år 1971</span>
                </b><b><span>    </span></b><b><span>CU 1971</span></b></p>"#,
        Some(r#"<p xmlns="">Civilutskottets betänkanden nr 13 år 1971 CU 1971</p>"#)
    )]
    #[case(
        r#"<text xmlns=""><div>
                <p>  Civilutskottets betänkanden nr 13 år 1971  </p>
                </div></text>"#,
        Some(r#"<text xmlns=""><p>Civilutskottets betänkanden nr 13 år 1971</p></text>"#)
    )]
    fn clean_element_cleans(#[case] given: &str, #[case] expected: Option<&str>) {
        let elem: Element = given.parse().unwrap();
        let expected: Option<Element> = expected.map(|e| e.parse().unwrap());

        let cleaned = clean_element(&elem);
        assert_eq!(
            cleaned.is_some(),
            expected.is_some(),
            "{:?} != {:?}",
            cleaned,
            expected
        );
        if expected.is_some() {
            assert_elem_equal(cleaned.as_ref().unwrap(), expected.as_ref().unwrap());
        }
    }
}
