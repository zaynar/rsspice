//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Element of a double precision set
///
/// Determine whether an item is an element of a double
/// precision set.
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
///  ITEM       I   Item to be tested.
///  A          I   Set to be tested.
///
///  The function returns .TRUE. if ITEM is an element of set A.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is an item which may or may not be an element of the
///           input set.
///
///  A        is a SPICE set.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if ITEM is a member of the set A, and
///  returns .FALSE. otherwise.
/// ```
///
/// # Particulars
///
/// ```text
///  The LOGICAL functions ELEMC, ELEMD and ELEMI correspond to the
///  set operator IN in the Pascal language.
///
///  This routine uses a binary search to check for the presence in
///  the set of the specified ITEM.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Check if the elements of a list of double precision numbers
///     belong to a given double precision set.
///
///
///     Example code begins here.
///
///
///           PROGRAM ELEMD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           LOGICAL                 ELEMD
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 LBCELL
///           PARAMETER             ( LBCELL = -5 )
///
///           INTEGER                 LISTSZ
///           PARAMETER             ( LISTSZ   = 6   )
///
///           INTEGER                 SETDIM
///           PARAMETER             ( SETDIM   = 8   )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION        A      ( LBCELL:SETDIM )
///           DOUBLE PRECISION        ITEMS  ( LISTSZ        )
///
///           INTEGER                 I
///
///     C
///     C     Set the values of the set and the list of double
///     C     precision numbers.
///     C
///           DATA                  ( A(I), I=1,SETDIM)  /
///          .                            -1.D0, 0.D0, 1.D0,  1.D0,
///          .                             3.D0, 5.D0, 0.D0, -3.D0 /
///
///           DATA                    ITEMS /    6.D0, -1.D0, 0.D0,
///          .                                   2.D0, 3.D0, -2.D0 /
///
///     C
///     C     Validate the set: Initialize the non-empty set, remove
///     C     duplicates and sort the elements.
///     C
///           CALL VALIDD ( SETDIM, SETDIM, A )
///
///     C
///     C     Check if the items in the list belong to the set.
///     C
///           DO I = 1, LISTSZ
///
///              IF ( ELEMD ( ITEMS(I), A ) ) THEN
///
///                 WRITE(*,'(A,F5.1,A)') 'Item ', ITEMS(I),
///          .                          ' is in the set.'
///
///              ELSE
///
///                 WRITE(*,'(A,F5.1,A)') 'Item ', ITEMS(I),
///          .                            ' is NOT in the set.'
///
///              END IF
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Item   6.0 is NOT in the set.
///     Item  -1.0 is in the set.
///     Item   0.0 is in the set.
///     Item   2.0 is NOT in the set.
///     Item   3.0 is in the set.
///     Item  -2.0 is NOT in the set.
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
/// -    SPICELIB Version 1.2.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of
///         the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
/// ```
pub fn elemd(ctx: &mut SpiceContext, item: f64, a: &[f64]) -> crate::Result<bool> {
    let ret = ELEMD(item, a, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure ELEMD ( Element of a double precision set )
pub fn ELEMD(ITEM: f64, A: &[f64], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let A = DummyArray::new(A, LBCELL..);
    let mut ELEMD: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Standard error handling:
    //
    if RETURN(ctx) {
        ELEMD = false;
        return Ok(ELEMD);
    } else {
        CHKIN(b"ELEMD", ctx)?;
    }

    //
    // Just a binary search.
    //
    ELEMD = (BSRCHD(ITEM, CARDD(A.as_slice(), ctx)?, A.subarray(1)) != 0);

    CHKOUT(b"ELEMD", ctx)?;
    Ok(ELEMD)
}
