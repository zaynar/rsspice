//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    SVSTEP: f64,
    SVINIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVSTEP: f64 = 0.0;
        let mut SVINIT: bool = false;

        SVINIT = false;
        SVSTEP = -1.0;

        Self { SVSTEP, SVINIT }
    }
}

/// GF, step size
///
/// Return the time step set by the most recent call to GFSSTP.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TIME       I   Ignored ET value.
///  STEP       O   Time step to take.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TIME     is an ignored double precision number. This argument
///           is present so the argument list of this routine is
///           compatible with the GF step size routine argument list
///           specification.
///
///           When this routine is called from within the GF
///           root-finding system, either the initial ET value of the
///           current interval of the confinement window, or the
///           value resulting from the last search step, is passed in
///           via the TIME argument.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STEP     is the output step size. This is the value set by the
///           most recent call to GFSSTP. Units are TDB seconds.
///
///           STEP is used in the GF search root-bracketing process.
///           STEP indicates how far to advance TIME so that TIME and
///           TIME+STEP may bracket a state transition and definitely
///           do not bracket more than one state transition.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called before a step size has been
///      set via a call to GFSSTP, the error SPICE(NOTINITIALIZED)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine returns the time step set by the most recent call to
///  GFSSTP.
/// ```
///
/// # Examples
///
/// ```text
///  1) In normal usage of a high-level GF API routine, the caller
///     will pass in a constant step size STEP. The API routine will
///     then make the call
///
///        CALL GFSSTP ( STEP )
///
///     Subsequent calls to GFSTEP during the search process conducted
///     by the API routine will return STEP.
///
///
///  2) User applications can pass GFSTEP to mid-level GF API routines
///     expecting a step size routine as an input argument. For
///     example, the GF API routine GFOCCE can be called as follows:
///
///
///        Set the step size.
///
///        CALL GFSSTP ( STEP )
///
///
///        Look for solutions. (GFSTEP is the 11th argument.)
///
///        CALL GFOCCE ( OCCTYP,  FRONT,   FSHAPE,  FFRAME,
///       .              BACK,    BSHAPE,  BFRAME,  ABCORR,
///       .              OBSRVR,  CNVTOL,  GFSTEP,  GFREFN,
///       .              RPT,     GFREPI,  GFREPU,  GFREPF,
///       .              BAIL,    GFBAIL,  CNFINE,  RESULT )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 31-AUG-2010 (EDW)
///
///         Expanded error message on STEP for clarity.
///
///         Added TIME = TIME declaration to eliminate unused dummy
///         variable warning during compilation.
///
/// -    SPICELIB Version 1.0.0, 05-MAR-2009 (NJB) (LSE) (IMU) (WLT) (EDW)
/// ```
pub fn gfstep(ctx: &mut SpiceContext, time: &mut f64, step: &mut f64) -> crate::Result<()> {
    GFSTEP(time, step, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFSTEP ( GF, step size )
pub fn GFSTEP(TIME: &mut f64, STEP: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Discovery check-in.
    //
    if !save.SVINIT {
        CHKIN(b"GFSTEP", ctx)?;
        SETMSG(b"Step size was never initialized.", ctx);
        SIGERR(b"SPICE(NOTINITIALIZED)", ctx)?;
        CHKOUT(b"GFSTEP", ctx)?;
        return Ok(());
    }

    //
    // Set STEP to the saved value from the last call to GFSSTP.
    //
    *STEP = save.SVSTEP;
    *TIME = *TIME;

    Ok(())
}

/// Geometry finder set step size
///
/// Set the step size to be returned by GFSTEP.
///
/// # Required Reading
///
/// * [GF](crate::required_reading::gf)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STEP       I   Time step to take.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STEP     is the output step size to be returned by the next call
///           GFSTEP. Units are TDB seconds.
///
///           STEP is used in the GF search root-bracketing process.
///           STEP indicates how far to advance TIME so that TIME and
///           TIME+STEP may bracket a state transition and definitely
///           do not bracket more than one state transition.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input step size is non-positive, the error
///      SPICE(INVALIDSTEP) is signaled. The stored step value
///      is not updated.
/// ```
///
/// # Particulars
///
/// ```text
///  See the header of GFSTEP above.
/// ```
///
/// # Examples
///
/// ```text
///  See the header of GFSTEP above.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine must be called before the first time
///      GFSTEP is called during a program run.
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
/// -    SPICELIB Version 1.1.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 31-AUG-2010 (EDW)
///
///         Expanded error message on STEP for clarity.
///
/// -    SPICELIB Version 1.0.0, 15-APR-2009 (LSE) (NJB)
/// ```
pub fn gfsstp(ctx: &mut SpiceContext, step: f64) -> crate::Result<()> {
    GFSSTP(step, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GFSSTP ( Geometry finder set step size )
pub fn GFSSTP(STEP: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Check the step size.
    //
    if (STEP <= 0.0) {
        CHKIN(b"GFSSTP", ctx)?;
        SETMSG(b"Step has value #; step size must be positive.", ctx);
        ERRDP(b"#", STEP, ctx);
        SIGERR(b"SPICE(INVALIDSTEP)", ctx)?;
        CHKOUT(b"GFSSTP", ctx)?;
        return Ok(());
    }

    save.SVSTEP = STEP;
    save.SVINIT = true;

    Ok(())
}
