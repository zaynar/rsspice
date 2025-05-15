//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODAEC ( Pod, append elements, character )
pub fn PODAEC(
    ELEMS: CharArray,
    N: i32,
    POD: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ELEMS = DummyCharArray::new(ELEMS, None, 1..);
    let mut POD = DummyCharArrayMut::new(POD, None, LBCELL..);
    let mut END: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"PODAEC", ctx)?;
    }

    //
    // We can't append a non-positive number of items.
    //
    if (N < 1) {
        spicelib::CHKOUT(b"PODAEC", ctx)?;
        return Ok(());
    }

    //
    // First see if there is room in the pod to append N elements.
    // If not, bail out.
    //
    if (spicelib::SIZEC(POD.as_arg(), ctx)? < (spicelib::CARDC(POD.as_arg(), ctx)? + N)) {
        spicelib::SETMSG(b"Cannot fit # elements into # spaces.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(
            b"#",
            (spicelib::SIZEC(POD.as_arg(), ctx)? - spicelib::CARDC(POD.as_arg(), ctx)?),
            ctx,
        );
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;

    //
    // There is ample room, so we find out where the end of the
    // active group is and simply loop through the individual
    // copies of ELEMS, adjusting the cardinality afterwards.
    // (Just like in $Examples, above.)
    //
    } else {
        END = spicelib::CARDC(POD.as_arg(), ctx)?;

        for I in 1..=N {
            fstr::assign(POD.get_mut((END + I)), ELEMS.get(I));
        }

        spicelib::SCARDC((END + N), POD.as_arg_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"PODAEC", ctx)?;
    Ok(())
}
