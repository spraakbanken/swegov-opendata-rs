pub fn corpusinfo(prefix: &str) -> Result<(&str, &[(&str, &str)], &[(&str, &str)]), UnknownCorpus> {
    for (corpus_prefix, corpus_info) in CORPUSINFO {
        if corpus_prefix == prefix {
            return Ok(corpus_info);
        }
    }
    Err(UnknownCorpus(prefix.to_string()))
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("Unknown corpus '{0}'")]
pub struct UnknownCorpus(String);

const CORPUSINFO: [
    (&str, (&str, &[
        (&str, &str)], &[
            (&str,&str)])); 21] = [
    (
        "bet",
        (
            "rd-bet",
            &[
                ("swe", "Riksdagens öppna data: Betänkande"),
                ("eng", "Riksdag's open data: Committee reports and statements")
            ],
            &[
                ("swe", "Utskottens betänkanden och utlåtanden, inklusive riksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet")
            ]
        )
    ),
    (
        "ds",
        (
            "rd-ds",
            &[
                ("swe", "Riksdagens öppna data: Departementsserien"),
                ("eng", "Riksdag's open data: Ministry Publications Series")
            ],
            &[
                ("swe", "Utredningar från regeringens departement")
            ]
        )
    ),
    (
        "EUN",
        (
            "rd-eun",
            &[
                ("swe", "Riksdagens öppna data: EUN"),
                ("eng", "Riksdag's open data: Committee on EU Affairs")
            ],
            &[
                ("swe", "Dokument från EU-nämnden, bland annat möteskallelser, föredragningslistor, protokoll och skriftliga samråd med regeringen"),
                ("eng", "Documents from the Committee on EU Affairs")
            ]
        )
    ),
    (
        "f-lista",
        (
            "rd-flista",
            &[
                ("swe", "Riksdagens öppna data: Föredragningslista"),
                ("eng", "Riksdag's open data: Order papers")
            ],
            &[
                ("swe", "Föredragningslistor för kammarens sammanträden")
            ]
        )
    ),
    (
        "fpm",
        (
            "rd-fpm",
            &[
                ("swe", "Riksdagens öppna data: Faktapromemoria"),
                ("eng", "Riksdag's open data: Explanatory memorandums on EU proposals")],
            &[
                ("swe", "Regeringens faktapromemorior om EU-kommissionens förslag")
            ]
        )
    ),
    (
        "frsrdg",
        (
            "rd-frsrdg",
            &[
                ("swe", "Riksdagens öppna data: Framställning/redogörelse"),
                ("eng", "Riksdag's open data: Reports")],
            &[
                ("swe", "Framställningar och redogörelser från organ som utsetts av riksdagen")
            ]
        )
    ),
    (
        "ip",
        (
            "rd-ip",
            &[
                ("swe", "Riksdagens öppna data: Interpellation"),
                ("eng", "Riksdag's open data: Interpellations")],
            &[
                ("swe", "Interpellationer från ledamöterna till regeringen"),
                ("eng", "Interpellations from members of the Riksdag to the government"),
            ])),
    (
        "kammakt",
        (
            "rd-kammakt",
            &[
                ("swe", "Riksdagens öppna data: Kammaraktiviteter"),
                ("eng", "Riksdag's open data: Activities in the Chamber")],
            &[
                ("swe", "")
            ]
        )
    ),
    (
        "kom",
        (
            "rd-kom",
            &[
                ("swe", "Riksdagens öppna data: KOM"),
                ("eng", "Riksdag's open data: EU initiatives")],
            &[
                ("swe", "EU-kommissionens förslag och redogörelser, så kallade KOM-dokument"),
                ("eng", "EU initiatives are documents from the European Commission, “COM documents”. ")
            ]
        )
    ),
    (
        "mot",
        (
            "rd-mot",
            &[
                ("swe", "Riksdagens öppna data: Motion"),
                ("eng", "Riksdag's open data: Motions")],
            &[
                ("swe", "Motioner från riksdagens ledamöter"),
                ("eng", "Motions from the members of the Riksdag"),
            ]
        )
    ),
    (
        "prop",
        (
            "rd-prop",
            &[
                ("swe", "Riksdagens öppna data: Proposition"),
                ("eng", "Riksdag's open data: Government bills")],
            &[
                ("swe", "Propositioner och skrivelser från regeringen")
            ]
        )
    ),
    (
        "prot",
        (
            "rd-prot",
            &[
                ("swe", "Riksdagens öppna data: Protokoll"),
                ("eng", "Riksdag's open data: Records of proceedings in the Chamber")],
            &[
                ("swe", "Protokoll från kammarens sammanträden"),
                ("eng", "Records of proceedings in the Chamber"),
            ]
        )
    ),
    (
        "rskr",
        (
            "rd-rskr",
            &[
                ("swe", "Riksdagens öppna data: Riksdagsskrivelse"),
                ("eng", "Riksdag's open data: Written communications from the Riksdag")],
            &[
                ("swe", "Skrivelser från riksdagen till regeringen"),
                ("eng", "Written communications from the Riksdag to the Government"),
            ]
        )
    ),
    (
        "samtr",
        (
            "rd-samtr",
            &[
                ("swe", "Riksdagens öppna data: Sammanträden"),
                ("eng", "Riksdag's open data: Meetings")],
            &[
                ("swe", "")
            ]
        )
    ),
    (
        "Skriftliga+frågor",
        (
            "rd-skfr",
            &[
                ("swe", "Riksdagens öppna data: Skriftliga frågor"),
                ("eng", "Riksdag's open data: Written questions")],
            &[
                ("swe", "Skriftliga frågor från ledamöterna till regeringen och svaren på dessa"),
                ("eng", "Written questions from members of the Riksdag to the Government and the answer to these")
            ]
        )
    ),
    (
        "sou",
        (
            "rd-sou",
            &[
                ("swe", "Riksdagens öppna data: Statens offentliga utredningar"),
                ("eng", "Riksdag's open data: Swedish Government Official Reports (SOU series)")],
            &[
                ("swe", "Olika utredningars förslag till regeringen")
            ]
        )
    ),
    (
        "t-lista",
        (
            "rd-tlista",
            &[
                ("swe", "Riksdagens öppna data: Talarlista"),
                ("eng", "Riksdag's open data: List of speakers")],
            &[
                ("swe", "Talarlistor för kammarens sammanträden"),
                ("eng", "List of speakers at meetings of the Chamber")
            ]
        )
    ),
    (
        "Utredningar",
        (
            "rd-utr",
            &[
                ("swe", "Riksdagens öppna data: Utredningar"),
            ],
            &[
                ("swe", "Kommittédirektiv och kommittéberättelser för utredningar som regeringen tillsätter")
            ]
        )
    ),
    (
        "utskottsdokument",
        (
            "rd-utsk",
            &[
                ("swe", "Riksdagens öppna data: Utskottsdokument"),
                ("eng", "Riksdag's open data: Documents from Committees")],
            &[
                ("swe", "Dokument från utskotten, bland annat KU-anmälningar, protokoll, verksamhetsberättelser och den gamla dokumentserien Utredningar från riksdagen")
            ]
        )
    ),
    (
        "yttr",
        (
            "rd-yttr",
            &[
                ("swe", "Riksdagens öppna data: Yttrande"),
                ("eng", "Riksdag's open data: Statements of opinion")],
            &[
                ("swe", "Utskottens yttranden")
            ]
        )
    ),
    (
        "Övrigt",
        (
            "rd-ovr",
            &[
                ("swe", "Riksdagens öppna data: Övrigt"),
                ("eng", "Riksdag's open data: Other documents")],
            &[
                ("swe", "Dokumentserierna Riksrevisionens granskningsrapporter, Utredningar från Riksdagsförvaltningen och Rapporter från riksdagen samt planeringsdokument, bilagor till dokument och uttag ur riksdagens databaser och de gamla dokumentserierna Utredningar från riksdag")
            ]
        )
    ),
];
