//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const PCK0: &[u8] = b"zzbodvcd.tpc";
const LNSIZE: i32 = 255;
const BUFSIZ: i32 = 10000;
const NAMLEN: i32 = 32;

struct SaveVars {
    ITEM: Vec<u8>,
    TITLE: Vec<u8>,
    DPBUFF: ActualArray<f64>,
    XBUFF: ActualArray<f64>,
    BODYID: i32,
    N: i32,
    XN: i32,
    VARCTR: StackArray<i32, 2>,
    XCTR: StackArray<i32, 2>,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ITEM = vec![b' '; NAMLEN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut DPBUFF = ActualArray::<f64>::new(1..=BUFSIZ);
        let mut XBUFF = ActualArray::<f64>::new(1..=BUFSIZ);
        let mut BODYID: i32 = 0;
        let mut N: i32 = 0;
        let mut XN: i32 = 0;
        let mut VARCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut XCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut UPDATE: bool = false;

        Self {
            ITEM,
            TITLE,
            DPBUFF,
            XBUFF,
            BODYID,
            N,
            XN,
            VARCTR,
            XCTR,
            UPDATE,
        }
    }
}

//$Procedure F_ZZBODVCD ( ZZBODVCD tests )
pub fn F_ZZBODVCD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZBODVCD", ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create PCK.", ctx)?;

    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_PCK08(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the user counters.
    //
    spicelib::ZZCTRUIN(save.VARCTR.as_slice_mut(), ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZCTRUIN(save.XCTR.as_slice_mut(), ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Fetch moon radii.");

    testutil::TCASE(&save.TITLE, ctx)?;

    fstr::assign(&mut save.ITEM, b"RADII");
    save.BODYID = 399;

    //
    // Get expected values.
    //
    spicelib::BODVCD(
        save.BODYID,
        &save.ITEM,
        3,
        &mut save.XN,
        save.XBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZBODVCD(
        save.BODYID,
        &save.ITEM,
        3,
        save.VARCTR.as_slice_mut(),
        &mut save.N,
        save.DPBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check counter.
    //
    spicelib::ZZPCTRCK(save.XCTR.as_slice_mut(), &mut save.UPDATE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    testutil::CHCKAI(
        b"VARCTR",
        save.VARCTR.as_slice(),
        b"=",
        save.XCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;

    //
    // Check data size and values.
    //
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    testutil::CHCKAD(
        &save.ITEM,
        save.DPBUFF.as_slice(),
        b"=",
        save.XBUFF.as_slice(),
        save.XN,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Fetch moon radii a second time. Counter should remain unchanged.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::ZZBODVCD(
        save.BODYID,
        &save.ITEM,
        3,
        save.VARCTR.as_slice_mut(),
        &mut save.N,
        save.DPBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check counter.
    //
    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    testutil::CHCKAI(
        b"VARCTR",
        save.VARCTR.as_slice(),
        b"=",
        save.XCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;

    //
    // Check data size and values.
    //
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    testutil::CHCKAD(
        &save.ITEM,
        save.DPBUFF.as_slice(),
        b"=",
        save.XBUFF.as_slice(),
        save.XN,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Touch the pool; fetch moon radii a third time. Counter should be updated.",
    );

    spicelib::DVPOOL(b"BODY599_PM", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZBODVCD(
        save.BODYID,
        &save.ITEM,
        3,
        save.VARCTR.as_slice_mut(),
        &mut save.N,
        save.DPBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check counter.
    //
    spicelib::ZZPCTRCK(save.XCTR.as_slice_mut(), &mut save.UPDATE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // Check counter.
    //
    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    testutil::CHCKAI(
        b"VARCTR",
        save.VARCTR.as_slice(),
        b"=",
        save.XCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;

    //
    // Check data size and values.
    //
    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    testutil::CHCKAD(
        &save.ITEM,
        save.DPBUFF.as_slice(),
        b"=",
        save.XBUFF.as_slice(),
        save.XN,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Verify that input values are returned if the pool wasn\'t touched.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD(BUFSIZ, save.DPBUFF.as_slice_mut());
    spicelib::CLEARD(BUFSIZ, save.XBUFF.as_slice_mut());
    save.N = 0;

    spicelib::ZZBODVCD(
        save.BODYID,
        &save.ITEM,
        3,
        save.VARCTR.as_slice_mut(),
        &mut save.N,
        save.DPBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check counter.
    //
    testutil::CHCKAI(
        b"VARCTR",
        save.VARCTR.as_slice(),
        b"=",
        save.XCTR.as_slice(),
        CTRSIZ,
        OK,
        ctx,
    )?;

    //
    // Check data size and values.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 0, 0, OK, ctx)?;

    testutil::CHCKAD(
        &save.ITEM,
        save.DPBUFF.as_slice(),
        b"=",
        save.XBUFF.as_slice(),
        BUFSIZ,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::DELFIL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
