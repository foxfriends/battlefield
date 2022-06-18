use super::{Connection, ConnectionNode, ConnectionResult, Cursor};

mod iterator_connector;
pub use iterator_connector::IteratorConnector;

pub trait Connector {
    type Node: ConnectionNode;

    fn len(&self) -> usize;
    fn first(&self, count: usize, after: Cursor) -> Connection<Self::Node>;
    fn last(&self, count: usize, before: Cursor) -> Connection<Self::Node>;

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

    fn initial(&self) -> Connection<Self::Node> {
        self.first(30, Cursor::Start)
    }
}
