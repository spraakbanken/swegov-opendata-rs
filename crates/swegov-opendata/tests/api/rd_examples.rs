use std::fs;

use anyhow::Context;
use rstest::rstest;

use swegov_opendata::DokumentStatusPage;

#[rstest]
#[case("assets/prot-ge091.json")]
#[case("assets/mot-gy02a1.json")]
#[case("assets/mot-h102xs24006.json")]
#[case("assets/prop-2018-2021-h603100.json")]
fn rd_example(#[case] filename: &str) -> anyhow::Result<()> {
    let source = fs::read_to_string(filename)?;
    let source = without_bom(&source);
    dbg!(&source);
    let map: serde_json::Value =
        serde_json::from_str(&source).with_context(|| "Failed deserialize to serde_json::Value")?;
    dbg!(&map);
    let dokumentstatus: DokumentStatusPage = serde_json::from_str(&source)
        .with_context(|| "Failed deserialize to DokumentStatusPage")?;
    dbg!(&dokumentstatus);
    Ok(())
}

pub fn without_bom(s: &str) -> &str {
    if &s[0..3] == "\u{feff}" {
        &s[3..]
    } else {
        &s[..]
    }
}
