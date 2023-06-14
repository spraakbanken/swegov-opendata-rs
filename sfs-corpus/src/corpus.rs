use std::error::Error as StdError;
use std::fmt;

use chrono::{NaiveDate, NaiveDateTime};
use minidom::Element;
use quick_xml::{events::Event, Reader};
use scraper::{ElementRef, Html, Selector};
use swegov_opendata::{DokUppgift, Dokument, DokumentStatus, DokumentStatusPage};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename = "corpus")]
pub struct Corpus {
    id: String,
    pub text: Vec<Text>,
}

impl Default for Corpus {
    fn default() -> Self {
        Self {
            id: String::default(),
            text: Vec::default(),
        }
    }
}

impl Corpus {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
    }

    pub fn add_text(&mut self, text: Text) {
        self.text.push(text);
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename = "text")]
pub struct Text {
    #[serde(rename = "@dok_id")]
    dok_id: String,
    #[serde(rename = "@title")]
    title: String,
    #[serde(rename = "@subtitle")]
    subtitle: String,
    #[serde(rename = "@rm")]
    rm: String,
    #[serde(rename = "@date")]
    date: NaiveDate,
    #[serde(
        rename = "@num_pages",
        skip_serializing_if = "Option::is_none",
        default
    )]
    num_pages: Option<String>,
    // text: String,
    #[serde(rename = "page")]
    pages: Vec<Page>,
    // paragraphs: Vec<Paragraph>,
}

#[derive(Debug)]
pub enum Error {
    Internal(String),
    XmlDe(quick_xml::DeError, Option<usize>),
    Xml(quick_xml::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(_err) => write!(f, "xml deserialization error "),
            Self::XmlDe(_err, num) => write!(f, "xml deserialization error on page {:?}", num),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Xml(err) => Some(err),
            Self::XmlDe(err, _) => Some(err),
            _ => None,
        }
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(value: quick_xml::DeError) -> Self {
        Self::XmlDe(value, None)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(value)
    }
}

impl TryFrom<DokumentStatus> for Text {
    type Error = Error;

    fn try_from(value: DokumentStatus) -> Result<Self, Self::Error> {
        // println!("value={:#?}", value);
        let DokumentStatus {
            dokument,
            dokuppgift,
            dokbilaga,
        } = value;
        let Dokument {
            dok_id,
            rm,
            datum,
            titel,
            html,
            subtitel,
            ..
        } = dokument;
        // let mut pages = Vec::new();
        if html.is_empty() {
            todo!("handle `html` empty");
        }

        let pages = parse_html(&html)?;
        // let fragment = Html::parse_fragment(&html);
        // if parse_ocr_div(&text, &mut pages)? {}
        // let div_selector = make_selector("div[class=\"sida\"]");
        // for (i, div_page) in fragment.select(&div_selector).enumerate() {
        //     let mut page = parse_page(div_page).map_err(|err| match err {
        //         Error::XmlDe(msg, _num) => Error::XmlDe(msg, Some(i)),
        //         _ => err,
        //     })?;
        //     page.set_number(i);
        //     pages.push(page);
        // }
        // if pages.len() == 0 {
        //     let div_selector = make_selector("div");
        //     for (i, div_page) in fragment.select(&div_selector).enumerate() {
        //         let mut page = parse_2023_div(div_page).map_err(|err| match err {
        //             Error::XmlDe(msg, _num) => Error::XmlDe(msg, Some(i)),
        //             _ => err,
        //         })?;
        //         page.set_number(i);
        //         pages.push(page);
        //     }
        // }
        // let p_selector = Selector::parse("p").unwrap();
        // let p_any_selector = Selector::parse("p > *").unwrap();
        // let mut paragraphs = Vec::new();
        // for div in fragment.select(&div_selector) {
        //     let mut paragraph = Paragraph::new();
        //     // println!("div={:?}", div.html());
        //     // let p: Element = div.html().parse().unwrap();
        //     // println!("p={:?}", p);
        //     for p in div.select(&p_any_selector) {
        //         let p_str = p.html().replace("<br>", "<br />");
        //         println!("p={:?}", p_str);
        //         let p_f: Fragment = quick_xml::de::from_str(&p_str).expect("parse paragraph");
        //         println!("fragment={:?}", p_f);
        //         paragraph.add_fragment(p_f);
        //     }
        //     paragraphs.push(paragraph);
        // }
        // println!("fragment={:?}", fragment.html());
        Ok(Self {
            dok_id,
            title: titel,
            subtitle: subtitel,
            rm,
            date: datum.date(),
            num_pages: num_pages(&dokuppgift),
            // text: html,
            pages,
        })
    }
}

