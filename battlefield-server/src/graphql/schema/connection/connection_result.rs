use super::{Connection, Connector};

pub struct ConnectionResult<T: Connector> {
    pub(crate) connector: T,
    pub(crate) connection: Connection<T::Node>,
}

impl<T: Connector> ConnectionResult<T> {
    pub fn new(connector: T, connection: Connection<T::Node>) -> Self {
        Self {
            connector,
            connection,
        }
    }
}

macro_rules! connection {
    (impl $(<$($lt:lifetime),+>)? for $t:ty as $n:literal) => {
        #[juniper::graphql_object(name = $n)]
        impl$(<$($lt),+>)? $crate::graphql::schema::connection::ConnectionResult<$t> {
            pub fn total_count(&self) -> i32 {
                self.connector.len() as i32
            }

            pub fn edges(&self) -> &[$crate::graphql::schema::connection::Edge<<$t as $crate::graphql::schema::connection::Connector>::Node>] {
                self.connection.edges()
            }

            pub fn page_info(&self) -> &$crate::graphql::schema::connection::PageInfo {
                self.connection.page_info()
            }
        }
    };
}

pub(crate) use connection;
