//! XF-0b2 Phase R-0 probe: mechanical re-derivation of the stage-14 family
//! `df4feb52882e` from the frozen kernel, never from the concordance gloss
//! (falsifier F-R1).
//!
//! Usage: cargo run -p pen-search --example xf0b2_r0_probe

use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_eval::typed_families::{
    CandidateExtractionOutcome, clause_presentation, extract_candidate_families,
    predecessor_closure,
};
use pen_search::enumerate::{EnumerationContext, LateFamilySurface, enumerate_exprs};
use pen_type::elaborate::{
    SealedSignature, elaborate_single_clause, elaborate_telescope, minimal_ambient_parameters,
};
use pen_type::equality::univalent_equality;
use pen_type::normalize::normalize;

const TARGET: &str = "blake3:df4feb52882ec5b6a56472960c340c6a16be3b8c48dd25d389f9b5dca8e9e62b";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signature = SealedSignature::genesis_del_h15();
    let t14 = Telescope::reference(14);

    println!("== R-0.1 elaboration of the sealed stage-14 telescope ==");
    println!("signature digest      : {}", signature.digest());
    println!(
        "stage-14 candidate    : {}",
        signature.entry(14).unwrap().candidate_hash
    );
    let ambient = minimal_ambient_parameters(&t14);
    println!("minimal ambient arity : {ambient}");

    let elaboration = elaborate_telescope(&signature, &t14, 13)
        .map_err(|failure| format!("stage-14 elaboration failed: {failure}"))?;
    println!("elaborated ambient    : {}", elaboration.ambient_parameters);
    println!("derivation hash       : {}", elaboration.derivation_hash);
    println!();
    for clause in &elaboration.clauses {
        println!(
            "clause {:>2} | role {:?} -> {:?} | ty {:?} | coarse {} | beta {} | stuck {} | expr {:?}",
            clause.clause_index,
            clause.declared_role,
            clause.kernel_role,
            clause.kernel_ty,
            clause.coarse_assumptions,
            clause.beta_steps,
            clause.stuck_applications.len(),
            t14.clauses[clause.clause_index as usize].expr,
        );
        println!("           NF {:?}", clause.normal_form);
    }

    println!();
    println!("== clause 3 derivation tree (verbatim) ==");
    let c3 = &elaboration.clauses[3];
    print_derivation(&c3.derivation, 0);

    println!();
    println!("== R-0.1 scope resolution of clause 3 ==");
    // Scope at clause 3 = [ambient params (A)] ++ [fields of clauses 0,1,2].
    let base = elaboration.ambient_parameters + 3;
    println!(
        "base scope at clause 3            : {base} (= ambient {} + 3 prior fields)",
        elaboration.ambient_parameters
    );
    println!("Pi binder level                   : {}", base + 1);
    println!("Lam (domain abstraction) binder   : {}", base + 1);
    println!("Sigma binder level (under Pi)     : {}", base + 2);
    println!(
        "occurrence Var 1 in Lam body      : {}",
        describe(1, elaboration.ambient_parameters, 3)
    );
    println!(
        "occurrence Var 1 in Sigma domain  : {}",
        describe(1, elaboration.ambient_parameters, 3)
    );
    println!(
        "occurrence Var 2 in Sigma codomain: {}",
        describe(2, elaboration.ambient_parameters, 3)
    );
    println!(
        "prior kernel roles                : {:?}",
        elaboration.clauses[..3]
            .iter()
            .map(|c| c.kernel_role)
            .collect::<Vec<_>>()
    );
    // Independence of the ambient reading: check every admissible ambient.
    println!("all admissible ambient readings (A = 0,1,2):");
    for a in 0u32..=2 {
        let b = a + 3;
        println!(
            "  A={a}: binders at levels {} and {}; Var 1 -> {}; Var 2 -> {}; both free? {}",
            b + 1,
            b + 2,
            describe(1, a, 3),
            describe(2, a, 3),
            1 <= b && 2 <= b
        );
    }

    println!();
    println!("== R-0.2 irreducibility ==");
    let c3_expr = &t14.clauses[3].expr;
    let nf = normalize(c3_expr, base, 256)?;
    println!("clause 3 beta steps to NF : {}", nf.steps);
    println!("NF equals the raw clause  : {}", &nf.expr == c3_expr);
    // The domain in isolation.
    let domain = Expr::Lam(Box::new(Expr::Var(1)));
    let dnf = normalize(&domain, base, 256)?;
    println!(
        "domain Lam(Var 1) steps   : {} (NF identical: {})",
        dnf.steps,
        dnf.expr == domain
    );
    // Is there any beta redex anywhere in the clause? (App node count)
    println!("App nodes in clause 3     : {}", count_apps(c3_expr));
    // eta: kernel equality is beta-NF equality; test the eta-expansion/contraction pair.
    // Lam(Var 1) at scope `base` binds level base+1; eta-contraction would need the
    // body to be App(f, Var(base+1)). It is Var 1, so no eta redex exists either.
    println!("eta-redex present         : {}", eta_redex(&domain, base));
    // A judgmental-equality probe: nothing whose head is not Lam can be equal to it.
    for probe in [
        Expr::Var(1),
        Expr::Univ,
        Expr::Lib(13),
        Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        Expr::App(
            Box::new(Expr::Lam(Box::new(Expr::Var(base + 1)))),
            Box::new(Expr::Var(1)),
        ),
    ] {
        let witness = univalent_equality(&domain, &probe, base, 256)?;
        println!("  Lam(Var 1) == {:?} ? {}", probe, witness.equal);
    }

    // Every sealed entry exports zero computation-role clauses: beta is the
    // only oriented computation clause in the kernel, so there is no eta and
    // no library-supplied conversion that could eliminate the abstraction.
    let mut computation_clauses = 0u32;
    let mut beta_clause_steps = Vec::new();
    for entry in signature.entries() {
        computation_clauses += u32::from(entry.exported_computation_clauses);
        if !entry.beta_clauses.is_empty() {
            beta_clause_steps.push((entry.step, entry.beta_clauses.clone()));
        }
    }
    println!("sealed computation-role clauses (all 15 entries): {computation_clauses}");
    println!("sealed beta-redex clauses: {beta_clause_steps:?}");

    // Bounded catalog certificate: over the generator catalog at this scope,
    // which expressions are judgmentally equal to the domain abstraction?
    let catalog = enumerate_exprs(EnumerationContext {
        library_size: 13,
        scope_size: base,
        max_path_dimension: 0,
        include_trunc: true,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes: 5,
        require_former_eliminator_clauses: false,
        require_initial_hit_clauses: false,
        require_truncation_hit_clauses: false,
        require_higher_hit_clauses: false,
        require_sphere_lift_clauses: false,
        require_axiomatic_bundle_clauses: false,
        require_modal_shell_clauses: false,
        require_connection_shell_clauses: false,
        require_curvature_shell_clauses: false,
        require_operator_bundle_clauses: false,
        require_hilbert_functional_clauses: false,
        require_temporal_shell_clauses: false,
        historical_anchor_ref: None,
        late_family_surface: LateFamilySurface::None,
    });
    let mut equal_to_domain = Vec::new();
    for candidate in &catalog {
        if let Ok(witness) = univalent_equality(&domain, candidate, base, 256) {
            if witness.equal {
                equal_to_domain.push(candidate.clone());
            }
        }
    }
    println!(
        "catalog at scope {base}, <=5 nodes: {} expressions; {} judgmentally equal to Lam(Var 1)",
        catalog.len(),
        equal_to_domain.len()
    );
    let non_lam: Vec<&Expr> = equal_to_domain
        .iter()
        .filter(|e| !matches!(e, Expr::Lam(_)))
        .collect();
    println!(
        "  of those, presentations that are NOT syntactically Lam-headed: {}",
        non_lam.len()
    );
    for e in &non_lam {
        println!("    {e:?}");
    }

    println!();
    println!("== R-0.3 family identity and marginality ==");
    // The certified family ids are minted against the PREDECESSOR signature
    // (steps 1..13), as recorded in the semantic provenance package.
    let prefix: Vec<(u32, Telescope)> = (1..=13).map(|s| (s, Telescope::reference(s))).collect();
    let prefix_signature = SealedSignature::from_telescopes(prefix);
    println!(
        "predecessor signature digest (steps 1..13): {}",
        prefix_signature.digest()
    );
    let closure = predecessor_closure(&prefix_signature)?;
    println!(
        "predecessor closure digest: {} ({} families)",
        closure.digest,
        closure.families.len()
    );

    let prior_roles: Vec<ClauseRole> = elaboration.clauses.iter().map(|c| c.kernel_role).collect();
    let presentation = clause_presentation(
        &c3.normal_form,
        elaboration.ambient_parameters + 3,
        &prior_roles[..3],
        elaboration.ambient_parameters,
    );
    println!("canonical NF   : {:?}", presentation.canonical_normal_form);
    println!("parameters     : {:?}", presentation.parameters);
    println!("renaming       : {:?}", presentation.renaming);

    match extract_candidate_families(&prefix_signature, &closure, &t14, 13) {
        CandidateExtractionOutcome::Extracted(extraction) => {
            println!("stage-14 families extracted: {}", extraction.families.len());
            println!(
                "marginal family count      : {}",
                extraction.marginal_family_count
            );
            for family in &extraction.families {
                let hit = family.id.as_str() == TARGET;
                println!(
                    "  {} {} | role {:?} | instances {} | marginal {}{}",
                    if hit { ">>" } else { "  " },
                    &family.id.as_str()[..20],
                    family.generator_role,
                    family.instances.len(),
                    family.marginality.is_marginal(),
                    if hit {
                        "   <== TARGET df4feb52882e"
                    } else {
                        ""
                    }
                );
                if hit {
                    println!(
                        "     canonical NF : {:?}",
                        family.presentation.canonical_normal_form
                    );
                    println!("     parameters   : {:?}", family.presentation.parameters);
                    println!("     kernel ty    : {:?}", family.generator_kernel_ty);
                    println!("     instances    : {:?}", family.instances);
                    println!("     marginality  : {:?}", family.marginality);
                    println!("     naturality   : {:?}", family.naturality);
                }
            }
        }
        CandidateExtractionOutcome::KernelInvalid { failure } => {
            println!("stage-14 is kernel-invalid: {failure}");
        }
    }

    println!();
    println!("== R-0.3 occurrences across the whole sealed signature ==");
    for step in 1..=15u32 {
        let telescope = Telescope::reference(step);
        for (index, clause) in telescope.clauses.iter().enumerate() {
            if contains_lam_in_pi_domain(&clause.expr) {
                println!(
                    "  step {step:>2} clause {index}: {:?}   <-- Lam in a Pi/Sigma domain",
                    clause.expr
                );
            }
        }
    }

    // ---------------------------------------------------------------------
    // Addendum, closing the two gaps the L1 analyst asked to have settled.
    // ---------------------------------------------------------------------
    println!();
    println!("== ADDENDUM A: is a binder-dependent codomain writable at this exact position? ==");
    let prior = [
        ClauseRole::Formation,
        ClauseRole::Formation,
        ClauseRole::Formation,
    ];
    let variants: [(&str, Expr); 4] = [
        (
            "the sealed clause                       ",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ),
        ),
        (
            "Pi-binder-dependent codomain (Var 5)    ",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(5)))),
            ),
        ),
        (
            "Sigma-binder-dependent codomain (Var 6) ",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(6)))),
            ),
        ),
        (
            "type domain + dependent codomain        ",
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(5)))),
            ),
        ),
    ];
    for (label, expr) in &variants {
        match elaborate_single_clause(expr, elaboration.ambient_parameters, &prior, 13) {
            Ok(single) => println!(
                "  {label} ELABORATES | ty {:?} | coarse {} | NF {:?}",
                single.kernel_ty, single.coarse_assumptions, single.normal_form
            ),
            Err(error) => println!("  {label} REJECTED   | {error}"),
        }
    }

    println!();
    println!("== ADDENDUM B: does any sealed clause bind a Var to a local binder? ==");
    let mut binder_uses = 0usize;
    for step in 1..=15u32 {
        let telescope = Telescope::reference(step);
        let step_ambient = minimal_ambient_parameters(&telescope);
        for (index, clause) in telescope.clauses.iter().enumerate() {
            let base = step_ambient + index as u32;
            let mut hits = Vec::new();
            collect_binder_uses(&clause.expr, base, 0, &mut hits);
            if !hits.is_empty() {
                binder_uses += hits.len();
                println!(
                    "  step {step:>2} clause {index} (ambient {step_ambient}, base {base}): {:?} -> binder levels {hits:?}",
                    clause.expr
                );
            }
        }
    }
    println!("  total binder-bound Var occurrences in the sealed corpus: {binder_uses}");

    // ---------------------------------------------------------------------
    // ADDENDUM C: is the subject clause's exact shape hard-coded in the
    // search's own admissibility layer? (Raised by independent analyst B.)
    // ---------------------------------------------------------------------
    println!();
    println!("== ADDENDUM C: package recognition under perturbation of clause 3 ==");
    let mut library: Library = Vec::new();
    for step in 1..=13u32 {
        let entry = LibraryEntry::from_telescope(&Telescope::reference(step), &library);
        library.push(entry);
    }
    println!("built library of {} entries (steps 1..13)", library.len());

    let perturbations: [(&str, Expr); 6] = [
        (
            "the sealed clause  Pi(Lam(Var 1), Sigma(Var 1, Var 2))",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ),
        ),
        (
            "eta-collapsed      Pi(Var 1, Sigma(Var 1, Var 2))",
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ),
        ),
        (
            "second factor X    Pi(Lam(Var 1), Sigma(Var 1, Var 1))",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ),
        (
            "abstraction at Y   Pi(Lam(Var 2), Sigma(Var 1, Var 2))",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(2)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ),
        ),
        (
            "factors swapped    Pi(Lam(Var 1), Sigma(Var 2, Var 1))",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ),
        ),
        (
            "binder-dependent   Pi(Lam(Var 1), Sigma(Var 1, Var 5))",
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(5)))),
            ),
        ),
    ];
    for (label, replacement) in &perturbations {
        let mut variant = Telescope::reference(14);
        variant.clauses[3] = ClauseRec::new(variant.clauses[3].role, replacement.clone());
        let entry = LibraryEntry::from_telescope(&variant, &library);
        println!(
            "  has_hilbert = {:<5} | {label}",
            entry.capabilities.has_hilbert
        );
    }
    println!();
    println!("  (has_hilbert is computed by matches_hilbert_functional_shell,");
    println!("   crates/pen-core/src/library.rs; the stage-14 admissibility gate");
    println!(
        "   supports_hilbert_functional_clause_at_position, crates/pen-search/src/enumerate.rs,"
    );
    println!("   carries the same per-position literals into the enumerator.)");

    Ok(())
}

