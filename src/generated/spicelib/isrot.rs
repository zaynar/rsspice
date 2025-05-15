//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Indicate whether a matrix is a rotation matrix
///
/// Indicate whether a 3x3 matrix is a rotation matrix.
///
/// # Required Reading
///
/// * [ROTATION](crate::required_reading::rotation)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M          I   A matrix to be tested.
///  NTOL       I   Tolerance for the norms of the columns of M.
///  DTOL       I   Tolerance for the determinant of a matrix whose
///                 columns are the unitized columns of M.
///
///  The function returns .TRUE. if and only if M is a rotation matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M        is a 3x3 matrix to be tested.
///
///  NTOL     is the tolerance for the norms of the columns
///           of M.
///
///  DTOL     is the tolerance for the determinant of a matrix
///           whose columns are the unitized columns of M.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if and only if M is found to be a
///  rotation matrix. The criteria that M must meet are:
///
///  1) The norm of each column of M must satisfy the relation
///
///        1.D0 - NTOL  <   || column ||   <  1.D0 + NTOL
///                     -                  -
///
///  2) The determinant of the matrix whose columns are the
///     unitized columns of M must satisfy
///
///        1.D0 - DTOL  <   determinant   <  1.D0 + DTOL
///                     -                 -
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If either of NTOL or DTOL is negative, the error
///      SPICE(VALUEOUTOFRANGE) is signaled. ISROT returns the
///      value .FALSE. in this case.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is an error checking `filter'; its purpose is to
///  detect gross errors, such as uninitialized matrices. Matrices
///  that do not pass the tests used by this routine hardly qualify as
///  rotation matrices. The test criteria can be adjusted by varying
///  the parameters NTOL and DTOL.
///
///  A property of rotation matrices is that their columns form a
///  right-handed, orthonormal basis in 3-dimensional space. The
///  converse is true: all 3x3 matrices with this property are
///  rotation matrices.
///
///  An ordered set of three vectors V1, V2, V3 forms a right-handed,
///  orthonormal basis if and only if
///
///     1)   || V1 ||  =  || V2 ||  =  || V3 ||  =  1
///
///     2)   V3 = V1 x V2. Since V1, V2, and V3 are unit vectors,
///          we also have
///
///          < V3, V1 x V2 > = 1.
///
///          This quantity is the determinant of the matrix whose
///          columns are V1, V2 and V3.
///
///  When finite precision numbers are used, rotation matrices will
///  usually fail to satisfy these criteria exactly. We must use
///  criteria that indicate approximate conformance to the criteria
///  listed above. We choose
///
///     1)   |   || Vi ||  -  1   |   <   NTOL,  i = 1, 2, 3.
///                                   -
///
///     2)   Let
///
///                    Vi
///             Ui = ------ ,   i = 1, 2, 3.
///                  ||Vi||
///
///          Then we require
///
///             | < U3, U1 x U2 > - 1 |  <  DTOL;
///                                      -
///
///          equivalently, letting U be the matrix whose columns
///          are U1, U2, and U3, we insist on
///
///             | det(U) - 1 |  <  DTOL.
///                             _
/// ```
///
/// # Examples
///
/// ```text
///  1)  We have obtained an instrument pointing matrix C from a
///      C-kernel, and we wish to test whether it is in fact a
///      rotation matrix. We can use ISROT to check this:
///
///         C
///         C    Obtain pointing matrix:
///         C
///              CALL CKGP ( INST, TIMEIN, TOL, REF, C, TIMOUT, FOUND )
///
///         C
///         C    Verify that C is a rotation:
///         C
///              IF ( .NOT. ISROT ( C )  ) THEN
///
///                 [ perform exception handling ]
///
///              ELSE
///
///                 [ code for the normal case goes here ]
///
///              END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
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
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 06-SEP-1990 (NJB)
/// ```
pub fn isrot(
    ctx: &mut SpiceContext,
    m: &[[f64; 3]; 3],
    ntol: f64,
    dtol: f64,
) -> crate::Result<bool> {
    let ret = ISROT(m.as_flattened(), ntol, dtol, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure ISROT ( Indicate whether a matrix is a rotation matrix )
pub fn ISROT(M: &[f64], NTOL: f64, DTOL: f64, ctx: &mut Context) -> f2rust_std::Result<bool> {
    let M = DummyArray2D::new(M, 1..=3, 1..=3);
    let mut ISROT: bool = false;
    let mut D: f64 = 0.0;
    let mut N1: f64 = 0.0;
    let mut N2: f64 = 0.0;
    let mut N3: f64 = 0.0;
    let mut UNIT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DETOK: bool = false;
    let mut NORMOK: bool = false;

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
        ISROT = false;
        return Ok(ISROT);
    } else {
        CHKIN(b"ISROT", ctx)?;
    }

    //
    // Tolerances must be non-negative.
    //
    if (NTOL < 0.0) {
        ISROT = false;

        SETMSG(b"NTOL should be non-negative; it is #.", ctx);
        ERRDP(b"#", NTOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ISROT", ctx)?;
        return Ok(ISROT);
    } else if (DTOL < 0.0) {
        ISROT = false;

        SETMSG(b"DTOL should be non-negative; it is #.", ctx);
        ERRDP(b"#", DTOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ISROT", ctx)?;
        return Ok(ISROT);
    }

    //
    // The columns of M must resemble unit vectors.  If the norms are
    // outside of the allowed range, M is not a rotation matrix.
    //
    // Also, the columns of M are required to be pretty nearly
    // orthogonal.  The discrepancy is gauged by taking the determinant
    // of the matrix UNIT, computed below, whose columns are the
    // unitized columns of M.
    //
    UNORM(M.subarray([1, 1]), UNIT.subarray_mut([1, 1]), &mut N1);
    UNORM(M.subarray([1, 2]), UNIT.subarray_mut([1, 2]), &mut N2);
    UNORM(M.subarray([1, 3]), UNIT.subarray_mut([1, 3]), &mut N3);

    D = DET(UNIT.as_slice());

    NORMOK = (((N1 == BRCKTD(N1, (1.0 - NTOL), (1.0 + NTOL)))
        && (N2 == BRCKTD(N2, (1.0 - NTOL), (1.0 + NTOL))))
        && (N3 == BRCKTD(N3, (1.0 - NTOL), (1.0 + NTOL))));

    DETOK = (D == BRCKTD(D, (1.0 - DTOL), (1.0 + DTOL)));

    if (NORMOK && DETOK) {
        ISROT = true;
    } else {
        ISROT = false;
    }

    CHKOUT(b"ISROT", ctx)?;
    Ok(ISROT)
}
