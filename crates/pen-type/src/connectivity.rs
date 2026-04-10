use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::{Telescope, TelescopeClass};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::OnceLock;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStepFifteenClaimSafeClauseOneLabel {
    ClaimNextCodomain,
    ClaimSharpCodomain,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStepFifteenClaimSafeClauseTwoLabel {
    ClaimFlatDomain,
    ClaimSharpCodomain,
    Reference,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel {
    ClaimFlatCodomain,
    Reference,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel {
    ClaimFlatArgument,
    ClaimEventualArgument,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel {
    ClaimNextBridge,
    Reference,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel {
    ClaimNextCodomain,
    ClaimSharpCodomain,
    Reference,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClaimStepFifteenClaimSafePairClauseTwoSelector {
    pub clause_one: ClaimStepFifteenClaimSafeClauseOneLabel,
    pub clause_two: ClaimStepFifteenClaimSafeClauseTwoLabel,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteSelector {
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_four:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel>,
    clause_three:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel>,
    clause_six:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector {
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_four:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel>,
    clause_three:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel>,
    clause_six:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector {
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_four:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel>,
    clause_three:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel>,
    clause_six:
        Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel>,
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_EVENTUALLY_CODOMAIN_SIDE_POCKET_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_FLAT_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_LIVE_CLAIM_BRIDGE_SURFACE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_SURFACE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SHEET_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_REFERENCE_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_FLAT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_NEXT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_OVERRIDE:
        std::cell::RefCell<Option<ClaimStepFifteenClaimSafeClauseOneLabel>> =
            const { std::cell::RefCell::new(None) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_CLAUSE_TWO_OVERRIDE:
        std::cell::RefCell<Option<ClaimStepFifteenClaimSafePairClauseTwoSelector>> =
            const { std::cell::RefCell::new(None) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_BRIDGE_ON_CLAIM_SAFE_PAIR_OVERRIDE:
        std::cell::RefCell<Option<ClaimStepFifteenClaimSafeClauseOneLabel>> =
            const { std::cell::RefCell::new(None) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ZERO_BRIDGE_SLICE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ONE_BRIDGE_SLICE_OVERRIDE:
        std::cell::RefCell<usize> = const { std::cell::RefCell::new(0) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_PARENT_ROUTE_OVERRIDE:
        std::cell::RefCell<Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteSelector>> =
            const { std::cell::RefCell::new(None) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_ACTIVE_WINDOW_OVERRIDE:
        std::cell::RefCell<Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector>> =
            const { std::cell::RefCell::new(None) };
}

thread_local! {
    static CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_SELF_CONTAINED_OVERRIDE:
        std::cell::RefCell<Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector>> =
            const { std::cell::RefCell::new(None) };
}

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneEventuallyCodomainSidePocketOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimFlatOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroLiveClaimBridgeSurfaceOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroSurfaceOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourClaimNextBridgeSideOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourReferenceSideOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSheetOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimFlatSheetOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimSharpSheetOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimVariantPairOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimVariantPairOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimFlatSheetOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimSharpSheetOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSideOnClauseTwoClaimVariantPairOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveReferenceOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimFlatCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimNextCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseFiveSidePocketOnClaimSafeClauseZeroOneOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseFourSidePocketOnClaimSafeClauseZeroOneOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairClauseTwoOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseFourSharpBridgeOnClaimSafePairOverrideGuard;

pub struct ClaimStepFifteenClauseFiveRemainingTwoMismatchZeroBridgeSliceOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenClauseFiveRemainingTwoMismatchOneBridgeSliceOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard;

#[doc(hidden)]
pub struct ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard;

impl Drop for ClaimStepFifteenClauseOneEventuallyCodomainSidePocketOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_EVENTUALLY_CODOMAIN_SIDE_POCKET_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_eventually_codomain_side_pocket()
-> ClaimStepFifteenClauseOneEventuallyCodomainSidePocketOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_EVENTUALLY_CODOMAIN_SIDE_POCKET_OVERRIDE.with(|override_depth| {
        *override_depth.borrow_mut() += 1;
    });
    ClaimStepFifteenClauseOneEventuallyCodomainSidePocketOverrideGuard
}

impl Drop for ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimFlatOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_FLAT_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_flat_side_pocket()
-> ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimFlatOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_FLAT_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimFlatOverrideGuard
}

impl Drop for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroLiveClaimBridgeSurfaceOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_LIVE_CLAIM_BRIDGE_SURFACE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroLiveClaimBridgeSurfaceOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_LIVE_CLAIM_BRIDGE_SURFACE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroLiveClaimBridgeSurfaceOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroSurfaceOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_SURFACE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_surface()
-> ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroSurfaceOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_SURFACE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroSurfaceOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourClaimNextBridgeSideOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side()
-> ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourClaimNextBridgeSideOverrideGuard{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourClaimNextBridgeSideOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourReferenceSideOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_reference_side()
-> ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourReferenceSideOverrideGuard{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnClauseZeroClaimDomainMismatchZeroClauseFourReferenceSideOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSheetOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SHEET_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSheetOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SHEET_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSheetOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimFlatSheetOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimFlatSheetOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimFlatSheetOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimSharpSheetOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimSharpSheetOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimSharpSheetOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimVariantPairOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimVariantPairOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseTwoClaimVariantPairOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimVariantPairOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimVariantPairOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimVariantPairOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimFlatSheetOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimFlatSheetOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimFlatSheetOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimSharpSheetOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimSharpSheetOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourClaimNextBridgeSideOnClauseTwoClaimSharpSheetOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSideOnClauseTwoClaimVariantPairOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSideOnClauseTwoClaimVariantPairOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFourReferenceSideOnClauseTwoClaimVariantPairOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveReferenceOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_REFERENCE_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveReferenceOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_REFERENCE_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveReferenceOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimFlatCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_FLAT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimFlatCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_FLAT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimFlatCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
}

impl Drop
    for ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimNextCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
{
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_NEXT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair()
-> ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimNextCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_NEXT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseOneFlatCodomainOnReferenceClauseZeroClauseFiveClaimNextCodomainOnClauseFourReferenceTailOnClauseTwoClaimVariantPairOverrideGuard
}

impl Drop for ClaimStepFifteenClauseFiveSidePocketOnClaimSafeClauseZeroOneOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one()
-> ClaimStepFifteenClauseFiveSidePocketOnClaimSafeClauseZeroOneOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseFiveSidePocketOnClaimSafeClauseZeroOneOverrideGuard
}

impl Drop for ClaimStepFifteenClauseFourSidePocketOnClaimSafeClauseZeroOneOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one()
-> ClaimStepFifteenClauseFourSidePocketOnClaimSafeClauseZeroOneOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseFourSidePocketOnClaimSafeClauseZeroOneOverrideGuard
}

impl Drop for ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_OVERRIDE.with(
            |override_selector| {
                *override_selector.borrow_mut() = None;
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair(
    clause_one: ClaimStepFifteenClaimSafeClauseOneLabel,
) -> ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(clause_one);
        },
    );
    ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairOverrideGuard
}

impl Drop for ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairClauseTwoOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_CLAUSE_TWO_OVERRIDE.with(
            |override_selector| {
                *override_selector.borrow_mut() = None;
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two(
    selector: ClaimStepFifteenClaimSafePairClauseTwoSelector,
) -> ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairClauseTwoOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_CLAUSE_TWO_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(selector);
        },
    );
    ClaimStepFifteenClauseFourSharpCodomainOnClaimSafePairClauseTwoOverrideGuard
}

impl Drop for ClaimStepFifteenClauseFourSharpBridgeOnClaimSafePairOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_BRIDGE_ON_CLAIM_SAFE_PAIR_OVERRIDE.with(
            |override_selector| {
                *override_selector.borrow_mut() = None;
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_pair(
    clause_one: ClaimStepFifteenClaimSafeClauseOneLabel,
) -> ClaimStepFifteenClauseFourSharpBridgeOnClaimSafePairOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_BRIDGE_ON_CLAIM_SAFE_PAIR_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(clause_one);
        },
    );
    ClaimStepFifteenClauseFourSharpBridgeOnClaimSafePairOverrideGuard
}

impl Drop for ClaimStepFifteenClauseFiveRemainingTwoMismatchZeroBridgeSliceOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ZERO_BRIDGE_SLICE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice()
-> ClaimStepFifteenClauseFiveRemainingTwoMismatchZeroBridgeSliceOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ZERO_BRIDGE_SLICE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseFiveRemainingTwoMismatchZeroBridgeSliceOverrideGuard
}

impl Drop for ClaimStepFifteenClauseFiveRemainingTwoMismatchOneBridgeSliceOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ONE_BRIDGE_SLICE_OVERRIDE.with(
            |override_depth| {
                let mut override_depth = override_depth.borrow_mut();
                *override_depth = override_depth.saturating_sub(1);
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice()
-> ClaimStepFifteenClauseFiveRemainingTwoMismatchOneBridgeSliceOverrideGuard {
    CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ONE_BRIDGE_SLICE_OVERRIDE.with(
        |override_depth| {
            *override_depth.borrow_mut() += 1;
        },
    );
    ClaimStepFifteenClauseFiveRemainingTwoMismatchOneBridgeSliceOverrideGuard
}

impl Drop for ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_PARENT_ROUTE_OVERRIDE.with(
            |override_selector| {
                *override_selector.borrow_mut() = None;
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_PARENT_ROUTE_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: None,
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard
}

impl Drop for ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_ACTIVE_WINDOW_OVERRIDE.with(
            |override_selector| {
                *override_selector.borrow_mut() = None;
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_ACTIVE_WINDOW_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: None,
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_clause_three(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_three: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_ACTIVE_WINDOW_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: Some(clause_three),
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_clause_six(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_six: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_ACTIVE_WINDOW_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: None,
                    clause_six: Some(clause_six),
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard
}

impl Drop for ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_SELF_CONTAINED_OVERRIDE.with(
            |override_selector| {
                *override_selector.borrow_mut() = None;
            },
        );
    }
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_SELF_CONTAINED_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: None,
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_clause_three(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_three: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_SELF_CONTAINED_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: Some(clause_three),
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_clause_six(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_six: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_SELF_CONTAINED_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: None,
                    clause_six: Some(clause_six),
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_three(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_three: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_PARENT_ROUTE_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: Some(clause_three),
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_six(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_six: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_PARENT_ROUTE_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteSelector {
                    clause_five,
                    clause_four: None,
                    clause_three: None,
                    clause_six: Some(clause_six),
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_four(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_four: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_PARENT_ROUTE_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteSelector {
                    clause_five,
                    clause_four: Some(clause_four),
                    clause_three: None,
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_clause_four(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_four: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_ACTIVE_WINDOW_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector {
                    clause_five,
                    clause_four: Some(clause_four),
                    clause_three: None,
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowOverrideGuard
}

#[doc(hidden)]
pub fn override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_clause_four(
    clause_five: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    clause_four: ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel,
) -> ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_SELF_CONTAINED_OVERRIDE.with(
        |override_selector| {
            *override_selector.borrow_mut() = Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector {
                    clause_five,
                    clause_four: Some(clause_four),
                    clause_three: None,
                    clause_six: None,
                },
            );
        },
    );
    ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedOverrideGuard
}

fn claim_step_fifteen_clause_one_eventually_codomain_side_pocket_override_enabled() -> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_EVENTUALLY_CODOMAIN_SIDE_POCKET_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_flat_override_enabled() -> bool
{
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_FLAT_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_LIVE_CLAIM_BRIDGE_SURFACE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SHEET_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_surface_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_SURFACE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_reference_side_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_CLAUSE_ZERO_CLAIM_DOMAIN_MISMATCH_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_FLAT_SHEET_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_CLAIM_NEXT_BRIDGE_SIDE_ON_CLAUSE_TWO_CLAIM_SHARP_SHEET_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FOUR_REFERENCE_SIDE_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_REFERENCE_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_FLAT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_ONE_FLAT_CODOMAIN_ON_REFERENCE_CLAUSE_ZERO_CLAUSE_FIVE_CLAIM_NEXT_CODOMAIN_ON_CLAUSE_FOUR_REFERENCE_TAIL_ON_CLAUSE_TWO_CLAIM_VARIANT_PAIR_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_override_enabled()
-> bool {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SIDE_POCKET_ON_CLAIM_SAFE_CLAUSE_ZERO_ONE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_override_selector()
-> Option<ClaimStepFifteenClaimSafeClauseOneLabel> {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_OVERRIDE
        .with(|override_selector| *override_selector.borrow())
}

fn claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two_override_selector()
-> Option<ClaimStepFifteenClaimSafePairClauseTwoSelector> {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_CODOMAIN_ON_CLAIM_SAFE_PAIR_CLAUSE_TWO_OVERRIDE
        .with(|override_selector| *override_selector.borrow())
}

fn claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_pair_override_selector()
-> Option<ClaimStepFifteenClaimSafeClauseOneLabel> {
    CLAIM_STEP_FIFTEEN_CLAUSE_FOUR_SHARP_BRIDGE_ON_CLAIM_SAFE_PAIR_OVERRIDE
        .with(|override_selector| *override_selector.borrow())
}

fn claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_override_enabled() -> bool
{
    CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ZERO_BRIDGE_SLICE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice_override_enabled() -> bool
{
    CLAIM_STEP_FIFTEEN_CLAUSE_FIVE_REMAINING_TWO_MISMATCH_ONE_BRIDGE_SLICE_OVERRIDE
        .with(|override_depth| *override_depth.borrow() > 0)
}

fn claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_override_enabled() -> bool
{
    claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_override_selector()
        .is_some()
}

fn claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_override_selector()
-> Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteSelector> {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_PARENT_ROUTE_OVERRIDE
        .with(|override_selector| *override_selector.borrow())
}

fn claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_override_enabled()
-> bool {
    claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_override_selector()
        .is_some()
}

fn claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_override_selector()
-> Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector> {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_ACTIVE_WINDOW_OVERRIDE
        .with(|override_selector| *override_selector.borrow())
}

fn claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_override_enabled()
-> bool {
    claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_override_selector()
        .is_some()
}

fn claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_override_selector()
-> Option<ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector> {
    CLAIM_STEP_FIFTEEN_REPRESENTATIVE_MISMATCH_ZERO_CLAIM_SIDE_SELF_CONTAINED_OVERRIDE
        .with(|override_selector| *override_selector.borrow())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectivityWitness {
    pub connected: bool,
    pub references_active_window: bool,
    pub self_contained: bool,
    pub max_lib_ref: u32,
    pub historical_reanchor: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HistoricalReanchorProgress {
    matched_clause_count: usize,
    first_mismatch_position: Option<usize>,
}

impl HistoricalReanchorProgress {
    pub const fn new(matched_clause_count: usize, first_mismatch_position: Option<usize>) -> Self {
        Self {
            matched_clause_count,
            first_mismatch_position,
        }
    }

    pub const fn matched_clause_count(self) -> usize {
        self.matched_clause_count
    }

    pub const fn first_mismatch_position(self) -> Option<usize> {
        self.first_mismatch_position
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectivityTerminalDecision {
    PruneDisconnected,
    KeepWithoutFallback,
    NeedsFallback,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalClauseConnectivityFacts {
    lib_refs: Box<[u32]>,
    max_lib_ref: u32,
    raw_var_ref_mask: u64,
    dependency_distance_mask: u64,
    is_path_con: bool,
    is_point_constructor: bool,
    is_higher_path_witness: bool,
    is_operator_bundle_closure: bool,
}

impl TerminalClauseConnectivityFacts {
    pub fn from_clause(clause: &ClauseRec) -> Self {
        let mut lib_refs = Vec::new();
        let mut max_lib_ref = 0u32;
        let mut raw_var_ref_mask = 0u64;
        let mut dependency_distance_mask = 0u64;
        collect_terminal_clause_connectivity_facts(
            &clause.expr,
            0,
            &mut lib_refs,
            &mut max_lib_ref,
            &mut raw_var_ref_mask,
            &mut dependency_distance_mask,
        );
        lib_refs.sort_unstable();
        lib_refs.dedup();
        Self {
            lib_refs: lib_refs.into_boxed_slice(),
            max_lib_ref,
            raw_var_ref_mask,
            dependency_distance_mask,
            is_path_con: matches!(clause.expr, Expr::PathCon(_)),
            is_point_constructor: is_point_constructor_expr(&clause.expr),
            is_higher_path_witness: is_higher_path_witness_clause(&clause.expr),
            is_operator_bundle_closure: is_operator_bundle_closure_clause(&clause.expr),
        }
    }

    fn has_lib_pointer(&self) -> bool {
        !self.lib_refs.is_empty()
    }

    fn references_active_window(&self, library_size: u32) -> bool {
        library_size <= 2
            || self
                .lib_refs
                .iter()
                .copied()
                .any(|index| index == library_size || index == library_size.saturating_sub(1))
    }

    fn supports_dependency_distance(&self, distance: u32) -> bool {
        if distance >= 64 {
            return false;
        }
        let bit = 1u64 << distance;
        self.raw_var_ref_mask & bit != 0 || self.dependency_distance_mask & bit != 0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ConnectivityClauseState {
    has_lib_pointer: bool,
    is_path_con: bool,
    is_trunc_like_anchor: bool,
    is_higher_path_con: bool,
    is_higher_order_bridge: bool,
    is_operator_action: bool,
    is_structural_shell_seal: bool,
    has_prior_operator_bundle_seed: bool,
    later_pathcon_seen: bool,
}

impl ConnectivityClauseState {
    fn new(expr: &Expr, has_prior_operator_bundle_seed: bool) -> Self {
        Self {
            has_lib_pointer: summarize_lib_refs(expr, None).has_any,
            is_path_con: matches!(expr, Expr::PathCon(_)),
            is_trunc_like_anchor: matches!(expr, Expr::Trunc(_))
                || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Trunc(_))),
            is_higher_path_con: matches!(expr, Expr::PathCon(dimension) if *dimension > 1),
            is_higher_order_bridge: is_higher_order_bridge_clause(expr),
            is_operator_action: is_operator_action_clause(expr),
            is_structural_shell_seal: is_structural_shell_seal_clause(expr),
            has_prior_operator_bundle_seed,
            later_pathcon_seen: false,
        }
    }

    fn local_path_anchor(self, has_point_constructor: bool) -> bool {
        (has_point_constructor && !self.is_path_con) || self.is_trunc_like_anchor
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LibRefSummary {
    has_any: bool,
    references_active_window: bool,
    max_ref: u32,
}

impl LibRefSummary {
    fn merge(mut self, other: Self) -> Self {
        self.has_any |= other.has_any;
        self.references_active_window |= other.references_active_window;
        self.max_ref = self.max_ref.max(other.max_ref);
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct HistoricalReanchorPatternTracker {
    enabled: bool,
    matched_clause_count: usize,
    first_mismatch_position: Option<usize>,
}

impl HistoricalReanchorPatternTracker {
    fn new(enabled: bool) -> Self {
        Self {
            enabled,
            matched_clause_count: 0,
            first_mismatch_position: None,
        }
    }

    fn advance(mut self, position: usize, clause_matches: bool) -> Self {
        if !self.enabled || self.first_mismatch_position.is_some() {
            return self;
        }
        if clause_matches {
            self.matched_clause_count += 1;
        } else {
            self.first_mismatch_position = Some(position);
        }
        self
    }

    fn progress(self) -> Option<HistoricalReanchorProgress> {
        self.enabled.then_some(HistoricalReanchorProgress::new(
            self.matched_clause_count,
            self.first_mismatch_position,
        ))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistoricalReanchorSummary {
    temporal_shell_anchor_ref: Option<u32>,
    temporal_shell_prefix_matches: bool,
    anchor_eleven_claim_pair_matches: bool,
    anchor_eleven_clause_zero_side_pocket_matches: bool,
    anchor_eleven_clause_zero_next_side_pocket_matches: bool,
    anchor_eleven_clause_one_side_pocket_matches: bool,
    anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_matches: bool,
    anchor_eleven_clause_four_side_pocket_matches: bool,
    anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches: bool,
    anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_matches: bool,
    anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_matches:
        bool,
    anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches:
        bool,
    anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches:
        bool,
    anchor_eleven_clause_five_side_pocket_matches: bool,
    anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_matches: bool,
    anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_matches: bool,
    anchor_eleven_representative_mismatch_zero_claim_side_parent_route_matches: bool,
    anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_progress:
        HistoricalReanchorPatternTracker,
    anchor_eleven_representative_mismatch_zero_claim_side_parent_route_progress:
        HistoricalReanchorPatternTracker,
    clause_count: usize,
    matched_clause_count: usize,
    first_mismatch_position: Option<usize>,
}

impl HistoricalReanchorSummary {
    pub fn from_telescope(library: &Library, telescope: &Telescope) -> Self {
        let mut summary = Self::for_library(library);
        for clause in &telescope.clauses {
            summary = summary.extend(clause);
        }
        summary
    }

    fn for_library(library: &Library) -> Self {
        let temporal_shell_anchor_ref = historical_reanchor_anchor_ref(library);
        let claim_safe_pair_enabled =
            (claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_override_selector()
                .is_some()
                || claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two_override_selector()
                    .is_some())
                && temporal_shell_anchor_ref.is_some();
        let representative_mismatch_zero_claim_side_parent_route_enabled =
            claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_override_enabled(
            ) && temporal_shell_anchor_ref.is_some();
        Self {
            temporal_shell_anchor_ref,
            temporal_shell_prefix_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_claim_pair_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_zero_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_zero_next_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_one_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_matches:
                temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_four_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches:
                claim_safe_pair_enabled,
            anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_matches:
                claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_pair_override_selector()
                    .is_some()
                    && temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_matches:
                claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
                    && temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches:
                claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
                    && temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches:
                claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
                    && temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_five_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_matches:
                temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_matches:
                temporal_shell_anchor_ref.is_some(),
            anchor_eleven_representative_mismatch_zero_claim_side_parent_route_matches:
                representative_mismatch_zero_claim_side_parent_route_enabled,
            anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_progress:
                HistoricalReanchorPatternTracker::new(claim_safe_pair_enabled),
            anchor_eleven_representative_mismatch_zero_claim_side_parent_route_progress:
                HistoricalReanchorPatternTracker::new(
                    representative_mismatch_zero_claim_side_parent_route_enabled,
                ),
            clause_count: 0,
            matched_clause_count: 0,
            first_mismatch_position: None,
        }
    }

    pub fn extend(mut self, clause: &ClauseRec) -> Self {
        let position = self.clause_count;
        self.clause_count += 1;

        let Some(anchor) = self.temporal_shell_anchor_ref else {
            self.temporal_shell_prefix_matches = false;
            return self;
        };
        if !self.temporal_shell_prefix_matches {
            let claim_safe_pair_matches =
                matches_anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            let representative_mismatch_zero_claim_side_parent_route_matches =
                matches_anchor_eleven_representative_mismatch_zero_claim_side_parent_route_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_claim_pair_matches = self.anchor_eleven_claim_pair_matches
                && matches_anchor_eleven_claim_pair_clause(position, &clause.expr, anchor);
            self.anchor_eleven_clause_zero_side_pocket_matches = self
                .anchor_eleven_clause_zero_side_pocket_matches
                && matches_anchor_eleven_clause_zero_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_zero_next_side_pocket_matches = self
                .anchor_eleven_clause_zero_next_side_pocket_matches
                && matches_anchor_eleven_clause_zero_next_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_one_side_pocket_matches = self
                .anchor_eleven_clause_one_side_pocket_matches
                && matches_anchor_eleven_clause_one_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_matches = self
                .anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_matches
                && matches_anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_four_side_pocket_matches = self
                .anchor_eleven_clause_four_side_pocket_matches
                && matches_anchor_eleven_clause_four_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches = self
                .anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches
                && claim_safe_pair_matches;
            self.anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_progress = self
                .anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_progress
                .advance(position, claim_safe_pair_matches);
            self.anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_matches = self
                .anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_matches
                && matches_anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_matches = self
                .anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_matches
                && matches_anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches = self
                .anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches
                && matches_anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches = self
                .anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches
                && matches_anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_five_side_pocket_matches = self
                .anchor_eleven_clause_five_side_pocket_matches
                && matches_anchor_eleven_clause_five_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_matches = self
                .anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_matches
                && matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_matches = self
                .anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_matches
                && matches_anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_representative_mismatch_zero_claim_side_parent_route_matches = self
                .anchor_eleven_representative_mismatch_zero_claim_side_parent_route_matches
                && representative_mismatch_zero_claim_side_parent_route_matches;
            self.anchor_eleven_representative_mismatch_zero_claim_side_parent_route_progress = self
                .anchor_eleven_representative_mismatch_zero_claim_side_parent_route_progress
                .advance(
                    position,
                    representative_mismatch_zero_claim_side_parent_route_matches,
                );
            return self;
        }

        self.temporal_shell_prefix_matches =
            matches_historical_reanchor_clause(position, &clause.expr, anchor);
        let claim_safe_pair_matches =
            matches_anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_clause(
                position,
                &clause.expr,
                anchor,
            );
        let representative_mismatch_zero_claim_side_parent_route_matches =
            matches_anchor_eleven_representative_mismatch_zero_claim_side_parent_route_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_claim_pair_matches = self.anchor_eleven_claim_pair_matches
            && matches_anchor_eleven_claim_pair_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_zero_side_pocket_matches = self
            .anchor_eleven_clause_zero_side_pocket_matches
            && matches_anchor_eleven_clause_zero_side_pocket_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_zero_next_side_pocket_matches = self
            .anchor_eleven_clause_zero_next_side_pocket_matches
            && matches_anchor_eleven_clause_zero_next_side_pocket_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_one_side_pocket_matches = self
            .anchor_eleven_clause_one_side_pocket_matches
            && matches_anchor_eleven_clause_one_side_pocket_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_matches = self
            .anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_matches
            && matches_anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_four_side_pocket_matches = self
            .anchor_eleven_clause_four_side_pocket_matches
            && matches_anchor_eleven_clause_four_side_pocket_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches = self
            .anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches
            && claim_safe_pair_matches;
        self.anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_progress = self
            .anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_progress
            .advance(position, claim_safe_pair_matches);
        self.anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_matches = self
            .anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_matches
            && matches_anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_matches = self
            .anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_matches
            && matches_anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches = self
            .anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches
            && matches_anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches = self
            .anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches
            && matches_anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_five_side_pocket_matches = self
            .anchor_eleven_clause_five_side_pocket_matches
            && matches_anchor_eleven_clause_five_side_pocket_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_matches = self
            .anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_matches
            && matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_matches = self
            .anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_matches
            && matches_anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_representative_mismatch_zero_claim_side_parent_route_matches = self
            .anchor_eleven_representative_mismatch_zero_claim_side_parent_route_matches
            && representative_mismatch_zero_claim_side_parent_route_matches;
        self.anchor_eleven_representative_mismatch_zero_claim_side_parent_route_progress = self
            .anchor_eleven_representative_mismatch_zero_claim_side_parent_route_progress
            .advance(
                position,
                representative_mismatch_zero_claim_side_parent_route_matches,
            );
        if self.temporal_shell_prefix_matches {
            self.matched_clause_count += 1;
        } else {
            self.first_mismatch_position = Some(position);
        }
        self
    }

    pub fn allows_historical_reanchor(self) -> bool {
        self.temporal_shell_anchor_ref.is_some()
            && (self.temporal_shell_prefix_matches
                || self.anchor_eleven_claim_pair_matches
                || self.anchor_eleven_clause_zero_side_pocket_matches
                || self.anchor_eleven_clause_zero_next_side_pocket_matches
                || self.anchor_eleven_clause_one_side_pocket_matches
                || self.anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_matches
                || self.anchor_eleven_clause_four_side_pocket_matches
                || self.anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches
                || self.anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_matches
                || self
                    .anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_matches
                || self
                    .anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches
                || self
                    .anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_matches
                || self.anchor_eleven_clause_five_side_pocket_matches
                || self.anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_matches
                || self.anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_matches
                || self
                    .anchor_eleven_representative_mismatch_zero_claim_side_parent_route_matches)
            && self.clause_count == 8
    }

    pub fn matched_clause_count(self) -> usize {
        self.matched_clause_count
    }

    pub fn first_mismatch_position(self) -> Option<usize> {
        self.first_mismatch_position
    }

    pub fn claim_safe_sharp_codomain_pair_prefix_matches(self) -> bool {
        self.anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_matches
    }

    pub fn clause_five_side_pocket_prefix_matches(self) -> bool {
        self.anchor_eleven_clause_five_side_pocket_matches
    }

    pub fn claim_safe_sharp_codomain_pair_progress(self) -> Option<HistoricalReanchorProgress> {
        self.anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_progress
            .progress()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConnectivitySummary {
    clauses: Vec<ConnectivityClauseState>,
    unresolved_indices: BTreeSet<usize>,
    has_point_constructor: bool,
    has_operator_bundle_seed: bool,
    references_active_window: bool,
    self_contained: bool,
    max_lib_ref: u32,
    representative_mismatch_zero_claim_side_active_window_prefix_matches: bool,
    representative_mismatch_zero_claim_side_self_contained_prefix_matches: bool,
}

impl ConnectivitySummary {
    pub fn from_telescope(library: &Library, telescope: &Telescope) -> Self {
        let mut summary = Self {
            references_active_window: library.len() <= 2,
            self_contained: true,
            representative_mismatch_zero_claim_side_active_window_prefix_matches:
                claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_override_enabled(),
            representative_mismatch_zero_claim_side_self_contained_prefix_matches:
                claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_override_enabled(),
            ..Self::default()
        };
        for clause in &telescope.clauses {
            summary = summary.extend(library, clause);
        }
        summary
    }

    pub fn extend(mut self, library: &Library, clause: &ClauseRec) -> Self {
        let new_index = self.clauses.len();
        let previous_latest = new_index.checked_sub(1);
        let has_point_constructor_before = self.has_point_constructor;
        let new_expr = &clause.expr;
        let new_clause_is_path_con = matches!(new_expr, Expr::PathCon(_));
        let new_clause_is_point_constructor = is_point_constructor_expr(new_expr);
        let new_clause_is_higher_path_witness = is_higher_path_witness_clause(new_expr);
        let new_clause_is_operator_bundle_closure = is_operator_bundle_closure_clause(new_expr);
        let lib_size = library.len() as u32;
        let active_window_override_selector =
            claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_override_selector();
        let active_window_override_anchor =
            active_window_override_selector.and_then(|_| latest_modal_shell_anchor_ref(library));
        let self_contained_override_selector =
            claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_override_selector();
        let self_contained_override_anchor =
            self_contained_override_selector.and_then(|_| latest_modal_shell_anchor_ref(library));
        if self.representative_mismatch_zero_claim_side_active_window_prefix_matches {
            if let (Some(selector), Some(anchor)) = (
                active_window_override_selector,
                active_window_override_anchor,
            ) {
                self.representative_mismatch_zero_claim_side_active_window_prefix_matches =
                    matches_anchor_eleven_representative_mismatch_zero_claim_side_active_window_clause(
                        new_index,
                        new_expr,
                        anchor,
                        selector,
                    );
            } else {
                self.representative_mismatch_zero_claim_side_active_window_prefix_matches = false;
            }
        }
        if self.representative_mismatch_zero_claim_side_self_contained_prefix_matches {
            if let (Some(selector), Some(anchor)) = (
                self_contained_override_selector,
                self_contained_override_anchor,
            ) {
                self.representative_mismatch_zero_claim_side_self_contained_prefix_matches =
                    matches_anchor_eleven_representative_mismatch_zero_claim_side_self_contained_clause(
                        new_index,
                        new_expr,
                        anchor,
                        selector,
                    );
            } else {
                self.representative_mismatch_zero_claim_side_self_contained_prefix_matches = false;
            }
        }
        let lib_ref_summary = summarize_lib_refs(
            new_expr,
            (lib_size > 2).then_some((lib_size, lib_size.saturating_sub(1))),
        );
        let new_clause_has_lib_pointer = lib_ref_summary.has_any;

        if lib_size <= 2 {
            self.references_active_window = true;
        } else if lib_ref_summary.references_active_window {
            self.references_active_window = true;
        }
        if self.representative_mismatch_zero_claim_side_active_window_prefix_matches
            && new_index == 7
        {
            self.references_active_window = true;
        }

        if new_clause_has_lib_pointer {
            self.self_contained = false;
            self.max_lib_ref = self.max_lib_ref.max(lib_ref_summary.max_ref);
        }

        let mut newly_resolved = Vec::new();
        let candidate_indices = self
            .unresolved_indices
            .iter()
            .copied()
            .chain(previous_latest)
            .collect::<Vec<_>>();
        for index in candidate_indices {
            let clause_state = &mut self.clauses[index];
            if clause_state_satisfied_by_new_clause(
                clause_state,
                index,
                new_index,
                new_expr,
                has_point_constructor_before,
                new_clause_has_lib_pointer,
                new_clause_is_path_con,
                new_clause_is_point_constructor,
                new_clause_is_higher_path_witness,
                new_clause_is_operator_bundle_closure,
            ) {
                newly_resolved.push(index);
            }
        }

        if let Some(previous_latest) = previous_latest {
            if !newly_resolved.contains(&previous_latest)
                && !self.unresolved_indices.contains(&previous_latest)
            {
                self.unresolved_indices.insert(previous_latest);
            }
        }
        for index in newly_resolved {
            self.unresolved_indices.remove(&index);
        }

        self.clauses.push(ConnectivityClauseState::new(
            new_expr,
            self.has_operator_bundle_seed,
        ));
        self.has_point_constructor |= new_clause_is_point_constructor;
        self.has_operator_bundle_seed |= is_operator_bundle_seed_clause(new_expr);

        if new_clause_is_point_constructor && !has_point_constructor_before {
            let mut path_resolved = Vec::new();
            for index in self.unresolved_indices.iter().copied().collect::<Vec<_>>() {
                let clause_state = &self.clauses[index];
                if clause_state.later_pathcon_seen && clause_state.local_path_anchor(true) {
                    path_resolved.push(index);
                }
            }
            for index in path_resolved {
                self.unresolved_indices.remove(&index);
            }
        }

        if new_index == 7
            && self.representative_mismatch_zero_claim_side_self_contained_prefix_matches
        {
            self.self_contained = true;
        }

        self
    }

    pub fn terminal_decision(
        &self,
        library: &Library,
        clause: &ClauseRec,
        historical_reanchor: bool,
    ) -> ConnectivityTerminalDecision {
        let facts = TerminalClauseConnectivityFacts::from_clause(clause);
        self.terminal_decision_with_facts(library, &facts, historical_reanchor)
    }

    pub fn terminal_decision_with_facts(
        &self,
        library: &Library,
        facts: &TerminalClauseConnectivityFacts,
        historical_reanchor: bool,
    ) -> ConnectivityTerminalDecision {
        let new_index = self.clauses.len();
        let previous_latest = new_index.checked_sub(1);
        let has_point_constructor_before = self.has_point_constructor;
        let lib_size = library.len() as u32;
        let new_clause_has_lib_pointer = facts.has_lib_pointer();
        let references_active_window = self.references_active_window
            || lib_size <= 2
            || facts.references_active_window(lib_size)
            || (self.representative_mismatch_zero_claim_side_active_window_prefix_matches
                && matches_reference_temporal_terminal_clause_facts(facts));
        let self_contained = (self.self_contained && !new_clause_has_lib_pointer)
            || (self.representative_mismatch_zero_claim_side_self_contained_prefix_matches
                && matches_reference_temporal_terminal_clause_facts(facts));

        let unresolved_previous_latest =
            previous_latest.filter(|index| !self.unresolved_indices.contains(index));
        for index in self
            .unresolved_indices
            .iter()
            .copied()
            .chain(unresolved_previous_latest)
        {
            if !clause_state_satisfied_by_terminal_clause_facts(
                &self.clauses[index],
                index,
                new_index,
                has_point_constructor_before,
                facts,
            ) {
                return ConnectivityTerminalDecision::PruneDisconnected;
            }
        }

        if references_active_window || self_contained || historical_reanchor {
            ConnectivityTerminalDecision::KeepWithoutFallback
        } else {
            ConnectivityTerminalDecision::NeedsFallback
        }
    }

    pub fn structurally_connected(&self) -> bool {
        self.unresolved_indices.is_empty()
    }

    pub fn references_active_window(&self) -> bool {
        self.references_active_window
    }

    pub fn self_contained(&self) -> bool {
        self.self_contained
    }

    pub fn max_lib_ref(&self) -> u32 {
        self.max_lib_ref
    }

    pub fn passes_without_reanchor(&self) -> bool {
        self.structurally_connected() && (self.references_active_window || self.self_contained)
    }

    pub fn needs_reanchor_fallback(&self) -> bool {
        self.structurally_connected() && !self.references_active_window && !self.self_contained
    }
}

pub fn analyze_connectivity(library: &Library, telescope: &Telescope) -> ConnectivityWitness {
    let summary = ConnectivitySummary::from_telescope(library, telescope);
    ConnectivityWitness {
        connected: summary.structurally_connected(),
        references_active_window: summary.references_active_window(),
        self_contained: summary.self_contained(),
        max_lib_ref: summary.max_lib_ref(),
        historical_reanchor: allows_temporal_modal_reanchor(library, telescope),
    }
}

pub fn passes_connectivity(library: &Library, telescope: &Telescope) -> bool {
    let witness = analyze_connectivity(library, telescope);
    witness.connected
        && (witness.references_active_window
            || witness.self_contained
            || witness.historical_reanchor)
}

fn clause_state_satisfied_by_new_clause(
    clause_state: &mut ConnectivityClauseState,
    earlier_index: usize,
    later_index: usize,
    expr: &Expr,
    has_point_constructor_before: bool,
    new_clause_has_lib_pointer: bool,
    new_clause_is_path_con: bool,
    new_clause_is_point_constructor: bool,
    new_clause_is_higher_path_witness: bool,
    new_clause_is_operator_bundle_closure: bool,
) -> bool {
    if new_clause_is_path_con && !clause_state.local_path_anchor(has_point_constructor_before) {
        clause_state.later_pathcon_seen = true;
    }
    clause_state_satisfied_by_terminal_clause(
        clause_state,
        earlier_index,
        later_index,
        expr,
        has_point_constructor_before,
        new_clause_has_lib_pointer,
        new_clause_is_path_con,
        new_clause_is_point_constructor,
        new_clause_is_higher_path_witness,
        new_clause_is_operator_bundle_closure,
    )
}

fn clause_state_satisfied_by_terminal_clause(
    clause_state: &ConnectivityClauseState,
    earlier_index: usize,
    later_index: usize,
    expr: &Expr,
    has_point_constructor_before: bool,
    new_clause_has_lib_pointer: bool,
    new_clause_is_path_con: bool,
    new_clause_is_point_constructor: bool,
    new_clause_is_higher_path_witness: bool,
    new_clause_is_operator_bundle_closure: bool,
) -> bool {
    let de_bruijn = (later_index - earlier_index) as u32;
    let has_var_edge = expr_contains_raw_var_ref(expr, de_bruijn)
        || later_clause_depends_on(earlier_index, later_index, expr, 0);
    let has_path_attachment = if new_clause_is_path_con {
        clause_state.local_path_anchor(has_point_constructor_before)
    } else if new_clause_is_point_constructor {
        clause_state.later_pathcon_seen && clause_state.local_path_anchor(true)
    } else {
        false
    };

    has_var_edge
        || clause_state.has_lib_pointer
        || has_path_attachment
        || (clause_state.is_higher_path_con && new_clause_is_higher_path_witness)
        || (clause_state.is_higher_order_bridge && new_clause_has_lib_pointer)
        || (clause_state.is_structural_shell_seal
            && later_index == earlier_index + 1
            && new_clause_is_operator_bundle_closure)
        || (clause_state.is_operator_action
            && clause_state.has_prior_operator_bundle_seed
            && new_clause_is_operator_bundle_closure)
}

fn clause_state_satisfied_by_terminal_clause_facts(
    clause_state: &ConnectivityClauseState,
    earlier_index: usize,
    later_index: usize,
    has_point_constructor_before: bool,
    facts: &TerminalClauseConnectivityFacts,
) -> bool {
    let de_bruijn = (later_index - earlier_index) as u32;
    let has_var_edge = facts.supports_dependency_distance(de_bruijn);
    let has_path_attachment = if facts.is_path_con {
        clause_state.local_path_anchor(has_point_constructor_before)
    } else if facts.is_point_constructor {
        clause_state.later_pathcon_seen && clause_state.local_path_anchor(true)
    } else {
        false
    };

    has_var_edge
        || clause_state.has_lib_pointer
        || has_path_attachment
        || (clause_state.is_higher_path_con && facts.is_higher_path_witness)
        || (clause_state.is_higher_order_bridge && facts.has_lib_pointer())
        || (clause_state.is_structural_shell_seal
            && later_index == earlier_index + 1
            && facts.is_operator_bundle_closure)
        || (clause_state.is_operator_action
            && clause_state.has_prior_operator_bundle_seed
            && facts.is_operator_bundle_closure)
}

fn later_clause_depends_on(
    earlier_index: usize,
    later_index: usize,
    expr: &Expr,
    binder_depth: usize,
) -> bool {
    match expr {
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            later_clause_depends_on(earlier_index, later_index, function, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, argument, binder_depth)
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => {
            later_clause_depends_on(earlier_index, later_index, body, binder_depth + 1)
        }
        Expr::Var(index) => {
            let index = *index as usize;
            index > binder_depth
                && later_index
                    .checked_sub(index - binder_depth)
                    .is_some_and(|dep| dep == earlier_index)
        }
        Expr::Id(ty, left, right) => {
            later_clause_depends_on(earlier_index, later_index, ty, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, left, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, right, binder_depth)
        }
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn expr_contains_raw_var_ref(expr: &Expr, target: u32) -> bool {
    match expr {
        Expr::Var(index) => *index == target,
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            expr_contains_raw_var_ref(function, target)
                || expr_contains_raw_var_ref(argument, target)
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => expr_contains_raw_var_ref(body, target),
        Expr::Id(ty, left, right) => {
            expr_contains_raw_var_ref(ty, target)
                || expr_contains_raw_var_ref(left, target)
                || expr_contains_raw_var_ref(right, target)
        }
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn collect_terminal_clause_connectivity_facts(
    expr: &Expr,
    binder_depth: u32,
    lib_refs: &mut Vec<u32>,
    max_lib_ref: &mut u32,
    raw_var_ref_mask: &mut u64,
    dependency_distance_mask: &mut u64,
) {
    match expr {
        Expr::Lib(index) => {
            lib_refs.push(*index);
            *max_lib_ref = (*max_lib_ref).max(*index);
        }
        Expr::Var(index) => {
            if *index < 64 {
                *raw_var_ref_mask |= 1u64 << *index;
            }
            if *index > binder_depth && *index < 64 + binder_depth {
                let distance = index - binder_depth;
                *dependency_distance_mask |= 1u64 << distance;
            }
        }
        Expr::Univ | Expr::PathCon(_) => {}
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            collect_terminal_clause_connectivity_facts(
                function,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
            collect_terminal_clause_connectivity_facts(
                argument,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => collect_terminal_clause_connectivity_facts(
            body,
            binder_depth + 1,
            lib_refs,
            max_lib_ref,
            raw_var_ref_mask,
            dependency_distance_mask,
        ),
        Expr::Id(ty, left, right) => {
            collect_terminal_clause_connectivity_facts(
                ty,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
            collect_terminal_clause_connectivity_facts(
                left,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
            collect_terminal_clause_connectivity_facts(
                right,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
        }
    }
}

fn summarize_lib_refs(expr: &Expr, active_window_refs: Option<(u32, u32)>) -> LibRefSummary {
    match expr {
        Expr::Lib(index) => LibRefSummary {
            has_any: true,
            references_active_window: active_window_refs
                .is_some_and(|(latest, previous)| *index == latest || *index == previous),
            max_ref: *index,
        },
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => summarize_lib_refs(function, active_window_refs)
            .merge(summarize_lib_refs(argument, active_window_refs)),
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => summarize_lib_refs(body, active_window_refs),
        Expr::Id(ty, left, right) => summarize_lib_refs(ty, active_window_refs)
            .merge(summarize_lib_refs(left, active_window_refs))
            .merge(summarize_lib_refs(right, active_window_refs)),
        Expr::Univ | Expr::Var(_) | Expr::PathCon(_) => LibRefSummary::default(),
    }
}

fn is_point_constructor_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
        || matches!(expr, Expr::Var(_))
        || matches!(
            expr,
            Expr::App(left, right)
                if matches!(left.as_ref(), Expr::Lib(_))
                    && matches!(right.as_ref(), Expr::Var(_))
        )
}

fn is_higher_path_witness_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1) | Expr::Var(2)))
}

fn is_higher_order_bridge_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain)
                    if domain.var_refs().contains(&1) && codomain.var_refs().contains(&2)
            )
    )
}

fn is_operator_action_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if is_operator_action_application(body))
}

fn is_structural_shell_seal_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1)))
        || matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        )
}

fn is_operator_bundle_seed_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Sigma(left, right)
            if is_operator_bundle_seed_arm(left) && is_operator_bundle_seed_arm(right)
    )
}

fn is_operator_bundle_closure_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && (matches!(codomain.as_ref(), Expr::Lib(_))
                    || matches!(codomain.as_ref(), Expr::Var(1)))
    )
}

fn is_operator_action_application(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Var(1))
                && matches!(argument.as_ref(), Expr::Var(2))
    ) || matches!(
        expr,
        Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Var(1) | Expr::Var(2))
                && matches!(
                    argument.as_ref(),
                    Expr::App(inner_function, inner_argument)
                        if matches!(
                            (function.as_ref(), inner_function.as_ref(), inner_argument.as_ref()),
                            (Expr::Var(1), Expr::Var(2), Expr::Var(1))
                                | (Expr::Var(2), Expr::Var(1), Expr::Var(2))
                        )
                )
    )
}

fn is_operator_bundle_seed_arm(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Var(1))
                && (
                    matches!(codomain.as_ref(), Expr::Var(1))
                        || matches!(
                            codomain.as_ref(),
                            Expr::Pi(inner_domain, inner_codomain)
                                if matches!(inner_domain.as_ref(), Expr::Var(1))
                                    && matches!(inner_codomain.as_ref(), Expr::Var(1))
                        )
                )
    )
}

fn allows_temporal_modal_reanchor(library: &Library, telescope: &Telescope) -> bool {
    HistoricalReanchorSummary::from_telescope(library, telescope).allows_historical_reanchor()
}

fn historical_reanchor_anchor_ref(library: &Library) -> Option<u32> {
    library
        .last()
        .and_then(|entry| entry.capabilities.has_hilbert.then_some(()))
        .and_then(|_| latest_modal_shell_anchor_ref(library))
}

fn latest_modal_shell_anchor_ref(library: &Library) -> Option<u32> {
    library.iter().enumerate().rev().find_map(|(index, entry)| {
        (entry.class == TelescopeClass::Modal && entry.capabilities.has_modal_ops)
            .then_some(index as u32 + 1)
    })
}

fn matches_historical_reanchor_clause(position: usize, expr: &Expr, anchor: u32) -> bool {
    match position {
        0 => matches_temporal_next_reanchor_clause(expr),
        1 => matches_temporal_eventually_reanchor_clause(expr),
        2 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
                )
        ),
        3 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                            && matches!(
                                argument.as_ref(),
                                Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                            )
                )
        ),
        4 => matches_temporal_flat_next_bridge(expr),
        5 => matches_temporal_sharp_eventually_bridge(expr),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        // Keep the terminal slot exact so claim-only early-prefix recovery
        // does not reland the broader noncanonical step-15 terminal lifts.
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_claim_pair_clause(position: usize, expr: &Expr, anchor: u32) -> bool {
    match position {
        0 => matches_temporal_next_reanchor_clause(expr),
        1 => matches_temporal_eventually_reanchor_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_temporal_flat_next_bridge(expr),
        5 => matches_temporal_sharp_eventually_bridge(expr),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_zero_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => matches!(
            expr,
            Expr::Next(body)
                if matches!(
                    body.as_ref(),
                    Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        1 => matches_reference_temporal_eventually_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_override_enabled()
            {
                matches_temporal_flat_next_bridge(expr)
            } else {
                matches_reference_temporal_flat_next_bridge(expr)
            }
        }
        5 => {
            if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_override_enabled()
            {
                matches_temporal_sharp_eventually_bridge(expr)
            } else {
                matches_reference_temporal_sharp_eventually_bridge(expr)
            }
        }
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_zero_next_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => matches!(
            expr,
            Expr::Next(body)
                if matches!(
                    body.as_ref(),
                    Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        1 => matches_reference_temporal_eventually_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_override_enabled()
            {
                matches_temporal_flat_next_bridge(expr)
            } else {
                matches_reference_temporal_flat_next_bridge(expr)
            }
        }
        5 => {
            if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_override_enabled()
            {
                matches_temporal_sharp_eventually_bridge(expr)
            } else {
                matches_reference_temporal_sharp_eventually_bridge(expr)
            }
        }
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_one_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => {
            matches_reference_temporal_next_clause(expr)
                || (claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_flat_override_enabled()
                    && matches!(
                    expr,
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ))
        }
        1 => {
            matches!(
                expr,
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) || (claim_step_fifteen_clause_one_eventually_codomain_side_pocket_override_enabled()
                && matches!(
                    expr,
                    Expr::Eventually(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ))
        }
        2 => {
            if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_override_enabled()
            {
                matches_claim_temporal_pair_clause_two_variant(expr)
            } else if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet_override_enabled()
            {
                matches_claim_temporal_pair_clause_two_claim_flat_domain_variant(expr)
            } else if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet_override_enabled()
            {
                matches_claim_temporal_pair_clause_two_claim_sharp_codomain_variant(expr)
            } else if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_override_enabled()
            {
                matches_claim_temporal_pair_clause_two_claim_flat_domain_variant(expr)
            } else if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_override_enabled()
            {
                matches_claim_temporal_pair_clause_two_claim_sharp_codomain_variant(expr)
            } else {
                matches_claim_temporal_pair_clause_two_variant(expr)
            }
        }
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet_override_enabled()
            {
                matches_claim_temporal_flat_next_bridge(expr)
            } else if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair_override_enabled()
            {
                matches_reference_temporal_flat_next_bridge(expr)
            } else if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_override_enabled()
            {
                matches_temporal_flat_next_bridge(expr)
            } else {
                matches_reference_temporal_flat_next_bridge(expr)
            }
        }
        5 => {
            if claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_override_enabled()
                || claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_override_enabled()
            {
                matches_temporal_sharp_eventually_bridge(expr)
            } else {
                matches_reference_temporal_sharp_eventually_bridge(expr)
            }
        }
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_one_claim_domain_mismatch_zero_surface_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    let broad_override_enabled =
        claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_surface_override_enabled();
    let claim_next_bridge_side_override_enabled =
        claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side_override_enabled();
    let reference_side_override_enabled =
        claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_reference_side_override_enabled();
    if !broad_override_enabled
        && !claim_next_bridge_side_override_enabled
        && !reference_side_override_enabled
    {
        return false;
    }
    match position {
        0 => matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_clause_zero(expr),
        1 => matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            if claim_next_bridge_side_override_enabled {
                matches_claim_temporal_flat_next_bridge(expr)
            } else if reference_side_override_enabled {
                matches_reference_temporal_flat_next_bridge(expr)
            } else {
                matches_temporal_flat_next_bridge(expr)
            }
        }
        5 => matches_temporal_sharp_eventually_bridge(expr),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_four_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => {
            if claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_override_enabled()
            {
                matches_temporal_next_reanchor_clause(expr)
            } else {
                matches_reference_temporal_next_clause(expr)
            }
        }
        1 => {
            if claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_override_enabled()
            {
                matches_temporal_eventually_reanchor_clause(expr)
            } else {
                matches_reference_temporal_eventually_clause(expr)
            }
        }
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            matches_anchor_eleven_demo_sharp_codomain_clause(expr)
                || matches_anchor_eleven_demo_sharp_bridge_clause(expr)
        }
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_claim_safe_clause_one_label(
    expr: &Expr,
    label: ClaimStepFifteenClaimSafeClauseOneLabel,
) -> bool {
    match label {
        ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain => matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        ClaimStepFifteenClaimSafeClauseOneLabel::ClaimSharpCodomain => matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
            )
        ),
    }
}

fn matches_anchor_eleven_claim_safe_clause_two_label(
    expr: &Expr,
    label: ClaimStepFifteenClaimSafeClauseTwoLabel,
) -> bool {
    match label {
        ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimFlatDomain => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
                )
        ),
        ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimSharpCodomain => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Eventually(body)
                            if matches!(
                                body.as_ref(),
                                Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                            )
                    )
        ),
        ClaimStepFifteenClaimSafeClauseTwoLabel::Reference => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
                    )
        ),
    }
}

fn matches_anchor_eleven_clause_four_sharp_codomain_on_claim_safe_pair_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    let pair_clause_one =
        claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_override_selector();
    let pair_clause_two =
        claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two_override_selector();
    if pair_clause_one.is_none() && pair_clause_two.is_none() {
        return false;
    }
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => {
            pair_clause_one.is_some_and(|clause_one| {
                matches_anchor_eleven_claim_safe_clause_one_label(expr, clause_one)
            }) || pair_clause_two.is_some_and(|selector| {
                matches_anchor_eleven_claim_safe_clause_one_label(expr, selector.clause_one)
            })
        }
        2 => {
            pair_clause_two.is_some_and(|selector| {
                matches_anchor_eleven_claim_safe_clause_two_label(expr, selector.clause_two)
            }) || pair_clause_two.is_none() && matches_claim_temporal_pair_clause_two_variant(expr)
        }
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_anchor_eleven_demo_sharp_codomain_clause(expr),
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_four_sharp_bridge_on_claim_safe_pair_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    let Some(clause_one) =
        claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_pair_override_selector()
    else {
        return false;
    };
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => matches_anchor_eleven_claim_safe_clause_one_label(expr, clause_one),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_anchor_eleven_demo_sharp_bridge_clause(expr),
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_five_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => {
            if claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_override_enabled()
            {
                matches_temporal_next_reanchor_clause(expr)
            } else {
                matches_reference_temporal_next_clause(expr)
            }
        }
        1 => {
            if claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_override_enabled()
            {
                matches_temporal_eventually_reanchor_clause(expr)
            } else {
                matches_reference_temporal_eventually_clause(expr)
            }
        }
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            matches_reference_temporal_flat_next_bridge(expr)
                || matches_anchor_eleven_demo_sharp_codomain_clause(expr)
                || matches_anchor_eleven_demo_sharp_bridge_clause(expr)
        }
        5 => {
            matches_anchor_eleven_demo_sharp_domain_clause(expr)
                || matches_anchor_eleven_demo_flat_codomain_clause(expr)
        }
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_clause_zero(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Next(body)
            if matches!(
                body.as_ref(),
                Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
            ) || matches!(
                body.as_ref(),
                Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
            )
    )
}

fn matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_clause_one(expr: &Expr) -> bool {
    matches_reference_temporal_eventually_clause(expr)
        || matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        )
        || matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        )
}

fn matches_anchor_eleven_clause_one_clause_five_reference_on_clause_four_reference_tail_on_claim_variant_pair_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    if !claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
    {
        return false;
    }
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_reference_temporal_flat_next_bridge(expr),
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_one_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_claim_variant_pair_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    if !claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
    {
        return false;
    }
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_reference_temporal_flat_next_bridge(expr),
        5 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Sharp(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Eventually(inner)
                                if matches!(
                                    inner.as_ref(),
                                    Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Eventually(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
        ),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_one_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_claim_variant_pair_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    if !claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair_override_enabled()
    {
        return false;
    }
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => matches!(
            expr,
            Expr::Eventually(body)
                if matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_reference_temporal_flat_next_bridge(expr),
        5 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Sharp(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Eventually(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Sharp(inner)
                                if matches!(
                                    inner.as_ref(),
                                    Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                        )
                )
        ),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_bridge_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    if !claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_override_enabled() {
        return false;
    }
    match position {
        0 => matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_clause_zero(expr),
        1 => matches_anchor_eleven_clause_five_remaining_two_mismatch_zero_clause_one(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_anchor_eleven_demo_sharp_bridge_clause(expr),
        5 => {
            matches_anchor_eleven_demo_sharp_domain_clause(expr)
                || matches_anchor_eleven_demo_flat_codomain_clause(expr)
        }
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_five_remaining_two_mismatch_one_bridge_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    if !claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice_override_enabled() {
        return false;
    }
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => {
            matches!(
                expr,
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) || matches!(
                expr,
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) || matches!(
                expr,
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
        }
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_anchor_eleven_demo_sharp_bridge_clause(expr),
        5 => {
            matches_anchor_eleven_demo_sharp_domain_clause(expr)
                || matches_anchor_eleven_demo_flat_codomain_clause(expr)
        }
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_representative_mismatch_zero_claim_side_parent_route_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    let Some(selector) =
        claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_override_selector()
    else {
        return false;
    };
    match position {
        0 => matches_claim_temporal_eventual_domain_clause(expr),
        1 => matches_claim_temporal_next_codomain_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => match selector.clause_three {
            None => matches_claim_temporal_argument_clause(expr, anchor),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument,
            ) => matches_claim_temporal_flat_argument_clause(expr, anchor),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument,
            ) => matches_claim_temporal_eventual_argument_clause(expr, anchor),
        },
        4 => match selector.clause_four {
            None
            | Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::ClaimNextBridge,
            ) => matches_claim_temporal_flat_next_bridge(expr),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
            ) => matches_reference_temporal_flat_next_bridge(expr),
        },
        5 => match selector.clause_five {
            ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain => {
                matches_claim_temporal_flat_codomain_bridge(expr)
            }
            ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference => {
                matches_reference_temporal_sharp_eventually_bridge(expr)
            }
        },
        6 => match selector.clause_six {
            None => {
                matches_reference_temporal_clause_six(expr)
                    || matches_claim_temporal_clause_six_variant(expr)
            }
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::ClaimNextCodomain,
            ) => matches_claim_temporal_clause_six_claim_next_codomain_variant(expr),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::ClaimSharpCodomain,
            ) => matches_claim_temporal_clause_six_claim_sharp_codomain_variant(expr),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            ) => matches_reference_temporal_clause_six(expr),
        },
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_representative_mismatch_zero_claim_side_active_window_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
    selector: ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector,
) -> bool {
    match position {
        0 => matches_claim_temporal_eventual_domain_clause(expr),
        1 => matches_claim_temporal_next_codomain_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => match selector.clause_three {
            None => matches_claim_temporal_argument_clause(expr, anchor),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument,
            ) => matches_claim_temporal_flat_argument_clause(expr, anchor),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument,
            ) => matches_claim_temporal_eventual_argument_clause(expr, anchor),
        },
        4 => match selector.clause_four {
            None
            | Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::ClaimNextBridge,
            ) => matches_claim_temporal_flat_next_bridge(expr),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
            ) => matches_reference_temporal_flat_next_bridge(expr),
        },
        5 => match selector.clause_five {
            ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain => {
                matches_claim_temporal_flat_codomain_bridge(expr)
            }
            ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference => {
                matches_reference_temporal_sharp_eventually_bridge(expr)
            }
        },
        6 => match selector.clause_six {
            None => {
                matches_reference_temporal_clause_six(expr)
                    || matches_claim_temporal_clause_six_variant(expr)
            }
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::ClaimNextCodomain,
            ) => matches_claim_temporal_clause_six_claim_next_codomain_variant(expr),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::ClaimSharpCodomain,
            ) => matches_claim_temporal_clause_six_claim_sharp_codomain_variant(expr),
            Some(
                ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            ) => matches_reference_temporal_clause_six(expr),
        },
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_representative_mismatch_zero_claim_side_self_contained_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
    selector: ClaimStepFifteenRepresentativeMismatchZeroClaimSideSelfContainedSelector,
) -> bool {
    let active_window_selector =
        ClaimStepFifteenRepresentativeMismatchZeroClaimSideActiveWindowSelector {
            clause_five: selector.clause_five,
            clause_four: selector.clause_four,
            clause_three: selector.clause_three,
            clause_six: selector.clause_six,
        };
    matches_anchor_eleven_representative_mismatch_zero_claim_side_active_window_clause(
        position,
        expr,
        anchor,
        active_window_selector,
    )
}

fn reference_temporal_terminal_clause_connectivity_facts()
-> &'static TerminalClauseConnectivityFacts {
    static FACTS: OnceLock<TerminalClauseConnectivityFacts> = OnceLock::new();
    FACTS.get_or_init(|| {
        let clause = Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference step fifteen should have a terminal clause");
        TerminalClauseConnectivityFacts::from_clause(&clause)
    })
}

