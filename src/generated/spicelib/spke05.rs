//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Evaluate SPK record, type 5
///
/// Evaluate a single SPK data record from a segment of type 5
/// (two body propagation between discrete state vectors).
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
///  RECORD     I   Data record.
///  STATE      O   State (position and velocity).
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is a target epoch, specified as ephemeris seconds past
///           J2000, at which a state vector is to be computed.
///
///  RECORD   is a data record which, when evaluated at epoch ET,
///           will give the state (position and velocity) of some
///           body, relative to some center, in some inertial
///           reference frame.
///
///           The structure of RECORD is:
///
///               RECORD(1)
///                  .            state of the body at epoch 1.
///                  .
///                  .
///               RECORD(6)
///
///               RECORD(7)
///                  .
///                  .            state of the body at epoch 2.
///                  .
///               RECORD(12)
///               RECORD(13)      epoch 1 in seconds past 2000.
///               RECORD(14)      epoch 2 in seconds past 2000.
///               RECORD(15)      GM for the center of motion.
///
///           Epoch 1 and epoch 2 are the times in the segment that
///           bracket ET. If ET is less than the first time in the
///           segment then both epochs 1 and 2 are equal to the
///           first time. And if ET is greater than the last time
///           then, epochs 1 and 2 are set equal to this last time.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state produced by evaluating RECORD at ET.
///           Units are km and km/sec.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there is a problem propagating, subject to the laws of two
///      body motion, either of the states from RECORD to the requested
///      time ET, an error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine interpolates a state from the two reference states
///  contained in RECORD.
///
///  It is assumed that this routine is used in conjunction with
///  the routine SPKR05 as shown here:
///
///     CALL SPKR05 ( HANDLE, DESCR, ET, RECORD         )
///     CALL SPKE05 (                ET, RECORD, STATE  )
///
///  Where it is known in advance that the HANDLE, DESCR pair points
///  to a type 05 data segment.
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
///        IF ( TYPE .EQ. 5 ) THEN
///
///           CALL SPKR05 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE05 ( ET, RECORD, STATE )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 31-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VADD call.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
/// -    SPICELIB Version 1.0.0, 01-APR-1992 (JML) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 31-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VADD call.
///
/// -    SPICELIB Version 1.1.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
/// -    SPICELIB Version 1.0.0, 01-APR-1992 (JML) (WLT) (IMU)
/// ```
pub fn spke05(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE05(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE05 ( Evaluate SPK record, type 5 )
pub fn SPKE05(
    ET: f64,
    RECORD: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = DummyArray::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut ARG: f64 = 0.0;
    let mut DARGDT: f64 = 0.0;
    let mut DENOM: f64 = 0.0;
    let mut DWDT: f64 = 0.0;
    let mut GM: f64 = 0.0;
    let mut NUMER: f64 = 0.0;
    let mut PV = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut S1 = StackArray::<f64, 6>::new(1..=6);
    let mut S2 = StackArray::<f64, 6>::new(1..=6);
    let mut T1: f64 = 0.0;
    let mut T2: f64 = 0.0;
    let mut VCOMP = StackArray::<f64, 3>::new(1..=3);
    let mut VEL = StackArray::<f64, 3>::new(1..=3);
    let mut W: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKE05", ctx)?;
    }

    //
    // Unpack the record, for easier reading.
    //
    MOVED(RECORD.as_slice(), 12, PV.as_slice_mut());

    T1 = RECORD[13];
    T2 = RECORD[14];
    GM = RECORD[15];

    //
    // Evaluate the two states. Call them s_1(t) and s_2(t).
    // Let the position and velocity components be: p_1, v_1, p_2, v_2.
    //
    // The final position is a weighted average.
    //
    // Let
    //
    //    W(t) =  0.5 + 0.5*COS( PI*(t-t1)/(t2-t1) )
    //
    // then
    //
    //    p  = W(t)*p_1(t) + (1 - W(t))*p_2(t)
    //    v  = W(t)*v_1(t) + (1 - W(t))*v_2(t) + W'(t)*(p_1(t) - p_2(t))
    //
    // If t1 = t2, the state is just s(t1).
    //
    //
    // Note: there are a number of weighting schemes we could have
    // used.  This one has the nice property that
    //
    // The graph of W is symmetric about the point
    //
    //
    //    ( (t1+t2)/2,  W( (t1+t2)/2 )
    //
    // The range of W is from 1 to 0.  And the derivative of W is
    // symmetric and zero at both t1 and t2.
    //
    //
    if (T1 != T2) {
        PROP2B(GM, PV.subarray([1, 1]), (ET - T1), S1.as_slice_mut(), ctx)?;
        PROP2B(GM, PV.subarray([1, 2]), (ET - T2), S2.as_slice_mut(), ctx)?;

        NUMER = (ET - T1);
        DENOM = (T2 - T1);
        ARG = ((NUMER * PI(ctx)) / DENOM);
        DARGDT = (PI(ctx) / DENOM);

        W = (0.5 + (0.5 * f64::cos(ARG)));
        DWDT = -((0.5 * f64::sin(ARG)) * DARGDT);

        VLCOMG(
            6,
            W,
            S1.as_slice(),
            (1.0 - W),
            S2.as_slice(),
            STATE.as_slice_mut(),
        );
        VLCOM(
            DWDT,
            S1.as_slice(),
            -DWDT,
            S2.as_slice(),
            VCOMP.as_slice_mut(),
        );
        VADD(STATE.subarray(4), VCOMP.as_slice(), VEL.as_slice_mut());
        VEQU(VEL.as_slice(), STATE.subarray_mut(4));
    } else {
        PROP2B(
            GM,
            PV.subarray([1, 1]),
            (ET - T1),
            STATE.as_slice_mut(),
            ctx,
        )?;
    }

    CHKOUT(b"SPKE05", ctx)?;
    Ok(())
}
