//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;

//$Procedure F_ZZCTR ( Family of tests for ZZCTR )
pub fn F_ZZCTR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NEWCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut OLDCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut SAVCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
    let mut UPDATE: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZCTR", ctx)?;

    //
    // Initialize counters to high value.
    //
    testutil::TCASE(b"Initialize counter to user values.", ctx)?;

    spicelib::ZZCTRUIN(OLDCTR.as_slice_mut(), ctx);

    testutil::CHCKSI(
        b"OLDCTR(1)",
        OLDCTR[1],
        b"=",
        spicelib::INTMAX(),
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"OLDCTR(2)",
        OLDCTR[2],
        b"=",
        spicelib::INTMAX(),
        0,
        OK,
        ctx,
    )?;

    //
    // Initialize counters to low value.
    //
    testutil::TCASE(b"Initialize counter to subsystem values.", ctx)?;

    spicelib::ZZCTRSIN(OLDCTR.as_slice_mut(), ctx);

    testutil::CHCKSI(
        b"OLDCTR(1)",
        OLDCTR[1],
        b"=",
        spicelib::INTMIN(),
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"OLDCTR(2)",
        OLDCTR[2],
        b"=",
        spicelib::INTMIN(),
        0,
        OK,
        ctx,
    )?;

    //
    // Increment counters.
    //
    testutil::TCASE(b"Increment counter.", ctx)?;

    spicelib::ZZCTRINC(OLDCTR.as_slice_mut(), ctx)?;

    testutil::CHCKSI(
        b"OLDCTR(1)",
        OLDCTR[1],
        b"=",
        (spicelib::INTMIN() + 1),
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"OLDCTR(2)",
        OLDCTR[2],
        b"=",
        spicelib::INTMIN(),
        0,
        OK,
        ctx,
    )?;

    //
    // Increment counters with roll over.
    //
    testutil::TCASE(b"Increment counter with roll over.", ctx)?;

    OLDCTR[1] = spicelib::INTMAX();

    spicelib::ZZCTRINC(OLDCTR.as_slice_mut(), ctx)?;

    testutil::CHCKSI(
        b"OLDCTR(1)",
        OLDCTR[1],
        b"=",
        spicelib::INTMIN(),
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"OLDCTR(2)",
        OLDCTR[2],
        b"=",
        (spicelib::INTMIN() + 1),
        0,
        OK,
        ctx,
    )?;

    //
    // Check counters without update.
    //
    testutil::TCASE(b"Check counter with no update.", ctx)?;

    spicelib::MOVEI(OLDCTR.as_slice(), CTRSIZ, NEWCTR.as_slice_mut());
    spicelib::MOVEI(OLDCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZCTRCHK(NEWCTR.as_slice(), OLDCTR.as_slice_mut(), &mut UPDATE, ctx);

    testutil::CHCKAI(
        b"OLDCTR",
        OLDCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"NEWCTR",
        NEWCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"UPDATE", UPDATE, false, OK, ctx)?;

    //
    // Check with first element different and update.
    //
    testutil::TCASE(b"Check counter with update, first element.", ctx)?;

    spicelib::MOVEI(OLDCTR.as_slice(), CTRSIZ, NEWCTR.as_slice_mut());
    NEWCTR[1] = (NEWCTR[1] + 1);
    spicelib::MOVEI(NEWCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZCTRCHK(NEWCTR.as_slice(), OLDCTR.as_slice_mut(), &mut UPDATE, ctx);

    testutil::CHCKAI(
        b"OLDCTR",
        OLDCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"NEWCTR",
        NEWCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Check with second element different and update.
    //
    testutil::TCASE(b"Check counter with update, second element.", ctx)?;

    spicelib::MOVEI(OLDCTR.as_slice(), CTRSIZ, NEWCTR.as_slice_mut());
    NEWCTR[2] = (NEWCTR[2] + 1);
    spicelib::MOVEI(NEWCTR.as_slice(), CTRSIZ, SAVCTR.as_slice_mut());

    spicelib::ZZCTRCHK(NEWCTR.as_slice(), OLDCTR.as_slice_mut(), &mut UPDATE, ctx);

    testutil::CHCKAI(
        b"OLDCTR",
        OLDCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKAI(
        b"NEWCTR",
        NEWCTR.as_slice(),
        b"=",
        SAVCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"UPDATE", UPDATE, true, OK, ctx)?;

    //
    // Exception: try to increment counters at maximum value.
    //
    testutil::TCASE(b"Exception: try to increment at maximum value.", ctx)?;

    spicelib::ZZCTRUIN(OLDCTR.as_slice_mut(), ctx);

    testutil::CHCKSI(
        b"OLDCTR(1)",
        OLDCTR[1],
        b"=",
        spicelib::INTMAX(),
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"OLDCTR(2)",
        OLDCTR[2],
        b"=",
        spicelib::INTMAX(),
        0,
        OK,
        ctx,
    )?;

    spicelib::ZZCTRINC(OLDCTR.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(SPICEISTIRED)", OK, ctx)?;
    testutil::CHCKSI(
        b"OLDCTR(1)",
        OLDCTR[1],
        b"=",
        spicelib::INTMAX(),
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"OLDCTR(2)",
        OLDCTR[2],
        b"=",
        spicelib::INTMAX(),
        0,
        OK,
        ctx,
    )?;

    //
    // This is good enough for now.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