/// Collect every `Var` occurrence that resolves to a LOCAL BINDER, i.e. a
/// level strictly above `base` where `base` is the clause's free scope.
/// `depth` is the number of enclosing binders at this position; binders are
/// `Lam` bodies and `Pi`/`Sigma` codomains only (frozen kernel convention).
fn collect_binder_uses(expr: &Expr, base: u32, depth: u32, hits: &mut Vec<u32>) {
    match expr {
        Expr::Var(level) => {
            if *level > base && *level <= base + depth {
                hits.push(*level);
            }
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            collect_binder_uses(domain, base, depth, hits);
            collect_binder_uses(codomain, base, depth + 1, hits);
        }
        Expr::Lam(body) => collect_binder_uses(body, base, depth + 1, hits),
        Expr::App(a, b) => {
            collect_binder_uses(a, base, depth, hits);
            collect_binder_uses(b, base, depth, hits);
        }
        Expr::Id(a, b, c) => {
            collect_binder_uses(a, base, depth, hits);
            collect_binder_uses(b, base, depth, hits);
            collect_binder_uses(c, base, depth, hits);
        }
        Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => collect_binder_uses(inner, base, depth, hits),
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => {}
    }
}

fn describe(level: u32, ambient: u32, priors: u32) -> String {
    if level == 0 {
        "invalid".to_string()
    } else if level <= ambient {
        format!("Ambient parameter #{level}")
    } else if level <= ambient + priors {
        format!("Field of clause {}", level - ambient - 1)
    } else {
        format!("Local binder #{}", level - ambient - priors)
    }
}

