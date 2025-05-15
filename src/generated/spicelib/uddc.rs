//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Derivative of function less than zero, df(x)/dx \< 0
///
/// Return .TRUE. if the derivative of the callback function UDFUNC
/// at a given abscissa value is negative.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UDFUNC     I   The routine that computes the scalar value
///                 of interest.
///  X          I   Independent variable of UDFUNC.
///  DX         I   Interval from X for derivative calculation.
///  ISDECR     O   Boolean indicating if the derivative is negative.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UDFUNC   is the routine that returns the value of the scalar
///           quantity function of interest at X. The calling
///           sequence for UDFUNC is:
///
///              CALL UDFUNC ( X, VALUE )
///
///           where:
///
///              X       the double precision value of the
///                      independent variable of the function
///                      at which to determine the scalar value.
///
///              VALUE   the double precision value returned by
///                      UDFUNC at X.
///
///           Functionally:
///
///              VALUE = UDFUNC ( X )
///
///  X        is a scalar double precision value at which to determine
///           the derivative of UDFUNC.
///
///           For many SPICE uses, X will represent ephemeris time,
///           expressed as seconds past J2000 TDB.
///
///  DX       is a scalar double precision value representing half the
///           interval in units of X separating the evaluation
///           values of UDFUNC; the evaluations occur at (X + DX)
///           and (X - DX).
///
///           DX may be negative but must be non-zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ISDECR   is a scalar boolean indicating if the first derivative
///           of UDFUNC with respect to the independent variable
///           at X is less than zero.
///
///           Functionally:
///
///                         d UDFUNC(x) |
///              ISDECR =   ----------- |  <  0
///                              dx     |
///                                      X
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If DX has a value of zero, an error is signaled by a routine
///      in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  If the evaluation of UDFUNC requires SPICE kernel data, the
///  appropriate kernels must be loaded before calling this routine.
///
///  -  SPK data: the calling application must load ephemeris data
///     for the targets, observer, and any intermediate objects in
///     a chain connecting the targets and observer for the time
///     used in the evaluation. If aberration corrections are
///     used, the states of target and observer relative to the
///     solar system barycenter must be calculable from the
///     available ephemeris data.
///
///  -  If non-inertial reference frames are used, then PCK
///     files, frame kernels, C-kernels, and SCLK kernels may be
///     needed.
///
///  Such kernel data are normally loaded once per program run, NOT
///  every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine only wraps a UDDF call, examining the sign of the
///  derivative value returned by UDDF. Please refer to this routine
///  for further details.
/// ```
///
/// # Examples
///
/// ```text
///  See GFUDS.
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
/// -    SPICELIB Version 1.0.0, 31-MAR-2010 (EDW) (NJB)
/// ```
pub fn uddc(
    ctx: &mut SpiceContext,
    udfunc: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    x: f64,
    dx: f64,
    isdecr: &mut bool,
) -> crate::Result<()> {
    UDDC(udfunc, x, dx, isdecr, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure UDDC ( Derivative of function less than zero, df(x)/dx < 0 )
pub fn UDDC(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    X: f64,
    DX: f64,
    ISDECR: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DERIV: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"UDDC", ctx)?;

    *ISDECR = false;

    //
    // Numerically calculate the derivative of UDFUNC at X.
    //
    UDDF(UDFUNC, X, DX, &mut DERIV, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"UDDC", ctx)?;
        return Ok(());
    }

    *ISDECR = (DERIV < 0.0);

    CHKOUT(b"UDDC", ctx)?;
    Ok(())
}
