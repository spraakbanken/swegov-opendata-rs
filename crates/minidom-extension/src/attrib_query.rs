use minidom::quick_xml::events::BytesStart;

pub fn attrib_equals(elem: &BytesStart<'_>, name: &[u8], value: &[u8]) -> bool {
    for attr in elem.attributes() {
        match attr {
            Ok(attr) => {
                if attr.key.local_name().as_ref() == name && attr.value.as_ref() == value {
                    return true;
                }
            }
            Err(err) => {
                tracing_log_error::log_error!(err, "Error reading attribute",);
            }
        }
    }
    false
}
