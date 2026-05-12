use exn::Exn;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, thiserror::Error)]
#[error("Invalid prefix in '{0}'")]
pub struct FindPrefixError(String);

pub fn find_prefix(zippath_name: &str) -> Result<Option<String>, Exn<FindPrefixError>> {
    static CORPUS_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\S+)\s?-\d{4}-.+").expect("valid regex"));
    if let Some(matches) = CORPUS_RE.captures(zippath_name) {
        if let Some(prefix) = matches.get(1) {
            return Ok(Some(prefix.as_str().replace(' ', "+")));
        } else {
            exn::bail!(FindPrefixError(zippath_name.to_string()))
        }
    }
    Ok(None)
}
