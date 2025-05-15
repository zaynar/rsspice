//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Cycle the elements of a character array
///
/// Cycle the elements of a character array forward or backward.
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
///                 'F' or 'f'  to cycle forward.
///                 'B' or 'b'  to cycle backward.
///
///  NCYCLE   is the number of times the elements in the array
///           are to be cycled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the input array after it has been cycled.
///           OUT may overwrite ARRAY.
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
///  Let the integer array A contain the following elements.
///
///        A(1) = 'apple'
///        A(2) = 'bear'
///        A(3) = 'cake'
///        A(4) = 'dragon'
///
///  Cycling A forward once yields the array
///
///        A(1) = 'dragon'
///        A(2) = 'apple'
///        A(3) = 'bear'
///        A(4) = 'cake'
///
///  Cycling A backward once yields the array
///
///        A(1) = 'bear'
///        A(2) = 'cake'
///        A(3) = 'dragon'
///        A(4) = 'apple'
///
///  Cycling by any multiple of the number of elements in the array
///  yields the same array.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The memory used for the output array must be identical to or
///      disjoint from the memory used for the input array.
///
///      That is:
///
///         CALL CYCLAC ( ARRAY, NELT, DIR, NCYCLE, ARRAY )
///
///      will produce correct results, while
///
///         CALL CYCLAC ( ARRAY, NELT-3, DIR, NCYCLE, ARRAY(4) )
///
///      will produce garbage.
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 1.0.2, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
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
pub fn cyclac(
    ctx: &mut SpiceContext,
    array: CharArray,
    nelt: i32,
    dir: char,
    ncycle: i32,
    out: CharArrayMut,
) -> crate::Result<()> {
    CYCLAC(
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

//$Procedure CYCLAC ( Cycle the elements of a character array )
pub fn CYCLAC(
    ARRAY: CharArray,
    NELT: i32,
    DIR: &[u8],
    NCYCLE: i32,
    OUT: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let DIR = &DIR[..1];
    let mut OUT = DummyCharArrayMut::new(OUT, None, 1..);
    let mut OUTLEN: i32 = 0;
    let mut WIDEST: i32 = 0;
    let mut LIMIT: i32 = 0;
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
        CHKIN(b"CYCLAC", ctx)?;
    }

    //
    // Don't even screw around if there are no elements in the array.
    //
    if (NELT < 1) {
        CHKOUT(b"CYCLAC", ctx)?;
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
        CHKOUT(b"CYCLAC", ctx)?;
        return Ok(());
    }

    if (K < 0) {
        K = (K + NELT);
    } else if (K == 0) {
        MOVEC(ARRAY.as_arg(), NELT, OUT.as_arg_mut());
        CHKOUT(b"CYCLAC", ctx)?;
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

    //
    // Don't swap the ends of strings if they're just blank padded.
    // And don't overwrite the elements of the output array, if they
    // happen to be shorter thAn those in the input array.
    //
    OUTLEN = intrinsics::LEN(&OUT[1]);
    WIDEST = NBWID(ARRAY.as_arg(), NELT);
    LIMIT = intrinsics::MIN0(&[OUTLEN, WIDEST]);

    //
    // The greatest common divisor need only be computed once.
    //
    G = GCD(K, NELT);
    M = (NELT / G);

    //
    // To make this a non-character routine, remove all references to C.
    //
    for C in 1..=LIMIT {
        for I in 1..=G {
            L = I;
            fstr::assign(&mut LAST, fstr::substr(ARRAY.get(L), C..=C));

            for J in 1..=M {
                L = (L + K);

                if (L > NELT) {
                    L = (L - NELT);
                }

                fstr::assign(&mut TEMP, fstr::substr(ARRAY.get(L), C..=C));
                fstr::assign(fstr::substr_mut(OUT.get_mut(L), C..=C), &LAST);
                fstr::assign(&mut LAST, &TEMP);
            }
        }
    }

    //
    // If needed, pad the output array with blanks.
    //
    if (OUTLEN > LIMIT) {
        for I in 1..=NELT {
            fstr::assign(fstr::substr_mut(OUT.get_mut(I), (LIMIT + 1)..), b" ");
        }
    }

    CHKOUT(b"CYCLAC", ctx)?;
    Ok(())
}
