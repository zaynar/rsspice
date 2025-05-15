//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const STLEN: i32 = 7;
const FAILUR: i32 = 1;
const SUCCES: i32 = 0;

/// Exit a program indicating an error status
///
/// Exit an executing program returning a success or failure status
/// to the operating system, if this capability is supported, or
/// simply exit the program.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STATUS     I   A string indicating the exit status of a program.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STATUS   is a character string which indicates the status
///           to use when exiting a program. The two status values
///           currently supported are 'SUCCESS' and 'FAILURE', which
///           have their obvious meanings. The case of the input is
///           not important, i.e., 'Success' or 'failure' are accepted.
///
///           If STATUS has a value of 'SUCCESS', then the calling
///           program will be terminated with a status that indicates
///           success.
///
///           If STATUS has a value of 'FAILURE', then the calling
///           program will be terminated with a status that indicates
///           failure.
///
///           If STATUS has a value that is not recognized, the calling
///           program will be terminated with a status that indicates
///           failure.
///
///           For environments which do not support the passage of a
///           status indicator to the operating system the value of
///           STATUS is not significant; an executing program will be
///           terminated using the Fortran STOP statement.
///
///           See the value of the parameter STLEN declared in the
///           Local $Parameters section for the maximum significant
///           length of an exit status string.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine is called by SIGERR to exit a program
///  returning a success or failure indication to the operating
///  system when this is possible. If returning a status
///  indication to the operating system is not possible, this
///  subroutine simply executes a Fortran STOP statement.
/// ```
///
/// # Examples
///
/// ```text
///  To exit a program indicating success:
///
///     CALL BYEBYE ( 'SUCCESS' )
///
///  To exit a program indicating failure:
///
///     CALL BYEBYE ( 'FAILURE' )
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
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.16.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 2.15.0, 10-MAR-2014 (BVS)
///
///         Updated for SUN-SOLARIS-64BIT-INTEL.
///
/// -    SPICELIB Version 2.14.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-LINUX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.13.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-GFORTRAN.
///
/// -    SPICELIB Version 2.12.0, 10-MAR-2014 (BVS)
///
///         Updated for PC-CYGWIN-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.11.0, 13-MAY-2010 (BVS)
///
///         Updated for SUN-SOLARIS-INTEL.
///
/// -    SPICELIB Version 2.10.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-WINDOWS-64BIT-IFORT.
///
/// -    SPICELIB Version 2.9.0, 13-MAY-2010 (BVS)
///
///         Updated for PC-LINUX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.8.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-IFORT.
///
/// -    SPICELIB Version 2.7.0, 13-MAY-2010 (BVS)
///
///         Updated for MAC-OSX-64BIT-GFORTRAN.
///
/// -    SPICELIB Version 2.6.0, 18-MAR-2009 (BVS)
///
///         Updated for PC-LINUX-GFORTRAN.
///
/// -    SPICELIB Version 2.5.0, 18-MAR-2009 (BVS)
///
///         Updated for MAC-OSX-GFORTRAN.
///
/// -    SPICELIB Version 2.4.0, 19-FEB-2008 (BVS)
///
///         Updated for PC-LINUX-IFORT.
///
/// -    SPICELIB Version 2.3.0, 14-NOV-2006 (BVS)
///
///         Updated for MAC-OSX-IFORT.
///
/// -    SPICELIB Version 2.2.0, 14-NOV-2006 (BVS)
///
///         Updated for PC-WINDOWS-IFORT.
///
/// -    SPICELIB Version 2.1.0, 03-JAN-2005 (BVS)
///
///         Updated for PC-CYGWIN.
///
/// -    SPICELIB Version 2.0.5, 17-JUL-2002 (BVS)
///
///         Added MAC-OSX environments.
///
/// -    SPICELIB Version 2.0.4, 28-AUG-2000 (EDW)
///
///         Included PROMPT call for MAC Absoft environment.
///         Without something to pause execution, the output window
///         collapses at program end.
///
/// -    SPICELIB Version 2.0.3, 08-OCT-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are WIN-NT
///
/// -    SPICELIB Version 2.0.2, 28-JUL-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. New
///         environments are PC-DIGITAL, SGI-O32 and SGI-N32.
///
///         The success flag for SGI-N32 is '0', failure '1'. These
///         were simply 0 and 1 respectively when the environment was
///         just SGI.
///
/// -    SPICELIB Version 2.0.1, 18-MAR-1999 (WLT)
///
///         The environment lines were expanded so that the supported
///         environments are now explicitly given. Previously,
///         environments such as SUN-SUNOS and SUN-SOLARIS were implied
///         by the environment label SUN.
///
/// -    SPICELIB Version 2.0.0, 05-APR-1998 (NJB)
///
///         Added the PC-LINUX environment.
///
/// -    SPICELIB Version 1.0.0, 25-APR-1996 (KRG)
/// ```
pub fn byebye(ctx: &mut SpiceContext, status: &str) -> crate::Result<()> {
    BYEBYE(status.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BYEBYE ( Exit a program indicating an error status )
pub fn BYEBYE(STATUS: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CHSTAT = [b' '; STLEN as usize];
    let mut MYSTAT: bool = false;

    //
    // Local Parameters
    //
    // Define the maximum length for the usable portion of a status
    // string.
    //
    //
    // Define the notion of 'SUCCESS' and 'FAILURE'.
    //

    LJUST(STATUS, &mut CHSTAT);
    UCASE(&CHSTAT.clone(), &mut CHSTAT, ctx);

    if fstr::eq(&CHSTAT, b"SUCCESS") {
        MYSTAT = true;
    } else {
        MYSTAT = false;
    }

    if MYSTAT {
        ctx.exit(&[SUCCES])?;
    } else {
        ctx.exit(&[FAILUR])?;
    }

    //
    // We never really get here, but for pedantic reasons we must
    // have a RETURN statement in a subroutine.
    //
    Ok(())
}
