//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const FILEN: i32 = 255;

struct SaveVars {
    SAVDEV: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SAVDEV = vec![b' '; FILEN as usize];

        fstr::assign(&mut SAVDEV, b"SCREEN");

        Self { SAVDEV }
    }
}

/// Store Error Output Device Specification
///
/// PUTDEV is a low-level data structure access routine which stores
/// the error output device specification.  DO NOT CALL THIS ROUTINE.
/// USE ERRDEV, NOT PUTDEV, TO CHOOSE THE ERROR OUTPUT DEVICE.
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
///  DEVICE     I   The error output device specification.
///  FILEN      P   The maximum length of a file name.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DEVICE   is the new value of the error output device
///           specification. This value will be saved.
/// ```
///
/// # Parameters
///
/// ```text
///  FILEN    is the maximum length of a file name.
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
///  DO NOT CALL THIS ROUTINE.
///
///  This is a data structure access routine for the SPICELIB
///  error output device specification. This routine should
///  be used for no other purpose; in particular, it should
///  not be used by non-toolkit routines to specify the error
///  error output device to be used by the toolkit. Use ERRDEV
///  for that.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  DO NOT CALL THIS ROUTINE.
///
///  2)  Calls to this routine by routines other than the SPICELIB
///      error handling routines may interfere with error processing.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
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
///         Added IMPLICT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry.
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
/// -    SPICELIB Version 3.0.0, 07-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 2.1.0, 05-JAN-1995 (HAN)
///
///         Module was updated to include one declaration for
///         the variable FILEN for the Macintosh environment.
///
/// -    SPICELIB Version 2.0.0, 09-NOV-1993 (HAN)
///
///         Module was updated to include the value for FILEN
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms. Also, the previous value of 256 for
///         Unix platforms was changed to 255.
///
/// -    SPICELIB Version 1.1.0, 12-OCT-1992 (HAN)
///
///         Updated module for multiple environments.
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 07-APR-1998 (NJB)
///
///         Module was updated for the PC-LINUX platform.
///
/// -    SPICELIB Version 2.0.0, 9-NOV-1993 (HAN)
///
///         Module was updated to include the value for FILEN
///         for the Silicon Graphics, DEC Alpha-OSF/1, and
///         NeXT platforms. Also, the previous value of 256 for
///         Unix platforms was changed to 255.
///
/// -    SPICELIB Version 1.1.0, 12-OCT-1992 (HAN)
///
///         Updated module for multiple environments.
///
///         The code was also reformatted so that a utility program can
///         create the source file for a specific environment given a
///         master source file.
///
/// -    Beta Version 1.0.1, 08-FEB-1989 (NJB)
///
///         Warnings added to discourage use of this routine in
///         non-error-handling code. $Parameters section added.
///         Parameter declarations moved to "Declarations" section.
/// ```
pub fn putdev(ctx: &mut SpiceContext, device: &str) {
    PUTDEV(device.as_bytes(), ctx.raw_context());
}

//$Procedure PUTDEV ( Store Error Output Device Specification )
pub fn PUTDEV(DEVICE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Local Variables:
    //

    //
    // The current error output device specification:
    //

    //
    // Initial values:
    //

    //
    // Executable Code:
    //

    fstr::assign(&mut save.SAVDEV, DEVICE);
}

/// Get Error Output Device Specification
///
/// Return the value of the current error output device
/// specification.
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
///  DEVICE     O   The error output device specification.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DEVICE   is the current error output device specification.
///           See the "required reading" file for a detailed
///           discussion of the error output device.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  However, this routine is part of the SPICELIB error
///      handling mechanism.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 03-JUN-2021 (JDR)
///
///         Added IMPLICT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Index_Entries entry.
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
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB)
/// ```
pub fn getdev(ctx: &mut SpiceContext, device: &mut str) {
    GETDEV(fstr::StrBytes::new(device).as_mut(), ctx.raw_context());
}

//$Procedure GETDEV ( Get Error Output Device Specification )
pub fn GETDEV(DEVICE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Executable Code:
    //

    //
    // Grab saved error output device specification:
    //

    fstr::assign(DEVICE, &save.SAVDEV);
}
