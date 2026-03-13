use crate::atom::Atom;
use crate::expr::Expr;
use crate::telescope::Telescope;

pub const fn atom_prefix_bits(atom: Atom) -> u16 {
    match atom {
        Atom::App => 2,
        Atom::Lam => 2,
        Atom::Pi => 3,
        Atom::Sigma => 4,
        Atom::Univ => 4,
        Atom::Var => 3,
        Atom::Lib => 3,
        Atom::Id => 5,
        Atom::Refl => 5,
        Atom::Susp => 5,
        Atom::Trunc => 6,
        Atom::PathCon => 6,
        Atom::Flat => 7,
        Atom::Sharp => 7,
        Atom::Disc => 7,
        Atom::Shape => 8,
        Atom::Next => 9,
        Atom::Eventually => 9,
    }
}

pub fn elias_gamma_bits(index: u32) -> u16 {
    assert!(
        index > 0,
        "Elias gamma coding is defined for positive integers"
    );
    let width = u32::BITS - index.leading_zeros();
    let total = (width * 2) - 1;
    u16::try_from(total).expect("gamma code width should fit into u16")
}

pub fn indexed_atom_bits(atom: Atom, index: u32) -> u16 {
    atom_prefix_bits(atom) + elias_gamma_bits(index)
}

pub fn expr_bit_length(expr: &Expr) -> u32 {
    match expr {
        Expr::App(function, argument) => 2 + expr_bit_length(function) + expr_bit_length(argument),
        Expr::Lam(body) => 2 + expr_bit_length(body),
        Expr::Pi(domain, codomain) => 3 + expr_bit_length(domain) + expr_bit_length(codomain),
        Expr::Sigma(domain, codomain) => 4 + expr_bit_length(domain) + expr_bit_length(codomain),
        Expr::Univ => 4,
        Expr::Var(index) | Expr::Lib(index) => 3 + u32::from(elias_gamma_bits(*index)),
        Expr::Id(a, x, y) => 5 + expr_bit_length(a) + expr_bit_length(x) + expr_bit_length(y),
        Expr::Refl(body) | Expr::Susp(body) => 5 + expr_bit_length(body),
        Expr::Trunc(body) => 6 + expr_bit_length(body),
        Expr::PathCon(dimension) => 6 + u32::from(elias_gamma_bits(*dimension)),
        Expr::Flat(body) | Expr::Sharp(body) | Expr::Disc(body) => 7 + expr_bit_length(body),
        Expr::Shape(body) => 8 + expr_bit_length(body),
        Expr::Next(body) | Expr::Eventually(body) => 9 + expr_bit_length(body),
    }
}

pub fn telescope_bit_cost(telescope: &Telescope) -> u32 {
    telescope
        .clauses
        .iter()
        .map(|clause| expr_bit_length(&clause.expr))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{atom_prefix_bits, elias_gamma_bits, expr_bit_length, indexed_atom_bits};
    use crate::atom::Atom;
    use crate::expr::Expr;

    #[test]
    fn frozen_prefix_bits_match_the_skill_notes() {
        assert_eq!(atom_prefix_bits(Atom::App), 2);
        assert_eq!(atom_prefix_bits(Atom::Susp), 5);
        assert_eq!(atom_prefix_bits(Atom::Eventually), 9);
    }

    #[test]
    fn indexed_atoms_add_gamma_costs() {
        assert_eq!(elias_gamma_bits(1), 1);
        assert_eq!(indexed_atom_bits(Atom::Var, 1), 4);
        assert_eq!(indexed_atom_bits(Atom::PathCon, 3), 9);
    }

    #[test]
    fn recursive_expr_bit_length_matches_donor_examples() {
        let univ_in_univ = Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)));
        assert_eq!(expr_bit_length(&Expr::Univ), 4);
        assert_eq!(expr_bit_length(&univ_in_univ), 10);
    }
}
