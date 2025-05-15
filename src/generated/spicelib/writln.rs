//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Write a text line to a logical unit
///
/// Write a single line of text to the Fortran logical unit UNIT.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LINE       I   The line which is to be written to UNIT.
///  UNIT       I   The Fortran unit number to use for output.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LINE     is the text line which is to be written to UNIT.
///
///           The value of this variable is not modified.
///
///  UNIT     is the Fortran unit number for the output. This may be
///           either the unit number for the terminal, or the unit
///           number of a previously opened text file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs while attempting to write to the text
///      file attached to UNIT, the error SPICE(FILEWRITEFAILED) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will write a single text line to the device
///  specified by UNIT. UNIT may be the terminal, or it may be
///  a logical unit number obtained from a Fortran OPEN or INQUIRE
///  statement. When written, the line will have trailing spaces
///  removed.
/// ```
///
/// # Examples
///
/// ```text
///  CALL WRITLN( LINE, UNIT )
///
///  You have now written a line of text to unit UNIT.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
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
/// -    SPICELIB Version 2.26.0, 16-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Removed implementation detail (discovery check-in) from
///         $Exceptions section.
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
/// -    SPICELIB Version 2.20.1, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
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
/// -    SPICELIB Version 2.0.0, 08-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 1.1.1, 20-AUG-1996 (WLT)
///
///         Corrected the heading for the $Index_Entries section.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         For the Macintosh, we need to use real Fortran I/O, i.e.,
///         using the first column for carriage control. The change
///         was to move the MAC environment indicator from one
///         environment case to the other.
///
///         Also, for UNIX environments, the parameter STDOUT is no
///         longer defined. This only appears for platforms that
///         need it to differentiate between writing to a file and
///         the terminal screen (standard output), currently: VAX,
///         PC-LAHEY, PC-MS, and MAC.
///
/// -    SPICELIB Version 1.0.0, 20-DEC-1995 (KRG) (HAN)
///
///         The routine graduated
///
///      Beta Version 3.1.0, 18-AUG-1995 (KRG)
///
///         Moved the PC-LAHEY environment indicator from one environment
///         case to the other. The Lahey compiler on the PC does treat text
///         files and the standard output device differently.
///
///      Beta Version 3.0.1, 01-JAN-1995 (KRG)
///
///         Moved the description of the input variable UNIT from the $
///         $Detailed_Output section of the header to the correct location
///         in the $Detailed_Input section of the header.
///
///      Beta Version 3.0.0, 11-JUL-1994 (HAN)
///
///         Edited master source file to correct the code for the
///         PC/Microsoft FORTRAN PowerStation environment. It should use
///         the same code as the VAX, not the PC/Lahey Fortran code. Also,
///         code was included for the DEC Alpha OpenVMS/DEC Fortran and
///         Sun Solaris/Sun Fortran environments.
///
///      Beta Version 2.0.0, 30-MAR-1994 (HAN)
///
///         Edited master source file to include new environments:
///         Silicon Graphics IRIX/Silicon Graphics Fortran,
///         DEC Alpha-OSF/1, and NeXT/Absoft Fortran.
///
///      Beta Version 1.0.0, 17-DEC-1992 (KRG)
/// ```
pub fn writln(ctx: &mut SpiceContext, line: &str, unit: i32) -> crate::Result<()> {
    WRITLN(line.as_bytes(), unit, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WRITLN ( Write a text line to a logical unit )
pub fn WRITLN(LINE: &[u8], UNIT: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // UNIX based fortran implementations typically do not distinguish
    // between a text file and the standard output unit, so no leading
    // vertical spacing character is required.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(fstr::substr(LINE, 1..=RTRIM(LINE)))?;
            writer.finish()?;
            Ok(())
        })?;
    }
    //
    // Check to see if we got a write error, and signal it if we did.
    // Also check in and check out.
    //
    if (IOSTAT != 0) {
        CHKIN(b"WRITLN", ctx)?;
        SETMSG(b"Error Writing to file: #. IOSTAT = #.", ctx);
        ERRFNM(b"#", UNIT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"WRITLN", ctx)?;
        return Ok(());
    }

    Ok(())
}
