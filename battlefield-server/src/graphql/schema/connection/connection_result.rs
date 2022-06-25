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
        #[juniper::graphql_object(name = $n, context = $crate::graphql::Context)]
        impl$(<$($lt),+>)? $crate::graphql::schema::connection::ConnectionResult<$t> {
            async fn total_count(&self) -> juniper::FieldResult<i32> {
                Ok($crate::graphql::schema::connection::Connector::len(&self.connector).await? as i32)
            }

            fn edges(&self) -> &[$crate::graphql::schema::connection::Edge<<$t as $crate::graphql::schema::connection::Connector>::Node>] {
                self.connection.edges()
            }

            fn page_info(&self) -> &$crate::graphql::schema::connection::PageInfo {
                self.connection.page_info()
            }
        }
    };
}

pub(crate) use connection;
