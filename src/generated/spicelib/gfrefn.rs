//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// GF, default refinement estimator
///
/// Estimate, using a bisection method, the next abscissa value at
/// which a state change occurs. This is the default GF refinement
/// method.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  T1         I   One of two values bracketing a state change.
///  T2         I   The other value that brackets a state change.
///  S1         I   State at T1.
///  S2         I   State at T2.
///  T          O   New value at which to check for transition.
/// ```
///
/// # Detailed Input
///
/// ```text
///  T1       is one of two abscissa values (usually times)
///           bracketing a state change.
///
///  T2       is the other abscissa value that brackets a state change.
///
///  S1       is the system state at T1. This argument is provided
///           for forward compatibility; it's not currently used.
///
///  S2       is the system state at T2. This argument is provided
///           for forward compatibility; it's not currently used.
/// ```
///
/// # Detailed Output
///
/// ```text
///  T        is the midpoint of T1 and T2.
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
///  "Refinement" means reducing the size of a bracketing interval on
///  the real line in which a solution is known to lie. In the GF
///  setting, the solution is the time of a state transition of a
///  binary function.
///
///  This routine supports solving for locations of bracketed state
///  transitions by the bisection method. This is the default
///  refinement method used by the GF system.
///
///  The argument list of this routine is compatible with the GF
///  system's general root finding routine. Refinement routines created
///  by users must have the same argument list in order to be used by
///  the GF mid-level APIs such as GFOCCE and GFFOVE.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment from an example program in the header
///  of GFOCCE shows the routine passed as the 12th argument.
///
///     C
///     C     Define as EXTERNAL the routines to pass to GFOCCE.
///     C
///           EXTERNAL              GFSTEP
///           EXTERNAL              GFREFN
///           EXTERNAL              GFREPI
///           EXTERNAL              GFREPU
///           EXTERNAL              GFREPF
///           EXTERNAL              GFBAIL
///
///              ... initialize for the search ...
///
///           CALL GFOCCE ( 'ANY',
///          .              'MOON',   'ellipsoid',  'IAU_MOON',
///          .              'SUN',    'ellipsoid',  'IAU_SUN',
///          .              'LT',     'EARTH',       CNVTOL,
///          .               GFSTEP,   GFREFN,       RPT,
///          .               GFREPI,   GFREPU,       GFREPF,
///          .               BAIL,     GFBAIL,       CNFINE,  RESULT )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 03-MAR-2009 (NJB) (EDW)
/// ```
pub fn gfrefn(t1: f64, t2: f64, s1: bool, s2: bool, t: &mut f64) {
    GFREFN(t1, t2, s1, s2, t);
}

//$Procedure GFREFN ( GF, default refinement estimator )
pub fn GFREFN(T1: f64, T2: f64, S1: bool, S2: bool, T: &mut f64) {
    let mut X: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    X = ((T1 * 0.5) + (T2 * 0.5));

    *T = BRCKTD(X, T1, T2);
}
