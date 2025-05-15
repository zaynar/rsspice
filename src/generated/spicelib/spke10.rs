//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NGRAVS: i32 = 3;
const NGRAVC: i32 = 8;
const WGS721: i32 = 1;
const WGS72: i32 = 2;
const WGS84: i32 = 3;
const P_RAD: i32 = 1;
const P_XKE: i32 = 2;
const P_MU: i32 = 3;
const P_TUMN: i32 = 4;
const P_J2: i32 = 5;
const P_J3: i32 = 6;
const P_J4: i32 = 7;
const P_J3J2: i32 = 8;
const K_J2: i32 = 1;
const K_J3: i32 = 2;
const K_J4: i32 = 3;
const K_KE: i32 = 4;
const K_QO: i32 = 5;
const K_SO: i32 = 6;
const K_ER: i32 = 7;
const K_AE: i32 = 8;
const NGEO: i32 = K_AE;
const AFSPC: i32 = 1;
const IMPRVD: i32 = 2;
const KNDT20: i32 = 1;
const KNDD60: i32 = 2;
const KBSTAR: i32 = 3;
const KINCL: i32 = 4;
const KNODE0: i32 = 5;
const KECC: i32 = 6;
const KOMEGA: i32 = 7;
const KMO: i32 = 8;
const KNO: i32 = 9;
const KEPOCH: i32 = 10;
const NELEMS: i32 = KEPOCH;
const NANGS: i32 = 4;
const TIME1: i32 = (NGEO + KEPOCH);
const TIME2: i32 = ((NELEMS + NANGS) + TIME1);
const MODEL1: i32 = (NGEO + 1);
const ANGS1: i32 = (MODEL1 + NELEMS);
const MODEL2: i32 = (ANGS1 + NANGS);

struct SaveVars {
    J2TM: StackArray2D<f64, 36>,
    TM2J: StackArray2D<f64, 36>,
    S1: StackArray<f64, 6>,
    S2: StackArray<f64, 6>,
    TMPSTA: StackArray<f64, 6>,
    VCOMP: StackArray<f64, 3>,
    ARG: f64,
    DARGDT: f64,
    DENOM: f64,
    DWDT: f64,
    MYPI: f64,
    NUMER: f64,
    T1: f64,
    T2: f64,
    W: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut J2TM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut TM2J = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut S1 = StackArray::<f64, 6>::new(1..=6);
        let mut S2 = StackArray::<f64, 6>::new(1..=6);
        let mut TMPSTA = StackArray::<f64, 6>::new(1..=6);
        let mut VCOMP = StackArray::<f64, 3>::new(1..=3);
        let mut ARG: f64 = 0.0;
        let mut DARGDT: f64 = 0.0;
        let mut DENOM: f64 = 0.0;
        let mut DWDT: f64 = 0.0;
        let mut MYPI: f64 = 0.0;
        let mut NUMER: f64 = 0.0;
        let mut T1: f64 = 0.0;
        let mut T2: f64 = 0.0;
        let mut W: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            J2TM,
            TM2J,
            S1,
            S2,
            TMPSTA,
            VCOMP,
            ARG,
            DARGDT,
            DENOM,
            DWDT,
            MYPI,
            NUMER,
            T1,
            T2,
            W,
            FIRST,
        }
    }
}

