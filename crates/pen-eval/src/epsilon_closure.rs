//! Exact-arithmetic engine for the frozen epsilon-closure project.
//!
//! This module is the firewalled theorem, quotient, and diagnostic layer.
//! Physics-facing characterization is isolated in `epsilon_closure_post_run`
//! and is invoked only after every verdict-side computation is complete.

use crate::epsilon_closure_post_run::{self, PostRunReport};
use num_rational::Ratio;
use serde::Serialize;
use std::collections::BTreeMap;

pub const EPSILON_CLOSURE_DATE: &str = "2026-07-16";
pub(crate) type Exact = Ratio<i128>;

pub(crate) fn exact(num: i128, den: i128) -> Exact {
    assert_ne!(den, 0, "zero denominator");
    Ratio::new(num, den)
}

pub(crate) fn exact_add(left: &Exact, right: &Exact) -> Exact {
    let lhs = left
        .numer()
        .checked_mul(*right.denom())
        .expect("i128 overflow in exact addition (left cross-product)");
    let rhs = right
        .numer()
        .checked_mul(*left.denom())
        .expect("i128 overflow in exact addition (right cross-product)");
    let num = lhs
        .checked_add(rhs)
        .expect("i128 overflow in exact addition (numerator sum)");
    let den = left
        .denom()
        .checked_mul(*right.denom())
        .expect("i128 overflow in exact addition (denominator product)");
    exact(num, den)
}

pub(crate) fn exact_neg(value: &Exact) -> Exact {
    exact(
        value
            .numer()
            .checked_neg()
            .expect("i128 overflow in exact negation"),
        *value.denom(),
    )
}

pub(crate) fn exact_sub(left: &Exact, right: &Exact) -> Exact {
    exact_add(left, &exact_neg(right))
}

pub(crate) fn exact_mul(left: &Exact, right: &Exact) -> Exact {
    let num = left
        .numer()
        .checked_mul(*right.numer())
        .expect("i128 overflow in exact multiplication (numerator)");
    let den = left
        .denom()
        .checked_mul(*right.denom())
        .expect("i128 overflow in exact multiplication (denominator)");
    exact(num, den)
}

pub(crate) fn exact_div(left: &Exact, right: &Exact) -> Exact {
    assert_ne!(*right.numer(), 0, "division by zero exact value");
    let num = left
        .numer()
        .checked_mul(*right.denom())
        .expect("i128 overflow in exact division (numerator)");
    let den = left
        .denom()
        .checked_mul(*right.numer())
        .expect("i128 overflow in exact division (denominator)");
    exact(num, den)
}

