use std::fmt;

use chrono::{NaiveDate, NaiveDateTime};
use html5ever::rcdom::{self, NodeData};
use minidom::{Element, Node};
use quick_xml::{events::Event, Reader};
use scraper::{ElementRef, Html, Selector};
use select::{
    document::Document,
    predicate::{self, Class},
};
use soup::prelude::*;
use swegov_opendata::{DokUppgift, Dokument, DokumentStatus, DokumentStatusPage};

use crate::error::Error;

#[derive(Debug, Clone)] //, serde::Deserialize, serde::Serialize)]
                        // #[serde(rename = "corpus")]
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

    pub fn build_elem(&self) -> Element {
        let mut elem_builder = Element::builder("corpus", "").attr("id", self.id.as_str());
        for text in &self.text {
            let text_elem = text.build_elem();
            elem_builder = elem_builder.append(text_elem);
        }
        elem_builder.build()
    }
}
#[derive(Debug, Clone)] //, serde::Deserialize, serde::Serialize)]
                        // #[serde(rename = "text")]
pub struct Text {
    // #[serde(rename = "@dok_id")]
    dok_id: String,
    // #[serde(rename = "@title")]
    title: String,
    // #[serde(rename = "@subtitle")]
    subtitle: String,
    // #[serde(rename = "@rm")]
    rm: String,
    // #[serde(rename = "@date")]
    date: NaiveDate,
    // #[serde(
    //     rename = "@num_pages",
    //     skip_serializing_if = "Option::is_none",
    //     default
    // )]
    num_pages: Option<String>,
    // text: String,
    // #[serde(rename = "page")]
    upphnr: Option<String>,
    upphavd: Option<NaiveDate>,
    pages: Vec<Page>,
    // paragraphs: Vec<Paragraph>,
}

impl Text {
    pub fn build_elem(&self) -> Element {
        let mut elem_builder = Element::builder("text", "")
            .attr("dok_id", &self.dok_id)
            .attr("title", &self.title.replace("\r\n", " "))
            .attr("subtitle", &self.subtitle)
            .attr("rm", &self.rm)
            .attr("date", self.date.to_string());
        if let Some(num_pages) = &self.num_pages {
            elem_builder = elem_builder.attr("num_pages", num_pages);
        }
        if let Some(upphnr) = &self.upphnr {
            elem_builder = elem_builder.attr("upphnr", upphnr);
        }
        if let Some(upphavd) = &self.upphavd {
            elem_builder = elem_builder.attr("upphavd", upphavd.to_string());
        }
        for page in &self.pages {
            elem_builder = elem_builder.append(page.element.clone());
        }
        let elem = elem_builder.build();
        // let mut buffer = Vec::new();
        // dbg!(&elem);
        // elem.write_to(&mut buffer).expect("valid elem");
        elem
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

        println!("parsing html ...");
        let pages = parse_html(&html)?;
        // let fragment = Html::parse_fragment(&html);

        // dbg!(&fragment.html());
        // let document = select::document::Document::from(html.as_str());
        // dbg!(&document);

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
        let upphavd_opt = dokuppgift.get_by_kod("uppphavd");
        let upphavd = if let Some(upphavd_str) = upphavd_opt {
            let (upphavd_at, _remaining) =
                NaiveDate::parse_and_remainder(&upphavd_str, "%Y-%m-%d")?;
            Some(upphavd_at)
        } else {
            None
        };
        println!("return from try_from for Text");
        Ok(Self {
            dok_id,
            title: titel,
            subtitle: subtitel,
            rm,
            date: datum.date(),
            num_pages: dokuppgift.get_by_kod("sidantal").cloned(),
            // text: html,
            upphnr: dokuppgift.get_by_kod("upphnr").cloned(),
            upphavd,
            pages,
        })
    }
}

