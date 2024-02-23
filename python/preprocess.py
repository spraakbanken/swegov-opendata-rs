
"""Script for creating original/xml files for all rd-corpora."""

from datetime import datetime
import json
import re
import unicodedata
import xml.sax.saxutils
import zipfile
from pathlib import Path

from lxml import etree, html

from corpusinfo import corpusinfo

# Parsing large XML files:
# http://making-security-measurable.1364806.n2.nabble.com/Parsing-large-9-5mb-XML-files-with-lxml-td7580657.html

RAWDIR = "rawdata"
PROCESSED_JSON = "processed.json"
MAX_SIZE = 10 * 1024 * 1024  # Max size in bytes for output XML files


def preprocess_corpora(corpora=None, skip_files=None, testfile=None):
    """Preprocess corpora.

    corpora: List that specifies which corpora (corpus-IDs) to process (default: all)
    skip_files: Zip files which should not be processed.
    testfile: Parse only 'testfile' and output result to 'test.xml'.
    """
    # Get previously processed data
    processed_json = {}
    if Path(PROCESSED_JSON).is_file():
        with open(PROCESSED_JSON) as f:
            processed_json = json.load(f)

    for zippath in Path(RAWDIR).iterdir():
        if zippath.name.startswith(".") or not str(zippath).endswith(".zip"):
            continue

        # Don't process if in 'skip_files'
        if skip_files is not None and zippath.name in skip_files:
            continue

        # Get corpus name
        m = re.match(r"(\S+)-\d{4}-.+", zippath.name)
        prefix = m.group(1)
        if prefix not in corpusinfo:
            raise Exception(f"File {prefix} seems to be a new corpus!")
        corpus_id, name, descr = corpusinfo[prefix]

        # Process only if in 'corpora'
        if corpora is not None and corpus_id not in corpora:
            continue

        print(f"\nProcessing {zippath}")
        corpus_source_dir = Path("material") / corpus_id / "source" / Path(zippath.stem).stem
        make_corpus_config(corpus_id, name, descr, Path("material") / corpus_id)

        total_size = 0
        result = []
        processed_zip_dict = processed_json.get(str(zippath.name), {})
        counter = len(processed_zip_dict.values()) + 1

        # Iterate through files in zip file
        zipf = zipfile.ZipFile(zippath)
        for zipobj in zipf.filelist:
            # print(zipobj.filename)

            if testfile:
                if zipobj.filename != testfile:
                    continue

            # Skip if already processed
            if processed_zip_dict.get(str(zipobj.filename)) and not testfile:
                print(f"  Skipping file '{zipobj.filename}' (already processed)")

            filecontents = zipf.read(zipobj)
            xmlstring = preprocess_xml(filecontents, zipobj.filename, testfile=testfile)

            if testfile:
                if xmlstring:
                    write_xml(xmlstring, "test.xml")
                exit()

            this_size = len(xmlstring)

            # If adding the latest result would lead to the file size going over the limit, save
            if xmlstring and total_size + this_size > MAX_SIZE:
                write_xml(b"\n".join(result), corpus_source_dir / f"{corpus_source_dir.parts[-1]}-{counter}.xml")
                total_size = 0
                result = []
                counter += 1

            if xmlstring:
                result.append(xmlstring)
            total_size += this_size
            processed_zip_dict[str(zipobj.filename)] = f"{corpus_source_dir.parts[-1]}-{counter}.xml"

        # Save last file
        if result:
            write_xml(b"\n".join(result), corpus_source_dir / f"{corpus_source_dir.parts[-1]}-{counter}.xml")

        if not testfile:
            processed_json[str(zippath.name)] = processed_zip_dict
            write_json(processed_json, PROCESSED_JSON)


