//! Encodes the rules about which accesses invalidate which loans. These are the rules from the
//! [NLL RFC].
//!
//! [NLL RFC]: https://github.com/rust-lang/rfcs/blob/master/text/2094-nll.md#borrow-checker-phase-2-reporting-errors

use derive_more::IsVariant;

use crate::ast::{self, Place, Projection};

pub struct Access<'a> {
    place: &'a Place,
    rw: AccessRw,
    depth: AccessDepth,
}

impl<'a> Access<'a> {
    pub fn from_expr(access: &'a ast::ExprAccess) -> Self {
        use self::AccessDepth::*;
        use self::AccessRw::*;

        let (rw, depth) = match access.kind {
            ast::AccessKind::Copy => (Read, Deep),
            ast::AccessKind::Move => (Write, Deep),
            ast::AccessKind::Borrow(_) => (Read, Deep),
            ast::AccessKind::BorrowMut(_) => (Write, Deep),
        };

        Access {
            place: &access.place,
            rw,
            depth,
        }
    }

    /// Returns the `Access` representing the left-hand side of an assignment (`lhs = ...`).
    pub fn from_assignment_lhs(lhs: &'a Place) -> Self {
        Access {
            place: lhs,
            rw: AccessRw::Write,
            depth: AccessDepth::Shallow,
        }
    }

    pub fn invalidates(&self, loan: Loan) -> bool {
        // An access of a place invalidates a loan of a place if all of the following hold.

        // - Either the access or the loan allows mutation.
        if self.rw.is_read() && loan.kind.is_shared() {
            return false;
        }

        // - One of the two places is a prefix of the other.
        if self.place.is_disjoint(&loan.place) {
            return false;
        }

        // - If the loan is a shallow one, the
        //
        // FIXME: Should this be less/greater than? Does a shallow access of `*p` invalidate a
        // loan of `p`?
        if self.depth.is_shallow() && self.place.num_derefs() != loan.place.num_derefs() {
            return false;
        }

        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IsVariant)]
pub enum AccessRw {
    Read,
    Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IsVariant)]
pub enum AccessDepth {
    /// An access that does not touch any origins in the type of the accessed place.
    Shallow,

    /// An access to some place `p` that accesses all origins in the type of `p`.
    Deep,
}

pub struct Loan<'a> {
    pub place: &'a Place,
    pub kind: BorrowKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IsVariant)]
pub enum BorrowKind {
    /// `&mut`.
    Unique,

    /// `&`.
    Shared,
}
