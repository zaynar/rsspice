//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure F_CKFXFM ( Test Family for CKFXFM )
pub fn F_CKFXFM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut ET: f64 = 0.0;
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TMPXFM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut EFRAME: i32 = 0;
    let mut FRAME: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CKFXFM", ctx)?;

    testutil::KILFIL(b"TEST.CK", ctx)?;
    testutil::KILFIL(b"TEST.SCLK", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure nothing is found if we don\'t have a C-kernel loaded or SLCK kernel loaded. ",
        ctx,
    )?;

    spicelib::TPARSE(b"1-JAN-1990", &mut ET, &mut ERROR, ctx)?;
    spicelib::CKFXFM(-9998, ET, XFORM.as_slice_mut(), &mut FRAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure nothing is found if we don\'t have a C-kernel loaded. ",
        ctx,
    )?;

    testutil::TSTCK3(
        b"TEST.CK",
        b"TEST.SCLK",
        false,
        true,
        true,
        &mut HANDLE,
        ctx,
    )?;

    spicelib::TPARSE(b"1-JAN-1990", &mut ET, &mut ERROR, ctx)?;
    spicelib::CKFXFM(-9998, ET, XFORM.as_slice_mut(), &mut FRAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure nothing is found if we don\'t have an instrument in the C-kernel. ",
        ctx,
    )?;

    //
    // Create and load a test C-kernel.
    //
    spicelib::CKLPF(b"TEST.CK", &mut HANDLE, ctx)?;

    spicelib::TPARSE(b"1-JAN-1990", &mut ET, &mut ERROR, ctx)?;
    spicelib::CKFXFM(-9998, ET, XFORM.as_slice_mut(), &mut FRAME, &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure that when data is available, that a frame transformation if found and that it has the correct value. ", ctx)?;

    spicelib::CKFXFM(-9999, ET, XFORM.as_slice_mut(), &mut FRAME, &mut FOUND, ctx)?;
    testutil::TSTATD(ET, ROT.as_slice_mut(), AV.as_slice_mut(), ctx);

    spicelib::RAV2XF(ROT.as_slice(), AV.as_slice(), TMPXFM.as_slice_mut());
    spicelib::INVSTM(TMPXFM.as_slice(), XFORM2.as_slice_mut(), ctx)?;

    spicelib::NAMFRM(b"GALACTIC", &mut EFRAME, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"FRAME", FRAME, b"=", EFRAME, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"||",
        XFORM2.as_slice(),
        36,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~~/",
        XFORM2.as_slice(),
        36,
        0.00000000000001,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Check to make sure CKMETA is properly used and that we get the expected frame transformation. (Object -10000)", ctx)?;

    spicelib::CKFXFM(
        -10000,
        ET,
        XFORM.as_slice_mut(),
        &mut FRAME,
        &mut FOUND,
        ctx,
    )?;
    testutil::TSTATD(ET, ROT.as_slice_mut(), AV.as_slice_mut(), ctx);
    spicelib::RAV2XF(ROT.as_slice(), AV.as_slice(), TMPXFM.as_slice_mut());
    spicelib::INVSTM(TMPXFM.as_slice(), XFORM2.as_slice_mut(), ctx)?;

    spicelib::NAMFRM(b"FK4", &mut EFRAME, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"FRAME", FRAME, b"=", EFRAME, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"||",
        XFORM2.as_slice(),
        36,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~~/",
        XFORM2.as_slice(),
        36,
        0.00000000000001,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Same test for object -10001", ctx)?;

    spicelib::CKFXFM(
        -10001,
        ET,
        XFORM.as_slice_mut(),
        &mut FRAME,
        &mut FOUND,
        ctx,
    )?;
    testutil::TSTATD(ET, ROT.as_slice_mut(), AV.as_slice_mut(), ctx);

    spicelib::RAV2XF(ROT.as_slice(), AV.as_slice(), TMPXFM.as_slice_mut());
    spicelib::INVSTM(TMPXFM.as_slice(), XFORM2.as_slice_mut(), ctx)?;

    spicelib::NAMFRM(b"J2000", &mut EFRAME, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"FRAME", FRAME, b"=", EFRAME, 0, OK, ctx)?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"||",
        XFORM2.as_slice(),
        36,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~~/",
        XFORM2.as_slice(),
        36,
        0.00000000000001,
        OK,
        ctx,
    )?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Perform the same test as before, but clear the kernel pool first. We should not find a transformation now, because there is no SCLK data available. ", ctx)?;

    spicelib::CLPOOL(ctx)?;

    spicelib::TPARSE(b"1-JAN-1990", &mut ET, &mut ERROR, ctx)?;
    spicelib::CKFXFM(
        -10001,
        ET,
        XFORM.as_slice_mut(),
        &mut FRAME,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    spicelib::CKUPF(HANDLE, ctx)?;
    testutil::KILFIL(b"TEST.CK", ctx)?;
    testutil::KILFIL(b"TEST.SCLK", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