pub fn unescape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        tracing::trace!("ch = {}", ch);
        result.push(if ch != '\\' {
            ch
        } else {
            match chars.next() {
                Some('"') => '"',
                // Some('\\') => '\\',
                Some(ch2) => {
                    tracing::trace!("ch2 = {}", ch2);
                    ch2
                }
                None => panic!("Malformed escape"),
            }
        })
    }
    result
}

// pub fn parse_html(html: &str) -> Result<Vec<Page>, Error> {
#[tracing::instrument]
pub fn parse_html(html: &str) -> Result<Vec<Page>, Error> {
    println!("parse_html");
    //     tracing::trace!("parsing html '{}'", html);
    let mut pages = Vec::new();
    let document = Document::from(html);

    for (i, node) in document.find(Class("sida")).enumerate() {
        // dbg!(&node);
        let mut elem_builder = Element::builder("div", "").attr("page-no", i);
        for (attr, value) in node.attrs() {
            if attr != "style" {
                elem_builder = elem_builder.attr(attr, value);
            }
        }
        for child in node.children() {
            if let Some(child_elem) = parse_html_node_to_elem(&child) {
                elem_builder = elem_builder.append(child_elem);
            }
        }
        let elem = elem_builder.build();
        let page = Page {
            number: i,
            element: elem,
        };
        // dbg!(&page);
        pages.push(page);
    }

    if pages.len() == 0 {
        let soup = Soup::new(html);
        pages = extract_pages_a1(&soup)?;
    }
    //     let html_str: String = unescape(html);
    //     let mut reader = Reader::from_str(&html_str);

    //     loop {
    //         match reader.read_event()? {
    //             Event::Start(e) => match e.name().as_ref() {
    //                 b"div" => println!("div: {:?}", e),
    //                 b"style" => {
    //                     let _ = reader.read_to_end(e.to_end().name())?;
    //                 }
    //                 b"p" => {
    //                     println!("p: {:?}", e)
    //                 }
    //                 b"span" => {
    //                     println!("span: {:?}", e)
    //                 }
    //                 b"table" => {
    //                     println!("table: {:?}", e)
    //                 }
    //                 b"tr" => {
    //                     println!("tr: {:?}", e)
    //                 }
    //                 b"td" => {
    //                     println!("td: {:?}", e)
    //                 }
    //                 _ => todo!("handle {:?}", e),
    //             },
    //             // Event::Text(e) => txt.push(e.unescape()?.into_owned()),
    //             Event::Eof => break,
    //             _ => (),
    //         }
    //     }
    println!("returning from parse_html");
    Ok(pages)
}

pub fn parse_html_node_to_elem<'a>(node: &select::node::Node<'a>) -> Option<Node> {
    println!("parse_html_node_to_elem node={:?}", node);
    if let Some(text) = node.as_text() {
        Some(Node::Text(text.to_string()))
    } else if let Some(name) = node.name() {
        if !allowed_elem_name(name) {
            tracing::warn!()
        }
        let mut elem_builder = Element::builder(name, "");
        for (attr, value) in node.attrs() {
            if attr != "style" {
                elem_builder = elem_builder.attr(attr, value);
            }
        }
        for child in node.children() {
            if let Some(child_elem) = parse_html_node_to_elem(&child) {
                elem_builder = elem_builder.append(child_elem);
            }
        }
        let elem = elem_builder.build();
        Some(Node::Element(elem))
    } else if let Some(text) = node.as_comment() {
        Some(Node::Text(text.to_string()))
    } else {
        todo!("parse child as node {:?}", node)
    }
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
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    number: usize,
    element: Element,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(rename = "p")]
pub struct Page_01 {
    #[serde(rename = "@nr")]
    number: usize,
    #[serde(rename = "div")]
    paragraphs: Vec<Paragraph>,
}

// impl Default for Page {
//     fn default() -> Self {
//         Self {
//             number: 0,
//             paragraphs: Vec::default(),
//         }
//     }
// }

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

