mod cell;
mod charvec;
mod errors;

pub use cell::*;
pub use charvec::*;
pub use errors::*;

use crate::raw;
use f2rust_std::io::FileManager;

/// SPICELIB API.
///
/// This provides access to most SPICELIB APIs.
/// It also stores all the 'global' state -- you can use multiple `SpiceContext`s concurrently
/// in separate threads.
///
/// See the [crate documentation](crate) for details.
pub struct SpiceContext<'a> {
    ctx: f2rust_std::Context<'a>,
}

impl Default for SpiceContext<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SpiceContext<'a> {
    /// Constructs a new `SpiceContext`.
    pub fn new() -> Self {
        let ctx = f2rust_std::Context::new();
        let mut r = Self { ctx };
        r.setup_errors();
        r
    }

    /// Constructs a new `SpiceContext`, where all filesystem access will
    /// go through `FileManager`. This is useful for tests and WebAssembly.
    pub fn with_vfs<F: FileManager<'a> + 'a>(file_manager: F) -> Self {
        let ctx = f2rust_std::Context::with_file_manager(file_manager);
        let mut r = Self { ctx };
        r.setup_errors();
        r
    }

    /// Prepare SPICELIB's error handling system.
    fn setup_errors(&mut self) {
        // Don't print errors to stdout
        let mut list = "NONE".to_owned();
        raw::errprt(self, "SET", &mut list).unwrap();

        // Return errors so we can handle them nicely
        let mut action = "RETURN".to_owned();
        raw::erract(self, "SET", &mut action).unwrap();
    }

    /// Called after each fallible SPICELIB function, to detect and convert errors.
    pub(crate) fn handle_errors(&mut self) -> Result<()> {
        if raw::failed(self) {
            // Get the error messages
            let mut short = " ".repeat(raw::inc::errhnd::SMSGLN as usize);
            let mut long = " ".repeat(raw::inc::errhnd::LMSGLN as usize);
            raw::getsms(self, &mut short);
            raw::getlms(self, &mut long);

            // Reset error state, so the user can continue after recoverable errors
            raw::reset(self);

            // Return the error
            let short = short.trim_ascii_end();
            let long = long.trim_ascii_end();
            Err(Error::from_spice(short, long))
        } else {
            Ok(())
        }
    }

    /// Returns the underlying `f2rust_std::Context`.
    /// This is an implementation detail that users should never need,
    /// except possibly with some callback function pointers.
    pub(crate) fn raw_context(&mut self) -> &mut f2rust_std::Context<'a> {
        &mut self.ctx
    }
}

#[cfg(test)]
mod tests {
    use crate::SpiceContext;
    use crate::raw;
    use approx::assert_relative_eq;
    use nalgebra as na;

    #[test]
    fn vadd() {
        let v1 = [1., 2., 3.];
        let v2 = [10., 20., 30.];
        let mut vout = [0.; 3];
        raw::vadd(&v1, &v2, &mut vout);
        assert_eq!(vout, [11., 22., 33.]);
    }

    #[test]
    fn vrotv() {
        let v = na::Vector3::new(1.0, 2.0, 3.0);
        let mut r = na::Vector3::zeros();
        raw::vrotv(
            v.as_ref(),
            &[0.0, 0.0, 1.0],
            std::f64::consts::FRAC_PI_2,
            r.as_mut(),
        );
        assert_relative_eq!(r, na::Vector3::new(-2.0, 1.0, 3.0));
    }

    #[test]
    fn invert() {
        let m = na::Matrix3::new(0.0, -1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut mout = na::Matrix3::zeros();
        raw::invert(m.as_ref(), mout.as_mut());

        assert_relative_eq!(
            mout,
            na::Matrix3::new(0.0, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn okay_utf8() {
        let mut ctx = SpiceContext::new();

        let mut s = String::from("abcd\u{1234}");
        raw::shiftc(&mut ctx, &s.clone(), 'L', 1, 'x', &mut s).unwrap();
        assert_eq!(s, "bcd\u{1234}x");
        raw::shiftc(&mut ctx, &s.clone(), 'L', 3, 'x', &mut s).unwrap();
        assert_eq!(s, "\u{1234}xxxx");
    }

    // #[test]
    // #[should_panic]
    // fn bad_utf8() {
    //     let mut ctx = SpiceContext::new();
    //
    //     let mut s = String::from("abcd\u{1234}");
    //     raw::shiftc(&mut ctx, &s.clone(), 'L', 5, 'x', &mut s).unwrap();
    // }
}
