//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZCPUTIM ( CPU Time )
pub fn ZZCPUTIM(TVEC: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TVEC = DummyArrayMut::new(TVEC, 1..=6);
    let mut DTIME = StackArray::<i32, 8>::new(1..=8);
    let mut RCLOCK = ActualCharArray::new(12, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZCPUTIM", ctx)?;
    }

    let [arg0, arg1, arg2] = RCLOCK
        .get_disjoint_mut([1, 2, 3])
        .expect("mutable array elements passed to function must have disjoint indexes");
    ctx.date_and_time(arg0, arg1, arg2, DTIME.as_slice_mut());
    //
    // Let's pack all this information into our double precision TVEC
    // array.
    //
    TVEC[1] = (DTIME[1] as f64);
    TVEC[2] = (DTIME[2] as f64);
    TVEC[3] = (DTIME[3] as f64);
    TVEC[4] = (DTIME[5] as f64);
    TVEC[5] = (DTIME[6] as f64);
    TVEC[6] = ((DTIME[7] as f64) + ((DTIME[8] as f64) / 1000.0));

    //
    // That's it.
    //
    CHKOUT(b"ZZCPUTIM", ctx)?;
    Ok(())
}
