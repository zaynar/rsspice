//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// GF, default bailout function
///
/// Serve as a placeholder for an interrupt detection function.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
///
/// # Brief I/O
///
/// ```text
///  The function returns the value .FALSE. in all cases.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .FALSE. in all cases.
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
///  This routine serves as a stub for interrupt function input
///  arguments in GF mid-level search routines such as
///
///     GFEVNT
///     GFOCCE
///     GFFOVE
///
///  Those routines allow the caller to pass in a custom interrupt
///  detection function.
///
///  Searches conducted with the GF APIs can be unexpectedly
///  time-consuming. When such searches are carried out by an
///  interactive application, it can be useful to be able to stop a
///  search without stopping the application itself. This enables a
///  user to avoid loss of previous work that may have been performed
///  during the program run.
///
///  The mid-level GF search APIs named above provide partial support
///  for interrupt handling. They allow the caller to pass in an
///  interrupt detection function; when their input "bail-out" flag
///  argument is set to .TRUE. by the caller, the low-level GF
///  root-finding routines invoked by these APIs will, over regular,
///  short time intervals (these intervals are usually determined by
///  the completion of loop passes), call the interrupt detection
///  function. These routines will return immediately if the function
///  indicates that an interrupt has occurred.
///
///  However, SPICELIB doesn't fully support interrupt handling
///  because ANSI Fortran 77 doesn't provide the features necessary to
///  implement an interrupt detection function.
///
///  Some Fortran platforms do provide non-standard routines that
///  support interrupt handling, so for these systems, SPICE users may
///  be able to create their own interrupt detection routines. Such
///  routines should have calling sequences identical to that of this
///  function. These routines should have a "reset" feature that
///  enables an application to make them return .FALSE. after an
///  interrupt has been indicated and processed.
///
///  For platforms where interrupt detection can't be implemented, or
///  in cases where applications must call mid-level GF APIs but don't
///  need interrupt handling, this routine can be used.
///
///  This routine has no interrupt detection capability: it always
///  returns the value .FALSE.
///
///  Developers of SPICE-based applications who have the choice of
///  writing code in Fortran or C may wish to consider the fact that
///  the CSPICE Toolkit does support interrupt detection: gfbail_c,
///  the CSPICE analog of this routine, is fully functional on all
///  platforms on which CSPICE is supported.
/// ```
///
/// # Examples
///
/// ```text
///  This example shows how to call a mid-level GF search API that
///  requires an input interrupt detection function.
///
///  If a custom interrupt detection function is available, it
///  can be referenced exactly as is GFBAIL in this example.
///
///  The code fragment below is from the first code example in the
///  header of
///
///     gfocce.for
///
///  Only the portions of that program relevant to use of GFBAIL are
///  copied here. Deleted portions of code are indicated by ellipses.
///
///  Note that GFBAIL is the third-to-last argument in the
///  GFOCCE call.
///
///
///           PROGRAM EX1
///
///           IMPLICIT NONE
///
///           ...
///
///           LOGICAL               GFBAIL
///           EXTERNAL              GFBAIL
///
///           ...
///
///     C
///     C     Turn on progress reporting; turn off interrupt
///     C     handling.
///     C
///
///           ...
///
///           BAIL = .FALSE.
///
///     C
///     C     Perform the search.
///     C
///           CALL GFOCCE ( 'ANY',
///          .              'MOON',   'ellipsoid',  'IAU_MOON',
///          .              'SUN',    'ellipsoid',  'IAU_SUN',
///          .              'LT',     'EARTH',      CNVTOL,
///          .              GFSTEP,   GFREFN,       RPT,
///          .              GFREPI,   GFREPU,       GFREPF,
///          .              BAIL,     GFBAIL,       CNFINE,  RESULT )
///
///
///          ...
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This is a stub routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 23-JUN-2010 (EDW)
///
///         Minor edit to $Declarations for correct header format.
///
/// -    SPICELIB Version 1.0.0, 05-MAR-2009 (NJB) (LSE) (EDW)
/// ```
pub fn gfbail() -> bool {
    let ret = GFBAIL();
    ret
}

//$Procedure GFBAIL ( GF, default bailout function )
pub fn GFBAIL() -> bool {
    let mut GFBAIL: bool = false;

    GFBAIL = false;

    GFBAIL
}
