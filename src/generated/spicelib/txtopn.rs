//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Text file, open new
///
/// Open a new text file for subsequent write access.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of file.
///  UNIT       O   Logical unit.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of the new text file to be opened.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UNIT     is the logical unit connected to the opened file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the file cannot be opened, the error SPICE(FILEOPENFAILED)
///      is signaled.
///
///  2)  If FNAME is a blank string, the error SPICE(BLANKFILENAME) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See FNAME and UNIT above.
/// ```
///
/// # Particulars
///
/// ```text
///  In SPICELIB, a text file is formatted and sequential and may
///  contain only printable ASCII characters and blanks (ASCII 32-127).
///  When printing a text file, records are single spaced; the first
///  character will not be interpreted as a carriage control character.
///
///  TXTOPN opens a new text file and makes use of the SPICELIB
///  mechanism for coordinating the use of logical units.
///
///  System Dependencies
///  ===================
///
///  The open statement will include the following keyword = value
///  pairs:
///
///         UNIT   =  UNIT
///         FILE   =  FNAME
///         FORM   = 'FORMATTED'
///         ACCESS = 'SEQUENTIAL'
///         STATUS = 'NEW'
///         IOSTAT =  IOSTAT
///
///  In addition, the statement will include
///
///         CARRIAGECONTROL = 'LIST'
///
///  for the Vax and Macintosh.
/// ```
///
/// # Examples
///
/// ```text
///  The following example reads a line from an input file,
///  'INPUT.TXT', and writes it to an output file, 'OUTPUT.TXT'.
///
///     CALL TXTOPR ( 'INPUT.TXT',  IN  )
///     CALL TXTOPN ( 'OUTPUT.TXT', OUT )
///
///     READ  ( IN,  FMT='(A)' ) LINE
///     WRITE ( OUT, FMT='(A)' ) LINE
///
///     CLOSE ( IN  )
///     CLOSE ( OUT )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  "Language Reference Manual", Absoft Fortran V3.2, page 7-12
///       (for the NeXT), 1993.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.E. McLean        (JPL)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.27.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 2.26.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.25.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 2.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.23.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 2.22.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.21.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 2.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 2.19.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 2.18.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 2.17.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 2.16.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 2.15.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.14.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 2.13.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 2.12.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.11.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.10.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 2.9.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 2.8.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 2.7.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 2.6.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 2.5.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 2.4.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 2.3.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 2.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 2.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 2.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 2.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 2.0.3, 16-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 2.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 2.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 07-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 1.2.0, 11-NOV-1993 (HAN)
///
///         Module was updated for the Silicon Graphics, DEC Alpha-OSF/1,
///         and NeXT platforms.
///
/// -    SPICELIB Version 1.1.0, 12-OCT-1992 (HAN)
///
///         The code was reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1991 (JEM)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 07-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 1.2.0, 11-NOV-1993 (HAN)
///
///         Module was updated for the Silicon Graphics, DEC Alpha-OSF/1,
///         and NeXT platforms.
///
/// -    SPICELIB Version 1.1.0, 12-OCT-1992 (HAN)
///
///         The code was reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
/// ```
pub fn txtopn(ctx: &mut SpiceContext, fname: &str, unit: &mut i32) -> crate::Result<()> {
    TXTOPN(fname.as_bytes(), unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TXTOPN ( Text file, open new )
pub fn TXTOPN(FNAME: &[u8], UNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;

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
        CHKIN(b"TXTOPN", ctx)?;
    }

    if fstr::eq(FNAME, b" ") {
        SETMSG(b"A blank string is unacceptable as a file name", ctx);
        SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        CHKOUT(b"TXTOPN", ctx)?;
        return Ok(());
    }

    GETLUN(UNIT, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(*UNIT),
            file: Some(FNAME),
            form: Some(b"FORMATTED"),
            access: Some(b"SEQUENTIAL"),
            status: Some(b"NEW"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    if (IOSTAT != 0) {
        SETMSG(b"Could not open file #. IOSTAT was #. ", ctx);
        ERRCH(b"#", FNAME, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        CHKOUT(b"TXTOPN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"TXTOPN", ctx)?;
    Ok(())
}
