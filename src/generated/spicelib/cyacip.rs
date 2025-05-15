//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Cycle the elements of a character array
///
/// Cycle the elements of a character array forward or backward
/// in place.
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
///  This routine cycles a character array in place. To cycle
///  an array and store the result in a new array, use CYCLAC.
///
///  An array is cycled when its contents are shifted forward or
///  backward by one place. An element pushed off one end of the
///  array is brought around to the other end of the array instead
///  of disappearing.
/// ```
///
/// # Examples
///
/// ```text
///  Let the integer array A contain the following elements.
///
///     A(1) = 'apple'
///     A(2) = 'bear'
///     A(3) = 'cake'
///     A(4) = 'dragon'
///
///  Cycling A forward once yields the array
///
///     A(1) = 'dragon'
///     A(2) = 'apple'
///     A(3) = 'bear'
///     A(4) = 'cake'
///
///  Cycling A backward once yields the array
///
///     A(1) = 'bear'
///     A(2) = 'cake'
///     A(3) = 'dragon'
///     A(4) = 'apple'
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
pub fn cyacip(
    ctx: &mut SpiceContext,
    nelt: i32,
    dir: char,
    ncycle: i32,
    array: CharArrayMut,
) -> crate::Result<()> {
    CYACIP(
        nelt,
        &[u8::try_from(dir).unwrap()],
        ncycle,
        array,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CYACIP ( Cycle the elements of a character array )
pub fn CYACIP(
    NELT: i32,
    DIR: &[u8],
    NCYCLE: i32,
    ARRAY: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DIR = &DIR[..1];
    let mut ARRAY = DummyCharArrayMut::new(ARRAY, None, 1..);
    let mut OUTLEN: i32 = 0;
    let mut WIDEST: i32 = 0;
    let mut LAST = [b' '; 1];
    let mut TEMP = [b' '; 1];
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
        CHKIN(b"CYACIP", ctx)?;
    }

    //
    // Don't even screw around if there are no elements in the array.
    //
    if (NELT < 1) {
        CHKOUT(b"CYACIP", ctx)?;
        return Ok(());
    }

    //
    // A backward cycle is the same as a forward cycle by the opposite
    // of NCYCLE.  Moreover a cycle by K is the same as a cycle by
    // K + m*N for any integer m.  Thus we compute the value of the
    // minimum forward right cycle that is equivalent to the inputs.
    // If the cycling direction is not recognized, signal an error.
    //
    if (fstr::eq(DIR, b"B") || fstr::eq(DIR, b"b")) {
        K = intrinsics::MOD(-NCYCLE, NELT);
    } else if (fstr::eq(DIR, b"F") || fstr::eq(DIR, b"f")) {
        K = intrinsics::MOD(NCYCLE, NELT);
    } else {
        SETMSG(b"Cycling direction was *.", ctx);
        ERRCH(b"*", DIR, ctx);
        SIGERR(b"SPICE(INVALIDDIRECTION)", ctx)?;
        CHKOUT(b"CYACIP", ctx)?;
        return Ok(());
    }

    if (K < 0) {
        K = (K + NELT);
    } else if (K == 0) {
        CHKOUT(b"CYACIP", ctx)?;
        return Ok(());
    }

    //
    // The algorithm used to cycle arrays is identical to the one used
    // to cycle character strings in CYCLEC. We won't repeat the (rather
    // lengthy) description here.
    //
    // The character version of CYCLAx differs from the other
    // versions in that a single character is cycled at a time. That
    // is, the first trip through the outermost loop cycles the first
    // characters of the array elements; the second trip cycles the
    // second characters; and so on. This allows the same algorithm to
    // be used for all the routines. The local storage required is just
    // a couple of characters.
    //
    // Don't swap the ends of strings if they're just blank padded.
    // And don't overwrite the elements of the output array, if they
    // happen to be shorter than those in the input array.
    //
    OUTLEN = intrinsics::LEN(&ARRAY[1]);
    WIDEST = NBWID(ARRAY.as_arg(), NELT);

    //
    // The greatest common divisor need only be computed once.
    //
    G = GCD(K, NELT);
    M = (NELT / G);

    //
    // To make this a non-character routine, remove all references to C.
    //
    for C in 1..=WIDEST {
        for I in 1..=G {
            L = I;
            fstr::assign(&mut LAST, fstr::substr(ARRAY.get(L), C..=C));

            for J in 1..=M {
                L = (L + K);

                if (L > NELT) {
                    L = (L - NELT);
                }

                fstr::assign(&mut TEMP, fstr::substr(ARRAY.get(L), C..=C));
                fstr::assign(fstr::substr_mut(ARRAY.get_mut(L), C..=C), &LAST);
                fstr::assign(&mut LAST, &TEMP);
            }
        }
    }

    //
    // If needed, pad the output array with blanks.
    //
    if (OUTLEN > WIDEST) {
        for I in 1..=NELT {
            fstr::assign(fstr::substr_mut(ARRAY.get_mut(I), (WIDEST + 1)..), b" ");
        }
    }

    CHKOUT(b"CYACIP", ctx)?;
    Ok(())
}
