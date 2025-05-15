//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXTRM: i32 = 25;
const MAXREC: i32 = 300;
const MAXDEG: i32 = 23;
const MAXDIM: i32 = MAXTRM;
const MDAOFF: i32 = (MAXTRM + 7);
const MDASIZ: i32 = MAXTRM;
const MAXCOF: i32 = (MAXDEG + 1);
const MSIZE: i32 = (MAXCOF * MAXCOF);

//$Procedure      T_T13XMD ( Type 13 SPK record to extended MDA )
pub fn T_T13XMD(
    T13REC: &[f64],
    MDABEG: f64,
    MDAEND: f64,
    T21REC: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let T13REC = DummyArray::new(T13REC, 1..);
    let mut T21REC = DummyArrayMut::new(T21REC, 1..);
    let mut ACCDER = StackArray::<f64, 25>::new(1..=MDASIZ);
    let mut D: f64 = 0.0;
    let mut DBUFF = StackArray::<f64, 72>::new(1..=(MAXCOF * 3));
    let mut DERIVS = StackArray::<f64, 24>::new(1..=MAXCOF);
    let mut DT = StackArray::<f64, 25>::new(1..=MDASIZ);
    let mut MBUFF = ActualArray::<f64>::new(1..=MSIZE);
    let mut RSYS = StackArray::<f64, 72>::new(1..=(MAXCOF * 3));
    let mut S: f64 = 0.0;
    let mut SPAN: f64 = 0.0;
    let mut SHRINK: f64 = 0.0;
    let mut TVALS = StackArray::<f64, 24>::new(1..=MAXCOF);
    let mut WORK = ActualArray2D::<f64>::new(1..=MAXREC, 1..=MAXREC);
    let mut X: f64 = 0.0;
    let mut DEGP: i32 = 0;
    let mut DLSIZE: i32 = 0;
    let mut FROM: i32 = 0;
    let mut LIDX: i32 = 0;
    let mut MDALOC: i32 = 0;
    let mut N: i32 = 0;
    let mut TO: i32 = 0;
    let mut TSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_T13XMD", ctx)?;

    //
    // Nothing found yet.
    //
    *FOUND = false;

    //
    // For each position component, find the Taylor expansion of its
    // polynomial representation at X. Convert the derivative set to a
    // difference line. Store the difference line in the type 21 record.
    //
    // Get the type 13 polynomial degree from the state count.
    //
    N = intrinsics::IDNINT(T13REC[1]);

    if spicelib::ODD(N) {
        spicelib::SETMSG(b"Window size is #; must be even", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        spicelib::CHKOUT(b"T_T13XMD", ctx)?;
        return Ok(());
    }

    DEGP = ((2 * N) - 1);

    if ((DEGP < 1) || (DEGP > MAXDEG)) {
        spicelib::SETMSG(b"Polynomial degree is #; must be in range 1:#.", ctx);
        spicelib::ERRINT(b"#", DEGP, ctx);
        spicelib::ERRINT(b"#", MAXDEG, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_T13XMD", ctx)?;
        return Ok(());
    }

    //
    // Let TSTART be the index in T13REC where the time values start.
    //
    TSTART = (2 + (6 * N));

    //
    // Find the span of the time values in the record.
    //
    SPAN = (T13REC[((TSTART + N) - 1)] - T13REC[TSTART]);

    //
    // Ensure that at least one time tag in the input record
    // strictly precedes MDAEND.
    //
    LIDX = spicelib::LSTLTD(MDAEND, N, T13REC.subarray(TSTART));

    if (LIDX == 0) {
        spicelib::SETMSG(
            b"Input time MDAEND = #; is not greater than first epoch in type 13 record #.",
            ctx,
        );
        spicelib::ERRDP(b"#", MDAEND, ctx);
        spicelib::ERRDP(b"#", T13REC[TSTART], ctx);
        spicelib::SIGERR(b"SPICE(TIMEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_T13XMD", ctx)?;
        return Ok(());
    }

    if (MDAEND > T13REC[((TSTART + N) - 1)]) {
        spicelib::SETMSG(
            b"Input time MDAEND = #; exceeds last epoch in type 13 record #.",
            ctx,
        );
        spicelib::ERRDP(b"#", MDAEND, ctx);
        spicelib::ERRDP(b"#", T13REC[((TSTART + N) - 1)], ctx);
        spicelib::SIGERR(b"SPICE(TIMEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_T13XMD", ctx)?;
        return Ok(());
    }

    //
    // Transform the time tags by scaling their distance from
    // the central point of the expansion.  We pick the scale to
    // make the width of the interval from first to last
    // transformed tag equal to 1.
    //
    for I in 1..=N {
        TVALS[I] = ((T13REC[((TSTART - 1) + I)] - MDAEND) / SPAN);
    }

    //
    // Let X be the transformed center of the expansion.
    //
    X = 0.0;

    //
    // Let D be the step size for the difference line.
    // D will be equal to the length of the interval
    // over which the polynomial expansion is applicable.
    //
    D = (MDAEND - MDABEG);

    //
    // Rather than evaluating the derivatives of our expansion, we'll
    // evaluate the derivatives of a related expansion that has support
    // on a shorter interval.
    //
    //    This shrinking operation was originally done to work around a
    //    numeric overflow problem in separate software run on a VAX and
    //    subject to configuration control restrictions that prevented
    //    correction of the problem.
    //
    //    Since the algorithm works and since there is no strong
    //    reason to change it, it is left in its original form.
    //
    //
    SHRINK = intrinsics::DMIN1(&[1.0, (1.0 / SPAN)]);

    //
    // Pack the function and derivative values.
    //
    for I in 1..=3 {
        TO = (1 + ((2 * N) * (I - 1)));

        for J in 1..=N {
            FROM = ((1 + ((J - 1) * 6)) + I);
            RSYS[TO] = T13REC[FROM];

            FROM = (((1 + ((J - 1) * 6)) + I) + 3);
            RSYS[(TO + N)] = (T13REC[FROM] * SPAN);

            TO = (TO + 1);
        }
    }

    //
    //
    // Construct a coefficient matrix.
    //
    T_TAYMAT(N, TVALS.as_slice(), X, MBUFF.as_slice_mut(), ctx)?;

    //
    // Solve for the Taylor coefficients at X, and compute derivatives
    // for all three vector components.
    //
    T_TAYHRM(
        N,
        3,
        MBUFF.as_slice_mut(),
        RSYS.as_slice_mut(),
        DBUFF.as_slice_mut(),
        FOUND,
        ctx,
    )?;

    if !*FOUND {
        spicelib::CHKOUT(b"T_T13XMD", ctx)?;
        return Ok(());
    }

    for I in 1..=3 {
        //
        // Unpack the derivatives for the Ith component.
        //
        FROM = (1 + ((2 * N) * (I - 1)));

        spicelib::MOVED(DBUFF.subarray(FROM), (2 * N), DERIVS.as_slice_mut());

        //
        // Now that we have the derivatives, obtain the corresponding
        // difference line and write it into the type 21 output record.
        // The record description from SPKE21 is copied below.
        //
        // Name     Dimension  Description
        // ------   ---------  -------------------------------
        // TL               1  Final epoch of record
        // G           MAXDIM  Stepsize function vector
        // REFPOS           3  Reference position vector
        // REFVEL           3  Reference velocity vector
        // DT      MAXDIM,NTE  Modified divided difference arrays
        // KQMAX1           1  Maximum integration order plus 1
        // KQ             NTE  Integration order array
        //
        //
        // Scale velocity and acceleration to compensate for the
        // shrinkage factor. The higher derivatives are scaled
        // separately.
        //
        DERIVS[2] = (DERIVS[2] * SHRINK);
        DERIVS[3] = (DERIVS[3] * f64::powi(SHRINK, 2));

        //
        // Compute the scaled step size S.
        //
        S = (D * SHRINK);

        //
        // We're going to scale the Jth element of the acceleration
        // portion (starting with the derivative of acceleration) of our
        // derivatives array by SHRINK**2 * S**J.  Since S can be large,
        // we must do this carefully.  We arrange our multiplications so
        // that we never compute a large power of S directly.
        //
        spicelib::CLEARD(MDASIZ, ACCDER.as_slice_mut());

        spicelib::MOVED(DERIVS.subarray(4), (DEGP - 2), ACCDER.as_slice_mut());

        for J in 1..=(DEGP - 2) {
            ACCDER[J] = ((ACCDER[J] * S) * f64::powi(SHRINK, 2));

            for K in 1..=(J - 1) {
                ACCDER[J] = (ACCDER[J] * S);
            }
        }

        spicelib::MOVED(ACCDER.as_slice(), (DEGP - 2), DERIVS.subarray_mut(4));

        spicelib::CLEARD(MDASIZ, DT.as_slice_mut());

        //
        // Compute the difference table for the current polynomial:
        //
        T_PD2DT(
            (DEGP - 2),
            DERIVS.subarray_mut(3),
            WORK.as_slice_mut(),
            DT.as_slice_mut(),
            ctx,
        );

        MDALOC = ((MDAOFF + 1) + ((I - 1) * MDASIZ));

        spicelib::MOVED(DT.as_slice(), MDASIZ, T21REC.subarray_mut(MDALOC));

        //
        // Put into the output array the reference position and velocity
        // values for the current state component.
        //
        MDALOC = ((2 + MDASIZ) + (2 * (I - 1)));

        T21REC[MDALOC] = DERIVS[1];

        MDALOC = (MDALOC + 1);

        T21REC[MDALOC] = DERIVS[2];
    }
    //
    // Fill in the rest of the type 21 record.
    //
    // Reference epoch.  This is also the final epoch covered by
    // the record:
    //
    T21REC[1] = MDAEND;

    //
    // Stepsize values:
    //
    spicelib::FILLD(D, MDASIZ, T21REC.subarray_mut(2));

    //
    // Maximum integration order plus 1:
    //
    MDALOC = ((((1 + MDASIZ) + (2 * 3)) + (3 * MDASIZ)) + 1);

    T21REC[MDALOC] = (MDASIZ + 1) as f64;

    //
    // Integration orders for each component:
    //
    MDALOC = (MDALOC + 1);

    spicelib::FILLD((MDASIZ as f64), 3, T21REC.subarray_mut(MDALOC));

    //
    // Shift the record to make room for the initial size value.
    //
    DLSIZE = ((4 * MDASIZ) + 11);

    for I in intrinsics::range(DLSIZE, 1, -1) {
        T21REC[(I + 1)] = T21REC[I];
    }

    T21REC[1] = (MAXDIM as f64);

    //
    // FOUND is set; it was set by TAYHRM.
    //
    spicelib::CHKOUT(b"T_T13XMD", ctx)?;
    Ok(())
}
