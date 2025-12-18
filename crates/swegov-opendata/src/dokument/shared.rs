#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DokumentBase {
    avdelningar: Option<Avdelningar>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
pub struct Avdelningar {
    avdelning: Vec<Avdelning>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
pub struct Avdelning {
    #[yaserde(text = true)]
    #[serde(rename = "$text")]
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_avdelningar() {
        let source = "<avdelningar><avdelning>alla</avdelning></avdelningar>";

        let _actual: Avdelningar = yaserde::de::from_str(source).unwrap();
    }
}