#[tracing::instrument]
pub fn extract_pages_a1(soup: &Soup) -> Result<Vec<Page>, Error> {
    let mut pages = Vec::new();

    let mut page: Page = Page {
        number: 0,
        element: Element::bare("page", ""),
    };
    let mut current_page_nr = 0;
    let body = soup.tag("body").find().expect("body tag");
    dbg_node(&body);
    for child in body.children() {
        dbg_node(&child);
        println!("converting soup to minidom");
        let node = soup_node_to_minidom(&child);
        if let Some(node) = node {
            println!("adding node to page.element {:?}", node);
            page.element.append_node(node);
            println!("added node to page.element");
            // if let Some(page) = &mut page {
            //     page.element.append_node(node);
            // } else {
            //     match node {
            //         Node::Element(element) => {
            //             page = Some(Page {
            //                 number: current_page_nr,
            //                 element: element,
            //             });
            //             current_page_nr += 1;
            //         }
            //         Node::Text(text) => todo!("handle this"),
            //     }
            // }
        }
    }
    // dbg!(&page);
    // for elem in soup.tag(true).find_all() {
    //     match &elem.data {
    //         NodeData::Element {
    //             name,
    //             attrs,
    //             template_contents,
    //             mathml_annotation_xml_integration_point,
    //         } => {
    //             if name.local == *"html"
    //                 || name.local == *"head"
    //                 || name.local == *"style"
    //                 || name.local == *"body"
    //             {
    //                 continue;
    //             }
    //             page = Some(Page {
    //                 number: current_page_nr,
    //                 element: Element::bare(name.local.to_string(), ""),
    //             });
    //             current_page_nr += 1;
    //         }
    //         _ => todo!("handle this"),
    //     }
    //     dbg_node(&elem);
    //     // todo!("help")
    // }
    pages.push(page);
    println!("returning from extract_pages_a1");
    Ok(pages)
}

fn soup_node_to_minidom(node: &rcdom::Handle) -> Option<minidom::Node> {
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents,
            mathml_annotation_xml_integration_point,
        } => {
            if name.local == *"html"
                || name.local == *"head"
                || name.local == *"style"
                || name.local == *"body"
            {
                return None;
            }
            println!("soup_node_to_minidom.element.name.local = {}", name.local);
            let mut elem_builder = Element::builder(name.local.to_string(), "");
            for attr in &*attrs.borrow() {
                let mut name = attr.name.local.to_string();
                // let name_trimmed = name.find(char::is_alphanumeric);.trim_start_matches(is_not_alphanumeric);
                // dbg!(name_trimmed);
                // name.retain(char::is_alphanumeric);
                let mut value = attr.value.to_string();
                println!(
                    "soup_node_to_minidom.element.attr: value (before trim) = '{}'",
                    value
                );
                // value.retain(char::is_alphanumeric);
                let value = value.trim_start_matches("\\\"");
                let value = value.trim_end_matches("\\\"");
                println!("soup_node_to_minidom.element.attr: {} = '{}'", name, value);

                elem_builder = elem_builder.attr(name, value);
                println!("added attr");
            }
            for child in node.children() {
                if let Some(child_node) = soup_node_to_minidom(&child) {
                    elem_builder = elem_builder.append(child_node);
                    println!("appended child_node")
                }
            }
            return Some(Node::Element(elem_builder.build()));
        }
        NodeData::Text { contents } => {
            println!("soup_node_to_minidom.contents = {:?}", contents);
            return Some(Node::Text(String::from(&*contents.borrow())));
        }
        _ => todo!("handle this"),
    }
    None
}

fn dbg_node(node: &rcdom::Handle) {
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents,
            mathml_annotation_xml_integration_point,
        } => {
            println!("Element {{ name = {},  }}", name.local);
        }
        NodeData::Text { contents } => {
            println!("Text '{:?}'", contents);
        }
        _ => todo!("handle"),
    }
}

