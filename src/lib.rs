//! Pure Rust port of the SPICE Toolkit for space geometry.
//!
//! This implementation is fully memory-safe and thread-safe,
//! and does not depend on any external C/FORTRAN libraries.
//! It provides nearly the entire SPICELIB API.
//! The code has been mechanically [translated](https://github.com/zaynar/f2rust)
//! from the FORTRAN version of the SPICE Toolkit into Rust.
//!
//! It is completely unofficial, unsupported, and not heavily tested
//! (though it does pass the Toolkit's regression tests so it's probably
//! not too bad). Use at your own risk.
//!
//! Users of this library should not expect any support from NAIF.
//! Use the [GitHub project](https://github.com/zaynar/rsspice)
//! for any issues or queries.
//!
//! # Usage example
//!
//! This example demonstrates the general design:
//!
//! ```no_run
//! use rsspice::*;
//!
//! const TIMFMT: &str = "YYYY MON DD HR:MN:SC.###### (TDB)::TDB";
//! const MAXWIN: usize = 2 * 100;
//!
//! // Find solar eclipses as seen from the center of the Earth.
//! fn main() -> Result<()> {
//!     let mut spice = SpiceContext::new();
//!
//!     spice.furnsh("gfoclt_ex1.tm")?;
//!
//!     let mut confine = Cell::with_capacity(2);
//!     let mut result = Cell::with_capacity(MAXWIN);
//!
//!     let et0 = spice.str2et("2027 JAN 01 00:00:00 TDB")?;
//!     let et1 = spice.str2et("2029 JAN 01 00:00:00 TDB")?;
//!
//!     spice.wninsd(et0, et1, &mut confine)?;
//!
//!     spice.gfoclt(
//!         "ANY",
//!         "MOON", "ellipsoid", "IAU_MOON",
//!         "SUN", "ellipsoid", "IAU_SUN",
//!         "LT", "EARTH", 180.0, &confine,
//!         &mut result,
//!     )?;
//!
//!     for i in 1..=spice.wncard(&result)? {
//!         let (left, right) = spice.wnfetd(&result, i)?;
//!         println!(
//!             "Interval {i}: {} - {}",
//!             spice.timout(left, TIMFMT)?,
//!             spice.timout(right, TIMFMT)?
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! (See more [examples](https://github.com/zaynar/rsspice/tree/main/examples)
//! and [lessons](https://github.com/zaynar/rsspice/tree/main/lessons).)
//!
//! The [`SpiceContext`] object encapsulates all the SPICE state, such as loaded
//! kernels. There is no process-wide global state, so you can run multiple
//! `SpiceContext`s concurrently in separate threads.
//!
//! There is a 1:1 mapping between functions in the FORTRAN API and
//! in the Rust API, so you can refer to the extensive FORTRAN documentation
//! and adapt its examples relatively easily.
//! (The API docs and [required reading](required_reading) are mirrored in rustdoc.
//! Further tutorials and lessons are available from [NAIF](https://naif.jpl.nasa.gov/).)
//!
//! Arguments are not exactly a 1:1 mapping -- we try to turn them into more idiomatic Rust types.
//! Output arguments are typically mapped onto return values.
//! SPICE errors are mapped onto Rust's error system, so they can
//! be easily handled with `Result` and `?`.
//!
//! FORTRAN arrays are typically indexed from 1, not 0.
//! Functions like `wnfetd` similarly start counting from 1; we do not attempt
//! any automatic translation of indexes.
//! (This differs from the CSPICE translation, and wrappers around CSPICE,
//! which count from 0.)
//!
//! There is a [`Cell`] type for SPICE cells (including windows and sets), as they are
//! particularly awkward to handle with standard Rust types.
//! But `Cell` only provides very basic functionality;
//! this library is not attempting to provide a higher-level API than the original SPICELIB.
//!
//! # Background
//!
//! SPICE is "an observation geometry system for space science missions",
//! developed by NASA's
//! [Navigation and Ancillary Information Facility (NAIF)](https://naif.jpl.nasa.gov/).
//!
//! A large amount of geometric data about planets, moons, and spacecraft is
//! publicly available as SPICE data, which can be processed using the SPICE Toolkit
//! software and APIs. NAIF also provides thorough documentation
//! of the system.
//!
//! The SPICE Toolkit is originally developed in FORTRAN, with an official
//! translation to C. Official and unofficial bindings for the C library
//! are available in several other languages (including
//! [rust-spice](https://crates.io/crates/rust-spice)).
//! `rsspice` is an unofficial translation from FORTRAN to Rust, with
//! a number of benefits and drawbacks:
//!
//! * Memory-safe: Rust's bounds-checking ensures that many errors will be
//! detected at runtime and will not result in data corruption.
//!
//! * Thread-safe: The FORTRAN and C implementations depend heavily on global
//! state. `rsspice` moves that state into the `SpiceContext` object, allowing
//! concurrency within a single process.
//! (See [a basic example](https://github.com/zaynar/f2rust/blob/main/rsspice/examples/gfoclt_ex1_mt.rs)
//! using `rayon` to split work across threads.)
//!
//! * Portability: This should work on any platform that Rust supports,
//! including WebAssembly (albeit with some complications around filesystem access).
//!
//! * Much less testing: `rsspice` includes a translation of the TSPICE
//! regression tests, which are reasonably extensive but do not have
//! full coverage of the whole API. There is a higher risk of bugs than
//! in a wrapper around the well-tested FORTRAN/C implementations.
//!
//! # API mapping details
//!
//! The API is mechanically translated from the FORTRAN API, following a number
//! of rules to make it closer to idiomatic Rust.
//! Understanding these can help when reading the FORTRAN documentation.
//!
//! ## Return values
//!
//! Arguments that are documented as "O" are turned into return values.
//! If there are multiple outputs, the function will return a tuple.
//! If one of the outputs is called `FOUND`, it will not be returned explicitly;
//! instead the function will return an `Option<_>`.
//!
//! There are some exceptions, typically array outputs where there is no
//! well-defined maximum size that we can allocate automatically. In this case
//! they are mapped onto `&mut` parameters, and you must initialise the
//! array with an appropriate size before the call.
//!
//! For returned errors, see [`Error`].
//!
//! ## Strings
//!
//! Input strings are `&str`, outputs are typically `String`, mixed input-output
//! and some outputs are `&mut str`.
//!
//! Since FORTRAN 77 does not have dynamic allocation, output strings are
//! allocated at the maximum possible size, and FORTRAN will pad the output
//! with trailing space characters.
//! We trim the trailing spaces before returning a `String`.
//! When using `&mut str`, you are responsible for allocating and trimming:
//!
//! ```
//! # use rsspice::*;
//! # let mut spice = SpiceContext::new();
//! let mut outstr = " ".repeat(80);
//! spice.replwd("Hello world", 1, "Goodbye", &mut outstr);
//! assert_eq!(outstr.trim_ascii_end(), "Goodbye world");
//! ```
//!
//! For FORTRAN `CHARACTER` arrays, see [`CharVec`].
//!
//! FORTRAN 77 barely even understands ASCII, never mind UTF-8.
//! Input strings are simply interpreted as a sequence of bytes.
//! For outputs, the implementation will panic if it ends up producing a non-UTF-8
//! string; this should not happen unless you pass non-ASCII characters into the API
//! (so don't do that).
//!
//! ## Hidden arguments
//!
//! Some functions (like [`GFDIST`](raw::gfdist)) have a `WORK` argument,
//! for temporary data storage.
//! In `rsspice` we dynamically allocate that storage space, so the `WORK`
//! argument and the corresponding `NW` size are removed from the API.
//!
//! ## Omitted functions
//!
//! A small number of functions are excluded from the API, because they are:
//! * Private;
//! * Deprecated/obsolete;
//! * Documented as "DO NOT CALL THIS ROUTINE", or return a `BOGUSENTRY` error;
//! * Redundant with basic Rust functionality, particularly string manipulation.
//!
//! These functions are still available in the [`raw`] API.
//!
//! ## Array arguments
//!
//! 3D vector arguments are typically represented as `&[f64; 3]`.
//! You can conveniently use [`nalgebra`](https://nalgebra.org/) for these:
//!
//! ```
//! # use rsspice::*;
//! # use approx::assert_relative_eq;
//! use nalgebra as na;
//! # let mut spice = SpiceContext::new();
//! let v = na::Vector3::new(1.0, 2.0, 3.0);
//! let r = na::Vector3::from(
//!     spice.vrotv(v.as_ref(), &[0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2));
//! assert_relative_eq!(r, na::Vector3::new(-2.0, 1.0, 3.0));
//! ```
//!
//! If you want to use an `&[f64]` slice (e.g. from a `Vec`), you can use
//! `try_into().unwrap()` (which will panic if the slice is too small):
//!
//! ```
//! # use rsspice::*;
//! # use approx::assert_relative_eq;
//! # let mut spice = SpiceContext::new();
//! let v = [0.0, 1.0, 2.0, 3.0, 4.0];
//! let r = spice.vrotv(&v[1..4].try_into().unwrap(),
//!     &[0.0, 0.0, 1.0],
//!     std::f64::consts::FRAC_PI_2);
//! assert_relative_eq!(r.as_slice(), [-2.0, 1.0, 3.0].as_slice());
//! ```
//!
//! Matrices are represented as `&[[f64; 3]; 3]` in column-major order
//! (i.e. the first column is stored in memory first, then the second column, etc).
//! This is compatible with `nalgebra::Matrix3`:
//!
//! ```
//! # use rsspice::*;
//! # use approx::assert_relative_eq;
//! use nalgebra as na;
//! # let mut spice = SpiceContext::new();
//! let m = na::Matrix3::new(
//!     0.0, -1.0, 0.0,
//!     0.5,  0.0, 0.0,
//!     0.0,  0.0, 1.0);
//! let mout = na::Matrix3::from(spice.invert(m.as_ref()));
//! assert_relative_eq!(
//!     mout,
//!     na::Matrix3::new(
//!          0.0, 2.0, 0.0,
//!         -1.0, 0.0, 0.0,
//!          0.0, 0.0, 1.0)
//! );
//! ```
//!
//! When reading any FORTRAN example code, note that multidimensional arrays in
//! Rust have indexes in the opposite order to FORTRAN:
//! `M(I, J)` corresponds to `m[j][i]`.
//! But `nalgebra` uses FORTRAN-like indexing: `m[(i, j)]`.
//!
//! # Raw API
//!
//! An alternative version of the API is available in the [`raw`] module.
//! This is a closer match to the FORTRAN API, without the special handling
//! of return values, cells, etc.
//! Each `raw` function reproduces the original FORTRAN API documentation,
//! detailing every input/output argument.
//! You probably shouldn't need to use this API directly,
//! but the documentation is very helpful.
//!
//! # License
//!
//! Much of this crate is derived from the SPICE Toolkit, which is made freely available
//! by NAIF but does not have a clear licensing situation.
//! See [LICENSE.md](https://github.com/zaynar/rsspice/blob/main/LICENSE.md) for details.
//!
//! The non-Toolkit-derived code in this crate is licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](https://github.com/zaynar/rsspice/blob/main/LICENSE-APACHE)
//!    or <http://www.apache.org/licenses/LICENSE-2.0>)
//!  * MIT license
//!    ([LICENSE-MIT](https://github.com/zaynar/rsspice/blob/main/LICENSE-MIT)
//!    or <http://opensource.org/licenses/MIT>)
//!
//! at your option.

mod api;
mod generated;

#[cfg(feature = "tspice")]
mod tspice;

pub use api::*;

/// Collection of reference documents describing the various SPICE subsystems.
///
/// Adapted from <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/FORTRAN/req/index.html>
pub use crate::generated::required_reading;

/// Lower-level SPICELIB API.
///
/// This module provides the complete SPICELIB API, with a very similar structure to the
/// original FORTRAN API, including the full API reference documentation.
///
/// Most applications should use the [`SpiceContext`] methods instead, as they are
/// slightly more convenient.
///
/// The `raw` functions require the caller to allocate space for outputs and pass them as
/// `&mut` references. `SpiceContext` automatically allocates the space and converts output
/// arguments into return values.
///
/// Most `raw` functions still require a `SpiceContext` instance to store their
/// 'global' state.
pub use crate::generated::spicelib::api as raw;
