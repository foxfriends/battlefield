use super::super::{Edge, PageInfo};
use super::{Connection, ConnectionNode, Connector, Cursor};

pub trait IteratorConnector {
    type Node: ConnectionNode;
    type Iter: Iterator<Item = Self::Node>;
    type IterRev: Iterator<Item = Self::Node>;

    fn len(&self) -> usize;
    fn items(&self) -> Self::Iter;
    fn items_rev(&self) -> Self::IterRev;
}

impl<T> Connector for T
where
    T: IteratorConnector,
{
    type Node = <Self as IteratorConnector>::Node;

    fn len(&self) -> usize {
        IteratorConnector::len(self)
    }

    fn first(&self, count: usize, after: Cursor) -> Connection<Self::Node> {
        let has_next_page;
        let mut has_previous_page = false;
        let mut start_cursor = Cursor::Start;

        let edges = match after {
            Cursor::Start => {
                let edges: Vec<_> = self.items().take(count).map(Edge).collect();
                if let Some(edge) = edges.first() {
                    start_cursor = edge.0.cursor();
                }
                has_next_page = edges.len() < self.len();
                edges
            }
            Cursor::End => {
                has_previous_page = self.len() != 0;
                start_cursor = Cursor::End;
                has_next_page = false;
                vec![]
            }
            after => {
                let mut skipped = 0;
                let edges: Vec<_> = self
                    .items()
                    .skip_while(|item| {
                        let skip = item.cursor() != after;
                        skipped += skip as usize;
                        skip
                    })
                    .skip(1)
                    .take(count)
                    .map(Edge)
                    .collect();
                if let Some(edge) = edges.first() {
                    start_cursor = edge.0.cursor();
                }
                has_previous_page = skipped != 0;
                has_next_page = skipped + count < self.len();
                edges
            }
        };

        let end_cursor = edges
            .last()
            .map(|edge| edge.0.cursor())
            .unwrap_or(Cursor::End);

        let page_info = PageInfo {
            has_next_page,
            has_previous_page,
            start_cursor,
            end_cursor,
        };
        Connection { edges, page_info }
    }

    fn last(&self, count: usize, before: Cursor) -> Connection<Self::Node> {
        let has_next_page;
        let mut has_previous_page = false;
        let mut end_cursor = Cursor::End;

        let edges = match before {
            Cursor::Start => {
                has_next_page = self.len() != 0;
                end_cursor = Cursor::Start;
                has_previous_page = false;
                vec![]
            }
            Cursor::End => {
                let skip = self.len().saturating_sub(count);
                let edges: Vec<_> = self.items().skip(skip).map(Edge).collect();
                has_next_page = skip + edges.len() < self.len();
                edges
            }
            before => {
                let mut skipped = 0;
                let mut edges: Vec<_> = self
                    .items_rev()
                    .skip_while(|item| {
                        let skip = item.cursor() != before;
                        skipped += skip as usize;
                        skip
                    })
                    .skip(1)
                    .take(count)
                    .map(Edge)
                    .collect();
                edges.reverse();
                if let Some(edge) = edges.last() {
                    end_cursor = edge.0.cursor();
                }
                has_next_page = skipped != 0;
                has_previous_page = skipped + count < self.len();
                edges
            }
        };

        let start_cursor = edges
            .first()
            .map(|edge| edge.0.cursor())
            .unwrap_or(Cursor::Start);

        let page_info = PageInfo {
            has_next_page,
            has_previous_page,
            start_cursor,
            end_cursor,
        };
        Connection { edges, page_info }
    }
}