fn is_not_alphanumeric(c: char) -> bool {
    !c.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    // #[test]
    // fn parse_fragment_duplicate_text() {
    //     let text = "<a name=\"K2\">2 kap. Har upphävts genom <i>lag (2016:51)</i>.\n</a>";
    //     let fragment = parse_fragment(text).unwrap();
    //     assert_eq!(
    //         fragment,
    //         Fragment::Tag(Tag {
    //             name: Some("K2".to_string()),
    //             href: None,
    //             text: "2 kap.".to_string()
    //         })
    //     )

    //     // let p_f: Fragment = match parse_fragment(&p_str) {
    //     //     Ok(p_f) => p_f,
    //     //     Err(err) => {
    //     //         eprintln!("Error: {:?}", err);
    //     //         println!("Error: {:?}", err);
    //     //         Fragment::Bad(Bad { text: p_str })
    //     //     }
    //     // };
    // }

    const EXAMPLE_HTML_FRAGMENT: &str = r##"<style>
    .document div {
        overflow: visible !important;
        width: 550px !important;
    }
</style><b>SFS nr</b>: 1976:257<br /><b>Departement/myndighet</b>: Bostadsdepartementet <br /> <b>Utfärdad</b>:
1976-05-13 <br /> <b>Upphävd</b>: 1992-01-01<br /><b>Författningen har upphävts genom</b>: SFS
1991:1929<br /><b>Ändringsregister</b>: <a href=\"http://rkrattsbaser.gov.se/sfsr?bet=1976:257\">SFSR
    (Regeringskansliet)</a> <br /><b>Källa</b>: <a href=\"http://rkrattsbaser.gov.se/sfst?bet=1976:257\">Fulltext
    (Regeringskansliet)</a> <br />
<hr />
<style>
    <!-- div.sfstoc {padding:10px;position:relative;width:90%;border-bottom:#ccc 1px solid;font-size:85%;}
    -->
</style>
<div class=\"sfstoc\">
    <h3>Innehåll:</h3>
    <ul>
        <li><a href=\"#overgang\">Övergångsbestämmelser</a></li>
    </ul>
</div>
<div>
    <p><a name=\"S1\"></a></p><a class=\"paragraf\" name=\"P1\"><b>1 §</b></a>   Enligt denna förordning får lån
    (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har<br />   1. inlösts enligt 56 a §
    arbetsmarknadskungörelsen (1966:368),<br />   2. avstyckats från jordbruksfastighet genom åtgärder i samband med
    jordbrukets rationalisering.<p><a name=\"P1S2\"></a></p>Förvärvslån får lämnas även för förvärv av
    egnahemsfastighet, som kan lösas in enligt arbetsmarknadskungörelsen, om inlösen undviks därigenom.<br />Förordning
    (1985:458).<p><a name=\"P1S3\"></a></p><a class=\"paragraf\" name=\"P2\"><b>2 §</b></a>   Låneverksamheten utövas av
    plan- och bostadsverket, länsbostadsnämnderna och kommunerna.<p><a name=\"P2S2\"></a></p>Organ för kommuns
    låneförmedling och annan verksamhet som sammanhänger med den (förmedlingsorgan) är kommunstyrelsen om ej kommunen
    har beslutat att verksamheten skall utövas av annat kommunalt organ.</div>"##;

    fn assert_node_equal(left: &Node, right: &Node) {
        match (left, right) {
            (Node::Text(left_text), Node::Text(right_text)) => assert_eq!(left_text, right_text),
            (Node::Element(left_elem), Node::Element(right_elem)) => {
                assert_elem_equal(left_elem, right_elem)
            }
            (l, r) => assert_eq!(l, r),
        }
    }

    fn assert_elem_equal(left: &Element, right: &Element) {
        use itertools::EitherOrBoth;

        dbg!(left, right);

        assert_eq!(left.name(), right.name());
        assert_eq!(left.ns(), right.ns());
        assert!(left.attrs().eq(right.attrs()));

        for cmp in left.nodes().zip_longest(right.nodes()) {
            match cmp {
                EitherOrBoth::Both(node1, node2) => assert_node_equal(node1, node2),
                EitherOrBoth::Left(node1) => {
                    dbg!(node1);

                    panic!("left is longer '{:?}'", node1)
                }
                EitherOrBoth::Right(node2) => {
                    dbg!(node2);
                    panic!("right is longer '{:?}'", node2)
                }
            }
        }
        // assert!(left
        //     .nodes()
        //     .zip_longest(right.nodes())
        //     .all(|either_or_both| {
        //         match either_or_both {
        //             itertools::EitherOrBoth::Both(node1, node2) => node1 == node2,
        //             _ => false,
        //         }
        //     }));
    }

    fn assert_pages_equal(left: &Page, right: &Page) {
        assert_eq!(left.number, right.number);

        assert_elem_equal(&left.element, &right.element);
    }

    #[test]
    fn test_extract_pages() {
        let soup = Soup::new(EXAMPLE_HTML_FRAGMENT);
        let pages = extract_pages_a1(&soup).unwrap();
        assert_eq!(pages.len(), 1);

        let expected = Page {
            number: 0,
            element: Element::builder("page", "")
                .append(Node::Element(
                    Element::builder("b", "")
                        .append(Node::Text("SFS nr".into()))
                        .build(),
                ))
                .append(Node::Text(": 1976:257".into()))
                .append(Element::bare("br", ""))
                .append(Node::Element(
                    Element::builder("b", "")
                        .append(Node::Text("Departement/myndighet".into()))
                        .build(),
                ))
                .append(Node::Text(": Bostadsdepartementet ".into()))
                .append(Element::bare("br", ""))
                .append(Node::Text(" ".into()))
                .append(Node::Element(
                    Element::builder("b", "")
                        .append(Node::Text("Utfärdad".into()))
                        .build(),
                ))
                .append(Node::Text(":\n1976-05-13 ".into()))
                .append(Element::bare("br", ""))
                .append(Node::Text(" ".into()))
                .append(Node::Element(
                    Element::builder("b", "")
                        .append(Node::Text("Upphävd".into()))
                        .build(),
                ))
                .append(Node::Text(": 1992-01-01".into()))
                .append(Element::bare("br", ""))
                .append(Node::Element(
                    Element::builder("b", "")
                        .append(Node::Text("Författningen har upphävts genom".into()))
                        .build(),
                ))
                .append(Node::Text(": SFS\n1991:1929".into()))
                .append(Element::bare("br", ""))
                .append(Node::Element(
                    Element::builder("b", "")
                        .append(Node::Text("Ändringsregister".into()))
                        .build(),
                ))
                .append(Node::Text(": ".into()))
                .append(Node::Element(
                    Element::builder("a", "")
                        .attr("href", "http://rkrattsbaser.gov.se/sfsr?bet=1976:257")
                        .append(Node::Text("SFSR\n    (Regeringskansliet)".into()))
                        .build(),
                ))
                .append(Node::Text(" ".into()))
                .append(Element::bare("br", ""))
                .append(Node::Element(
                    Element::builder("b", "")
                        .append(Node::Text("Källa".into()))
                        .build(),
                ))
                .append(Node::Text(": ".into()))
                .append(Node::Element(
                    Element::builder("a", "")
                        .attr("href", "http://rkrattsbaser.gov.se/sfst?bet=1976:257")
                        .append(Node::Text("Fulltext\n    (Regeringskansliet)".into()))
                        .build(),
                ))
                .append(Node::Text(" ".into()))
                .append(Element::bare("br", ""))
                .append(Node::Text("\n".into()))
                .append(Element::bare("hr", ""))
                .append(Node::Text("\n".into()))
                .append(Node::Text("\n".into()))
                .append(Node::Element(
                    Element::builder("div", "")
                        .attr("class", "sfstoc")
                        .append(Node::Text("\n    ".into()))
                        .append(Node::Element(
                            Element::builder("h3", "")
                                .append(Node::Text("Innehåll:".into()))
                                .build(),
                        ))
                        .append(Node::Text("\n    ".into()))
                        .append(Node::Element(
                            Element::builder("ul", "")
                                .append(Node::Text("\n        ".into()))
                                .append(Node::Element(
                                    Element::builder("li", "")
                                        .append(Node::Element(
                                            Element::builder("a", "")
                                                .attr("href", "#overgang")
                                                .append(Node::Text("Övergångsbestämmelser".into()))
                                                .build(),
                                        ))
                                        .build(),
                                ))
                                .append(Node::Text("\n    ".into()))
                                .build(),
                        ))
                        .append(Node::Text("\n".into()))
                        .build(),
                ))
                .append(Node::Text("\n".into()))
                .append(Node::Element(
                    Element::builder("div", "")
                        .append(Node::Text("\n    ".into()))
                        .append(Node::Element(
                            Element::builder("p", "")
                                .append(Node::Element(
                                    Element::builder("a", "").attr("name", "S1").build(),
                                ))
                                .build(),
                        ))
                        .append(Node::Element(
                            Element::builder("a", "")
                                .attr("name", "P1")
                                .attr("class", "paragraf")
                                .append(Node::Element(
                                    Element::builder("b", "")
                                        .append(Node::Text("1 §".into()))
                                        .build(),
                                ))
                                .build(),
                        ))
                        .append(Node::Text(" \u{a0}\u{a0}Enligt denna förordning får lån\n    (förvärvslån) lämnas för förvärv från staten av egnahemsfastighet som har".into()))
                        .append(Element::bare("br", ""))
                        .append(Node::Text("\u{a0}\u{a0}\u{a0}1. inlösts enligt 56 a §\n    arbetsmarknadskungörelsen (1966:368),".into()))
                        .append(Element::bare("br", ""))
                        .append(Node::Text("\u{a0}\u{a0}\u{a0}2. avstyckats från jordbruksfastighet genom åtgärder i samband med\n    jordbrukets rationalisering.".into()))
                        .append(Node::Element(
                            Element::builder("p", "")
                                .append(Node::Element(
                                    Element::builder("a", "")
                                        .attr("name", "P1S2")

                                        .build(),
                                ))
                                .build(),
                        ))
                        .append(Node::Text("Förvärvslån får lämnas även för förvärv av\n    egnahemsfastighet, som kan lösas in enligt arbetsmarknadskungörelsen, om inlösen undviks därigenom.".into()))
                        .append(Element::bare("br", ""))
                        .append(Node::Text("Förordning\n    (1985:458).".into()))
                        .append(Node::Element(
                            Element::builder("p", "")
                                .append(Node::Element(
                                    Element::builder("a", "")
                                        .attr("name", "P1S3")

                                        .build(),
                                ))
                                .build(),
                        ))
                        .append(Node::Element(
                            Element::builder("a", "")
                                .attr("name", "P2").attr("class", "paragraf")
                                .append(Node::Element(Element::builder("b", "").append((Node::Text("2 §".into()))).build()))
                                .build(),
                        ))
                        .append(Node::Text(" \u{a0}\u{a0}Låneverksamheten utövas av\n    plan- och bostadsverket, länsbostadsnämnderna och kommunerna.".into()))
                        .append(Node::Element(
                            Element::builder("p", "")
                                .append(Node::Element(
                                    Element::builder("a", "")
                                        .attr("name", "P2S2")

                                        .build(),
                                ))
                                .build(),
                        ))
                        .append(Node::Text("Organ för kommuns\n    låneförmedling och annan verksamhet som sammanhänger med den (förmedlingsorgan) är kommunstyrelsen om ej kommunen\n    har beslutat att verksamheten skall utövas av annat kommunalt organ.".into()))
                        .build(),
                ))
                .build(),
        };
        assert_pages_equal(&pages[0], &expected);
    }
}
