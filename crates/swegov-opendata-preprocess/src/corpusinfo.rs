pub fn corpusinfo(prefix: &str) -> Result<&CorpusInfo, UnknownCorpus> {
    for (corpus_prefix, corpus_info) in CORPUSINFO {
        if *corpus_prefix == prefix {
            return Ok(corpus_info);
        }
    }
    Err(UnknownCorpus(prefix.to_string()))
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("Unknown corpus '{0}'")]
pub struct UnknownCorpus(String);

pub struct CorpusInfo {
    pub id: &'static str,
    pub names: &'static [(&'static str, &'static str)],
    pub descriptions: &'static [(&'static str, &'static str)],
}

const CORPUSINFO: &[(&str, CorpusInfo)] = &[
    (
        "bet",
        CorpusInfo {
            id: "rd-bet",
            names:&[
                ("swe", "Riksdagens öppna data: Betänkande"),
                ("eng", "Riksdag's open data: Committee reports and statements")
            ],
            descriptions:&[
                ("swe", "Utskottens betänkanden och utlåtanden, inklusive riksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet")
            ]
        }
    ),
    (
        "ds",
        CorpusInfo {
            id:
            "rd-ds",
            names:&[
                ("swe", "Riksdagens öppna data: Departementsserien"),
                ("eng", "Riksdag's open data: Ministry Publications Series")
            ],
            descriptions:&[
                ("swe", "Utredningar från regeringens departement")
            ]
}

    ),
    (
        "EUN",
        CorpusInfo {
            id:
            "rd-eun",
            names:&[
                ("swe", "Riksdagens öppna data: EUN"),
                ("eng", "Riksdag's open data: Committee on EU Affairs")
            ],
            descriptions:&[
                ("swe", "Dokument från EU-nämnden, bland annat möteskallelser, föredragningslistor, protokoll och skriftliga samråd med regeringen"),
                ("eng", "Documents from the Committee on EU Affairs")
            ]
}

    ),
    (
        "f-lista",
        CorpusInfo {
            id:
            "rd-flista",
            names:&[
                ("swe", "Riksdagens öppna data: Föredragningslista"),
                ("eng", "Riksdag's open data: Order papers")
            ],
            descriptions:&[
                ("swe", "Föredragningslistor för kammarens sammanträden")
            ]
}

    ),
    (
        "fpm",
        CorpusInfo {
            id:
            "rd-fpm",
            names:&[
                ("swe", "Riksdagens öppna data: Faktapromemoria"),
                ("eng", "Riksdag's open data: Explanatory memorandums on EU proposals")],
            descriptions:&[
                ("swe", "Regeringens faktapromemorior om EU-kommissionens förslag")
            ]
}

    ),
    (
        "frsrdg",
        CorpusInfo {
            id:
            "rd-frsrdg",
            names:&[
                ("swe", "Riksdagens öppna data: Framställning/redogörelse"),
                ("eng", "Riksdag's open data: Reports")],
            descriptions:&[
                ("swe", "Framställningar och redogörelser från organ som utsetts av riksdagen")
            ]
}

    ),
    (
        "ip",
        CorpusInfo {
            id:
            "rd-ip",
            names:&[
                ("swe", "Riksdagens öppna data: Interpellation"),
                ("eng", "Riksdag's open data: Interpellations")],
            descriptions:&[
                ("swe", "Interpellationer från ledamöterna till regeringen"),
                ("eng", "Interpellations from members of the Riksdag to the government"),
            ]
}
    ),
    (
        "kammakt",
        CorpusInfo {
            id:
            "rd-kammakt",
            names:&[
                ("swe", "Riksdagens öppna data: Kammaraktiviteter"),
                ("eng", "Riksdag's open data: Activities in the Chamber")],
            descriptions:&[
                ("swe", "")
            ]
}

    ),
    (
        "kom",
        CorpusInfo {
            id:
            "rd-kom",
            names:&[
                ("swe", "Riksdagens öppna data: KOM"),
                ("eng", "Riksdag's open data: EU initiatives")],
            descriptions:&[
                ("swe", "EU-kommissionens förslag och redogörelser, så kallade KOM-dokument"),
                ("eng", "EU initiatives are documents from the European Commission, “COM documents”. ")
            ]
}

    ),
    (
        "mot",
        CorpusInfo {
            id:
            "rd-mot",
            names:&[
                ("swe", "Riksdagens öppna data: Motion"),
                ("eng", "Riksdag's open data: Motions")],
            descriptions:&[
                ("swe", "Motioner från riksdagens ledamöter"),
                ("eng", "Motions from the members of the Riksdag"),
            ]
}

    ),
    (
        "prop",
        CorpusInfo {
            id:
            "rd-prop",
            names:&[
                ("swe", "Riksdagens öppna data: Proposition"),
                ("eng", "Riksdag's open data: Government bills")],
            descriptions:&[
                ("swe", "Propositioner och skrivelser från regeringen")
            ]
}

    ),
    (
        "prot",
        CorpusInfo {
            id:
            "rd-prot",
            names:&[
                ("swe", "Riksdagens öppna data: Protokoll"),
                ("eng", "Riksdag's open data: Records of proceedings in the Chamber")],
            descriptions:&[
                ("swe", "Protokoll från kammarens sammanträden"),
                ("eng", "Records of proceedings in the Chamber"),
            ]
}

    ),
    (
        "rskr",
        CorpusInfo {
            id:
            "rd-rskr",
            names:&[
                ("swe", "Riksdagens öppna data: Riksdagsskrivelse"),
                ("eng", "Riksdag's open data: Written communications from the Riksdag")],
            descriptions:&[
                ("swe", "Skrivelser från riksdagen till regeringen"),
                ("eng", "Written communications from the Riksdag to the Government"),
            ]
}

    ),
    (
        "samtr",
        CorpusInfo {
            id:
            "rd-samtr",
            names:&[
                ("swe", "Riksdagens öppna data: Sammanträden"),
                ("eng", "Riksdag's open data: Meetings")],
            descriptions:&[
                ("swe", "")
            ]
}

    ),
    (
        "Skriftliga+frågor",
        CorpusInfo {
            id:
            "rd-skfr",
            names:&[
                ("swe", "Riksdagens öppna data: Skriftliga frågor"),
                ("eng", "Riksdag's open data: Written questions")],
            descriptions:&[
                ("swe", "Skriftliga frågor från ledamöterna till regeringen och svaren på dessa"),
                ("eng", "Written questions from members of the Riksdag to the Government and the answer to these")
            ]
}

    ),
    (
        "sou",
        CorpusInfo {
            id:
            "rd-sou",
            names:&[
                ("swe", "Riksdagens öppna data: Statens offentliga utredningar"),
                ("eng", "Riksdag's open data: Swedish Government Official Reports (SOU series)")],
            descriptions:&[
                ("swe", "Olika utredningars förslag till regeringen")
            ]
}

    ),
    (
        "t-lista",
        CorpusInfo {
            id:
            "rd-tlista",
            names:&[
                ("swe", "Riksdagens öppna data: Talarlista"),
                ("eng", "Riksdag's open data: List of speakers")],
            descriptions:&[
                ("swe", "Talarlistor för kammarens sammanträden"),
                ("eng", "List of speakers at meetings of the Chamber")
            ]
}

    ),
    (
        "Utredningar",
        CorpusInfo {
            id:
            "rd-utr",
            names:&[
                ("swe", "Riksdagens öppna data: Utredningar"),
            ],
            descriptions:&[
                ("swe", "Kommittédirektiv och kommittéberättelser för utredningar som regeringen tillsätter")
            ]
}

    ),
    (
        "utskottsdokument",
        CorpusInfo {
            id:
            "rd-utsk",
            names:&[
                ("swe", "Riksdagens öppna data: Utskottsdokument"),
                ("eng", "Riksdag's open data: Documents from Committees")],
            descriptions:&[
                ("swe", "Dokument från utskotten, bland annat KU-anmälningar, protokoll, verksamhetsberättelser och den gamla dokumentserien Utredningar från riksdagen")
            ]
}

    ),
    (
        "yttr",
        CorpusInfo {
            id:
            "rd-yttr",
            names:&[
                ("swe", "Riksdagens öppna data: Yttrande"),
                ("eng", "Riksdag's open data: Statements of opinion")],
            descriptions:&[
                ("swe", "Utskottens yttranden")
            ]
}

    ),
    (
        "Övrigt",
        CorpusInfo {
            id: "rd-ovr",
            names:&[
                ("swe", "Riksdagens öppna data: Övrigt"),
                ("eng", "Riksdag's open data: Other documents")],
            descriptions:&[
                ("swe", "Dokumentserierna Riksrevisionens granskningsrapporter, Utredningar från Riksdagsförvaltningen och Rapporter från riksdagen samt planeringsdokument, bilagor till dokument och uttag ur riksdagens databaser och de gamla dokumentserierna Utredningar från riksdag")
            ]
        }

    ),
];
