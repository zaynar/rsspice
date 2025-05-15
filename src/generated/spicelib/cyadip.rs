//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Cycle the elements of a DP array, in place
///
/// Cycle the elements of a double precision array forward
/// or backward in place.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NELT       I   Number of elements.
///  DIR        I   Direction to cycle: 'F' or 'B'.
///  NCYCLE     I   Number of times to cycle.
///  ARRAY     I-O  Array to be cycled/cycled array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NELT     is the number of elements in the input array.
///
///  DIR      is the direction in which the elements in the
///           array are to be cycled.
///
///              'F' or 'f'  to cycle forward.
///              'B' or 'b'  to cycle backward.
///
///  NCYCLE   is the number of times the elements in the array
///           are to be cycled.
///
///  ARRAY    is the array to be cycled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRAY    is the input array after it has been cycled.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of DIR is not recognized, the error
///      SPICE(INVALIDDIRECTION) is signaled.
///
///  2)  If NELT is less than 1, the output array is not modified.
///
///  3)  If NCYCLE is negative, the array is cycled NCYCLE times in
///      the opposite direction of DIR.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine cycles a double precision array in place. To cycle
///  an array and store the result in a new array, use CYCLAD.
///
///  An array is cycled when its contents are shifted forward or
///  backward by one place. An element pushed off one end of the array
///  is brought around to the other end of the array instead of
///  disappearing.
/// ```
///
/// # Examples
///
/// ```text
///  Let the double precision A contain the following elements.
///
///     A(1) = 1.D0
///     A(2) = 2.D0
///     A(3) = 3.D0
///     A(4) = 4.D0
///
///  Cycling A forward once yields the array
///
///     A(1) = 4.D0
///     A(2) = 1.D0
///     A(3) = 2.D0
///     A(4) = 3.D0
///
///  Cycling A backward once yields the array
///
///     A(1) = 2.D0
///     A(2) = 3.D0
///     A(3) = 4.D0
///     A(4) = 1.D0
///
///  Cycling by any multiple of the number of elements in the array
///  yields the same array.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 09-SEP-2005 (NJB) (HAN) (WLT) (IMU)
/// ```
pub fn cyadip(
    ctx: &mut SpiceContext,
    nelt: i32,
    dir: char,
    ncycle: i32,
    array: &mut [f64],
) -> crate::Result<()> {
    CYADIP(
        nelt,
        &[u8::try_from(dir).unwrap()],
        ncycle,
        array,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CYADIP ( Cycle the elements of a DP array, in place )
pub fn CYADIP(
    NELT: i32,
    DIR: &[u8],
    NCYCLE: i32,
    ARRAY: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DIR = &DIR[..1];
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);
    let mut LAST: f64 = 0.0;
    let mut TEMP: f64 = 0.0;
    let mut G: i32 = 0;
    let mut K: i32 = 0;
    let mut L: i32 = 0;
    let mut M: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CYADIP", ctx)?;
    }

    //
    // Don't even screw around if there are no elements in the array.
    //
    if (NELT < 1) {
        CHKOUT(b"CYADIP", ctx)?;
        return Ok(());
    }

    //
    // A backward cycle is the same as a forward cycle by the opposite
    // of NCYCLE.  Moreover a cycle by K is the same as a cycle by
    // K + m*N for any integer m.  Thus we compute the value of the
    // minimum forward right cycle that is equivalent to the inputs.
    //
    if (fstr::eq(DIR, b"B") || fstr::eq(DIR, b"b")) {
        K = intrinsics::MOD(-NCYCLE, NELT);
    } else if (fstr::eq(DIR, b"F") || fstr::eq(DIR, b"F")) {
        K = intrinsics::MOD(NCYCLE, NELT);
    } else {
        SETMSG(b"Cycling direction was *.", ctx);
        ERRCH(b"*", DIR, ctx);
        SIGERR(b"SPICE(INVALIDDIRECTION)", ctx)?;
        CHKOUT(b"CYADIP", ctx)?;
        return Ok(());
    }

    if (K < 0) {
        K = (K + NELT);
    } else if (K == 0) {
        CHKOUT(b"CYADIP", ctx)?;
        return Ok(());
    }

    //
    // The algorithm used to cycle arrays is identical to the one used
    // to cycle character strings in CYCLEC. We won't repeat the (rather
    // lengthy) description here.
    //
    G = GCD(K, NELT);
    M = (NELT / G);

    for I in 1..=G {
        L = I;
        LAST = ARRAY[L];

        for J in 1..=M {
            L = (L + K);

            if (L > NELT) {
                L = (L - NELT);
            }

            TEMP = ARRAY[L];
            ARRAY[L] = LAST;
            LAST = TEMP;
        }
    }

    CHKOUT(b"CYADIP", ctx)?;
    Ok(())
}
