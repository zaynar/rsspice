//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Insert at location in a character array
///
/// Insert one or more elements into a character array at the
/// indicated location.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ELTS       I   Elements to be inserted.
///  NE         I   Number of elements to be inserted.
///  LOC        I   Location of the first inserted element.
///  ARRAY     I-O  Input/output array.
///  NA        I-O  Number of elements in the input/output array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ELTS     contains one or more elements which are to be
///           inserted into the input array.
///
///  NE       is the number of elements to be inserted.
///
///  LOC      is the location in the array at which the first
///           element of ELTS is to be inserted. LOC must be
///           within the interval [1, NA+1]. To append to
///           ARRAY, set LOC equal to NA+1.
///
///  ARRAY    on input, is the original array.
///
///  NA       on input, is the number of elements in ARRAY.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARRAY    on output, is the original array with the elements
///           of ELT inserted into positions LOC through LOC+NE-1.
///           The original elements in these positions are moved
///           back to make room for the inserted elements. If the
///           new elements are longer than the declared lengths
///           of the elements of ARRAY, the new elements are
///           truncated on the right.
///
///  NA       on output, is the number of elements in ARRAY.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The dimension of the array is set equal to zero if its
///      input value is less than one.
///
///  2)  If LOC is not in the interval [1, NA+1], the error
///      SPICE(INVALIDINDEX) is signaled.
///
///  3)  If the number of elements to be inserted is less than one,
///      the array is not modified.
/// ```
///
/// # Particulars
///
/// ```text
///  The elements in positions LOC through LOC+NE-1 are moved back
///  by NE spaces to make room for the new elements, which are then
///  inserted into the vacated spaces.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///        ELTS(1) = 'very'       NA = 4      ARRAY(1) =  'I'
///        ELTS(2) = 'big'                    ARRAY(2) =  'saw'
///        ELTS(3) = 'brown'                  ARRAY(3) =  'a'
///                                           ARRAY(4) =  'dog'
///
///  Then the call
///
///        CALL INSLAC ( ELTS, 3, 4, ARRAY, NA )
///
///  yields the following result:
///
///        NA = 7      ARRAY(1) = 'I'
///                    ARRAY(2) = 'saw'
///                    ARRAY(3) = 'a'
///                    ARRAY(4) = 'very'
///                    ARRAY(5) = 'big'
///                    ARRAY(6) = 'brown'
///                    ARRAY(7) = 'dog'
///
///
///  The following calls to INSLAC signal errors.
///
///  CALL INSLAC ( ELTS, 3, -1, ARRAY, NA )
///  CALL INSLAC ( ELTS, 3,  6, ARRAY, NA )
///  CALL INSLAC ( ELTS, 3,  2, ARRAY, -1 )
///  CALL INSLAC ( ELTS, 3, -1, ARRAY, -1 )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The array must be large enough to contain both the original
///      and the inserted elements.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 05-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 2.0.0, 30-DEC-1988 (HAN)
///
///          If the location at which the elements are to be inserted is
///          not in the interval [1, NA+1], an error is signaled.
///          Locations not within that interval refer to non-existent
///          array elements. (To append to the array, the location
///          should be equal to NA+1.)
///
///          A negative dimension bug was fixed. The results of the
///          old version were unpredictable if the input array dimension
///          was negative. To avoid this problem the maximum of zero and
///          the input dimension becomes the dimension used by the
///          the routine. In this case, the only valid location at which
///          to insert is 1. If it is not 1, an error is signaled
///          when the location is checked.
/// ```
pub fn inslac(
    ctx: &mut SpiceContext,
    elts: CharArray,
    ne: i32,
    loc: i32,
    array: CharArrayMut,
    na: &mut i32,
) -> crate::Result<()> {
    INSLAC(elts, ne, loc, array, na, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure INSLAC ( Insert at location in a character array )
pub fn INSLAC(
    ELTS: CharArray,
    NE: i32,
    LOC: i32,
    ARRAY: CharArrayMut,
    NA: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ELTS = DummyCharArray::new(ELTS, None, 1..);
    let mut ARRAY = DummyCharArrayMut::new(ARRAY, None, 1..);
    let mut SIZE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
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
        CHKIN(b"INSLAC", ctx)?;
    }

    //
    // Check the dimension of the array.
    //
    SIZE = intrinsics::MAX0(&[0, *NA]);

    //
    // Make sure the location at which the elements are to be inserted
    // is not out of range. If it is, signal an error and bail out.
    //

    if ((LOC < 1) || (LOC > (SIZE + 1))) {
        SETMSG(b"Location was *.", ctx);
        ERRINT(b"*", LOC, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"INSLAC", ctx)?;
        return Ok(());
    }

    //
    // If the number of elements to be inserted is greater than zero,
    // insert them. If not, do not modify the array.
    //
    if (NE > 0) {
        //
        // Move the trailing elements back to make room for the new ones.
        //
        for I in intrinsics::range(SIZE, LOC, -1) {
            let val = ARRAY.get(I).to_vec();
            fstr::assign(ARRAY.get_mut((I + NE)), &val);
        }

        //
        // Now put the new elements in the vacated spaces.
        //
        for I in 1..=NE {
            fstr::assign(ARRAY.get_mut(((LOC + I) - 1)), ELTS.get(I));
        }

        //
        // Update the number of elements in the array.
        //
        *NA = (SIZE + NE);
    }

    CHKOUT(b"INSLAC", ctx)?;
    Ok(())
}
