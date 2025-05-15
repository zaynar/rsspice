//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Evaluate a type 17 SPK data record
///
/// Evaluate a single SPK data record from a segment of type 17
/// (Equinoctial Elements).
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
///              RECIN (1)  epoch of the elements in ephemeris seconds
///                         past J2000.
///
///              RECIN (2)-RECIN (10) Equinoctial Elements:
///
///                 RECIN (2)  is the semi-major axis (A) of the orbit.
///
///                 RECIN (3)  is the value of H at the specified
///                            epoch. ( E*SIN(ARGP+NODE) ).
///
///                 RECIN (4)  is the value of K at the specified epoch
///                            ( E*COS(ARGP+NODE) ).
///
///                 RECIN (5)  is the mean longitude (MEAN0+ARGP+NODE)
///                            at the epoch of the elements.
///
///                 RECIN (6)  is the value of P
///                            (TAN(INC/2)*SIN(NODE)) at the specified
///                            epoch.
///
///                 RECIN (7)  is the value of Q
///                            (TAN(INC/2)*COS(NODE)) at the specified
///                            epoch.
///
///                 RECIN (8)  is the rate of the longitude of periapse
///                            (dARGP/dt + dNODE/dt ) at the epoch of
///                            the elements. This rate is assumed to
///                            hold for all time.
///
///                 RECIN (9)  is the derivative of the mean longitude
///                            ( dM/dt + dARGP/dt + dNODE/dt ).  This
///                            rate is assumed to be constant.
///
///                 RECIN (10)  is the rate of the longitude of the
///                             ascending node ( dNODE/dt).
///
///              RECIN (11) Right Ascension of the pole of the
///                         orbital reference system relative to the
///                         reference frame of the associated SPK
///                         segment.
///
///              RECIN (12) Declination of the pole of the
///                         orbital reference system relative to
///                         the reference frame of the associated
///                         SPK segment.
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
///  1)  If the eccentricity is greater than 0.9, the error
///      SPICE(BADECCENTRICITY) is signaled.
///
///  2)  If the semi-major axis is non-positive, the error
///      SPICE(BADSEMIAXIS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine performs a cursory examination of the elements
///  of a type 17 SPK data record and then passes the equinoctial
///  elements contained in that record on to the SPICE routine
///  EQNCPV for evaluation.
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
///        IF ( TYPE .EQ. 17 ) THEN
///
///           CALL SPKR17 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE17 ( ET, RECORD, STATE )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 08-JAN-1997 (WLT)
/// ```
pub fn spke17(
    ctx: &mut SpiceContext,
    et: f64,
    recin: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE17(et, recin, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE17 ( Evaluate a type 17 SPK data record)
pub fn SPKE17(
    ET: f64,
    RECIN: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECIN = DummyArray::new(RECIN, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut A: f64 = 0.0;
    let mut RAPOLE: f64 = 0.0;
    let mut DECPOL: f64 = 0.0;
    let mut H: f64 = 0.0;
    let mut K: f64 = 0.0;
    let mut ECC: f64 = 0.0;
    let mut EPOCH: f64 = 0.0;

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

    CHKIN(b"SPKE17", ctx)?;

    //
    // Fetch the various entities from the input record, first the epoch.
    //
    EPOCH = RECIN[1];
    A = RECIN[2];
    H = RECIN[3];
    K = RECIN[4];
    ECC = f64::sqrt(((H * H) + (K * K)));

    RAPOLE = RECIN[11];
    DECPOL = RECIN[12];

    //
    // Check all the inputs here for obvious failures.  Yes, perhaps
    // this is overkill.  However, there is a lot more computation
    // going on in this routine so that the small amount of overhead
    // here should not be significant.
    //
    if (A <= 0 as f64) {
        SETMSG(b"The semi-major axis supplied to the SPK type 17 evaluator was non-positive.  This value must be positive. The value supplied was #.", ctx);
        ERRDP(b"#", A, ctx);
        SIGERR(b"SPICE(BADSEMIAXIS)", ctx)?;
        CHKOUT(b"SPKE17", ctx)?;
        return Ok(());
    } else if (ECC > 0.9) {
        SETMSG(b"The eccentricity supplied for a type 17 segment is greater than 0.9.  It must be less than 0.9.The value supplied to the type 17 evaluator was #. ", ctx);
        ERRDP(b"#", ECC, ctx);
        SIGERR(b"SPICE(BADECCENTRICITY)", ctx)?;
        CHKOUT(b"SPKE17", ctx)?;
        return Ok(());
    }

    //
    // That's all for here, just plug the elements into the routine
    // knows how to evaluate the equinoctial elements.
    //
    EQNCPV(
        ET,
        EPOCH,
        RECIN.subarray(2),
        RAPOLE,
        DECPOL,
        STATE.as_slice_mut(),
        ctx,
    )?;

    //
    // That's all folks.  Check out and return.
    //
    CHKOUT(b"SPKE17", ctx)?;
    Ok(())
}