/// Evaluate SPK record, type 10
///
/// Evaluate a single SPK data record from a segment of type 10
/// (NORAD two-line element sets.). This evaluator uses algorithms
/// as described in Vallado 2006 \[4].
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
///                  .            Geophysical Constants such as
///                  .            GM, J2, J3, J4, etc.
///                  .
///               RECORD(NGEO)
///
///               RECORD(NGEO + 1)
///                  .
///                  .            elements and epoch for the body
///                  .            at epoch 1.
///                  .
///               RECORD(NGEO + NELEMS )
///
///               RECORD(NGEO + NELEMS + 1)
///                  .
///                  .            elements and epoch for the body
///                  .            at epoch 2.
///                  .
///               RECORD(NGEO + 2*NELEMS )
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
///           Units are km and km/sec relative to the J2000
///           reference frame.
/// ```
///
/// # Parameters
///
/// ```text
///  NGEO     is the number of geophysical constants for SGP4 SPK
///           records.
///
///  AFSPC    set the SGP4 propagator to use the original
///           Space Track #3 GST algorithm as described in Hoots [1];
///           value defined in zzsgp4.inc.
///
///  IMPRVD   set the SGP4 propagator to use the improved GST
///           algorithm as defined in Vallado [4]; value defined in
///           zzsgp4.inc.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If a problem occurs when evaluating the two-line elements, an
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine interpolates a state from the two reference sets
///  of two-line element sets contained in RECORD.
///
///  It is assumed that this routine is used in conjunction with
///  the routine SPKR10 as shown here:
///
///     CALL SPKR10   ( HANDLE, DESCR, ET, RECORD         )
///     CALL SPKE10   (                ET, RECORD, STATE  )
///
///  Where it is known in advance that the HANDLE, DESCR pair points
///  to a type 10 data segment.
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
///        IF ( TYPE .EQ. 10 ) THEN
///
///           CALL SPKR10 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///           CALL SPKE10 ( ET, RECORD, STATE )
///               .
///               .  Check out the evaluated state.
///               .
///        END IF
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  F. Hoots and R. Roehrich, "Spacetrack Report #3: Models for
///       Propagation of the NORAD Element Sets," U.S. Air Force
///       Aerospace Defense Command, Colorado Springs, CO, 1980.
///
///  [2]  F. Hoots, "Spacetrack Report #6: Models for Propagation of
///       Space Command Element Sets,"  U.S. Air Force Aerospace
///       Defense Command, Colorado Springs, CO, 1986.
///
///  [3]  F. Hoots, P. Schumacher and R. Glover, "History of Analytical
///       Orbit Modeling in the U. S. Space Surveillance System,"
///       Journal of Guidance, Control, and Dynamics. 27(2):174-185,
///       2004.
///
///  [4]  D. Vallado, P. Crawford, R. Hujsak and T. Kelso, "Revisiting
///       Spacetrack Report #3," paper AIAA 2006-6753 presented at the
///       AIAA/AAS Astrodynamics Specialist Conference, Keystone, CO.,
///       August 21-24, 2006.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 10-OCT-2021 (JDR) (EDW)
///
///         Use of modified ZZTEME to eliminate a matrix inversion.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.0.0, 18-FEB-2015 (EDW)
///
///         Evaluator now uses Vallado derived propagator as described
///         in Vallado 2006 [4].
///
/// -    SPICELIB Version 2.0.0, 01-JAN-2011 (EDW)
///
///         Correction of state transformation calculation. Algorithm
///         now computes state transformation as from TEME to J2000.
///         The previous version of this routine calculated TETE to
///         J2000.
///
/// -    SPICELIB Version 1.1.0, 01-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MTXV and VADD calls.
///
/// -    SPICELIB Version 1.0.0, 18-JUL-1997 (WLT)
/// ```
pub fn spke10(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE10(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE10 ( Evaluate SPK record, type 10 )
pub fn SPKE10(
    ET: f64,
    RECORD: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RECORD = DummyArray::new(RECORD, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // The nutation in obliquity and longitude as well as their rates
    // follow the elements.  So we've got four angles/angle rates
    // following the elements
    //

    //
    // The locations of the epochs and the starts of the element
    // sets are given below.
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
        CHKIN(b"SPKE10", ctx)?;
    }

    if save.FIRST {
        save.FIRST = false;
        save.MYPI = PI(ctx);
    }

    //
    // Fetch the two epochs stored in the record.
    //
    save.T1 = RECORD[TIME1];
    save.T2 = RECORD[TIME2];

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
    // Note: there are a number of weighting schemes we could have
    // used.  This one has the nice property that
    //
    // The graph of W is symmetric about the point
    //
    //    ( (t1+t2)/2,  W( (t1+t2)/2 ) )
    //
    // The range of W is from 1 to 0. The derivative of W is
    // symmetric and zero at both t1 and t2.
    //

    if (save.T1 != save.T2) {
        //
        // Initialize then propagate.
        //
        // XXSGP4E returns on entry if XXSGP4I signals an error.
        //

        //
        // Evaluate first TLE.
        //
        XXSGP4I(RECORD.subarray(1), RECORD.subarray(MODEL1), AFSPC, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKE10", ctx)?;
            return Ok(());
        }

        //
        // Time from epoch of set 1 in minutes.
        //
        XXSGP4E(((ET - save.T1) / 60.0), save.S1.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKE10", ctx)?;
            return Ok(());
        }

        //
        // Evaluate second TLE.
        //
        XXSGP4I(RECORD.subarray(1), RECORD.subarray(MODEL2), AFSPC, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKE10", ctx)?;
            return Ok(());
        }

        //
        // Time from epoch of set 2 in minutes.
        //
        XXSGP4E(((ET - save.T2) / 60.0), save.S2.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKE10", ctx)?;
            return Ok(());
        }

        //
        // Compute the weighting function that we'll need later
        // when we combine states 1 and 2.
        //
        save.NUMER = (ET - save.T1);
        save.DENOM = (save.T2 - save.T1);
        save.ARG = ((save.NUMER * save.MYPI) / save.DENOM);
        save.DARGDT = (save.MYPI / save.DENOM);

        save.W = (0.5 + (0.5 * f64::cos(save.ARG)));
        save.DWDT = -((0.5 * f64::sin(save.ARG)) * save.DARGDT);

        //
        // Now compute the weighted average of the two states.
        //
        VLCOMG(
            6,
            save.W,
            save.S1.as_slice(),
            (1.0 - save.W),
            save.S2.as_slice(),
            STATE.as_slice_mut(),
        );
        VLCOM(
            save.DWDT,
            save.S1.as_slice(),
            -save.DWDT,
            save.S2.as_slice(),
            save.VCOMP.as_slice_mut(),
        );
        VADD(
            STATE.subarray(4),
            save.VCOMP.as_slice(),
            save.TMPSTA.subarray_mut(4),
        );
        VEQU(save.TMPSTA.subarray(4), STATE.subarray_mut(4));
    } else {
        //
        // Evaluate the TLE.
        //
        XXSGP4I(RECORD.subarray(1), RECORD.subarray(MODEL1), AFSPC, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKE10", ctx)?;
            return Ok(());
        }

        //
        // Time from epoch of set 1 in minutes.
        //
        XXSGP4E(((ET - save.T1) / 60.0), STATE.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKE10", ctx)?;
            return Ok(());
        }
    }

    //
    // Finally, convert the TEME state to J2000.  First
    // calculate the mapping from J2000 to TEME (J2TM), and from
    // TEME to J2000 (TM2J) at time ET...
    //
    //                     -1
    // Note that J2TM = TM2J
    //
    ZZTEME(ET, save.J2TM.as_slice_mut(), save.TM2J.as_slice_mut(), ctx)?;

    //
    // ...now convert the TEME state to a J2000 state.
    //
    MXVG(
        save.TM2J.as_slice(),
        STATE.as_slice(),
        6,
        6,
        save.TMPSTA.as_slice_mut(),
    );
    MOVED(save.TMPSTA.as_slice(), 6, STATE.as_slice_mut());

    CHKOUT(b"SPKE10", ctx)?;
    Ok(())
}
