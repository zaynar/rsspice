//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Determine state from conic elements
///
/// Determine the state (position, velocity) of an orbiting body
/// from a set of elliptic, hyperbolic, or parabolic orbital
/// elements.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ELTS       I   Conic elements.
///  ET         I   Input time.
///  STATE      O   State of orbiting body at ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ELTS     are conic elements describing the orbit of a body
///           around a primary. The elements are, in order:
///
///              RP      Perifocal distance.
///              ECC     Eccentricity.
///              INC     Inclination.
///              LNODE   Longitude of the ascending node.
///              ARGP    Argument of periapse.
///              M0      Mean anomaly at epoch.
///              T0      Epoch.
///              MU      Gravitational parameter.
///
///           Units are km, rad, rad/sec, km**3/sec**2. The epoch
///           is given in ephemeris seconds past J2000. The same
///           elements are used to describe all three types
///           (elliptic, hyperbolic, and parabolic) of conic orbit.
///
///  ET       is the time at which the state of the orbiting body
///           is to be determined, in ephemeris seconds J2000.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state (position and velocity) of the body at
///           time ET. Components are x, y, z, dx/dt, dy/dt, dz/dt.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the eccentricity supplied is less than 0, the error
///      SPICE(BADECCENTRICITY) is signaled.
///
///  2)  If a non-positive periapse distance is supplied, the error
///      SPICE(BADPERIAPSEVALUE) is signaled.
///
///  3)  If a non-positive value for the attracting mass is supplied,
///      the error SPICE(BADGM) is signaled.
///
///  4)  If ELTS is such that the resulting orbit at periapsis has
///      either its position or velocity equal to zero, or the square
///      of the resulting specific angular momentum's magnitude is
///      zero, an error is signaled by a routine in the call tree of
///      this routine. This is an indication of invalid ELTS elements.
///
///  5)  If ET is such that the offset in time from periapsis, at which
///      the state is to be determined, is so large that there is a
///      danger of floating point overflow during computation, an error
///      is signaled by a routine in the call tree of this routine.
/// ```
///
/// # Examples
///
/// ```text
///  Let VINIT contain the initial state of a spacecraft relative to
///  the center of a planet at epoch ET, and let GM be the gravitation
///  parameter of the planet. The call
///
///     CALL OSCELT ( VINIT, ET, GM, ELTS )
///
///  produces a set of osculating elements describing the nominal
///  orbit that the spacecraft would follow in the absence of all
///  other bodies in the solar system and non-gravitational forces
///  on the spacecraft.
///
///  Now let STATE contain the state of the same spacecraft at some
///  other epoch, LATER. The difference between this state and the
///  state predicted by the nominal orbit at the same epoch can be
///  computed as follows.
///
///     CALL CONICS ( ELTS, LATER, NOMINAL )
///     CALL VSUBG  ( NOMINAL, STATE, 6, DIFF )
///
///     WRITE (*,*) 'Perturbation in x, dx/dt = ', DIFF(1), DIFF(4)
///     WRITE (*,*) '                y, dy/dt = ', DIFF(2), DIFF(5)
///     WRITE (*,*) '                z, dz/dt = ', DIFF(3), DIFF(6)
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
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries in $Revisions section.
///
///         Added entry #4 and updated entry #5 in $Exceptions section.
///
/// -    SPICELIB Version 4.0.0, 26-MAR-1998 (WLT)
///
///         There was a coding error in the computation of the mean
///         anomaly in the parabolic case. This problem has been
///         corrected.
///
/// -    SPICELIB Version 3.0.1, 15-OCT-1996 (WLT)
///
///         Corrected a typo in the description of the units associated
///         with the input elements.
///
/// -    SPICELIB Version 3.0.0, 12-NOV-1992 (WLT)
///
///         The routine was re-written to make use of NAIF's universal
///         variables formulation for state propagation (PROP2B). As
///         a result, several problems were simultaneously corrected.
///
///         A major bug was fixed that caused improper state evaluations
///         for ET's that precede the epoch of the elements in the
///         elliptic case.
///
///         A danger of non-convergence in the solution of Kepler's
///         equation has been eliminated.
///
///         In addition to this reformulation of CONICS checks were
///         installed that ensure the elements supplied are physically
///         meaningful. Eccentricity must be non-negative. The
///         distance at periapse and central mass must be positive. If
///         not errors are signaled.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 19-APR-1991 (WLT)
///
///         An error in the hyperbolic state generation was corrected.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 12-NOV-1992 (WLT)
///
///         The routine was re-written to make use of NAIF's universal
///         variables formulation for state propagation (PROP2B). As
///         a result, several problems were simultaneously corrected.
///
///         A major bug was fixed that caused improper state
///         evaluations for ET's that precede the epoch of the
///         elements in the elliptic case.
///
///         A danger of non-convergence in the solution of Kepler's
///         equation has been eliminated.
///
///         In addition to this reformulation of CONICS checks were
///         installed that ensure the elements supplied are physically
///         meaningful. Eccentricity must be non-negative. The
///         distance at periapse and central mass must be positive.
///         If not errors are signaled.
///
///         These changes were prompted by the discovery that the old
///         formulation had a severe bug for elliptic orbits and
///         epochs prior to the epoch of the input elements, and by
///         the discovery that the time of flight routines had
///         problems with convergence.
///
/// -    SPICELIB Version 2.0.0, 19-APR-1991 (WLT)
///
///         The original version of the routine had a bug in that
///         it attempted to restrict the hyperbolic anomaly to
///         the interval 0 to 2*PI. This has been fixed.
/// ```
pub fn conics(
    ctx: &mut SpiceContext,
    elts: &[f64; 8],
    et: f64,
    state: &mut [f64; 6],
) -> crate::Result<()> {
    CONICS(elts, et, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CONICS ( Determine state from conic elements )
pub fn CONICS(
    ELTS: &[f64],
    ET: f64,
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ELTS = DummyArray::new(ELTS, 1..=8);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut RP: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut INC: f64 = 0.0;
    let mut LNODE: f64 = 0.0;
    let mut ARGP: f64 = 0.0;
    let mut M0: f64 = 0.0;
    let mut T0: f64 = 0.0;
    let mut MU: f64 = 0.0;
    let mut N: f64 = 0.0;
    let mut PERIOD: f64 = 0.0;
    let mut AINVRS: f64 = 0.0;
    let mut DT: f64 = 0.0;
    let mut COSI: f64 = 0.0;
    let mut SINI: f64 = 0.0;
    let mut COSN: f64 = 0.0;
    let mut SINN: f64 = 0.0;
    let mut COSW: f64 = 0.0;
    let mut SINW: f64 = 0.0;
    let mut SNCI: f64 = 0.0;
    let mut CNCI: f64 = 0.0;
    let mut BASISP = StackArray::<f64, 3>::new(1..=3);
    let mut BASISQ = StackArray::<f64, 3>::new(1..=3);
    let mut PSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut V: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // The only real work required by this routine is the construction
    // of a preliminary state vector from the input elements.  Once this
    // is in hand, we can simply let the routine PROP2B do the real
    // work, free from the instabilities inherent in the classical
    // elements formulation of two-body motion.
    //
    // To do this we shall construct a basis of vectors that lie in the
    // plane of the orbit.  The first vector P shall point towards the
    // position of the orbiting body at periapse.  The second
    // vector Q shall point along the velocity vector of the body at
    // periapse.
    //
    // The only other consideration is determining an epoch, TP, of
    // this state and the delta time ET - TP.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CONICS", ctx)?;
    }

    //
    // Unpack the element vector.
    //
    RP = ELTS[1];
    ECC = ELTS[2];
    INC = ELTS[3];
    LNODE = ELTS[4];
    ARGP = ELTS[5];
    M0 = ELTS[6];
    T0 = ELTS[7];
    MU = ELTS[8];

    //
    // Handle all of the exceptions first.
    //

    if (ECC < 0 as f64) {
        SETMSG(b"The eccentricity supplied was negative. Only positive values are meaningful.  The value was #", ctx);
        ERRDP(b"#", ECC, ctx);
        SIGERR(b"SPICE(BADECCENTRICITY)", ctx)?;
        CHKOUT(b"CONICS", ctx)?;
        return Ok(());
    }

    if (RP <= 0 as f64) {
        SETMSG(b"The value of periapse range supplied was non-positive.  Only positive values are allowed.  The value supplied was #. ", ctx);
        ERRDP(b"#", RP, ctx);
        SIGERR(b"SPICE(BADPERIAPSEVALUE)", ctx)?;
        CHKOUT(b"CONICS", ctx)?;
        return Ok(());
    }

    if (MU <= 0 as f64) {
        SETMSG(b"The value of GM supplied was non-positive.  Only positive values are allowed.  The value supplied was #. ", ctx);
        ERRDP(b"#", MU, ctx);
        SIGERR(b"SPICE(BADGM)", ctx)?;
        CHKOUT(b"CONICS", ctx)?;
        return Ok(());
    }

    //
    // First construct the orthonormal basis vectors that span the orbit
    // plane.
    //
    COSI = f64::cos(INC);
    SINI = f64::sin(INC);
    COSN = f64::cos(LNODE);
    SINN = f64::sin(LNODE);
    COSW = f64::cos(ARGP);
    SINW = f64::sin(ARGP);

    SNCI = (SINN * COSI);
    CNCI = (COSN * COSI);

    BASISP[1] = ((COSN * COSW) - (SNCI * SINW));
    BASISP[2] = ((SINN * COSW) + (CNCI * SINW));
    BASISP[3] = (SINI * SINW);

    BASISQ[1] = (-(COSN * SINW) - (SNCI * COSW));
    BASISQ[2] = (-(SINN * SINW) + (CNCI * COSW));
    BASISQ[3] = (SINI * COSW);

    //
    // Next construct the state at periapse.
    //
    // The position at periapse is just BASISP scaled by the distance
    // at periapse.
    //
    // The velocity must be constructed so that we can get an orbit
    // of this shape.  Recall that the magnitude of the specific angular
    // momentum vector is given by DSQRT ( MU*RP*(1+ECC) )
    // The velocity will be given by V * BASISQ.  But we must have the
    // magnitude of the cross product of position and velocity be
    // equal to DSQRT ( MU*RP*(1+ECC) ). So we must have
    //
    //    RP*V = DSQRT( MU*RP*(1+ECC) )
    //
    // so that:
    //
    V = f64::sqrt(((MU * (1.0 + ECC)) / RP));

    VSCL(RP, BASISP.as_slice(), PSTATE.subarray_mut(1));
    VSCL(V, BASISQ.as_slice(), PSTATE.subarray_mut(4));

    //
    // Finally compute DT the elapsed time since the epoch of periapse.
    // Ellipses first, since they are the most common.
    //
    if (ECC < 1.0) {
        //
        // Recall that:
        //
        // N ( mean motion ) is given by DSQRT( MU / A**3 ).
        // But since, A = RP / ( 1 - ECC ) ...
        //
        AINVRS = ((1.0 - ECC) / RP);
        N = (f64::sqrt((MU * AINVRS)) * AINVRS);
        PERIOD = (TWOPI(ctx) / N);

        //
        // In general the mean anomaly is given by
        //
        //    M  = (T - TP) * N
        //
        // Where TP is the time of periapse passage.  M0 is the mean
        // anomaly at time T0 so that
        // Thus
        //
        //    M0 = ( T0 - TP ) * N
        //
        // So TP = T0-M0/N hence the time since periapse at time ET
        // is given by ET - T0 + M0/N.  Finally, since elliptic orbits are
        // periodic, we can mod this value by the period of the orbit.
        //
        DT = intrinsics::DMOD(((ET - T0) + (M0 / N)), PERIOD);

    //
    // Hyperbolas next.
    //
    } else if (ECC > 1 as f64) {
        //
        // Again, recall that:
        //
        // N ( mean motion ) is given by DSQRT( MU / |A**3| ).
        // But since, |A| = RP / ( ECC - 1 ) ...
        //
        AINVRS = ((ECC - 1.0) / RP);
        N = (f64::sqrt((MU * AINVRS)) * AINVRS);
        DT = ((ET - T0) + (M0 / N));

    //
    // Finally, parabolas.
    //
    } else {
        N = (f64::sqrt((MU / (2.0 * RP))) / RP);
        DT = ((ET - T0) + (M0 / N));
    }

    //
    // Now let PROP2B do the work of propagating the state.
    //
    PROP2B(MU, PSTATE.as_slice(), DT, STATE.as_slice_mut(), ctx)?;

    CHKOUT(b"CONICS", ctx)?;
    Ok(())
}