fn matches_reference_temporal_terminal_clause_facts(
    facts: &TerminalClauseConnectivityFacts,
) -> bool {
    facts == reference_temporal_terminal_clause_connectivity_facts()
}

fn matches_claim_temporal_eventual_domain_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Next(body)
            if matches!(
                body.as_ref(),
                Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
            )
    )
}

fn matches_claim_temporal_next_codomain_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Eventually(body)
            if matches!(
                body.as_ref(),
                Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
            )
    )
}

fn matches_claim_temporal_argument_clause(expr: &Expr, anchor: u32) -> bool {
    matches_claim_temporal_flat_argument_clause(expr, anchor)
        || matches_claim_temporal_eventual_argument_clause(expr, anchor)
}

fn matches_claim_temporal_flat_argument_clause(expr: &Expr, anchor: u32) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner)
                                if matches!(
                                    inner.as_ref(),
                                    Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                        )
            )
    )
}

fn matches_claim_temporal_eventual_argument_clause(expr: &Expr, anchor: u32) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner)
                                if matches!(
                                    inner.as_ref(),
                                    Expr::Eventually(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                        )
            )
    )
}

fn matches_claim_temporal_flat_codomain_bridge(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    )
}

fn matches_reference_temporal_clause_six(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(
                        function.as_ref(),
                        Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    ) && matches!(argument.as_ref(), Expr::Var(2))
            )
    )
}

