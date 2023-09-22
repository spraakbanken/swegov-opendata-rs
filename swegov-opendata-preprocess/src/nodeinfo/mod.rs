pub mod minidom;
mod rcdom;

pub use self::minidom::{minidom_collect_texts, minidom_text_len};
pub use self::rcdom::{dbg_rcdom_node, rcdom_collect_texts, rcdom_text_len};
