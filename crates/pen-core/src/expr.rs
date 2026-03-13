use crate::atom::Atom;
use crate::ids::ExprId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
pub enum Expr {
    App(Box<Expr>, Box<Expr>),
    Lam(Box<Expr>),
    Pi(Box<Expr>, Box<Expr>),
    Sigma(Box<Expr>, Box<Expr>),
    Univ,
    Var(u32),
    Lib(u32),
    Id(Box<Expr>, Box<Expr>, Box<Expr>),
    Refl(Box<Expr>),
    Susp(Box<Expr>),
    Trunc(Box<Expr>),
    PathCon(u32),
    Flat(Box<Expr>),
    Sharp(Box<Expr>),
    Disc(Box<Expr>),
    Shape(Box<Expr>),
    Next(Box<Expr>),
    Eventually(Box<Expr>),
}

impl Expr {
    pub const fn atom(&self) -> Atom {
        match self {
            Self::App(_, _) => Atom::App,
            Self::Lam(_) => Atom::Lam,
            Self::Pi(_, _) => Atom::Pi,
            Self::Sigma(_, _) => Atom::Sigma,
            Self::Univ => Atom::Univ,
            Self::Var(_) => Atom::Var,
            Self::Lib(_) => Atom::Lib,
            Self::Id(_, _, _) => Atom::Id,
            Self::Refl(_) => Atom::Refl,
            Self::Susp(_) => Atom::Susp,
            Self::Trunc(_) => Atom::Trunc,
            Self::PathCon(_) => Atom::PathCon,
            Self::Flat(_) => Atom::Flat,
            Self::Sharp(_) => Atom::Sharp,
            Self::Disc(_) => Atom::Disc,
            Self::Shape(_) => Atom::Shape,
            Self::Next(_) => Atom::Next,
            Self::Eventually(_) => Atom::Eventually,
        }
    }

    pub fn lib_refs(&self) -> BTreeSet<u32> {
        let mut refs = BTreeSet::new();
        self.collect_lib_refs(&mut refs);
        refs
    }

    pub fn var_refs(&self) -> BTreeSet<u32> {
        let mut refs = BTreeSet::new();
        self.collect_var_refs(&mut refs);
        refs
    }

    pub fn has_lib_pointer(&self) -> bool {
        !self.lib_refs().is_empty()
    }

    pub fn is_temporal(&self) -> bool {
        match self {
            Self::Next(_) | Self::Eventually(_) => true,
            Self::App(left, right) | Self::Pi(left, right) | Self::Sigma(left, right) => {
                left.is_temporal() || right.is_temporal()
            }
            Self::Lam(body)
            | Self::Refl(body)
            | Self::Susp(body)
            | Self::Trunc(body)
            | Self::Flat(body)
            | Self::Sharp(body)
            | Self::Disc(body)
            | Self::Shape(body) => body.is_temporal(),
            Self::Id(a, x, y) => a.is_temporal() || x.is_temporal() || y.is_temporal(),
            Self::Univ | Self::Var(_) | Self::Lib(_) | Self::PathCon(_) => false,
        }
    }

    pub fn is_modal(&self) -> bool {
        matches!(
            self,
            Self::Flat(_) | Self::Sharp(_) | Self::Disc(_) | Self::Shape(_)
        )
    }

    fn collect_lib_refs(&self, refs: &mut BTreeSet<u32>) {
        match self {
            Self::App(left, right) | Self::Pi(left, right) | Self::Sigma(left, right) => {
                left.collect_lib_refs(refs);
                right.collect_lib_refs(refs);
            }
            Self::Lam(body)
            | Self::Refl(body)
            | Self::Susp(body)
            | Self::Trunc(body)
            | Self::Flat(body)
            | Self::Sharp(body)
            | Self::Disc(body)
            | Self::Shape(body)
            | Self::Next(body)
            | Self::Eventually(body) => body.collect_lib_refs(refs),
            Self::Id(a, x, y) => {
                a.collect_lib_refs(refs);
                x.collect_lib_refs(refs);
                y.collect_lib_refs(refs);
            }
            Self::Lib(index) => {
                refs.insert(*index);
            }
            Self::Univ | Self::Var(_) | Self::PathCon(_) => {}
        }
    }

