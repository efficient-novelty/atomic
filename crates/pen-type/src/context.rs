#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CheckContext {
    pub library_size: usize,
    pub ambient_depth: u32,
    pub clause_depth: u32,
}

impl CheckContext {
    pub fn scope_size(self) -> u32 {
        self.ambient_depth + self.clause_depth
    }
}
