use super::{Scenario, ScenariosConnection};
use crate::graphql::schema::connection::{
    connection, ConnectionNode, Connector, Cursor, Edge, PageInfo,
};
use crate::graphql::schema::Context;

pub struct ScenariosConnector<'a> {
    context: &'a Context,
}

impl<'a> ScenariosConnector<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl<'a> Connector for ScenariosConnector<'a> {
    type Connection = ScenariosConnection<'a>;

    fn len(&self) -> usize {
        self.context.engine.scenarios().len()
    }

    fn first(&self, count: usize, after: Cursor) -> Self::Connection {
        let has_next_page;
        let mut has_previous_page = false;
        let mut start_cursor = Cursor::Start;

        let edges = match after {
            Cursor::Start => {
                let edges: Vec<_> = self
                    .context
                    .engine
                    .scenarios()
                    .take(count)
                    .map(Scenario)
                    .map(Edge)
                    .collect();
                if let Some(edge) = edges.first() {
                    start_cursor = edge.0.cursor();
                }
                has_next_page = edges.len() < self.len();
                edges
            }
            Cursor::End => {
                has_previous_page = self.context.engine.scenarios().len() != 0;
                start_cursor = Cursor::End;
                has_next_page = false;
                vec![]
            }
            Cursor::Node(after) => {
                let mut skipped = 0;
                let edges: Vec<_> = self
                    .context
                    .engine
                    .scenarios()
                    .skip_while(|scenario| {
                        let skip = scenario.name() != after;
                        skipped += skip as usize;
                        skip
                    })
                    .take(count)
                    .map(Scenario)
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
        ScenariosConnection { edges, page_info }
    }

    fn last(&self, count: usize, before: Cursor) -> Self::Connection {
        let has_next_page;
        let mut has_previous_page = false;
        let mut end_cursor = Cursor::End;

        let edges = match before {
            Cursor::Start => {
                has_next_page = self.context.engine.scenarios().len() != 0;
                end_cursor = Cursor::Start;
                has_previous_page = false;
                vec![]
            }
            Cursor::End => {
                let skip = self.len().saturating_sub(count);
                let edges: Vec<_> = self
                    .context
                    .engine
                    .scenarios()
                    .skip(skip)
                    .map(Scenario)
                    .map(Edge)
                    .collect();
                has_next_page = skip + edges.len() < self.len();
                edges
            }
            Cursor::Node(after) => {
                let mut skipped = 0;
                let mut edges: Vec<_> = self
                    .context
                    .engine
                    .scenarios()
                    .rev()
                    .skip_while(|scenario| {
                        let skip = scenario.name() != after;
                        skipped += skip as usize;
                        skip
                    })
                    .take(count)
                    .map(Scenario)
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
        ScenariosConnection { edges, page_info }
    }
}

connection!(impl<'a> for ScenariosConnector<'a> as "ScenariosConnection");
