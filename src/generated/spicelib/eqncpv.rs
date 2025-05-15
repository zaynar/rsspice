//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    FIRST: bool,
    PI2: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIRST: bool = false;
        let mut PI2: f64 = 0.0;

        FIRST = true;

        Self { FIRST, PI2 }
    }
}

/// Equinoctial Elements to position and velocity
///
/// Compute the state (position and velocity) of an object whose
/// trajectory is described via equinoctial elements relative to some
/// fixed plane (usually the equatorial plane of some planet).
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
///  ET         I   Epoch in seconds past J2000 to find state
///  EPOCH      I   Epoch of elements in seconds past J2000
///  EQEL       I   Array of equinoctial elements
///  RAPOL      I   Right Ascension of the pole of the reference plane
///  DECPOL     I   Declination of the pole of the reference plane
///  STATE      O   State of the object described by EQEL.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch (ephemeris time) at which the state
///           of the target body is to be computed. ET is measured
///           in seconds past the J2000 epoch.
///
///  EPOCH    is the epoch of the equinoctial elements in seconds
///           past the J2000 epoch.
///
///  EQEL     is an array of 9 double precision numbers that are the
///           equinoctial elements for some orbit expressed relative to
///           the equatorial frame of the central body defined as
///
///           -  The Z-axis of the equatorial frame is the direction
///              of the pole of the central body relative to some
///              inertial frame;
///
///           -  The X-axis is given by the cross product of the Z-axis
///              of the inertial frame with the direction of the pole
///              of the central body; and
///
///           -  The Y-axis completes a right handed frame.
///
///           If the X-axis of the equatorial frame is aligned with the
///           X-axis of the inertial frame, then the X-axis of the
///           equatorial frame will be located at 90 degrees + RAPOL in
///           the inertial frame.
///
///           The specific arrangement of the elements is spelled out
///           below:
///
///              EQEL(1)   is the semi-major axis (A) of the orbit in
///                        km.
///
///              EQEL(2)   is the value of H at the specified epoch.
///                        ( E*SIN(ARGP+NODE) ).
///
///              EQEL(3)   is the value of K at the specified epoch
///                        ( E*COS(ARGP+NODE) ).
///
///              EQEL(4)   is the mean longitude (MEAN0+ARGP+NODE) at
///                        the epoch of the elements measured in
///                        radians.
///
///              EQEL(5)   is the value of P (TAN(INC/2)*SIN(NODE)) at
///                        the specified epoch.
///
///              EQEL(6)   is the value of Q (TAN(INC/2)*COS(NODE)) at
///                        the specified epoch.
///
///              EQEL(7)   is the rate of the longitude of periapse
///                        (dARGP/dt + dNODE/dt ) at the epoch of
///                        the elements. This rate is assumed to hold
///                        for all time. The rate is measured in
///                        radians per second.
///
///              EQEL(8)   is the derivative of the mean longitude
///                        ( dM/dt + dARGP/dt + dNODE/dt ). This
///                        rate is assumed to be constant and is
///                        measured in radians/second.
///
///              EQEL(9)   is the rate of the longitude of the
///                        ascending node ( dNODE/dt). This rate is
///                        measured in radians per second.
///
///           where
///
///              INC       is the inclination of the orbit,
///
///              ARGP      is the argument of periapse,
///
///              NODE      is longitude of the ascending node, and
///
///              E         is eccentricity of the orbit.
///
///  RAPOL    is the Right Ascension of the pole of the reference plane
///           with respect to some inertial frame (measured in
///           radians).
///
///  DECPOL   is the Declination of the pole of the reference plane
///           with respect to some inertial frame (measured in
///           radians).
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state of the object described by EQEL relative to
///           the inertial frame used to define RAPOL and DECPOL. Units
///           are in km and km/sec.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the eccentricity corresponding to the input elements is
///      greater than 0.9, the error SPICE(ECCOUTOFRANGE) is signaled.
///
///  2)  If the semi-major axis of the elements is non-positive, the
///      error SPICE(BADSEMIAXIS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine evaluates the input equinoctial elements for
///  the specified epoch and return the corresponding state.
///
///  This routine was adapted from a routine provided by
///  Bob Jacobson of the Planetary Dynamics Group of
///  the Navigation and Flight Mechanics Section at JPL.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you have classical elements and rates of
///  change of the ascending node and argument of periapse
///  for some satellite of the earth.
///
///  By transforming the classical elements
///  this routine can be used to compute the state of the
///  object at an arbitrary epoch. The code below illustrates
///  how you might do this.
///
///  The table below illustrates the meanings of the various
///  variables used in the discussion below.
///
///        Variable     Meaning
///        --------     ----------------------------------
///        A            Semi-major axis in km
///        ECC          Eccentricity of orbit
///        INC          Inclination of orbit
///        NODE         Longitude of the ascending node at epoch
///        OMEGA        Argument of periapse at epoch
///        M            Mean anomaly at epoch
///        DMDT         Mean anomaly rate in radians/second
///        DNODE        Rate of change of longitude of ascending node
///                     in radians/second
///        DARGP        Rate of change of argument of periapse in
///                     radians/second
///        EPOCH        is the epoch of the elements in seconds past
///                     the J2000 epoch.
///
///
///     EQEL(1) = A
///     EQEL(2) = ECC * DSIN ( OMEGA + NODE )
///     EQEL(3) = ECC * DCOS ( OMEGA + NODE )
///
///     EQEL(4) = M + OMEGA + NODE
///
///     EQEL(5) = TAN(INC/2.0D0) * DSIN(NODE)
///     EQEL(6) = TAN(INC/2.0D0) * DCOS(NODE)
///
///     EQEL(7) = DARGP
///     EQEL(8) = DARGP + DMDT + DNODE
///     EQEL(9) = DNODE
///
///
///     We shall compute the state of the satellite in the
///     pole and equator reference system.
///
///     RAPOL   = -HALFPI()
///     DECPOL  =  HALFPI()
///
///
///     Now compute the state at the desired epoch ET.
///
///     CALL EQNCPV ( ET, EPOCH, EQEL, RAPOL, DECPOL, STATE )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The equinoctial elements used by this routine are taken
///      from  "Tangent" formulation of equinoctial elements
///
///         p = tan(inclination/2) * sin(R.A. of ascending node)
///         q = tan(inclination/2) * cos(R.A. of ascending node)
///
///      Other formulations use Sine instead of Tangent. We shall
///      call these the "Sine" formulations.
///
///         p = sin(inclination/2) * sin(R.A. of ascending node)
///         q = sin(inclination/2) * cos(R.A. of ascending node)
///
///      If you have equinoctial elements from this alternative
///      formulation you should replace p and q  by the
///      expressions below.
///
///         P = P / DSQRT ( 1.0D0 - P*P - Q*Q )
///         Q = Q / DSQRT ( 1.0D0 - P*P - Q*Q )
///
///      This will convert the Sine formulation to the Tangent
///      formulation.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  W. Owen and R. Vaughan, "Optical Navigation Program
///       Mathematical Models," JPL Engineering Memorandum 314-513,
///       August 9, 1991.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  R.A. Jacobson      (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.3, 14-APR-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added SPK
///         required reading.
///
/// -    SPICELIB Version 1.0.2, 18-MAY-2010 (BVS)
///
///         Removed "C$" marker from text in the header.
///
/// -    SPICELIB Version 1.0.1, 31-JAN-2008 (BVS)
///
///         Removed non-standard header section heading
///         'Declarations_of_external_functions'.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1997 (WLT) (RAJ)
/// ```
pub fn eqncpv(
    ctx: &mut SpiceContext,
    et: f64,
    epoch: f64,
    eqel: &[f64; 9],
    rapol: f64,
    decpol: f64,
    state: &mut [f64; 6],
) -> crate::Result<()> {
    EQNCPV(et, epoch, eqel, rapol, decpol, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EQNCPV (Equinoctial Elements to position and velocity)
pub fn EQNCPV(
    ET: f64,
    EPOCH: f64,
    EQEL: &[f64],
    RAPOL: f64,
    DECPOL: f64,
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let EQEL = DummyArray::new(EQEL, 1..=9);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut CAN: f64 = 0.0;
    let mut CF: f64 = 0.0;
    let mut CN: f64 = 0.0;
    let mut DT: f64 = 0.0;
    let mut DI: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut NODEDT: f64 = 0.0;
    let mut NODE: f64 = 0.0;
    let mut DLPDT: f64 = 0.0;
    let mut DLP: f64 = 0.0;
    let mut MLDT: f64 = 0.0;
    let mut TRANS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SA: f64 = 0.0;
    let mut CA: f64 = 0.0;
    let mut SD: f64 = 0.0;
    let mut CD: f64 = 0.0;
    let mut DX: f64 = 0.0;
    let mut DX1: f64 = 0.0;
    let mut DY: f64 = 0.0;
    let mut DY1: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut Q: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut RA: f64 = 0.0;
    let mut PRATE: f64 = 0.0;
    let mut RB: f64 = 0.0;
    let mut SAN: f64 = 0.0;
    let mut SF: f64 = 0.0;
    let mut SN: f64 = 0.0;
    let mut TEMP = StackArray::<f64, 3>::new(1..=3);
    let mut X1: f64 = 0.0;
    let mut Y1: f64 = 0.0;
    let mut H: f64 = 0.0;
    let mut K: f64 = 0.0;
    let mut L: f64 = 0.0;
    let mut ML: f64 = 0.0;
    let mut NFAC: f64 = 0.0;
    let mut EECAN: f64 = 0.0;
    let mut XHOLD = StackArray::<f64, 6>::new(1..=6);
    let mut VF = StackArray::<f64, 3>::new(1..=3);
    let mut VG = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB Functions.
    //

    //
    // LOCAL VARIABLES
    //

    //
    // Constants computed on first pass
    //

    //
    // Standard SPICE exception handling code.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"EQNCPV", ctx)?;

    //
    // The first time through this routine we fetch the various
    // constants we need for this routine.
    //
    if save.FIRST {
        save.FIRST = false;
        save.PI2 = TWOPI(ctx);
    }

    //
    // Take care of the various errors that can arise with the
    // input elements.
    //
    if (EQEL[1] <= 0.0) {
        SETMSG(b"The semi-major axis supplied to EQNCPV was non-positive. The value is required to be positive by this routine. The value supplied was #. ", ctx);

        ERRDP(b"#", EQEL[1], ctx);
        SIGERR(b"SPICE(BADSEMIAXIS)", ctx)?;
        CHKOUT(b"EQNCPV", ctx)?;
        return Ok(());
    }

    ECC = f64::sqrt(((EQEL[2] * EQEL[2]) + (EQEL[3] * EQEL[3])));

    if (ECC > 0.9) {
        SETMSG(b"The routine EQNCPV can reliably evaluate states from equinoctial elements if the eccentricity of the orbit associated with the elements is less than 0.9.  The eccentricity associated with the elements supplies is #.  The values of H and K are: # and # respectively. ", ctx);

        ERRDP(b"#", ECC, ctx);
        ERRDP(b"#", EQEL[2], ctx);
        ERRDP(b"#", EQEL[3], ctx);
        SIGERR(b"SPICE(ECCOUTOFRANGE)", ctx)?;
        CHKOUT(b"EQNCPV", ctx)?;
        return Ok(());
    }

    //
    // Form the transformation from planetary equator to the inertial
    // reference frame.
    //
    SA = f64::sin(RAPOL);
    CA = f64::cos(RAPOL);

    SD = f64::sin(DECPOL);
    CD = f64::cos(DECPOL);

    TRANS[[1, 1]] = -SA;
    TRANS[[1, 2]] = -(CA * SD);
    TRANS[[1, 3]] = (CA * CD);

    TRANS[[2, 1]] = CA;
    TRANS[[2, 2]] = -(SA * SD);
    TRANS[[2, 3]] = (SA * CD);

    TRANS[[3, 1]] = 0.0;
    TRANS[[3, 2]] = CD;
    TRANS[[3, 3]] = SD;
    //
    // Compute the offset of the input epoch (ET) from the
    // epoch of the elements.
    //
    DT = (ET - EPOCH);

    //
    // Obtain the elements, rates, and other parameters. First get
    // the semi-major axis.
    //
    A = EQEL[1];

    //
    // Recall that H and K at the epoch of the elements are in
    // EQEL(2) and EQEL(3) respectively.
    //
    //    H_0 = E*Sin(ARGP_0 + NODE_0 )
    //    K_0 = E*Cos(ARGP_0 + NODE_0 )
    //
    // The values of H and K at the epoch of interest is
    //
    //    H_dt = E*Sin(ARGP_0 + NODE_0 + dt*d(ARGP+NODE)/dt )
    //    K_dt = E*Cos(ARGP_0 + NODE_0 + dt*d(ARGP+NODE)/dt )
    //
    // But using the identities Sin(A+B) = Sin(A)Cos(B) + Sin(B)Cos(A)
    //                          Cos(A+B) = Cos(A)Cos(B) - Sin(A)Sin(B)
    //
    // We can re-write the expression for H_dt and K_dt as
    //
    //    H_dt = E*Sin(ARGP_0 + NODE_0 )Cos(dt*d(ARGP+NODE)/dt )
    //         + E*Cos(ARGP_0 + NODE_0 )Sin(dt*d(ARGP+NODE)/dt )
    //
    //
    //         = H_0 * Cos(dt*d(ARGP+NODE)/dt )
    //         + K_0 * Sin(dt*d(ARGP+NODE)/dt )
    // and
    //
    //    K_dt = E*Cos(ARGP_0 + NODE_0)Cos(dt*d(ARGP+NODE)/dt)
    //         - E*Sin(ARGP_0 + NODE_0)Sin(dt*d(ARGP+NODE)/dt)
    //
    //         = K_0 * Cos(dt*d(ARGP+NODE)/dt)
    //         - H_0 * Sin(dt*d(ARGP+NODE)/dt)
    //
    // Thus we can easily compute H and K at the current epoch.
    // Recall that the derivative of the longitude of periapse is
    // in entry 7 of EQEL.
    //
    DLPDT = EQEL[7];
    DLP = (DT * DLPDT);

    CAN = f64::cos(DLP);
    SAN = f64::sin(DLP);

    H = ((EQEL[2] * CAN) + (EQEL[3] * SAN));
    K = ((EQEL[3] * CAN) - (EQEL[2] * SAN));

    //
    // The mean longitude at epoch is in the 4th element of EQEL.
    //
    L = EQEL[4];

    //
    // The values for P and Q at epoch are stored in entries 5 and 6
    // of the array EQEL.  Recall that
    //
    //    P_0 = TAN(INC/2)*SIN(NODE_0)
    //    Q_0 = TAN(INC/2)*COS(NODE_0)
    //
    // We need P and Q offset from the initial epoch by DT.
    //
    //    P   = TAN(INC/2)*SIN(NODE_0 + dt*dNODE/dt)
    //    Q   = TAN(INC/2)*COS(NODE_0 + dt*dNODE/dt)
    //
    // Applying the same identities as we did before we have
    //
    //    P    = P_0 * Cos( dt*dNODE/dt ) + Q_0 * Sin( dt*dNODE/dt )
    //    Q    = Q_0 * Cos( dt*dNODE/dt ) - P_0 * Sin( dt*dNODE/dt )
    //
    NODEDT = EQEL[9];
    NODE = (DT * NODEDT);

    CN = f64::cos(NODE);
    SN = f64::sin(NODE);

    P = ((EQEL[5] * CN) + (EQEL[6] * SN));
    Q = ((EQEL[6] * CN) - (EQEL[5] * SN));

    MLDT = EQEL[8];
    //
    // We compute the rate of change of the argument of periapse
    // by taking the difference between the rate of the longitude
    // of periapse and the rate of the node.
    //
    PRATE = (DLPDT - NODEDT);

    //
    // Form Broucke's beta parameter
    //
    B = f64::sqrt(((1.0 - (H * H)) - (K * K)));
    B = (1.0 / (1.0 + B));

    //
    // Construct the coordinate axes
    //
    DI = (1.0 / ((1.0 + (P * P)) + (Q * Q)));

    VF[1] = (((1.0 - (P * P)) + (Q * Q)) * DI);
    VF[2] = (((2.0 * P) * Q) * DI);
    VF[3] = -((2.0 * P) * DI);

    VG[1] = (((2.0 * P) * Q) * DI);
    VG[2] = (((1.0 + (P * P)) - (Q * Q)) * DI);
    VG[3] = ((2.0 * Q) * DI);

    //
    // Compute the mean longitude
    //
    ML = (L + intrinsics::DMOD((MLDT * DT), save.PI2));

    //
    // Obtain the eccentric longitude from Kepler's equation
    //
    EECAN = KEPLEQ(ML, H, K, ctx)?;

    //
    // Trigonometric functions of the eccentric longitude
    //
    SF = f64::sin(EECAN);
    CF = f64::cos(EECAN);

    //
    // Position in the orbit plane
    //
    X1 = (A * (((1.0 - (B * f64::powi(H, 2))) * CF) + ((((H * K) * B) * SF) - K)));
    Y1 = (A * (((1.0 - (B * f64::powi(K, 2))) * SF) + ((((H * K) * B) * CF) - H)));

    //
    // Radial distance and functions of the radial distance
    //
    RB = ((H * SF) + (K * CF));
    R = (A * (1.0 - RB));
    RA = (((MLDT * A) * A) / R);
    //
    //
    // Velocity in the orbit plane
    //
    DX1 = (RA * (-SF + ((H * B) * RB)));
    DY1 = (RA * (CF - ((K * B) * RB)));

    //
    // Correction factor for periapsis rate
    //
    NFAC = (1.0 - (DLPDT / MLDT));

    //
    // Include precession in velocity
    //
    DX = ((NFAC * DX1) - (PRATE * Y1));
    DY = ((NFAC * DY1) + (PRATE * X1));

    //
    // Form the planetary mean equator position vector
    //
    VLCOM(X1, VF.as_slice(), Y1, VG.as_slice(), XHOLD.as_slice_mut());

    //
    // Form the planetary mean equator velocity vector
    //
    TEMP[1] = -(NODEDT * XHOLD[2]);
    TEMP[2] = (NODEDT * XHOLD[1]);
    TEMP[3] = 0.0;

    VLCOM3(
        1.0,
        TEMP.as_slice(),
        DX,
        VF.as_slice(),
        DY,
        VG.as_slice(),
        XHOLD.subarray_mut(4),
    );

    //
    // Transform to an inertial state vector
    //
    MXV(TRANS.as_slice(), XHOLD.subarray(1), STATE.subarray_mut(1));
    MXV(TRANS.as_slice(), XHOLD.subarray(4), STATE.subarray_mut(4));

    CHKOUT(b"EQNCPV", ctx)?;
    Ok(())
}
