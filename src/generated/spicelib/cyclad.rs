//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Cycle the elements of a DP array
///
/// Cycle the elements of a double precision array forward
/// or backward.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARRAY      I   Input array.
///  NELT       I   Number of elements.
///  DIR        I   Direction to cycle: 'F' or 'B'.
///  NCYCLE     I   Number of times to cycle.
///  OUT        O   Cycled array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARRAY    is the array to be cycled.
///
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
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the input array after it has been cycled.
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
///  An array is cycled when its contents are shifted forward or
///  backward by one place. An element pushed off one end of the
///  array is brought around to the other end of the array instead
///  of disappearing.
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
/// # Restrictions
///
/// ```text
///  1)  The memory used for the output array must be disjoint from the
///      memory used for the input array.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.3, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 1.0.2, 23-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (WLT) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 2.0.0, 16-JAN-1989 (HAN)
///
///          Error handling was added to detect an invalid value for
///          the cycling direction. If the direction is not recognized
///          the error SPICE(INVALIDDIRECTION) is signaled and the
///          output array is not modified. (The routine used to copy the
///          input array into the output array if the direction was not
///          recognized.)
///
///          The "Exceptions" section was filled out in more detail.
/// ```
pub fn cyclad(
    ctx: &mut SpiceContext,
    array: &[f64],
    nelt: i32,
    dir: char,
    ncycle: i32,
    out: &mut [f64],
) -> crate::Result<()> {
    CYCLAD(
        array,
        nelt,
        &[u8::try_from(dir).unwrap()],
        ncycle,
        out,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CYCLAD ( Cycle the elements of a DP array )
pub fn CYCLAD(
    ARRAY: &[f64],
    NELT: i32,
    DIR: &[u8],
    NCYCLE: i32,
    OUT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let DIR = &DIR[..1];
    let mut OUT = DummyArrayMut::new(OUT, 1..);
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
        CHKIN(b"CYCLAD", ctx)?;
    }

    //
    // Don't even screw around if there are no elements in the array.
    //
    if (NELT < 1) {
        CHKOUT(b"CYCLAD", ctx)?;
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
        CHKOUT(b"CYCLAD", ctx)?;
        return Ok(());
    }

    if (K < 0) {
        K = (K + NELT);
    } else if (K == 0) {
        MOVED(ARRAY.as_slice(), NELT, OUT.as_slice_mut());
        CHKOUT(b"CYCLAD", ctx)?;
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
            OUT[L] = LAST;
            LAST = TEMP;
        }
    }

    CHKOUT(b"CYCLAD", ctx)?;
    Ok(())
}
