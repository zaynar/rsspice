//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    BOUND: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BOUND: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self { BOUND, FIRST }
    }
}

/// Invert nearly orthogonal matrices
///
/// Construct the inverse of a 3x3 matrix with orthogonal columns and
/// non-zero column norms using a numerically stable algorithm. The
/// rows of the output matrix are the columns of the input matrix
/// divided by the length squared of the corresponding columns.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M          I   A 3x3 matrix.
///  MIT        O   M after transposition and scaling of rows.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M        is a 3x3 matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MIT      is the matrix obtained by transposing M and dividing
///           the rows by squares of their norms.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If any of the columns of M have zero length, the error
///      SPICE(ZEROLENGTHCOLUMN) is signaled.
///
///  2)  If any column is too short to allow computation of the
///      reciprocal of its length without causing a floating
///      point overflow, the error SPICE(COLUMNTOOSMALL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Suppose that M is the matrix
///
///         .-                      -.
///         |   A*u    B*v     C*w   |
///         |      1      1       1  |
///         |                        |
///         |   A*u    B*v     C*w   |
///         |      2      2       2  |
///         |                        |
///         |   A*u    B*v     C*w   |
///         |      3      3       3  |
///         `-                      -'
///
///  where the vectors (u , u , u ),  (v , v , v ),  and (w , w , w )
///                      1   2   3      1   2   3          1   2   3
///  are unit vectors. This routine produces the matrix:
///
///
///         .-                      -.
///         |   a*u    a*u     a*u   |
///         |      1      2       3  |
///         |                        |
///         |   b*v    b*v     b*v   |
///         |      1      2       3  |
///         |                        |
///         |   c*w    c*w     c*w   |
///         |      1      2       3  |
///         `-                      -'
///
///  where a = 1/A, b = 1/B, and c = 1/C.
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
///  1) Given a double precision 3x3 matrix with mutually orthogonal
///     rows of arbitrary length, compute its inverse. Check that the
///     original matrix times the computed inverse produces the
///     identity matrix.
///
///     Example code begins here.
///
///
///           PROGRAM INVORT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      IMAT ( 3, 3 )
///           DOUBLE PRECISION      M    ( 3, 3 )
///           DOUBLE PRECISION      MOUT ( 3, 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define a matrix to invert.
///     C
///           DATA                  M  /  0.D0,  0.5D0, 0.D0,
///          .                           -1.D0,  0.D0,  0.D0,
///          .                            0.D0,  0.D0,  1.D0 /
///
///           WRITE(*,*) 'Original Matrix:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( M(I,J), J=1,3 )
///
///           END DO
///     C
///     C     Invert the matrix, then output.
///     C
///           CALL INVORT ( M, MOUT )
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Inverse Matrix:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( MOUT(I,J), J=1,3 )
///
///           END DO
///
///     C
///     C     Check the M times MOUT produces the identity matrix.
///     C
///           CALL MXM ( M, MOUT, IMAT )
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Original times inverse:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( IMAT(I,J), J=1,3 )
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
///      Original Matrix:
///            0.0000000      -1.0000000       0.0000000
///            0.5000000       0.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
///
///      Inverse Matrix:
///            0.0000000       2.0000000       0.0000000
///           -1.0000000       0.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
///
///      Original times inverse:
///            1.0000000       0.0000000       0.0000000
///            0.0000000       1.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of argument MIT in $Brief_I/O table. Extended $Abstract
///         section.
///
///         Added complete code example to $Examples section.
///
/// -    SPICELIB Version 1.1.1, 14-NOV-2013 (EDW)
///
///         Edit to $Abstract. Eliminated unneeded $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 02-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL call.
///
/// -    SPICELIB Version 1.0.0, 02-JAN-2002 (WLT)
/// ```
pub fn invort(
    ctx: &mut SpiceContext,
    m: &[[f64; 3]; 3],
    mit: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    INVORT(m.as_flattened(), mit.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure INVORT ( Invert nearly orthogonal matrices )
pub fn INVORT(M: &[f64], MIT: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let M = DummyArray2D::new(M, 1..=3, 1..=3);
    let mut MIT = DummyArrayMut2D::new(MIT, 1..=3, 1..=3);
    let mut LENGTH: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut TEMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Use discovery check-in.
    //

    //
    // The first time through, get a copy of DPMAX.
    //
    if save.FIRST {
        save.BOUND = DPMAX();
        save.FIRST = false;
    }

    //
    // For each column, construct a scaled copy. However, make sure
    // everything is do-able before trying something.
    //
    for I in 1..=3 {
        UNORM(M.subarray([1, I]), TEMP.subarray_mut([1, I]), &mut LENGTH);

        if (LENGTH == 0.0) {
            CHKIN(b"INVORT", ctx)?;
            SETMSG(b"Column # of the input matrix has a norm of zero. ", ctx);
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(ZEROLENGTHCOLUMN)", ctx)?;
            CHKOUT(b"INVORT", ctx)?;
            return Ok(());
        }

        //
        // Make sure we can actually rescale the rows.
        //
        if (LENGTH < 1.0) {
            if ((LENGTH * save.BOUND) < 1.0) {
                CHKIN(b"INVORT", ctx)?;
                SETMSG(b"The length of column # is #. This number cannot be inverted.  For this reason, the scaled transpose of the input matrix cannot be formed. ", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", LENGTH, ctx);
                SIGERR(b"SPICE(COLUMNTOOSMALL)", ctx)?;
                CHKOUT(b"INVORT", ctx)?;
                return Ok(());
            }
        }

        SCALE = (1.0 / LENGTH);

        VSCLIP(SCALE, TEMP.subarray_mut([1, I]));
    }

    //
    // If we make it this far, we just need to transpose TEMP into MIT.
    //
    XPOSE(TEMP.as_slice(), MIT.as_slice_mut());
    Ok(())
}
