//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Transpose a matrix by blocks
///
/// Transpose the square blocks within a matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BMAT       I   a matrix composed of square block submatrices
///  NROW       I   the number of rows in the matrix BMAT
///  NCOL       I   the number of columns in the matrix BMAT
///  BSIZE      I   the size of the square blocks in BMAT
///  BTMAT      O   the result of transposing the blocks of BMAT
/// ```
///
/// # Detailed Input
///
/// ```text
///  NROW     is the number of rows in the input matrix.
///
///  NCOL     is the number of columns in the input matrix.
///
///  BSIZE    is the number of rows and columns in each block
///           of the input matrix.
///
///  BMAT     is a block structured matrix. In other words
///           it looks like:
///
///
///              -                                -
///             |      :      :       :     :      |
///             |  B   :  B   :   B   :     :  B   |
///             |   11 :   12 :    13 : ... :   1C |
///             |......:......:.......:     :......|
///             |      :      :       :     :      |
///             |  B   :  B   :   B   :     :  B   |
///             |   21 :   22 :    23 : ... :   2C |
///             |......:......:.......:     :......|
///             |      .                           |
///             |      .                           |
///             |      .                           |
///             |......................     .......|
///             |      :      :       :     :      |
///             |  B   :  B   :   B   :     :  B   |
///             |   R1 :   R2 :    R3 : ... :   RC |
///             |......:......:.......:     :......|
///              -                                -
///
///           where each B   is a square matrix of BSIZE rows and
///                       ij
///           and columns.
/// ```
///
/// # Detailed Output
///
/// ```text
///  BTMAT    is the matrix obtained from BMAT when each of its
///           blocks is transposed. Given the description of
///           BMAT above, BTMAT looks like:
///
///
///               -                                -
///              |   t  :   t  :    t  :     :   t  |
///              |  B   :  B   :   B   :     :  B   |
///              |   11 :   12 :    13 : ... :   1C |
///              |......:......:.......:     :......|
///              |      :      :       :     :      |
///              |   t  :   t  :    t  :     :   t  |
///              |  B   :  B   :   B   :     :  B   |
///              |   21 :   22 :    23 : ... :   2C |
///              |......:......:.......:     :......|
///              |      .                           |
///              |      .                           |
///              |      .                           |
///              |......................     .......|
///              |      :      :       :     :      |
///              |   t  :   t  :    t  :     :   t  |
///              |  B   :  B   :   B   :     :  B   |
///              |   R1 :   R2 :    R3 : ... :   RC |
///              |......:......:.......:     :......|
///               -                                -
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of rows input is not positive, the error
///      SPICE(BADROWCOUNT) is signaled.
///
///  2)  If the number of columns input is not positive, the error
///      SPICE(BADCOLUMNCOUNT) is signaled.
///
///  3)  If the block size input is not positive, the error
///      SPICE(BADBLOCKSIZE) is signaled.
///
///  4)  If BMAT cannot be partitioned into an integer number of
///      blocks, the error SPICE(BLOCKSNOTEVEN) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine transposes the blocks of a block structured
///  matrix. This operation is valuable, as it is a means
///  for computing the inverse of a state transformation matrix
///  (see the example below).
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates how you would convert
///  a state relative to earth-fixed coordinates to J2000 coordinates.
///
///  C
///  C     We want to state earthfixed coordinates (399) to J2000
///  C     coordinates
///  C
///        BODY = 399
///        REF  = 'J2000'
///
///  C
///  C     Get the 6 by 6 state transformation matrix from J2000
///  C     coordinates to earthfixed coordinates.
///  C
///        CALL TISBOD ( REF, BODY, ET, TISPM )
///
///  C
///  C     The inverse of TISPM can be obtained by transposing the
///  C     3 by 3 blocks of the 6 by 6 matrix TISPM.
///  C
///        CALL XPOSBL ( TISPM, 6, 6, 3, TSPMI )
///
///
///  C
///  C     Now transform the earthfixed state (ESTATE) to the
///  C     inertial state (ISTATE).
///  C
///        CALL MXVG   ( TSPMI, ESTATE, 6, 6, ISTATE )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 16-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Updated
///         $Exceptions section.
///
/// -    SPICELIB Version 1.0.2, 22-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-NOV-1990 (WLT)
/// ```
pub fn xposbl(
    ctx: &mut SpiceContext,
    bmat: &[f64],
    nrow: i32,
    ncol: i32,
    bsize: i32,
    btmat: &mut [f64],
) -> crate::Result<()> {
    XPOSBL(bmat, nrow, ncol, bsize, btmat, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure XPOSBL ( Transpose a matrix by blocks    )
pub fn XPOSBL(
    BMAT: &[f64],
    NROW: i32,
    NCOL: i32,
    BSIZE: i32,
    BTMAT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BMAT = DummyArray2D::new(BMAT, 1..=NROW, 1..=NCOL);
    let mut BTMAT = DummyArrayMut2D::new(BTMAT, 1..=NROW, 1..=NCOL);
    let mut TEMP: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Ok. Here's what's going to happen.
    //
    // The matrix has the form:
    //
    //   -                                -
    //  |      :      :       :     :      |
    //  |  B   :  B   :   B   :     :  B   |
    //  |   11 :   12 :    13 : ... :   1C |
    //  |......:......:.......:     :......|
    //  |      :      :       :     :      |
    //  |  B   :  B   :   B   :     :  B   |
    //  |   21 :   22 :    23 : ... :   2C |
    //  |......:......:.......:     :......|
    //  |      .                           |
    //  |      .                           |
    //  |      .                           |
    //  |......................     .......|
    //  |      :      :       :     :      |
    //  |  B   :  B   :   B   :     :  B   |
    //  |   R1 :   R2 :    R3 : ... :   RC |
    //  |......:......:.......:     :......|
    //
    //  Where each block B   is a square matrix.
    //                    ij
    //
    //  All we really need to do is figure out how to transpose any
    //  of the blocks.  Once that is done we can just cycle over
    //  all of the blocks in the matrix.
    //
    //  So what does the ij block look like? Well, this is it.
    //
    //
    //         a              a               ...    a
    //          RB+1 CB+1      RB+1 CB+2              RB+1 CB+BSIZE
    //
    //         a              a               ...    a
    //          RB+2 CB+1      RB+2 CB+2              RB+2 CB+BSIZE
    //
    //         a              a               ...    a
    //          RB+3 CB+1      RB+3 CB+2              RB+3 CB+BSIZE
    //
    //                             .
    //                             .
    //                             .
    //
    //          a              a                 ... a
    //           RB+BSIZE CB+1  RB+BSIZE CB+2         RB+BSIZE CB+BSIZE
    //
    //
    // where RB = (i-1)*BSIZE, and CB = (j-1)*BSIZE.  But inspection of
    // this block shows that to transpose it we simply need to swap
    // the entries
    //
    //            a           and  a
    //             RB+m CB+n        RB+n CB+m
    //
    // where m and n range over all integers from 1 to BSIZE.
    //

    //
    // Let's first check to make sure that the requested operation
    // makes sense.  Are all of the integers positive?
    //
    if (BSIZE < 1) {
        CHKIN(b"XPOSBL", ctx)?;
        SETMSG(b"The block size is not positive. The block size is #.", ctx);
        ERRINT(b"#", BSIZE, ctx);
        SIGERR(b"SPICE(BADBLOCKSIZE)", ctx)?;
        CHKOUT(b"XPOSBL", ctx)?;
        return Ok(());
    }

    if (NROW < 1) {
        CHKIN(b"XPOSBL", ctx)?;
        SETMSG(
            b"The number of rows in the matrix is not positive. The number of rows is #.",
            ctx,
        );
        ERRINT(b"#", NROW, ctx);
        SIGERR(b"SPICE(BADROWCOUNT)", ctx)?;
        CHKOUT(b"XPOSBL", ctx)?;
        return Ok(());
    }

    if (NCOL < 1) {
        CHKIN(b"XPOSBL", ctx)?;
        SETMSG(
            b"The number of columns in the matrix is not positive. The number of columns is #.",
            ctx,
        );
        ERRINT(b"#", NCOL, ctx);
        SIGERR(b"SPICE(BADCOLUMNCOUNT)", ctx)?;
        CHKOUT(b"XPOSBL", ctx)?;
        return Ok(());
    }

    //
    // Is there a whole number of blocks in the matrix.
    //
    if ((intrinsics::MOD(NCOL, BSIZE) != 0) || (intrinsics::MOD(NROW, BSIZE) != 0)) {
        CHKIN(b"XPOSBL", ctx)?;
        SETMSG(b"The block size does not evenly divide both the number of rows and the number of columns. The block size is #; the number of rows is #; the number of columns is #. ", ctx);

        ERRINT(b"#", BSIZE, ctx);
        ERRINT(b"#", NROW, ctx);
        ERRINT(b"#", NCOL, ctx);
        SIGERR(b"SPICE(BLOCKSNOTEVEN)", ctx)?;
        CHKOUT(b"XPOSBL", ctx)?;
        return Ok(());
    }

    //
    // If we get to this point we are ready to do the transposes.
    // Cycle over all of the blocks in the matrix.
    //
    for CB in intrinsics::range(0, (NCOL - 1), BSIZE) {
        for RB in intrinsics::range(0, (NROW - 1), BSIZE) {
            //
            // OK. Transpose block ( RB, CB ).
            //
            for I in 1..=BSIZE {
                for J in 1..=I {
                    if (I == J) {
                        BTMAT[[(RB + I), (CB + J)]] = BMAT[[(RB + I), (CB + J)]];
                    } else {
                        TEMP = BMAT[[(RB + I), (CB + J)]];
                        BTMAT[[(RB + I), (CB + J)]] = BMAT[[(RB + J), (CB + I)]];
                        BTMAT[[(RB + J), (CB + I)]] = TEMP;
                    }
                }
            }
        }
    }

    Ok(())
}
