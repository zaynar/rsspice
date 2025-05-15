//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// The ordinal position of an element in a set
///
/// Return the ordinal position of a given item in a set. If the
/// item does not appear in the set, return zero.
///
/// # Required Reading
///
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ITEM       I   An item to locate within a set.
///  SET        I   A set to search for a given item.
///
///  The function returns the ordinal position of ITEM within the SET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is a string to be located within a character set.
///           Trailing blanks are not significant in the comparison.
///
///  SET      is a properly validated SPICE set that is to be
///           searched for the occurrence of ITEM. Trailing blanks are
///           not significant in the comparison.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the ordinal position of ITEM within SET.
///  Ordinal positions range from 1 to N, where N is the cardinality
///  of the set.
///
///  If ITEM is not an element of SET, the function returns 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input set has invalid cardinality, an error is signaled
///      by a routine in the call tree of this routine. ORDC returns an
///      unspecified value in this case.
///
///  2)  If the input set has invalid size, an error is signaled by a
///      routine in the call tree of this routine. ORDC returns an
///      unspecified value in this case.
/// ```
///
/// # Particulars
///
/// ```text
///  A natural ordering can be imposed upon the elements of any
///  SPICE set, be it INTEGER, CHARACTER or DOUBLE PRECISION. For
///  character strings the ASCII collating sequence serves as the
///  ordering relation, for DOUBLE PRECISION and INTEGER variables
///  the arithmetic ordering is used.
///
///  Given any element of a set, its location within this ordered
///  sequence of elements is called its ordinal position within
///  the set.
///
///  For illustrative purposes suppose that SET represents the set
///
///           { 8, 1, 2, 9, 7, 4, 10 }
///
///  The ordinal position of:     8 is 5
///                               1 is 1
///                               2 is 2
///                               9 is 6
///                               7 is 4
///                               4 is 3
///                              10 is 7
///
///  Given an item of the SET, this routine returns its ordinal
///  position. If the item is not in the set, this function returns
///  a value of 0.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you wished to find the relative position of a value
///  in a large list of values stored within an array. Say we want
///  to know the relative position of item I of ARRAY withing the
///  set of values represented in ARRAY.
///
///  The following sequence of subroutine calls would allow you
///  determine the relative position of the value ARRAY(I).
///
///     INTEGER               N
///     PARAMETER           ( N = something useful )
///
///     CHARACTER*(*)         ARRAY (         N )
///     CHARACTER*(*)         SET   ( LBCELL: N )
///     INTEGER               I
///
///     INTEGER               NVALID
///     INTEGER               POSITION
///
///
///  set the value of NVALID to be the number of valid elements in the
///  array ARRAY
///
///     CALL MOVEC  ( ARRAY, N,      SET(1) )
///     CALL VALIDC ( N,     NVALID, SET    )
///
///     POSITION = ORDC ( ARRAY(I), SET )
///
///  POSITION now contains the ordinal position of ARRAY(I) within the
///  values represented in the array.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SET must be a validated or empty set.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Extended
///         $Detailed_Input, $Detailed_Output and $Exceptions sections.
///
///         Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of the
///         function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
/// ```
pub fn ordc(ctx: &mut SpiceContext, item: &str, set: CharArray) -> crate::Result<i32> {
    let ret = ORDC(item.as_bytes(), set, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure ORDC ( The ordinal position of an element in a set )
pub fn ORDC(ITEM: &[u8], SET: CharArray, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let SET = DummyCharArray::new(SET, None, LBCELL..);
    let mut ORDC: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Standard error handling:
    //
    if RETURN(ctx) {
        ORDC = 0;
        return Ok(ORDC);
    } else {
        CHKIN(b"ORDC", ctx)?;
    }

    //
    // Given the structure of sets, there's not much to do.
    //
    ORDC = BSRCHC(ITEM, CARDC(SET.as_arg(), ctx)?, SET.subarray(1));

    CHKOUT(b"ORDC", ctx)?;
    Ok(ORDC)
}
