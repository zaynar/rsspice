//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODIED ( Pod, insert elements, double precision )
pub fn PODIED(
    ELEMS: &[f64],
    N: i32,
    LOC: i32,
    POD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ELEMS = DummyArray::new(ELEMS, 1..);
    let mut POD = DummyArrayMut::new(POD, LBCELL..);
    let mut NUMBER: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut END: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"PODIED", ctx)?;
    }

    //
    // Three things can go `wrong':
    //
    //    1) No items to insert.
    //
    //    2) Too many items to insert.
    //
    //    3) No place to insert them.
    //
    PODOND(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    if (N < 1) {
        spicelib::CHKOUT(b"PODIED", ctx)?;
        return Ok(());
    } else if ((spicelib::CARDD(POD.as_slice(), ctx)? + N) > spicelib::SIZED(POD.as_slice(), ctx)?)
    {
        spicelib::SETMSG(b"Cannot fit # elements into # spaces.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(
            b"#",
            (spicelib::SIZED(POD.as_slice(), ctx)? - spicelib::CARDD(POD.as_slice(), ctx)?),
            ctx,
        );
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODIED", ctx)?;
        return Ok(());
    } else if ((LOC < 1) || (LOC > (NUMBER + 1))) {
        spicelib::SETMSG(b"Location (#) must be in the range [1,#].", ctx);
        spicelib::ERRINT(b"#", LOC, ctx);
        spicelib::ERRINT(b"#", (NUMBER + 1), ctx);
        spicelib::SIGERR(b"SPICE(BADPODLOCATION)", ctx)?;
        spicelib::CHKOUT(b"PODIED", ctx)?;
        return Ok(());
    }

    //
    // In theory, we are home free. The rest looks just like the
    // code in $Examples, above.
    //
    END = (OFFSET + NUMBER);

    spicelib::INSLAD(
        ELEMS.as_slice(),
        N,
        (OFFSET + LOC),
        POD.subarray_mut(1),
        &mut END,
        ctx,
    )?;
    spicelib::SCARDD(END, POD.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"PODIED", ctx)?;
    Ok(())
}
