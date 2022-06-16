use super::Connector;

pub struct ConnectionResult<T: Connector> {
    pub(crate) connector: T,
    pub(crate) connection: T::Connection,
}

impl<T: Connector> ConnectionResult<T> {
    pub fn new(connector: T, connection: T::Connection) -> Self {
        Self {
            connector,
            connection,
        }
    }
}

macro_rules! connection_result {
    (impl $(<$($lt:lifetime),+>)? for $t:ty as $n:literal) => {
        #[juniper::graphql_object(name = $n)]
        impl$(<$($lt),+>)? $crate::graphql::schema::connection::ConnectionResult<$t> {
            pub fn total_count(&self) -> i32 {
                self.connector.len() as i32
            }

            pub fn edges(&self) -> &[$crate::graphql::schema::connection::Edge<<<$t as $crate::graphql::schema::connection::Connector>::Connection as $crate::graphql::schema::connection::Connection>::Node>] {
                self.connection.edges()
            }

            pub fn page_info(&self) -> PageInfo {
                self.connection.page_info()
            }
        }
    };
}

pub(crate) use connection_result;
