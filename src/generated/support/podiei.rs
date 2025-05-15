//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODIEI ( Pod, insert elements, integer )
pub fn PODIEI(
    ELEMS: &[i32],
    N: i32,
    LOC: i32,
    POD: &mut [i32],
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
        spicelib::CHKIN(b"PODIEI", ctx)?;
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
    PODONI(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    if (N < 1) {
        spicelib::CHKOUT(b"PODIEI", ctx)?;
        return Ok(());
    } else if ((spicelib::CARDI(POD.as_slice(), ctx)? + N) > spicelib::SIZEI(POD.as_slice(), ctx)?)
    {
        spicelib::SETMSG(b"Cannot fit # elements into # spaces.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(
            b"#",
            (spicelib::SIZEI(POD.as_slice(), ctx)? - spicelib::CARDI(POD.as_slice(), ctx)?),
            ctx,
        );
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODIEI", ctx)?;
        return Ok(());
    } else if ((LOC < 1) || (LOC > (NUMBER + 1))) {
        spicelib::SETMSG(b"Location (#) must be in the range [1,#].", ctx);
        spicelib::ERRINT(b"#", LOC, ctx);
        spicelib::ERRINT(b"#", (NUMBER + 1), ctx);
        spicelib::SIGERR(b"SPICE(BADPODLOCATION)", ctx)?;
        spicelib::CHKOUT(b"PODIEI", ctx)?;
        return Ok(());
    }

    //
    // In theory, we are home free. The rest looks just like the
    // code in $Examples, above.
    //
    END = (OFFSET + NUMBER);

    spicelib::INSLAI(
        ELEMS.as_slice(),
        N,
        (OFFSET + LOC),
        POD.subarray_mut(1),
        &mut END,
        ctx,
    )?;
    spicelib::SCARDI(END, POD.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"PODIEI", ctx)?;
    Ok(())
}
