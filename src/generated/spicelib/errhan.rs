//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;
const MAXNUM: i32 = 32;

/// Insert DAF/DAS file name into long error message
///
/// Substitute the first occurrence of a marker in the current long
/// error message with the file name associated with a given
/// DAF/DAS handle.
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
///  MARKER     I   A substring in the long error message to be
///                 replaced.
///  HANDLE     I   DAF/DAS handle associated with a file.
///  FILEN      P   Maximum length of filename.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MARKER   is a character string that marks a position in
///           the long error message where a file name is to be
///           substituted. Leading and trailing blanks in MARKER
///           are not significant.
///
///           Case IS significant; 'XX' is considered to be
///           a different marker from 'xx'.
///
///  HANDLE   is the DAF/DAS handle associated with the file of
///           interest. HANDLE must be associated with a currently
///           loaded DAF or DAS file.
/// ```
///
/// # Parameters
///
/// ```text
///  FILEN    is the maximum file name length that can be
///           accommodated by this routine. Currently this
///           parameter is defined in the include file
///           zzddhman.inc.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If HANDLE refers to a scratch DAS file, the string inserted
///      into the long error message is
///
///         'DAS SCRATCH FILE'
///
///  2)  If HANDLE is not associated with a loaded DAF or DAS file,
///      the string inserted into the long error message is:
///
///         '<No name found for handle #>'
///
///      where the handle number is substituted for the marker '#'.
/// ```
///
/// # Files
///
/// ```text
///  See "Detailed_Input" description of the variable HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a convenient and error-free mechanism
///  for inserting a DAF or DAS file name into an error message,
///  given the file handle associated with the file of interest.
/// ```
///
/// # Examples
///
/// ```text
///  1) Create an error message pertaining to an SPK file
///     designated by HANDLE, then signal an error.
///
///        CALL SETMSG ( 'SPK file # contains a type 3 segment ' //
///       .              'with invalid polynomial degree #. '    //
///       .              'Segment index in file is #.'            )
///        CALL ERRHAN ( '#',  HANDLE                             )
///        CALL ERRINT ( '#',  DEGREE                             )
///        CALL ERRINT ( '#',  I                                  )
///        CALL SIGERR ( 'SPICE(INVALIDDEGREE)'                   )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The supported filename length is limited by the parameter
///      FILEN.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 02-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 05-FEB-2015 (NJB)
///
///         Removed header comments about restrictions
///         on using this routine for DAS files.
///
/// -    SPICELIB Version 1.0.0, 04-JAN-2002 (NJB)
/// ```
pub fn errhan(ctx: &mut SpiceContext, marker: &str, handle: i32) -> crate::Result<()> {
    ERRHAN(marker.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ERRHAN ( Insert DAF/DAS file name into long error message )
pub fn ERRHAN(MARKER: &[u8], HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FNAME = [b' '; FILEN as usize];
    let mut NUMSTR = [b' '; MAXNUM as usize];
    let mut INTARC: i32 = 0;
    let mut INTBFF: i32 = 0;
    let mut INTAMH: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Get the name of the file designated by the input handle.
    //
    ZZDDHNFO(
        HANDLE,
        &mut FNAME,
        &mut INTARC,
        &mut INTBFF,
        &mut INTAMH,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        INTSTR(HANDLE, &mut NUMSTR, ctx);

        fstr::assign(&mut FNAME, b"<No name found for handle ");
        SUFFIX(&NUMSTR, 1, &mut FNAME);
        SUFFIX(b">", 0, &mut FNAME);
    }

    //
    // Insert the file name string into the long error message.
    //
    ERRCH(MARKER, &FNAME, ctx);

    Ok(())
}
