use super::Cursor;

#[derive(Clone, Debug, juniper::GraphQLObject)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: Cursor,
    pub end_cursor: Cursor,
}

impl PageInfo {
    pub fn default_start() -> Self {
        Self {
            has_previous_page: false,
            has_next_page: false,
            start_cursor: Cursor::Start,
            end_cursor: Cursor::Start,
        }
    }

    pub fn default_end() -> Self {
        Self {
            has_previous_page: false,
            has_next_page: false,
            start_cursor: Cursor::End,
            end_cursor: Cursor::End,
        }
    }

    pub fn empty() -> Self {
        Self {
            has_previous_page: false,
            has_next_page: false,
            start_cursor: Cursor::End,
            end_cursor: Cursor::Start,
        }
    }
}
