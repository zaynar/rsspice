//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SPK0: &[u8] = b"test.bsp";
const FILSIZ: i32 = 255;
const TYPSIZ: i32 = 4;

struct SaveVars {
    KTYPE: Vec<u8>,
    KSRC: Vec<u8>,
    HAN0: i32,
    HAN1: i32,
    HAN2: i32,
    HAN3: i32,
    FOUND0: bool,
    FOUND1: bool,
    FOUND2: bool,
    FOUND3: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut KTYPE = vec![b' '; FILSIZ as usize];
        let mut KSRC = vec![b' '; TYPSIZ as usize];
        let mut HAN0: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut HAN3: i32 = 0;
        let mut FOUND0: bool = false;
        let mut FOUND1: bool = false;
        let mut FOUND2: bool = false;
        let mut FOUND3: bool = false;

        Self {
            KTYPE,
            KSRC,
            HAN0,
            HAN1,
            HAN2,
            HAN3,
            FOUND0,
            FOUND1,
            FOUND2,
            FOUND3,
        }
    }
}

//$Procedure F_KPBUG ( UNLOAD bug demonstration )
pub fn F_KPBUG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //
    //
    // Save all local variables to avoid stack problems on some
    // platforms.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_KPBUG", ctx)?;

    //
    //     Open a new SPK file for writing.
    //
    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Setup: create kernels.", ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create SPK file.", ctx)?;
    //
    // Create a small SPK file containing 1 record. We'll
    // use the moon as the target and the earth-moon barycenter
    // as the center.
    //

    //
    // Load test SPK to provide data.
    //
    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTSPK(SPK0, true, &mut save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN0, ctx)?;
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Demonstrate multiple load bug.", ctx)?;

    spicelib::FURNSH(SPK0, ctx)?;
    spicelib::KINFO(
        SPK0,
        &mut save.KTYPE,
        &mut save.KSRC,
        &mut save.HAN0,
        &mut save.FOUND0,
        ctx,
    );
    testutil::CHCKSL(b"KEEPBUG 0", save.FOUND0, true, OK, ctx)?;

    spicelib::FURNSH(SPK0, ctx)?;
    spicelib::KINFO(
        SPK0,
        &mut save.KTYPE,
        &mut save.KSRC,
        &mut save.HAN1,
        &mut save.FOUND1,
        ctx,
    );
    testutil::CHCKSL(b"KEEPBUG 1", save.FOUND1, true, OK, ctx)?;
    testutil::CHCKSI(b"KEEPBUG 2", save.HAN0, b"=", save.HAN1, 0, OK, ctx)?;

    spicelib::UNLOAD(SPK0, ctx)?;
    spicelib::KINFO(
        SPK0,
        &mut save.KTYPE,
        &mut save.KSRC,
        &mut save.HAN2,
        &mut save.FOUND2,
        ctx,
    );
    testutil::CHCKSL(b"KEEPBUG 3", save.FOUND2, true, OK, ctx)?;
    testutil::CHCKSI(b"KEEPBUG 4", save.HAN0, b"=", save.HAN2, 0, OK, ctx)?;

    spicelib::UNLOAD(SPK0, ctx)?;
    spicelib::KINFO(
        SPK0,
        &mut save.KTYPE,
        &mut save.KSRC,
        &mut save.HAN3,
        &mut save.FOUND3,
        ctx,
    );
    testutil::CHCKSL(b"KEEPBUG 5", save.FOUND3, false, OK, ctx)?;

    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
