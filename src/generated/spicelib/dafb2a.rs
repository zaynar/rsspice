//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// DAF, binary to ASCII
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine DAFBT. NAIF supports this routine only to provide backward
/// compatibility.
///
/// Convert a binary DAF to an equivalent ASCII (text) DAF.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BINARY     I   Name of an existing binary DAF.
///  ASCII      I   Name of an ASCII (text) DAF to be created.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BINARY   is the name of an existing binary DAF.
///
///  ASCII    is the name of an ASCII (text) DAF to be created.
///           The ASCII file contains the same data as the binary
///           file, but in a form more suitable for transfer
///           between heterogeneous computing environments.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while reading the input binary DAF file,
///      the error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If an error occurs while converting the input binary DAF file
///      to text format, the error is signaled by a routine in the call
///      tree of this routine.
///
///  3)  If an error occurs while writing data to the output ASCII text
///      DAF file, the error is signaled by a routine in the call tree
///      of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See arguments BINARY, ASCII.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine has been made obsolete by the new DAF binary to text
///  conversion routine DAFBT. This routine remains available for
///  reasons of backward compatibility. We strongly recommend that the
///  conversion routine DAFBT be used for any new software development.
///  Please see the header of the routine DAFBT for details.
///
///  Note that the contents of reserved records in the binary file
///  are not stored in the ASCII file.
/// ```
///
/// # Examples
///
/// ```text
///  DAFB2A and DAFA2B are typically used to transfer files.
///  If file A.DAF is a binary DAF in environment 1, it can be
///  transferred to environment 2 in three steps.
///
///     1) Convert it to ASCII,
///
///           CALL DAFB2A ( 'A.DAF', 'A.ASCII' )
///
///     2) Transfer the ASCII file, using FTP, Kermit, or some other
///        file transfer utility,
///
///           ftp> put a.ascii
///
///     3) Convert it to binary on the new machine,
///
///           CALL DAFA2B ( 'A.ASCII', 'A.DAF', RESV )
///
///  Note that DAFB2A and DAFA2B work in any standard Fortran-77
///  environment.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Updated
///         $Exceptions section to better describe the issues detected by
///         this routine. Moved DAF required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 2.1.1, 26-JUL-2012 (EDW)
///
///         Edited $Abstract section to use "Deprecated" keyword
///         and state replacement routine.
///
///         Eliminated unneeded $Revisions section.
///
/// -    SPICELIB Version 2.1.0, 18-JUN-1999 (WLT)
///
///         Fixed call to CHKOUT with wrong name.
///
/// -    SPICELIB Version 2.0.0, 04-OCT-1993 (KRG)
///
///         This routine was completely rewritten to make use of the
///         routines DAFB2T and TXTOPN, for converting a text file to
///         binary and opening a text file. It now simply calls the
///         routine DAFT2B after opening the text file with TXTOPN.
///
///         Added a statement to the $Particulars section to the effect
///         that this routine has been made obsolete by the introduction of
///         the routine DAFBT, and that we strongly recommend the use of
///         the new routine.
///
///         Modified the $Abstract section to reflect the fact that this
///         routine is obsolete.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafb2a(ctx: &mut SpiceContext, binary: &str, ascii: &str) -> crate::Result<()> {
    DAFB2A(binary.as_bytes(), ascii.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFB2A ( DAF, binary to ASCII )
pub fn DAFB2A(BINARY: &[u8], ASCII: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut UNIT: i32 = 0;

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
        CHKIN(b"DAFB2A", ctx)?;
    }
    //
    // Open the ASCII file for writing. If an error occurs, then check
    // out and return. An appropriate error message will have already
    // been set.
    //
    TXTOPN(ASCII, &mut UNIT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFB2A", ctx)?;
        return Ok(());
    }
    //
    // Attempt to perform the file conversion. If it fails, close the
    // text file with STATUS = 'DELETE', check out and return, as an
    // appropriate error message should have already been set.
    //
    DAFB2T(BINARY, UNIT, ctx)?;

    if FAILED(ctx) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UNIT),
                status: Some(b"DELETE"),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
        CHKOUT(b"DAFB2A", ctx)?;
        return Ok(());
    }
    //
    // Close the text file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    CHKOUT(b"DAFB2A", ctx)?;
    Ok(())
}