def preprocess_xml(xml_string, filename, testfile=False):
    """Extract meta data and html from f."""
    p = etree.XMLParser(huge_tree=True)
    tree = etree.fromstring(xml_string, p)

    # Create new element and build document
    docelem = etree.Element("dokument")
    textelem = etree.SubElement(docelem, "text")
    textelem.set("datatyp", "huvuddokument")

    search_document = tree.find("dokument")

    # The html node contains the main text. Files without <html> may have other text nodes
    if search_document.find("html") is None:
        print(f"    WARNING: No html found in {filename}")

    for elem in search_document:
        # Process contents
        if elem.tag == "html":
            process_html(elem, textelem, filename, testfile=testfile)
        # Skip "text" element (if it exists, it contains the same text as <html>)
        elif elem.tag == "text":
            continue
        # Collect meta data from within "dokument"
        elif elem.text is not None:
            if elem.text.strip():
                if elem.tag in ["images"]:
                    continue
                # Assign document attrs to docelem and the rest to textelem
                elif elem.tag in ["dok_id", "dokumentstatus_url_xml", "dokument_url_text", "dokument_url_html"]:
                    docelem.set(elem.tag, elem.text)
                else:
                    textelem.set(elem.tag, elem.text)

    # Collect more metadata (attributes outside of "dokument") and look for other text nodes
    intressenter = []
    interesting_textnodes = [
        'anforande > anf_text',
        'forslag > lydelse',
        'forslag > lydelse2',
        'uppgift > text',
        'utskottsforslag > votering_sammanfattning_html'
    ]
    for elem in tree.iter():
        # Skip dokument, html and text since they are already processed; ignore comments
        if elem.tag in ["dokument", "html", "text"] or type(elem) == etree._Comment:
            continue
        # Collect "intressent" metadata and process later
        elif elem.tag == "intressent":
            children = dict((c.tag, c.text) for c in elem.getchildren())
            name = children.get("namn", "")
            party = (children.get("partibet", "") or "").upper()
            if name and party:
                intressenter.append((name, party))
        # Collect all other relevant text nodes
        elif elem.text is not None and elem.text.strip():
            parent = next(elem.iterancestors())
            ancestorstring = parent.tag + " > " + elem.tag
            if ancestorstring in interesting_textnodes:
                textelem = etree.SubElement(docelem, "text")
                textelem.set("datatyp", parent.tag)
                process_html(elem, textelem, filename, testfile=testfile)
                # Metadata for new elements
                for child in list(parent):
                    if child != elem and child.text:
                        attrname = child.tag
                        if attrname.startswith("anf_"):
                            attrname = attrname[4:]
                        textelem.set(attrname, child.text.strip())

    # Check if docelem contains any text
    has_text = False
    for i in etree.ElementTextIterator(docelem):
        if len(i):
            has_text = True
            break
    if not has_text:
        return ""

    # Add "intressent" metadata as sets
    if intressenter:
        intressenter = set(intressenter)
        name_party = []
        name = []
        party = set()
        for n, p in intressenter:
            name_party.append(n + " (" + p + ")")
            name.append(n)
            party.add(p)
        textelem.set("intressent_namn_parti", "|" + "|".join(name_party) + "|")
        textelem.set("intressent_namn", "|" + "|".join(name) + "|")
        textelem.set("intressent_parti", "|" + "|".join(party) + "|")

    # Combine "datum" and "datumtid"
    docattrs = list(docelem.attrib.keys())
    if "datumtid" in docattrs:
        docelem.set("datum", docelem.attrib["datumtid"])
        docattrs.attrib.pop("datumtid")
    for textelem in docelem.iter("text"):
        textattrs = list(textelem.attrib.keys())
        if "datumtid" in textattrs:
            textelem.set("datum", textelem.attrib["datumtid"])
            textelem.attrib.pop("datumtid")
    # Remove invalid dates (year out of range)
    year = int(textelem.get("datum")[:4])
    this_year = int(datetime.today().strftime("%Y"))
    if (year < 1900) or (year > this_year):
        textelem.attrib.pop("datum")

    # Return new XML
    tree = etree.ElementTree(docelem)
    return etree.tostring(tree, pretty_print=True, encoding="utf-8")


