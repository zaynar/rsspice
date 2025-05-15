//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const DAT: &[u8] = b"begindata";
const TXT: &[u8] = b"begintext";
const BLANK: &[u8] = b" ";
const TABCDE: i32 = 9;
const MRKLEN: i32 = 10;
const BSLASH: i32 = 92;
const LNSIZE: i32 = 80;
const FILSIZ: i32 = 255;
const INTEXT: i32 = 1;
const INDATA: i32 = (INTEXT + 1);
const ENDOFF: i32 = (INDATA + 1);

struct SaveVars {
    BEGDAT: Vec<u8>,
    BEGTXT: Vec<u8>,
    FILE: Vec<u8>,
    FIRST: Vec<u8>,
    R: i32,
    STATUS: i32,
    LINNUM: i32,
    END: bool,
    FRSTIM: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BEGDAT = vec![b' '; MRKLEN as usize];
        let mut BEGTXT = vec![b' '; MRKLEN as usize];
        let mut FILE = vec![b' '; FILSIZ as usize];
        let mut FIRST = vec![b' '; LNSIZE as usize];
        let mut R: i32 = 0;
        let mut STATUS: i32 = 0;
        let mut LINNUM: i32 = 0;
        let mut END: bool = false;
        let mut FRSTIM: bool = false;

        FRSTIM = true;
        fstr::assign(&mut FILE, b" ");
        LINNUM = 0;

        Self {
            BEGDAT,
            BEGTXT,
            FILE,
            FIRST,
            R,
            STATUS,
            LINNUM,
            END,
            FRSTIM,
        }
    }
}

