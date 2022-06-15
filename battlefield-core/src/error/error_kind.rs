#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    ModuleNotFound,
    ScenarioNotFound,
}
