use chrono::NaiveDate;
use html5ever::rcdom::{self, NodeData};
use minidom::{Element, Node};

use soup::prelude::*;
use swegov_opendata::{Dokument, DokumentStatus};

use crate::error::Error;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Text {
    dok_id: String,
    title: String,
    subtitle: String,
    rm: String,
    date: NaiveDate,
    num_pages: Option<String>,
    upphnr: Option<String>,
    upphavd: Option<NaiveDate>,
    source: Option<String>,
    source_id: Option<String>,
    source_url: Option<String>,
    pages: Vec<Page>,
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
        if let Some(source) = &self.source {
            elem_builder = elem_builder.attr("source", source);
        }
        if let Some(source_id) = &self.source_id {
            elem_builder = elem_builder.attr("source_id", source_id);
        }
        if let Some(source_url) = &self.source_url {
            elem_builder = elem_builder.attr("source_url", source_url);
        }
        for page in &self.pages {
            elem_builder = elem_builder.append(page.element.clone());
        }
        let elem = elem_builder.build();
        elem
    }
}

impl TryFrom<DokumentStatus> for Text {
    type Error = Error;

    fn try_from(value: DokumentStatus) -> Result<Self, Self::Error> {
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
        if html.is_empty() {
            todo!("handle `html` empty");
        }

        tracing::debug!("parsing html ...");
        let pages = parse_html(&html)?;

        let upphavd_opt = dokuppgift.get_by_kod("uppphavd");
        let upphavd = if let Some(upphavd_str) = upphavd_opt {
            let (upphavd_at, _remaining) =
                NaiveDate::parse_and_remainder(&upphavd_str, "%Y-%m-%d")?;
            Some(upphavd_at)
        } else {
            None
        };
        let source = dokument.source.clone();
        let source_id = dokument.sourceid.clone();
        let source_url = if let Some(dokbilaga) = dokbilaga {
            let mut source_url = None;
            for bilaga in dokbilaga.bilaga.iter() {
                if !bilaga.fil_url.is_empty() {
                    if source_url.is_some() {
                        panic!(
                            "source_url is already {:?} and also found '{}'",
                            source_url, &bilaga.fil_url
                        );
                    }
                    source_url = Some(bilaga.fil_url.clone());
                }
            }
            source_url
        } else {
            None
        };
        tracing::debug!("return from try_from for Text");
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
            source,
            source_id,
            source_url,
            pages,
        })
    }
}

pub fn parse_html(html: &str) -> Result<Vec<Page>, Error> {
    let soup = Soup::new(html);
    let pages = extract_pages_a1(&soup)?;

    tracing::debug!("returning from parse_html");
    Ok(pages)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    number: usize,
    element: Element,
}