def process_html(elem, textelem, filename, testfile=False):
    """Process the actual text content of the document."""
    def clean_html(text):
        # Remove chars between 0-31 and 127-159, but keep 10 (line break).
        # TODO: does this have any effect?
        text = re.sub(r'&#(' + "|".join(str(i) for i in [*range(0, 10), *range(11, 32), *range(127, 160)]) + ');', "", text)
        chars = [chr(i) for i in [*range(0, 10), *range(11, 32), *range(127, 160)]]
        text = text.translate(dict((ord(c), None) for c in chars))
        # Remove control and formatting chars
        text = "".join(c for c in text if unicodedata.category(c)[0:2] != "Cc")
        text = "".join(c for c in text if unicodedata.category(c)[0:2] != "Cf")
        # Replace "special" spaces with ordinary spaces
        text = re.sub(u"\u00A0", " ", text)
        text = re.sub(u"\u2006", " ", text)
        # Remove long rows of underscores
        text = re.sub(r"__+", "", text)
        # Remove some more dirt
        text = re.sub("<\!\[if ! IE\]>", "", text)
        text = re.sub("<\!--\[if lte IE 7\]>", "", text)
        text = re.sub("<\!--\[if gte IE 8\]>", "", text)
        text = re.sub("<\!\[if \!vml\]>", "", text)
        text = re.sub("<\!\[if \!supportMisalignedColumns\]>", "", text)
        text = re.sub("<\!\[if \!supportLineBreakNewLine\]>", "", text)
        text = re.sub("<\!\[endif\]-->", "", text)
        text = re.sub("<\!\[endif\]>", "", text)
        text = re.sub("<\!\[if \!supportEmptyParas\]>", "", text)
        text = re.sub(r"<\/?NOBR>", "", text)
        text = re.sub("&nbsp;", "", text)
        # Replace br tags with line breaks
        text = re.sub(r"<(br|BR)( [^>]*)?\/?>", "\n", text)

        # Remove soft hyphens
        text = re.sub(u"\u00AD", "", text)
        # Escape all tags that are not html. "INGENBILD" is not html but is not escaped so it can be removed more easily.
        # text = re.sub(r"<(\/?(?!(?:!--|\!DOCTYPE|ingenbild|INGENBILD|a|A|abbr|ABBR|acronym|ACRONYM|address|ADDRESS|applet|APPLET|area|AREA|article|ARTICLE|aside|ASIDE|audio|AUDIO|b|B|base|BASE|basefont|BASEFONT|bdi|BDI|bdo|BDO|big|BIG|blockquote|BLOCKQUOTE|body|BODY|br|BR|button|BUTTON|canvas|CANVAS|caption|CAPTION|center|CENTER|cite|CITE|code|CODE|col|COL|colgroup|COLGROUP|datalist|DATALIST|dd|DD|del|DEL|details|DETAILS|dfn|DFN|dialog|DIALOG|dir|DIR|div|DIV|dl|DL|dt|DT|em|EM|embed|EMBED|fieldset|FIELDSET|figcaption|FIGCAPTION|figure|FIGURE|font|FONT|footer|FOOTER|form|FORM|frame|FRAME|frameset|FRAMESET|h1|H1|h2|H2|h3|H3|h4|H4|h5|H5|h6|H6|head|HEAD|header|HEADER|hr|HR|html|HTML|i|I|iframe|IFRAME|img|IMG|input|INPUT|ins|INS|kbd|KBD|keygen|KEYGEN|label|LABEL|legend|LEGEND|li|LI|link|LINK|main|MAIN|map|MAP|mark|MARK|menu|MENU|menuitem|MENUITEM|meta|META|meter|METER|nav|NAV|noframes|NOFRAMES|noscript|NOSCRIPT|object|OBJECT|ol|OL|optgroup|OPTGROUP|option|OPTION|output|OUTPUT|p|P|param|PARAM|pre|PRE|progress|PROGRESS|q|Q|rp|RP|rt|RT|ruby|RUBY|s|S|samp|SAMP|script|SCRIPT|section|SECTION|select|SELECT|small|SMALL|source|SOURCE|span|SPAN|strike|STRIKE|strong|STRONG|style|STYLE|sub|SUB|summary|SUMMARY|sup|SUP|table|TABLE|tbody|TBODY|td|TD|textarea|TEXTAREA|tfoot|TFOOT|th|TH|thead|THEAD|time|TIME|title|TITLE|tr|TR|track|TRACK|tt|TT|u|U|ul|UL|var|VAR|video|VIDEO|wbr|WBR)(?:\b|\s))(?:[^>\/][^>]*)*)>", r"&lt;\1&gt;", text)
        return text

    contents = xml.sax.saxutils.unescape(elem.text)
    if not isinstance(contents, str):
        contents = contents.decode("UTF-8")

    cleaned_content = clean_html(contents)
    if not cleaned_content:
        return False

    # print(cleaned_content)
    # print("\n-----\n")
    contents = "<text>" + cleaned_content + "</text>"
    contentsxml = html.fromstring(contents, parser=etree.HTMLParser(remove_comments=True, remove_pis=True))


    # Remove some attributes and tags
    etree.strip_attributes(contentsxml, ["style", "class", "cellpadding", "cellspacing", "colspan", "images" ".",
                                         "align", "valign", "name", "rowspan"])
    etree.strip_elements(contentsxml, ["style", "STYLE", "meta", "META", "ingenbild", "INGENBILD", "script", "SCRIPT"])

    # Check text length
    orig_text_length = 0
    for i in etree.ElementTextIterator(contentsxml):
        orig_text_length += len(i)
    if not orig_text_length:
        return False

    if testfile:
        write_xml(etree.tostring(contentsxml, encoding="utf-8"), "test_orig.xml")

    # Remove tags but keep contents
    etree.strip_tags(contentsxml, ["table", "thead", "tbody", "form", "caption", "a", "link", "span", "em",
                                   "strong", "sub", "sup", "b", "i", "u", "nobr", "ul", "ol", "colgroup", "col", "tt",
                                   "dir", "del", "ins", "s", "label", "pre", "spanstyle", "metricconverterproductid",
                                   "spanclass", "bstyle", "istyle", "brclear", "brstyle", "comment", "img", "hr",
                                   "fontsize", "aname", "metricconverter", "astyle", "personname", "spanlang", "date",
                                   "font", "fontcolor", "ahref", "textovervagande", "rubrikavvikandemening"])


    # Replace divs with more meaningful tags
    for element in list(contentsxml.iter("div")):
        if "id" in element.attrib:
            if element.attrib["id"].startswith("page_"):
                element.tag = "page"
                element.attrib["id"] = element.attrib["id"][5:]
                continue

    # Replace some tags with p
    for element in list(contentsxml.iter("title", "h1", "h2", "h3", "h4", "h5", "h6", "li", "tr", "td", "th")):
        element.tag = "p"


    # Remove nested pages (but keep contents)
    for outer_page_elem in contentsxml.xpath("//page[.//page]"):
        outer_page_elem.tag = "kasta"

    # Remove attributes from p and remove nested ps (but keep contents)
    for outer_p_elem in contentsxml.xpath("//p[.//p]"):
        outer_p_elem.tag = "kasta"
    for p_elem in contentsxml.xpath("//p"):
        for attr in p_elem.attrib:
            p_elem.attrib.pop(attr)

    # Decompose anything that's not "text", "page" or "p"
    allowed_tags = set(["kasta", "div", "text", "page", "p"])
    forbidden_tags = set()
    for element in contentsxml.iter():
        if element.tag not in allowed_tags:
            forbidden_tags.add(element.tag)
            element.tag = "kasta"
    etree.strip_tags(contentsxml, ["kasta", "div"])
    if forbidden_tags:
        print(f"    Removed forbidden tags from {filename}: {forbidden_tags}")

    # Check text length to see that nothing was lost
    text_length = 0
    for i in etree.ElementTextIterator(contentsxml):
        text_length += len(i)
    if text_length != orig_text_length:
        diff = orig_text_length - text_length
        # if diff > 100:
        print(f"    Contents were lost in {filename} ({diff} chars missing)")
    if not text_length:
        return False

    # Remove unnecessary whitespace
    for element in contentsxml.iter():
        if element.tail is not None and not element.tail.strip():
            element.tail = None
        if element.text and not element.text.strip():
            element.text = None

    # Remove empty tags
    for element in contentsxml.xpath(".//*[not(node())]"):
        element.getparent().remove(element)

    # # Check for nested tags
    # if contentsxml.xpath("//*/p/*/p"):
    #     print(f"    WARNING: Found nested element 'p'")
    #     etree.dump(contentsxml.xpath("//*/p/*/p")[0])

    # Attach to parent
    if len(contentsxml) == 0:
        textelem.text = contentsxml.text
    for element in contentsxml:
        textelem.append(element)
    return True


