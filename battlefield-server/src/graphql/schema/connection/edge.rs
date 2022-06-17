use super::ConnectionNode;

pub struct Edge<T: ConnectionNode>(pub(crate) T);

macro_rules! connection_edge {
    (impl $(<$($lt:lifetime),+>)? for $t:ty as $n:literal) => {
        #[juniper::graphql_object(name = $n)]
        impl$(<$($lt),+>)? $crate::graphql::schema::connection::Edge<$t> {
            pub fn node(&self) -> &$t {
                &self.0
            }

            pub fn cursor(&self) -> $crate::graphql::schema::connection::Cursor {
                use $crate::graphql::schema::connection::ConnectionNode;
                self.0.cursor()
            }
        }
    };
}

pub(crate) use connection_edge;
