use minidom_extension::minidom::Element;

use super::{process_html, remove_cdata};

#[test]
fn test_process_html_from_string() {
    let html = "Riksdagen<br>a) antar lag om sprängämnesprekursorer<br>b) antar lag om ändring i lagen (1996:701) om Tullverkets befogenheter vid Sveriges gräns mot ett annat land inom Europeiska unionen.<br>\r\n";

    let mut textelem = Element::bare("text", "");
    process_html(&html, &mut textelem);

    insta::assert_debug_snapshot!(textelem);
}

#[test]
fn test_remove_cdata() {
    let text = "<P style=\"margin-left:1px;margin-top:12px;margin-right:-25px;margin-bottom:12px;\" class=\"p410 ft90\"><![endif]>No</P></TD>";

    assert_eq!(remove_cdata(text), "<P style=\"margin-left:1px;margin-top:12px;margin-right:-25px;margin-bottom:12px;\" class=\"p410 ft90\">No</P></TD>");
}
