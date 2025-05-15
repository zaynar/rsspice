//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODIEC ( Pod, insert elements, character )
pub fn PODIEC(
    ELEMS: CharArray,
    N: i32,
    LOC: i32,
    POD: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ELEMS = DummyCharArray::new(ELEMS, None, 1..);
    let mut POD = DummyCharArrayMut::new(POD, None, LBCELL..);
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
        spicelib::CHKIN(b"PODIEC", ctx)?;
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
    PODONC(POD.as_arg(), &mut OFFSET, &mut NUMBER, ctx)?;

    if (N < 1) {
        spicelib::CHKOUT(b"PODIEC", ctx)?;
        return Ok(());
    } else if ((spicelib::CARDC(POD.as_arg(), ctx)? + N) > spicelib::SIZEC(POD.as_arg(), ctx)?) {
        spicelib::SETMSG(b"Cannot fit # elements into # spaces.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(
            b"#",
            (spicelib::SIZEC(POD.as_arg(), ctx)? - spicelib::CARDC(POD.as_arg(), ctx)?),
            ctx,
        );
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODIEC", ctx)?;
        return Ok(());
    } else if ((LOC < 1) || (LOC > (NUMBER + 1))) {
        spicelib::SETMSG(b"Location (#) must be in the range [1,#].", ctx);
        spicelib::ERRINT(b"#", LOC, ctx);
        spicelib::ERRINT(b"#", (NUMBER + 1), ctx);
        spicelib::SIGERR(b"SPICE(BADPODLOCATION)", ctx)?;
        spicelib::CHKOUT(b"PODIEC", ctx)?;
        return Ok(());
    }

    //
    // In theory, we are home free. The rest looks just like the
    // code in $Examples, above.
    //
    END = (OFFSET + NUMBER);

    spicelib::INSLAC(
        ELEMS.as_arg(),
        N,
        (OFFSET + LOC),
        POD.subarray_mut(1),
        &mut END,
        ctx,
    )?;
    spicelib::SCARDC(END, POD.as_arg_mut(), ctx)?;

    spicelib::CHKOUT(b"PODIEC", ctx)?;
    Ok(())
}
