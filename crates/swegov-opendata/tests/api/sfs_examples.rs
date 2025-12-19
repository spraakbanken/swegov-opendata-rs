use fs_err as fs;

use rstest::rstest;
use swegov_opendata::{DokumentLista, DokumentStatus};

#[rstest]
#[case("assets/dokumentlista.xml")]
#[case::dokumentlista_1901_1920("assets/dokumentlista_1901_1920.xml")]
#[case::dokumentlista_1961_1980("assets/dokumentlista_1961_1980.xml")]
fn sfs_dokumentlista(#[case] filename: &str) -> anyhow::Result<()> {
    let source = fs::read_to_string(filename)?;

    let dokumentlista: DokumentLista = yaserde::de::from_str(&source).unwrap();
    dbg!(&dokumentlista);
    Ok(())
}

#[rstest]
#[case("assets/dokumentstatus.xml")]
fn sfs_dokumentstatus(#[case] filename: &str) -> anyhow::Result<()> {
    let source = fs::read_to_string(filename)?;

    let dokumentlista: DokumentStatus = yaserde::de::from_str(&source).unwrap();
    dbg!(&dokumentlista);
    Ok(())
}
