use minidom_extension::minidom::Element;

use super::process_html;

#[test]
fn test_process_html_from_string() {
    let html = "Riksdagen<br>a) antar lag om sprängämnesprekursorer<br>b) antar lag om ändring i lagen (1996:701) om Tullverkets befogenheter vid Sveriges gräns mot ett annat land inom Europeiska unionen.<br>\r\n";

    let mut textelem = Element::bare("text", "");
    process_html(&html, &mut textelem);

    insta::assert_debug_snapshot!(textelem);
}
