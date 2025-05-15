//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Pack a double precision array
///
/// Pack the contents of a double precision array. That is,
/// take a set of arbitrarily spaced elements from an input
/// array, and make them adjacent elements in an output array.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input array.
///  PACK       I   Indices of elements to be packed.
///  NPACK      I   Number of indices.
///  MAXOUT     I   Maximum number of elements in the output array.
///  NOUT       O   Number of elements in the output array.
///  OUT        O   Output array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is the input array.
///
///  PACK     is the set of elements to be packed into the output
///           array. PACK(i) is the index of the element in the
///           input array that is to become the i'th element of the
///           output array.
///
///  NPACK    is the number of elements to be packed into the
///           output array.
///
///  MAXOUT   is the maximum number of elements to be packed into
///           the output array. If NPACK is larger than MAXOUT, the
///           extra items are ignored.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NOUT     is the number of elements in the output array.
///
///  OUT      is the output array. This array contains up to
///           MAXOUT elements from the input array, located
///           in the first NOUT elements of the array.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an element in the PACK array is less than 1, the error
///      SPICE(INVALIDINDEX) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The indicated elements are moved from their current locations
///  in the input array to consecutive positions in the output array.
///
///     OUT(   1) = IN(PACK(   1))
///     OUT(   2) = IN(PACK(   2))
///          .
///          .
///    OUT(NOUT) = IN(PACK(NOUT))
///
///  NOUT is either NPACK or MAXOUT, whichever is smaller.
/// ```
///
/// # Examples
///
/// ```text
///  The most common use for this routine is to remove unwanted items
///  from an array or set of arrays. For example, suppose that the
///  arrays NAME, CODE, RADIUS and MASS contain the names, NAIF
///  integer ID codes, radii, and masses of a set of NSAT satellites.
///  Suppose further that the user selects a subset of the original
///  set of satellites from a menu of some sort. Let the indices of
///  these satellites be the NSEL elements of the array SEL. The
///  following sequence would remove the names, codes, etc., of the
///  unselected satellites from the arrays.
///
///     CALL PACKAC ( NAME,   SEL, NSEL, NSAT, NOUT, NAME2   )
///     CALL PACKAI ( CODE,   SEL, NSEL, NSAT, NOUT, CODE2   )
///     CALL PACKAD ( RADIUS, SEL, NSEL, NSAT, NOUT, RADIUS2 )
///     CALL PACKAD ( MASS,   SEL, NSEL, NSAT, NOUT, MASS2   )
///
///  In the example above, suppose that NAME and PACK contain
///  the following:
///
///     NAME = 'MIMAS'          PACK = 2, 4, 6, 7
///            'ENCELADUS'
///            'TETHYS'
///            'DIONE'
///            'RHEA'
///            'TITAN'
///            'HYPERION'
///            'IAPETUS'
///            'PHOEBE'
///
///  Then, following the call to PACKAC, NOUT and NAME2 contain
///  the following:
///
///     NOUT = 4                 NAME2 = 'ENCELADUS'
///                                      'DIONE'
///                                      'TITAN'
///                                      'HYPERION'
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
/// -    SPICELIB Version 1.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 2.0.0, 4-JAN-1989 (HAN)
///
///          Error handling was added to detect array indices that are
///          out of bound. If any element contained in the PACK array is
///          less than one, an error is signaled, and the output array is
///          not packed.
/// ```
pub fn packad(
    ctx: &mut SpiceContext,
    in_: &[f64],
    pack: &[i32],
    npack: i32,
    maxout: i32,
    nout: &mut i32,
    out: &mut [f64],
) -> crate::Result<()> {
    PACKAD(in_, pack, npack, maxout, nout, out, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PACKAD ( Pack a double precision array )
pub fn PACKAD(
    IN: &[f64],
    PACK: &[i32],
    NPACK: i32,
    MAXOUT: i32,
    NOUT: &mut i32,
    OUT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let IN = DummyArray::new(IN, 1..);
    let PACK = DummyArray::new(PACK, 1..);
    let mut OUT = DummyArrayMut::new(OUT, 1..);

    //
    // Spicelib functions
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
        CHKIN(b"PACKAD", ctx)?;
    }

    //
    // First, determine how many items to transfer.
    //
    *NOUT = intrinsics::MIN0(&[NPACK, MAXOUT]);

    //
    // Check to see if PACK contains valid array indices.
    //
    for I in 1..=*NOUT {
        if (PACK[I] < 1) {
            SETMSG(b"Element number * contains index *.", ctx);
            ERRINT(b"*", I, ctx);
            ERRINT(b"*", PACK[I], ctx);
            SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
            CHKOUT(b"PACKAD", ctx)?;
            return Ok(());
        }
    }

    //
    // Transfer them. Just like it says in the header.
    //
    for I in 1..=*NOUT {
        OUT[I] = IN[PACK[I]];
    }

    CHKOUT(b"PACKAD", ctx)?;
    Ok(())
}