fn count_apps(expr: &Expr) -> usize {
    match expr {
        Expr::App(a, b) => 1 + count_apps(a) + count_apps(b),
        Expr::Pi(a, b) | Expr::Sigma(a, b) => count_apps(a) + count_apps(b),
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => count_apps(inner),
        Expr::Id(a, b, c) => count_apps(a) + count_apps(b) + count_apps(c),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => 0,
    }
}

/// An eta redex at scope `scope`: `Lam(App(f, Var(scope + 1)))` where the
/// binder does not occur free in `f`.
fn eta_redex(expr: &Expr, scope: u32) -> bool {
    match expr {
        Expr::Lam(body) => match body.as_ref() {
            Expr::App(function, argument) => {
                **argument == Expr::Var(scope + 1) && !function.var_refs().contains(&(scope + 1))
            }
            _ => false,
        },
        _ => false,
    }
}

fn contains_lam_in_pi_domain(expr: &Expr) -> bool {
    match expr {
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            matches!(domain.as_ref(), Expr::Lam(_))
                || contains_lam_in_pi_domain(domain)
                || contains_lam_in_pi_domain(codomain)
        }
        Expr::App(a, b) => contains_lam_in_pi_domain(a) || contains_lam_in_pi_domain(b),
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => contains_lam_in_pi_domain(inner),
        Expr::Id(a, b, c) => {
            contains_lam_in_pi_domain(a)
                || contains_lam_in_pi_domain(b)
                || contains_lam_in_pi_domain(c)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn print_derivation(node: &pen_type::elaborate::DerivationNode, depth: usize) {
    println!(
        "{:indent$}{} :: {:?}{}",
        "",
        node.rule,
        node.kernel_ty,
        if node.coarse { "   [COARSE]" } else { "" },
        indent = depth * 2
    );
    for child in &node.children {
        print_derivation(child, depth + 1);
    }
}
