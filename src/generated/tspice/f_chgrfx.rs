//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      F_CHGRFX ( Test the Change IRF exceptions )
pub fn F_CHGRFX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CHGRFX", ctx)?;

    testutil::TCASE(
        b"Make sure an unrecognized FROM frame is properly diagnosed. ",
        ctx,
    )?;

    spicelib::IRFROT(1, 10013, ROT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(IRFNOTREC)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure an unrecognized TO frame is properly diagnosed. ",
        ctx,
    )?;

    spicelib::IRFROT(10013, 1, ROT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(IRFNOTREC)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure an unrecognized DEFAULT frame is properly diagnosed. ",
        ctx,
    )?;

    spicelib::IRFDEF(10013, ctx)?;
    testutil::CHCKXC(true, b"SPICE(IRFNOTREC)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
