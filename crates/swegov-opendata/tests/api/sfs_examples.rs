use fs_err as fs;

use rstest::rstest;
use swegov_opendata::{DokumentLista, DokumentStatus};

#[rstest]
#[case("assets/dokumentlista_20251216.xml")]
fn sfs_dokumentlista(#[case] filename: &str) -> anyhow::Result<()> {
    dbg!(filename);
    let source = fs::read_to_string(filename)?;

    let dokumentlista: DokumentLista = quick_xml::de::from_str(&source)?;
    dbg!(&dokumentlista);
    Ok(())
}
#[rstest]
#[case("assets/dokumentlista_20251216.xml")]
fn sfs_dokumentlista_yaserde(#[case] filename: &str) -> anyhow::Result<()> {
    dbg!(filename);
    let source = fs::read_to_string(filename)?;

    let dokumentlista: DokumentLista = yaserde::de::from_str(&source).unwrap();
    dbg!(&dokumentlista);
    Ok(())
}

#[rstest]
#[case("assets/dokumentstatus.xml")]
fn sfs_dokumentstatus_yaserde(#[case] filename: &str) -> anyhow::Result<()> {
    dbg!(filename);
    let source = fs::read_to_string(filename)?;

    let dokumentlista: DokumentStatus = yaserde::de::from_str(&source).unwrap();
    dbg!(&dokumentlista);
    Ok(())
}