/// Read a kernel file
///
/// Open and read the contents of a SPICE ASCII kernel file.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  KERNEL     I   RDKNEW
///  LINE       O   RDKDAT
///  NUMBER     O   RDKLIN
///  EOF        O   RDKDAT
/// ```
///
/// # Detailed Input
///
/// ```text
///  All input is through entry RDKNEW.
/// ```
///
/// # Detailed Output
///
/// ```text
///  All output is through entry RDKDAT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If RDKER is called directly, the error SPICE(BOGUSENTRY) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  The SPICE ASCII kernel file KERNEL is opened by RDKNEW and read
///  by RDKDAT. The entry point RDKLIN is available for reporting
///  the name of the open file and the number of the last line that
///  was read from that file.
/// ```
///
/// # Particulars
///
/// ```text
///  RDKER should never be called directly, but should instead be
///  accessed only through its entry points, RDKNEW, RDKDAT and
///  RDKLIN.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, RDKNEW and RDKDAT are used to read
///  the contents of a kernel file.
///
///  Let the file KERNEL contain the following lines.
///
///     =============================================================
///
///     DELTA_T_A is defined to be 32.184 seconds, and should not
///     be changed except under the most unusual circumstances.
///
///     \begindata
///
///     DELTA_T_A       =   32.184
///
///     \begintext
///
///     The next three items determine the relativistic correction
///     in the difference ET - TAI. To turn the correction off,
///     just set K to zero.
///
///     \begindata
///
///     K               =    1.657D-3
///     ORBIT_ECC       =    1.671D-2
///     MEAN_ANOM       = (  6.239996D0,  1.99096871D-7 )
///
///     =============================================================
///
///  Then the code fragment
///
///     CALL RDKNEW ( KERNEL )
///     CALL RDKDAT ( LINE, EOF )
///
///     DO WHILE ( (.NOT. EOF) .AND. ( .NOT. FAILED () ) )
///        WRITE (6,*) LINE
///        CALL RDKDAT ( LINE, EOF )
///     END DO
///
///  prints the following lines.
///
///     =============================================================
///     DELTA_T_A       =   32.184
///     K               =    1.657D-3
///     ORBIT_ECC       =    1.671D-2
///     MEAN_ANOM       = (  6.239996D0,  1.99096871D-7 )
///     =============================================================
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The input file must be opened and initialized by RDKNEW prior
///      to the first call to RDKDAT.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.7.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 3.6.1, 17-JUN-2021 (JDR)
///
///         Edited the header of the RDKER umbrella routine and all its
///         entry entry points to comply with NAIF standard.
///
/// -    SPICELIB Version 3.6.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 3.5.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 3.4.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 3.3.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 3.2.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 3.1.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 3.0.0, 11-FEB-2008 (NJB)
///
///         Entry points RDKNEW and RDKDAT have been updated so as to be
///         able to parse text kernel lines containing tab characters.
///
/// -    SPICELIB Version 2.5.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 2.4.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 2.3.0, 14-NOV-2005 (BVS)
///
///         Reinstated HP_C environment.
///
/// -    SPICELIB Version 2.2.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 2.1.0, 03-OCT-2005 (EDW)
///
///         File rdker.f made a master file so as to
///         add the ZZSETNNREAD call. This call will exist
///         only in FORTRAN source intended for conversion
///         to C by the f2c utility.
///
///         The ZZSETNNREAD call activates and deactivates
///         the non-native text line read capability for the
///         CSPICE toolkit.
///
/// -    SPICELIB Version 2.0.1, 22-AUG-2001 (EDW)
///
///         Corrected ENDIF to END IF.
///
/// -    SPICELIB Version 2.0.0, 20-SEP-1995 (WLT)
///
///          The entry point RDKLIN was added.
///
/// -    SPICELIB Version 1.3.0, 22-SEP-1993 (NJB)
///
///          Updated for port to NeXT. The "previous kernel" is now closed
///          only if there actually was a previous kernel.
///
/// -    SPICELIB Version 1.2.0, 01-JUN-1992 (MJS)
///
///          RDKER now initializes the variables BEGDAT and BEGTXT
///          in a portable way. On the first valid entry to this routine,
///          the backslash character in the form CHAR(92) is concatenated
///          individually to 'begindata' and 'begintext'.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.1.0, 07-DEC-1990 (HAN)
///
///          The declarations for BEGDAT and BEGTXT were changed from
///          CHARACTER*10 to CHARACTER*(*).
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -     SPICELIB Version 2.0.0, 20-SEP-1995 (WLT)
///
///          The entry point RDKLIN was added.
///
/// -     SPICELIB Version 1.3.0, 22-SEP-1993 (NJB)
///
///          Updated for port to NeXT. The "previous kernel" is now closed
///          only if there actually was a previous kernel.
///
///          In the last version of this routine, on the first entry into
///          the routine, the variable FILE, which records the name of
///          the last kernel accessed, was passed to CLTEXT.  CLTEXT
///          executed an INQUIRE statement using this name, which was
///          not initialized. On the NeXT, this caused the INQUIRE
///          statement to fail.
///
///
/// -     SPICELIB Version 1.2.0, 01-JUN-1992 (MJS)
///
///          RDKER now initializes the variables BEGDAT and BEGTXT
///          in a portable way. On the first valid entry to this routine,
///          the backslash character in the form CHAR(92) is concatenated
///          individually to 'begindata' and 'begintext'. As a result of
///          this change, this module is no longer considered environment
///          specific. All references in the header to the previous method
///          of initialization were removed.
///
///          FILE is now initialized to ' '. Before this modification, if
///          a call to RDKDAT was performed prior to RDKNEW, RDTEXT
///          would have printed out garbage (on some machines) in its
///          error message when notifying the user that it couldn't read
///          from FILE.
///
/// -     SPICELIB Version 1.1.0, 7-DEC-1990 (HAN)
///
///          The declarations for BEGDAT and BEGTXT were changed from
///          CHARACTER*10 to CHARACTER*(*). The fixed length of 10 was
///          not long enough.
///
/// -     Beta Version 1.1.0, 9-MAR-1989 (HAN)
///
///          Moved the declaration of the parameters BEGDAT and
///          BEGTXT from the code to the $Declarations section.
///          Filled out the Brief I/O and $Parameters sections.
/// ```
pub fn rdker(
    ctx: &mut SpiceContext,
    kernel: &str,
    line: &str,
    number: i32,
    eof: bool,
) -> crate::Result<()> {
    RDKER(
        kernel.as_bytes(),
        line.as_bytes(),
        number,
        eof,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDKER ( Read a kernel file )
pub fn RDKER(
    KERNEL: &[u8],
    LINE: &[u8],
    NUMBER: i32,
    EOF: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Because some environments (such as the SUN) treat the backslash
    // character as a special character, some gyrations are needed to
    // put it into a variable in a "portable" way. This is the reason
    // for the following block of declarations. Admittedly this is
    // bizarre, but it works.
    //

    //
    // The ASCII decimal code for the tab character is 9.
    //

    //
    // Local variables
    //

    //
    // Save EVERYTHING.
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"RDKER", ctx)?;
    }

    //
    // Calling RDKER directly is a serious breach of protocol.
    // If RDKER is called, an error is signaled.
    //

    SETMSG(b"RDKER: You have called an entry which performs no run-time function. This may indicate a bug. Please check the documentation for the subroutine RDKER.", ctx);

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"RDKER", ctx)?;
    Ok(())
}

