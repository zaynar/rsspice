//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXSIZ: i32 = 25;

//$Procedure T_SPKCHB ( Test utility, fit SPK data with Chebys )
pub fn T_SPKCHB(
    TARGET: &[u8],
    CENTER: &[u8],
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    NPOS: i32,
    NVEL: i32,
    WORK: &mut [f64],
    POSCOF: &mut [f64],
    VELCOF: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut WORK = DummyArrayMut::new(WORK, 1..);
    let mut POSCOF = DummyArrayMut2D::new(POSCOF, 1..=NPOS, 1..=3);
    let mut VELCOF = DummyArrayMut2D::new(VELCOF, 1..=NVEL, 1..=3);
    let mut RETVAL: f64 = 0.0;
    let mut CENTID: i32 = 0;
    let mut TARGID: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
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

    spicelib::CHKIN(b"T_SPKCHB", ctx)?;

    spicelib::BODS2C(TARGET, &mut TARGID, &mut FOUND, ctx)?;

    if !FOUND {
        spicelib::SETMSG(b"Target name # could not be translated to an ID code.", ctx);
        spicelib::ERRCH(b"#", TARGET, ctx);
        spicelib::SIGERR(b"SPICE(NOTRANSLATION)", ctx)?;
        spicelib::CHKOUT(b"T_SPKCHB", ctx)?;
        return Ok(());
    }

    spicelib::BODS2C(CENTER, &mut CENTID, &mut FOUND, ctx)?;

    if !FOUND {
        spicelib::SETMSG(b"Center name # could not be translated to an ID code.", ctx);
        spicelib::ERRCH(b"#", CENTER, ctx);
        spicelib::SIGERR(b"SPICE(NOTRANSLATION)", ctx)?;
        spicelib::CHKOUT(b"T_SPKCHB", ctx)?;
        return Ok(());
    }

    if ((NPOS < 1) || (NPOS > MAXSIZ)) {
        spicelib::SETMSG(
            b"Position coefficient count # must be in the range 1:# but was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", MAXSIZ, ctx);
        spicelib::ERRINT(b"#", NPOS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_SPKCHB", ctx)?;
        return Ok(());
    }

    if ((NVEL < 1) || (NVEL > MAXSIZ)) {
        spicelib::SETMSG(
            b"Velocity coefficient count # must be in the range 1:# but was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", MAXSIZ, ctx);
        spicelib::ERRINT(b"#", NVEL, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_SPKCHB", ctx)?;
        return Ok(());
    }
    //
    // Initialize the state component functions. The returned
    // value is not used; the called function operates by
    // side effects.
    //
    RETVAL = T_STCINI(TARGID, FRAME, b"NONE", CENTID, ctx);
    //
    // Fit the state vector components.
    //
    support::CHBFIT(
        T_GETX,
        FIRST,
        LAST,
        NPOS,
        WORK.as_slice_mut(),
        POSCOF.subarray_mut([1, 1]),
        ctx,
    )?;
    support::CHBFIT(
        T_GETY,
        FIRST,
        LAST,
        NPOS,
        WORK.as_slice_mut(),
        POSCOF.subarray_mut([1, 2]),
        ctx,
    )?;
    support::CHBFIT(
        T_GETZ,
        FIRST,
        LAST,
        NPOS,
        WORK.as_slice_mut(),
        POSCOF.subarray_mut([1, 3]),
        ctx,
    )?;
    support::CHBFIT(
        T_GETDX,
        FIRST,
        LAST,
        NVEL,
        WORK.as_slice_mut(),
        VELCOF.subarray_mut([1, 1]),
        ctx,
    )?;
    support::CHBFIT(
        T_GETDY,
        FIRST,
        LAST,
        NVEL,
        WORK.as_slice_mut(),
        VELCOF.subarray_mut([1, 2]),
        ctx,
    )?;
    support::CHBFIT(
        T_GETDZ,
        FIRST,
        LAST,
        NVEL,
        WORK.as_slice_mut(),
        VELCOF.subarray_mut([1, 3]),
        ctx,
    )?;

    spicelib::CHKOUT(b"T_SPKCHB", ctx)?;
    Ok(())
}
