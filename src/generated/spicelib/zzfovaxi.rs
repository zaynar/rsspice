//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MARGIN: f64 = 0.000000000001;

//$Procedure   ZZFOVAXI ( Generate an axis vector for polygonal FOV )
pub fn ZZFOVAXI(
    INST: &[u8],
    N: i32,
    BOUNDS: &[f64],
    AXIS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=3, 1..=N);
    let mut AXIS = DummyArrayMut::new(AXIS, 1..=3);
    let mut CP = StackArray::<f64, 3>::new(1..=3);
    let mut LIMIT: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut UVEC = StackArray::<f64, 3>::new(1..=3);
    let mut I: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut OK: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZFOVAXI", ctx)?;

    //
    // We must have at least 3 boundary vectors.
    //
    if (N < 3) {
        SETMSG(
            b"Polygonal FOV requires at least 3 boundary vectors but number supplied for # was #.",
            ctx,
        );
        ERRCH(b"#", INST, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZFOVAXI", ctx)?;
        return Ok(());
    }

    //
    // Check for linearly dependent consecutive boundary vectors.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Set the index of the next ray. When we get to the
            // last boundary vector, the next ray is the first.
            //
            if (I == N) {
                NEXT = 1;
            } else {
                NEXT = (I + 1);
            }

            //
            // Find the cross product of the first ray with the
            // second. Depending on the ordering of the boundary
            // vectors, this could be an inward or outward normal,
            // in the case the current face is is exterior.
            //
            VCRSS(
                BOUNDS.subarray([1, I]),
                BOUNDS.subarray([1, NEXT]),
                CP.as_slice_mut(),
            );

            //
            // We insist on consecutive boundary vectors being
            // linearly independent.
            //
            if VZERO(CP.as_slice()) {
                SETMSG(b"Polygonal FOV must have linearly independent consecutive boundary but vectors at indices # and # have cross product equal to the zero vector. Instrument is #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", NEXT, ctx);
                ERRCH(b"#", INST, ctx);
                SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                CHKOUT(b"ZZFOVAXI", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // First try the average of the FOV unit boundary vectors as
    // a candidate axis. In many cases, this simple approach
    // does the trick.
    //
    CLEARD(3, AXIS.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            VHAT(BOUNDS.subarray([1, I]), UVEC.as_slice_mut());
            VADD(UVEC.as_slice(), AXIS.as_slice(), V.as_slice_mut());
            VEQU(V.as_slice(), AXIS.as_slice_mut());

            I += m3__;
        }
    }

    VSCLIP((1.0 / N as f64), AXIS.as_slice_mut());

    //
    // If each boundary vector has sufficiently small
    // angular separation from AXIS, we're done.
    //
    LIMIT = (HALFPI(ctx) - MARGIN);

    OK = true;
    I = 1;

    while ((I <= N) && OK) {
        SEP = VSEP(BOUNDS.subarray([1, I]), AXIS.as_slice(), ctx);

        if (SEP > LIMIT) {
            OK = false;
        } else {
            I = (I + 1);
        }
    }

    if !OK {
        //
        // See whether we can find an axis using a
        // method based on finding a face of the convex
        // hull of the FOV. ZZHULLAX signals an error
        // if it doesn't succeed.
        //
        ZZHULLAX(INST, N, BOUNDS.as_slice(), AXIS.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZFOVAXI", ctx)?;
            return Ok(());
        }
    }

    //
    // At this point AXIS is valid. Make the axis vector unit length.
    //
    VHATIP(AXIS.as_slice_mut());

    CHKOUT(b"ZZFOVAXI", ctx)?;
    Ok(())
}
