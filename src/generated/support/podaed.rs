//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODAED ( Pod, append elements, double precision )
pub fn PODAED(ELEMS: &[f64], N: i32, POD: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let ELEMS = DummyArray::new(ELEMS, 1..);
    let mut POD = DummyArrayMut::new(POD, LBCELL..);
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
        spicelib::CHKIN(b"PODAED", ctx)?;
    }

    //
    // We can't append a non-positive number of items.
    //
    if (N < 1) {
        spicelib::CHKOUT(b"PODAED", ctx)?;
        return Ok(());
    }

    //
    // First see if there is room in the pod to append N elements.
    // If not, bail out.
    //
    END = spicelib::CARDD(POD.as_slice(), ctx)?;

    if (spicelib::SIZED(POD.as_slice(), ctx)? < (END + N)) {
        spicelib::SETMSG(b"Cannot fit # elements into # spaces.", ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(b"#", (spicelib::SIZED(POD.as_slice(), ctx)? - END), ctx);
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;

    //
    // There is ample room, so we find out where the end of the
    // active group is and simply loop through the individual
    // copies of ELEMS, adjusting the cardinality afterwards.
    // (Just like in $Examples, above.)
    //
    } else {
        for I in 1..=N {
            POD[(END + I)] = ELEMS[I];
        }

        spicelib::SCARDD((END + N), POD.as_slice_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"PODAED", ctx)?;
    Ok(())
}
