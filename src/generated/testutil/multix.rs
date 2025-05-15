//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure MULTIX ( Multiple dimensional index )
pub fn MULTIX(
    BASIDX: i32,
    N: i32,
    DIMS: &[i32],
    OFFSET: i32,
    COORDS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DIMS = DummyArray::new(DIMS, 1..=N);
    let mut COORDS = DummyArrayMut::new(COORDS, 1..=N);
    let mut M: i32 = 0;
    let mut MAXIDX: i32 = 0;
    let mut Q: i32 = 0;
    let mut REM: i32 = 0;

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if (N < 1) {
        //
        // If N is less than 1, this routine may crash before
        // reaching these lines.  In case it doesn't....
        //
        spicelib::CHKIN(b"MULTIX", ctx)?;
        spicelib::SETMSG(b"N must be at least 1;  N = #", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"MULTIX", ctx)?;
        return Ok(());
    }

    if (OFFSET < BASIDX) {
        spicelib::CHKIN(b"MULTIX", ctx)?;
        spicelib::SETMSG(
            b"OFFSET must not be less than BASIDX, which is #;  OFFSET = #",
            ctx,
        );
        spicelib::ERRINT(b"#", BASIDX, ctx);
        spicelib::ERRINT(b"#", OFFSET, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"MULTIX", ctx)?;
        return Ok(());
    }

    //
    // We're going to use the output array COORDS as work space.
    // Later, we'll fill in the output values.
    //
    // Compute the products of the first I dimensions, I = 1 to N-1.
    // The Ith product will go into element I+1 of COORDS.
    //
    COORDS[1] = 1;

    for I in 2..=N {
        COORDS[I] = (DIMS[(I - 1)] * COORDS[(I - 1)]);
    }

    //
    // Convert to 0-relative indexing.  We assume OFFSET ranges from
    // BASIDX to MAXIDX, where MAXIDX is
    //
    //    <the array size>   -  1  +  BASIDX.
    //
    M = (OFFSET - BASIDX);

    //
    // At this point we can check to make sure OFFSET is not too large.
    //
    MAXIDX = (((COORDS[N] * DIMS[N]) - 1) + BASIDX);

    if (OFFSET > MAXIDX) {
        spicelib::CHKIN(b"MULTIX", ctx)?;
        spicelib::SETMSG(b"OFFSET must not exceed #;  OFFSET = #", ctx);
        spicelib::ERRINT(b"#", MAXIDX, ctx);
        spicelib::ERRINT(b"#", OFFSET, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"MULTIX", ctx)?;
        return Ok(());
    }

    //
    // Pick off the coordinates, starting with the highest-indexed.
    //
    for I in intrinsics::range(N, 2, -1) {
        //
        // After the RMAINI call, we no longer need the work space
        // value of COORDS(I).  At this point, we overwrite that
        // value with the output value of COORDS(I).
        //
        spicelib::RMAINI(M, COORDS[I], &mut Q, &mut REM, ctx)?;

        M = REM;
        COORDS[I] = Q;
    }

    COORDS[1] = REM;

    //
    // If the index base is non-zero, map each coordinate back to
    // the original representation.
    //
    if (BASIDX != 0) {
        for I in 1..=N {
            COORDS[I] = (COORDS[I] + BASIDX);
        }
    }

    Ok(())
}
