use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_process_html() -> error_stack::Result<(), SfsPreprocessError> {
    let src = "<div><p><a name=\"S1\"></a></p><a class=\"paragraf\" name=\"P1\"><b>1 §</b></a>   Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har<br />\r\n   1. inlösts enligt 56 a § arbetsmarknadskungörelsen (1966:368),<br />\r\n   2. avstyckats från jordbruksfastighet genom åtgärder i samband med jordbrukets rationalisering.<h3 name=\"overgang\"><a name=\"overgang\">Övergångsbestämmelser</a></h3>\r\n1985:458<p><a name=\"P11S2\"></a></p>\r\n\r\nDenna förordning träder i kraft den 1 juli 1985.</div>";
    let mut actual_elem = Element::bare("text", "");
    process_html(src, &mut actual_elem)?;
    let actual_elem = clean_element(&actual_elem);
    let mut actual = Vec::new();
    actual_elem
        .write_to(&mut actual)
        .change_context_lazy(|| SfsPreprocessError::Internal("writing actual".to_string()))?;
    let expected = r#"<text xmlns=""><p>1 § Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har<br/> 1. inlösts enligt 56 a § arbetsmarknadskungörelsen (1966:368),<br/> 2. avstyckats från jordbruksfastighet genom åtgärder i samband med jordbrukets rationalisering.</p><p>Övergångsbestämmelser</p><p>1985:458</p><p>Denna förordning träder i kraft den 1 juli 1985.</p></text>"#;
    let actual_str = String::from_utf8_lossy(&actual);
    assert_eq!(actual_str, expected);
    Ok(())
}
