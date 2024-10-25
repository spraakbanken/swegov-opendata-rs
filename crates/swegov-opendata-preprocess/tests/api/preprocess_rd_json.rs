use std::fs;

use minidom_extension::minidom::Element;
use rstest::rstest;
use swegov_opendata::{DataSet, DokumentStatusPageRef, DokumentStatusRef};
use swegov_opendata_preprocess::{preprocess_rd, shared::io_ext};

#[rstest]
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
#[case("assets/Riksdagens diarium-2014-2017-h5d2467.json")]
#[case("assets/Skriftliga frågor-1990-1997-gl11103.json")]
#[case("assets/Skriftliga frågor-1990-1997-gl12229.json")]
#[case("assets/utskottsdokument-1998-2001-gma1cc3.json")]
#[case("assets/utskottsdokument-2002-2005-gta1aureg.json")]
#[case("assets/yttr-2018-2021-h605au1y.json")]
#[case("assets/Övrigt-2014-2017-h4d1amt.json")]
#[case("assets/Övrigt-2014-2017-h50n48f9da.json")]
fn preprocess_rd_json(#[case] filename: &str) -> anyhow::Result<()> {
    let metadata_path = format!("{}.metadata.json", filename.rsplit_once('-').unwrap().0);
    println!("reading test data from '{}'", filename);
    let file_data = fs::read_to_string(filename)?;
    println!("reading metadata from '{}'", metadata_path);
    let metadata_data = match fs::read_to_string(&metadata_path) {
        Ok(data) => data,
        Err(err) => {
            println!("Error reading from '{}': {:?}", metadata_path, err);
            let new_metadata_path = format!(
                "{}.metadata.json",
                metadata_path.rsplit_once('-').unwrap().0
            );
            println!("reading metadata from '{}'", new_metadata_path);
            fs::read_to_string(new_metadata_path)?
        }
    };
    let metadata: DataSet = serde_json::from_str(&metadata_data)?;

    let xmlstring = preprocess_rd::preprocess_json(&file_data, &metadata)?;
    let xmlstring = String::from_utf8(xmlstring)?;
    insta::assert_snapshot!(filename, xmlstring);

    Ok(())
}

#[rstest]
#[case("assets/bet-1998-2001-gp01bou1.json")]
#[case("assets/bet-2010-2013-h101föu11.json")]
fn preprocess_rd_html(#[case] filename: &str) -> anyhow::Result<()> {
    println!("reading test data from '{}'", filename);
    let source = fs::read_to_string(filename)?;
    let source = io_ext::without_bom(&source);
    let DokumentStatusPageRef {
        dokumentstatus: DokumentStatusRef { dokument, .. },
    } = serde_json::from_str(source)?;

    let mut textelem = Element::bare("text", "");
    preprocess_rd::process_html(dokument.html().expect("valid html"), &mut textelem);

    insta::assert_debug_snapshot!(format!("html-{}", filename), textelem);
    Ok(())
}
