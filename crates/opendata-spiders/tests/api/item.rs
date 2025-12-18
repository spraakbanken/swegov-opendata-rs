use fs_err as fs;

use opendata_spiders::item::Item;
use rstest::rstest;

#[rstest]
#[case::dokumentlista("assets/dokumentlista.xml")]
#[case::dokumentstatus("assets/dokumentstatus.xml")]
fn item_deserialize(#[case] filename: &str) -> anyhow::Result<()> {
    let source = fs::read_to_string(filename)?;
    let item: Item = yaserde::de::from_str(&source).unwrap();
    assert!(matches!(
        item,
        Item::DokumentLista(_) | Item::DokumentStatus(_)
    ));
    Ok(())
}
