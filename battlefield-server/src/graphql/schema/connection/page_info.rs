use super::Cursor;

#[derive(Clone, Debug, juniper::GraphQLObject)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: Cursor,
    pub end_cursor: Cursor,
}
