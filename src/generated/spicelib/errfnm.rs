//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const FILEN: i32 = 128;

/// Insert filename into long error message text
///
/// Substitute the first occurrence of a marker in the current long
/// error message with the name of the file attached to the logical
/// unit number.
///
/// # Required Reading
///
/// * [ERROR](crate::required_reading::error)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MARKER     I   A substring in the error message that is to be
///                 replaced.
///  UNIT       I   Logical unit number attached to a file.
///  FILEN      P   Maximum length of filename.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MARKER   is a character string which marks a position in
///           the long error message where a character string
///           is to be substituted. Leading and trailing blanks
///           in MARKER are not significant.
///
///           Case IS significant;  'XX' is considered to be
///           a different marker from 'xx'.
///
///  UNIT     is the logical unit number attached to a file.
/// ```
///
/// # Parameters
///
/// ```text
///  FILEN    is the maximum file name length that can be
///           accommodated by this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the logical unit number is not attached to a file, the
///      string inserted into the long error message is:
///
///      '<unavailable from the system>'
///
///  2)  If the FORTRAN INQUIRE statement fails to execute properly,
///      the string inserted into the long error message is:
///
///      '<unavailable from the system>'
/// ```
///
/// # Files
///
/// ```text
///  See "Detailed_Input" description of the variable UNIT.
/// ```
///
/// # Examples
///
/// ```text
///  1. The following code fragment reads a record from a file
///     then checks to see if the read was successful. If the
///     read failed, an error message is constructed that
///     specifies the record number, the filename and the value
///     of IOSTAT.
///
///     ERRFNM is used to replace the marker in the long error
///     message with the name of the file.
///
///
///     READ ( UNIT, REC=RECNUM, IOSTAT=IOSTAT ) RECORD
///
///      IF ( IOSTAT .NE. 0 ) THEN
///
///         CALL SETMSG ( 'Error reading record number # from ' //
///   .                   'file FILENAME. The value of IOSTAT ' //
///   .                   'was #.'                              )
///
///         CALL ERRINT ( '#',         RECNUM   )
///         CALL ERRFNM ( 'FILENAME',  UNIT     )
///         CALL ERRINT ( '#',         IOSTAT   )
///         CALL SIGERR ( 'SPICE(READFAILURE)'  )
///         CALL CHKOUT ( 'SAMPLE'              )
///         RETURN
///
///      END IF
///
///
///      If the unit is attached to the file SAMPLE.DAT, RECNUM
///      is 15 and IOSTAT is 36, and the INQUIRE statement in
///      this routine executed successfully, the long error
///      message is:
///
///        'Error reading record number 15 from file SAMPLE.DAT.
///         The value of IOSTAT was 36.'
///
///
///      If the unit is not attached to a file or if the INQUIRE
///      statement in this routine failed to execute successfully,
///      the long error message is:
///
///        'Error reading record number 15 from file
///        <unavailable from the system>. The value of IOSTAT
///        was 36.'
///
///
///  2. Note that the case of the marker is significant. The following
///     code fragment contains a call to ERRFNM using a marker
///     that does not appear in the long error message.
///
///
///     READ ( UNIT, REC=RECNUM, IOSTAT=IOSTAT ) RECORD
///
///      IF ( IOSTAT .NE. 0 ) THEN
///
///         CALL SETMSG ( 'Error reading record number # from ' //
///   .                   'file FILENAME. The value of IOSTAT ' //
///   .                   'was #.'                              )
///
///         CALL ERRINT ( '#',         RECNUM   )
///         CALL ERRFNM ( 'filename',  UNIT     )
///         CALL ERRINT ( '#',         IOSTAT   )
///         CALL SIGERR ( 'SPICE(READFAILURE)'  )
///         CALL CHKOUT ( 'SAMPLE'              )
///         RETURN
///
///      END IF
///
///
///      If the marker is not found, ERRFNM does not substitute
///      the filename for the marker. The long error message in
///      this case is:
///
///        'Error reading record number 15 from file FILENAME.
///         The value of IOSTAT was 36.'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The filename length is restricted by the parameter FILEN.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 02-JUN-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (HAN)
/// ```
pub fn errfnm(ctx: &mut SpiceContext, marker: &str, unit: i32) -> crate::Result<()> {
    ERRFNM(marker.as_bytes(), unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ERRFNM ( Insert filename into long error message text )
pub fn ERRFNM(MARKER: &[u8], UNIT: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NAME = [b' '; FILEN as usize];
    let mut IOSTAT: i32 = 0;

    //
    // Local variables
    //

    //
    // Initialize the variables.
    //
    fstr::assign(&mut NAME, b" ");

    //
    // Get the name of the file attached to the logical unit number.
    //

    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            unit: Some(UNIT),
            name: Some(&mut NAME),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // If the INQUIRE statement executed successfully and the unit
    // was attached to a file, we have a filename.
    //
    // If the INQUIRE statement didn't execute successfully the value
    // of IOSTAT is not equal to zero. If the unit is not connected to
    // a file the filename is blank. If either of these two things
    // are true, we must construct a string indicating that the
    // filename was unavailable from the system.
    //

    if ((IOSTAT != 0) || fstr::eq(&NAME, b" ")) {
        fstr::assign(&mut NAME, b"<unavailable from the system>");
    }

    //
    // Let the error handling routine take it from here.
    //
    ERRCH(MARKER, &NAME, ctx);

    Ok(())
}
