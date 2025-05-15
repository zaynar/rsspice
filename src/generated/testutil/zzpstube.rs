//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZPSTUBE ( Plate set, create tube )
pub fn ZZPSTUBE(
    N: i32,
    VRTCES: &[f64],
    CRVSUB: fn(f64, &mut [f64], &mut [f64], &mut f64, &mut Context) -> f2rust_std::Result<()>,
    NSAMP: i32,
    CLOSED: bool,
    VOUT: &mut [f64],
    POUT: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VRTCES = DummyArray2D::new(VRTCES, 1..=3, 1..);
    let mut VOUT = DummyArrayMut::new(VOUT, LBCELL..);
    let mut POUT = DummyArrayMut::new(POUT, LBCELL..);
    let mut ANGLE: f64 = 0.0;
    let mut CP = StackArray::<f64, 3>::new(1..=3);
    let mut CURVE = StackArray::<f64, 3>::new(1..=3);
    let mut DERIV = StackArray::<f64, 3>::new(1..=3);
    let mut DP: f64 = 0.0;
    let mut DTWIST: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PRVCRV = StackArray::<f64, 3>::new(1..=3);
    let mut PRVDIR = StackArray::<f64, 3>::new(1..=3);
    let mut PRVTWS: f64 = 0.0;
    let mut STEP: f64 = 0.0;
    let mut T: f64 = 0.0;
    let mut TWIST: f64 = 0.0;
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP2 = StackArray::<f64, 3>::new(1..=3);
    let mut CPOUT: i32 = 0;
    let mut CVOUT: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NP: i32 = 0;
    let mut NV: i32 = 0;
    let mut P1: i32 = 0;
    let mut P2: i32 = 0;
    let mut P3: i32 = 0;
    let mut PSTART: i32 = 0;
    let mut VSTART: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZPSTUBE", ctx)?;

    //
    // Check the sample count.
    //
    if (NSAMP < 2) {
        spicelib::SETMSG(
            b"Cross section sample count was #; count must be at least 2.",
            ctx,
        );
        spicelib::ERRINT(b"#", NSAMP, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"ZZPSTUBE", ctx)?;
        return Ok(());
    }

    //
    // Set the independent variable step size.
    //
    STEP = (1.0 / (NSAMP - 1) as f64);

    //
    // Check the space in the output cells.
    //
    NV = (N * NSAMP);
    CVOUT = (3 * NV);

    NP = ((2 * N) * (NSAMP - 1));
    CPOUT = (3 * NP);

    if (spicelib::SIZED(VOUT.as_slice(), ctx)? < CVOUT) {
        spicelib::SETMSG(
            b"Output vertex array size is #; required space is # elements.",
            ctx,
        );
        spicelib::ERRINT(b"#", spicelib::SIZED(VOUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CVOUT, ctx);
        spicelib::SIGERR(b"SPICE(VERTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSTUBE", ctx)?;
        return Ok(());
    }

    if (spicelib::SIZEI(POUT.as_slice(), ctx)? < CPOUT) {
        spicelib::SETMSG(
            b"Output plate array size is #; required space is # elements.",
            ctx,
        );
        spicelib::ERRINT(b"#", spicelib::SIZEI(POUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CPOUT, ctx);
        spicelib::SIGERR(b"SPICE(PLTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSTUBE", ctx)?;
        return Ok(());
    }

    //
    // Look up the initial polygon center and curve direction.
    //
    CRVSUB(
        0.0,
        PRVCRV.as_slice_mut(),
        PRVDIR.as_slice_mut(),
        &mut PRVTWS,
        ctx,
    )?;

    spicelib::VHATIP(PRVDIR.as_slice_mut());

    //
    // Check the order of the cross section vectors.
    //
    for I in 2..=N {
        spicelib::VCRSS(
            VRTCES.subarray([1, (I - 1)]),
            VRTCES.subarray([1, I]),
            CP.as_slice_mut(),
        );

        DP = spicelib::VDOT(CP.as_slice(), PRVDIR.as_slice());

        if (DP <= 0.0) {
            spicelib::SETMSG(b"Cross section vertices are not in strictly increasing order: rotation angle from vertex # to vertex # about the initial curve derivative vector is not positive.", ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRINT(b"#", (I + 1), ctx);
            spicelib::SIGERR(b"SPICE(BADVERTEXORDER)", ctx)?;
            spicelib::CHKOUT(b"ZZPSTUBE", ctx)?;
            return Ok(());
        }
    }

    //
    // Store the input vertices. Each vertex is an offset from
    // CURVE.
    //
    for I in 1..=N {
        spicelib::VROTV(
            VRTCES.subarray([1, I]),
            PRVDIR.as_slice(),
            PRVTWS,
            VTEMP.as_slice_mut(),
        );

        J = ((3 * (I - 1)) + 1);

        spicelib::VADD(VTEMP.as_slice(), PRVCRV.as_slice(), VOUT.subarray_mut(J));
    }

    //
    // Add vertices and plates for each sample.
    //
    VSTART = ((3 * N) + 1);
    PSTART = 1;

    for I in 2..=NSAMP {
        if ((I < NSAMP) || !CLOSED) {
            T = (((I - 1) as f64) * STEP);

            CRVSUB(
                T,
                CURVE.as_slice_mut(),
                DERIV.as_slice_mut(),
                &mut TWIST,
                ctx,
            )?;

            spicelib::VHATIP(DERIV.as_slice_mut());

            DTWIST = (TWIST - PRVTWS);

            //
            // Compute the cross product of the previous and current
            // curve direction.
            //
            spicelib::VCRSS(PRVDIR.as_slice(), DERIV.as_slice(), NORMAL.as_slice_mut());

            ANGLE = f64::asin(spicelib::VNORM(NORMAL.as_slice()));

            if (ANGLE == 0.0) {
                //
                // Just apply twist to the previous cross section.
                //
                K = (((3 * N) * (I - 2)) + 1);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = N;
                    let m3__: i32 = 1;
                    J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        spicelib::VSUB(VOUT.subarray(K), PRVCRV.as_slice(), VTEMP2.as_slice_mut());

                        spicelib::VROTV(
                            VTEMP2.as_slice(),
                            DERIV.as_slice(),
                            DTWIST,
                            VTEMP.as_slice_mut(),
                        );

                        spicelib::VADD(
                            VTEMP.as_slice(),
                            CURVE.as_slice(),
                            VOUT.subarray_mut(VSTART),
                        );

                        VSTART = (VSTART + 3);
                        K = (K + 3);

                        J += m3__;
                    }
                }
            } else {
                //
                // Rotate the cross section by ANGLE about NORMAL.
                // Apply twist to each rotated vertex.
                //
                K = (((3 * N) * (I - 2)) + 1);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = N;
                    let m3__: i32 = 1;
                    J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        spicelib::VSUB(VOUT.subarray(K), PRVCRV.as_slice(), VTEMP2.as_slice_mut());

                        spicelib::VROTV(
                            VTEMP2.as_slice(),
                            NORMAL.as_slice(),
                            ANGLE,
                            VTEMP.as_slice_mut(),
                        );

                        spicelib::VROTV(
                            VTEMP.as_slice(),
                            DERIV.as_slice(),
                            DTWIST,
                            VTEMP2.as_slice_mut(),
                        );

                        spicelib::VADD(
                            VTEMP2.as_slice(),
                            CURVE.as_slice(),
                            VOUT.subarray_mut(VSTART),
                        );

                        VSTART = (VSTART + 3);
                        K = (K + 3);

                        J += m3__;
                    }
                }
            }

            spicelib::VEQU(CURVE.as_slice(), PRVCRV.as_slice_mut());
            spicelib::VEQU(DERIV.as_slice(), PRVDIR.as_slice_mut());

            PRVTWS = TWIST;
        } else {
            //
            // The tube is closed and I is equal to NSAMP. We
            // simply copy the input cross section to obtain
            // the current cross section.
            //
            K = ((3 * N) * (NSAMP - 1));

            spicelib::MOVED(
                &VOUT.subarray(1).to_vec(),
                (3 * N),
                VOUT.subarray_mut((K + 1)),
            );
        }

        //
        // Create plates.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = N;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (J < N) {
                    NEXT = (J + 1);
                } else {
                    NEXT = 1;
                }

                P1 = (((I - 2) * N) + J);
                P2 = (((I - 2) * N) + NEXT);
                P3 = (P1 + N);

                POUT[PSTART] = P1;
                POUT[(PSTART + 1)] = P2;
                POUT[(PSTART + 2)] = P3;

                P1 = (((I - 2) * N) + NEXT);
                P2 = (P1 + N);
                P3 = (((I - 1) * N) + J);

                POUT[(PSTART + 3)] = P1;
                POUT[(PSTART + 4)] = P2;
                POUT[(PSTART + 5)] = P3;

                PSTART = (PSTART + 6);

                J += m3__;
            }
        }
    }

    spicelib::SCARDD(CVOUT, VOUT.as_slice_mut(), ctx)?;
    spicelib::SCARDI(CPOUT, POUT.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"ZZPSTUBE", ctx)?;
    Ok(())
}
