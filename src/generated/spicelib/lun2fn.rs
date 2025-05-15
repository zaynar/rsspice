//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Map logical unit of open file to its name.
///
/// Map the logical unit of an open file to its associated filename.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LUNIT      I   A logical unit to be mapped to a filename.
///  FILNAM     O   Name of the file associated with LUNIT.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LUNIT    is the Fortran logical unit that is to be mapped to the
///           filename with which it is associated. The file must be
///           open for this routine to work properly.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FILNAM   is the filename that is associated with the Fortran
///           logical unit LUNIT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the logical unit is not positive, the error
///      SPICE(INVALIDARGUMENT) is signaled.
///
///  2)  If an error occurs during the execution of the Fortran INQUIRE
///      statement, the error SPICE(INQUIREFAILED) is signaled.
///
///  3)  If the logical unit is not attached to an open file, the
///      error SPICE(FILENOTOPEN) is signaled.
///
///  4)  In the event of an error the contents of the variable FILNAM
///      are not defined and should not be used.
/// ```
///
/// # Particulars
///
/// ```text
///  Uses the Fortran INQUIRE statement to determine the filename that
///  is associated with the Fortran logical unit LUNIT.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates the use of LUN2FN.
///
///  C
///  C      Convert the logical unit to its filename and display it.
///  C
///         CALL LUN2FN ( UNIT1, FNAME1 )
///         WRITE (*,*) 'The filename is: ', FNAME1
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
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 16-AUG-1994 (KRG)
/// ```
pub fn lun2fn(ctx: &mut SpiceContext, lunit: i32, filnam: &mut str) -> crate::Result<()> {
    LUN2FN(
        lunit,
        fstr::StrBytes::new(filnam).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LUN2FN ( Map logical unit of open file to its name. )
pub fn LUN2FN(LUNIT: i32, FILNAM: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;
    let mut OPENED: bool = false;

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
        CHKIN(b"LUN2FN", ctx)?;
    }

    //
    // First we test to see if the filename is blank.
    //
    if (LUNIT <= 0) {
        SETMSG(b"The Fortran logical unit was not positive: #.", ctx);
        ERRINT(b"#", LUNIT, ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        CHKOUT(b"LUN2FN", ctx)?;
        return Ok(());
    }

    //
    // So simple, it defies explanation: just INQUIRE.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            unit: Some(LUNIT),
            name: Some(FILNAM),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"An error occurred while INQUIRing on unit #. The IOSTAT value is #.",
            ctx,
        );
        ERRINT(b"#", LUNIT, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        CHKOUT(b"LUN2FN", ctx)?;
        return Ok(());
    }

    //
    // If there is no open file associated with the logical unit LUNIT
    // we cannot get a filename. So signal an error.
    //
    if !OPENED {
        SETMSG(
            b"There was no open file associated with the logical unit #.",
            ctx,
        );
        ERRINT(b"#", LUNIT, ctx);
        SIGERR(b"SPICE(FILENOTOPEN)", ctx)?;
        CHKOUT(b"LUN2FN", ctx)?;
        return Ok(());
    }

    //
    // If we made it to here, we are done. Just check out and return.
    //
    CHKOUT(b"LUN2FN", ctx)?;
    Ok(())
}
