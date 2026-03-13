use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Atom {
    App,
    Lam,
    Pi,
    Sigma,
    Univ,
    Var,
    Lib,
    Id,
    Refl,
    Susp,
    Trunc,
    PathCon,
    Flat,
    Sharp,
    Disc,
    Shape,
    Next,
    Eventually,
}

pub const FROZEN_V1_ATOMS: [Atom; 18] = [
    Atom::App,
    Atom::Lam,
    Atom::Pi,
    Atom::Sigma,
    Atom::Univ,
    Atom::Var,
    Atom::Lib,
    Atom::Id,
    Atom::Refl,
    Atom::Susp,
    Atom::Trunc,
    Atom::PathCon,
    Atom::Flat,
    Atom::Sharp,
    Atom::Disc,
    Atom::Shape,
    Atom::Next,
    Atom::Eventually,
];

impl Atom {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::App => "App",
            Self::Lam => "Lam",
            Self::Pi => "Pi",
            Self::Sigma => "Sigma",
            Self::Univ => "Univ",
            Self::Var => "Var",
            Self::Lib => "Lib",
            Self::Id => "Id",
            Self::Refl => "Refl",
            Self::Susp => "Susp",
            Self::Trunc => "Trunc",
            Self::PathCon => "PathCon",
            Self::Flat => "Flat",
            Self::Sharp => "Sharp",
            Self::Disc => "Disc",
            Self::Shape => "Shape",
            Self::Next => "Next",
            Self::Eventually => "Eventually",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Atom, FROZEN_V1_ATOMS};

    #[test]
    fn frozen_v1_atom_order_is_stable() {
        let names: Vec<_> = FROZEN_V1_ATOMS.into_iter().map(Atom::as_str).collect();
        assert_eq!(
            names,
            vec![
                "App",
                "Lam",
                "Pi",
                "Sigma",
                "Univ",
                "Var",
                "Lib",
                "Id",
                "Refl",
                "Susp",
                "Trunc",
                "PathCon",
                "Flat",
                "Sharp",
                "Disc",
                "Shape",
                "Next",
                "Eventually",
            ]
        );
    }
}