/// Open and initialize a new kernel file
///
/// Open and initialize a SPICE ASCII kernel file.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  KERNEL     I   Kernel file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  KERNEL   is the name of the SPICE ASCII kernel file to be opened
///           and initialized.
/// ```
///
/// # Files
///
/// ```text
///  The SPICE ASCII kernel file KERNEL is opened by RDKNEW and read
///  by RDKDAT.
/// ```
///
/// # Particulars
///
/// ```text
///  RDKNEW should be called prior to the first call to RDKDAT.
///  RDKNEW opens the kernel file and RDKDAT reads the lines of
///  data in the file.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, RDKNEW and RDKDAT are used to read
///  the contents of a kernel file.
///
///  Let the file KERNEL contain the following lines.
///
///     =============================================================
///
///     DELTA_T_A is defined to be 32.184 seconds, and should not
///     be changed except under the most unusual circumstances.
///
///     \begindata
///
///     DELTA_T_A       =   32.184
///
///     \begintext
///
///     The next three items determine the relativistic correction
///     in the difference ET - TAI. To turn the correction off,
///     just set K to zero.
///
///     \begindata
///
///     K               =    1.657D-3
///     ORBIT_ECC       =    1.671D-2
///     MEAN_ANOM       = (  6.239996D0,  1.99096871D-7 )
///
///     =============================================================
///
///  Then the code fragment
///
///     CALL RDKNEW ( KERNEL )
///     CALL RDKDAT ( LINE, EOF )
///
///     DO WHILE ( (.NOT. EOF) .AND. ( .NOT. FAILED () ) )
///        WRITE (6,*) LINE
///        CALL RDKDAT ( LINE, EOF )
///     END DO
///
///  prints the following lines.
///
///     =============================================================
///     DELTA_T_A       =   32.184
///     K               =    1.657D-3
///     ORBIT_ECC       =    1.671D-2
///     MEAN_ANOM       = (  6.239996D0,  1.99096871D-7 )
///     =============================================================
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.1, 17-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 11-FEB-2008 (NJB)
///
///         This entry point has been updated so as to be
///         able to parse text kernel lines containing tab
///         characters.
///
/// -    SPICELIB Version 2.1.0, 03-OCT-2005 (EDW)
///
///         File rdker.f made a master file so as to
///         add the ZZSETNNREAD call. This call will exist
///         only in FORTRAN source intended for conversion
///         to C by the f2c utility.
///
///         The ZZSETNNREAD call activates and deactivates
///         the non-native text line read capability for the
///         CSPICE toolkit.
///
/// -    SPICELIB Version 2.0.0, 20-SEP-1995 (WLT)
///
///          The entry point RDKLIN was added.
///
/// -    SPICELIB Version 1.2.0, 01-JUN-1992 (MJS)
///
///          RDKER now initializes the variables BEGDAT and BEGTXT
///          in a portable way. On the first valid entry to this routine,
///          the backslash character in the form CHAR(92) is concatenated
///          individually to 'begindata' and 'begintext'.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn rdknew(ctx: &mut SpiceContext, kernel: &str) -> crate::Result<()> {
    RDKNEW(kernel.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDKNEW ( Open and initialize a new kernel file )
pub fn RDKNEW(KERNEL: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"RDKNEW", ctx)?;
    }

    //
    // Initialize the data delimiters if it hasn't been done already.
    //
    if save.FRSTIM {
        fstr::assign(
            &mut save.BEGDAT,
            &fstr::concat(&intrinsics::CHAR(BSLASH), DAT),
        );
        fstr::assign(
            &mut save.BEGTXT,
            &fstr::concat(&intrinsics::CHAR(BSLASH), TXT),
        );

        save.FRSTIM = false;
    } else {
        //
        // Close the previous file, if it hasn't been closed already.
        //
        CLTEXT(&save.FILE, ctx)?;
    }

    //
    // Close the new file, too, in case they are the same. No sense
    // burning up logical units.
    //
    CLTEXT(KERNEL, ctx)?;

    //
    // Read the first line of the file. It can't possibly be a data
    // line, since data must be preceded by a \begindata marker, so
    // we needn't take any pains to save it.
    //
    // We also initialize LINNUM to 1 so we know
    // the line number of the last line read and can return this
    // information from RDKLIN.
    //

    RDTEXT(KERNEL, &mut save.FIRST, &mut save.END, ctx)?;

    //
    // Replace any tab characters with blanks.
    //
    save.R = RTRIM(&save.FIRST);

    for I in 1..=save.R {
        if (intrinsics::ICHAR(fstr::substr(&save.FIRST, I..=I)) == TABCDE) {
            fstr::assign(fstr::substr_mut(&mut save.FIRST, I..=I), b" ");
        }
    }

    LJUST(&save.FIRST.to_vec(), &mut save.FIRST);
    save.LINNUM = 1;

    //
    // The first line is enough to set the status for subsequent
    // calls to RDKDAT.
    //
    if save.END {
        save.STATUS = ENDOFF;
        CLTEXT(KERNEL, ctx)?;
    } else if fstr::eq(&save.FIRST, &save.BEGDAT) {
        save.STATUS = INDATA;
    } else {
        save.STATUS = INTEXT;
    }

    //
    // Save the name of the file for future reference.
    //
    fstr::assign(&mut save.FILE, KERNEL);

    CHKOUT(b"RDKNEW", ctx)?;
    Ok(())
}

