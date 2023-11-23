use minidom::quick_xml::events::BytesStart;
use minidom_extension::minidom;

pub fn attrib_equals(elem: &BytesStart<'_>, name: &[u8], value: &[u8]) -> bool {
    for attr in elem.attributes() {
        if let Ok(attr) = attr {
            if attr.key.local_name().as_ref() == name && attr.value.as_ref() == value {
                return true;
            }
        } else {
            todo!("handle error");
        }
    }
    return false;
}
