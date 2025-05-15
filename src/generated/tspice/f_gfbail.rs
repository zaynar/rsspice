//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    STATUS: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STATUS: bool = false;

        Self { STATUS }
    }
}

//$Procedure      F_GFBAIL ( Test GFBAIL )
pub fn F_GFBAIL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFBAIL", ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Make sure function returns FALSE on initial call.", ctx)?;

    save.STATUS = true;
    save.STATUS = spicelib::GFBAIL();

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"STATUS", save.STATUS, false, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Make sure function returns FALSE on 2nd call.", ctx)?;

    save.STATUS = true;
    save.STATUS = spicelib::GFBAIL();

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"STATUS", save.STATUS, false, OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Test routine\'s action when a SPICE error condition exists.",
        ctx,
    )?;

    spicelib::SIGERR(b"SPICE(TESTERROR)", ctx)?;

    save.STATUS = true;
    save.STATUS = spicelib::GFBAIL();

    testutil::CHCKXC(true, b"SPICE(TESTERROR)", OK, ctx)?;

    testutil::CHCKSL(b"STATUS", save.STATUS, false, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
