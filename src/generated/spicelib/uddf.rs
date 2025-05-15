//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// First derivative of a function, df(x)/dx
///
/// Calculate the first derivative of a caller-specified scalar
/// function using a three-point estimation.
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
///  DERIV      O   Approximate derivative of UDFUNC at X.
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
///  DERIV    is the scalar double precision approximate value of the
///           first derivative of UDFUNC with respect to X.
///
///           Functionally:
///
///                       d UDFUNC ( y )  |
///              DERIV = ---------------- |
///                             dy        | y=X
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
///  This routine provides a simple interface to numerically calculate
///  the first derivative of a scalar quantity function, UDFUNC.
///  UDFUNC is expected to be "well behaved" across at the evaluation
///  interval [ X - DX, X + DX ]. This means a linear approximation to
///  the function over the interval is sufficiently accurate to
///  calculate the approximate derivative at X.
///
///  The routine QDERIV performs the differentiation using a three
///  point estimation. See the header of the SPICE routine QDERIV for
///  details of the discrete derivative computation performed by this
///  routine.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Calculate the time derivative of the light time corresponding
///     to the apparent position of Mercury relative to the Moon at
///     time "JAN 1 2009."
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: uddf_ex1.tm
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
///           File name                     Contents
///           ---------                     --------
///           de421.bsp                     Planetary ephemeris
///           naif0009.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de421.bsp',
///                               'naif0009.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM UDDF_EX1
///           IMPLICIT NONE
///
///           EXTERNAL                 UDFUNC
///
///           DOUBLE PRECISION         ET
///           DOUBLE PRECISION         DT
///           DOUBLE PRECISION         DERIV
///
///     C
///     C     Load leapsecond and SPK kernels. The name of the
///     C     meta kernel file shown here is fictitious; you
///     C     must supply the name of a file available
///     C     on your own computer system.
///     C
///           CALL FURNSH ( 'uddf_ex1.tm' )
///
///     C
///     C     Use a shift of one second off the epoch of interest.
///     C
///           DT = 1.D0
///
///     C
///     C     Convert the epoch date string to ephemeris seconds.
///     C
///           CALL STR2ET ( 'JAN 1 2009', ET )
///
///     C
///     C     Calculate the derivative of UDFUNC at ET.
///     C
///           CALL UDDF ( UDFUNC, ET, DT, DERIV )
///
///     C
///     C     Output the calculated derivative.
///     C
///           WRITE(*,*) DERIV
///
///           END
///
///     C
///     C     A scalar quantity function that returns the light-time
///     C     between the Moon and Mercury at ET.
///     C
///           SUBROUTINE UDFUNC ( ET, VALUE )
///
///           IMPLICIT NONE
///
///           DOUBLE PRECISION         ET
///           DOUBLE PRECISION         VALUE
///
///           DOUBLE PRECISION         POS  (3)
///           DOUBLE PRECISION         LT
///
///     C
///     C     Evaluate the apparent position of Mercury with respect
///     C     to the Moon at ET.
///     C
///           CALL SPKPOS ( 'MERCURY', ET, 'J2000', 'LT+S', 'MOON',
///          .               POS,   LT )
///
///     C
///     C     Return the light-time value as the scalar quantity.
///     C
///           VALUE = LT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///       -1.3567094055133566E-004
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The function UDFUNC must exist everywhere within
///      [X - DX, X + DX].
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
/// -    SPICELIB Version 1.0.1, 05-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Included
///         required meta-kernel and added IMPICIT NONE statement to code
///         example.
///
///         Moved reference to QDERIV header from $Literature_References to
///         $Particulars with description of what it is expected from that
///         header.
///
/// -    SPICELIB Version 1.0.0, 31-MAR-2010 (EDW) (NJB)
/// ```
pub fn uddf(
    ctx: &mut SpiceContext,
    udfunc: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    x: f64,
    dx: f64,
    deriv: &mut f64,
) -> crate::Result<()> {
    UDDF(udfunc, x, dx, deriv, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure UDDF ( First derivative of a function, df(x)/dx )
pub fn UDDF(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    X: f64,
    DX: f64,
    DERIV: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut N: i32 = 0;
    let mut DFDX = StackArray::<f64, 1>::new(1..=1);
    let mut UDVAL = StackArray::<f64, 2>::new(1..=2);

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"UDDF", ctx)?;

    //
    // Apply a three-point estimation of the derivative for
    // UDFUNC at X by evaluating UDFUNC at [X-DX, X+DX].
    //
    // The QDERIV call returns a single value in the DFDX array.
    //
    N = 1;

    //
    // Evaluate the scalar function at the interval boundaries.
    // Check for a FAILED event.
    //
    UDFUNC(&mut (X - DX).clone(), &mut UDVAL[1], ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"UDDF", ctx)?;
        return Ok(());
    }

    UDFUNC(&mut (X + DX).clone(), &mut UDVAL[2], ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"UDDF", ctx)?;
        return Ok(());
    }

    //
    // Estimate the derivative at X.
    //
    QDERIV(
        N,
        UDVAL.subarray(1),
        UDVAL.subarray(2),
        DX,
        DFDX.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"UDDF", ctx)?;
        return Ok(());
    }

    *DERIV = DFDX[1];

    CHKOUT(b"UDDF", ctx)?;
    Ok(())
}
