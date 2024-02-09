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
