//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CLOSE: f64 = 0.0000000001;

struct SaveVars {
    ZVEC: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ZVEC = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            ZVEC.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ZVEC }
    }
}

/// Determine conic elements from state
///
/// Determine the set of osculating conic orbital elements that
/// corresponds to the state (position, velocity) of a body at
/// some epoch.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STATE      I   State of body at epoch of elements.
///  ET         I   Epoch of elements.
///  MU         I   Gravitational parameter (GM) of primary body.
///  ELTS       O   Equivalent conic elements.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STATE    is the state (position and velocity) of the body
///           at some epoch. Components are x, y, z, dx/dt, dy/dt,
///           dz/dt. STATE must be expressed relative to an
///           inertial reference frame. Units are km and km/sec.
///
///
///  ET       is the epoch of the input state, in ephemeris seconds
///           past J2000.
///
///                                                 3    2
///  MU       is the gravitational parameter (GM, km /sec ) of
///           the primary body.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ELTS     are equivalent conic elements describing the orbit
///           of the body around its primary. The elements are,
///           in order:
///
///              RP      Perifocal distance.
///              ECC     Eccentricity.
///              INC     Inclination.
///              LNODE   Longitude of the ascending node.
///              ARGP    Argument of periapsis.
///              M0      Mean anomaly at epoch.
///              T0      Epoch.
///              MU      Gravitational parameter.
///
///           The epoch of the elements is the epoch of the input
///           state. Units are km, rad, rad/sec. The same elements
///           are used to describe all three types (elliptic,
///           hyperbolic, and parabolic) of conic orbit.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If MU is not positive, the error SPICE(NONPOSITIVEMASS)
///      is signaled.
///
///  2)  If the specific angular momentum vector derived from STATE
///      is the zero vector, the error SPICE(DEGENERATECASE)
///      is signaled.
///
///  3)  If the position or velocity vectors derived from STATE
///      is the zero vector, the error SPICE(DEGENERATECASE)
///      is signaled.
///
///  4)  If the inclination is determined to be zero or 180 degrees,
///      the longitude of the ascending node is set to zero.
///
///  5)  If the eccentricity is determined to be zero, the argument of
///      periapse is set to zero.
///
///  6)  If the eccentricity of the orbit is very close to but not
///      equal to zero, the argument of periapse may not be accurately
///      determined.
///
///  7)  For inclinations near but not equal to 0 or 180 degrees,
///      the longitude of the ascending node may not be determined
///      accurately. The argument of periapse and mean anomaly may
///      also be inaccurate.
///
///  8)  For eccentricities very close to but not equal to 1, the
///      results of this routine are unreliable.
///
///  9)  If the specific angular momentum vector is non-zero but
///      "close" to zero, the results of this routine are unreliable.
///
///  10) If STATE is expressed relative to a non-inertial reference
///      frame, the resulting elements are invalid. No error checking
///      is done to detect this problem.
/// ```
///
/// # Particulars
///
/// ```text
///  The SPICELIB routine CONICS is the inverse of this routine:
///  CONICS maps a set of osculating elements and a time to a state
///  vector.
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
///  1) Determine the osculating elements of Phobos with respect to
///     Mars at some arbitrary time in the J2000 inertial reference
///     frame.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: oscelt_ex1.tm
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
///           mar097.bsp                    Mars satellite ephemeris
///           gm_de431.tpc                  Gravitational constants
///           naif0012.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'mar097.bsp',
///                               'gm_de431.tpc',
///                               'naif0012.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM OSCELT_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      ELTS   (8)
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      MU     (1)
///           DOUBLE PRECISION      STATE  (6)
///           DOUBLE PRECISION      STEP
///
///           INTEGER               DIM
///
///     C
///     C     Load the meta kernel listing the needed SPK, LSK and
///     C     PCK with gravitational parameters kernels.
///     C
///           CALL FURNSH ( 'oscelt_ex1.tm' )
///
///     C
///     C     Convert the time string to ephemeris time
///     C
///           CALL STR2ET ( 'Dec 25, 2007', ET )
///
///     C
///     C     Retrieve the state of Phobos with respect to Mars in
///     C     J2000.
///     C
///           CALL SPKEZR ( 'PHOBOS', ET, 'J2000', 'NONE', 'MARS',
///          .               STATE,   LT                          )
///
///     C
///     C     Read the gravitational parameter for Mars.
///     C
///           CALL BODVRD ( 'MARS', 'GM', 1, DIM, MU )
///
///     C
///     C     Convert the state 6-vector to the elts 8-vector. Note:
///     C     BODVRD returns data as arrays, so to access the
///     C     gravitational parameter (the only value in the array),
///     C     we use MU(1).
///     C
///           CALL OSCELT ( STATE, ET, MU(1), ELTS )
///
///     C
///     C     Output the elts vector.
///     C
///           WRITE(*,'(A,F21.10)')
///          .  'Perifocal distance          (km): ', ELTS(1)
///           WRITE(*,'(A,F21.10)')
///          .  'Eccentricity                    : ', ELTS(2)
///           WRITE(*,'(A,F21.10)')
///          .  'Inclination                (deg): ', ELTS(3) * DPR( )
///           WRITE(*,'(A,F21.10)')
///          .  'Lon of ascending node      (deg): ', ELTS(4) * DPR( )
///           WRITE(*,'(A,F21.10)')
///          .  'Argument of periapsis      (deg): ', ELTS(5) * DPR( )
///           WRITE(*,'(A,F21.10)')
///          .  'Mean anomaly at epoch      (deg): ', ELTS(6) * DPR( )
///           WRITE(*,'(A,F21.10)')
///          .  'Epoch                        (s): ', ELTS(7)
///           WRITE(*,'(A,F21.10)')
///          .  'Gravitational parameter (km3/s2): ', ELTS(8)
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Perifocal distance          (km):       9232.5746716211
///     Eccentricity                    :          0.0156113904
///     Inclination                (deg):         38.1225231660
///     Lon of ascending node      (deg):         47.0384055902
///     Argument of periapsis      (deg):        214.1546430017
///     Mean anomaly at epoch      (deg):        340.5048466068
///     Epoch                        (s):  251812865.1837092042
///     Gravitational parameter (km3/s2):      42828.3736206991
///
///
///  2) Calculate the history of Phobos's orbit plane inclination
///     with respect to Mars in the J2000 frame at intervals of six
///     months for a time interval of 10 years.
///
///     Use the meta-kernel from the first example.
///
///
///     Example code begins here.
///
///
///           PROGRAM OSCELT_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      SPD
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 24 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(TIMLEN)    UTCSTR
///
///           DOUBLE PRECISION      ELTS   (8)
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      MU     (1)
///           DOUBLE PRECISION      STATE  (6)
///           DOUBLE PRECISION      STEP
///
///           INTEGER               DIM
///           INTEGER               I
///
///     C
///     C     Load the meta kernel listing the needed SPK, LSK and
///     C     PCK with gravitational parameters kernels.
///     C
///           CALL FURNSH ( 'oscelt_ex1.tm' )
///
///     C
///     C     Read the gravitational parameter for Mars.
///     C
///           CALL BODVRD ( 'MARS', 'GM', 1, DIM, MU )
///
///     C
///     C     Convert the time string to ephemeris time
///     C
///           CALL STR2ET ( 'Jan 1, 2000 12:00:00', ET )
///
///     C
///     C     A step of six months - in seconds.
///     C
///           STEP = 180.0D0 * SPD( )
///
///     C
///     C     10 years in steps of six months starting
///     C     approximately Jan 1, 2000.
///     C
///           WRITE(*,'(A)') '        UCT Time          Inclination'
///           WRITE(*,'(A)') '------------------------  -----------'
///
///           DO I= 1, 20
///
///     C
///     C        Retrieve the state; convert to osculating elements.
///     C
///              CALL SPKEZR ( 'PHOBOS', ET, 'J2000', 'NONE', 'MARS',
///          .                  STATE,   LT                          )
///              CALL OSCELT ( STATE, ET, MU(1), ELTS )
///
///     C
///     C        Convert the ephemeris time to calendar UTC.
///     C
///              CALL ET2UTC ( ET, 'C', 3, UTCSTR )
///
///              WRITE(*,'(A,X,F12.6)') UTCSTR,  ELTS(3) * DPR( )
///
///              ET = ET + STEP
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///             UCT Time          Inclination
///     ------------------------  -----------
///     2000 JAN 01 12:00:00.000    36.055248
///     2000 JUN 29 12:00:00.000    37.112144
///     2000 DEC 26 12:00:00.000    38.152129
///     2001 JUN 24 12:00:00.000    37.552071
///     2001 DEC 21 12:00:00.000    36.242049
///     2002 JUN 19 11:59:59.999    36.330470
///     2002 DEC 16 12:00:00.000    37.674595
///     2003 JUN 14 11:59:59.999    38.121191
///     2003 DEC 11 12:00:00.001    36.973204
///     2004 JUN 08 11:59:59.999    36.033732
///     2004 DEC 05 12:00:00.001    36.844542
///     2005 JUN 03 11:59:59.999    38.077365
///     2005 NOV 30 12:00:00.001    37.786106
///     2006 MAY 29 11:59:58.999    36.413540
///     2006 NOV 25 11:59:59.001    36.171050
///     2007 MAY 24 11:59:58.999    37.448015
///     2007 NOV 20 11:59:59.001    38.189118
///     2008 MAY 18 11:59:58.999    37.223573
///     2008 NOV 14 11:59:59.001    36.084745
///     2009 MAY 13 11:59:57.999    36.608971
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The input state vector must be expressed relative to an
///      inertial reference frame.
///
///  2)  Osculating elements are generally not useful for
///      high-accuracy work.
///
///  3)  Accurate osculating elements may be difficult to derive for
///      near-circular or near-equatorial orbits. Osculating elements
///      for such orbits should be used with caution.
///
///  4)  Extracting osculating elements from a state vector is a
///      mathematically simple but numerically challenging task. The
///      mapping from a state vector to equivalent elements is
///      undefined for certain state vectors, and the mapping is
///      difficult to implement with finite precision arithmetic for
///      states near the subsets of R6 where singularities occur.
///
///      In general, the elements found by this routine can have
///      two kinds of problems:
///
///      -  The elements are not accurate but still represent
///         the input state accurately. The can happen in
///         cases where the inclination is near zero or 180
///         degrees, or for near-circular orbits.
///
///      -  The elements are garbage. This can occur when
///         the eccentricity of the orbit is close to but
///         not equal to 1. In general, any inputs that cause
///         great loss of precision in the computation of the
///         specific angular momentum vector or the eccentricity
///         vector will result in invalid outputs.
///
///      For further details, see the $Exceptions section.
///
///      Users of this routine should carefully consider whether
///      it is suitable for their applications. One recommended
///      "sanity check" on the outputs is to supply them to the
///      SPICELIB routine CONICS and compare the resulting state
///      vector with the one supplied to this routine.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  R. Bate, D. Mueller, and J. White, "Fundamentals of
///       Astrodynamics," Dover Publications Inc., 1971.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.2, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples to $Examples section.
///
/// -    SPICELIB Version 1.3.1, 28-FEB-2008 (NJB)
///
///         Updated $Index_Entries header section to use keywords
///         "osculating" and "convert." Updated $Particulars header
///         section to refer to CONICS. Fixed typo in in-line
///         comments.
///
/// -    SPICELIB Version 1.3.0, 17-NOV-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL call.
///
///         The $Exceptions and $Restrictions header sections were updated.
///
/// -    SPICELIB Version 1.2.0, 28-JAN-2003 (NJB) (EDW)
///
///         Bug fixes: routine previously didn't correctly compute
///         the argument of periapse or mean anomaly for some cases.
///         Also, the arguments of the ACOS and DACOSH functions were
///         able to go out of range, causing floating-point exceptions.
///
///         The computations of M0 and INC were re-coded for improved
///         accuracy.
///
///         Also, added error checks for non-positive MU, zero
///         position, velocity, and specific angular momentum vectors.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
/// -    SPICELIB Version 1.0.2, 06-APR-1995 (WLT)
///
///         A typo was fixed in the description of the node vector
///         in the comments of the routine.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.3.0, 17-NOV-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL call.
///
/// -    SPICELIB Version 1.2.0, 28-JAN-2003 (NJB) (EDW)
///
///         Bug fixes: routine previously didn't correctly compute
///         the argument of periapse or mean anomaly for some cases.
///         Also, the arguments of the ACOS and DACOSH functions were
///         able to go out of range, causing floating-point exceptions.
///
///         The old computation of ARGP did not work for cases where
///         the inclination was 0 or pi: the sign of ARGP was sometimes
///         incorrect.
///
///         The new method uses the criterion: for inclination zero or pi
///         the argument of periapse is between zero and pi radians when
///
///            e  *  ( h x n )  >  0
///            -       -   -    -
///
///         where
///
///            e  is the eccentricity vector,
///            -
///
///            h  is the specific angular momentum vector,
///            -
///
///            n  is the node vector.
///            -
///
///         The computation of M0 was re-coded for improved accuracy.
///         The new computation uses ATAN2 rather than ACOS to find
///         the eccentric anomaly for the ellipse case. The quadrant
///         of M0 is now found by converting the position to the
///         perifocal frame and finding the corresponding longitude.
///
///         The old method, using the sign of <r,v>, did not work
///         for circular orbits and was unreliable for near-circular
///         orbits.
///
///         Inclination is now computed using VSEP.
///
///         Also, added error checks for non-positive MU, zero
///         position, velocity, and specific angular momentum vectors.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
/// -    SPICELIB Version 1.0.2, 6-APR-1995 (WLT)
///
///         A typo was fixed in the description of the node vector
///         in the comments of the routine.
///
/// -    Beta Version 1.0.1, 27-JAN-1989 (IMU)
///
///         $Examples section completed.
/// ```
pub fn oscelt(
    ctx: &mut SpiceContext,
    state: &[f64; 6],
    et: f64,
    mu: f64,
    elts: &mut [f64; 8],
) -> crate::Result<()> {
    OSCELT(state, et, mu, elts, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure OSCELT ( Determine conic elements from state )
pub fn OSCELT(
    STATE: &[f64],
    ET: f64,
    MU: f64,
    ELTS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STATE = DummyArray::new(STATE, 1..=6);
    let mut ELTS = DummyArrayMut::new(ELTS, 1..=8);
    let mut ARGP: f64 = 0.0;
    let mut COSEA: f64 = 0.0;
    let mut COSHF: f64 = 0.0;
    let mut E = StackArray::<f64, 3>::new(1..=3);
    let mut EA: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut H = StackArray::<f64, 3>::new(1..=3);
    let mut INC: f64 = 0.0;
    let mut LNODE: f64 = 0.0;
    let mut M0: f64 = 0.0;
    let mut N = StackArray::<f64, 3>::new(1..=3);
    let mut NU: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut PERIX = StackArray::<f64, 3>::new(1..=3);
    let mut PERIY = StackArray::<f64, 3>::new(1..=3);
    let mut R = StackArray::<f64, 3>::new(1..=3);
    let mut RMAG: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut SINEA: f64 = 0.0;
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut VMAG: f64 = 0.0;
    let mut XPROD = StackArray::<f64, 3>::new(1..=3);

    //
    // External functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"OSCELT", ctx)?;
    }

    if (MU <= 0.0) {
        SETMSG(b"MU = #; non-positive gravitational parameter", ctx);
        ERRDP(b"#", MU, ctx);
        SIGERR(b"SPICE(NONPOSITIVEMASS)", ctx)?;
        CHKOUT(b"OSCELT", ctx)?;
        return Ok(());
    }

    //
    // In order to convert a position and velocity to an equivalent
    // set of (osculating) orbital elements, we need to determine three
    // principal vectors associated with the orbit:
    //
    //    h         The angular momentum vector. This is perpendicular
    //    -         to the plane of the orbit.
    //
    //                    h = r X v
    //                    -   -   -
    //
    //    n         The node vector. This is perpendicular to the
    //    -         normals of both the reference and orbital planes;
    //              it lies in the intersection of these planes,
    //              pointing toward the ascending node.
    //
    //                         ^
    //                    n  = k X h  = ( -h , h , 0 )
    //                    -        -        y   x
    //
    //    e         The eccentricity vector. This lies in the plane
    //    -         of the orbit, and points toward periapse. The
    //              magnitude of this vector is the eccentricity.
    //
    //                                  2
    //                    e = (1/mu)( (v  - mu/r) r  -  <r,v> v )
    //                    -                       -      - -  -
    //
    VEQU(STATE.subarray(1), R.as_slice_mut());
    VEQU(STATE.subarray(4), V.as_slice_mut());

    //
    // Check for non-physical cases. Probably due to user
    // input error
    //
    if VZERO(R.as_slice()) {
        SETMSG(b"Zero vector for input position vector.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"OSCELT", ctx)?;
        return Ok(());
    }

    if VZERO(V.as_slice()) {
        SETMSG(b"Zero vector for input velocity vector.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"OSCELT", ctx)?;
        return Ok(());
    }

    RMAG = VNORM(R.as_slice());
    VMAG = VNORM(V.as_slice());

    VCRSS(R.as_slice(), V.as_slice(), H.as_slice_mut());

    //
    // If the specific angular momentum vector is the zero vector,
    // we have a degenerate orbit and cannot proceed.
    //
    if VZERO(H.as_slice()) {
        SETMSG(b"Input position and velocity are too close to parallel; the specific angular momentum vector is zero.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"OSCELT", ctx)?;
        return Ok(());
    }

    VPACK(-H[2], H[1], 0.0, N.as_slice_mut());

    VLCOM(
        (f64::powi(VMAG, 2) - (MU / RMAG)),
        R.as_slice(),
        -VDOT(R.as_slice(), V.as_slice()),
        V.as_slice(),
        E.as_slice_mut(),
    );
    VSCLIP((1.0 / MU), E.as_slice_mut());

    //
    // We begin by determining the size and shape of the orbit.
    //
    // The eccentricity of the orbit is the magnitude of the
    // eccentricity vector. If the eccentricity is "close" to one,
    // go ahead and make this a parabola.
    //
    // The perifocal distance depends on the eccentricity and the
    // semi-latus rectum, which in turn orbit depends only on the
    // specific angular momentum of the orbiting object.
    //

    ECC = EXACT(VNORM(E.as_slice()), 1.0, CLOSE);

    P = (VDOT(H.as_slice(), H.as_slice()) / MU);
    RP = (P / (1.0 + ECC));

    //
    // Next, the orientation of the orbit.
    //                                                   ^
    // The inclination of the orbit is the angle between k (which is
    // perpendicular to the equator) and h (which is perpendicular to
    // the orbit.                        -
    //
    // If close to zero or pi, make it exact. In either case, the node
    // vector becomes undefined.
    //

    INC = VSEP(H.as_slice(), save.ZVEC.as_slice(), ctx);

    if (f64::abs((INC - 0.0)) < CLOSE) {
        INC = 0.0;
        VPACK(1.0, 0.0, 0.0, N.as_slice_mut());
    } else if (f64::abs((INC - PI(ctx))) < CLOSE) {
        INC = PI(ctx);
        VPACK(1.0, 0.0, 0.0, N.as_slice_mut());
    }

    //
    //                                                          ^
    // The longitude of the ascending node is the angle between i
    // (the x-axis) and the node vector, n.
    //                                   -
    //
    LNODE = f64::atan2(N[2], N[1]);

    if (LNODE < 0.0) {
        LNODE = (LNODE + TWOPI(ctx));
    }

    //
    // The argument of periapsis is the angle between the node vector
    // n, and the eccentricity vector e. This is not defined for
    // -                              -
    // circular orbits.
    //
    //
    if (ECC == 0.0) {
        ARGP = 0.0;
    } else {
        //
        // Set the magnitude of ARGP; we'll determine the sign next.
        //
        ARGP = VSEP(N.as_slice(), E.as_slice(), ctx);

        if (ARGP != 0.0) {
            if ((INC == 0.0) || (INC == PI(ctx))) {
                //
                // The quadrant of ARGP is determined by the component of E
                // in the direction H x N.
                //
                UCRSS(H.as_slice(), N.as_slice(), XPROD.as_slice_mut());

                if (VDOT(E.as_slice(), XPROD.as_slice()) < 0.0) {
                    ARGP = (TWOPI(ctx) - ARGP);
                }
            } else if (E[3] < 0.0) {
                //
                // The periapsis is below the reference plane;  the argument
                // of periapsis must be greater than 180 degrees.
                //
                ARGP = (TWOPI(ctx) - ARGP);
            }
        }
    }

    //
    // And finally, the position of the object within the orbit.
    // The true anomaly, nu, is the angle between the eccentricity
    // and radius vectors, e and r. (For circular orbits, substitute
    // n for e.)           -     -
    // -     -
    //
    // This angle increases in the counterclockwise direction about h.
    // We express the position in the perifocal frame in order to
    // extract nu.
    //
    if (ECC == 0.0) {
        //
        // In this case, the argument of periapse is set to zero,
        // so the nu is measured from N.
        //
        VHAT(N.as_slice(), PERIX.as_slice_mut());
    } else {
        VHAT(E.as_slice(), PERIX.as_slice_mut());
    }

    UCRSS(H.as_slice(), PERIX.as_slice(), PERIY.as_slice_mut());

    NU = f64::atan2(
        VDOT(R.as_slice(), PERIY.as_slice()),
        VDOT(R.as_slice(), PERIX.as_slice()),
    );

    //
    // Unfortunately, the other element routines need the mean
    // anomaly, M. The true and mean anomalies are related through
    // the eccentric anomalies D (parabolas), E (ellipses), and
    // F (hyperbolas), as shown below.
    //
    //                  e + cos(nu)
    //       cos(E)  = ---------------         (ellipse)
    //                  1 + e cos(nu)
    //
    //       M       = E - e sin(E)
    //
    //
    //                   e + cos(nu)
    //       cosh(F) = ---------------         (hyperbola)
    //                   1 + e cos(nu)
    //
    //       M       = e sinh(F) - F
    //
    //
    //       D       = tan(nu/2)               (parabola)
    //
    //                       3
    //       M       =  D + D / 3
    //
    // For elliptic orbits, the mean anomaly should be in [0,2*pi].
    //

    if (ECC < 1.0) {
        //
        // For improved numerical performance, we compute both the
        // sine and cosine of the eccentric anomaly, then let ATAN2
        // find the eccentric anomaly.
        //
        COSEA = ((ECC + f64::cos(NU)) / (1.0 + (ECC * f64::cos(NU))));

        //
        // Here we use the relationships (here b is the length
        // of the semi-minor axis):
        //
        //    a sin(E) = (a/b) r sin(nu)
        //
        //    sin(E)   = (r/b) sin(nu)
        //                        ______________
        //             = (r/rp) \/ (1-e) / (1+e)  sin(nu)
        //
        //
        SINEA = (((RMAG / RP) * f64::sqrt(((1.0 - ECC) / (1.0 + ECC)))) * f64::sin(NU));

        EA = f64::atan2(SINEA, COSEA);

        M0 = f64::copysign((EA - (ECC * f64::sin(EA))), NU);

        if (M0 < 0.0) {
            M0 = (M0 + TWOPI(ctx));
        }
    } else if (ECC > 1.0) {
        COSHF = ((ECC + f64::cos(NU)) / (1.0 + (ECC * f64::cos(NU))));

        EA = DACOSH(intrinsics::DMAX1(&[1.0, COSHF]), ctx)?;

        M0 = f64::copysign(((ECC * f64::sinh(EA)) - EA), NU);
    } else {
        EA = f64::tan((NU / 2.0));
        M0 = f64::copysign((EA + (f64::powi(EA, 3) / 3.0)), NU);
    }

    //
    // Return the elements as a vector, suitable for input to CONICS.
    //
    ELTS[1] = RP;
    ELTS[2] = ECC;
    ELTS[3] = INC;
    ELTS[4] = LNODE;
    ELTS[5] = ARGP;
    ELTS[6] = M0;
    ELTS[7] = ET;
    ELTS[8] = MU;

    CHKOUT(b"OSCELT", ctx)?;
    Ok(())
}
