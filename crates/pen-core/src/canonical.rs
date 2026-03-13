use crate::clause::ClauseRec;
use crate::expr::Expr;
use crate::telescope::Telescope;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CanonKey(pub String);

pub fn canonicalize_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::App(function, argument) => Expr::App(
            Box::new(canonicalize_expr(function)),
            Box::new(canonicalize_expr(argument)),
        ),
        Expr::Lam(body) => Expr::Lam(Box::new(canonicalize_expr(body))),
        Expr::Pi(domain, codomain) => Expr::Pi(
            Box::new(canonicalize_expr(domain)),
            Box::new(canonicalize_expr(codomain)),
        ),
        Expr::Sigma(domain, codomain) => Expr::Sigma(
            Box::new(canonicalize_expr(domain)),
            Box::new(canonicalize_expr(codomain)),
        ),
        Expr::Id(a, x, y) => Expr::Id(
            Box::new(canonicalize_expr(a)),
            Box::new(canonicalize_expr(x)),
            Box::new(canonicalize_expr(y)),
        ),
        Expr::Refl(body) => Expr::Refl(Box::new(canonicalize_expr(body))),
        Expr::Susp(body) => Expr::Susp(Box::new(canonicalize_expr(body))),
        Expr::Trunc(body) => Expr::Trunc(Box::new(canonicalize_expr(body))),
        Expr::Flat(body) => Expr::Flat(Box::new(canonicalize_expr(body))),
        Expr::Sharp(body) => Expr::Sharp(Box::new(canonicalize_expr(body))),
        Expr::Disc(body) => Expr::Disc(Box::new(canonicalize_expr(body))),
        Expr::Shape(body) => Expr::Shape(Box::new(canonicalize_expr(body))),
        Expr::Next(body) => Expr::Next(Box::new(canonicalize_expr(body))),
        Expr::Eventually(body) => Expr::Eventually(Box::new(canonicalize_expr(body))),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => expr.clone(),
    }
}

pub fn canonicalize_telescope(telescope: &Telescope) -> Telescope {
    Telescope {
        clauses: telescope
            .clauses
            .iter()
            .map(|clause| ClauseRec {
                role: clause.role,
                expr: canonicalize_expr(&clause.expr),
            })
            .collect(),
    }
}

pub fn canonical_key_expr(expr: &Expr) -> CanonKey {
    CanonKey(fnv1a_hex(&encode_expr(&canonicalize_expr(expr))))
}

pub fn canonical_key_telescope(telescope: &Telescope) -> CanonKey {
    let encoded = canonicalize_telescope(telescope)
        .clauses
        .iter()
        .map(|clause| encode_expr(&clause.expr))
        .collect::<Vec<_>>()
        .join("");
    CanonKey(fnv1a_hex(&encoded))
}

fn encode_expr(expr: &Expr) -> String {
    match expr {
        Expr::App(function, argument) => {
            format!("A({})({})", encode_expr(function), encode_expr(argument))
        }
        Expr::Lam(body) => format!("L({})", encode_expr(body)),
        Expr::Pi(domain, codomain) => {
            format!("P({})({})", encode_expr(domain), encode_expr(codomain))
        }
        Expr::Sigma(domain, codomain) => {
            format!("S({})({})", encode_expr(domain), encode_expr(codomain))
        }
        Expr::Univ => "U".to_owned(),
        Expr::Var(index) => format!("V{index}"),
        Expr::Lib(index) => format!("B{index}"),
        Expr::Id(a, x, y) => format!(
            "I({})({})({})",
            encode_expr(a),
            encode_expr(x),
            encode_expr(y)
        ),
        Expr::Refl(body) => format!("R({})", encode_expr(body)),
        Expr::Susp(body) => format!("Q({})", encode_expr(body)),
        Expr::Trunc(body) => format!("T({})", encode_expr(body)),
        Expr::PathCon(dimension) => format!("C{dimension}"),
        Expr::Flat(body) => format!("F({})", encode_expr(body)),
        Expr::Sharp(body) => format!("H({})", encode_expr(body)),
        Expr::Disc(body) => format!("D({})", encode_expr(body)),
        Expr::Shape(body) => format!("G({})", encode_expr(body)),
        Expr::Next(body) => format!("N({})", encode_expr(body)),
        Expr::Eventually(body) => format!("E({})", encode_expr(body)),
    }
}

fn fnv1a_hex(input: &str) -> String {
    let mut hash: u32 = 2_166_136_261;
    for byte in input.bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(16_777_619);
    }
    format!("{hash:08x}")
}

#[cfg(test)]
mod tests {
    use super::{canonical_key_expr, canonical_key_telescope};
    use crate::expr::Expr;
    use crate::telescope::Telescope;

    #[test]
    fn canonicalization_is_structural_and_stable() {
        let expr = Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)));
        assert_eq!(canonical_key_expr(&expr).0, canonical_key_expr(&expr).0);
    }

    #[test]
    fn telescope_keys_are_deterministic() {
        let telescope = Telescope::reference(5);
        assert_eq!(
            canonical_key_telescope(&telescope).0,
            canonical_key_telescope(&telescope).0
        );
    }
}
