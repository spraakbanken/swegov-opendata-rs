use swegov_opendata::DataSet;

pub enum Item {
    Raw(Vec<u8>),
    Metadata(DataSet),
}
