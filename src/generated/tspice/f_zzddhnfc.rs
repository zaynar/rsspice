//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

struct SaveVars {
    FMTSTR: Vec<u8>,
    BFFSTR: Vec<u8>,
    NATBFF: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FMTSTR = vec![b' '; LNSIZE as usize];
        let mut BFFSTR = vec![b' '; LNSIZE as usize];
        let mut NATBFF: i32 = 0;

        Self {
            FMTSTR,
            BFFSTR,
            NATBFF,
        }
    }
}

//$Procedure F_ZZDDHNFC ( ZZDDHNFC tests )
pub fn F_ZZDDHNFC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZDDHNFC", ctx)?;

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Fetch the integer BFF code for the host system.", ctx)?;

    //
    // Look up the BFF integer code.
    //
    spicelib::ZZDDHNFC(&mut save.NATBFF, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the equivalent string.
    //
    spicelib::ZZDDHGSD(b"BFF", save.NATBFF, &mut save.BFFSTR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZPLATFM(b"FILE_FORMAT", &mut save.FMTSTR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"BFFSTR", &save.BFFSTR, b"=", &save.FMTSTR, OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // None.
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
