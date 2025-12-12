use swegov_opendata::DataSet;

#[allow(clippy::large_enum_variant)]
pub enum Item {
    Raw(Vec<u8>),
    Metadata(DataSet),
}
