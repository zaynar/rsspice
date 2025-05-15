//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Evaluate a type 15 SPK data record
///
/// Evaluate a single SPK data record from a segment of type 15
/// (Precessing Conic Propagation).
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Target epoch.
///  RECIN      I   Data record.
///  STATE      O   State (position and velocity).
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is a target epoch, specified as ephemeris seconds past
///           J2000, at which a state vector is to be computed.
///
///  RECIN    is a data record which, when evaluated at epoch ET,
///           will give the state (position and velocity) of some
///           body, relative to some center, in some inertial
///           reference frame.
///
///           The structure of RECIN is:
///
///           RECIN(1)             epoch of periapsis
///                                in ephemeris seconds past J2000.
///           RECIN(2)-RECIN(4)    unit trajectory pole vector
///           RECIN(5)-RECIN(7)    unit periapsis vector
///           RECIN(8)             semi-latus rectum---p in the
///                                equation:
///
///                                r = p/(1 + ECC*COS(Nu))
///
///           RECIN(9)             eccentricity
///           RECIN(10)            J2 processing flag describing
///                                what J2 corrections are to be
///                                applied when the orbit is
///                                propagated.
///
///                                All J2 corrections are applied
///                                if this flag has a value that
///                                is not 1,2 or 3.
///
///                                If the value of the flag is 3
///                                no corrections are done.
///
///                                If the value of the flag is 1
///                                no corrections are computed for
///                                the precession of the line
///                                of apsides. However, regression
///                                of the line of nodes is
///                                performed.
///
///                                If the value of the flag is 2
///                                no corrections are done for
///                                the regression of the line of
///                                nodes. However, precession of the
///                                line of apsides is performed.
///
///                                Note that J2 effects are computed
///                                only if the orbit is elliptic and
///                                does not intersect the central
///                                body.
///
///           RECIN(11)-RECIN(13)  unit central body pole vector
///           RECIN(14)            central body GM
///           RECIN(15)            central body J2
///           RECIN(16)            central body radius
///
///           Units are radians, km, seconds
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state produced by evaluating RECIN at ET.
///           Units are km and km/sec.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the eccentricity is less than zero, the error
///      SPICE(BADECCENTRICITY) is signaled.
///
///  2)  If the semi-latus rectum is non-positive, the error
///      SPICE(BADLATUSRECTUM) is signaled.
///
///  3)  If the pole vector, trajectory pole vector or periapsis vector
///      has zero length, the error SPICE(BADVECTOR) is signaled.
///
///  4)  If the trajectory pole vector and the periapsis vector are not
///      orthogonal, the error SPICE(BADINITSTATE) is signaled. The
///      test for orthogonality is very crude. The routine simply
///      checks that the absolute value of the dot product of the unit
///      vectors parallel to the trajectory pole and periapse vectors
///      is less than 0.00001. This check is intended to catch
///      blunders, not to enforce orthogonality to double precision
///      tolerance.
///
///  5)  If the mass of the central body is non-positive, the error
///      SPICE(NONPOSITIVEMASS) is signaled.
///
///  6)  If the radius of the central body is negative, the error
///      SPICE(BADRADIUS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This algorithm applies J2 corrections for precessing the
///  node and argument of periapse for an object orbiting an
///  oblate spheroid.
///
///  Note the effects of J2 are incorporated only for elliptic
///  orbits that do not intersect the central body.
///
///  While the derivation of the effect of the various harmonics
///  of gravitational field are beyond the scope of this header
///  the effect of the J2 term of the gravity model are as follows
///
///
///     The line of node precesses. Over one orbit average rate of
///     precession,  DNode/dNu,  is given by
///
///                             3 J2
///           dNode/dNu =  -  -----------------  DCOS( inc )
///                             2 (P/RPL)**2
///
///     (Since this is always less than zero for oblate spheroids, this
///        should be called regression of nodes.)
///
///     The line of apsides precesses. The average rate of precession
///     DPeri/dNu is given by
///                                3 J2
///           dPeri/dNu =     ----------------- ( 5*DCOS ( inc ) - 1 )
///                             2 (P/RPL)**2
///
///     Details of these formulae are given in the Battin's book (see
///     literature references below).
///
///
///  It is assumed that this routine is used in conjunction with
///  the routine SPKR15 as shown here:
///
///     CALL SPKR15 ( HANDLE, DESCR, ET, RECIN         )
///     CALL SPKE15 (                ET, RECIN, STATE  )
///
///  where it is known in advance that the HANDLE, DESCR pair points
///  to a type 15 data segment.
/// ```
///
/// # Examples
///
/// ```text
///  The SPKEnn routines are almost always used in conjunction with
///  the corresponding SPKRnn routines, which read the records from
///  SPK files.
///
///  The data returned by the SPKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the SPKRnn
///  routines might be used to examine raw segment data before
///  evaluating it with the SPKEnn routines.
///
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL SPKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///  C
///  C     Look at parts of the descriptor.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 15 ) THEN
///
///           CALL SPKR15 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE15 ( ET, RECORD, STATE )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Danby, "Fundamentals of Celestial Mechanics," 2nd Edition,
///       pp.345-347, Willman-Bell, 1989.
///
///  [2]  R. H. Battin, "Astronautical Guidance," pp.199, McGraw-Hill
///       Book Company, 1964.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  S. Schlaifer       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.2.0, 02-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VHAT, VROTV, and VSCL calls.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
/// -    SPICELIB Version 1.0.0, 15-NOV-1994 (WLT) (SS)
/// ```
pub fn spke15(
    ctx: &mut SpiceContext,
    et: f64,
    recin: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE15(et, recin, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE15 ( Evaluate a type 15 SPK data record)
pub fn SPKE15(
    ET: f64,
    RECIN: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECIN = DummyArray::new(RECIN, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut ANGLE: f64 = 0.0;
    let mut COSINC: f64 = 0.0;
    let mut DMDT: f64 = 0.0;
    let mut DNODE: f64 = 0.0;
    let mut DOT: f64 = 0.0;
    let mut DPERI: f64 = 0.0;
    let mut DT: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut EPOCH: f64 = 0.0;
    let mut GM: f64 = 0.0;
    let mut K2PI: f64 = 0.0;
    let mut MANOM: f64 = 0.0;
    let mut NEAR: f64 = 0.0;
    let mut OJ2: f64 = 0.0;
    let mut ONEME2: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut PA = StackArray::<f64, 3>::new(1..=3);
    let mut PV = StackArray::<f64, 3>::new(1..=3);
    let mut RPL: f64 = 0.0;
    let mut SPEED: f64 = 0.0;
    let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
    let mut TA: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut TMPSTA = StackArray::<f64, 6>::new(1..=6);
    let mut TP = StackArray::<f64, 3>::new(1..=3);
    let mut Z: f64 = 0.0;
    let mut J2FLG: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKE15", ctx)?;

    //
    // Fetch the various entities from the input record, first the epoch.
    //
    EPOCH = RECIN[1];

    //
    // The trajectory pole vector.
    //
    VEQU(RECIN.subarray(2), TP.as_slice_mut());

    //
    // The periapsis vector.
    //
    VEQU(RECIN.subarray(5), PA.as_slice_mut());

    //
    // Semi-latus rectum ( P in the P/(1 + ECC*COS(Nu)  ),
    // and eccentricity.
    //
    P = RECIN[8];
    ECC = RECIN[9];

    //
    // J2 processing flag.
    //
    J2FLG = (RECIN[10] as i32);

    //
    // Central body pole vector.
    //
    VEQU(RECIN.subarray(11), PV.as_slice_mut());

    //
    // The central mass, J2 and radius of the central body.
    //
    GM = RECIN[14];
    OJ2 = RECIN[15];
    RPL = RECIN[16];

    //
    // Check all the inputs here for obvious failures.  Yes, perhaps
    // this is overkill.  However, there is a lot more computation
    // going on in this routine so that the small amount of overhead
    // here should not be significant.
    //

    if (P <= 0 as f64) {
        SETMSG(b"The semi-latus rectum supplied to the SPK type 15 evaluator was non-positive.  This value must be positive. The value supplied was #.", ctx);
        ERRDP(b"#", P, ctx);
        SIGERR(b"SPICE(BADLATUSRECTUM)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    } else if (ECC < 0.0) {
        SETMSG(b"The eccentricity supplied for a type 15 segment is negative.  It must be non-negative. The value supplied to the type 15 evaluator was #. ", ctx);
        ERRDP(b"#", ECC, ctx);
        SIGERR(b"SPICE(BADECCENTRICITY)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    } else if (GM <= 0.0) {
        SETMSG(b"The mass supplied for the central body of a type 15 segment was non-positive. Masses must be positive.  The value supplied was #. ", ctx);
        ERRDP(b"#", GM, ctx);
        SIGERR(b"SPICE(NONPOSITIVEMASS)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    } else if VZERO(TP.as_slice()) {
        SETMSG(b"The trajectory pole vector supplied to SPKE15 had length zero. The most likely cause of this problem is a corrupted SPK (ephemeris) file. ", ctx);
        SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    } else if VZERO(PA.as_slice()) {
        SETMSG(b"The periapse vector supplied to SPKE15 had length zero. The most likely cause of this problem is a corrupted SPK (ephemeris) file. ", ctx);
        SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    } else if VZERO(PV.as_slice()) {
        SETMSG(b"The central pole vector supplied to SPKE15 had length zero. The most likely cause of this problem is a corrupted SPK (ephemeris) file. ", ctx);
        SIGERR(b"SPICE(BADVECTOR)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    } else if (RPL < 0.0) {
        SETMSG(b"The central body radius was negative. It must be zero or positive.  The value supplied was #. ", ctx);
        ERRDP(b"#", RPL, ctx);
        SIGERR(b"SPICE(BADRADIUS)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    }

    //
    // Convert TP, PV and PA to unit vectors.
    // (It won't hurt to polish them up a bit here if they are already
    //  unit vectors.)
    //
    VHATIP(PA.as_slice_mut());
    VHATIP(TP.as_slice_mut());
    VHATIP(PV.as_slice_mut());

    //
    // One final check.  Make sure the pole and periapse vectors are
    // orthogonal. (We will use a very crude check but this should
    // rule out any obvious errors.)
    //
    DOT = VDOT(PA.as_slice(), TP.as_slice());

    if (f64::abs(DOT) > 0.00001) {
        ANGLE = (VSEP(PA.as_slice(), TP.as_slice(), ctx) * DPR(ctx));

        SETMSG(b"The periapsis and trajectory pole vectors are not orthogonal. The anglebetween them is # degrees. ", ctx);
        ERRDP(b"#", ANGLE, ctx);
        SIGERR(b"SPICE(BADINITSTATE)", ctx)?;
        CHKOUT(b"SPKE15", ctx)?;
        return Ok(());
    }

    //
    // Compute the distance and speed at periapse.
    //
    NEAR = (P / (1.0 + ECC));
    SPEED = (f64::sqrt((GM / P)) * (1.0 + ECC));

    //
    // Next get the position at periapse ...
    //
    VSCL(NEAR, PA.as_slice(), STATE0.as_slice_mut());

    //
    // ... and the velocity at periapsis.
    //
    VCRSS(TP.as_slice(), PA.as_slice(), STATE0.subarray_mut(4));
    VSCLIP(SPEED, STATE0.subarray_mut(4));

    //
    // Determine the elapsed time from periapse to the requested
    // epoch and propagate the state at periapsis to the epoch of
    // interest.
    //
    // Note that we are making use of the following fact.
    //
    //    If R is a rotation, then the states obtained by
    //    the following blocks of code are mathematically the
    //    same. (In reality they may differ slightly due to
    //    roundoff.)
    //
    //    Code block 1.
    //
    //       CALL MXV   ( R,  STATE0,     STATE0    )
    //       CALL MXV   ( R,  STATE0(4),  STATE0(4) )
    //       CALL PROP2B( GM, STATE0, DT, STATE     )
    //
    //    Code block 2.
    //
    //       CALL PROP2B( GM, STATE0, DT, STATE    )
    //       CALL MXV   ( R,  STATE,      STATE    )
    //       CALL MXV   ( R,  STATE(4),   STATE(4) )
    //
    //
    // This allows us to first compute the propagation of our initial
    // state and then if needed perform the precession of the line
    // of nodes and apsides by simply precessing the resulting state.
    //
    DT = (ET - EPOCH);
    PROP2B(GM, STATE0.as_slice(), DT, STATE.as_slice_mut(), ctx)?;

    //
    // If called for, handle precession needed due to the J2 term.  Note
    // that the motion of the lines of nodes and apsides is formulated
    // in terms of the true anomaly.  This means we need the accumulated
    // true anomaly in order to properly transform the state.
    //
    if ((((J2FLG != 3) && (OJ2 != 0.0)) && (ECC < 1.0)) && (NEAR > RPL)) {
        //
        // First compute the change in mean anomaly since periapsis.
        //
        ONEME2 = (1.0 - f64::powi(ECC, 2));
        DMDT = ((ONEME2 / P) * f64::sqrt(((GM * ONEME2) / P)));
        MANOM = (DMDT * DT);
        //
        // Next compute the angle THETA such that THETA is between
        // -pi and pi and such than MANOM = THETA + K*2*pi for
        // some integer K.
        //
        THETA = intrinsics::DMOD(MANOM, TWOPI(ctx));

        if (f64::abs(THETA) > PI(ctx)) {
            THETA = (THETA - f64::copysign(TWOPI(ctx), THETA));
        }

        K2PI = (MANOM - THETA);
        //
        // We can get the accumulated true anomaly from the propagated
        // state theta and the accumulated mean anomaly prior to this
        // orbit.
        //
        TA = VSEP(PA.as_slice(), STATE.as_slice(), ctx);
        TA = f64::copysign(TA, THETA);
        TA = (TA + K2PI);

        //
        // Determine how far the line of nodes and periapsis have moved.
        //
        COSINC = VDOT(PV.as_slice(), TP.as_slice());

        Z = (((TA * 1.5) * OJ2) * f64::powi((RPL / P), 2));
        DNODE = -(Z * COSINC);
        DPERI = (Z * ((2.5 * f64::powi(COSINC, 2)) - 0.5));

        //
        // Precess the periapsis by rotating the state vector about the
        // trajectory pole
        //
        if (J2FLG != 1) {
            VROTV(
                STATE.subarray(1),
                TP.as_slice(),
                DPERI,
                TMPSTA.subarray_mut(1),
            );
            VROTV(
                STATE.subarray(4),
                TP.as_slice(),
                DPERI,
                TMPSTA.subarray_mut(4),
            );
            MOVED(TMPSTA.as_slice(), 6, STATE.as_slice_mut());
        }

        //
        // Regress the line of nodes by rotating the state
        // about the pole of the central body.
        //
        if (J2FLG != 2) {
            VROTV(
                STATE.as_slice(),
                PV.as_slice(),
                DNODE,
                TMPSTA.subarray_mut(1),
            );
            VROTV(
                STATE.subarray(4),
                PV.as_slice(),
                DNODE,
                TMPSTA.subarray_mut(4),
            );
            MOVED(TMPSTA.as_slice(), 6, STATE.as_slice_mut());
        }

        //
        // We could perform the rotations above in the other order,
        // but we would also have to rotate the pole before precessing
        // the line of apsides.
        //
    }

    //
    // That's all folks.  Check out and return.
    //
    CHKOUT(b"SPKE15", ctx)?;
    Ok(())
}
