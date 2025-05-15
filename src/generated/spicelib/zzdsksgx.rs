//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZDSKSGX ( DSK, ray-segment intercept )
pub fn ZZDSKSGX(
    HANDLE: i32,
    DLADSC: &[i32],
    DTYPE: i32,
    ET: f64,
    VERTEX: &[f64],
    RAYDIR: &[f64],
    XPT: &mut [f64],
    DC: &mut [f64],
    IC: &mut [i32],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DLADSC = DummyArray::new(DLADSC, 1..);
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut DC = DummyArrayMut::new(DC, 1..);
    let mut IC = DummyArrayMut::new(IC, 1..);
    let mut RETVAL: f64 = 0.0;
    let mut PLID: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKSGX", ctx)?;

    //
    // Note: input argument ET is provided to support time-dependent
    // data types.
    //
    RETVAL = TOUCHD(ET);
    DC[1] = TOUCHD(*DC.first());

    if (DTYPE == 2) {
        //
        // The intercept plate ID is returned in element 1 of
        // IC, if an intercept is found.
        //
        DSKX02(
            HANDLE,
            DLADSC.as_slice(),
            VERTEX.as_slice(),
            RAYDIR.as_slice(),
            &mut PLID,
            XPT.as_slice_mut(),
            FOUND,
            ctx,
        )?;

        if *FOUND {
            IC[1] = PLID;
        }
    } else {
        SETMSG(
            b"DSK ray-surface intercepts are not supported for DSK data type #.",
            ctx,
        );
        ERRINT(b"#", DTYPE, ctx);
        SIGERR(b"SPICE(TYPENOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZDSKSGX", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZDSKSGX", ctx)?;
    Ok(())
}
