use fs_err as fs;

use anyhow::Context;
use rstest::rstest;

use swegov_opendata::{DatasetLista, DokumentStatusPage, DokumentStatusPageRef};

#[rstest]
#[case("assets/Riksdagens diarium-2014-2017-h5d2467.json")]
#[case("assets/bet-1998-2001-gp01bou1.json")]
#[case("assets/bet-2006-2009-gu01au1.json")]
#[case("assets/bet-2006-2009-gw01fiu3.json")]
#[case("assets/bet-2010-2013-gy01au1.json")]
#[case("assets/bet-2010-2013-h101föu11.json")]
#[case("assets/bet-2018-2021-h601au1.json")]
#[case("assets/bet-2018-2021-h601au6.json")]
#[case("assets/bet-2022-2025-ha01au6.json")]
#[case("assets/frsrdg-2018-2021-h604nr1.json")]
#[case("assets/ip-2006-2009-gu10110.json")]
#[case("assets/ip-2006-2009-gu10379.json")]
#[case("assets/ip-2014-2017-h210566.json")]
#[case("assets/ip-2014-2017-h210692.json")]
#[case("assets/ip-2014-2017-h410591.json")]
#[case("assets/kammakt-2018-2021-h6c120190118zz.json")]
#[case("assets/kammakt-2018-2021-h9c120220419bu.json")]
#[case("assets/kom-2010-2014-h2b643.json")]
#[case("assets/kom-2020--h8b6447.json")]
#[case("assets/mot-1971-1979-g3021833.json")]
#[case("assets/mot-1998-2001-gm02bo208.json")]
#[case("assets/mot-2010-2013-gy02a1.json")]
#[case("assets/mot-2010-2013-gy02a245.json")]
#[case("assets/mot-2010-2013-gy02x-s68106.json")]
#[case("assets/mot-2010-2013-h102xs24006.json")]
#[case("assets/mot-2014-2017-h2021148.json")]
#[case("assets/prop-2014-2017-h203100.json")]
#[case("assets/prop-2018-2021-h603100.json")]
#[case("assets/prot-1990-1997-ge091.json")]
#[case("assets/utskottsdokument-2002-2005-gta1aureg.json")]
#[case("assets/yttr-2018-2021-h605au1y.json")]
fn rd_example(#[case] filename: &str) -> anyhow::Result<()> {
    dbg!(filename);
    let source = fs::read_to_string(filename)?;
    let source = without_bom(&source);

    let dokumentstatus: DokumentStatusPage = serde_json::from_str(&source)
        .with_context(|| "Failed deserialize to DokumentStatusPage")?;
    dbg!(&dokumentstatus);
    Ok(())
}

#[rstest]
#[case("assets/Riksdagens diarium-2014-2017-h5d2467.json")]
#[case("assets/bet-1998-2001-gp01bou1.json")]
#[case("assets/bet-2006-2009-gu01au1.json")]
#[case("assets/bet-2006-2009-gw01fiu3.json")]
#[case("assets/bet-2010-2013-gy01au1.json")]
#[case("assets/bet-2010-2013-h101föu11.json")]
#[case("assets/bet-2018-2021-h601au1.json")]
#[case("assets/bet-2018-2021-h601au6.json")]
#[case("assets/bet-2022-2025-ha01au6.json")]
#[case("assets/frsrdg-2018-2021-h604nr1.json")]
#[case("assets/ip-2006-2009-gu10110.json")]
#[case("assets/ip-2006-2009-gu10379.json")]
#[case("assets/ip-2014-2017-h210566.json")]
#[case("assets/ip-2014-2017-h210692.json")]
#[case("assets/ip-2014-2017-h410591.json")]
#[case("assets/kammakt-2018-2021-h6c120190118zz.json")]
#[case("assets/kammakt-2018-2021-h9c120220419bu.json")]
#[case("assets/kom-2010-2014-h2b643.json")]
#[case("assets/kom-2020--h8b6447.json")]
#[case("assets/mot-1971-1979-g3021833.json")]
#[case("assets/mot-1998-2001-gm02bo208.json")]
#[case("assets/mot-2010-2013-gy02a1.json")]
#[case("assets/mot-2010-2013-gy02a245.json")]
#[case("assets/mot-2010-2013-gy02x-s68106.json")]
#[case("assets/mot-2010-2013-h102xs24006.json")]
#[case("assets/mot-2014-2017-h2021148.json")]
#[case("assets/prop-2014-2017-h203100.json")]
#[case("assets/prop-2018-2021-h603100.json")]
#[case("assets/prot-1990-1997-ge091.json")]
#[case("assets/utskottsdokument-2002-2005-gta1aureg.json")]
#[case("assets/yttr-2018-2021-h605au1y.json")]
fn rd_example_ref(#[case] filename: &str) -> anyhow::Result<()> {
    dbg!(filename);
    let source = fs::read_to_string(filename)?;
    let source = without_bom(&source);

    let dokumentstatus: DokumentStatusPageRef<'_> = serde_json::from_str(&source)
        .with_context(|| "Failed deserialize to DokumentStatusPage")?;
    dbg!(&dokumentstatus);
    Ok(())
}

#[rstest]
#[case("assets/datasetlista.xml")]
fn rd_datasetlista(#[case] filename: &str) -> anyhow::Result<()> {
    let source = fs::read_to_string(filename)?;

    let datasetlista: DatasetLista = yaserde::de::from_str(&source).unwrap();
    dbg!(&datasetlista);
    Ok(())
}

pub fn without_bom(s: &str) -> &str {
    if &s[0..3] == "\u{feff}" {
        &s[3..]
    } else {
        s
    }
}