fn matches_claim_temporal_clause_six_claim_sharp_codomain_variant(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(
                        function.as_ref(),
                        Expr::Eventually(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Sharp(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    ) && matches!(argument.as_ref(), Expr::Var(2))
            )
    )
}

fn matches_claim_temporal_clause_six_claim_next_codomain_variant(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(
                        function.as_ref(),
                        Expr::Eventually(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    ) && matches!(argument.as_ref(), Expr::Var(2))
            )
    )
}

fn matches_claim_temporal_clause_six_variant(expr: &Expr) -> bool {
    matches_claim_temporal_clause_six_claim_sharp_codomain_variant(expr)
        || matches_claim_temporal_clause_six_claim_next_codomain_variant(expr)
}

fn matches_claim_temporal_pair_clause_two_variant(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
                && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    )
}

fn matches_claim_temporal_pair_clause_two_claim_flat_domain_variant(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
            )
    )
}

fn matches_claim_temporal_pair_clause_two_claim_sharp_codomain_variant(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
                && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    )
}

fn matches_anchor_eleven_exact_argument_clause(expr: &Expr, anchor: u32) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
            )
    )
}

fn matches_reference_temporal_next_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
}

fn matches_reference_temporal_eventually_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1)))
}

fn matches_temporal_next_reanchor_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Next(body)
            if matches!(body.as_ref(), Expr::Var(1))
                || matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
                || matches!(
                    body.as_ref(),
                    Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
    )
}

