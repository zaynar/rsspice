//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure T_XFORM ( Test state transformation )
pub fn T_XFORM(
    XFORM: &[f64],
    RMINUS: &[f64],
    RPLUS: &[f64],
    DELTA: f64,
    NRMERR: &mut f64,
    DETERR: &mut f64,
    DRVERR: &mut f64,
    DRLERR: &mut f64,
    DRDIFF: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let XFORM = DummyArray2D::new(XFORM, 1..=6, 1..=6);
    let RMINUS = DummyArray2D::new(RMINUS, 1..=3, 1..=3);
    let RPLUS = DummyArray2D::new(RPLUS, 1..=3, 1..=3);
    let mut DRDIFF = DummyArrayMut2D::new(DRDIFF, 1..=3, 1..=3);
    let mut BLERR: f64 = 0.0;
    let mut DRBLCK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DSCRET = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DSCRTR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DSHARP = StackArray::<f64, 6>::new(1..=6);
    let mut LRBLCK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut ULBLCK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut URBLCK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XPBLCK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XPD = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_XFORM", ctx)?;

    //
    // Extract the 3x3 blocks of the input state
    // transformation matrix.
    //
    spicelib::MOVED(XFORM.subarray([1, 1]), 3, ULBLCK.subarray_mut([1, 1]));
    spicelib::MOVED(XFORM.subarray([1, 2]), 3, ULBLCK.subarray_mut([1, 2]));
    spicelib::MOVED(XFORM.subarray([1, 3]), 3, ULBLCK.subarray_mut([1, 3]));

    spicelib::MOVED(XFORM.subarray([1, 4]), 3, URBLCK.subarray_mut([1, 1]));
    spicelib::MOVED(XFORM.subarray([1, 5]), 3, URBLCK.subarray_mut([1, 2]));
    spicelib::MOVED(XFORM.subarray([1, 6]), 3, URBLCK.subarray_mut([1, 3]));

    spicelib::MOVED(XFORM.subarray([4, 1]), 3, DRBLCK.subarray_mut([1, 1]));
    spicelib::MOVED(XFORM.subarray([4, 2]), 3, DRBLCK.subarray_mut([1, 2]));
    spicelib::MOVED(XFORM.subarray([4, 3]), 3, DRBLCK.subarray_mut([1, 3]));

    spicelib::MOVED(XFORM.subarray([4, 4]), 3, LRBLCK.subarray_mut([1, 1]));
    spicelib::MOVED(XFORM.subarray([4, 5]), 3, LRBLCK.subarray_mut([1, 2]));
    spicelib::MOVED(XFORM.subarray([4, 6]), 3, LRBLCK.subarray_mut([1, 3]));

    //
    // The upper left and lower right blocks should be identical.
    //
    BLERR = spicelib::VDISTG(ULBLCK.as_slice(), LRBLCK.as_slice(), 9);

    if (BLERR != 0.0) {
        spicelib::SETMSG(
            b"L2 distance between blocks on main diagonal is #; this distance must be zero.",
            ctx,
        );
        spicelib::ERRDP(b"#", BLERR, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDMATRIX)", ctx)?;
        spicelib::CHKOUT(b"T_XFORM", ctx)?;
        return Ok(());
    }

    //
    // Start out by finding the norms of rows and columns in
    // the upper left block.
    //
    *NRMERR = 0.0;

    for I in 1..=3 {
        *NRMERR = intrinsics::DMAX1(&[
            *NRMERR,
            f64::abs((1.0 - spicelib::VNORM(ULBLCK.subarray([1, I])))),
        ]);
    }

    spicelib::XPOSE(ULBLCK.as_slice(), XPBLCK.as_slice_mut());

    for I in 1..=3 {
        *NRMERR = intrinsics::DMAX1(&[
            *NRMERR,
            f64::abs((1.0 - spicelib::VNORM(XPBLCK.subarray([1, I])))),
        ]);
    }

    //
    // Find the determinant error of the upper left block.
    //
    *DETERR = f64::abs((1.0 - spicelib::DET(ULBLCK.as_slice())));

    //
    // Compute a discrete derivative of the upper left block using
    // the matrices corresponding to time offsets of +/- DELTA.
    //
    spicelib::QDERIV(
        9,
        RMINUS.as_slice(),
        RPLUS.as_slice(),
        DELTA,
        DSCRET.as_slice_mut(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_XFORM", ctx)?;
        return Ok(());
    }
    //
    // "Sharpen" the discrete derivative:  we know the rows of the
    // discrete derivative matrix should be orthogonal to those of the
    // corresponding rows of the rotation ULBLCK.  We'll find it
    // easier to work with columns of matrices, so let XPD be the
    // transpose of the discrete derivative.
    //
    spicelib::XPOSE(DSCRET.as_slice(), XPD.as_slice_mut());

    for I in 1..=3 {
        //
        // XPBLCK contains the transpose of XFORM's upper left rotation
        // block.  Make a state vector out of the Ith column of XPBLCK
        // and the corresponding discrete derivative.  Find the state of
        // corresponding unit vector; store the velocity of the unit
        // vector in the Ith column of DSCRTR.
        //
        spicelib::VEQU(XPBLCK.subarray([1, I]), RSTATE.as_slice_mut());
        spicelib::VEQU(XPD.subarray([1, I]), RSTATE.subarray_mut(4));
        spicelib::DVHAT(RSTATE.as_slice(), DSHARP.as_slice_mut());
        spicelib::VEQU(DSHARP.subarray(4), DSCRTR.subarray_mut([1, I]));
    }

    //
    // Replace the discrete derivative with the sharpened version
    // we just computed.
    //
    spicelib::XPOSE(DSCRTR.as_slice(), DSCRET.as_slice_mut());

    //
    // Find the L2 error between the discrete derivative and the
    // derivative block of XFORM.  Also find the relative error.
    //
    *DRVERR = spicelib::VDISTG(DSCRET.as_slice(), DRBLCK.as_slice(), 9);
    *DRLERR = spicelib::VRELG(DSCRET.as_slice(), DRBLCK.as_slice(), 9);

    //
    // Find the matrix difference of the derivatives.
    //
    spicelib::VSUBG(
        DSCRET.as_slice(),
        DRBLCK.as_slice(),
        9,
        DRDIFF.as_slice_mut(),
    );

    spicelib::CHKOUT(b"T_XFORM", ctx)?;
    Ok(())
}
