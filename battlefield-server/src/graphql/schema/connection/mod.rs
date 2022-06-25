mod connection_node;
mod connection_result;
mod connector;
mod cursor;
mod edge;
mod page_info;

pub(crate) use connection_result::connection;
pub(crate) use edge::connection_edge;

pub use connection_node::ConnectionNode;
pub use connection_result::ConnectionResult;
pub use connector::{Connector, IteratorConnector};
pub use cursor::Cursor;
pub use edge::Edge;
pub use page_info::PageInfo;

pub struct Connection<T: ConnectionNode> {
    edges: Vec<Edge<T>>,
    page_info: PageInfo,
}

impl<T: ConnectionNode> Connection<T> {
    pub fn new(edges: Vec<Edge<T>>, page_info: PageInfo) -> Self {
        Self { edges, page_info }
    }

    pub fn edges(&self) -> &[Edge<T>] {
        &self.edges
    }

    pub fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    pub fn empty_end() -> Self {
        Self {
            edges: vec![],
            page_info: PageInfo::default_end(),
        }
    }

    pub fn empty_start() -> Self {
        Self {
            edges: vec![],
            page_info: PageInfo::default_start(),
        }
    }
}