pub fn num_pages(dokuppgift: &DokUppgift) -> Option<String> {
    for uppgift in &dokuppgift.uppgift {
        if uppgift.kod.as_str() == "sidantal" {
            return uppgift.text.clone();
        }
    }
    None
}

pub fn parse_html(html: &str) -> Result<Vec<Page>, Error> {
    let mut pages = Vec::new();
    let mut reader = Reader::from_str(html);

    loop {
        match reader.read_event()? {
            Event::Start(e) => match e.name().as_ref() {
                b"div" => println!("{:?}", e),
                b"style" => {
                    let _ = reader.read_to_end(e.to_end().name())?;
                }
                _ => todo!("handle {:?}", e),
            },
            // Event::Text(e) => txt.push(e.unescape()?.into_owned()),
            Event::Eof => break,
            _ => (),
        }
    }
    Ok(pages)
}
pub fn make_selector(selectors: &str) -> Selector {
    Selector::parse(selectors).expect("corpus: valid selectors")
}
// pub fn parse_2023_div(div: ElementRef) -> Result<Page, Error> {
//     let mut paragraphs = Vec::new();
//     // let mut paragraph = Paragraph::new();

//     // let p_str = div.inner_html().replace("<br>", "<br />");
//     // println!("  p={:?}", p_str);
//     // let p_f: Fragment = match quick_xml::de::from_str(&p_str) {
//     //     Ok(p_f) => p_f,
//     //     Err(err) => {
//     //         eprintln!("parse_2023_div: Error: {:?}", err);
//     //         println!("Error: {:?}", err);
//     //         Fragment::Bad(Bad { text: p_str })
//     //     }
//     // };
//     // println!("fragment={:?}", p_f);
//     // paragraph.add_fragment(p_f);
//     // paragraphs.push(paragraph);

//     let div_selector = make_selector("div");
//     let p_any_selector = make_selector("div>*");
//     for div in div.select(&p_any_selector) {
//         let mut paragraph = Paragraph::new();
//         println!("  [parse_2023_div] p={:?}", div.html());
//         // let p: Element = div.html().parse().unwrap();
//         // println!("p={:?}", p);
//         for p in div.select(&p_any_selector) {
//             let p_str = p.html().replace("<br>", "<br />");
//             println!("  p={:?}", p_str);
//             let p_f: Fragment = match quick_xml::de::from_str(&p_str) {
//                 Ok(p_f) => p_f,
//                 Err(err) => {
//                     eprintln!("Error: {:?}", err);
//                     println!("Error: {:?}", err);
//                     Fragment::Bad(Bad { text: p_str })
//                 }
//             };
//             println!("fragment={:?}", p_f);
//             paragraph.add_fragment(p_f);
//         }
//         paragraphs.push(paragraph);
//     }
//     Ok(Page {
//         paragraphs,
//         ..Default::default()
//     })
// }
// pub fn parse_ocr_div(div: ElementRef) -> Result<Page, Error> {
//     let div_selector = make_selector("div[class=\"sida\"]");
//     for (i, div_page) in fragment.select(&div_selector).enumerate() {
//         let mut page = parse_page(div_page).map_err(|err| match err {
//             Error::XmlDe(msg, _num) => Error::XmlDe(msg, Some(i)),
//             _ => err,
//         })?;
//         page.set_number(i);
//         pages.push(page);
//     }
//     let mut paragraphs = Vec::new();
//     let div_selector = make_selector("div");
//     let p_any_selector = make_selector("p > *");
//     for div in div.select(&div_selector) {
//         let mut paragraph = Paragraph::new();
//         // println!("div={:?}", div.html());
//         // let p: Element = div.html().parse().unwrap();
//         // println!("p={:?}", p);
//         for p in div.select(&p_any_selector) {
//             let p_str = p.html().replace("<br>", "<br />");
//             println!("  [parse_page] p={:?}", p_str);
//             let p_f: Fragment = match quick_xml::de::from_str(&p_str) {
//                 Ok(p_f) => p_f,
//                 Err(err) => {
//                     eprintln!("Error: {:?}", err);
//                     println!("Error: {:?}", err);
//                     Fragment::Bad(Bad { text: p_str })
//                 }
//             };
//             println!("fragment={:?}", p_f);
//             paragraph.add_fragment(p_f);
//         }
//         paragraphs.push(paragraph);
//     }
//     Ok(Page {
//         paragraphs,
//         ..Default::default()
//     })
// }

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(rename = "p")]
pub struct Page {
    #[serde(rename = "@nr")]
    number: usize,
    #[serde(rename = "div")]
    paragraphs: Vec<Paragraph>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            number: 0,
            paragraphs: Vec::default(),
        }
    }
}

