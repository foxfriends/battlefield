use battlefield_server::graphql::schema;

fn main() {
    println!("{}", schema().as_schema_language());
}