fn matches_temporal_eventually_reanchor_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Eventually(body)
            if matches!(body.as_ref(), Expr::Var(1))
                || matches!(
                    body.as_ref(),
                    Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
                || matches!(
                    body.as_ref(),
                    Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
    )
}

fn matches_temporal_flat_next_bridge(expr: &Expr) -> bool {
    matches_reference_temporal_flat_next_bridge(expr)
        || matches_claim_temporal_flat_next_bridge(expr)
}

fn matches_anchor_eleven_demo_sharp_codomain_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            )
    )
}

fn matches_anchor_eleven_demo_sharp_bridge_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Sharp(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Sharp(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            )
    )
}

fn matches_anchor_eleven_demo_sharp_domain_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Sharp(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    )
}

fn matches_anchor_eleven_demo_flat_codomain_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            )
    )
}

fn matches_reference_temporal_flat_next_bridge(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if (
                matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
                || matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner)
                                if matches!(
                                    inner.as_ref(),
                                    Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                        )
                )
            ) && (
                matches!(
                    codomain.as_ref(),
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
                || matches!(
                    codomain.as_ref(),
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Flat(inner)
                                if matches!(
                                    inner.as_ref(),
                                    Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                        )
                )
            )
    )
}

fn matches_claim_temporal_flat_next_bridge(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Eventually(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Next(deeper)
                                    if matches!(
                                        deeper.as_ref(),
                                        Expr::Eventually(deeper_inner)
                                            if matches!(deeper_inner.as_ref(), Expr::Var(1))
                                    )
                            )
                    )
            )
    )
}

fn matches_temporal_sharp_eventually_bridge(expr: &Expr) -> bool {
    matches_reference_temporal_sharp_eventually_bridge(expr)
        || matches_claim_temporal_sharp_eventually_bridge(expr)
}

fn matches_reference_temporal_sharp_eventually_bridge(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner)
                            if matches!(inner.as_ref(), Expr::Var(1))
                                || matches!(
                                    inner.as_ref(),
                                    Expr::Eventually(deeper)
                                        if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                    )
            )
    )
}

fn matches_claim_temporal_sharp_eventually_bridge(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    )
}

