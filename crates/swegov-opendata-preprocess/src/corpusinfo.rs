pub fn corpusinfo(prefix: &str) -> Result<&CorpusInfo, UnknownCorpus> {
    CorpusInfo::from_prefix(prefix)
}

#[derive(Debug, thiserror::Error)]
#[error("Unknown corpus '{0}'")]
pub struct UnknownCorpus(String);

pub struct CorpusInfo {
    pub id: &'static str,
    pub names: &'static [(&'static str, &'static str)],
    pub descriptions: &'static [(&'static str, &'static str)],
    pub doi: Option<&'static str>,
}

impl CorpusInfo {
    pub fn from_prefix(prefix: &str) -> Result<&CorpusInfo, UnknownCorpus> {
        for (corpus_prefix, corpus_info) in CORPUSINFO {
            if *corpus_prefix == prefix {
                return Ok(corpus_info);
            }
        }
        Err(UnknownCorpus(prefix.to_string()))
    }
    pub fn from_id(id: &str) -> Result<&CorpusInfo, UnknownCorpus> {
        for (_corpus_prefix, corpus_info) in CORPUSINFO {
            if corpus_info.id == id {
                return Ok(corpus_info);
            }
        }
        Err(UnknownCorpus(id.to_string()))
    }
}

const CORPUSINFO: &[(&str, CorpusInfo)] = &[
    (
        "bet",
        CorpusInfo {
            id: "rd-bet",
            names: &[
                ("swe", "Riksdagens öppna data: Betänkande"),
                ("eng", "Riksdag's open data: Committee reports and statements")
            ],
            descriptions: &[
                ("swe", "Utskottens betänkanden och utlåtanden, inklusive riksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet")
            ],
            doi: Some("10.23695/k819-cs56"),
        }
    ),
    (
        "segreg-bet",
        CorpusInfo {
            id: "segreg-rd-bet",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Betänkande"),
                ("eng", "Segregation texts: Riksdag's open data: Committee reports and statements")
            ],
            descriptions: &[
                ("swe", "Utskottens betänkanden och utlåtanden, inklusive riksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet")
            ],
            doi: Some("10.23695/fw90-xg54"),
        }
    ),
    (
        "ds",
        CorpusInfo {
            id: "rd-ds",
            names: &[
                ("swe", "Riksdagens öppna data: Departementsserien"),
                ("eng", "Riksdag's open data: Ministry Publications Series")
            ],
            descriptions: &[
                ("swe", "Utredningar från regeringens departement")
            ],
            doi: Some("10.23695/yc3t-mg58"),
        }
    ),
    (
        "segreg-ds",
        CorpusInfo {
            id: "segreg-rd-ds",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Departementsserien"),
                ("eng", "Segregation texts: Riksdag's open data: Ministry Publications Series")
            ],
            descriptions: &[
                ("swe", "Utredningar från regeringens departement")
            ],
            doi: Some("10.23695/a9be-2d31"),
        }
    ),
    (
        "EUN",
        CorpusInfo {
            id: "rd-eun",
            names: &[
                ("swe", "Riksdagens öppna data: EUN"),
                ("eng", "Riksdag's open data: Committee on EU Affairs")
            ],
            descriptions: &[
                ("swe", "Dokument från EU-nämnden, bland annat möteskallelser, föredragningslistor, protokoll och skriftliga samråd med regeringen"),
                ("eng", "Documents from the Committee on EU Affairs")
            ],
            doi: Some("10.23695/rgq0-pr79"),
        }
    ),
    (
        "segreg-EUN",
        CorpusInfo {
            id: "segreg-rd-eun",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: EUN"),
                ("eng", "Segregation texts: Riksdag's open data: Committee on EU Affairs")
            ],
            descriptions: &[
                ("swe", "Dokument från EU-nämnden, bland annat möteskallelser, föredragningslistor, protokoll och skriftliga samråd med regeringen"),
                ("eng", "Documents from the Committee on EU Affairs")
            ],
            doi: Some("hz4b-6h68"),
        }
    ),
    (
        "f-lista",
        CorpusInfo {
            id: "rd-flista",
            names: &[
                ("swe", "Riksdagens öppna data: Föredragningslista"),
                ("eng", "Riksdag's open data: Order papers")
            ],
            descriptions: &[
                ("swe", "Föredragningslistor för kammarens sammanträden")
            ],
            doi: Some("10.23695/mzj1-8e48"),
        }
    ),
    (
        "segreg-f-lista",
        CorpusInfo {
            id: "segreg-rd-flista",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Föredragningslista"),
                ("eng", "Segregation texts: Riksdag's open data: Order papers")
            ],
            descriptions: &[
                ("swe", "Föredragningslistor för kammarens sammanträden")
            ],
            doi: Some("10.23695/mkaa-z088"),
        }
    ),
    (
        "fpm",
        CorpusInfo {
            id: "rd-fpm",
            names: &[
                ("swe", "Riksdagens öppna data: Faktapromemoria"),
                ("eng", "Riksdag's open data: Explanatory memorandums on EU proposals")],
            descriptions: &[
                ("swe", "Regeringens faktapromemorior om EU-kommissionens förslag")
            ],
            doi: Some("10.23695/8sa3-ma31"),
        }
    ),
    (
        "segreg-fpm",
        CorpusInfo {
            id: "segreg-rd-fpm",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Faktapromemoria"),
                ("eng", "Segregation texts: Riksdag's open data: Explanatory memorandums on EU proposals")],
            descriptions: &[
                ("swe", "Regeringens faktapromemorior om EU-kommissionens förslag")
            ],
            doi: Some("10.23695/kvjc-k519"),
        }
    ),
    (
        "frsrdg",
        CorpusInfo {
            id: "rd-frsrdg",
            names: &[
                ("swe", "Riksdagens öppna data: Framställning/redogörelse"),
                ("eng", "Riksdag's open data: Reports")],
            descriptions: &[
                ("swe", "Framställningar och redogörelser från organ som utsetts av riksdagen")
            ],
            doi: Some("10.23695/pbvv-vj96"),
        }
    ),
    (
        "segreg-frsrdg",
        CorpusInfo {
            id: "segreg-rd-frsrdg",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Framställning/redogörelse"),
                ("eng", "Segregation texts: Riksdag's open data: Reports")],
            descriptions: &[
                ("swe", "Framställningar och redogörelser från organ som utsetts av riksdagen")
            ],
            doi: Some("10.23695/25df-cc14"),
        }
    ),
    (
        "ip",
        CorpusInfo {
            id: "rd-ip",
            names: &[
                ("swe", "Riksdagens öppna data: Interpellation"),
                ("eng", "Riksdag's open data: Interpellations")],
            descriptions: &[
                ("swe", "Interpellationer från ledamöterna till regeringen"),
                ("eng", "Interpellations from members of the Riksdag to the government"),
            ],
            doi: Some("10.23695/r20y-qp56"),
        }
    ),
    (
        "segreg-ip",
        CorpusInfo {
            id: "segreg-rd-ip",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Interpellation"),
                ("eng", "Segregation texts: Riksdag's open data: Interpellations")],
            descriptions: &[
                ("swe", "Interpellationer från ledamöterna till regeringen"),
                ("eng", "Interpellations from members of the Riksdag to the government"),
            ],
            doi: Some("10.23695/k0mr-5610"),
        }
    ),
    (
        "kammakt",
        CorpusInfo {
            id: "rd-kammakt",
            names: &[
                ("swe", "Riksdagens öppna data: Kammaraktiviteter"),
                ("eng", "Riksdag's open data: Activities in the Chamber")],
            descriptions: &[
                ("swe", "")
            ],
            doi: Some("10.23695/aadh-h368"),
        }
    ),
    (
        "segreg-kammakt",
        CorpusInfo {
            id: "segreg-rd-kammakt",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Kammaraktiviteter"),
                ("eng", "Segregation texts: Riksdag's open data: Activities in the Chamber")],
            descriptions: &[
                ("swe", "")
            ],
            doi: Some("10.23695/bes3-mz24"),
        }
    ),
    (
        "kom",
        CorpusInfo {
            id: "rd-kom",
            names: &[
                ("swe", "Riksdagens öppna data: KOM"),
                ("eng", "Riksdag's open data: EU initiatives")],
            descriptions: &[
                ("swe", "EU-kommissionens förslag och redogörelser, så kallade KOM-dokument"),
                ("eng", "EU initiatives are documents from the European Commission, “COM documents”. ")
            ],
            doi: Some("10.23695/s9np-h680"),
        }
    ),
    (
        "segreg-kom",
        CorpusInfo {
            id: "segreg-rd-kom",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: KOM"),
                ("eng", "Segregation texts: Riksdag's open data: EU initiatives")],
            descriptions: &[
                ("swe", "EU-kommissionens förslag och redogörelser, så kallade KOM-dokument"),
                ("eng", "EU initiatives are documents from the European Commission, “COM documents”. ")
            ],
            doi: Some("10.23695/5wwx-na18"),
        }
    ),
    (
        "mot",
        CorpusInfo {
            id: "rd-mot",
            names: &[
                ("swe", "Riksdagens öppna data: Motion"),
                ("eng", "Riksdag's open data: Motions")],
            descriptions: &[
                ("swe", "Motioner från riksdagens ledamöter"),
                ("eng", "Motions from the members of the Riksdag"),
            ],
            doi: Some("10.23695/9hd8-5t52"),
        }
    ),
    (
        "segreg-mot",
        CorpusInfo {
            id: "segreg-rd-mot",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Motion"),
                ("eng", "Segregation texts: Riksdag's open data: Motions")],
            descriptions: &[
                ("swe", "Motioner från riksdagens ledamöter"),
                ("eng", "Motions from the members of the Riksdag"),
            ],
            doi: Some("10.23695/39f8-n459"),
        }
    ),
    (
        "prop",
        CorpusInfo {
            id: "rd-prop",
            names: &[
                ("swe", "Riksdagens öppna data: Proposition"),
                ("eng", "Riksdag's open data: Government bills")],
            descriptions: &[
                ("swe", "Propositioner och skrivelser från regeringen")
            ],
            doi: Some("10.23695/9rtb-tx57"),
        }
    ),
    (
        "segreg-prop",
        CorpusInfo {
            id: "segreg-rd-prop",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Proposition"),
                ("eng", "Segregation texts: Riksdag's open data: Government bills")],
            descriptions: &[
                ("swe", "Propositioner och skrivelser från regeringen")
            ],
            doi: Some("10.23695/kjpk-jc17"),
        }
    ),
    (
        "prot",
        CorpusInfo {
            id: "rd-prot",
            names: &[
                ("swe", "Riksdagens öppna data: Protokoll"),
                ("eng", "Riksdag's open data: Records of proceedings in the Chamber")],
            descriptions: &[
                ("swe", "Protokoll från kammarens sammanträden"),
                ("eng", "Records of proceedings in the Chamber"),
            ],
            doi: Some("10.23695/k6qn-5180"),
        }
    ),
    (
        "segreg-prot",
        CorpusInfo {
            id: "segreg-rd-prot",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Protokoll"),
                ("eng", "Segregation texts: Riksdag's open data: Records of proceedings in the Chamber")],
            descriptions: &[
                ("swe", "Protokoll från kammarens sammanträden"),
                ("eng", "Records of proceedings in the Chamber"),
            ],
            doi: Some("10.23695/f84c-rr14"),
        }
    ),
    (
        "rskr",
        CorpusInfo {
            id: "rd-rskr",
            names: &[
                ("swe", "Riksdagens öppna data: Riksdagsskrivelse"),
                ("eng", "Riksdag's open data: Written communications from the Riksdag")],
            descriptions: &[
                ("swe", "Skrivelser från riksdagen till regeringen"),
                ("eng", "Written communications from the Riksdag to the Government"),
            ],
            doi: Some("10.23695/1k29-qb51"),
        }
    ),
    (
        "segreg-rskr",
        CorpusInfo {
            id: "segreg-rd-rskr",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Riksdagsskrivelse"),
                ("eng", "Segregation texts: Riksdag's open data: Written communications from the Riksdag")],
            descriptions: &[
                ("swe", "Skrivelser från riksdagen till regeringen"),
                ("eng", "Written communications from the Riksdag to the Government"),
            ],
            doi: None,
        }
    ),
    (
        "samtr",
        CorpusInfo {
            id: "rd-samtr",
            names: &[
                ("swe", "Riksdagens öppna data: Sammanträden"),
                ("eng", "Riksdag's open data: Meetings")],
            descriptions: &[
                ("swe", "")
            ],
            doi: Some("10.23695/6ymp-dr58"),
        }
    ),
    (
        "segreg-samtr",
        CorpusInfo {
            id: "segreg-rd-samtr",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Sammanträden"),
                ("eng", "Segregation texts: Riksdag's open data: Meetings")],
            descriptions: &[
                ("swe", "")
            ],
            doi: None,
        }
    ),
    (
        "sfs",
        CorpusInfo {
            id: "sfs",
            names: &[
                ("swe", "Riksdagens öppna data: Svensk författningssamling"),
                ("eng", "Riksdag's open data: Swedish Code of Statues"),
            ],
            descriptions: &[
                ("swe", "Svensk författningssamling"),
                ("eng", "Swedish Code of Statues"),
            ],
            doi: Some("10.23695/469a-gq88")
        }
    ),
    (
        "sfs-aktuella",
        CorpusInfo {
            id: "sfs-aktuella",
            names: &[
                ("swe", "Riksdagens öppna data: Svensk författningssamling (aktuella)"),
                ("eng", "Riksdag's open data: Swedish Code of Statues (current)"),
            ],
            descriptions: &[
                ("swe", "Svensk författningssamling (aktuella)"),
                ("eng", "Swedish Code of Statues (current)"),
            ],
            doi: None,
        }
    ),
    (
        "sfs-upphävda",
        CorpusInfo {
            id: "sfs-upphavda",
            names: &[
                ("swe", "Riksdagens öppna data: Svensk författningssamling (upphävda)"),
                ("eng", "Riksdag's open data: Swedish Code of Statues (repealed)"),
            ],
            descriptions: &[
                ("swe", "Svensk författningssamling (upphävda)"),
                ("eng", "Swedish Code of Statues (repealed)"),
            ],
            doi: None
        }
    ),
    (
        "Skriftliga+frågor",
        CorpusInfo {
            id: "rd-skfr",
            names: &[
                ("swe", "Riksdagens öppna data: Skriftliga frågor"),
                ("eng", "Riksdag's open data: Written questions")],
            descriptions: &[
                ("swe", "Skriftliga frågor från ledamöterna till regeringen och svaren på dessa"),
                ("eng", "Written questions from members of the Riksdag to the Government and the answer to these")
            ],
            doi: Some("10.23695/w468-bn43"),
        }
    ),
    (
        "segreg-Skriftliga+frågor",
        CorpusInfo {
            id: "segreg-rd-skfr",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Skriftliga frågor"),
                ("eng", "Segregation texts: Riksdag's open data: Written questions")],
            descriptions: &[
                ("swe", "Skriftliga frågor från ledamöterna till regeringen och svaren på dessa"),
                ("eng", "Written questions from members of the Riksdag to the Government and the answer to these")
            ],
            doi: Some("10.23695/vhwd-mp90"),
        }
    ),
    (
        "sou",
        CorpusInfo {
            id: "rd-sou",
            names: &[
                ("swe", "Riksdagens öppna data: Statens offentliga utredningar"),
                ("eng", "Riksdag's open data: Swedish Government Official Reports (SOU series)")],
            descriptions: &[
                ("swe", "Olika utredningars förslag till regeringen")
            ],
            doi: Some("10.23695/9tff-ay21"),
        }
    ),
    (
        "segreg-sou",
        CorpusInfo {
            id: "segreg-rd-sou",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Statens offentliga utredningar"),
                ("eng", "Segregation texts: Riksdag's open data: Swedish Government Official Reports (SOU series)")],
            descriptions: &[
                ("swe", "Olika utredningars förslag till regeringen")
            ],
            doi: Some("10.23695/ksp6-j384"),
        }
    ),
    (
        "t-lista",
        CorpusInfo {
            id: "rd-tlista",
            names: &[
                ("swe", "Riksdagens öppna data: Talarlista"),
                ("eng", "Riksdag's open data: List of speakers")],
            descriptions: &[
                ("swe", "Talarlistor för kammarens sammanträden"),
                ("eng", "List of speakers at meetings of the Chamber")
            ],
            doi: Some("10.23695/tc3k-1n67"),
        }
    ),
    (
        "segreg-t-lista",
        CorpusInfo {
            id: "segreg-rd-tlista",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Talarlista"),
                ("eng", "Segregation texts: Riksdag's open data: List of speakers")],
            descriptions: &[
                ("swe", "Talarlistor för kammarens sammanträden"),
                ("eng", "List of speakers at meetings of the Chamber")
            ],
            doi: None,
        }
    ),
    (
        "Utredningar",
        CorpusInfo {
            id: "rd-utr",
            names: &[
                ("swe", "Riksdagens öppna data: Utredningar"),
            ],
            descriptions: &[
                ("swe", "Kommittédirektiv och kommittéberättelser för utredningar som regeringen tillsätter")
            ],
            doi: Some("10.23695/mq2k-cd08"),
        }
    ),
    (
        "segreg-Utredningar",
        CorpusInfo {
            id: "segreg-rd-utr",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Utredningar"),
            ],
            descriptions: &[
                ("swe", "Kommittédirektiv och kommittéberättelser för utredningar som regeringen tillsätter")
            ],
            doi: Some("10.23695/tjbn-jh96"),
        }
    ),
    (
        "utskottsdokument",
        CorpusInfo {
            id: "rd-utsk",
            names: &[
                ("swe", "Riksdagens öppna data: Utskottsdokument"),
                ("eng", "Riksdag's open data: Documents from Committees")],
            descriptions: &[
                ("swe", "Dokument från utskotten, bland annat KU-anmälningar, protokoll, verksamhetsberättelser och den gamla dokumentserien Utredningar från riksdagen")
            ],
            doi: Some("10.23695/1ykn-nt41"),
        }
    ),
    (
        "segreg-utskottsdokument",
        CorpusInfo {
            id: "segreg-rd-utsk",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Utskottsdokument"),
                ("eng", "Segregation texts: Riksdag's open data: Documents from Committees")],
            descriptions: &[
                ("swe", "Dokument från utskotten, bland annat KU-anmälningar, protokoll, verksamhetsberättelser och den gamla dokumentserien Utredningar från riksdagen")
            ],
            doi: Some("10.23695/mzsg-b466"),
        }
    ),
    (
        "yttr",
        CorpusInfo {
            id: "rd-yttr",
            names: &[
                ("swe", "Riksdagens öppna data: Yttrande"),
                ("eng", "Riksdag's open data: Statements of opinion")],
            descriptions: &[
                ("swe", "Utskottens yttranden")
            ],
            doi: Some("10.23695/paky-de67"),
        }
    ),
    (
        "segreg-yttr",
        CorpusInfo {
            id: "segreg-rd-yttr",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Yttrande"),
                ("eng", "Segregation texts: Riksdag's open data: Statements of opinion")],
            descriptions: &[
                ("swe", "Utskottens yttranden")
            ],
            doi: Some("10.23695/s704-f362"),
        }
    ),
    (
        "Övrigt",
        CorpusInfo {
            id: "rd-ovr",
            names: &[
                ("swe", "Riksdagens öppna data: Övrigt"),
                ("eng", "Riksdag's open data: Other documents")],
            descriptions: &[
                ("swe", "Dokumentserierna Riksrevisionens granskningsrapporter, Utredningar från Riksdagsförvaltningen och Rapporter från riksdagen samt planeringsdokument, bilagor till dokument och uttag ur riksdagens databaser och de gamla dokumentserierna Utredningar från riksdag")
            ],
            doi: Some("10.23695/e9vn-ar51"),
        }
    ),
    (
        "segreg-Övrigt",
        CorpusInfo {
            id: "segreg-rd-ovr",
            names: &[
                ("swe", "Segregationstexter: Riksdagens öppna data: Övrigt"),
                ("eng", "Segregation texts: Riksdag's open data: Other documents")],
            descriptions: &[
                ("swe", "Dokumentserierna Riksrevisionens granskningsrapporter, Utredningar från Riksdagsförvaltningen och Rapporter från riksdagen samt planeringsdokument, bilagor till dokument och uttag ur riksdagens databaser och de gamla dokumentserierna Utredningar från riksdag")
            ],
            doi: Some("10.23695/1ehc-c002"),
        }
    ),
];

pub const ALL_CORPORA: &[&str] = &[
    "rd-bet",
    "rd-ds",
    "rd-eun",
    "rd-flista",
    "rd-fpm",
    "rd-frsrdg",
    "rd-ip",
    "rd-kammakt",
    "rd-kom",
    "rd-mot",
    "rd-prop",
    "rd-prot",
    "rd-rskr",
    "rd-samtr",
    "rd-skfr",
    "rd-sou",
    "rd-tlista",
    "rd-utr",
    "rd-utsk",
    "rd-yttr",
    "rd-ovr",
];

#[cfg(test)]
mod tests {
    use crate::corpusinfo::{CorpusInfo, ALL_CORPORA, CORPUSINFO};

    fn find_corpus_by_id(corpus_id: &str) -> Option<&CorpusInfo> {
        CORPUSINFO
            .iter()
            .map(|(_, corpusinfo)| corpusinfo)
            .find(|&corpusinfo| corpusinfo.id == corpus_id)
            .map(|v| v as _)
    }

    #[test]
    fn all_corpora_are_valid_ids() {
        for corpus in ALL_CORPORA {
            assert!(find_corpus_by_id(corpus).is_some());
        }
    }
}
