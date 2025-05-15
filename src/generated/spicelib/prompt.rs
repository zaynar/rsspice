//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Prompt a user for a string
///
/// Prompt a user for keyboard input.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  DSPMSG     I   The prompt to use when asking for input.
///  BUFFER     O   The response typed by a user.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DSPMSG   is a character string that will be displayed from the
///           current cursor position and describes the input that
///           the user is expected to enter. The string DSPMSG should
///           be relatively short, i.e., 50 or fewer characters, so
///           that a response may be typed on the line where the
///           prompt appears.
///
///           All characters (including trailing blanks) in DSPMSG
///           are considered significant and will be displayed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  BUFFER   is a character string that contains the string
///           entered by the user.
/// ```
///
/// # Exceptions
///
/// ```text
///  This subroutine uses discovery check-in so that it may be called
///  after an error has occurred.
///
///  1)  If the attempt to write the prompt to the standard output
///      device fails, returning an IOSTAT value not equal to zero, the
///      error SPICE(WRITEFAILED) is signaled.
///
///  2)  If the attempt to read the response from the standard input
///      device fails, returning an IOSTAT value not equal to zero, the
///      error SPICE(READFAILED) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a utility that allows you to "easily" request information
///  from a program user. At a high level, it frees you from the
///  peculiarities of a particular implementation of FORTRAN cursor
///  control.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose you have an interactive program that computes state
///     vectors by calling SPKEZR. The program prompts the user for
///     the inputs to SPKEZR. After each prompt is written, the program
///     leaves the cursor at the end of the string as shown here:
///
///        Enter UTC epoch  > _
///
///     (The underscore indicates the cursor position).
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: prompt_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                        Contents
///           ---------                        --------
///           de430.bsp                        Planetary ephemeris
///           mar097.bsp                       Mars satellite ephemeris
///           naif0011.tls                     Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'naif0011.tls' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM PROMPT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT
///           PARAMETER           ( FMT    = '(A,F22.10)' )
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 36 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(STRLEN)    OBS
///           CHARACTER*(STRLEN)    TARGET
///           CHARACTER*(STRLEN)    EPOCH
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE ( 6 )
///
///           INTEGER               I
///
///     C
///     C     Load kernel.
///     C
///           CALL FURNSH( 'prompt_ex1.tm' )
///
///     C
///     C     Prompt for the required inputs.
///     C
///           CALL PROMPT ( 'Enter UTC epoch             > ', EPOCH  )
///           CALL PROMPT ( 'Enter observer name         > ', OBS    )
///           CALL PROMPT ( 'Enter target name           > ', TARGET )
///
///     C
///     C     Convert the UTC request time to ET (seconds past
///     C     J2000, TDB).
///     C
///           CALL STR2ET( EPOCH, ET )
///
///     C
///     C     Look up the state vector at the requested ET
///     C
///           CALL SPKEZR( TARGET, ET, 'J2000', 'NONE', OBS, STATE, LT)
///
///     C
///     C     Output...
///     C
///           WRITE(*,*) ' '
///           WRITE(*,FMT) 'Epoch               : ', ET
///           WRITE(*,FMT) '   x-position   (km): ', STATE(1)
///           WRITE(*,FMT) '   y-position   (km): ', STATE(2)
///           WRITE(*,FMT) '   z-position   (km): ', STATE(3)
///           WRITE(*,FMT) '   x-velocity (km/s): ', STATE(4)
///           WRITE(*,FMT) '   y-velocity (km/s): ', STATE(5)
///           WRITE(*,FMT) '   z-velocity (km/s): ', STATE(6)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the time string '2017-07-14T19:46:00' as epoch,
///     'MARS' as target and 'EARTH' as observer, the output was:
///
///
///     Enter UTC epoch             > 2017-07-14T19:46:00
///     Enter observer name         > MARS
///     Enter target name           > EARTH
///
///     Epoch               :   553333628.1837273836
///        x-position   (km):   173881563.8231496215
///        y-position   (km):  -322898311.5398598909
///        z-position   (km):  -147992421.0068917871
///        x-velocity (km/s):          47.4619819770
///        y-velocity (km/s):          19.0770886182
///        z-velocity (km/s):           7.9424268278
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine is environment specific. Standard FORTRAN does
///      not provide for user control of cursor position after write
///      statements.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.27.0, 28-NOV-2021 (BVS)
///
///         Updated for MAC-OSX-M1-64BIT-CLANG_C.
///
/// -    SPICELIB Version 3.26.0, 13-AUG-2021 (JDR)
///
///         Changed argument names PRMPT and STRING to DSPMSG and BUFFER
///         for consistency with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 3.25.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 3.24.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 3.23.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 3.22.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.21.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GCC_C.
///
/// -    SPICELIB Version 3.20.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 3.19.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-CC_C.
///
/// -    SPICELIB Version 3.18.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL-64BIT-CC_C.
///
/// -    SPICELIB Version 3.17.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-NATIVE_C.
///
/// -    SPICELIB Version 3.16.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 3.15.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.14.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-64BIT-MS_C.
///
/// -    SPICELIB Version 3.13.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-INTEL_C.
///
/// -    SPICELIB Version 3.12.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 3.11.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 3.10.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 3.9.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 3.8.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 3.7.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-LINUX-64BIT-GCC_C.
///
/// -    SPICELIB Version 3.6.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-INTEL_C.
///
/// -    SPICELIB Version 3.5.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 3.4.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 3.3.0, 26-OCT-2005 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-GCC_C.
///
/// -    SPICELIB Version 3.2.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN_C.
///
/// -    SPICELIB Version 3.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 3.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 3.0.4, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 3.0.3, 24-SEP-1999 (NJB)
///
///         CSPICE environments were added. Some typos were corrected.
///
/// -    SPICELIB Version 3.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
/// -    SPICELIB Version 3.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 3.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 2.0.0, 20-JUL-1995 (WLT) (KRG)
///
///         This routine now participates in error handling. It
///         checks to make sure no I/O errors have occurred while
///         attempting to write to standard output or read from standard
///         input. It uses discovery checkin if an error is detected.
///
///         Restructured the subroutine a little bit; the writing of the
///         prompt is the only bit that is environment specific, so the
///         code was rearranged to reflect this. There is now only a single
///         READ statement.
///
/// -    SPICELIB Version 1.0.0, 15-OCT-1992 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 2.0.0, 20-JUL-1995 (WLT) (KRG)
///
///         This routine now participates in error handling. It
///         checks to make sure no I/O errors have occurred while
///         attempting to write to standard output or read from standard
///         input. It uses discovery checkin if an error is detected.
///
///         Restructured the subroutine a little bit; the writing of the
///         prompt is the only bit that is environment specific, so the
///         code was rearranged to reflect this. There is now only a single
///         READ statement.
/// ```
pub fn prompt(ctx: &mut SpiceContext, dspmsg: &str, buffer: &mut str) -> crate::Result<()> {
    PROMPT(
        dspmsg.as_bytes(),
        fstr::StrBytes::new(buffer).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PROMPT ( Prompt a user for a string )
pub fn PROMPT(DSPMSG: &[u8], BUFFER: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;

    //
    // Local variables
    //

    //

    //
    //
    // The code below should be used in the following environments:
    //
    // SUN/Fortran,
    // HP/HP-Fortran,
    // Silicon Graphics/Silicon Graphics Fortran,
    // DEC Alpha-OSF/1--DEC Fortran,
    // NeXT/Absoft Fortran
    // PC Linux/Fort77
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.default_write_unit()?, None, b"(A,$)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(DSPMSG)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    //
    // If none of the write statements above works on a particular
    // unsupported platform, read on...
    //
    // Although, this isn't really what you want, if you need to port
    // this quickly to an environment that does not support the format
    // statement in any of the cases above, you can comment out the
    // write statement above and un-comment the write statement below.
    // In this way you can get a program working quickly in the new
    // environment while you figure out how to control cursor
    // positioning.
    //
    //  WRITE (*,*, IOSTAT=IOSTAT ) DSPMSG
    //
    // Check for a write error. It's not likely, but the standard output
    // can be redirected. Better safe than confused later.
    //
    if (IOSTAT != 0) {
        CHKIN(b"PROMPT", ctx)?;
        SETMSG(b"An error occurred while attempting to write a prompt to the standard output device, possibly because standard output has been redirected to a file. There is not much that can be done about this if it happens. We do not try to determine whether standard output has been redirected, so be sure that there are sufficient resources available for the operation being performed.", ctx);
        SIGERR(b"SPICE(WRITEFAILED)", ctx)?;
        CHKOUT(b"PROMPT", ctx)?;
        return Ok(());
    }

    //
    // Now that we've written out the prompt and there was no error, we
    // can read in the response.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::FormattedReader::new(ctx.default_read_unit()?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(BUFFER)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        CHKIN(b"PROMPT", ctx)?;
        SETMSG(b"An error occurred while attempting to retrieve a reply to the prompt \"#\".  A possible cause is that you have exhausted the input buffer while attempting to type your response.  It may help if you limit your response to # or fewer characters. ", ctx);
        ERRCH(b"#", DSPMSG, ctx);
        ERRINT(b"#", intrinsics::MIN0(&[intrinsics::LEN(BUFFER), 131]), ctx);
        SIGERR(b"SPICE(READFAILED)", ctx)?;
        CHKOUT(b"PROMPT", ctx)?;
        return Ok(());
    }

    Ok(())
}
