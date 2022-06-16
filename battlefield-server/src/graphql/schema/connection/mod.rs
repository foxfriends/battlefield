mod connection_result;
mod cursor;
mod edge;
mod page_info;

pub(crate) use connection_result::connection_result;
pub(crate) use edge::connection_edge;

pub use connection_result::ConnectionResult;
pub use cursor::Cursor;
pub use edge::Edge;
pub use page_info::PageInfo;

pub trait ConnectionNode {
    fn cursor(&self) -> Cursor;
}

pub trait Connection {
    type Node: ConnectionNode;

    fn edges(&self) -> &[Edge<Self::Node>];
    fn page_info(&self) -> PageInfo;
}

pub trait Connector {
    type Connection: Connection;

    fn len(&self) -> usize;

    fn first(&self, count: usize, after: Cursor) -> Self::Connection;
    fn last(&self, count: usize, before: Cursor) -> Self::Connection;

    fn get(
        self,
        first: Option<i32>,
        after: Option<Cursor>,
        last: Option<i32>,
        before: Option<Cursor>,
    ) -> ConnectionResult<Self>
    where
        Self: Sized,
    {
        let connection = if let Some(count) = first {
            let count = if count < 0 { 0 } else { count as usize };
            self.first(count, after.unwrap_or(Cursor::Start))
        } else if let Some(count) = last {
            let count = if count < 0 { 0 } else { count as usize };
            self.last(count, before.unwrap_or(Cursor::End))
        } else {
            self.initial()
        };
        ConnectionResult::new(self, connection)
    }

    fn initial(&self) -> Self::Connection {
        self.first(30, Cursor::Start)
    }
}