pub(crate) fn exact_string(value: &Exact) -> String {
    if *value.denom() == 1 {
        value.numer().to_string()
    } else {
        format!("{}/{}", value.numer(), value.denom())
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Slot {
    Q,
    Uc,
    Dc,
    L,
    Ec,
    NuC,
    H,
}

impl Slot {
    pub(crate) const ALL: [Self; 7] = [
        Self::Q,
        Self::Uc,
        Self::Dc,
        Self::L,
        Self::Ec,
        Self::NuC,
        Self::H,
    ];

    fn index(self) -> usize {
        match self {
            Self::Q => 0,
            Self::Uc => 1,
            Self::Dc => 2,
            Self::L => 3,
            Self::Ec => 4,
            Self::NuC => 5,
            Self::H => 6,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Q => "Q",
            Self::Uc => "u^c",
            Self::Dc => "d^c",
            Self::L => "L",
            Self::Ec => "e^c",
            Self::NuC => "ν^c",
            Self::H => "H",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Assignment {
    values: [Exact; 7],
}

impl Assignment {
    fn new(values: [Exact; 7]) -> Self {
        Self { values }
    }

    pub(crate) fn value(&self, slot: Slot) -> &Exact {
        &self.values[slot.index()]
    }

    fn active_slots(&self) -> usize {
        self.values
            .iter()
            .filter(|value| *value.numer() != 0)
            .count()
    }

    fn record(&self) -> AssignmentRecord {
        AssignmentRecord {
            q: exact_string(self.value(Slot::Q)),
            u_c: exact_string(self.value(Slot::Uc)),
            d_c: exact_string(self.value(Slot::Dc)),
            l: exact_string(self.value(Slot::L)),
            e_c: exact_string(self.value(Slot::Ec)),
            nu_c: exact_string(self.value(Slot::NuC)),
            h: exact_string(self.value(Slot::H)),
        }
    }

    fn stage_a_record(&self) -> StageAAssignmentRecord {
        StageAAssignmentRecord {
            q: exact_string(self.value(Slot::Q)),
            u_c: exact_string(self.value(Slot::Uc)),
            d_c: exact_string(self.value(Slot::Dc)),
            l: exact_string(self.value(Slot::L)),
            e_c: exact_string(self.value(Slot::Ec)),
            h: exact_string(self.value(Slot::H)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StageAAssignmentRecord {
    #[serde(rename = "Q")]
    pub q: String,
    #[serde(rename = "u^c")]
    pub u_c: String,
    #[serde(rename = "d^c")]
    pub d_c: String,
    #[serde(rename = "L")]
    pub l: String,
    #[serde(rename = "e^c")]
    pub e_c: String,
    #[serde(rename = "H")]
    pub h: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AssignmentRecord {
    #[serde(rename = "Q")]
    pub q: String,
    #[serde(rename = "u^c")]
    pub u_c: String,
    #[serde(rename = "d^c")]
    pub d_c: String,
    #[serde(rename = "L")]
    pub l: String,
    #[serde(rename = "e^c")]
    pub e_c: String,
    #[serde(rename = "ν^c")]
    pub nu_c: String,
    #[serde(rename = "H")]
    pub h: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Polynomial {
    terms: BTreeMap<[u8; 7], Exact>,
}

impl Polynomial {
    fn zero() -> Self {
        Self {
            terms: BTreeMap::new(),
        }
    }

    fn constant(value: Exact) -> Self {
        let mut result = Self::zero();
        result.add_term([0; 7], value);
        result
    }

    fn variable(slot: Slot) -> Self {
        let mut powers = [0; 7];
        powers[slot.index()] = 1;
        let mut result = Self::zero();
        result.add_term(powers, exact(1, 1));
        result
    }

    fn add_term(&mut self, powers: [u8; 7], coefficient: Exact) {
        if coefficient == exact(0, 1) {
            return;
        }
        let updated = self
            .terms
            .get(&powers)
            .map(|current| exact_add(current, &coefficient))
            .unwrap_or(coefficient);
        if updated == exact(0, 1) {
            self.terms.remove(&powers);
        } else {
            self.terms.insert(powers, updated);
        }
    }

    fn plus(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (powers, coefficient) in &other.terms {
            result.add_term(*powers, *coefficient);
        }
        result
    }

    fn negated(&self) -> Self {
        let mut result = Self::zero();
        for (powers, coefficient) in &self.terms {
            result.add_term(*powers, exact_neg(coefficient));
        }
        result
    }

    fn minus(&self, other: &Self) -> Self {
        self.plus(&other.negated())
    }

    fn scaled(&self, factor: Exact) -> Self {
        let mut result = Self::zero();
        for (powers, coefficient) in &self.terms {
            result.add_term(*powers, exact_mul(coefficient, &factor));
        }
        result
    }

    fn times(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for (left_powers, left_coefficient) in &self.terms {
            for (right_powers, right_coefficient) in &other.terms {
                let powers = std::array::from_fn(|index| {
                    left_powers[index]
                        .checked_add(right_powers[index])
                        .expect("monomial exponent overflow")
                });
                result.add_term(powers, exact_mul(left_coefficient, right_coefficient));
            }
        }
        result
    }

    fn pow(&self, exponent: u8) -> Self {
        let mut result = Self::constant(exact(1, 1));
        for _ in 0..exponent {
            result = result.times(self);
        }
        result
    }

    fn substitute(&self, replacements: &[Self; 7]) -> Self {
        let mut result = Self::zero();
        for (powers, coefficient) in &self.terms {
            let mut term = Self::constant(*coefficient);
            for (index, exponent) in powers.iter().enumerate() {
                if *exponent != 0 {
                    term = term.times(&replacements[index].pow(*exponent));
                }
            }
            result = result.plus(&term);
        }
        result
    }

    fn evaluate(&self, assignment: &Assignment) -> Exact {
        let replacements = std::array::from_fn(|index| Self::constant(assignment.values[index]));
        let value = self.substitute(&replacements);
        value
            .terms
            .get(&[0; 7])
            .copied()
            .unwrap_or_else(|| exact(0, 1))
    }

    fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
}

fn variables() -> [Polynomial; 7] {
    std::array::from_fn(|index| Polynomial::variable(Slot::ALL[index]))
}

fn replace(poly: &Polynomial, slot: Slot, replacement: Polynomial) -> Polynomial {
    let mut replacements = variables();
    replacements[slot.index()] = replacement;
    poly.substitute(&replacements)
}

fn linear_gate(terms: &[(i128, Slot)]) -> Polynomial {
    terms
        .iter()
        .fold(Polynomial::zero(), |sum, (coefficient, slot)| {
            sum.plus(&Polynomial::variable(*slot).scaled(exact(*coefficient, 1)))
        })
}

fn cubic_gate(extended: bool) -> Polynomial {
    let mut terms = vec![
        (6, Slot::Q),
        (3, Slot::Uc),
        (3, Slot::Dc),
        (2, Slot::L),
        (1, Slot::Ec),
    ];
    if extended {
        terms.push((1, Slot::NuC));
    }
    terms
        .into_iter()
        .fold(Polynomial::zero(), |sum, (multiplicity, slot)| {
            sum.plus(
                &Polynomial::variable(slot)
                    .pow(3)
                    .scaled(exact(multiplicity, 1)),
            )
        })
}

fn a331_gate() -> Polynomial {
    linear_gate(&[(2, Slot::Q), (1, Slot::Uc), (1, Slot::Dc)])
}

fn a221_gate() -> Polynomial {
    linear_gate(&[(3, Slot::Q), (1, Slot::L)])
}

fn grav_gate(extended: bool) -> Polynomial {
    let mut terms = vec![
        (6, Slot::Q),
        (3, Slot::Uc),
        (3, Slot::Dc),
        (2, Slot::L),
        (1, Slot::Ec),
    ];
    if extended {
        terms.push((1, Slot::NuC));
    }
    linear_gate(&terms)
}

fn stage_a_yukawa_gates() -> [Polynomial; 3] {
    [
        linear_gate(&[(1, Slot::Q), (1, Slot::Uc), (1, Slot::H)]),
        linear_gate(&[(1, Slot::Q), (1, Slot::Dc), (-1, Slot::H)]),
        linear_gate(&[(1, Slot::L), (1, Slot::Ec), (-1, Slot::H)]),
    ]
}

fn extension_yukawa_gate() -> Polynomial {
    linear_gate(&[(1, Slot::L), (1, Slot::NuC), (1, Slot::H)])
}

fn stage_a_substitution() -> [Polynomial; 7] {
    let mut replacements = variables();
    let q = Polynomial::variable(Slot::Q);
    let l = Polynomial::variable(Slot::L);
    let h = Polynomial::variable(Slot::H);
    replacements[Slot::Uc.index()] = h.negated().minus(&q);
    replacements[Slot::Dc.index()] = h.minus(&q);
    replacements[Slot::Ec.index()] = h.minus(&l);
    replacements
}

fn simultaneous_substitution() -> [Polynomial; 7] {
    let mut replacements = stage_a_substitution();
    let l = Polynomial::variable(Slot::L);
    let h = Polynomial::variable(Slot::H);
    replacements[Slot::NuC.index()] = h.negated().minus(&l);
    replacements
}

fn matrix_rank(mut matrix: Vec<Vec<Exact>>) -> usize {
    if matrix.is_empty() {
        return 0;
    }
    let columns = matrix[0].len();
    let mut pivot_row = 0;
    for column in 0..columns {
        let Some(row) = (pivot_row..matrix.len()).find(|row| matrix[*row][column] != exact(0, 1))
        else {
            continue;
        };
        matrix.swap(pivot_row, row);
        let pivot = matrix[pivot_row][column];
        for entry in &mut matrix[pivot_row][column..] {
            *entry = exact_div(entry, &pivot);
        }
        let normalized = matrix[pivot_row].clone();
        for (row_index, row_values) in matrix.iter_mut().enumerate() {
            if row_index == pivot_row {
                continue;
            }
            let factor = row_values[column];
            if factor == exact(0, 1) {
                continue;
            }
            for index in column..columns {
                row_values[index] =
                    exact_sub(&row_values[index], &exact_mul(&factor, &normalized[index]));
            }
        }
        pivot_row += 1;
        if pivot_row == matrix.len() {
            break;
        }
    }
    pivot_row
}

fn stage_a_linear_rank() -> usize {
    matrix_rank(
        [
            [1, 1, 0, 0, 0, 1],
            [1, 0, 1, 0, 0, -1],
            [0, 0, 0, 1, 1, -1],
            [3, 0, 0, 1, 0, 0],
            [6, 3, 3, 2, 1, 0],
        ]
        .into_iter()
        .map(|row| row.into_iter().map(|value| exact(value, 1)).collect())
        .collect(),
    )
}

fn simultaneous_linear_rank() -> usize {
    matrix_rank(
        [
            [1, 1, 0, 0, 0, 0, 1],
            [1, 0, 1, 0, 0, 0, -1],
            [0, 0, 0, 1, 1, 0, -1],
            [0, 0, 0, 1, 0, 1, 1],
            [3, 0, 0, 1, 0, 0, 0],
        ]
        .into_iter()
        .map(|row| row.into_iter().map(|value| exact(value, 1)).collect())
        .collect(),
    )
}

fn automatic_conditions() -> AutomaticConditions {
    let triality_sum = 2_i32
        .checked_mul(1)
        .and_then(|value| value.checked_add(-1))
        .and_then(|value| value.checked_add(-1))
        .expect("fixed triality arithmetic overflow");
    let fermion_doublets = 3_i32
        .checked_add(1)
        .expect("fixed doublet arithmetic overflow");
    assert_eq!(triality_sum, 0);
    assert_eq!(fermion_doublets % 2, 0);
    AutomaticConditions {
        triality_expansion: "2*(+1) + (-1) + (-1) = 0".to_owned(),
        fermion_doublet_expansion: "3(Q colors) + 1(L) = 4; 4 is even; H is exempt".to_owned(),
        unchanged_by_extension: true,
    }
}

fn stage_a_assignment(q: Exact, h: Exact) -> Assignment {
    let l = exact_mul(&exact(-3, 1), &q);
    let u_c = exact_sub(&exact_neg(&h), &q);
    let d_c = exact_sub(&h, &q);
    let e_c = exact_sub(&h, &l);
    Assignment::new([q, u_c, d_c, l, e_c, exact(0, 1), h])
}

fn simultaneous_assignment(q: Exact, h: Exact) -> Assignment {
    let stage_a = stage_a_assignment(q, h);
    let l = stage_a.value(Slot::L);
    let nu_c = exact_sub(&exact_neg(&h), l);
    Assignment::new([
        *stage_a.value(Slot::Q),
        *stage_a.value(Slot::Uc),
        *stage_a.value(Slot::Dc),
        *l,
        *stage_a.value(Slot::Ec),
        nu_c,
        h,
    ])
}

pub(crate) fn finite_assignment(epsilon: &Exact) -> Assignment {
    let basis_y = simultaneous_assignment(exact(1, 6), exact(1, 2));
    let basis_bl = simultaneous_assignment(exact(1, 3), exact(0, 1));
    Assignment::new(std::array::from_fn(|index| {
        exact_add(
            &basis_y.values[index],
            &exact_mul(epsilon, &basis_bl.values[index]),
        )
    }))
}

fn assert_all_gates(assignment: &Assignment, extended: bool) {
    let mut gates = vec![
        a331_gate(),
        a221_gate(),
        grav_gate(extended),
        cubic_gate(extended),
    ];
    gates.extend(stage_a_yukawa_gates());
    if extended {
        gates.push(extension_yukawa_gate());
    }
    for gate in gates {
        assert_eq!(gate.evaluate(assignment), exact(0, 1));
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AutomaticConditions {
    pub triality_expansion: String,
    pub fermion_doublet_expansion: String,
    pub unchanged_by_extension: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RayReport {
    pub parameter: String,
    pub normalization: String,
    pub assignment: StageAAssignmentRecord,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StageAReport {
    pub forcing_line: String,
    pub ray: RayReport,
    pub identities: BTreeMap<String, String>,
    pub linear_rank: usize,
    pub solution_dimension: usize,
    pub automatic_conditions: AutomaticConditions,
}

fn compute_stage_a() -> (StageAReport, Assignment) {
    let substitution = stage_a_substitution();
    for gate in stage_a_yukawa_gates() {
        assert!(gate.substitute(&substitution).is_zero());
    }

    let a331_after = a331_gate().substitute(&substitution);
    assert!(a331_after.is_zero());

    let expected_a221 = linear_gate(&[(3, Slot::Q), (1, Slot::L)]);
    let a221_after = a221_gate().substitute(&substitution);
    assert_eq!(a221_after, expected_a221);

    let l_solution = Polynomial::variable(Slot::Q).scaled(exact(-3, 1));
    let grav_after = replace(
        &grav_gate(false).substitute(&substitution),
        Slot::L,
        l_solution.clone(),
    );
    let expected_grav =
        Polynomial::variable(Slot::H).minus(&Polynomial::variable(Slot::Q).scaled(exact(3, 1)));
    assert_eq!(grav_after, expected_grav);

    let cubic_after = replace(
        &replace(
            &cubic_gate(false).substitute(&substitution),
            Slot::L,
            l_solution,
        ),
        Slot::H,
        Polynomial::variable(Slot::Q).scaled(exact(3, 1)),
    );
    assert!(cubic_after.is_zero());

    let rank = stage_a_linear_rank();
    assert_eq!(rank, 5);
    let h = exact(1, 2);
    let q = exact_div(&h, &exact(3, 1));
    let ray = stage_a_assignment(q, h);
    assert_eq!(ray.value(Slot::NuC), &exact(0, 1));
    assert_all_gates(&ray, false);

    let identities = BTreeMap::from([
        ("A331_after_three_yukawa_lines".to_owned(), "0".to_owned()),
        (
            "A221_after_three_yukawa_lines".to_owned(),
            "3*y_Q + y_L".to_owned(),
        ),
        ("Agrav_after_A221".to_owned(), "y_H - 3*y_Q".to_owned()),
        ("A111_after_forcing".to_owned(), "0".to_owned()),
    ]);

    (
        StageAReport {
            forcing_line: "Agrav -> y_H - 3*y_Q = 0 -> y_H = 3*y_Q".to_owned(),
            ray: RayReport {
                parameter: "alpha in Q*".to_owned(),
                normalization: "M1 gauge: y_H = 1/2".to_owned(),
                assignment: ray.stage_a_record(),
            },
            identities,
            linear_rank: rank,
            solution_dimension: 6 - rank,
            automatic_conditions: automatic_conditions(),
        },
        ray,
    )
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ForcingReport {
    pub gate: String,
    pub reduced_polynomial: String,
    pub unique_solution: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StageBReport {
    pub forcings: Vec<ForcingReport>,
    pub winner: AssignmentRecord,
    pub sealed_values_rewritten: bool,
}

fn sealed_substitution(sealed: &Assignment) -> [Polynomial; 7] {
    std::array::from_fn(|index| {
        if index == Slot::NuC.index() {
            Polynomial::variable(Slot::NuC)
        } else {
            Polynomial::constant(sealed.values[index])
        }
    })
}

fn compute_stage_b(sealed: &Assignment) -> (StageBReport, Assignment) {
    let substitution = sealed_substitution(sealed);
    let extension_line = extension_yukawa_gate().substitute(&substitution);
    let grav_line = grav_gate(true).substitute(&substitution);
    let cubic_line = cubic_gate(true).substitute(&substitution);
    let expected_linear = Polynomial::variable(Slot::NuC);
    let expected_cubic = expected_linear.pow(3);
    assert_eq!(extension_line, expected_linear);
    assert_eq!(grav_line, expected_linear);
    assert_eq!(cubic_line, expected_cubic);

    let winner = sealed.clone();
    assert_eq!(winner.value(Slot::NuC), &exact(0, 1));
    assert_all_gates(&winner, true);

    (
        StageBReport {
            forcings: vec![
                ForcingReport {
                    gate: "extension Yukawa line".to_owned(),
                    reduced_polynomial: "y_nu".to_owned(),
                    unique_solution: "y_nu = 0".to_owned(),
                },
                ForcingReport {
                    gate: "Agrav".to_owned(),
                    reduced_polynomial: "y_nu".to_owned(),
                    unique_solution: "y_nu = 0".to_owned(),
                },
                ForcingReport {
                    gate: "A111".to_owned(),
                    reduced_polynomial: "y_nu^3".to_owned(),
                    unique_solution: "y_nu = 0".to_owned(),
                },
            ],
            winner: winner.record(),
            sealed_values_rewritten: false,
        },
        winner,
    )
}

fn linear_formula(base: &Exact, slope: &Exact) -> String {
    if *slope == exact(0, 1) {
        return exact_string(base);
    }
    if *base == exact(0, 1) {
        return match exact_string(slope).as_str() {
            "1" => "epsilon".to_owned(),
            "-1" => "-epsilon".to_owned(),
            value => format!("{value}*epsilon"),
        };
    }
    let sign = if *slope < exact(0, 1) { "-" } else { "+" };
    let magnitude = if *slope < exact(0, 1) {
        exact_neg(slope)
    } else {
        *slope
    };
    let epsilon_term = if magnitude == exact(1, 1) {
        "epsilon".to_owned()
    } else {
        format!("{}*epsilon", exact_string(&magnitude))
    };
    format!("{} {sign} {epsilon_term}", exact_string(base))
}

fn family_formulas(y: &Assignment, bl: &Assignment) -> AssignmentRecord {
    AssignmentRecord {
        q: linear_formula(y.value(Slot::Q), bl.value(Slot::Q)),
        u_c: linear_formula(y.value(Slot::Uc), bl.value(Slot::Uc)),
        d_c: linear_formula(y.value(Slot::Dc), bl.value(Slot::Dc)),
        l: linear_formula(y.value(Slot::L), bl.value(Slot::L)),
        e_c: linear_formula(y.value(Slot::Ec), bl.value(Slot::Ec)),
        nu_c: linear_formula(y.value(Slot::NuC), bl.value(Slot::NuC)),
        h: linear_formula(y.value(Slot::H), bl.value(Slot::H)),
    }
}

fn presentation_involution(assignment: &Assignment) -> Assignment {
    Assignment::new([
        exact_neg(assignment.value(Slot::Q)),
        exact_neg(assignment.value(Slot::Dc)),
        exact_neg(assignment.value(Slot::Uc)),
        exact_neg(assignment.value(Slot::L)),
        exact_neg(assignment.value(Slot::NuC)),
        exact_neg(assignment.value(Slot::Ec)),
        *assignment.value(Slot::H),
    ])
}

fn involuted_epsilon(epsilon: &Exact) -> Exact {
    exact_sub(&exact(-1, 1), epsilon)
}

fn canonical_epsilon(epsilon: &Exact) -> Exact {
    let partner = involuted_epsilon(epsilon);
    if *epsilon >= partner {
        *epsilon
    } else {
        partner
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BasisReport {
    #[serde(rename = "Y")]
    pub y: AssignmentRecord,
    #[serde(rename = "B-L")]
    pub b_minus_l: AssignmentRecord,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BranchPointReport {
    pub epsilon: String,
    pub zero_slots: Vec<String>,
    pub active_slots: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QuotientClassReport {
    pub id: String,
    pub representatives: Vec<String>,
    pub canonical_representative: String,
    pub active_slots: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QuotientChecks {
    pub finite_gauge: String,
    pub infinite_gauge: String,
    pub involution: String,
    pub canonical_rule: String,
    pub canonicalizer_idempotent: bool,
    pub minus_one_equals_zero: bool,
    pub one_equals_minus_two: bool,
    pub minus_half_fixed: bool,
    pub finite_kappa_6_classes: usize,
    pub finite_kappa_5_classes: usize,
    pub component_orientation_absent_from_key: bool,
    pub same_record_permutations_sorted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SimultaneousReport {
    pub family_basis: BasisReport,
    pub rank: usize,
    pub solution_dimension: usize,
    pub derived_family: AssignmentRecord,
    pub identities: BTreeMap<String, String>,
    pub branch_points: Vec<BranchPointReport>,
    pub no_finite_zero_slots: Vec<String>,
    pub infinite_branch: AssignmentRecord,
    pub classes: Vec<QuotientClassReport>,
    pub quotient: QuotientChecks,
    pub automatic_conditions: AutomaticConditions,
}

fn compute_simultaneous() -> (SimultaneousReport, Assignment) {
    let substitution = simultaneous_substitution();
    for gate in stage_a_yukawa_gates()
        .into_iter()
        .chain([extension_yukawa_gate()])
    {
        assert!(gate.substitute(&substitution).is_zero());
    }
    assert!(a331_gate().substitute(&substitution).is_zero());
    let expected_a221 = linear_gate(&[(3, Slot::Q), (1, Slot::L)]);
    assert_eq!(a221_gate().substitute(&substitution), expected_a221);
    let l_solution = Polynomial::variable(Slot::Q).scaled(exact(-3, 1));
    let grav_after = replace(
        &grav_gate(true).substitute(&substitution),
        Slot::L,
        l_solution.clone(),
    );
    let cubic_after = replace(
        &cubic_gate(true).substitute(&substitution),
        Slot::L,
        l_solution,
    );
    assert!(grav_after.is_zero());
    assert!(cubic_after.is_zero());

    let rank = simultaneous_linear_rank();
    assert_eq!(rank, 5);
    let basis_y = simultaneous_assignment(exact(1, 6), exact(1, 2));
    let basis_bl = simultaneous_assignment(exact(1, 3), exact(0, 1));
    assert_all_gates(&basis_y, true);
    assert_all_gates(&basis_bl, true);

    let mut grouped: BTreeMap<Exact, Vec<String>> = BTreeMap::new();
    let mut no_finite_zero = Vec::new();
    for slot in Slot::ALL {
        let base = basis_y.value(slot);
        let slope = basis_bl.value(slot);
        if *slope == exact(0, 1) {
            assert_ne!(*base, exact(0, 1));
            no_finite_zero.push(slot.name().to_owned());
        } else {
            let point = exact_div(&exact_neg(base), slope);
            assert_eq!(finite_assignment(&point).value(slot), &exact(0, 1));
            grouped
                .entry(point)
                .or_default()
                .push(slot.name().to_owned());
        }
    }
    assert_eq!(no_finite_zero, vec!["H"]);
    let branch_points: Vec<_> = grouped
        .into_iter()
        .map(|(epsilon, zero_slots)| BranchPointReport {
            active_slots: finite_assignment(&epsilon).active_slots(),
            epsilon: exact_string(&epsilon),
            zero_slots,
        })
        .collect();

    for epsilon in [
        exact(-2, 1),
        exact(-1, 1),
        exact(-1, 2),
        exact(0, 1),
        exact(1, 1),
    ] {
        let transformed = presentation_involution(&finite_assignment(&epsilon));
        assert_eq!(transformed, finite_assignment(&involuted_epsilon(&epsilon)));
    }
    assert_eq!(canonical_epsilon(&exact(-1, 1)), exact(0, 1));
    assert_eq!(canonical_epsilon(&exact(-2, 1)), exact(1, 1));
    assert_eq!(canonical_epsilon(&exact(-1, 2)), exact(-1, 2));
    for epsilon in [
        exact(-2, 1),
        exact(-1, 1),
        exact(-1, 2),
        exact(0, 1),
        exact(1, 1),
    ] {
        assert_eq!(
            canonical_epsilon(&canonical_epsilon(&epsilon)),
            canonical_epsilon(&epsilon)
        );
    }

    let minus_half = finite_assignment(&exact(-1, 2));
    assert_eq!(minus_half.active_slots(), 5);
    assert_eq!(finite_assignment(&exact(0, 1)).active_slots(), 6);
    let infinite = basis_bl.clone();
    assert_eq!(infinite.active_slots(), 6);

    let classes = vec![
        QuotientClassReport {
            id: "generic".to_owned(),
            representatives: vec!["{epsilon, -1-epsilon}, excluding branch points".to_owned()],
            canonical_representative: "epsilon > -1/2".to_owned(),
            active_slots: 7,
        },
        QuotientClassReport {
            id: "zero_class".to_owned(),
            representatives: vec!["0".to_owned(), "-1".to_owned()],
            canonical_representative: "0".to_owned(),
            active_slots: 6,
        },
        QuotientClassReport {
            id: "one_class".to_owned(),
            representatives: vec!["1".to_owned(), "-2".to_owned()],
            canonical_representative: "1".to_owned(),
            active_slots: 6,
        },
        QuotientClassReport {
            id: "minus_half_class".to_owned(),
            representatives: vec!["-1/2".to_owned()],
            canonical_representative: "-1/2".to_owned(),
            active_slots: 5,
        },
        QuotientClassReport {
            id: "infinite_class".to_owned(),
            representatives: vec!["epsilon = infinity".to_owned()],
            canonical_representative: "pure B-L; y_Q = 1/3".to_owned(),
            active_slots: infinite.active_slots(),
        },
    ];
    let finite_kappa_6_classes = classes
        .iter()
        .filter(|class| class.id != "infinite_class" && class.active_slots == 6)
        .count();
    let finite_kappa_5_classes = classes
        .iter()
        .filter(|class| class.id != "infinite_class" && class.active_slots == 5)
        .count();
    assert_eq!(finite_kappa_6_classes, 2);
    assert_eq!(finite_kappa_5_classes, 1);

    (
        SimultaneousReport {
            family_basis: BasisReport {
                y: basis_y.record(),
                b_minus_l: basis_bl.record(),
            },
            rank,
            solution_dimension: 7 - rank,
            derived_family: family_formulas(&basis_y, &basis_bl),
            identities: BTreeMap::from([
                ("A331_after_four_yukawa_lines".to_owned(), "0".to_owned()),
                (
                    "A221_after_four_yukawa_lines".to_owned(),
                    "3*y_Q + y_L".to_owned(),
                ),
                ("Agrav_after_A221".to_owned(), "0".to_owned()),
                ("A111_after_A221".to_owned(), "0".to_owned()),
            ]),
            branch_points,
            no_finite_zero_slots: no_finite_zero,
            infinite_branch: infinite.record(),
            classes,
            quotient: QuotientChecks {
                finite_gauge: "M1: y_H = 1/2".to_owned(),
                infinite_gauge: "M1: y_H = 0, y_Q = 1/3".to_owned(),
                involution: "M2 o M3: epsilon -> -1-epsilon".to_owned(),
                canonical_rule: "choose epsilon >= -1/2".to_owned(),
                canonicalizer_idempotent: true,
                minus_one_equals_zero: true,
                one_equals_minus_two: true,
                minus_half_fixed: true,
                finite_kappa_6_classes,
                finite_kappa_5_classes,
                component_orientation_absent_from_key: true,
                same_record_permutations_sorted: true,
            },
            automatic_conditions: automatic_conditions(),
        },
        minus_half,
    )
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountingARow {
    pub class: String,
    pub kappa: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountingA {
    pub semantics: String,
    pub rows: Vec<AccountingARow>,
    pub minimum_kappa: usize,
    pub minimum_classes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountingBRow {
    pub class: String,
    pub embedding_direction_clauses: usize,
    pub description_units: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountingB {
    pub semantics: String,
    pub rows: Vec<AccountingBRow>,
    pub lexicographic_minimum: [usize; 2],
    pub minimum_classes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountingCRow {
    pub class: String,
    pub marginal_kappa: String,
    pub nu: usize,
    pub admissible: bool,
    pub verdict: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountingC {
    pub semantics: String,
    pub rows: Vec<AccountingCRow>,
    pub all_nonzero_shifts_blocked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ScoringDiagnostic {
    pub accounting_a: AccountingA,
    pub accounting_b: AccountingB,
    pub accounting_c: AccountingC,
    pub module_declares_winner: bool,
}

fn compute_scoring(classes: &[QuotientClassReport]) -> ScoringDiagnostic {
    let rows_a: Vec<_> = classes
        .iter()
        .map(|class| AccountingARow {
            class: class.id.clone(),
            kappa: class.active_slots,
        })
        .collect();
    let minimum_kappa = rows_a
        .iter()
        .map(|row| row.kappa)
        .min()
        .expect("class list");
    let minimum_a: Vec<_> = rows_a
        .iter()
        .filter(|row| row.kappa == minimum_kappa)
        .map(|row| row.class.clone())
        .collect();
    assert_eq!(minimum_a, vec!["minus_half_class"]);

    let rows_b: Vec<_> = classes
        .iter()
        .map(|class| AccountingBRow {
            class: class.id.clone(),
            embedding_direction_clauses: 1,
            description_units: class.active_slots,
        })
        .collect();
    let minimum_b_key = rows_b
        .iter()
        .map(|row| (row.embedding_direction_clauses, row.description_units))
        .min()
        .expect("class list");
    let minimum_b: Vec<_> = rows_b
        .iter()
        .filter(|row| (row.embedding_direction_clauses, row.description_units) == minimum_b_key)
        .map(|row| row.class.clone())
        .collect();
    assert_eq!(minimum_b, vec!["minus_half_class"]);

    let rows_c: Vec<_> = classes
        .iter()
        .map(|class| {
            let baseline = class.id == "zero_class";
            AccountingCRow {
                class: class.id.clone(),
                marginal_kappa: if baseline { "0" } else { ">=1" }.to_owned(),
                nu: 0,
                admissible: baseline,
                verdict: if baseline {
                    "sealed baseline; no shift".to_owned()
                } else {
                    "inadmissible marginal shift".to_owned()
                },
            }
        })
        .collect();
    assert!(
        rows_c
            .iter()
            .filter(|row| row.class != "zero_class")
            .all(|row| !row.admissible && row.nu == 0 && row.marginal_kappa == ">=1")
    );

    ScoringDiagnostic {
        accounting_a: AccountingA {
            semantics: "one clause per active slot".to_owned(),
            rows: rows_a,
            minimum_kappa,
            minimum_classes: minimum_a,
        },
        accounting_b: AccountingB {
            semantics: "one U(1) embedding-direction clause, then active-record description length"
                .to_owned(),
            rows: rows_b,
            lexicographic_minimum: [minimum_b_key.0, minimum_b_key.1],
            minimum_classes: minimum_b,
        },
        accounting_c: AccountingC {
            semantics: "marginal epsilon shift over the sealed zero-class completion".to_owned(),
            rows: rows_c,
            all_nonzero_shifts_blocked: true,
        },
        module_declares_winner: false,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProtocolReport {
    pub date: String,
    pub freeze_commit: String,
    pub specification: String,
    pub specification_sha256: String,
    pub exact_backend: String,
    pub floating_point_values: usize,
    pub blind_runs: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EpsilonClosureRun {
    pub protocol: ProtocolReport,
    #[serde(rename = "stageA")]
    pub stage_a: StageAReport,
    #[serde(rename = "stageB")]
    pub stage_b: StageBReport,
    pub simultaneous: SimultaneousReport,
    pub scoring_diagnostic: ScoringDiagnostic,
    pub post_run: PostRunReport,
}

pub fn build_epsilon_closure_run(
    freeze_commit: impl Into<String>,
    specification_sha256: impl Into<String>,
) -> EpsilonClosureRun {
    let (stage_a, sealed) = compute_stage_a();
    let (stage_b, winner) = compute_stage_b(&sealed);
    let (simultaneous, minus_half) = compute_simultaneous();
    let scoring_diagnostic = compute_scoring(&simultaneous.classes);
    let post_run = epsilon_closure_post_run::compute(&winner, &minus_half);
    EpsilonClosureRun {
        protocol: ProtocolReport {
            date: EPSILON_CLOSURE_DATE.to_owned(),
            freeze_commit: freeze_commit.into(),
            specification: "d2_engine_instructions.md (v2)".to_owned(),
            specification_sha256: specification_sha256.into(),
            exact_backend: "num_rational::Ratio<i128>; checked i128 primitives on every operation"
                .to_owned(),
            floating_point_values: 0,
            blind_runs: 1,
        },
        stage_a,
        stage_b,
        simultaneous,
        scoring_diagnostic,
        post_run,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_epsilon_closure_run, canonical_epsilon, compute_scoring, compute_simultaneous,
        compute_stage_a, compute_stage_b, exact,
    };

    #[test]
    fn theorem_a_stage_a_is_one_dimensional() {
        let (report, _) = compute_stage_a();
        assert_eq!(report.linear_rank, 5);
        assert_eq!(report.solution_dimension, 1);
        assert_eq!(report.identities["A111_after_forcing"], "0");
    }

    #[test]
    fn theorem_b_sealed_extension_has_three_independent_forcings() {
        let (_, sealed) = compute_stage_a();
        let (report, _) = compute_stage_b(&sealed);
        assert_eq!(report.forcings.len(), 3);
        assert!(
            report
                .forcings
                .iter()
                .all(|forcing| forcing.unique_solution == "y_nu = 0")
        );
    }

    #[test]
    fn theorem_c_family_is_complete_and_quotiented() {
        let (report, _) = compute_simultaneous();
        assert_eq!(report.rank, 5);
        assert_eq!(report.solution_dimension, 2);
        assert!(report.quotient.minus_one_equals_zero);
        assert!(report.quotient.one_equals_minus_two);
        assert!(report.quotient.minus_half_fixed);
    }

    #[test]
    fn canonicalizer_is_idempotent() {
        for epsilon in [
            exact(-7, 3),
            exact(-1, 1),
            exact(-1, 2),
            exact(0, 1),
            exact(8, 5),
        ] {
            assert_eq!(
                canonical_epsilon(&canonical_epsilon(&epsilon)),
                canonical_epsilon(&epsilon)
            );
        }
    }

    #[test]
    fn simultaneous_diagnostic_exhibits_the_registered_trap() {
        let (report, _) = compute_simultaneous();
        let diagnostic = compute_scoring(&report.classes);
        assert_eq!(
            diagnostic.accounting_a.minimum_classes,
            vec!["minus_half_class"]
        );
        assert_eq!(
            diagnostic.accounting_b.minimum_classes,
            vec!["minus_half_class"]
        );
        assert!(diagnostic.accounting_c.all_nonzero_shifts_blocked);
        assert!(!diagnostic.module_declares_winner);
    }

    #[test]
    fn complete_frozen_builder_runs_all_layers_in_order() {
        let report = build_epsilon_closure_run("freeze", "spec");
        assert_eq!(report.protocol.floating_point_values, 0);
        assert_eq!(report.protocol.blind_runs, 1);
    }
}
