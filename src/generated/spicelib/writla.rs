//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Write array of lines to a logical unit
///
/// Write an array of text lines to a Fortran logical unit.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUMLIN    I    Number of lines to be written to the file.
///  ARRAY     I    Array containing the lines to be written.
///  UNIT      I    Fortran unit number to use for output.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMLIN   is the number of text lines in ARRAY which are to be
///           written to UNIT. NUMLIN > 0.
///
///  ARRAY    is the array which contains the text lines to be written
///           to UNIT.
///
///           The contents of this variable are not modified.
///
///  UNIT     is the Fortran unit number for the output. This may
///           be either the unit number for the terminal, or the
///           unit number of a previously opened text file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of lines, NUMLIN, is not positive, the error
///      SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If an error occurs while attempting to write to the text file
///      attached to UNIT, the error is signaled by a routine in the
///      call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See the description of UNIT above.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes an array of character strings to a specified
///  Fortran logical unit, writing each array element as a line of
///  output.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) The following example demonstrates the use of this routine,
///     displaying a short poem on the standard output device,
///     typically a terminal screen.
///
///     Example code begins here.
///
///
///           PROGRAM WRITLA_EX1
///           IMPLICIT NONE
///
///     C
///     C     Example program for WRITLA.
///     C
///           CHARACTER*(80) LINES(4)
///
///           LINES(1) = 'Mary had a little lamb'
///           LINES(2) = 'Whose fleece was white as snow'
///           LINES(3) = 'And everywhere that mary went'
///           LINES(4) = 'The lamb was sure to go'
///
///           CALL WRITLA ( 4, LINES, 6 )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Mary had a little lamb
///     Whose fleece was white as snow
///     And everywhere that mary went
///     The lamb was sure to go
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated to remove unnecessary lines of code in the
///         Standard SPICE error handling CHKIN statements.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 20-DEC-1995 (KRG)
///
///         The routine graduated
///
///      Beta Version 2.0.0, 13-OCT-1994 (KRG)
///
///         This routine now participates fully with the SPICELIB error
///         handler, checking in on entry and checking out on exit. The
///         overhead associated with the error handler should not be
///         significant relative to the operation of this routine.
///
///      Beta Version 1.0.0, 18-DEC-1992 (KRG)
/// ```
pub fn writla(
    ctx: &mut SpiceContext,
    numlin: i32,
    array: CharArray,
    unit: i32,
) -> crate::Result<()> {
    WRITLA(numlin, array, unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WRITLA ( Write array of lines to a logical unit )
pub fn WRITLA(
    NUMLIN: i32,
    ARRAY: CharArray,
    UNIT: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);

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
    }

    CHKIN(b"WRITLA", ctx)?;

    //
    // Check to see if the maximum number of lines is positive.
    //
    if (NUMLIN <= 0) {
        SETMSG(
            b"The number of lines to be written was not positive. It was #.",
            ctx,
        );
        ERRINT(b"#", NUMLIN, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"WRITLA", ctx)?;
        return Ok(());
    }

    //
    // Begin writing the lines to UNIT. Stop when an error occurs, or
    // when we have finished writing all of the lines.
    //
    for I in 1..=NUMLIN {
        WRITLN(&ARRAY[I], UNIT, ctx)?;

        if FAILED(ctx) {
            //
            // If the write failed, an appropriate error message has
            // already been set, so we simply need to return.
            //
            CHKOUT(b"WRITLA", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"WRITLA", ctx)?;
    Ok(())
}
