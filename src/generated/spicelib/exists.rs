//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Does the file exist?
///
/// Determine whether a file exists.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of the file in question.
///
///  The function returns the value .TRUE. if the file exists, .FALSE.
///  otherwise.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of the file in question. This may be any
///           unambiguous file name valid on the user's computer, for
///           example
///
///              '/usr/dir1/dir2/DATA.DAT'
///              './DATA.DAT'
///              'c:\usr\dir1\dir2\data.dat'
///
///           Environment or shell variables may not be used.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .TRUE. if the file exists, .FALSE.
///  otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the filename is blank, the error SPICE(BLANKFILENAME) is
///      signaled.
///
///  2)  If an I/O error occurs while checking the existence of the
///      indicated file, the error SPICE(INQUIREFAILED) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Use the Fortran INQUIRE statement to determine the existence
///  of FNAME.
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
///  1) Given two arbitrary files (one of them the actual code example
///     source file), determine if they exists.
///
///     Example code begins here.
///
///
///           PROGRAM EXISTS_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           LOGICAL                 EXISTS
///           INTEGER                 RTRIM
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 FILEN
///           PARAMETER             ( FILEN = 14 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FILEN)       FNAME  ( 2 )
///
///           INTEGER                 I
///
///     C
///     C     Define an array of file names.
///     C
///           DATA                    FNAME / 'exists_ex1.txt',
///          .                                'exists_ex1.pgm' /
///
///           DO I = 1, 2
///
///              IF ( EXISTS ( FNAME(I) ) ) THEN
///
///                 WRITE(*,*) 'The file ', FNAME(I)(:RTRIM(FNAME(I))),
///          .                 ' exists.'
///
///              ELSE
///
///                 WRITE(*,*) 'Cannot find the file ', FNAME(I)
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
///      Cannot find the file exists_ex1.txt
///      The file exists_ex1.pgm exists.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.3.0, 17-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
///         Changed input argument name FILE to FNAME for consistency with
///         other routines.
///
///         Added IMPLICIT NONE statement.
///
/// -    SPICELIB Version 2.2.1, 01-JUL-2014 (NJB)
///
///         VAX examples were deleted from the header.
///
/// -    SPICELIB Version 2.2.0, 09-DEC-1999 (WLT)
///
///         The input file name is now "trimmed" of trailing blanks
///         before checking its existence.
///
/// -    SPICELIB Version 2.1.0, 04-MAR-1996 (KRG)
///
///         Added a local logical variable that is used as temporary
///         storage for the results from the INQUIRE statement rather
///         than using the function name. This solved a problem on the
///         macintosh.
///
/// -    SPICELIB Version 2.0.0, 04-AUG-1994 (KRG)
///
///         Added a test to see if the filename was blank before the
///         INQUIRE statement. This allows a meaningful error message to
///         be presented.
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 2.0.0, 29-DEC-1988 (HAN)
///
///         The IOSTAT specifier was added to the INQUIRE statement.
///         If the value of IOSTAT is not equal to zero, an error
///         occurred during the execution of the INQUIRE statement.
///         In this case, a SPICELIB error is signaled and the routine
///         checks out.
/// ```
pub fn exists(ctx: &mut SpiceContext, fname: &str) -> crate::Result<bool> {
    let ret = EXISTS(fname.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure EXISTS ( Does the file exist? )
pub fn EXISTS(FNAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let mut EXISTS: bool = false;
    let mut IOSTAT: i32 = 0;
    let mut R: i32 = 0;
    let mut MYEXST: bool = false;

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
        EXISTS = false;
        return Ok(EXISTS);
    } else {
        CHKIN(b"EXISTS", ctx)?;
    }
    //
    // Initialize the local variable MYEXST to be .FALSE.
    //
    MYEXST = false;

    //
    // First we test to see if the filename is blank.
    //
    if fstr::eq(FNAME, b" ") {
        EXISTS = false;
        SETMSG(b"The file name is blank. ", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"EXISTS", ctx)?;
        return Ok(EXISTS);
    }

    R = RTRIM(FNAME);
    //
    // So simple, it defies explanation.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            file: Some(fstr::substr(FNAME, 1..=R)),
            exist: Some(&mut MYEXST),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    if (IOSTAT != 0) {
        EXISTS = false;
        SETMSG(b"Value of IOSTAT was *.", ctx);
        ERRINT(b"*", IOSTAT, ctx);
        SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        CHKOUT(b"EXISTS", ctx)?;
        return Ok(EXISTS);
    }
    //
    // Set the value of the function, check out and return.
    //
    EXISTS = MYEXST;

    CHKOUT(b"EXISTS", ctx)?;
    Ok(EXISTS)
}