################################################################################
# Auxiliaries
################################################################################

def make_corpus_config(corpus_id, name, descr, path):
    """Write Sparv corpus config file for sub corpus."""
    config_file = path / "config.yaml"
    if config_file.is_file():
        return
    path.mkdir(parents=True, exist_ok=True)
    config_content = (
        "parent: ../config.yaml\n"
        "\n"
        "metadata:\n"
        f"  id: {corpus_id}\n"
        "  name:\n"
        f"    swe: Riksdagens öppna data: {name}\n"
        "  description:\n"
        f"    swe: {descr}\n"
    )
    with open(config_file, "w") as f:
        f.write(config_content)
    print(f"  Config {config_file} written")


def write_xml(text, xmlpath):
    """Wrap 'text' in a file tag and save as 'xmlpath'."""
    corpus_source_dir = Path(xmlpath).parent
    corpus_source_dir.mkdir(exist_ok=True, parents=True)
    text = b"<file>\n" + text + b"\n</file>"
    with open(xmlpath, "wb") as f:
        f.write(text)
    print(f"  File {xmlpath} written")


def write_json(data, filepath):
    """Write json data to filepath."""
    dirpath = Path(filepath).parent
    dirpath.mkdir(parents=True, exist_ok=True)
    with open(filepath, "w") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)


if __name__ == "__main__":
    # corpora = ["rd-bet", "rd-ds", "rd-eun", "rd-flista", "rd-fpm", "rd-frsrdg", "rd-ip",
    #            "rd-kammakt", "rd-kom", "rd-mot", "rd-ovr", "rd-prop", "rd-prot", "rd-rskr",
    #            "rd-samtr", "rd-skfr", "rd-sou", "rd-tlista", "rd-utr", "rd-utsk", "rd-yttr"]

    preprocess_corpora(corpora=["rd-fpm"], skip_files=[],
                     #  testfile="h7a2eun10p.xml"
                       )

    # with open("gr091.xml") as f:
    #     xml_string = f.read()
    #     xmlstring = preprocess_xml(xml_string, "test", testfile=True)
    #     write_xml(xmlstring, "test.xml")
