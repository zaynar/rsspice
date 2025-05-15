//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ANGTOL: f64 = 0.001;

//$Procedure  CK3SDN ( Down sample type 3 CK data prepared for writing )
pub fn CK3SDN(
    SDNTOL: f64,
    AVFLAG: bool,
    NREC: &mut i32,
    SCLKDP: &mut [f64],
    QUATS: &mut [f64],
    AVVS: &mut [f64],
    NINTS: i32,
    STARTS: &[f64],
    DPARR: &mut [f64],
    INTARR: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SCLKDP = DummyArrayMut::new(SCLKDP, 1..);
    let mut QUATS = DummyArrayMut2D::new(QUATS, 0..=3, 1..);
    let mut AVVS = DummyArrayMut2D::new(AVVS, 1..=3, 1..);
    let STARTS = DummyArray::new(STARTS, 1..);
    let mut DPARR = DummyArrayMut::new(DPARR, 1..);
    let mut INTARR = DummyArrayMut::new(INTARR, 1..);
    let mut FRAC: f64 = 0.0;
    let mut QINTRP = StackArray::<f64, 4>::new(0..=3);
    let mut QKEEPF = StackArray::<f64, 4>::new(0..=3);
    let mut QKEEPL = StackArray::<f64, 4>::new(0..=3);
    let mut QLNEG = StackArray::<f64, 4>::new(0..=3);
    let mut QLINPT = StackArray::<f64, 4>::new(0..=3);
    let mut ANGLE: f64 = 0.0;
    let mut DIST2: f64 = 0.0;
    let mut DIST2A: f64 = 0.0;
    let mut DIST2B: f64 = 0.0;
    let mut DPOS: f64 = 0.0;
    let mut DNEG: f64 = 0.0;
    let mut COSVAL: f64 = 0.0;
    let mut J: i32 = 0;
    let mut INTNRF: i32 = 0;
    let mut INTCRF: i32 = 0;
    let mut INTCRL: i32 = 0;
    let mut KEEPF: i32 = 0;
    let mut KEEPL: i32 = 0;
    let mut NDROPD: i32 = 0;
    let mut LEFT: i32 = 0;
    let mut RIGHT: i32 = 0;
    let mut SKIPIT: bool = false;
    let mut FITOK: bool = false;

    //
    // SPICELIB functions.
    //

    //
    // Local parameters.
    //
    // Tolerance for 180 separation check, in radians.
    //

    //
    // Local variables.
    //

    //
    // SPICELIB functions.
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CK3SDN", ctx)?;
    }

    //
    // Let's do some sanity checks that needed to make sure that future
    // loops and comparisons don't blow up. First, verify that the
    // number pointing records is greater that zero.
    //
    if (*NREC <= 0) {
        spicelib::SETMSG(
            b"The number of pointing records must be greater than zero. It was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", *NREC, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDNUMBEROFRECORDS)", ctx)?;
        spicelib::CHKOUT(b"CK3SDN", ctx)?;
        return Ok(());
    }

    //
    // Then, verify that the number intervals is greater that zero.
    //
    if (NINTS <= 0) {
        spicelib::SETMSG(
            b"The number of interval starts must be greater than zero. It was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", NINTS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDNUMBEROFINTERVALS)", ctx)?;
        spicelib::CHKOUT(b"CK3SDN", ctx)?;
        return Ok(());
    }

    //
    // Then, verify that the number intervals is less than or equal to
    // the number of records.
    //
    if (NINTS > *NREC) {
        spicelib::SETMSG(b"The number of interval starts, #, is not less than or equal to the number of records, #.", ctx);
        spicelib::ERRINT(b"#", NINTS, ctx);
        spicelib::ERRINT(b"#", *NREC, ctx);
        spicelib::SIGERR(b"SPICE(BUFFERSIZESMISMATCH)", ctx)?;
        spicelib::CHKOUT(b"CK3SDN", ctx)?;
        return Ok(());
    }

    //
    // Then verify that the first time in the intervals array is the same
    // as the first time in the records array.
    //
    if (SCLKDP[1] != STARTS[1]) {
        spicelib::SETMSG(
            b"The first interval start time, #, is not the same as the first record time, #.",
            ctx,
        );
        spicelib::ERRDP(b"#", SCLKDP[1], ctx);
        spicelib::ERRDP(b"#", STARTS[1], ctx);
        spicelib::SIGERR(b"SPICE(FIRSTRECORDMISMATCH)", ctx)?;
        spicelib::CHKOUT(b"CK3SDN", ctx)?;
        return Ok(());
    }

    //
    // Finally verify that input down sampling tolerance is not positive
    // number.
    //
    if (SDNTOL < 0.0) {
        spicelib::SETMSG(
            b"The down sampling tolerance must be a non-negative number. It was #.",
            ctx,
        );
        spicelib::ERRDP(b"#", SDNTOL, ctx);
        spicelib::SIGERR(b"SPICE(BADDOWNSAMPLINGTOL)", ctx)?;
        spicelib::CHKOUT(b"CK3SDN", ctx)?;
        return Ok(());
    }

    //
    // This variable will hold to the index of the pointing record that
    // matches the start of the next interval. For the first interval
    // it is set to one.
    //
    INTNRF = 1;

    //
    // We will count the number of points that were dropped.
    //
    NDROPD = 0;

    //
    // Loop through interpolation intervals.
    //
    for I in 1..=NINTS {
        //
        // Assign the index of the pointing record that matches the
        // begin time of this interval.
        //
        INTCRF = INTNRF;

        //
        // Find the index of the pointing record that ends this interval.
        // If this the last interval, it is the last pointing record in
        // pointing buffer.
        //
        if (I == NINTS) {
            INTCRL = *NREC;
        } else {
            //
            // This is not the last interval. To get its end time we need
            // to find the pointing record that matches the start of the
            // next interval and pick the record before it.
            //
            // First we find index of the pointing record that corresponds
            // to the start of the next interval.
            //
            INTNRF = spicelib::BSRCHD(
                STARTS[(I + 1)],
                ((*NREC - INTCRF) + 1),
                SCLKDP.subarray(INTCRF),
            );

            if (INTNRF != 0) {
                //
                // Found index must be adjusted to be relative to the
                // beginning of the buffer. Currently it is relative to the
                // start of the current interval.
                //
                INTNRF = ((INTNRF + INTCRF) - 1);
                //
                // The index of the last record belonging to this interval
                // in the found index minus 1.
                //
                INTCRL = (INTNRF - 1);
            } else {
                //
                // We did not find such record. The input buffer must have
                // been formed improperly for this to happen. Signal an
                // error.
                //
                spicelib::SETMSG(b"Cannot find pointing record with time that matches the start time # (encoded SCLK ticks) of the interpolation interval number #.", ctx);
                spicelib::ERRDP(b"#", STARTS[(I + 1)], ctx);
                spicelib::ERRINT(b"#", (I + 1), ctx);
                spicelib::SIGERR(b"SPICE(INTERVALSTARTNOTFOUND)", ctx)?;
                spicelib::CHKOUT(b"CK3SDN", ctx)?;
                return Ok(());
            }
        }

        //
        // Let's look at the indexes of the pointing records
        // corresponding to the begin and end of this interval. If they
        // are the same (meaning it's a singleton interval) or if they
        // are next to each other (meaning that the whole set of
        // interval's pointing data is comprised of only its begin
        // and end points) there is no down sampling to do.
        //
        SKIPIT = ((INTCRF == INTCRL) || (INTCRF == (INTCRL - 1)));

        //
        // Set initial values for a binary search.
        //
        KEEPF = INTCRF;
        LEFT = INTCRF;
        RIGHT = INTCRL;

        while (!SKIPIT && (KEEPF < INTCRL)) {
            //
            // Set the right endpoint of the interval by dividing the
            // binary search region in half.
            //
            KEEPL = ((LEFT + RIGHT) / 2);

            //
            // Unitize bracketing quaternions as QMINI seems to be
            // very sensitive to that. :)
            //
            spicelib::VHATG(QUATS.subarray([0, KEEPF]), 4, QKEEPF.as_slice_mut());
            spicelib::VHATG(QUATS.subarray([0, KEEPL]), 4, QKEEPL.as_slice_mut());

            //
            // Pick the closer of the right quaternion or its negative to
            // QKEEPF for input into QMINI to ensure that QMINI does
            // interpolation in the "shortest arc" direction.
            //
            spicelib::VMINUG(QKEEPL.as_slice(), 4, QLNEG.as_slice_mut());

            DPOS = spicelib::VDISTG(QKEEPL.as_slice(), QKEEPF.as_slice(), 4);
            DNEG = spicelib::VDISTG(QLNEG.as_slice(), QKEEPF.as_slice(), 4);

            if (DNEG < DPOS) {
                spicelib::MOVED(QLNEG.as_slice(), 4, QLINPT.as_slice_mut());
            } else {
                spicelib::MOVED(QKEEPL.as_slice(), 4, QLINPT.as_slice_mut());
            }

            //
            // If the currently picked window ends are not 180 degrees
            // apart, check all records between them to see if
            // interpolated pointing is within tolerance of the actual
            // pointing. If the currently picked window ends are close to
            // 180 degrees apart, don't consider them as a possibility.
            //
            COSVAL =
                ((QKEEPF[0] * QLINPT[0]) + spicelib::VDOT(QKEEPF.subarray(1), QLINPT.subarray(1)));

            ANGLE = (f64::acos(spicelib::BRCKTD(COSVAL, -1.0, 1.0)) * 2.0);

            FITOK = (f64::abs((spicelib::PI(ctx) - ANGLE)) > ANGTOL);

            //
            // If KEEPF and KEEPL points are next to each other, we will
            // declare the fit to be OK even if it does not pass the ``too
            // close to 180 degrees apart'' test and set LEFT equal to
            // RIGHT to move KEEPF forward. If we don't do this, the
            // algorithm goes into indefinite loop.
            //
            if (((KEEPF + 1) == KEEPL) && !FITOK) {
                FITOK = true;
                LEFT = RIGHT;
            }

            J = (KEEPF + 1);

            while ((J <= (KEEPL - 1)) && FITOK) {
                //
                // Compute interpolation fraction for this pointing record.
                //
                if ((SCLKDP[KEEPL] - SCLKDP[KEEPF]) != 0.0) {
                    FRAC = ((SCLKDP[J] - SCLKDP[KEEPF]) / (SCLKDP[KEEPL] - SCLKDP[KEEPF]));
                } else {
                    spicelib::SIGERR(b"SPICE(CK3SDNBUG)", ctx)?;
                    spicelib::CHKOUT(b"CK3SDN", ctx)?;
                    return Ok(());
                }

                //
                // Call Nat's fast quaternion interpolation routine to
                // compute interpolated rotation for this point.
                //
                QMINI(
                    QKEEPF.as_slice(),
                    QLINPT.as_slice(),
                    FRAC,
                    QINTRP.as_slice_mut(),
                );

                //
                // Find the squared distance between the interpolated
                // and input quaternions.
                //
                DIST2A = (((((QUATS[[0, J]] - QINTRP[0]) * (QUATS[[0, J]] - QINTRP[0]))
                    + ((QUATS[[1, J]] - QINTRP[1]) * (QUATS[[1, J]] - QINTRP[1])))
                    + ((QUATS[[2, J]] - QINTRP[2]) * (QUATS[[2, J]] - QINTRP[2])))
                    + ((QUATS[[3, J]] - QINTRP[3]) * (QUATS[[3, J]] - QINTRP[3])));

                DIST2B = (((((QUATS[[0, J]] + QINTRP[0]) * (QUATS[[0, J]] + QINTRP[0]))
                    + ((QUATS[[1, J]] + QINTRP[1]) * (QUATS[[1, J]] + QINTRP[1])))
                    + ((QUATS[[2, J]] + QINTRP[2]) * (QUATS[[2, J]] + QINTRP[2])))
                    + ((QUATS[[3, J]] + QINTRP[3]) * (QUATS[[3, J]] + QINTRP[3])));

                DIST2 = intrinsics::DMIN1(&[DIST2A, DIST2B]);

                //
                // The rotation angle theta is related to the distance by
                // the formula
                //
                //    || Q1 - Q2 ||     =  2 * | sin(theta/4) |
                //
                ANGLE = (4.0 * f64::asin(intrinsics::DMIN1(&[(f64::sqrt(DIST2) / 2.0), 1.0])));

                //
                // Compare the angle with specified threshold.
                //
                FITOK = (FITOK && (f64::abs(ANGLE) <= SDNTOL));

                //
                // Increment index to move to the next record.
                //
                J = (J + 1);
            }

            //
            // Was the fit OK?
            //
            if FITOK {
                //
                // Fit was OK. Check if left and right are equal; if so we
                // found the point that were were looking for.
                //
                if (LEFT == RIGHT) {
                    //
                    // Mark all records between fist and last with DPMAX.
                    //
                    {
                        let m1__: i32 = (KEEPF + 1);
                        let m2__: i32 = (KEEPL - 1);
                        let m3__: i32 = 1;
                        J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            SCLKDP[J] = spicelib::DPMAX();
                            NDROPD = (NDROPD + 1);
                            J += m3__;
                        }
                    }

                    //
                    // Set first point for the next search to be equal to
                    // the to the found point.
                    //
                    KEEPF = KEEPL;

                    //
                    // Reset window boundaries for binary search.
                    //
                    LEFT = KEEPL;
                    RIGHT = INTCRL;
                } else {
                    //
                    // Left and right sides haven't converged yet; shift
                    // left side of the binary search window forward.
                    //
                    LEFT = (KEEPL + 1);
                }
            } else {
                //
                // No fit; shift right side of the binary search window
                // backwards.
                //
                RIGHT = (KEEPL - 1);

                //
                // If right side went "over" the left side, set left side
                // to be equal to the right side.
                //
                if (RIGHT < LEFT) {
                    LEFT = RIGHT;
                }
            }
        }
    }

    //
    // At this point all records that are to be removed, if any, have
    // been "tagged" with DPMAX in the times buffer. We need to re-sort
    // the buffers to push these records to the bottom and re-set the
    // number of records to indicate that only the top portion should be
    // used.
    //
    if (NDROPD != 0) {
        //
        // Since SCLKs were the ones "marked" by DPMAX, we will use them
        // to get the order vector.
        //
        spicelib::ORDERD(SCLKDP.as_slice(), *NREC, INTARR.as_slice_mut());

        //
        // Now, with the order vector in hand, sort the SCLKs ...
        //
        spicelib::REORDD(INTARR.as_slice_mut(), *NREC, SCLKDP.as_slice_mut());

        //
        // ... then sort quaternions (element by element) ...
        //
        for I in 0..=3 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = *NREC;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    DPARR[J] = QUATS[[I, J]];
                    J += m3__;
                }
            }
            spicelib::REORDD(INTARR.as_slice_mut(), *NREC, DPARR.as_slice_mut());
            {
                let m1__: i32 = 1;
                let m2__: i32 = *NREC;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    QUATS[[I, J]] = DPARR[J];
                    J += m3__;
                }
            }
        }

        //
        // ... and, finally, if requested, sort AVs (also element by
        // element) ...
        //
        if AVFLAG {
            for I in 1..=3 {
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = *NREC;
                    let m3__: i32 = 1;
                    J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        DPARR[J] = AVVS[[I, J]];
                        J += m3__;
                    }
                }
                spicelib::REORDD(INTARR.as_slice_mut(), *NREC, DPARR.as_slice_mut());
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = *NREC;
                    let m3__: i32 = 1;
                    J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        AVVS[[I, J]] = DPARR[J];
                        J += m3__;
                    }
                }
            }
        }

        //
        // Reset the number of points.
        //
        *NREC = (*NREC - NDROPD);
    }

    //
    // All done. Check out.
    //
    spicelib::CHKOUT(b"CK3SDN", ctx)?;

    Ok(())
}
