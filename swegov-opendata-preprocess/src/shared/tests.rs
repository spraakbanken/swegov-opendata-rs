use super::*;

use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case("1 §  Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av
egnahemsfastighet som har", "1 § Enligt denna förordning får lån (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har")]
#[case(
    "Av 6 kap. 1 § jordabalken
följer
  att registrering av en inteckning i pantbrevsregistret innebär att ett datapantbrev
utfärdas.
  Lag (2008:546).",
    "Av 6 kap. 1 § jordabalken följer att registrering av en inteckning i pantbrevsregistret innebär att ett datapantbrev utfärdas. Lag (2008:546)."
)]
fn test_clean_text2(#[case] given: &str, #[case] expected: &str) {
    assert_eq!(clean_text(given), expected)
}

#[rstest]
#[case("example")]
#[case("valid space")]
fn clean_text_doesnt_touch_cleaned(#[case] given: &str) {
    let expected = given;
    assert_eq!(clean_text(given), expected);
}

#[rstest]
// #[case("example")]
#[case(" initial space", "initial space")]
#[case("  initial space", "initial space")]
#[case("invalid   space", "invalid space")]
#[case("invalid  space", "invalid space")]
#[case("invalid\nspace", "invalid space")]
#[case("invalid\u{a0}space", "invalid space")]
#[case("trailing space ", "trailing space")]
#[case("trailing space  ", "trailing space")]
#[case("  \u{a0}\u{a0}\u{a0}   ", "")]
fn clean_text_cleans_text(#[case] given: &str, #[case] expected: &str) {
    assert_eq!(clean_text(given), expected);
}