/// Read the next data line from a kernel file
///
/// Read the next line of data from a SPICE ASCII kernel file.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LINE       O   Next line of kernel data.
///  EOF        O   End of file indicator.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LINE     is the next line of data from the kernel file most
///           recently opened by NEWKER. Data lines are non-blank lines
///           which lie between \begindata and \begintext markers.
///           Lines are returned left justified.
///
///  EOF      is .TRUE. when the end of the kernel file has been
///           reached, and is .FALSE. otherwise. The kernel file is
///           closed automatically when the end of the file is reached.
/// ```
///
/// # Files
///
/// ```text
///  The SPICE ASCII kernel file KERNEL is opened by RDKNEW and read
///  by RDKDAT.
/// ```
///
/// # Particulars
///
/// ```text
///  RDKDAT is used internally by RDKVAR to retrieve successive lines
///  of data from the current kernel file. It exists primarily to
///  relieve RDKVAR of the responsibility of dealing with comment
///  blocks and blank lines.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, RDKNEW and RDKDAT are used to read
///  the contents of a kernel file.
///
///  Let the file KERNEL contain the following lines.
///
///     =============================================================
///
///     DELTA_T_A is defined to be 32.184 seconds, and should not
///     be changed except under the most unusual circumstances.
///
///     \begindata
///
///     DELTA_T_A       =   32.184
///
///     \begintext
///
///     The next three items determine the relativistic correction
///     in the difference ET - TAI. To turn the correction off,
///     just set K to zero.
///
///     \begindata
///
///     K               =    1.657D-3
///     ORBIT_ECC       =    1.671D-2
///     MEAN_ANOM       = (  6.239996D0,  1.99096871D-7 )
///
///     =============================================================
///
///  Then the code fragment
///
///     CALL RDKNEW ( KERNEL )
///     CALL RDKDAT ( LINE, EOF )
///
///     DO WHILE ( (.NOT. EOF) .AND. ( .NOT. FAILED () ) )
///        WRITE (6,*) LINE
///        CALL RDKDAT ( LINE, EOF )
///     END DO
///
///  prints the following lines.
///
///     =============================================================
///     DELTA_T_A       =   32.184
///     K               =    1.657D-3
///     ORBIT_ECC       =    1.671D-2
///     MEAN_ANOM       = (  6.239996D0,  1.99096871D-7 )
///     =============================================================
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The input file must be opened and initialized by NEWKER prior
///      to the first call to RDKDAT.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.1, 17-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 11-FEB-2008 (NJB)
///
///         This entry point has been updated so as to be
///         able to parse text kernel lines containing tab
///         characters.
///
/// -    SPICELIB Version 2.1.0, 03-OCT-2005 (EDW)
///
///         File rdker.f made a master file so as to
///         add the ZZSETNNREAD call. This call will exist
///         only in FORTRAN source intended for conversion
///         to C by the f2c utility.
///
///         The ZZSETNNREAD call activates and deactivates
///         the non-native text line read capability for the
///         CSPICE toolkit.
///
/// -    SPICELIB Version 2.0.1, 22-AUG-2001 (EDW)
///
///         Corrected ENDIF to END IF.
///
/// -    SPICELIB Version 2.0.0, 20-SEP-1995 (WLT)
///
///         The entry point RDKLIN was added.
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
/// -     Beta Version 2.0.0, 23-OCT-1989 (HAN)
///
///         A FAILED test was added to the DO-loop which reads
///         lines in the kernel file.
///
///         If the error action was set to 'RETURN' an infinite loop
///         could have resulted if RDTEXT failed and the loop conditions
///         were satisfied.
/// ```
pub fn rdkdat(ctx: &mut SpiceContext, line: &mut str, eof: &mut bool) -> crate::Result<()> {
    RDKDAT(fstr::StrBytes::new(line).as_mut(), eof, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RDKDAT ( Read the next data line from a kernel file )
pub fn RDKDAT(LINE: &mut [u8], EOF: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"RDKDAT", ctx)?;
    }

    //
    // If the previous call detected the end of the file,
    // this one should do the same.
    //
    if (save.STATUS == ENDOFF) {
        *EOF = true;
        CHKOUT(b"RDKDAT", ctx)?;
        return Ok(());
    }

    //
    // Well, at least we can try to read a line. Adjust the status as
    // needed, return if appropriate, read another line if necessary.
    // Basically, we're looking for a non-blank line in a data segment.
    //
    // Note that after every read, we increment LINNUM so we know
    // the line number of the last line read and can return this
    // information from RDKLIN.
    //
    fstr::assign(LINE, BLANK);

    while (!FAILED(ctx) && ((save.STATUS == INTEXT) || fstr::eq(LINE, BLANK))) {
        RDTEXT(&save.FILE, LINE, EOF, ctx)?;

        //
        // Replace any tab characters with blanks.
        //
        save.R = RTRIM(LINE);

        for I in 1..=save.R {
            if (intrinsics::ICHAR(fstr::substr(LINE, I..=I)) == TABCDE) {
                fstr::assign(fstr::substr_mut(LINE, I..=I), b" ");
            }
        }

        LJUST(&LINE.to_vec(), LINE);
        save.LINNUM = (save.LINNUM + 1);

        if *EOF {
            save.STATUS = ENDOFF;
            CLTEXT(&save.FILE, ctx)?;
            CHKOUT(b"RDKDAT", ctx)?;
            return Ok(());
        } else if fstr::eq(LINE, &save.BEGTXT) {
            save.STATUS = INTEXT;
        } else if fstr::eq(LINE, &save.BEGDAT) {
            save.STATUS = INDATA;
            fstr::assign(LINE, BLANK);
        }
    }

    CHKOUT(b"RDKDAT", ctx)?;
    Ok(())
}

