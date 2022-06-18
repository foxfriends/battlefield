use super::Cursor;

pub trait ConnectionNode {
    fn cursor(&self) -> Cursor;
}