impl Page {
    pub fn set_number(&mut self, number: usize) {
        self.number = number;
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(rename = "p")]
pub struct Paragraph {
    // #[serde(flatten)]
    #[serde(rename = "p")]
    fragments: Vec<Holder>,
}

impl Default for Paragraph {
    fn default() -> Self {
        Self {
            fragments: Vec::default(),
        }
    }
}
impl Paragraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_fragment(&mut self, fragment: Fragment) {
        self.fragments.push(Holder { any_name: fragment })
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Holder {
    #[serde(rename = "$value")]
    any_name: Fragment,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Fragment {
    #[serde(rename = "span")]
    Span(Span),
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "i")]
    Italic(Italic),
    #[serde(rename = "b")]
    Bold(Bold),
    #[serde(rename = "a")]
    Tag(Tag),
    #[serde(rename = "li")]
    ListItem(ListItem),
    // #[serde(rename = "$text")]
    // Text(String>),
    #[serde(rename = "bad")]
    Bad(Bad),
    // #[serde(rename = "$text", default)]
    // Text(String),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Span {
    #[serde(rename = "$text", default)]
    text: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Italic {
    #[serde(rename = "$text", default)]
    text: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Bold {
    #[serde(rename = "$text", default)]
    text: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct ListItem {
    #[serde(rename = "$text", default)]
    text: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Bad {
    #[serde(rename = "$text", default)]
    text: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Tag {
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "@href")]
    href: Option<String>,
    #[serde(rename = "$text", default)]
    text: String,
}

pub fn parse_fragment(text: &str) -> Result<Fragment, Error> {
    let p_f: Fragment = match quick_xml::de::from_str(text) {
        Ok(p_f) => p_f,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            println!("Error: {:?}", err);
            Fragment::Bad(Bad {
                text: text.to_string(),
            })
        }
    };
    Ok(p_f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fragment_duplicate_text() {
        let text = "<a name=\"K2\">2 kap. Har upphävts genom <i>lag (2016:51)</i>.\n</a>";
        let fragment = parse_fragment(text).unwrap();
        assert_eq!(
            fragment,
            Fragment::Tag(Tag {
                name: Some("K2".to_string()),
                href: None,
                text: "2 kap.".to_string()
            })
        )

        // let p_f: Fragment = match parse_fragment(&p_str) {
        //     Ok(p_f) => p_f,
        //     Err(err) => {
        //         eprintln!("Error: {:?}", err);
        //         println!("Error: {:?}", err);
        //         Fragment::Bad(Bad { text: p_str })
        //     }
        // };
    }
}
