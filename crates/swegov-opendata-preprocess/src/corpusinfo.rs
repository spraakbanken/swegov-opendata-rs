pub fn corpusinfo(prefix: &str) -> Result<(&str, &str, &str), UnknownCorpus> {
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

const CORPUSINFO: [(&str, (&str, &str, &str));21] = [
    ("bet", (
        "rd-bet",
        "Betänkande",
        "Utskottens betänkanden och utlåtanden, inklusive rksdagens beslut, en sammanfattning av voteringsresultaten och Beslut i korthet")),
    ("ds", (
        "rd-ds",
        "Departementsserien",
        "Utredningar från regeringens departement")),
    ("EUN", (
        "rd-eun",
        "EUN",
        "Dokument från EU-nämnden, bland annat möteskallelser, föredragningslistor, protokoll och skriftliga samråd med regeringen")),
    ("f-lista", (
        "rd-flista",
        "Föredragningslista",
        "Föredragningslistor för kammarens sammanträden")),
    ("fpm", (
        "rd-fpm",
        "Faktapromemoria",
        "Regeringens faktapromemorior om EU-kommissionens förslag")),
    ("frsrdg", (
        "rd-frsrdg",
        "Framställning/redogörelse",
            "Framställningar och redogörelser från organ som utsetts av riksdagen")),
    ("ip", (
        "rd-ip",
        "Interpellation",
        "Interpellationer från ledamöterna till regeringen")),
    ("kammakt", (
        "rd-kammakt",
        "Kammaraktiviteter",
        "")),
    ("kom", (
        "rd-kom",
        "KOM",
        "EU-kommissionens förslag och redogörelser, så kallade KOM-dokument")),
    ("mot", (
        "rd-mot",
        "Motion",
        "Motioner från riksdagens ledamöter")),
    ("prop", (
        "rd-prop",
        "Proposition",
        "Propositioner och skrivelser från regeringen")),
    ("prot", (
        "rd-prot",
        "Protokoll",
        "Protokoll från kammarens sammanträden")),
    ("rskr", (
        "rd-rskr",
        "Riksdagsskrivelse",
        "Skrivelser från riksdagen till regeringen")),
    ("samtr", (
        "rd-samtr",
        "Sammanträden",
        "")),
    ("Skriftliga+frågor", (
        "rd-skfr",
        "Skriftliga frågor",
        "Skriftliga frågor från ledamöterna till regeringen och svaren på dessa")),
    ("sou", (
        "rd-sou",
        "Statens offentliga utredningar",
        "Olika utredningars förslag till regeringen")),
    ("t-lista", (
        "rd-tlista",
        "Talarlista",
        "Talarlistor för kammarens sammanträden")),
    ("Utredningar", (
        "rd-utr",
        "Utredningar",
        "Kommittédirektiv och kommittéberättelser för utredningar som regeringen tillsätter")),
    ("utskottsdokument", (
        "rd-utsk",
        "Utskottsdokument",
        "Dokument från utskotten, bland annat KU-anmälningar, protokoll, verksamhetsberättelser och den gamla dokumentserien Utredningar från riksdagen")),
    ("yttr", (
        "rd-yttr",
        "Yttrande",
        "Utskottens yttranden")),
    ("Övrigt", (
        "rd-ovr",
        "Övrigt",
        "Dokumentserierna Riksrevisionens granskningsrapporter, Utredningar från Riksdagsförvaltningen och Rapporter från riksdagen samt planeringsdokument, bilagor till dokument och uttag ur riksdagens databaser och de gamla dokumentserierna Utredningar från riksdag")),
];