    fn collect_var_refs(&self, refs: &mut BTreeSet<u32>) {
        match self {
            Self::App(left, right) | Self::Pi(left, right) | Self::Sigma(left, right) => {
                left.collect_var_refs(refs);
                right.collect_var_refs(refs);
            }
            Self::Lam(body)
            | Self::Refl(body)
            | Self::Susp(body)
            | Self::Trunc(body)
            | Self::Flat(body)
            | Self::Sharp(body)
            | Self::Disc(body)
            | Self::Shape(body)
            | Self::Next(body)
            | Self::Eventually(body) => body.collect_var_refs(refs),
            Self::Id(a, x, y) => {
                a.collect_var_refs(refs);
                x.collect_var_refs(refs);
                y.collect_var_refs(refs);
            }
            Self::Var(index) => {
                refs.insert(*index);
            }
            Self::Univ | Self::Lib(_) | Self::PathCon(_) => {}
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
pub enum ExprNode {
    App {
        function: ExprId,
        argument: ExprId,
    },
    Lam {
        body: ExprId,
    },
    Pi {
        domain: ExprId,
        codomain: ExprId,
    },
    Sigma {
        domain: ExprId,
        codomain: ExprId,
    },
    Univ,
    Var {
        index: u32,
    },
    Lib {
        index: u32,
    },
    Id {
        ty: ExprId,
        left: ExprId,
        right: ExprId,
    },
    Refl {
        expr: ExprId,
    },
    Susp {
        expr: ExprId,
    },
    Trunc {
        expr: ExprId,
    },
    PathCon {
        dimension: u32,
    },
    Flat {
        expr: ExprId,
    },
    Sharp {
        expr: ExprId,
    },
    Disc {
        expr: ExprId,
    },
    Shape {
        expr: ExprId,
    },
    Next {
        expr: ExprId,
    },
    Eventually {
        expr: ExprId,
    },
}

impl ExprNode {
    pub const fn atom(&self) -> Atom {
        match self {
            Self::App { .. } => Atom::App,
            Self::Lam { .. } => Atom::Lam,
            Self::Pi { .. } => Atom::Pi,
            Self::Sigma { .. } => Atom::Sigma,
            Self::Univ => Atom::Univ,
            Self::Var { .. } => Atom::Var,
            Self::Lib { .. } => Atom::Lib,
            Self::Id { .. } => Atom::Id,
            Self::Refl { .. } => Atom::Refl,
            Self::Susp { .. } => Atom::Susp,
            Self::Trunc { .. } => Atom::Trunc,
            Self::PathCon { .. } => Atom::PathCon,
            Self::Flat { .. } => Atom::Flat,
            Self::Sharp { .. } => Atom::Sharp,
            Self::Disc { .. } => Atom::Disc,
            Self::Shape { .. } => Atom::Shape,
            Self::Next { .. } => Atom::Next,
            Self::Eventually { .. } => Atom::Eventually,
        }
    }

    pub fn child_ids(&self) -> Vec<ExprId> {
        match self {
            Self::App { function, argument } => vec![*function, *argument],
            Self::Lam { body }
            | Self::Refl { expr: body }
            | Self::Susp { expr: body }
            | Self::Trunc { expr: body }
            | Self::Flat { expr: body }
            | Self::Sharp { expr: body }
            | Self::Disc { expr: body }
            | Self::Shape { expr: body }
            | Self::Next { expr: body }
            | Self::Eventually { expr: body } => vec![*body],
            Self::Pi { domain, codomain } | Self::Sigma { domain, codomain } => {
                vec![*domain, *codomain]
            }
            Self::Id { ty, left, right } => vec![*ty, *left, *right],
            Self::Univ | Self::Var { .. } | Self::Lib { .. } | Self::PathCon { .. } => vec![],
        }
    }

    pub const fn index_payload(&self) -> Option<u32> {
        match self {
            Self::Var { index } | Self::Lib { index } => Some(*index),
            Self::PathCon { dimension } => Some(*dimension),
            _ => None,
        }
    }
}