impl Default for Page {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Page {
    pub fn new(number: usize) -> Self {
        let element = Element::builder("page", "")
            .attr("page-number", number.to_string())
            .build();

        Self { number, element }
    }
}

pub fn extract_pages_a1(soup: &Soup) -> Result<Vec<Page>, Error> {
    let mut pages = Vec::new();

    let mut current_page_nr = 0;
    let mut page: Page = Page::new(current_page_nr);
    let body = soup.tag("body").find().expect("body tag");
    let body = body.tag("div").class("dok").find().unwrap_or(body);
    tracing::trace!("scanning childs");
    for (i, child) in body.children().enumerate() {
        tracing::trace!("- child {}, current_page_nr = {}", i, current_page_nr);
        tracing::trace!("converting soup to minidom");
        let node = soup_node_to_minidom(&child);
        if let Some(node) = node {
            if let Some(elem) = node.as_element() {
                if soup_attr_equals(&child, "class", "pageWrap") {
                    tracing::trace!("- found 'pageWrap' current_page_nr={}", current_page_nr);
                    if current_page_nr > 0 {
                        tracing::trace!("- pushing page number {}", page.number);
                        pages.push(page);
                        page = Page::new(current_page_nr);
                    }
                    current_page_nr += 1;
                }
            }
            tracing::trace!("adding node to page.element {:?}", node);
            page.element.append_node(node);
            tracing::trace!("added node to page.element");
        }
    }

    tracing::trace!("- pushing page number {}", page.number);
    pages.push(page);
    tracing::trace!("returning from extract_pages_a1");
    Ok(pages)
}

fn soup_attr_equals<'a>(node: &'a rcdom::Handle, name: &str, test: &str) -> bool {
    match &node.data {
        NodeData::Element {
            name: _,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => attrs
            .borrow()
            .iter()
            .find(|attr| attr.name.local.as_ref() == name)
            .map(|attr| attr.value.as_ref() == test)
            .unwrap_or(false),
        _ => false,
    }
}
fn soup_node_to_minidom(node: &rcdom::Handle) -> Option<minidom::Node> {
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            if name.local == *"html"
                || name.local == *"head"
                || name.local == *"style"
                || name.local == *"body"
            {
                return None;
            }
            tracing::trace!("soup_node_to_minidom.element.name.local = {}", name.local);
            if !allowed_elem_name(&name.local) {
                tracing::warn!("!!! Unallowed name '{}'", name.local);
                return Some(Node::Text(format!("<{}", name.local)));
            }
            let mut elem_builder = Element::builder(name.local.to_string(), "");
            for attr in &*attrs.borrow() {
                if ["style", "class"].contains(&attr.name.local.as_ref()) {
                    continue;
                }

                let value = attr.value.trim_start_matches("\\\"");
                let value = value.trim_end_matches("\\\"");
                tracing::trace!(
                    "soup_node_to_minidom.element.attr: {} = '{}'",
                    attr.name.local,
                    value
                );
                elem_builder = elem_builder.attr(attr.name.local.as_ref(), value);
                tracing::trace!("added attr");
            }
            for child in node.children() {
                if let Some(child_node) = soup_node_to_minidom(&child) {
                    elem_builder = elem_builder.append(child_node);
                    tracing::trace!("appended child_node")
                }
            }
            return Some(Node::Element(elem_builder.build()));
        }
        NodeData::Text { contents } => {
            tracing::trace!("soup_node_to_minidom.contents = {:?}", contents);
            return Some(Node::Text(String::from(&*contents.borrow())));
        }
        NodeData::Comment { contents } => {
            if contents.contains("begin:pages") {
                return None;
            }
            tracing::trace!("soup_node_to_minidom.Comment = {:?}", contents);
            return Some(Node::Text(String::from(&*contents)));
        }
        _ => {
            dbg_node(&node);
            todo!("handle this")
        }
    }
    None
}

fn allowed_elem_name(name: &str) -> bool {
    name == "div"
        || name == "span"
        || name == "a"
        || name == "p"
        || name == "b"
        || name == "br"
        || name == "table"
        || name == "tbody"
        || name == "tr"
        || name == "td"
        || name == "hr"
        || name == "h2"
        || name == "h3"
        || name == "h4"
        || name == "ul"
        || name == "li"
        || name == "pre"
        || name == "i"
}

fn dbg_node(node: &rcdom::Handle) -> String {
    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents: _,
            mathml_annotation_xml_integration_point: _,
        } => {
            let attrs_str: Vec<String> = if attrs.borrow().len() > 0 {
                attrs
                    .borrow()
                    .iter()
                    .map(|a| format!("{}='{}'", a.name.local, a.value))
                    .collect()
            } else {
                vec![]
            };
            format!(
                "Element {{ name = {}, attrs = {:?} }}",
                name.local, attrs_str
            )
        }
        NodeData::Text { contents } => {
            format!("Text '{:?}'", contents)
        }
        NodeData::Comment { contents } => {
            format!("Comment '{:?}'", contents)
        }
        _ => todo!("handle"),
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

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
                                .append(Node::Element(Element::builder("b", "").append(Node::Text("2 §".into())).build()))
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
