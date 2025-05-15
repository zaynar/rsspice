//
// GENERATED FILE
//

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_mut, unused_assignments)]
#![allow(unused_parens, clippy::double_parens)]
#![allow(unused_variables)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::let_and_return)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_bool_assign)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_update)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::self_assignment)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::unnecessary_to_owned)]
#![allow(clippy::while_immutable_condition)]

mod api;
pub mod required_reading;

pub mod spicelib;

#[cfg(feature = "tspice")]
pub mod support;

#[cfg(feature = "tspice")]
pub mod testutil;

#[cfg(feature = "tspice")]
pub mod tspice;
