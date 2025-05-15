//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXTRM: i32 = 25;

struct SaveVars {
    TL: f64,
    G: StackArray<f64, 25>,
    REFPOS: StackArray<f64, 3>,
    REFVEL: StackArray<f64, 3>,
    DT: StackArray2D<f64, 75>,
    KQMAX1: i32,
    KQ: StackArray<i32, 3>,
    FC: StackArray<f64, 25>,
    SUM: f64,
    DELTA: f64,
    TP: f64,
    WC: StackArray<f64, 24>,
    W: StackArray<f64, 27>,
    MAXDIM: i32,
    MQ2: i32,
    KS1: i32,
    KS: i32,
    KQQ: i32,
    JX: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TL: f64 = 0.0;
        let mut G = StackArray::<f64, 25>::new(1..=MAXTRM);
        let mut REFPOS = StackArray::<f64, 3>::new(1..=3);
        let mut REFVEL = StackArray::<f64, 3>::new(1..=3);
        let mut DT = StackArray2D::<f64, 75>::new(1..=MAXTRM, 1..=3);
        let mut KQMAX1: i32 = 0;
        let mut KQ = StackArray::<i32, 3>::new(1..=3);
        let mut FC = StackArray::<f64, 25>::new(1..=MAXTRM);
        let mut SUM: f64 = 0.0;
        let mut DELTA: f64 = 0.0;
        let mut TP: f64 = 0.0;
        let mut WC = StackArray::<f64, 24>::new(1..=(MAXTRM - 1));
        let mut W = StackArray::<f64, 27>::new(1..=(MAXTRM + 2));
        let mut MAXDIM: i32 = 0;
        let mut MQ2: i32 = 0;
        let mut KS1: i32 = 0;
        let mut KS: i32 = 0;
        let mut KQQ: i32 = 0;
        let mut JX: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0)].into_iter();
            FC[1] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TL,
            G,
            REFPOS,
            REFVEL,
            DT,
            KQMAX1,
            KQ,
            FC,
            SUM,
            DELTA,
            TP,
            WC,
            W,
            MAXDIM,
            MQ2,
            KS1,
            KS,
            KQQ,
            JX,
        }
    }
}