/// Reading kernel at line number
///
/// Return the name of file and line number of the last line read by
/// the entry point RDKDAT.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  KERNEL     O   The name of the current file that is being read.
///  NUMBER     O   The line number of the last line read in the file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  KERNEL   is the name of the last file supplied via a call to
///           RDKNEW. If no call to RDKNEW have been made KERNEL is
///           returned as a blank. If KERNEL is not sufficiently long
///           to hold th name of the file, the file name will be
///           truncated on the right.
///
///  NUMBER   is the number of the last line in KERNEL returned by a
///           call to RDKDAT. If no call to RDKNEW or RDKDAT have been
///           made NUMBER is returned with the value 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If no calls to RDKNEW have been made, KERNEL is returned as
///      a blank and NUMBER is returned with the value 0.
///
///  2)  If no calls to RDKDAT have been made but RDKNEW has been
///      called, NUMBER is returned with the value 1.
///
///  3)  If KERNEL is not sufficiently long to hold the name of the
///      file being read, the name will be truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is a utility to aid in determining the last
///  line read in a text file that is being read via RDKDAT.
///
///  It is particular useful in pointing out the location of
///  an error in an input file.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you are processing a file and have detected an
///  error in the syntax in the file. The following code fragment
///  illustrates how you can use this routine to inform a user of
///  the location of the error in the file.
///
///     CALL RDKLIN ( FILE, NUMBER )
///     R =  RTRIM  ( FILE )
///
///     WRITE (*,*) 'An error occurred while reading line ', NUMBER
///     WRITE (*,*) 'of the file ''', FILE(1:R), ''''
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.1, 17-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Corrected
///         output argument name in $Declarations section (changed FILE to
///         KERNEL)
///
/// -    SPICELIB Version 2.1.0, 03-OCT-2005 (EDW)
///
///         File rdker.f made a master file so as to
///         add the ZZSETNNREAD call. This call will exist
///         only in FORTRAN source intended for conversion
///         to C by the f2c utility.
///
///         The ZZSETNNREAD call activates and deactivates
///         the non-native text line read capability for the
///         CSPICE toolkit.
///
/// -    SPICELIB Version 2.0.0, 20-SEP-1995 (WLT)
/// ```
pub fn rdklin(ctx: &mut SpiceContext, kernel: &mut str, number: &mut i32) {
    RDKLIN(
        fstr::StrBytes::new(kernel).as_mut(),
        number,
        ctx.raw_context(),
    );
}

//$Procedure RDKLIN ( Reading kernel at line number )
pub fn RDKLIN(KERNEL: &mut [u8], NUMBER: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Not much to do here.  Just copy the information and return.
    //
    fstr::assign(KERNEL, &save.FILE);
    *NUMBER = save.LINNUM;
}