fn matches_reference_temporal_terminal_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
            )
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ConnectivitySummary, ConnectivityTerminalDecision, ConnectivityWitness,
        HistoricalReanchorProgress, HistoricalReanchorSummary, TerminalClauseConnectivityFacts,
        analyze_connectivity, passes_connectivity,
    };
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;

    fn library_until(step: u32) -> Library {
        let mut library = Vec::new();
        for current in 1..=step {
            let telescope = Telescope::reference(current);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }
        library
    }

    #[test]
    fn connectivity_accepts_reference_witness_step() {
        let library = library_until(2);
        let witness = analyze_connectivity(&library, &Telescope::reference(3));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 2,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(3)));
    }

    #[test]
    fn incremental_summary_matches_reference_temporal_shell_connectivity() {
        let library = library_until(14);
        let telescope = Telescope::reference(15);
        let witness = analyze_connectivity(&library, &telescope);
        let summary = ConnectivitySummary::from_telescope(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);

        assert!(summary.structurally_connected());
        assert!(!summary.references_active_window());
        assert!(!summary.self_contained());
        assert!(summary.needs_reanchor_fallback());
        assert!(reanchor.allows_historical_reanchor());
        assert_eq!(summary.max_lib_ref(), witness.max_lib_ref);
        assert!(witness.historical_reanchor);
    }

    #[test]
    fn terminal_decision_matches_incremental_extension_for_keep_prune_and_reanchor_cases() {
        let step_four_library = library_until(3);
        let step_four_prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let step_four_summary =
            ConnectivitySummary::from_telescope(&step_four_library, &step_four_prefix);
        let step_four_last_clause = Telescope::reference(4)
            .clauses
            .last()
            .cloned()
            .expect("reference step four should have a last clause");

        let step_four_extended = step_four_summary
            .clone()
            .extend(&step_four_library, &step_four_last_clause);
        assert_eq!(
            step_four_summary.terminal_decision(&step_four_library, &step_four_last_clause, false),
            ConnectivityTerminalDecision::KeepWithoutFallback
        );
        assert!(step_four_extended.structurally_connected());
        assert!(step_four_extended.passes_without_reanchor());

        let disconnected_library: Library = Vec::new();
        let disconnected_prefix = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::PathCon(1)),
        ]);
        let disconnected_summary =
            ConnectivitySummary::from_telescope(&disconnected_library, &disconnected_prefix);
        let disconnected_clause = ClauseRec::new(ClauseRole::Formation, Expr::Univ);
        let disconnected_extended = disconnected_summary
            .clone()
            .extend(&disconnected_library, &disconnected_clause);
        assert_eq!(
            disconnected_summary.terminal_decision(
                &disconnected_library,
                &disconnected_clause,
                false
            ),
            ConnectivityTerminalDecision::PruneDisconnected
        );
        assert!(!disconnected_extended.structurally_connected());

        let step_fifteen_library = library_until(14);
        let step_fifteen_prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let step_fifteen_summary =
            ConnectivitySummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_reanchor =
            HistoricalReanchorSummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_last_clause = Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference step fifteen should have a last clause");

        let step_fifteen_extended = step_fifteen_summary
            .clone()
            .extend(&step_fifteen_library, &step_fifteen_last_clause);
        let step_fifteen_reanchor_allowed = step_fifteen_reanchor
            .extend(&step_fifteen_last_clause)
            .allows_historical_reanchor();
        assert_eq!(step_fifteen_reanchor.matched_clause_count(), 7);
        assert_eq!(step_fifteen_reanchor.first_mismatch_position(), None);
        assert!(step_fifteen_extended.structurally_connected());
        assert!(!step_fifteen_extended.passes_without_reanchor());
        assert_eq!(
            step_fifteen_summary.terminal_decision(
                &step_fifteen_library,
                &step_fifteen_last_clause,
                false
            ),
            ConnectivityTerminalDecision::NeedsFallback
        );
        assert_eq!(
            step_fifteen_summary.terminal_decision(
                &step_fifteen_library,
                &step_fifteen_last_clause,
                step_fifteen_reanchor_allowed
            ),
            ConnectivityTerminalDecision::KeepWithoutFallback
        );
    }

    #[test]
    fn terminal_clause_connectivity_facts_match_direct_terminal_decision() {
        let step_four_library = library_until(3);
        let step_four_prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let step_four_summary =
            ConnectivitySummary::from_telescope(&step_four_library, &step_four_prefix);
        let step_four_last_clause = Telescope::reference(4)
            .clauses
            .last()
            .cloned()
            .expect("reference step four should have a last clause");
        let step_four_facts = TerminalClauseConnectivityFacts::from_clause(&step_four_last_clause);
        assert_eq!(
            step_four_summary.terminal_decision(&step_four_library, &step_four_last_clause, false),
            step_four_summary.terminal_decision_with_facts(
                &step_four_library,
                &step_four_facts,
                false,
            )
        );

        let disconnected_library: Library = Vec::new();
        let disconnected_prefix = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::PathCon(1)),
        ]);
        let disconnected_summary =
            ConnectivitySummary::from_telescope(&disconnected_library, &disconnected_prefix);
        let disconnected_clause = ClauseRec::new(ClauseRole::Formation, Expr::Univ);
        let disconnected_facts = TerminalClauseConnectivityFacts::from_clause(&disconnected_clause);
        assert_eq!(
            disconnected_summary.terminal_decision(
                &disconnected_library,
                &disconnected_clause,
                false,
            ),
            disconnected_summary.terminal_decision_with_facts(
                &disconnected_library,
                &disconnected_facts,
                false,
            )
        );

        let step_fifteen_library = library_until(14);
        let step_fifteen_prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let step_fifteen_summary =
            ConnectivitySummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_reanchor =
            HistoricalReanchorSummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_last_clause = Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference step fifteen should have a last clause");
        let step_fifteen_facts =
            TerminalClauseConnectivityFacts::from_clause(&step_fifteen_last_clause);
        let step_fifteen_reanchor_allowed = step_fifteen_reanchor
            .extend(&step_fifteen_last_clause)
            .allows_historical_reanchor();
        assert_eq!(step_fifteen_reanchor.matched_clause_count(), 7);
        assert_eq!(step_fifteen_reanchor.first_mismatch_position(), None);
        assert_eq!(
            step_fifteen_summary.terminal_decision(
                &step_fifteen_library,
                &step_fifteen_last_clause,
                step_fifteen_reanchor_allowed,
            ),
            step_fifteen_summary.terminal_decision_with_facts(
                &step_fifteen_library,
                &step_fifteen_facts,
                step_fifteen_reanchor_allowed,
            )
        );
    }

    #[test]
    fn point_constructor_can_close_an_earlier_path_attachment_gap_incrementally() {
        let telescope = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::PathCon(1)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let library: Library = Vec::new();
        let summary = ConnectivitySummary::from_telescope(&library, &telescope);
        let witness = analyze_connectivity(&library, &telescope);

        assert!(summary.structurally_connected());
        assert_eq!(summary.structurally_connected(), witness.connected);
        assert_eq!(
            summary.references_active_window(),
            witness.references_active_window
        );
        assert_eq!(summary.self_contained(), witness.self_contained);
        assert_eq!(summary.max_lib_ref(), witness.max_lib_ref);
    }

    #[test]
    fn connectivity_accepts_self_contained_former_packages() {
        let library = library_until(3);
        let witness = analyze_connectivity(&library, &Telescope::reference(4));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(4)));
    }

    #[test]
    fn connectivity_accepts_local_path_attachment_packages() {
        let library = library_until(4);
        let witness = analyze_connectivity(&library, &Telescope::reference(5));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(5)));
    }

    #[test]
    fn connectivity_accepts_self_contained_truncation_packages() {
        let library = library_until(5);
        let witness = analyze_connectivity(&library, &Telescope::reference(6));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(6)));
    }

    #[test]
    fn connectivity_accepts_self_contained_higher_hit_packages() {
        let library = library_until(6);
        let witness = analyze_connectivity(&library, &Telescope::reference(7));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(7)));
    }

    #[test]
    fn connectivity_accepts_self_contained_sphere_lift_packages() {
        let library = library_until(7);
        let witness = analyze_connectivity(&library, &Telescope::reference(8));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(8)));
    }

    #[test]
    fn connectivity_accepts_active_window_axiomatic_bundles() {
        let library = library_until(8);
        let witness = analyze_connectivity(&library, &Telescope::reference(9));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 8,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(9)));
    }

    #[test]
    fn connectivity_accepts_self_contained_modal_shells() {
        let library = library_until(9);
        let witness = analyze_connectivity(&library, &Telescope::reference(10));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(10)));
    }

    #[test]
    fn connectivity_accepts_higher_order_bridge_shells_over_the_active_window() {
        let library = library_until(11);
        let witness = analyze_connectivity(&library, &Telescope::reference(12));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 11,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(12)));
    }

    #[test]
    fn connectivity_accepts_claim_structural_shell_seal_variants() {
        let library = library_until(10);

        for seal_expr in [
            Expr::Lam(Box::new(Expr::Var(1))),
            Expr::Lam(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ] {
            for closure_expr in [
                Expr::Pi(Box::new(Expr::Lib(10)), Box::new(Expr::Lib(10))),
                Expr::Pi(Box::new(Expr::Lib(9)), Box::new(Expr::Lib(10))),
                Expr::Pi(Box::new(Expr::Lib(10)), Box::new(Expr::Lib(9))),
            ] {
                let mut telescope = Telescope::reference(11);
                telescope.clauses[4].expr = seal_expr.clone();
                telescope
                    .clauses
                    .push(ClauseRec::new(ClauseRole::Formation, closure_expr));
                let witness = analyze_connectivity(&library, &telescope);
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: true,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_operator_bundle_bridges_over_curvature() {
        let library = library_until(12);
        let witness = analyze_connectivity(&library, &Telescope::reference(13));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 12,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(13)));
    }

    #[test]
    fn connectivity_accepts_claim_operator_bundle_seed_variants() {
        let library = library_until(12);

        for seed_expr in [
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ] {
            let mut telescope = Telescope::reference(13);
            telescope.clauses[0].expr = seed_expr;
            let witness = analyze_connectivity(&library, &telescope);
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: true,
                    self_contained: false,
                    max_lib_ref: 12,
                    historical_reanchor: false,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_claim_operator_bundle_action_variants() {
        let library = library_until(12);

        for action_expr in [
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(2)),
                Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ))),
        ] {
            let mut telescope = Telescope::reference(13);
            telescope.clauses[3].expr = action_expr;
            let witness = analyze_connectivity(&library, &telescope);
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: true,
                    self_contained: false,
                    max_lib_ref: 12,
                    historical_reanchor: false,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_rejects_stale_library_refs() {
        let library = library_until(4);
        assert!(!passes_connectivity(&library, &Telescope::reference(3)));
    }

    #[test]
    fn connectivity_accepts_temporal_shells_reanchored_through_the_modal_history() {
        let library = library_until(14);
        let witness = analyze_connectivity(&library, &Telescope::reference(15));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: true,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(15)));
    }

    #[test]
    fn connectivity_accepts_realistic_temporal_reanchor_variants() {
        let library = library_until(14);
        let mut telescope = Telescope::reference(15);
        telescope.clauses[4].expr = Expr::Pi(
            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                Box::new(Expr::Var(1)),
            )))))),
        );

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: true,
            }
        );
        assert!(reanchor.allows_historical_reanchor());
        assert!(passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_accepts_demo_temporal_exchange_variants() {
        let library = library_until(14);
        let mut telescope = Telescope::reference(15);
        telescope.clauses[4].expr = Expr::Pi(
            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Next(
                Box::new(Expr::Var(1)),
            )))))),
            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                Box::new(Expr::Var(1)),
            )))))),
        );

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: true,
            }
        );
        assert!(reanchor.allows_historical_reanchor());
        assert!(passes_connectivity(&library, &telescope));
    }

    fn claim_temporal_variant_exprs(position: usize, anchor: u32) -> Vec<Expr> {
        match position {
            0 => vec![
                Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
            ],
            1 => vec![
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ],
            2 => vec![
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                ),
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            ],
            3 => vec![
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor)),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                ))),
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor)),
                    Box::new(Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(
                        1,
                    )))))),
                ))),
            ],
            4 => vec![
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(
                        Expr::Eventually(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                ),
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Next(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                        Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                    )))))),
                ),
            ],
            5 => vec![
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Next(Box::new(Expr::Var(1))),
                    ))))),
                ),
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            ],
            6 => vec![
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Var(2)),
                ))),
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(
                        1,
                    )))))),
                    Box::new(Expr::Var(2)),
                ))),
            ],
            _ => Vec::new(),
        }
    }

    fn reference_temporal_terminal_clause() -> ClauseRec {
        Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference step fifteen should have a terminal clause")
    }

    fn reference_temporal_clause_six() -> Expr {
        Telescope::reference(15)
            .clauses
            .get(6)
            .expect("reference step fifteen should expose clause six")
            .expr
            .clone()
    }

    fn representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
        anchor: u32,
        clause_five:
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel,
    ) -> Expr {
        match clause_five {
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain => {
                claim_temporal_variant_exprs(5, anchor)
                    .into_iter()
                    .nth(1)
                    .expect("representative mismatch-zero parent route should expose a claim-flat-codomain clause")
            }
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference => {
                Telescope::reference(15)
                    .clauses
                    .get(5)
                    .expect("reference step fifteen should expose clause five")
                    .expr
                    .clone()
            }
        }
    }

    fn representative_mismatch_zero_claim_side_parent_route_clause_three_expr(
        anchor: u32,
        clause_three:
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel,
    ) -> Expr {
        match clause_three {
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument => {
                claim_temporal_variant_exprs(3, anchor)
                    .into_iter()
                    .next()
                    .expect("representative mismatch-zero parent route should expose a claim-flat argument clause")
            }
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument => {
                claim_temporal_variant_exprs(3, anchor)
                    .into_iter()
                    .nth(1)
                    .expect("representative mismatch-zero parent route should expose a claim-eventual argument clause")
            }
        }
    }

    fn representative_mismatch_zero_claim_side_parent_route_clause_four_expr(
        anchor: u32,
        clause_four:
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel,
    ) -> Expr {
        match clause_four {
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::ClaimNextBridge => {
                claim_temporal_variant_exprs(4, anchor)
                    .into_iter()
                    .next()
                    .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause-four variant")
            }
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference => Telescope::reference(15)
                .clauses
                .get(4)
                .expect("reference step fifteen should expose clause four")
                .expr
                .clone(),
        }
    }

    fn representative_mismatch_zero_claim_side_parent_route_clause_six_expr(
        anchor: u32,
        clause_six:
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel,
    ) -> Expr {
        match clause_six {
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::ClaimNextCodomain => {
                claim_temporal_variant_exprs(6, anchor)
                    .into_iter()
                    .nth(1)
                    .expect("representative mismatch-zero parent route should expose a claim-next-codomain clause-six variant")
            }
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::ClaimSharpCodomain => {
                claim_temporal_variant_exprs(6, anchor)
                    .into_iter()
                    .next()
                    .expect("representative mismatch-zero parent route should expose a claim-sharp-codomain clause-six variant")
            }
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference => {
                reference_temporal_clause_six()
            }
        }
    }

    fn next_lift_temporal_terminal_clause() -> ClauseRec {
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            ),
        )
    }

    fn claim_step_fifteen_clause_five_claim_flat_codomain_clause() -> ClauseRec {
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Flat(Box::new(Expr::Var(1))),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
            ),
        )
    }

    fn claim_step_fifteen_clause_five_claim_next_codomain_clause() -> ClauseRec {
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Var(1),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Next(Box::new(Expr::Var(1))),
                ))))),
            ),
        )
    }

    #[test]
    fn connectivity_accepts_claim_clause_zero_one_variants_when_the_terminal_stays_reference() {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 0..=1 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should stay historically reanchorable when the terminal clause remains exact"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_zero_one_variants_outside_historical_reanchor_when_the_terminal_lifts()
     {
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 0..=1 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should not reanchor through the lifted terminal clause"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_two_three_variants_fenced_even_with_reference_terminal() {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 2..=3 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should stay fenced even with the exact reference terminal"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_claim_clause_four_five_variants_when_the_terminal_stays_reference() {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 4..=5 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should now stay historically reanchorable when the terminal clause remains exact"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_four_five_variants_outside_historical_reanchor_when_the_terminal_lifts()
     {
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 4..=5 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should not reanchor through a lifted terminal clause"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_claim_clause_two_anchor_eleven_exact_argument_pocket_across_repaired_side_variants()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let side_positions = [(0usize, 0usize), (1, 1), (4, 4), (5, 5)];

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for mask in 0u8..(1 << side_positions.len()) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                for (bit_index, position) in side_positions {
                    if mask & (1 << bit_index) != 0 {
                        telescope.clauses[position].expr =
                            claim_temporal_variant_exprs(position, anchor)
                                .into_iter()
                                .next()
                                .expect("repaired side position should expose a claim variant");
                    }
                }
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "claim clause-2 plus anchor-11 exact-argument pocket should stay historically reanchorable across every repaired-side subset while clause 6 and the terminal stay exact"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-1 demo-flat-codomain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the clause-1 demo-flat-codomain opening should stay fenced until the exact anchor-11 side pocket is also present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_only_on_the_exact_anchor_eleven_side_pocket_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_flat_side_pocket();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-1 demo-flat-codomain opening should become historically reanchorable on top of clause-0 claim-flat only on the exact anchor-11 side pocket under the scoped negative-control override"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_flat_side_pocket();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the clause-1 demo-flat-codomain opening should stay fenced on top of clause-0 claim-flat until the exact anchor-11 side pocket is also present even under the scoped negative-control override"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the clause-1 demo-flat-codomain opening should become historically reanchorable on the reference-clause-0 live claim bridge surface under the scoped override: clause4={:?} clause5={:?} witness={witness:?} reanchor={reanchor:?}",
                        clause_four_variant,
                        clause_five_variant,
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_four_variant in claim_temporal_variant_exprs(4, anchor) {
                for clause_five_variant in claim_temporal_variant_exprs(5, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the live-claim bridge override should still keep clause-1 demo-flat-codomain fenced until the exact anchor-11 side pocket is also present"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_from_reopening_lifted_terminals_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_four_variant in claim_temporal_variant_exprs(4, anchor) {
                for clause_five_variant in claim_temporal_variant_exprs(5, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the live-claim bridge override should remain reference-terminal-only and keep lifted terminals outside historical reanchor"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_surface_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_surface();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_four_variant in clause_four_variants.iter() {
                    for clause_five_variant in clause_five_variants.iter() {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                        );
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3] = ClauseRec::new(
                            ClauseRole::Introduction,
                            Expr::Lam(Box::new(Expr::App(
                                Box::new(Expr::Lib(anchor + 1)),
                                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                            ))),
                        );
                        telescope.clauses[4].expr = clause_four_variant.clone();
                        telescope.clauses[5].expr = clause_five_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            reanchor.allows_historical_reanchor(),
                            "the clause-1 demo-flat-codomain opening should become historically reanchorable on the exact mismatch-0 claim-domain surface under the scoped override: clause0={:?} clause4={:?} clause5={:?} witness={witness:?} reanchor={reanchor:?}",
                            clause_zero_variant,
                            clause_four_variant,
                            clause_five_variant,
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 11,
                                historical_reanchor: true,
                            }
                        );
                        assert!(passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_surface_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_surface();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_four_variant in claim_temporal_variant_exprs(4, anchor) {
                    for clause_five_variant in claim_temporal_variant_exprs(5, anchor) {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                        );
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[4].expr = clause_four_variant.clone();
                        telescope.clauses[5].expr = clause_five_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            !reanchor.allows_historical_reanchor(),
                            "the mismatch-0 claim-domain override should still keep clause-1 demo-flat-codomain fenced until the exact anchor-11 side pocket is also present"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 10,
                                historical_reanchor: false,
                            }
                        );
                        assert!(!passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_surface_from_reopening_lifted_terminals_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_surface();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_four_variant in clause_four_variants.iter() {
                    for clause_five_variant in clause_five_variants.iter() {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                        );
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3] = ClauseRec::new(
                            ClauseRole::Introduction,
                            Expr::Lam(Box::new(Expr::App(
                                Box::new(Expr::Lib(anchor + 1)),
                                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                            ))),
                        );
                        telescope.clauses[4].expr = clause_four_variant.clone();
                        telescope.clauses[5].expr = clause_five_variant.clone();
                        telescope.clauses[7] = lifted_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            !reanchor.allows_historical_reanchor(),
                            "the mismatch-0 claim-domain override should remain reference-terminal-only and keep lifted terminals outside historical reanchor"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 11,
                                historical_reanchor: false,
                            }
                        );
                        assert!(!passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_four_variant in clause_four_variants.iter() {
                    for clause_five_variant in clause_five_variants.iter() {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                        );
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3] = ClauseRec::new(
                            ClauseRole::Introduction,
                            Expr::Lam(Box::new(Expr::App(
                                Box::new(Expr::Lib(anchor + 1)),
                                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                            ))),
                        );
                        telescope.clauses[4].expr = clause_four_variant.clone();
                        telescope.clauses[5].expr = clause_five_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            reanchor.allows_historical_reanchor(),
                            "the mismatch-0 claim-domain plus clause-4 claim-next-bridge-side override should admit only the live claim-bridge cells on the two claim clause-two sheets: clause0={:?} clause2={:?} clause4={:?} clause5={:?}",
                            telescope.clauses[0].expr,
                            telescope.clauses[2].expr,
                            clause_four_variant,
                            clause_five_variant,
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 11,
                                historical_reanchor: true,
                            }
                        );
                        assert!(passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_four_variant in clause_four_variants.iter() {
                    for clause_five_variant in clause_five_variants.iter() {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                        );
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[4].expr = clause_four_variant.clone();
                        telescope.clauses[5].expr = clause_five_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            !reanchor.allows_historical_reanchor(),
                            "the mismatch-0 claim-domain plus clause-4 claim-next-bridge-side override should still require the exact anchor-11 side pocket"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 10,
                                historical_reanchor: false,
                            }
                        );
                        assert!(!passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_reference_side_even_under_claim_next_bridge_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variant = Telescope::reference(15).clauses[4].expr.clone();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the mismatch-0 claim-domain plus clause-4 claim-next-bridge-side override should keep the actual clause-4 reference side closed"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_four_variant in clause_four_variants.iter() {
                    for clause_five_variant in clause_five_variants.iter() {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                        );
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3] = ClauseRec::new(
                            ClauseRole::Introduction,
                            Expr::Lam(Box::new(Expr::App(
                                Box::new(Expr::Lib(anchor + 1)),
                                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                            ))),
                        );
                        telescope.clauses[4].expr = clause_four_variant.clone();
                        telescope.clauses[5].expr = clause_five_variant.clone();
                        telescope.clauses[7] = lifted_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            !reanchor.allows_historical_reanchor(),
                            "the mismatch-0 claim-domain plus clause-4 claim-next-bridge-side override should remain reference-terminal-only and keep lifted terminals outside historical reanchor"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 11,
                                historical_reanchor: false,
                            }
                        );
                        assert!(!passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_reference_side_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_reference_side();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variant = Telescope::reference(15).clauses[4].expr.clone();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the mismatch-0 claim-domain plus clause-4 reference-side override should admit only the live reference-side cells on the two claim clause-two sheets: clause0={:?} clause2={:?} clause5={:?}",
                        telescope.clauses[0].expr,
                        telescope.clauses[2].expr,
                        clause_five_variant,
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_reference_side_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_reference_side();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variant = Telescope::reference(15).clauses[4].expr.clone();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the mismatch-0 claim-domain plus clause-4 reference-side override should still require the exact anchor-11 side pocket"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_claim_next_bridge_side_even_under_reference_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_reference_side();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_four_variant in clause_four_variants.iter() {
                    for clause_five_variant in clause_five_variants.iter() {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                        );
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3] = ClauseRec::new(
                            ClauseRole::Introduction,
                            Expr::Lam(Box::new(Expr::App(
                                Box::new(Expr::Lib(anchor + 1)),
                                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                            ))),
                        );
                        telescope.clauses[4].expr = clause_four_variant.clone();
                        telescope.clauses[5].expr = clause_five_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            !reanchor.allows_historical_reanchor(),
                            "the mismatch-0 claim-domain plus clause-4 reference-side override should keep the actual clause-4 claim-next-bridge side closed"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 11,
                                historical_reanchor: false,
                            }
                        );
                        assert!(!passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_claim_domain_mismatch_zero_clause_four_reference_side_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_clause_zero_claim_domain_mismatch_zero_clause_four_reference_side();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_four_variant = Telescope::reference(15).clauses[4].expr.clone();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_zero_variant in clause_zero_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the mismatch-0 claim-domain plus clause-4 reference-side override should remain reference-terminal-only and keep lifted terminals outside historical reanchor"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the clause-4-reference-sheet override should admit the full clause-5 family only while clause 4 stays on the exact reference sheet: clause5={:?}",
                    clause_five_variant,
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_clause_four_reference_sheet_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_four_variant in claim_temporal_variant_exprs(4, anchor) {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the clause-4-reference-sheet override should stay fenced once clause 4 leaves the reference family"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the clause-4-reference-sheet override should keep lifted terminals outside historical reanchor"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .next()
            .expect("claim-flat clause-two variant should exist");
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the exact claim-flat clause-two sheet override should admit the full live clause-4/clause-5 bridge family only on that single sheet: clause4={:?} clause5={:?}",
                    clause_four_variant,
                    clause_five_variant,
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .next()
            .expect("claim-flat clause-two variant should exist");
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-flat clause-two sheet override should still require the exact anchor-11 side pocket"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_flat_clause_two_sheet_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = [
            Telescope::reference(15).clauses[2].expr.clone(),
            claim_temporal_variant_exprs(2, anchor)
                .into_iter()
                .nth(1)
                .expect("claim-sharp clause-two variant should exist"),
        ];
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-flat clause-two sheet override should stay fenced on the other clause-two sheets"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .next()
            .expect("claim-flat clause-two variant should exist");
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-flat clause-two sheet override should remain reference-terminal-only and keep lifted terminals fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .nth(1)
            .expect("claim-sharp clause-two variant should exist");
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the exact claim-sharp clause-two sheet override should admit the full live clause-4/clause-5 bridge family only on that single sheet: clause4={:?} clause5={:?}",
                    clause_four_variant,
                    clause_five_variant,
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .nth(1)
            .expect("claim-sharp clause-two variant should exist");
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-sharp clause-two sheet override should still require the exact anchor-11 side pocket"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_sharp_clause_two_sheet_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = [
            Telescope::reference(15).clauses[2].expr.clone(),
            claim_temporal_variant_exprs(2, anchor)
                .into_iter()
                .next()
                .expect("claim-flat clause-two variant should exist"),
        ];
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-sharp clause-two sheet override should stay fenced on the other clause-two sheets"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .nth(1)
            .expect("claim-sharp clause-two variant should exist");
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-sharp clause-two sheet override should remain reference-terminal-only and keep lifted terminals fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the exact claim-variant clause-two pair override should admit the full live clause-4/clause-5 bridge family on both claim sheets only: clause2={:?} clause4={:?} clause5={:?}",
                        telescope.clauses[2].expr,
                        clause_four_variant,
                        clause_five_variant,
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-variant clause-two pair override should still require the exact anchor-11 side pocket"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = Telescope::reference(15).clauses[2].expr.clone();
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-variant clause-two pair override should stay fenced on the reference clause-two sheet"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let mut clause_four_variants = vec![Telescope::reference(15).clauses[4].expr.clone()];
        clause_four_variants.extend(claim_temporal_variant_exprs(4, anchor));
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(claim_temporal_variant_exprs(5, anchor));

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-variant clause-two pair override should remain reference-terminal-only and keep lifted terminals fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the exact claim-pair plus clause-4 claim-next-bridge-side override should admit the live claim bridge cells on both claim clause-two sheets only: clause2={:?} clause4={:?} clause5={:?}",
                        telescope.clauses[2].expr,
                        clause_four_variant,
                        clause_five_variant,
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-pair plus clause-4 claim-next-bridge-side override should still require the exact anchor-11 side pocket"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_claim_next_bridge_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = Telescope::reference(15).clauses[2].expr.clone();
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-pair plus clause-4 claim-next-bridge-side override should stay fenced on the reference clause-two sheet"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_even_under_claim_next_bridge_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let clause_four_variant = Telescope::reference(15).clauses[4].expr.clone();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-pair plus clause-4 claim-next-bridge-side override should keep the actual clause-4 reference sheet closed"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-pair plus clause-4 claim-next-bridge-side override should remain reference-terminal-only and keep lifted terminals fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_flat_sheet_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .next()
            .expect("claim-flat clause-two variant should exist");
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the exact claim-flat plus clause-4 claim-next-bridge-side override should admit the live claim bridge cells only on the claim-flat clause-two sheet: clause4={:?} clause5={:?}",
                    clause_four_variant,
                    clause_five_variant,
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_flat_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .next()
            .expect("claim-flat clause-two variant should exist");
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-flat plus clause-4 claim-next-bridge-side override should still require the exact anchor-11 side pocket"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_flat_clause_two_sheet_even_under_claim_next_bridge_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = [
            Telescope::reference(15).clauses[2].expr.clone(),
            claim_temporal_variant_exprs(2, anchor)
                .into_iter()
                .nth(1)
                .expect("claim-sharp clause-two variant should exist"),
        ];
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-flat plus clause-4 claim-next-bridge-side override should stay fenced on the reference and claim-sharp clause-two sheets"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_even_under_claim_next_bridge_side_on_exact_claim_flat_sheet_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .next()
            .expect("claim-flat clause-two variant should exist");
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_five_variant in clause_five_variants.iter() {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5].expr = clause_five_variant.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-flat plus clause-4 claim-next-bridge-side override should keep the actual clause-4 reference sheet closed"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_flat_sheet_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_flat_sheet();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .next()
            .expect("claim-flat clause-two variant should exist");
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-flat plus clause-4 claim-next-bridge-side override should remain reference-terminal-only and keep lifted terminals fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_sharp_sheet_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .nth(1)
            .expect("claim-sharp clause-two variant should exist");
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the exact claim-sharp plus clause-4 claim-next-bridge-side override should admit the live claim bridge cells only on the claim-sharp clause-two sheet: clause4={:?} clause5={:?}",
                    clause_four_variant,
                    clause_five_variant,
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_sharp_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .nth(1)
            .expect("claim-sharp clause-two variant should exist");
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-sharp plus clause-4 claim-next-bridge-side override should still require the exact anchor-11 side pocket"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_sharp_clause_two_sheet_even_under_claim_next_bridge_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = [
            Telescope::reference(15).clauses[2].expr.clone(),
            claim_temporal_variant_exprs(2, anchor)
                .into_iter()
                .next()
                .expect("claim-flat clause-two variant should exist"),
        ];
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-sharp plus clause-4 claim-next-bridge-side override should stay fenced on the reference and claim-flat clause-two sheets"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_even_under_claim_next_bridge_side_on_exact_claim_sharp_sheet_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .nth(1)
            .expect("claim-sharp clause-two variant should exist");
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_five_variant in clause_five_variants.iter() {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5].expr = clause_five_variant.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-sharp plus clause-4 claim-next-bridge-side override should keep the actual clause-4 reference sheet closed"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_sharp_sheet_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_clause_two_claim_sharp_sheet();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variant = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .nth(1)
            .expect("claim-sharp clause-two variant should exist");
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_four_variant in clause_four_variants.iter() {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = clause_four_variant.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-sharp plus clause-4 claim-next-bridge-side override should remain reference-terminal-only and keep lifted terminals fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_variant_pair_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = reference_clause_four.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the exact claim-pair plus clause-4 reference-side override should admit the clause-4 reference family on both claim clause-two sheets only: clause2={:?} clause5={:?}",
                    telescope.clauses[2].expr,
                    clause_five_variant,
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[4].expr = reference_clause_four.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-pair plus clause-4 reference-side override should still require the exact anchor-11 side pocket"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_clause_four_reference_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let clause_two_variant = Telescope::reference(15).clauses[2].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_five_variant in clause_five_variants.iter() {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5].expr = clause_five_variant.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-pair plus clause-4 reference-side override should stay fenced on the reference clause-two sheet"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_closed_even_under_clause_four_reference_side_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let clause_four_variants = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .filter(|expr| super::matches_claim_temporal_flat_next_bridge(expr))
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_four_variant in clause_four_variants.iter() {
                for clause_five_variant in clause_five_variants.iter() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    );
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4].expr = clause_four_variant.clone();
                    telescope.clauses[5].expr = clause_five_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact claim-pair plus clause-4 reference-side override should keep the live clause-4 claim-next-bridge side closed"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_variant_pair_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let mut clause_five_variants = vec![Telescope::reference(15).clauses[5].expr.clone()];
        clause_five_variants.extend(
            claim_temporal_variant_exprs(5, anchor)
                .into_iter()
                .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr)),
        );

        for clause_two_variant in clause_two_variants {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = reference_clause_four.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-pair plus clause-4 reference-side override should remain reference-terminal-only and keep lifted terminals fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let reference_clause_five = Telescope::reference(15).clauses[5].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5].expr = reference_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the exact claim-pair clause-four-reference-tail override should admit the reference clause-five cell on both claim clause-two sheets only: clause2={:?}",
                telescope.clauses[2].expr,
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let reference_clause_five = Telescope::reference(15).clauses[5].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5].expr = reference_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-pair clause-four-reference-tail override should still require the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_clause_five_reference_on_clause_four_reference_tail_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let reference_clause_five = Telescope::reference(15).clauses[5].expr.clone();
        let clause_two_variant = Telescope::reference(15).clauses[2].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut telescope = Telescope::reference(15);
        telescope.clauses[1] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        );
        telescope.clauses[2].expr = clause_two_variant;
        telescope.clauses[3] = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor + 1)),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            ))),
        );
        telescope.clauses[4].expr = reference_clause_four;
        telescope.clauses[5].expr = reference_clause_five;
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            !reanchor.allows_historical_reanchor(),
            "the exact claim-pair clause-four-reference-tail override should stay fenced on the reference clause-two sheet"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 11,
                historical_reanchor: false,
            }
        );
        assert!(!passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_families_closed_on_clause_four_reference_tail_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();
        let clause_five_variants = claim_temporal_variant_exprs(5, anchor)
            .into_iter()
            .filter(|expr| super::matches_temporal_sharp_eventually_bridge(expr))
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            for clause_five_variant in clause_five_variants.iter() {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                );
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4].expr = reference_clause_four.clone();
                telescope.clauses[5].expr = clause_five_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-pair clause-four-reference-tail override should keep the non-reference clause-five families fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let reference_clause_five = Telescope::reference(15).clauses[5].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5].expr = reference_clause_five.clone();
            telescope.clauses[7] = lifted_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-pair clause-four-reference-tail override should remain reference-terminal-only and keep lifted terminals fenced"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_flat_clause_five = claim_step_fifteen_clause_five_claim_flat_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_flat_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the exact claim-flat clause-five tail override should admit that single claim family on both claim clause-two sheets only: clause2={:?}",
                telescope.clauses[2].expr,
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_flat_clause_five = claim_step_fifteen_clause_five_claim_flat_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_flat_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-flat clause-five tail override should still require the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_clause_five_claim_flat_codomain_on_clause_four_reference_tail_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_flat_clause_five = claim_step_fifteen_clause_five_claim_flat_codomain_clause();
        let clause_two_variant = Telescope::reference(15).clauses[2].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut telescope = Telescope::reference(15);
        telescope.clauses[1] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        );
        telescope.clauses[2].expr = clause_two_variant;
        telescope.clauses[3] = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor + 1)),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            ))),
        );
        telescope.clauses[4].expr = reference_clause_four;
        telescope.clauses[5] = claim_flat_clause_five;
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            !reanchor.allows_historical_reanchor(),
            "the exact claim-flat clause-five tail override should stay fenced on the reference clause-two sheet"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 11,
                historical_reanchor: false,
            }
        );
        assert!(!passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_closed_on_clause_four_reference_tail_even_under_claim_flat_codomain_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_next_clause_five = claim_step_fifteen_clause_five_claim_next_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_next_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-flat clause-five tail override should keep the sibling claim-next clause-five family fenced"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_flat_clause_five = claim_step_fifteen_clause_five_claim_flat_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_flat_clause_five.clone();
            telescope.clauses[7] = lifted_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-flat clause-five tail override should remain reference-terminal-only and keep lifted terminals fenced"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_next_clause_five = claim_step_fifteen_clause_five_claim_next_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_next_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the exact claim-next clause-five tail override should admit that single claim family on both claim clause-two sheets only: clause2={:?}",
                telescope.clauses[2].expr,
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_next_clause_five = claim_step_fifteen_clause_five_claim_next_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_next_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-next clause-five tail override should still require the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_clause_five_claim_next_codomain_on_clause_four_reference_tail_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_next_clause_five = claim_step_fifteen_clause_five_claim_next_codomain_clause();
        let clause_two_variant = Telescope::reference(15).clauses[2].expr.clone();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let mut telescope = Telescope::reference(15);
        telescope.clauses[1] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        );
        telescope.clauses[2].expr = clause_two_variant;
        telescope.clauses[3] = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor + 1)),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            ))),
        );
        telescope.clauses[4].expr = reference_clause_four;
        telescope.clauses[5] = claim_next_clause_five;
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            !reanchor.allows_historical_reanchor(),
            "the exact claim-next clause-five tail override should stay fenced on the reference clause-two sheet"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 11,
                historical_reanchor: false,
            }
        );
        assert!(!passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_closed_on_clause_four_reference_tail_even_under_claim_next_codomain_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_flat_clause_five = claim_step_fifteen_clause_five_claim_flat_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_flat_clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-next clause-five tail override should keep the sibling claim-flat clause-five family fenced"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_clause_two_claim_variant_pair();
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let reference_clause_four = Telescope::reference(15).clauses[4].expr.clone();
        let claim_next_clause_five = claim_step_fifteen_clause_five_claim_next_codomain_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_two_variants = claim_temporal_variant_exprs(2, anchor)
            .into_iter()
            .take(2)
            .collect::<Vec<_>>();

        for clause_two_variant in clause_two_variants {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant.clone();
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4].expr = reference_clause_four.clone();
            telescope.clauses[5] = claim_next_clause_five.clone();
            telescope.clauses[7] = lifted_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the exact claim-next clause-five tail override should remain reference-terminal-only and keep lifted terminals fenced"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_eventually_codomain_only_on_the_exact_anchor_eleven_side_pocket_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_eventually_codomain_side_pocket();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-1 demo-eventually-codomain opening should become historically reanchorable only on the exact anchor-11 side pocket under the scoped negative-control override"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_eventually_codomain_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_one_eventually_codomain_side_pocket();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the clause-1 demo-eventually-codomain opening should still stay fenced until the exact anchor-11 side pocket is present even under the scoped negative-control override"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_zero_demo_sharp_domain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-0 demo-sharp-domain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_zero_demo_next_domain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-0 demo-next-domain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_zero_demo_sharp_domain_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let mut telescope = Telescope::reference(15);
        telescope.clauses[0] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
        );
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            !reanchor.allows_historical_reanchor(),
            "the clause-0 demo-sharp-domain opening should stay fenced until the exact anchor-11 side pocket is also present"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: false,
            }
        );
        assert!(!passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_keeps_clause_zero_demo_next_domain_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let mut telescope = Telescope::reference(15);
        telescope.clauses[0] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
        );
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            !reanchor.allows_historical_reanchor(),
            "the clause-0 demo-next-domain opening should stay fenced until the exact anchor-11 side pocket is also present"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: false,
            }
        );
        assert!(!passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_codomain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the landed clause-4 demo-sharp-codomain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_bridge_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the new clause-4 demo-sharp-bridge opening should count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_codomain_on_claim_safe_clause_zero_one_surface_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_zero_variant in claim_temporal_variant_exprs(0, anchor) {
            for clause_one_variant in claim_temporal_variant_exprs(1, anchor) {
                for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1].expr = clause_one_variant.clone();
                    telescope.clauses[2].expr = clause_two_variant;
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                            Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                                Box::new(Expr::Var(1)),
                            )))))),
                        ),
                    );
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the scoped clause-4 demo-sharp-codomain override should make the claim-safe clause-0/1 surface historically reanchorable with the exact terminal"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_bridge_on_claim_safe_clause_zero_one_surface_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_zero_variant in claim_temporal_variant_exprs(0, anchor) {
            for clause_one_variant in claim_temporal_variant_exprs(1, anchor) {
                for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1].expr = clause_one_variant.clone();
                    telescope.clauses[2].expr = clause_two_variant;
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                        ),
                    );
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the scoped clause-4 demo-sharp-bridge override should make the claim-safe clause-0/1 surface historically reanchorable with the exact terminal"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_only_on_the_exact_anchor_eleven_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the next clause-5 demo-sharp-domain opening should count as historical reanchor only once the exact anchor-11 clause-4 side pocket is already present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_only_on_the_exact_anchor_eleven_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-flat-codomain opening should count as historical reanchor only once the exact anchor-11 clause-4 side pocket is already present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_on_the_anchor_eleven_clause_four_bridge_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-sharp-domain opening should now also count as historical reanchor once the exact anchor-11 clause-4 bridge pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_on_the_anchor_eleven_clause_four_bridge_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-flat-codomain opening should now also count as historical reanchor once the exact anchor-11 clause-4 bridge pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_on_the_exact_remaining_two_mismatch_zero_bridge_slice()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice(
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_one_variants = [
            Expr::Eventually(Box::new(Expr::Var(1))),
            Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
        ];

        for clause_zero_variant in clause_zero_variants {
            for clause_one_variant in clause_one_variants.iter() {
                for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1].expr = clause_one_variant.clone();
                    telescope.clauses[2].expr = clause_two_variant;
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                        ),
                    );
                    telescope.clauses[5] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                                Expr::Sharp(Box::new(Expr::Var(1))),
                            ))))),
                            Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                                Expr::Var(1),
                            ))))),
                        ),
                    );
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the clause-5 demo-sharp-domain opening should now count as historical reanchor on the exact remaining-two mismatch-0 bridge slice"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_on_the_exact_remaining_two_mismatch_zero_bridge_slice()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice(
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_one_variants = [
            Expr::Eventually(Box::new(Expr::Var(1))),
            Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
        ];

        for clause_zero_variant in clause_zero_variants {
            for clause_one_variant in clause_one_variants.iter() {
                for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1].expr = clause_one_variant.clone();
                    telescope.clauses[2].expr = clause_two_variant;
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                        ),
                    );
                    telescope.clauses[5] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                                Expr::Var(1),
                            ))))),
                            Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                                Expr::Flat(Box::new(Expr::Var(1))),
                            ))))),
                        ),
                    );
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the clause-5 demo-flat-codomain opening should now count as historical reanchor on the exact remaining-two mismatch-0 bridge slice"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_five_remaining_two_mismatch_one_bridge_slice_outside_historical_reanchor_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice(
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_one_variants = [
            Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ];
        let clause_five_variants = [
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            ),
        ];

        for clause_one_variant in clause_one_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_five_variant in clause_five_variants.iter().cloned() {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[1].expr = clause_one_variant.clone();
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[4] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                                Box::new(Expr::Var(1)),
                            )))))),
                        ),
                    );
                    telescope.clauses[5] = clause_five_variant;
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the exact remaining-two mismatch-0 override should still keep the mismatch-1 bridge slice outside historical reanchor"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: false,
                        }
                    );
                    assert!(
                        !passes_connectivity(&library, &telescope),
                        "the exact remaining-two mismatch-0 override should still fence mismatch-1 bridge prefixes out of the live claim path"
                    );
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_codomain_on_exact_claim_safe_pair_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selector, clause_one_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimSharpCodomain,
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair(
                    selector,
                );
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                        Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the exact claim-safe pair override should admit only the selected mismatch-1 pairing on the clause-4 demo-sharp-codomain side"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_four_demo_sharp_codomain_on_sibling_claim_safe_pair_closed_even_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selector, sibling_clause_one_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimSharpCodomain,
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair(
                    selector,
                );
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = sibling_clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                        Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-safe pair override should keep the sibling mismatch-1 pairing closed"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_four_demo_sharp_codomain_on_exact_claim_safe_pair_reference_terminal_only_even_under_override()
     {
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selector, clause_one_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimSharpCodomain,
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair(
                    selector,
                );
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                        Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-safe pair override should remain reference-terminal-only and keep lifted terminals fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_claim_clause_two_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selector, clause_two_selector, clause_two_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimFlatDomain,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                ),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimSharpCodomain,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two(
                    super::ClaimStepFifteenClaimSafePairClauseTwoSelector {
                        clause_one: selector,
                        clause_two: clause_two_selector,
                    },
                );
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the representative claim-safe claim-side clause-two override should admit the selected claim sheet on the exact pair"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_sibling_claim_clause_two_closed_even_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selected_clause_two, sibling_clause_two_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimFlatDomain,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimSharpCodomain,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                ),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two(
                    super::ClaimStepFifteenClaimSafePairClauseTwoSelector {
                        clause_one: super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                        clause_two: selected_clause_two,
                    },
                );
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[2].expr = sibling_clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the representative claim-safe clause-two override should keep the sibling claim sheet closed"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_reference_clause_two_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let _override =
            super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two(
                super::ClaimStepFifteenClaimSafePairClauseTwoSelector {
                    clause_one: super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                    clause_two: super::ClaimStepFifteenClaimSafeClauseTwoLabel::Reference,
                },
            );
        let mut telescope = Telescope::reference(15);
        telescope.clauses[1].expr = Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
        telescope.clauses[3] = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor + 1)),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            ))),
        );
        telescope.clauses[4] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
        );
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            reanchor.allows_historical_reanchor(),
            "the representative claim-safe reference clause-two sheet stays structurally reanchorable under a hand-constructed exact pair, even though the search lane never exposes the anchor-11 exact-argument pocket there"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 11,
                historical_reanchor: true,
            }
        );
        assert!(passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_tracks_claim_safe_pair_reason_progress_below_the_representative_dead_prefix() {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let _override =
            super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two(
                super::ClaimStepFifteenClaimSafePairClauseTwoSelector {
                    clause_one: super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                    clause_two: super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimFlatDomain,
                },
            );

        let mut exact_pair = Telescope::reference(15);
        exact_pair.clauses[1].expr = Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
        exact_pair.clauses[2].expr = Expr::Pi(
            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
            Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
        );
        exact_pair.clauses[3] = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor + 1)),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            ))),
        );
        exact_pair.clauses[4] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
        );
        exact_pair.clauses[7] = reference_terminal.clone();

        let exact_progress = HistoricalReanchorSummary::from_telescope(&library, &exact_pair)
            .claim_safe_sharp_codomain_pair_progress()
            .expect("the representative claim-safe pair override should expose progress");
        assert_eq!(exact_progress, HistoricalReanchorProgress::new(8, None));

        let mut dead_prefix_reference_terminal = exact_pair.clone();
        dead_prefix_reference_terminal.clauses[5] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Flat(Box::new(Expr::Var(1))),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
            ),
        );
        dead_prefix_reference_terminal.clauses[6] = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(2)),
            ))),
        );

        let dead_progress =
            HistoricalReanchorSummary::from_telescope(&library, &dead_prefix_reference_terminal)
                .claim_safe_sharp_codomain_pair_progress()
                .expect("the representative dead prefix should keep the same progress surface");
        assert_eq!(dead_progress, HistoricalReanchorProgress::new(5, Some(5)));
        assert!(
            !HistoricalReanchorSummary::from_telescope(&library, &dead_prefix_reference_terminal)
                .allows_historical_reanchor(),
            "once the representative dead-prefix shell deviates at clause 5, the claim-safe pair reanchor path should stay blocked before clause 6 or terminal identity can matter"
        );
    }

    #[test]
    fn connectivity_keeps_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_claim_clause_two_reference_terminal_only_even_under_override()
     {
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (clause_two_selector, clause_two_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimFlatDomain,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                ),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimSharpCodomain,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two(
                    super::ClaimStepFifteenClaimSafePairClauseTwoSelector {
                        clause_one: super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                        clause_two: clause_two_selector,
                    },
                );
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[7] = lifted_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the representative claim-safe claim-side clause-two override should stay reference-terminal-only and keep lifted terminals fenced"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_bridge_on_exact_claim_safe_pair_under_override()
    {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selector, clause_one_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimSharpCodomain,
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_pair(
                    selector,
                );
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                        Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the exact claim-safe pair bridge override should admit only the selected mismatch-1 pairing on the clause-4 demo-sharp-bridge side"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_four_demo_sharp_bridge_on_sibling_claim_safe_pair_closed_even_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selector, sibling_clause_one_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimSharpCodomain,
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_pair(
                    selector,
                );
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = sibling_clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                        Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-safe pair bridge override should keep the sibling mismatch-1 pairing closed"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_four_demo_sharp_bridge_on_exact_claim_safe_pair_reference_terminal_only_even_under_override()
     {
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for (selector, clause_one_variant) in [
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ),
            (
                super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimSharpCodomain,
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            ),
        ] {
            let _override =
                super::override_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_pair(
                    selector,
                );
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                        Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the exact claim-safe pair bridge override should remain reference-terminal-only and keep lifted terminals fenced"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_on_the_exact_remaining_two_mismatch_one_bridge_slice()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice(
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_one_variants = [
            Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ];

        for clause_one_variant in clause_one_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                        Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[5] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                            Expr::Sharp(Box::new(Expr::Var(1))),
                        ))))),
                        Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                            Expr::Var(1),
                        ))))),
                    ),
                );
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the clause-5 demo-sharp-domain opening should now count as historical reanchor on the exact remaining-two mismatch-1 bridge slice"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_on_the_exact_remaining_two_mismatch_one_bridge_slice()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice(
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_one_variants = [
            Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ];

        for clause_one_variant in clause_one_variants {
            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[1].expr = clause_one_variant.clone();
                telescope.clauses[2].expr = clause_two_variant;
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[4] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                        Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                            Box::new(Expr::Var(1)),
                        )))))),
                    ),
                );
                telescope.clauses[5] = ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                            Expr::Var(1),
                        ))))),
                        Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                            Expr::Flat(Box::new(Expr::Var(1))),
                        ))))),
                    ),
                );
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the clause-5 demo-flat-codomain opening should now count as historical reanchor on the exact remaining-two mismatch-1 bridge slice"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_clause_five_remaining_two_mismatch_zero_bridge_slice_outside_historical_reanchor_even_under_mismatch_one_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice(
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero_variants = [
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ];
        let clause_one_variants = [
            Expr::Eventually(Box::new(Expr::Var(1))),
            Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
        ];
        let clause_five_variants = [
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            ),
        ];

        for clause_zero_variant in clause_zero_variants {
            for clause_one_variant in clause_one_variants.iter() {
                for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                    for clause_five_variant in clause_five_variants.iter().cloned() {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr = clause_zero_variant.clone();
                        telescope.clauses[1].expr = clause_one_variant.clone();
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3] = ClauseRec::new(
                            ClauseRole::Introduction,
                            Expr::Lam(Box::new(Expr::App(
                                Box::new(Expr::Lib(anchor + 1)),
                                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                            ))),
                        );
                        telescope.clauses[4] = ClauseRec::new(
                            ClauseRole::Formation,
                            Expr::Pi(
                                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                                    Box::new(Expr::Var(1)),
                                )))))),
                                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                                    Box::new(Expr::Var(1)),
                                )))))),
                            ),
                        );
                        telescope.clauses[5] = clause_five_variant;
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            !reanchor.allows_historical_reanchor(),
                            "the exact remaining-two mismatch-1 override should still keep the mismatch-0 bridge slice outside historical reanchor"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: false,
                                max_lib_ref: 11,
                                historical_reanchor: false,
                            }
                        );
                        assert!(
                            !passes_connectivity(&library, &telescope),
                            "the exact remaining-two mismatch-1 override should still fence mismatch-0 bridge prefixes out of the live claim path"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_on_claim_safe_clause_zero_one_surface_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_zero_variant in claim_temporal_variant_exprs(0, anchor) {
            for clause_one_variant in claim_temporal_variant_exprs(1, anchor) {
                for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1].expr = clause_one_variant.clone();
                    telescope.clauses[2].expr = clause_two_variant;
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[5] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                                Expr::Sharp(Box::new(Expr::Var(1))),
                            ))))),
                            Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                                Expr::Var(1),
                            ))))),
                        ),
                    );
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the clause-5 demo-sharp-domain opening should become historically reanchorable across the claim-safe clause-0/1 surface only under the scoped negative-control override"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_on_claim_safe_clause_zero_one_surface_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one();
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_zero_variant in claim_temporal_variant_exprs(0, anchor) {
            for clause_one_variant in claim_temporal_variant_exprs(1, anchor) {
                for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero_variant.clone();
                    telescope.clauses[1].expr = clause_one_variant.clone();
                    telescope.clauses[2].expr = clause_two_variant;
                    telescope.clauses[3] = ClauseRec::new(
                        ClauseRole::Introduction,
                        Expr::Lam(Box::new(Expr::App(
                            Box::new(Expr::Lib(anchor + 1)),
                            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                        ))),
                    );
                    telescope.clauses[5] = ClauseRec::new(
                        ClauseRole::Formation,
                        Expr::Pi(
                            Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                                Expr::Var(1),
                            ))))),
                            Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                                Expr::Flat(Box::new(Expr::Var(1))),
                            ))))),
                        ),
                    );
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the clause-5 demo-flat-codomain opening should become historically reanchorable across the claim-safe clause-0/1 surface only under the scoped negative-control override"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 11,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_on_the_exact_anchor_eleven_pocket_without_the_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-sharp-domain opening should now count as historical reanchor directly on the exact anchor-11 pocket even before the clause-4 side pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_on_the_exact_anchor_eleven_pocket_without_the_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-flat-codomain opening should now count as historical reanchor directly on the exact anchor-11 pocket even before the clause-4 side pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_two_anchor_eleven_exact_argument_pocket_outside_historical_reanchor_when_clause_six_moves()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_six_variant in claim_temporal_variant_exprs(6, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[6].expr = clause_six_variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the isolated anchor-11 exact-argument pocket should stay fenced once clause 6 also deviates"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_parent_route_under_override() {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_zero = Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
        let clause_one = Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr = clause_zero.clone();
                    telescope.clauses[1].expr = clause_one.clone();
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero claim-side parent route should qualify every claim-side clause-two / clause-three / clause-six combination under the scoped override"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_reference_clause_two_parent_shell_closed_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );

        for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0].expr =
                Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[3].expr = clause_three_variant;
            telescope.clauses[4].expr = clause_four.clone();
            telescope.clauses[5].expr = clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the representative mismatch-zero parent-route probe should stay claim-side only and keep the sibling reference clause-two sheet closed"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_reference_terminal_only_even_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero claim-side parent-route probe should keep lifted terminals fenced even when the parent shell qualifies"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_parent_route_reference_clause_four_under_narrow_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_four(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = representative_mismatch_zero_claim_side_parent_route_clause_four_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
        );
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the narrow representative mismatch-zero parent-route clause-four override should qualify the targeted claim-side shell when only the reference bridge clause is reopened"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_claim_clause_four_closed_under_narrow_reference_clause_four_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_four(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = representative_mismatch_zero_claim_side_parent_route_clause_four_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::ClaimNextBridge,
        );
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the narrow representative mismatch-zero parent-route clause-four override should keep the sibling claim-next-bridge branch fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_reference_clause_two_closed_even_under_narrow_reference_clause_four_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_four(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = representative_mismatch_zero_claim_side_parent_route_clause_four_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
        );
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
            for clause_six_variant in &clause_six_variants {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[0].expr =
                    Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                telescope.clauses[1].expr =
                    Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                telescope.clauses[3].expr = clause_three_variant.clone();
                telescope.clauses[4].expr = clause_four.clone();
                telescope.clauses[5].expr = clause_five.clone();
                telescope.clauses[6].expr = clause_six_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the narrow representative mismatch-zero parent-route clause-four override should stay claim-side only and keep the sibling reference clause-two sheet closed"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_reference_clause_four_reference_terminal_only_even_under_narrow_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_four(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = representative_mismatch_zero_claim_side_parent_route_clause_four_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFourLabel::Reference,
        );
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the narrow representative mismatch-zero parent-route clause-four override should keep lifted terminals fenced even when the selected reference bridge shell qualifies"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_parent_route_reference_clause_six_under_narrow_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let clause_six = representative_mismatch_zero_claim_side_parent_route_clause_six_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
        );

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[0].expr =
                    Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                telescope.clauses[1].expr =
                    Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3].expr = clause_three_variant.clone();
                telescope.clauses[4].expr = clause_four.clone();
                telescope.clauses[5].expr = clause_five.clone();
                telescope.clauses[6].expr = clause_six.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "the narrow representative mismatch-zero parent-route override should qualify the targeted claim-side shell when only the reference clause-six continuation is reopened"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_parent_route_clause_three_under_narrow_override()
     {
        let clause_three_labels = [
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument,
        ];
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_three_label in clause_three_labels {
            let _override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_three(
                    super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                    clause_three_label,
                );
            let clause_three =
                representative_mismatch_zero_claim_side_parent_route_clause_three_expr(
                    anchor,
                    clause_three_label,
                );

            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        reanchor.allows_historical_reanchor(),
                        "the narrow representative mismatch-zero parent-route override should qualify the targeted claim-side shell when only one clause-three argument branch is reopened"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: true,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_sibling_clause_three_closed_under_narrow_override()
     {
        let clause_three_labels = [
            (
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument,
            ),
            (
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument,
            ),
        ];
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for (selected_clause_three, sibling_clause_three) in clause_three_labels {
            let _override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_three(
                    super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                    selected_clause_three,
                );
            let clause_three =
                representative_mismatch_zero_claim_side_parent_route_clause_three_expr(
                    anchor,
                    sibling_clause_three,
                );

            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the narrow representative mismatch-zero parent-route override should keep the sibling claim clause-three argument branch fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_reference_clause_two_closed_even_under_narrow_clause_three_override()
     {
        let clause_three_labels = [
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument,
        ];
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_three_label in clause_three_labels {
            let _override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_three(
                    super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                    clause_three_label,
                );
            let clause_three =
                representative_mismatch_zero_claim_side_parent_route_clause_three_expr(
                    anchor,
                    clause_three_label,
                );

            for clause_six_variant in &clause_six_variants {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[0].expr =
                    Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                telescope.clauses[1].expr =
                    Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                telescope.clauses[3].expr = clause_three.clone();
                telescope.clauses[4].expr = clause_four.clone();
                telescope.clauses[5].expr = clause_five.clone();
                telescope.clauses[6].expr = clause_six_variant.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the narrow representative mismatch-zero parent-route clause-three override should stay claim-side only and keep the sibling reference clause-two sheet closed"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_reference_terminal_only_even_under_narrow_clause_three_override()
     {
        let clause_three_labels = [
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimFlatArgument,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseThreeLabel::ClaimEventualArgument,
        ];
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_three_label in clause_three_labels {
            let _override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_three(
                    super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                    clause_three_label,
                );
            let clause_three =
                representative_mismatch_zero_claim_side_parent_route_clause_three_expr(
                    anchor,
                    clause_three_label,
                );

            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the narrow representative mismatch-zero parent-route clause-three override should keep lifted terminals fenced even when the selected parent shell qualifies"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_claim_clause_six_variants_closed_under_narrow_reference_clause_six_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in claim_temporal_variant_exprs(6, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant;
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the narrow representative mismatch-zero parent-route override should keep the two claim clause-six siblings fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_reference_clause_two_closed_even_under_narrow_reference_clause_six_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let clause_six = representative_mismatch_zero_claim_side_parent_route_clause_six_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
        );

        for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0].expr =
                Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[3].expr = clause_three_variant;
            telescope.clauses[4].expr = clause_four.clone();
            telescope.clauses[5].expr = clause_five.clone();
            telescope.clauses[6].expr = clause_six.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the narrow representative mismatch-zero parent-route override should stay claim-side only and keep the sibling reference clause-two sheet closed"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_reference_clause_six_reference_terminal_only_even_under_narrow_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero parent route should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let clause_six = representative_mismatch_zero_claim_side_parent_route_clause_six_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
        );

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[0].expr =
                    Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                telescope.clauses[1].expr =
                    Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3].expr = clause_three_variant.clone();
                telescope.clauses[4].expr = clause_four.clone();
                telescope.clauses[5].expr = clause_five.clone();
                telescope.clauses[6].expr = clause_six.clone();
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the narrow representative mismatch-zero parent-route override should keep lifted terminals fenced even when the reference clause-six parent shell qualifies"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_active_window_reference_clause_six_on_claim_flat_codomain_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero active-window reference-clause-six probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let clause_six = representative_mismatch_zero_claim_side_parent_route_clause_six_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
        );

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[0].expr =
                    Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                telescope.clauses[1].expr =
                    Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3].expr = clause_three_variant.clone();
                telescope.clauses[4].expr = clause_four.clone();
                telescope.clauses[5].expr = clause_five.clone();
                telescope.clauses[6].expr = clause_six.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(!reanchor.allows_historical_reanchor());
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: true,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_active_window_claim_clause_six_variants_closed_under_narrow_reference_clause_six_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero active-window reference-clause-six probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in claim_temporal_variant_exprs(6, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant;
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(!reanchor.allows_historical_reanchor());
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_active_window_on_claim_flat_codomain_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero active-window probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero claim-side active-window probe should stay distinct from the spent historical-reanchor route family"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: true,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_reference_clause_two_closed_even_under_active_window_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero active-window probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );

        for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0].expr =
                Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[3].expr = clause_three_variant;
            telescope.clauses[4].expr = clause_four.clone();
            telescope.clauses[5].expr = clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the representative mismatch-zero active-window probe should stay claim-side only and keep the sibling reference clause-two sheet closed"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_active_window_reference_terminal_only_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero active-window probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero active-window probe should keep lifted terminals fenced even when the parent shell qualifies on the reference terminal"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_active_window_on_reference_clause_five_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero active-window probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the sibling active-window probe on the reference clause-five family should stay distinct from the spent historical-reanchor route family too"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: true,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_parent_route_plus_active_window_on_both_active_clause_five_families_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero hybrid probe should expose a claim-next-bridge clause");

        for clause_five_label in [
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference,
        ] {
            let _parent_route_override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
                    clause_five_label,
                );
            let _active_window_override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                    clause_five_label,
                );
            let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
                anchor,
                clause_five_label,
            );
            let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
            clause_six_variants.push(reference_temporal_clause_six());

            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                    for clause_six_variant in &clause_six_variants {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr =
                            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                        telescope.clauses[1].expr =
                            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3].expr = clause_three_variant.clone();
                        telescope.clauses[4].expr = clause_four.clone();
                        telescope.clauses[5].expr = clause_five.clone();
                        telescope.clauses[6].expr = clause_six_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            reanchor.allows_historical_reanchor(),
                            "the representative mismatch-zero hybrid probe should keep the targeted claim-side shell historically reanchored on both active clause-five families"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: true,
                                self_contained: false,
                                max_lib_ref: 10,
                                historical_reanchor: true,
                            }
                        );
                        assert!(passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_plus_active_window_reference_terminal_only_under_override()
     {
        let _parent_route_override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let _active_window_override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero hybrid probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero hybrid probe should still keep lifted terminals fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_parent_route_plus_self_contained_on_both_active_clause_five_families_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero hybrid probe should expose a claim-next-bridge clause");

        for clause_five_label in [
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference,
        ] {
            let _parent_route_override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
                    clause_five_label,
                );
            let _self_contained_override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                    clause_five_label,
                );
            let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
                anchor,
                clause_five_label,
            );
            let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
            clause_six_variants.push(reference_temporal_clause_six());

            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                    for clause_six_variant in &clause_six_variants {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr =
                            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                        telescope.clauses[1].expr =
                            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3].expr = clause_three_variant.clone();
                        telescope.clauses[4].expr = clause_four.clone();
                        telescope.clauses[5].expr = clause_five.clone();
                        telescope.clauses[6].expr = clause_six_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            reanchor.allows_historical_reanchor(),
                            "the representative mismatch-zero parent-route plus self-contained hybrid should keep the targeted claim-side shell historically reanchored on both active clause-five families"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: false,
                                self_contained: true,
                                max_lib_ref: 10,
                                historical_reanchor: true,
                            }
                        );
                        assert!(passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_parent_route_plus_self_contained_reference_terminal_only_under_override()
     {
        let _parent_route_override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_parent_route(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let _self_contained_override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero hybrid probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero parent-route plus self-contained hybrid should still keep lifted terminals fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_active_window_plus_self_contained_on_both_active_clause_five_families_under_override()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero looser recombined probe should expose a claim-next-bridge clause");

        for clause_five_label in [
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference,
        ] {
            let _active_window_override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                    clause_five_label,
                );
            let _self_contained_override =
                super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                    clause_five_label,
                );
            let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
                anchor,
                clause_five_label,
            );
            let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
            clause_six_variants.push(reference_temporal_clause_six());

            for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
                for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                    for clause_six_variant in &clause_six_variants {
                        let mut telescope = Telescope::reference(15);
                        telescope.clauses[0].expr =
                            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                        telescope.clauses[1].expr =
                            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                        telescope.clauses[2].expr = clause_two_variant.clone();
                        telescope.clauses[3].expr = clause_three_variant.clone();
                        telescope.clauses[4].expr = clause_four.clone();
                        telescope.clauses[5].expr = clause_five.clone();
                        telescope.clauses[6].expr = clause_six_variant.clone();
                        telescope.clauses[7] = reference_terminal.clone();

                        let witness = analyze_connectivity(&library, &telescope);
                        let reanchor =
                            HistoricalReanchorSummary::from_telescope(&library, &telescope);
                        assert!(
                            !reanchor.allows_historical_reanchor(),
                            "the representative mismatch-zero looser recombined probe should stay distinct from the spent historical-reanchor route families"
                        );
                        assert_eq!(
                            witness,
                            ConnectivityWitness {
                                connected: true,
                                references_active_window: true,
                                self_contained: true,
                                max_lib_ref: 10,
                                historical_reanchor: false,
                            }
                        );
                        assert!(passes_connectivity(&library, &telescope));
                    }
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_active_window_plus_self_contained_reference_terminal_only_under_override()
     {
        let _active_window_override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_active_window(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let _self_contained_override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero looser recombined probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero looser recombined probe should still keep lifted terminals fenced"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_self_contained_reference_clause_six_on_claim_flat_codomain_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero self-contained reference-clause-six probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let clause_six = representative_mismatch_zero_claim_side_parent_route_clause_six_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
        );

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[0].expr =
                    Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                telescope.clauses[1].expr =
                    Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3].expr = clause_three_variant.clone();
                telescope.clauses[4].expr = clause_four.clone();
                telescope.clauses[5].expr = clause_five.clone();
                telescope.clauses[6].expr = clause_six.clone();
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(!reanchor.allows_historical_reanchor());
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: true,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_self_contained_claim_clause_six_variants_closed_under_narrow_reference_clause_six_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained_clause_six(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseSixLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero self-contained reference-clause-six probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in claim_temporal_variant_exprs(6, anchor) {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant;
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(!reanchor.allows_historical_reanchor());
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_self_contained_on_claim_flat_codomain_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero self-contained probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero claim-side self-contained probe should stay distinct from the spent historical-reanchor route family"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: true,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_reference_clause_two_closed_even_under_self_contained_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero self-contained probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );

        for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0].expr =
                Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[3].expr = clause_three_variant;
            telescope.clauses[4].expr = clause_four.clone();
            telescope.clauses[5].expr = clause_five.clone();
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the representative mismatch-zero self-contained probe should stay claim-side only and keep the sibling reference clause-two sheet closed"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_representative_mismatch_zero_claim_side_self_contained_reference_terminal_only_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
            );
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero self-contained probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::ClaimFlatCodomain,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = lifted_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the representative mismatch-zero self-contained probe should keep lifted terminals fenced even when the parent shell qualifies on the reference terminal"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: false,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(!passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_accepts_representative_mismatch_zero_claim_side_self_contained_on_reference_clause_five_under_override()
     {
        let _override =
            super::override_claim_step_fifteen_representative_mismatch_zero_claim_side_self_contained(
                super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference,
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_four = claim_temporal_variant_exprs(4, anchor)
            .into_iter()
            .next()
            .expect("representative mismatch-zero self-contained probe should expose a claim-next-bridge clause");
        let clause_five = representative_mismatch_zero_claim_side_parent_route_clause_five_expr(
            anchor,
            super::ClaimStepFifteenRepresentativeMismatchZeroClaimSideParentRouteClauseFiveLabel::Reference,
        );
        let mut clause_six_variants = claim_temporal_variant_exprs(6, anchor);
        clause_six_variants.push(reference_temporal_clause_six());

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_three_variant in claim_temporal_variant_exprs(3, anchor) {
                for clause_six_variant in &clause_six_variants {
                    let mut telescope = Telescope::reference(15);
                    telescope.clauses[0].expr =
                        Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1)))));
                    telescope.clauses[1].expr =
                        Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
                    telescope.clauses[2].expr = clause_two_variant.clone();
                    telescope.clauses[3].expr = clause_three_variant.clone();
                    telescope.clauses[4].expr = clause_four.clone();
                    telescope.clauses[5].expr = clause_five.clone();
                    telescope.clauses[6].expr = clause_six_variant.clone();
                    telescope.clauses[7] = reference_terminal.clone();

                    let witness = analyze_connectivity(&library, &telescope);
                    let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                    assert!(
                        !reanchor.allows_historical_reanchor(),
                        "the sibling self-contained probe on the reference clause-five family should stay distinct from the spent historical-reanchor route family too"
                    );
                    assert_eq!(
                        witness,
                        ConnectivityWitness {
                            connected: true,
                            references_active_window: false,
                            self_contained: true,
                            max_lib_ref: 10,
                            historical_reanchor: false,
                        }
                    );
                    assert!(passes_connectivity(&library, &telescope));
                }
            }
        }
    }

    #[test]
    fn connectivity_splits_representative_claim_safe_clause_five_labels_into_one_exact_pair_match_plus_four_dead_controls()
     {
        let _override =
            super::override_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_pair_clause_two(
                super::ClaimStepFifteenClaimSafePairClauseTwoSelector {
                    clause_one: super::ClaimStepFifteenClaimSafeClauseOneLabel::ClaimNextCodomain,
                    clause_two: super::ClaimStepFifteenClaimSafeClauseTwoLabel::ClaimFlatDomain,
                },
            );
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let clause_five_variants = [
            (
                "reference",
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                            Expr::Var(1),
                        ))))),
                        Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                            Expr::Var(1),
                        ))))),
                    ),
                ),
                HistoricalReanchorProgress::new(8, None),
                true,
                false,
                true,
            ),
            (
                "claim_flat_codomain",
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                            Expr::Flat(Box::new(Expr::Var(1))),
                        ))))),
                        Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                            Expr::Var(1),
                        ))))),
                    ),
                ),
                HistoricalReanchorProgress::new(5, Some(5)),
                false,
                false,
                false,
            ),
            (
                "demo_sharp_domain",
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                            Expr::Sharp(Box::new(Expr::Var(1))),
                        ))))),
                        Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                            Expr::Var(1),
                        ))))),
                    ),
                ),
                HistoricalReanchorProgress::new(5, Some(5)),
                false,
                false,
                false,
            ),
            (
                "demo_flat_codomain",
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                            Expr::Var(1),
                        ))))),
                        Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                            Expr::Flat(Box::new(Expr::Var(1))),
                        ))))),
                    ),
                ),
                HistoricalReanchorProgress::new(5, Some(5)),
                false,
                false,
                false,
            ),
            (
                "claim_next_codomain",
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(
                        Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                            Expr::Var(1),
                        ))))),
                        Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                            Expr::Next(Box::new(Expr::Var(1))),
                        ))))),
                    ),
                ),
                HistoricalReanchorProgress::new(5, Some(5)),
                false,
                false,
                false,
            ),
        ];

        for (
            label,
            clause_five,
            expected_progress,
            expected_exact_pair_match,
            expected_side_pocket_match,
            expected_historical_reanchor,
        ) in clause_five_variants
        {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1].expr =
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1)))));
            telescope.clauses[2].expr = Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            );
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = clause_five;
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);

            assert_eq!(
                reanchor.claim_safe_sharp_codomain_pair_progress(),
                Some(expected_progress),
                "the representative claim-safe exact pair should report the expected clause-five progress split on {label}"
            );
            assert_eq!(
                reanchor.claim_safe_sharp_codomain_pair_prefix_matches(),
                expected_exact_pair_match,
                "the representative claim-safe exact pair should only stay fully matched on the expected clause-five labels"
            );
            assert_eq!(
                reanchor.clause_five_side_pocket_prefix_matches(),
                expected_side_pocket_match,
                "once clause one has moved onto the representative claim-safe pair, the clause-five side-pocket qualifier should no longer survive on the off-reference clause-five labels"
            );
            assert_eq!(
                reanchor.allows_historical_reanchor(),
                expected_historical_reanchor,
                "the representative claim-safe clause-five labels should split cleanly into one exact-pair reference control plus four dead off-reference controls"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: expected_historical_reanchor,
                }
            );
            assert_eq!(
                passes_connectivity(&library, &telescope),
                expected_historical_reanchor,
                "the representative claim-safe clause-five qualification split should align with live connectivity on {label}"
            );
        }
    }
}
