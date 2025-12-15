use crate::dokument::forslag::UtskottsForslag;

const UTSKOTTSFORSLAG_SRC: &[&'static str] = &[
    r#"{
    "punkt": "1",
    "rubrik": "Framtidens arbetsmarknad",
    "forslag": "<BR/>Riksdagen avslår motionerna \r\n\r\n2018/19:296 av Ali Esbati m.fl. (V), \r\n\r\n2018/19:794 av Ann-Britt Åsebol och Lotta Finstorp (båda M) yrkande 1, \r\n\r\n2018/19:1812 av Marianne Pettersson m.fl. (S), \r\n\r\n2018/19:2539 av Peter Helander och Helena Lindahl (båda C) yrkande 2, \r\n\r\n2018/19:2807 av Jessica Polfjärd m.fl. (M) yrkande 22, \r\n\r\n2018/19:2842 av Martin Ådahl m.fl. (C) yrkande 7 och \r\n\r\n2018/19:2883 av Lotta Finstorp m.fl. (M, C, KD).<BR/><BR/>",
    "beslutstyp": "acklamation",
    "motforslag_nummer": "2",
    "motforslag_partier": "\"V\"",
    "votering_id": null,
    "votering_sammanfattning_html": null,
    "votering_url_xml": null,
    "rm": "2018/19",
    "bet": "AU6",
    "vinnare": "utskottet",
    "voteringskrav": "Enkel majoritet",
    "beslutsregelkvot": null,
    "beslutsregelparagraf": null,
    "punkttyp": null
  }"#,
    r#"{
    "punkt": "2",
    "rubrik": "Kompetensutveckling",
    "forslag": "<BR/>Riksdagen avslår motionerna \r\n\r\n2018/19:164 av Mattias Bäckström Johansson m.fl. (SD) yrkande 7, \r\n\r\n2018/19:1056 av Isak From och Björn Wiechel (båda S) yrkande 3, \r\n\r\n2018/19:1081 av Patrik Engström m.fl. (S) yrkandena 1 och 2, \r\n\r\n2018/19:2064 av Gulan Avci m.fl. (L), \r\n\r\n2018/19:2735 av Camilla Brodin m.fl. (KD) yrkande 21, \r\n\r\n2018/19:2807 av Jessica Polfjärd m.fl. (M) yrkande 19, \r\n\r\n2018/19:2860 av Solveig Zander m.fl. (C) yrkande 4 och \r\n\r\n2018/19:2900 av Jessica Rosencrantz m.fl. (M) yrkande 1.<BR/><BR/>",
    "beslutstyp": "acklamation",
    "motforslag_nummer": "4",
    "motforslag_partier": "\"SD\"",
    "votering_id": null,
    "votering_sammanfattning_html": null,
    "votering_url_xml": null,
    "rm": "2018/19",
    "bet": "AU6",
    "vinnare": "utskottet",
    "voteringskrav": "Enkel majoritet",
    "beslutsregelkvot": null,
    "beslutsregelparagraf": null,
    "punkttyp": null
  }"#,
    r#"{
    "punkt": "3",
    "rubrik": "Arbetsförmedlingen",
    "forslag": "<BR/>Riksdagen avslår motionerna \r\n\r\n2018/19:785 av Ann-Britt Åsebol och Elisabeth Björnsdotter Rahm (båda M), \r\n\r\n2018/19:1107 av Erik Bengtzboe (M), \r\n\r\n2018/19:1127 av Sten Bergheden och Camilla Waltersson Grönvall (båda M), \r\n\r\n2018/19:1430 av Josefin Malmqvist (M), \r\n\r\n2018/19:1471 av Solveig Zander och Anders Åkesson (båda C) yrkande 2, \r\n\r\n2018/19:1837 av Jan Ericson (M), \r\n\r\n2018/19:2807 av Jessica Polfjärd m.fl. (M) yrkande 6, \r\n\r\n2018/19:2842 av Martin Ådahl m.fl. (C) yrkande 4 och \r\n\r\n2018/19:2992 av Christian Carlsson m.fl. (KD) yrkande 9.<BR/><BR/>",
    "beslutstyp": "röstning",
    "motforslag_nummer": "6",
    "motforslag_partier": "\"M\"",
    "votering_id": "c9f0e47d-ffd5-4b00-b0b2-99f16d107133",
    "votering_sammanfattning_html": {
      "table": [
        {
          "tr": [
            {
              "td": {
                "h4": "Omröstning i sakfrågan",
                "p": "Utskottets förslag mot reservation 6 (M)"
              }
            },
            {
              "th": [
                "Parti",
                "Ja",
                "Nej",
                "Avstående",
                "Frånvarande"
              ]
            },
            {
              "td": [
                "S",
                "92",
                "0",
                "0",
                "8"
              ]
            },
            {
              "td": [
                "M",
                "0",
                "63",
                "0",
                "7"
              ]
            },
            {
              "td": [
                "SD",
                "58",
                "0",
                "0",
                "4"
              ]
            },
            {
              "td": [
                "C",
                "29",
                "0",
                "0",
                "2"
              ]
            },
            {
              "td": [
                "V",
                "27",
                "0",
                "1",
                "0"
              ]
            },
            {
              "td": [
                "KD",
                "0",
                "0",
                "22",
                "0"
              ]
            },
            {
              "td": [
                "L",
                "18",
                "0",
                "0",
                "1"
              ]
            },
            {
              "td": [
                "MP",
                "14",
                "0",
                "0",
                "2"
              ]
            },
            {
              "td": [
                "-",
                "0",
                "0",
                "0",
                "1"
              ]
            },
            {
              "td": [
                "Totalt",
                "238",
                "63",
                "23",
                "25"
              ]
            },
            {
              "td": {
                "h4": null
              }
            }
          ]
        },
        {
          "tr": [
            {
              "td": {
                "h4": "Omröstning i motivfrågan",
                "p": "Utskottets förslag mot reservation 8 (V)"
              }
            },
            {
              "th": [
                "Parti",
                "Ja",
                "Nej",
                "Avstående",
                "Frånvarande"
              ]
            },
            {
              "td": [
                "S",
                "93",
                "0",
                "0",
                "7"
              ]
            },
            {
              "td": [
                "M",
                "28",
                "1",
                "34",
                "7"
              ]
            },
            {
              "td": [
                "SD",
                "58",
                "0",
                "0",
                "4"
              ]
            },
            {
              "td": [
                "C",
                "29",
                "0",
                "0",
                "2"
              ]
            },
            {
              "td": [
                "V",
                "0",
                "28",
                "0",
                "0"
              ]
            },
            {
              "td": [
                "KD",
                "1",
                "0",
                "21",
                "0"
              ]
            },
            {
              "td": [
                "L",
                "18",
                "0",
                "0",
                "1"
              ]
            },
            {
              "td": [
                "MP",
                "14",
                "0",
                "0",
                "2"
              ]
            },
            {
              "td": [
                "-",
                "0",
                "0",
                "0",
                "1"
              ]
            },
            {
              "td": [
                "Totalt",
                "241",
                "29",
                "55",
                "24"
              ]
            },
            {
              "td": {
                "h4": null
              }
            }
          ]
        }
      ],
      "br": [
        null,
        null
      ]
    },
    "votering_url_xml": "http://data.riksdagen.se/votering/C9F0E47D-FFD5-4B00-B0B2-99F16D107133",
    "rm": "2018/19",
    "bet": "AU6",
    "vinnare": "utskottet",
    "voteringskrav": "Enkel majoritet",
    "beslutsregelkvot": null,
    "beslutsregelparagraf": null,
    "punkttyp": null
  }"#,
    r#"{
    "punkt": "4",
    "rubrik": "Arbetsmarknadspolitiska program och insatser",
    "forslag": "<BR/>Riksdagen avslår motionerna \r\n\r\n2018/19:1437 av Josefin Malmqvist och Niklas Wykman (båda M) yrkande 4, \r\n\r\n2018/19:2027 av Azadeh Rojhan Gustafsson m.fl. (S) yrkandena 1 och 2, \r\n\r\n2018/19:2141 av Joakim Sandell m.fl. (S) yrkande 6, \r\n\r\n2018/19:2296 av Åsa Lindhagen m.fl. (MP), \r\n\r\n2018/19:2595 av Jan Björklund m.fl. (L) yrkandena 5-7 och 9, \r\n\r\n2018/19:2660 av Sultan Kayhan (S) yrkande 3, \r\n\r\n2018/19:2690 av Roland Utbult m.fl. (KD) yrkande 3, \r\n\r\n2018/19:2740 av Leila Ali-Elmi (MP) yrkandena 1 och 2, \r\n\r\n2018/19:2807 av Jessica Polfjärd m.fl. (M) yrkande 8, \r\n\r\n2018/19:2842 av Martin Ådahl m.fl. (C) yrkandena 5 och 6 samt \r\n\r\n2018/19:2992 av Christian Carlsson m.fl. (KD) yrkandena 14 och 19.<BR/><BR/>",
    "beslutstyp": "acklamation",
    "motforslag_nummer": "9",
    "motforslag_partier": "\"M\"",
    "votering_id": null,
    "votering_sammanfattning_html": null,
    "votering_url_xml": null,
    "rm": "2018/19",
    "bet": "AU6",
    "vinnare": "utskottet",
    "voteringskrav": "Enkel majoritet",
    "beslutsregelkvot": null,
    "beslutsregelparagraf": null,
    "punkttyp": null
    }"#,
    r#"{
    "punkt": "5",
    "rubrik": "Kontroll av subventionerade anställningar",
    "forslag": "<BR/>Riksdagen avslår motionerna \r\n\r\n2018/19:383 av Jonas Sjöstedt m.fl. (V) yrkandena 7-13 och \r\n\r\n2018/19:1162 av Joakim Sandell m.fl. (S).<BR/><BR/>",
    "beslutstyp": "acklamation",
    "motforslag_nummer": "12",
    "motforslag_partier": "\"V\"",
    "votering_id": null,
    "votering_sammanfattning_html": null,
    "votering_url_xml": null,
    "rm": "2018/19",
    "bet": "AU6",
    "vinnare": "utskottet",
    "voteringskrav": "Enkel majoritet",
    "beslutsregelkvot": null,
    "beslutsregelparagraf": null,
    "punkttyp": null
  }"#,
    r#"{
    "punkt": "6",
    "rubrik": "Arbetsmarknaden för personer med funktionsnedsättning",
    "forslag": "<BR/>Riksdagen avslår motionerna \r\n\r\n2018/19:182 av Jimmy Loord (KD), \r\n\r\n2018/19:693 av Helena Bouveng (M), \r\n\r\n2018/19:784 av Ann-Britt Åsebol (M), \r\n\r\n2018/19:856 av Carina Ståhl Herrstedt m.fl. (SD) yrkandena 10 och 11, \r\n\r\n2018/19:1153 av Malin Larsson och Jasenko Omanovic (båda S), \r\n\r\n2018/19:1164 av Ida Karkiainen (S), \r\n\r\n2018/19:1466 av Per Lodenius (C), \r\n\r\n2018/19:2053 av Jan Björklund m.fl. (L) yrkandena 19 och 20, \r\n\r\n2018/19:2327 av Åsa Lindhagen (MP) yrkande 2 och \r\n\r\n2018/19:2807 av Jessica Polfjärd m.fl. (M) yrkande 10.<BR/><BR/>",
    "beslutstyp": "röstning",
    "motforslag_nummer": "15",
    "motforslag_partier": "\"C\"",
    "votering_id": "5dd28581-d0d8-4eeb-a61d-40031a373271",
    "votering_sammanfattning_html": {
      "table": {
        "tr": [
          {
            "td": {
              "h4": "Omröstning i sakfrågan",
              "p": "Utskottets förslag mot reservation 14 (SD)"
            }
          },
          {
            "th": [
              "Parti",
              "Ja",
              "Nej",
              "Avstående",
              "Frånvarande"
            ]
          },
          {
            "td": [
              "S",
              "93",
              "0",
              "0",
              "7"
            ]
          },
          {
            "td": [
              "M",
              "0",
              "0",
              "63",
              "7"
            ]
          },
          {
            "td": [
              "SD",
              "0",
              "58",
              "0",
              "4"
            ]
          },
          {
            "td": [
              "C",
              "0",
              "0",
              "29",
              "2"
            ]
          },
          {
            "td": [
              "V",
              "28",
              "0",
              "0",
              "0"
            ]
          },
          {
            "td": [
              "KD",
              "0",
              "0",
              "22",
              "0"
            ]
          },
          {
            "td": [
              "L",
              "0",
              "0",
              "18",
              "1"
            ]
          },
          {
            "td": [
              "MP",
              "14",
              "0",
              "0",
              "2"
            ]
          },
          {
            "td": [
              "-",
              "0",
              "0",
              "0",
              "1"
            ]
          },
          {
            "td": [
              "Totalt",
              "135",
              "58",
              "132",
              "24"
            ]
          },
          {
            "td": {
              "h4": null
            }
          }
        ]
      },
      "br": null
    },
    "votering_url_xml": "http://data.riksdagen.se/votering/5DD28581-D0D8-4EEB-A61D-40031A373271",
    "rm": "2018/19",
    "bet": "AU6",
    "vinnare": "utskottet",
    "voteringskrav": "Enkel majoritet",
    "beslutsregelkvot": null,
    "beslutsregelparagraf": null,
    "punkttyp": null
  }"#,
    r#"{
    "punkt": "7",
    "rubrik": "Arbetslöshetsförsäkringen",
    "forslag": "<BR/>Riksdagen avslår motionerna \r\n\r\n2018/19:78 av Magnus Persson m.fl. (SD) yrkandena 2, 3 och 9, \r\n\r\n2018/19:621 av Carina Ödebrink och Peter Persson (båda S), \r\n\r\n2018/19:660 av Rasmus Ling (MP), \r\n\r\n2018/19:781 av Lotta Finstorp (M), \r\n\r\n2018/19:860 av Edward Riedl (M), \r\n\r\n2018/19:1471 av Solveig Zander och Anders Åkesson (båda C) yrkande 1, \r\n\r\n2018/19:1765 av ClasGöran Carlsson och Monica Haider (båda S), \r\n\r\n2018/19:2034 av Gulan Avci m.fl. (L) yrkandena 1 och 2, \r\n\r\n2018/19:2547 av Rickard Nordin (C) yrkande 2, \r\n\r\n2018/19:2680 av Mathias Tegnér och Fredrik Lundh Sammeli (båda S) yrkande 2 och \r\n\r\n2018/19:2807 av Jessica Polfjärd m.fl. (M) yrkandena 2 och 3.<BR/><BR/>",
    "beslutstyp": "röstning",
    "motforslag_nummer": "20",
    "motforslag_partier": "\"L\"",
    "votering_id": "53799107-7125-48e3-85cd-5d157fc885f6",
    "votering_sammanfattning_html": {
      "table": {
        "tr": [
          {
            "td": {
              "h4": "Omröstning i sakfrågan",
              "p": "Utskottets förslag mot reservation 18 (M)"
            }
          },
          {
            "th": [
              "Parti",
              "Ja",
              "Nej",
              "Avstående",
              "Frånvarande"
            ]
          },
          {
            "td": [
              "S",
              "93",
              "0",
              "0",
              "7"
            ]
          },
          {
            "td": [
              "M",
              "0",
              "63",
              "0",
              "7"
            ]
          },
          {
            "td": [
              "SD",
              "0",
              "0",
              "58",
              "4"
            ]
          },
          {
            "td": [
              "C",
              "29",
              "0",
              "0",
              "2"
            ]
          },
          {
            "td": [
              "V",
              "28",
              "0",
              "0",
              "0"
            ]
          },
          {
            "td": [
              "KD",
              "21",
              "0",
              "1",
              "0"
            ]
          },
          {
            "td": [
              "L",
              "0",
              "0",
              "18",
              "1"
            ]
          },
          {
            "td": [
              "MP",
              "14",
              "0",
              "0",
              "2"
            ]
          },
          {
            "td": [
              "-",
              "0",
              "0",
              "0",
              "1"
            ]
          },
          {
            "td": [
              "Totalt",
              "185",
              "63",
              "77",
              "24"
            ]
          },
          {
            "td": {
              "h4": null
            }
          }
        ]
      },
      "br": null
    },
    "votering_url_xml": "http://data.riksdagen.se/votering/53799107-7125-48E3-85CD-5D157FC885F6",
    "rm": "2018/19",
    "bet": "AU6",
    "vinnare": "utskottet",
    "voteringskrav": "Enkel majoritet",
    "beslutsregelkvot": null,
    "beslutsregelparagraf": null,
    "punkttyp": null
  }"#,
];
#[test]
fn deserialize_utskottsforslag() -> anyhow::Result<()> {
    for i in 0..UTSKOTTSFORSLAG_SRC.len() {
        println!("dezerializing example {}", i);
        let _value: UtskottsForslag = serde_json::from_str(UTSKOTTSFORSLAG_SRC[i])?;
    }
    Ok(())
}
