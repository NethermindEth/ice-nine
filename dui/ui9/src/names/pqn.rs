use crate::names::Fqn;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pqn {
    components: Vec<String>,
}

impl Pqn {
    pub fn from_iter<'a>(components: impl IntoIterator<Item = &'a str>) -> Self {
        let components = components.into_iter().map(String::from).collect();
        Self { components }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ str> {
        self.components.iter().map(String::as_ref)
    }

    pub fn into_full(self) -> Fqn {
        Fqn::from_iter(self.iter())
    }
}

impl From<Pqn> for Fqn {
    fn from(pqn: Pqn) -> Self {
        pqn.into_full()
    }
}
