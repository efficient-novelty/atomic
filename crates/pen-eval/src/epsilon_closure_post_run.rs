//! Post-verdict characterization for the epsilon-closure project.
//!
//! This module is deliberately separate from the theorem and scoring layer.

use crate::epsilon_closure::{
    Assignment, Exact, Slot, exact, exact_add, exact_div, exact_mul, exact_neg, exact_string,
};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CheckQuantity {
    pub name: String,
    pub formula: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TraceReport {
    #[serde(rename = "Tr_T3_squared")]
    pub tr_t3_squared: String,
    #[serde(rename = "Tr_Y_squared")]
    pub tr_y_squared: String,
    #[serde(rename = "Tr_Q_squared")]
    pub tr_q_squared: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GroupElement {
    pub a_mod_3: i128,
    pub b_mod_2: i128,
    pub theta_mod_1: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IntegralityValue {
    pub slot: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StabilizerReport {
    pub lattice_normalizer: String,
    pub integer_weights: Vec<IntegralityValue>,
    pub order: usize,
    pub cyclic: bool,
    pub generator: GroupElement,
    pub elements: Vec<GroupElement>,
    pub integrality_formula: String,
    pub integrality_witness: Vec<IntegralityValue>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostRunReport {
    pub q_table: Vec<CheckQuantity>,
    pub traces: TraceReport,
    pub normalization: String,
    pub stabilizer_winner: StabilizerReport,
    pub stabilizer_minus_half: StabilizerReport,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Element {
    a: i128,
    b: i128,
    theta: Exact,
}

fn gcd(mut left: i128, mut right: i128) -> i128 {
    left = left.checked_abs().expect("i128 overflow in lattice gcd");
    right = right.checked_abs().expect("i128 overflow in lattice gcd");
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn lcm(left: i128, right: i128) -> i128 {
    if left == 0 || right == 0 {
        return 0;
    }
    left.checked_div(gcd(left, right))
        .and_then(|value| value.checked_mul(right))
        .and_then(i128::checked_abs)
        .expect("i128 overflow in lattice lcm")
}

fn mod_one(value: &Exact) -> Exact {
    let denominator = *value.denom();
    let mut numerator = *value.numer() % denominator;
    if numerator < 0 {
        numerator = numerator
            .checked_add(denominator)
            .expect("i128 overflow reducing modulo one");
    }
    exact(numerator, denominator)
}

fn lattice_normalizer(assignment: &Assignment) -> Exact {
    let denominator_lcm = Slot::ALL.iter().fold(1_i128, |current, slot| {
        lcm(current, *assignment.value(*slot).denom())
    });
    let integer_gcd = Slot::ALL.iter().fold(0_i128, |current, slot| {
        let scaled = assignment
            .value(*slot)
            .numer()
            .checked_mul(
                denominator_lcm
                    .checked_div(*assignment.value(*slot).denom())
                    .expect("denominator must divide lcm"),
            )
            .expect("i128 overflow scaling lattice value");
        gcd(current, scaled)
    });
    assert_ne!(integer_gcd, 0);
    exact(integer_gcd, denominator_lcm)
}

fn tags(slot: Slot) -> (i128, i128) {
    match slot {
        Slot::Q => (1, 1),
        Slot::Uc | Slot::Dc => (-1, 0),
        Slot::L | Slot::H => (0, 1),
        Slot::Ec | Slot::NuC => (0, 0),
    }
}

fn integer_weights(assignment: &Assignment, unit: &Exact) -> Vec<(Slot, i128)> {
    Slot::ALL
        .into_iter()
        .map(|slot| {
            let weight = exact_div(assignment.value(slot), unit);
            assert_eq!(*weight.denom(), 1);
            (slot, *weight.numer())
        })
        .collect()
}

fn condition_value(element: &Element, slot: Slot, weight: i128) -> Exact {
    let (t, d) = tags(slot);
    exact_add(
        &exact_add(
            &exact_mul(&exact(element.a, 1), &exact(t, 3)),
            &exact_mul(&exact(element.b, 1), &exact(d, 2)),
        ),
        &exact_mul(&element.theta, &exact(weight, 1)),
    )
}

fn enumerate_group(assignment: &Assignment, unit: &Exact) -> Vec<Element> {
    let weights = integer_weights(assignment, unit);
    let weight_lcm = weights
        .iter()
        .map(|(_, weight)| weight.abs())
        .filter(|weight| *weight != 0)
        .fold(1_i128, lcm);
    let theta_denominator = 6_i128
        .checked_mul(weight_lcm)
        .expect("i128 overflow in theta denominator");
    let mut elements = BTreeSet::new();
    for a in 0..3_i128 {
        for b in 0..2_i128 {
            for numerator in 0..theta_denominator {
                let element = Element {
                    a,
                    b,
                    theta: exact(numerator, theta_denominator),
                };
                if weights
                    .iter()
                    .all(|(slot, weight)| condition_value(&element, *slot, *weight).denom() == &1)
                {
                    elements.insert(element);
                }
            }
        }
    }
    elements.into_iter().collect()
}

fn add_elements(left: &Element, right: &Element) -> Element {
    Element {
        a: left
            .a
            .checked_add(right.a)
            .expect("i128 overflow in group addition")
            % 3,
        b: left
            .b
            .checked_add(right.b)
            .expect("i128 overflow in group addition")
            % 2,
        theta: mod_one(&exact_add(&left.theta, &right.theta)),
    }
}

fn element_order(element: &Element, group_bound: usize) -> usize {
    let identity = Element {
        a: 0,
        b: 0,
        theta: exact(0, 1),
    };
    let mut current = identity.clone();
    for order in 1..=group_bound {
        current = add_elements(&current, element);
        if current == identity {
            return order;
        }
    }
    panic!("element order exceeded finite group bound")
}

fn public_element(element: &Element) -> GroupElement {
    GroupElement {
        a_mod_3: element.a,
        b_mod_2: element.b,
        theta_mod_1: exact_string(&element.theta),
    }
}

fn stabilizer(assignment: &Assignment) -> StabilizerReport {
    let unit = lattice_normalizer(assignment);
    let weights = integer_weights(assignment, &unit);
    let elements = enumerate_group(assignment, &unit);
    let generator = elements
        .iter()
        .find(|element| element_order(element, elements.len()) == elements.len())
        .cloned();
    let cyclic = generator.is_some();
    let generator = generator.unwrap_or_else(|| Element {
        a: 0,
        b: 0,
        theta: exact(0, 1),
    });
    let witness = weights
        .iter()
        .map(|(slot, weight)| IntegralityValue {
            slot: slot.name().to_owned(),
            value: exact_string(&condition_value(&generator, *slot, *weight)),
        })
        .collect();
    StabilizerReport {
        lattice_normalizer: exact_string(&unit),
        integer_weights: weights
            .iter()
            .map(|(slot, weight)| IntegralityValue {
                slot: slot.name().to_owned(),
                value: weight.to_string(),
            })
            .collect(),
        order: elements.len(),
        cyclic,
        generator: public_element(&generator),
        elements: elements.iter().map(public_element).collect(),
        integrality_formula: "a*t/3 + b*d/2 + theta*(y/y_unit)".to_owned(),
        integrality_witness: witness,
    }
}

fn t3_trace() -> Exact {
    let q_components = exact_mul(&exact(3, 1), &exact_add(&exact(1, 4), &exact(1, 4)));
    let l_components = exact_add(&exact(1, 4), &exact(1, 4));
    exact_add(&q_components, &l_components)
}

fn y_trace(assignment: &Assignment) -> Exact {
    [
        (6, Slot::Q),
        (3, Slot::Uc),
        (3, Slot::Dc),
        (2, Slot::L),
        (1, Slot::Ec),
        (1, Slot::NuC),
    ]
    .into_iter()
    .fold(exact(0, 1), |sum, (multiplicity, slot)| {
        exact_add(
            &sum,
            &exact_mul(
                &exact(multiplicity, 1),
                &exact_mul(assignment.value(slot), assignment.value(slot)),
            ),
        )
    })
}

fn q_trace(assignment: &Assignment) -> Exact {
    let q_upper = exact_add(&exact(1, 2), assignment.value(Slot::Q));
    let q_lower = exact_add(&exact(-1, 2), assignment.value(Slot::Q));
    let l_upper = exact_add(&exact(1, 2), assignment.value(Slot::L));
    let l_lower = exact_add(&exact(-1, 2), assignment.value(Slot::L));
    let mut sum = exact_mul(
        &exact(3, 1),
        &exact_add(
            &exact_mul(&q_upper, &q_upper),
            &exact_mul(&q_lower, &q_lower),
        ),
    );
    sum = exact_add(
        &sum,
        &exact_mul(
            &exact(3, 1),
            &exact_mul(assignment.value(Slot::Uc), assignment.value(Slot::Uc)),
        ),
    );
    sum = exact_add(
        &sum,
        &exact_mul(
            &exact(3, 1),
            &exact_mul(assignment.value(Slot::Dc), assignment.value(Slot::Dc)),
        ),
    );
    sum = exact_add(&sum, &exact_mul(&l_upper, &l_upper));
    sum = exact_add(&sum, &exact_mul(&l_lower, &l_lower));
    sum = exact_add(
        &sum,
        &exact_mul(assignment.value(Slot::Ec), assignment.value(Slot::Ec)),
    );
    exact_add(
        &sum,
        &exact_mul(assignment.value(Slot::NuC), assignment.value(Slot::NuC)),
    )
}

pub(crate) fn compute(winner: &Assignment, minus_half: &Assignment) -> PostRunReport {
    let q_nu = exact_neg(winner.value(Slot::NuC));
    let q_e = exact_neg(winner.value(Slot::Ec));
    let q_u = exact_neg(winner.value(Slot::Uc));
    let q_d = exact_neg(winner.value(Slot::Dc));
    let tr_t3 = t3_trace();
    let tr_y = y_trace(winner);
    let tr_q = q_trace(winner);
    assert_eq!(tr_t3, exact(2, 1));
    assert_eq!(tr_y, exact(10, 3));
    assert_eq!(tr_q, exact(16, 3));
    let normalization = exact_div(&tr_t3, &tr_q);
    assert_eq!(normalization, exact(3, 8));

    let stabilizer_winner = stabilizer(winner);
    assert_eq!(stabilizer_winner.order, 6);
    assert!(stabilizer_winner.cyclic);
    let expected_witness = ["1", "-1", "0", "0", "1", "0", "1"];
    assert_eq!(
        stabilizer_winner
            .integrality_witness
            .iter()
            .map(|entry| entry.value.as_str())
            .collect::<Vec<_>>(),
        expected_witness
    );
    let stabilizer_minus_half = stabilizer(minus_half);

    PostRunReport {
        q_table: vec![
            CheckQuantity {
                name: "q_nu".to_owned(),
                formula: "-epsilon".to_owned(),
                value: exact_string(&q_nu),
            },
            CheckQuantity {
                name: "q_e".to_owned(),
                formula: "-1-epsilon".to_owned(),
                value: exact_string(&q_e),
            },
            CheckQuantity {
                name: "q_u".to_owned(),
                formula: "2/3+epsilon/3".to_owned(),
                value: exact_string(&q_u),
            },
            CheckQuantity {
                name: "q_d".to_owned(),
                formula: "-1/3+epsilon/3".to_owned(),
                value: exact_string(&q_d),
            },
        ],
        traces: TraceReport {
            tr_t3_squared: exact_string(&tr_t3),
            tr_y_squared: exact_string(&tr_y),
            tr_q_squared: exact_string(&tr_q),
        },
        normalization: exact_string(&normalization),
        stabilizer_winner,
        stabilizer_minus_half,
    }
}

#[cfg(test)]
mod tests {
    use super::compute;
    use crate::epsilon_closure::{exact, finite_assignment};

    #[test]
    fn post_run_values_are_computed_only_after_the_zero_class_verdict() {
        let winner = finite_assignment(&exact(0, 1));
        let minus_half = finite_assignment(&exact(-1, 2));
        let report = compute(&winner, &minus_half);
        assert_eq!(report.normalization, "3/8");
        assert_eq!(report.stabilizer_winner.order, 6);
        assert!(report.stabilizer_winner.cyclic);
    }
}