/// S/P Kernel, evaluate, type 21
///
/// Evaluate a single SPK data record from a segment of type 21
/// (Extended Difference Lines).
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Evaluation epoch.
///  RECORD     I   Data record.
///  STATE      O   State (position and velocity).
///  MAXTRM     P   Maximum number of terms per difference table
///                 component.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is an epoch at which a state vector is to be
///           computed. The epoch is represented as seconds past
///           J2000 TDB.
///
///  RECORD   is a data record which, when evaluated at epoch ET,
///           will give the state (position and velocity) of an
///           ephemeris object, relative to its center of motion,
///           in an inertial reference frame.
///
///           The contents of RECORD are as follows:
///
///              RECORD(1):         The difference table size per
///                                 Cartesian component. Call this
///                                 size MAXDIM; then the difference
///                                 line (MDA) size DLSIZE is
///
///                                   ( 4 * MAXDIM ) + 11
///
///              RECORD(2)
///                 ...
///              RECORD(1+DLSIZE):  An extended difference line.
///                                 The contents are:
///
///                 Dimension  Description
///                 ---------  ----------------------------------
///                 1          Reference epoch of difference line
///                 MAXDIM     Stepsize function vector
///                 1          Reference position vector,  x
///                 1          Reference velocity vector,  x
///                 1          Reference position vector,  y
///                 1          Reference velocity vector,  y
///                 1          Reference position vector,  z
///                 1          Reference velocity vector,  z
///                 MAXDIM,3   Modified divided difference
///                            arrays (MDAs)
///                 1          Maximum integration order plus 1
///                 3          Integration order array
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state resulting from evaluation of the input
///           record at ET. Units are km and km/sec.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXTRM   is the maximum number of terms allowed in
///           each component of the difference table
///           contained in the input argument RECORD.
///           See the INCLUDE file spk21.inc for the value
///           of MAXTRM.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the maximum table size of the input record exceeds
///      MAXTRM, the error SPICE(DIFFLINETOOLARGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The exact format and structure of type 21 (difference lines)
///  segments are described in the SPK Required Reading file.
///
///  SPKE21 is a modified version of SPKE01. The routine has been
///  generalized to support variable size difference lines.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  F.T. Krogh         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 14-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved SPK
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 1.0.0, 03-FEB-2014 (NJB) (FTK) (WLT) (IMU)
/// ```
pub fn spke21(
    ctx: &mut SpiceContext,
    et: f64,
    record: &[f64],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    SPKE21(et, record, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKE21 ( S/P Kernel, evaluate, type 21 )
pub fn SPKE21(
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
    // Local parameters
    //

    //
    // Local variables
    //
    // The names below are original to the routine. They correspond
    // roughly to the original memos written by Fred Krogh to explain
    // how all this stuff really works.
    //

    //
    // Save everything between calls.
    //

    //
    // Initial values
    //

    //
    // Use discovery check-in.
    //
    // If the RETURN function is set, don't even bother with this.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    //
    // The first element of the input record is the dimension
    // of the difference table MAXDIM.
    //
    save.MAXDIM = intrinsics::IDNINT(RECORD[1]);

    if (save.MAXDIM > MAXTRM) {
        CHKIN(b"SPKE21", ctx)?;
        SETMSG(b"The input record has a maximum table dimension of #, while the maximum supported by this routine is #. It is possible that this problem is due to your SPICE Toolkit being out of date.", ctx);
        ERRINT(b"#", save.MAXDIM, ctx);
        ERRINT(b"#", MAXTRM, ctx);
        SIGERR(b"SPICE(DIFFLINETOOLARGE)", ctx)?;
        CHKOUT(b"SPKE21", ctx)?;
        return Ok(());
    }

    //
    // Unpack the contents of the MDA array.
    //
    //    Name     Dimension  Description
    //    ------   ---------  -------------------------------
    //    TL               1  Reference epoch of record
    //    G           MAXDIM  Stepsize function vector
    //    REFPOS           3  Reference position vector
    //    REFVEL           3  Reference velocity vector
    //    DT      MAXDIM,NTE  Modified divided difference arrays
    //    KQMAX1           1  Maximum integration order plus 1
    //    KQ             NTE  Integration order array
    //
    // For our purposes, NTE is always 3.
    //

    MOVED(RECORD.subarray(2), 1, std::slice::from_mut(&mut save.TL));
    MOVED(RECORD.subarray(3), save.MAXDIM, save.G.as_slice_mut());

    //
    // Collect the reference position and velocity.
    //
    save.REFPOS[1] = RECORD[(save.MAXDIM + 3)];
    save.REFVEL[1] = RECORD[(save.MAXDIM + 4)];

    save.REFPOS[2] = RECORD[(save.MAXDIM + 5)];
    save.REFVEL[2] = RECORD[(save.MAXDIM + 6)];

    save.REFPOS[3] = RECORD[(save.MAXDIM + 7)];
    save.REFVEL[3] = RECORD[(save.MAXDIM + 8)];

    //
    // Initializing the difference table is one aspect of this routine
    // that's a bit different from SPKE01. Here the first dimension of
    // the table in the input record can be smaller than MAXTRM. So, we
    // must transfer separately the portions of the table corresponding
    // to each component.
    //
    for I in 1..=3 {
        MOVED(
            RECORD.subarray(((I * save.MAXDIM) + 9)),
            save.MAXDIM,
            save.DT.subarray_mut([1, I]),
        );
    }

    save.KQMAX1 = (RECORD[((4 * save.MAXDIM) + 9)] as i32);
    save.KQ[1] = (RECORD[((4 * save.MAXDIM) + 10)] as i32);
    save.KQ[2] = (RECORD[((4 * save.MAXDIM) + 11)] as i32);
    save.KQ[3] = (RECORD[((4 * save.MAXDIM) + 12)] as i32);

    //
    // Next we set up for the computation of the various differences
    //
    save.DELTA = (ET - save.TL);
    save.TP = save.DELTA;
    save.MQ2 = (save.KQMAX1 - 2);
    save.KS = (save.KQMAX1 - 1);

    //
    // This is clearly collecting some kind of coefficients.
    // The problem is that we have no idea what they are...
    //
    // The G coefficients are supposed to be some kind of step size
    // vector.
    //
    // TP starts out as the delta t between the request time and the
    // difference line's reference epoch. We then change it from DELTA
    // by the components of the stepsize vector G.
    //
    for J in 1..=save.MQ2 {
        //
        // Make sure we're not about to attempt division by zero.
        //
        if (save.G[J] == 0.0) {
            CHKIN(b"SPKE21", ctx)?;
            SETMSG(
                b"A  value of zero was found at index # of the step size vector.",
                ctx,
            );
            ERRINT(b"#", J, ctx);
            SIGERR(b"SPICE(ZEROSTEP)", ctx)?;
            CHKOUT(b"SPKE21", ctx)?;
            return Ok(());
        }

        save.FC[(J + 1)] = (save.TP / save.G[J]);
        save.WC[J] = (save.DELTA / save.G[J]);
        save.TP = (save.DELTA + save.G[J]);
    }

    //
    // Collect KQMAX1 reciprocals.
    //
    for J in 1..=save.KQMAX1 {
        save.W[J] = (1.0 / (J as f64));
    }

    //
    // Compute the W(K) terms needed for the position interpolation
    // (Note,  it is assumed throughout this routine that KS, which
    // starts out as KQMAX1-1 (the ``maximum integration'')
    // is at least 2.
    //
    save.JX = 0;
    save.KS1 = (save.KS - 1);

    while (save.KS >= 2) {
        save.JX = (save.JX + 1);

        for J in 1..=save.JX {
            save.W[(J + save.KS)] = ((save.FC[(J + 1)] * save.W[(J + save.KS1)])
                - (save.WC[J] * save.W[(J + save.KS)]));
        }

        save.KS = save.KS1;
        save.KS1 = (save.KS1 - 1);
    }

    //
    // Perform position interpolation: (Note that KS = 1 right now.
    // We don't know much more than that.)
    //
    for I in 1..=3 {
        save.KQQ = save.KQ[I];
        save.SUM = 0.0;

        for J in intrinsics::range(save.KQQ, 1, -1) {
            save.SUM = (save.SUM + (save.DT[[J, I]] * save.W[(J + save.KS)]));
        }

        STATE[I] = (save.REFPOS[I] + (save.DELTA * (save.REFVEL[I] + (save.DELTA * save.SUM))));
    }

    //
    // Again we need to compute the W(K) coefficients that are
    // going to be used in the velocity interpolation.
    // (Note, at this point, KS = 1, KS1 = 0.)
    //
    for J in 1..=save.JX {
        save.W[(J + save.KS)] =
            ((save.FC[(J + 1)] * save.W[(J + save.KS1)]) - (save.WC[J] * save.W[(J + save.KS)]));
    }

    save.KS = (save.KS - 1);

    //
    // Perform velocity interpolation:
    //
    for I in 1..=3 {
        save.KQQ = save.KQ[I];
        save.SUM = 0.0;

        for J in intrinsics::range(save.KQQ, 1, -1) {
            save.SUM = (save.SUM + (save.DT[[J, I]] * save.W[(J + save.KS)]));
        }

        STATE[(I + 3)] = (save.REFVEL[I] + (save.DELTA * save.SUM));
    }

    Ok(())
}
