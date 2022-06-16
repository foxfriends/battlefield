pub struct Scenario<'a>(pub &'a battlefield_core::Scenario);

#[juniper::graphql_object]
impl Scenario<'_> {
    fn path(&self) -> String {
        self.0.path().display().to_string()
    }

    fn name(&self) -> &str {
        self.0.name()
    }

    fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    fn description(&self) -> Option<&str> {
        self.0.data().map(|data| data.description())
    }
}
