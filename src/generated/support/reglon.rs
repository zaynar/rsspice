//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);

//$Procedure REGLON ( Regularize longitude intervals )
pub fn REGLON(
    NIVALS: i32,
    BOUNDS: &[f64],
    MAXN: i32,
    NOUT: &mut i32,
    MINLON: &mut f64,
    MAXLON: &mut f64,
    OUTBDS: &mut [f64],
    SRCS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=NIVALS);
    let mut OUTBDS = DummyArrayMut2D::new(OUTBDS, 1..=2, 1..);
    let mut SRCS = DummyArrayMut::new(SRCS, 1..);
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut LB: f64 = 0.0;
    let mut LOCLB: f64 = 0.0;
    let mut LOCUB: f64 = 0.0;
    let mut UB: f64 = 0.0;
    let mut NREQ: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"REGLON", ctx)?;

    //
    // No output intervals have been found yet.
    //
    *NOUT = 0;

    if (NIVALS == 0) {
        spicelib::CHKOUT(b"REGLON", ctx)?;
        return Ok(());
    }

    //
    // Get lower and upper bounds of input values.
    //
    *MINLON = spicelib::DPMAX();
    *MAXLON = spicelib::DPMIN();

    for I in 1..=NIVALS {
        LB = BOUNDS[[1, I]];
        UB = BOUNDS[[2, I]];
        //
        // Rectangles of zero longitude extent not allowed.
        //
        if (LB == UB) {
            spicelib::SETMSG(
                b"Longitude lower bound # (# degrees) equals upper bound.",
                ctx,
            );
            spicelib::ERRDP(b"#", LB, ctx);
            spicelib::ERRDP(b"#", (LB * spicelib::DPR(ctx)), ctx);
            spicelib::SIGERR(b"SPICE(ZEROBOUNDSEXTENT)", ctx)?;
            spicelib::CHKOUT(b"REGLON", ctx)?;
            return Ok(());
        }

        //
        // Adjust UB if necessary before deciding on the output
        // range.
        //
        if (UB < LB) {
            UB = (UB + spicelib::TWOPI(ctx));
        }

        *MINLON = intrinsics::DMIN1(&[LB, UB, *MINLON]);
        *MAXLON = intrinsics::DMAX1(&[LB, UB, *MAXLON]);
    }

    //
    // If MAXLON and MINLON lie within the range
    //
    //    0 - ANGMRG : 2*pi + ANGMRG
    //
    // we'll set the output longitudes to lie in the range
    //
    //    0 : 2*pi
    //
    //
    // If MAXLON and MINLON lie within the range
    //
    //    -pi - ANGMRG : pi + ANGMRG
    //
    // we'll set the output longitudes to lie in the range
    //
    //    -pi : pi
    //
    //
    // We use the latter range if neither of the first two
    // conditions are met.
    //
    //
    if ((*MINLON > -ANGMRG) && (*MAXLON < (spicelib::TWOPI(ctx) + ANGMRG))) {
        A = 0.0;
        B = spicelib::TWOPI(ctx);
    } else {
        //
        // We arbitrarily pick the output longitude range
        //
        //    -pi : pi
        //
        A = -spicelib::PI(ctx);
        B = spicelib::PI(ctx);
    }

    //
    // Set the output values of MINLON and MAXLON.
    //
    *MINLON = A;
    *MAXLON = B;

    //
    // Process each input interval.
    //
    for I in 1..=NIVALS {
        LB = BOUNDS[[1, I]];
        UB = BOUNDS[[2, I]];
        //
        // We'll adjust the inputs to ensure they're in range.
        //
        // First, make sure we're starting with values in
        // the range [-2*pi, 2*pi].
        //
        spicelib::ZZNRMLON(LB, UB, ANGMRG, &mut LOCLB, &mut LOCUB, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"REGLON", ctx)?;
            return Ok(());
        }

        //
        // Move each output into the range [A, B].
        //
        if (LOCLB < A) {
            LOCLB = (LOCLB + spicelib::TWOPI(ctx));
        } else if (LOCLB > B) {
            LOCLB = (LOCLB - spicelib::TWOPI(ctx));
        }

        if (LOCUB < A) {
            LOCUB = (LOCUB + spicelib::TWOPI(ctx));
        } else if (LOCUB > B) {
            LOCUB = (LOCUB - spicelib::TWOPI(ctx));
        }

        //
        // Now the bounds are in range, but they may be
        // out of order.
        //
        if (LOCLB < LOCUB) {
            //
            // The bounds are in order. Add the interval to
            // the list of output intervals.
            //
            *NOUT = (*NOUT + 1);

            if (*NOUT > MAXN) {
                //
                // We're out of room.
                //
                spicelib::SETMSG(b"Output arrays have room for # intervals we have found # output intervals so far.", ctx);
                spicelib::ERRINT(b"#", MAXN, ctx);
                spicelib::ERRINT(b"#", *NOUT, ctx);
                spicelib::SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
                spicelib::CHKOUT(b"REGLON", ctx)?;
                return Ok(());
            }

            OUTBDS[[1, *NOUT]] = LOCLB;
            OUTBDS[[2, *NOUT]] = LOCUB;
            SRCS[*NOUT] = I;
        } else {
            //
            // The bounds are in range but out of order.
            // We'll split the input interval into two
            // output intervals.
            //
            NREQ = 0;

            if (A < LOCUB) {
                NREQ = (NREQ + 1);
            }

            if (LOCLB < B) {
                NREQ = (NREQ + 1);
            }

            if ((*NOUT + NREQ) > MAXN) {
                //
                // We're out of room.
                //
                spicelib::SETMSG(b"Output arrays have room for # intervals we have found # output intervals so far.", ctx);
                spicelib::ERRINT(b"#", MAXN, ctx);
                spicelib::ERRINT(b"#", (*NOUT + NREQ), ctx);
                spicelib::SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
                spicelib::CHKOUT(b"REGLON", ctx)?;
                return Ok(());
            }
            //
            // The input interval "wraps around" the output boundaries.
            //
            // The output intervals extend from A to the upper bound and
            // from the lower bound to B.
            //
            if (A < LOCUB) {
                *NOUT = (*NOUT + 1);

                OUTBDS[[1, *NOUT]] = A;
                OUTBDS[[2, *NOUT]] = LOCUB;
                SRCS[*NOUT] = I;
            }

            if (LOCLB < B) {
                *NOUT = (*NOUT + 1);

                OUTBDS[[1, *NOUT]] = LOCLB;
                OUTBDS[[2, *NOUT]] = B;
                SRCS[*NOUT] = I;
            }
        }
    }

    spicelib::CHKOUT(b"REGLON", ctx)?;
    Ok(())
}
