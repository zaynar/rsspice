//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LINLEN: i32 = 255;
const BUFSIZ: i32 = 22;

/// DAS extract comments to a logical unit
///
/// Extract comments from a previously opened binary DAS file to a
/// previously opened text file attached to a Fortran logical unit.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///   HANDLE    I   Handle of a DAS file opened with read access.
///   COMLUN    I   Logical unit of an opened text file.
///   COMNTS    O   Logical flag, indicating comments were found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle for a binary DAS file that has been
///           opened with read access.
///
///  COMLUN   is the Fortran logical unit of a previously opened text
///           file to which the comments from a binary DAS file are
///           to be written.
///
///           The comments will be placed into the text file beginning
///           at the current location in the file, and continuing
///           until all of the comments have been written.
/// ```
///
/// # Detailed Output
///
/// ```text
///  COMNTS   is a logical flag indicating whether or not any comments
///           were found in the comment area of a DAS file. COMNTS will
///           have the value .TRUE. if there were some comments, and
///           the value .FALSE. otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while reading from the binary DAS file
///      attached to HANDLE, the error is signaled by a routine in the
///      call tree of this routine.
///
///  2)  If an error occurs while writing to the text file attached to
///      COMLUN, the error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Files
///
/// ```text
///  See parameters COMLUN and HANDLE in the $Detailed_Inputs section.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will extract all of the comments from the comment
///  area of a binary DAS file, placing them into a text file
///  attached to COMLUN, beginning at the current position in the
///  text file. If there are no comments in the DAS file, nothing is
///  written to the text file attached to COMLUN.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///     HANDLE   be the DAS file handle of a previously opened binary
///              DAS file.
///
///     COMLUN   be the Fortran logical unit of a previously opened
///              text file that is to accept the comments from the
///              DAS comment area.
///
///  The subroutine call
///
///     CALL DASECU ( HANDLE, COMLUN, COMNTS )
///
///  will extract the comments from the comment area of the binary
///  DAS file attached to HANDLE, if there are any, and write them
///  to the logical unit COMLUN. Upon successful completion, the
///  value of COMNTS will be .TRUE. if there were some comments
///  in the comment area and .FALSE. otherwise.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The maximum length of a single line comment in the comment
///      area is specified by the parameter LINLEN defined below.
///      Currently this value is 255 characters.
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
/// -    SPICELIB Version 1.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 05-JAN-1993 (KRG)
/// ```
pub fn dasecu(
    ctx: &mut SpiceContext,
    handle: i32,
    comlun: i32,
    comnts: &mut bool,
) -> crate::Result<()> {
    DASECU(handle, comlun, comnts, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASECU ( DAS extract comments to a logical unit )
pub fn DASECU(
    HANDLE: i32,
    COMLUN: i32,
    COMNTS: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COMBUF = ActualCharArray::new(LINLEN, 1..=BUFSIZ);
    let mut NUMCOM: i32 = 0;
    let mut EOC: bool = false;
    let mut GOTSOM: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    // Set the value for the maximum length of a text line.
    //
    //
    // Set the size of the comment buffer.
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
        CHKIN(b"DASECU", ctx)?;
    }
    //
    // Verify that the DAS file attached to HANDLE is opened for reading.
    //
    DASSIH(HANDLE, b"READ", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASECU", ctx)?;
        return Ok(());
    }
    //
    // Initialize some things before the loop.
    //
    NUMCOM = 0;
    EOC = false;
    GOTSOM = false;

    while !EOC {
        //
        // While we have not reached the end of the comments, get some
        // more.
        //
        DASEC(
            HANDLE,
            BUFSIZ,
            &mut NUMCOM,
            COMBUF.as_arg_mut(),
            &mut EOC,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"DASECU", ctx)?;
            return Ok(());
        }

        if (NUMCOM > 0) {
            //
            // If NUMCOM .GT. 0 then we did get some comments, and we need
            // to write them out, but first, set the flag indicating that
            // we got some comments.
            //
            if !GOTSOM {
                GOTSOM = true;
            }

            WRITLA(NUMCOM, COMBUF.as_arg(), COMLUN, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASECU", ctx)?;
                return Ok(());
            }
        }
    }
    //
    // Set the output flag indicating whether or not we got any comments.
    //
    *COMNTS = GOTSOM;

    CHKOUT(b"DASECU", ctx)?;
    Ok(())
}
