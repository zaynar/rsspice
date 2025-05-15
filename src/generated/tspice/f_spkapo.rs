//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_SPKAPO ( Family of tests for SPKAPO)
pub fn F_SPKAPO(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut ET: f64 = 0.0;
    let mut ELT: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut SOBS = StackArray::<f64, 6>::new(1..=6);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut ESTATE = StackArray::<f64, 6>::new(1..=6);
    let mut SPKHAN: i32 = 0;

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
    testutil::TOPEN(b"F_SPKAPO", ctx)?;

    testutil::KILFIL(b"test_pck.ker", ctx)?;
    testutil::TSTPCK(b"test_pck.ker", true, false, ctx)?;
    testutil::TSTSPK(b"test_spk.bsp", true, &mut SPKHAN, ctx)?;
    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;

    testutil::TCASE(
        b"Make sure unrecognized aberration corrections are handle as exceptions. ",
        ctx,
    )?;

    spicelib::SPKSSB(399, ET, b"J2000", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        499,
        ET,
        b"J2000",
        SOBS.as_slice(),
        b"LTIME",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINVALIDOPTION)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that non-inertial frames are detected and diagnosed as unacceptable. ",
        ctx,
    )?;

    spicelib::SPKSSB(399, ET, b"IAU_EARTH", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        499,
        ET,
        b"IAU_EARTH",
        SOBS.as_slice(),
        b"NONE",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    testutil::TCASE(
        b"Compare SPKAPO and SPKGEO when no corrections are requested ",
        ctx,
    )?;

    spicelib::SPKGPS(
        499,
        ET,
        b"DE-125",
        399,
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"NONE",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(
        b"Compare SPKAPO and SPKAPP when light time only is requested. ",
        ctx,
    )?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"LT",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKAPP(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"LT",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    ELT = (spicelib::VNORM(ESTATE.as_slice()) / spicelib::CLIGHT());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Repeat for transmission case.", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKAPP(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    ELT = (spicelib::VNORM(ESTATE.as_slice()) / spicelib::CLIGHT());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Compare SPKAPO and SPKAPP when light time plus stellar aberration corrections are applied. ", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"LT+S",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKAPP(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"LT+S",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    ELT = (spicelib::VNORM(ESTATE.as_slice()) / spicelib::CLIGHT());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Repeat for transmission case.", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT+S",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKAPP(
        499,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT+S",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    ELT = (spicelib::VNORM(ESTATE.as_slice()) / spicelib::CLIGHT());

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.00000000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;

    testutil::TCASE(b"Examine the differences in LT and STATES for when simple light time corrections are applied versus converged light time corrections. ", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"LT",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"CN",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.0000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~", ELT, 0.0000001, OK, ctx)?;

    testutil::TCASE(b"Repeat for transmission case.", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XCN",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE.as_slice(),
        b"~/",
        ESTATE.as_slice(),
        3,
        0.0000000001,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LT", LT, b"~", ELT, 0.0000001, OK, ctx)?;

    testutil::TCASE(b"Test transmit light time:  make sure XLT corrections gives negative of position obtained when target observer vector is computed using at ET+LT using LT correction.", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    spicelib::SPKSSB(799, (ET + ELT), b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        399,
        (ET + ELT),
        b"DE-125",
        SOBS.as_slice(),
        b"LT",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::VMINUS(STATE.as_slice(), STATE2.as_slice_mut());

    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000001, OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE2.as_slice(),
        b"~~/",
        ESTATE.as_slice(),
        3,
        0.00000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Test transmit light time:  make sure XCN corrections gives negative of position obtained when target observer vector is computed using at ET+LT using CN correction.", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XCN",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    spicelib::SPKSSB(799, (ET + ELT), b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        399,
        (ET + ELT),
        b"DE-125",
        SOBS.as_slice(),
        b"CN",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::VMINUS(STATE.as_slice(), STATE2.as_slice_mut());

    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE2.as_slice(),
        b"~~/",
        ESTATE.as_slice(),
        3,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Repeat, using mixed case and white space in flag.", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b" Xc N",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    spicelib::SPKSSB(799, (ET + ELT), b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        399,
        (ET + ELT),
        b"DE-125",
        SOBS.as_slice(),
        b"C n",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    spicelib::VMINUS(STATE.as_slice(), STATE2.as_slice_mut());

    testutil::CHCKSD(b"LT", LT, b"~/", ELT, 0.00000000000001, OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE2.as_slice(),
        b"~~/",
        ESTATE.as_slice(),
        3,
        0.00000000000001,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Test transmit stellar aberration:  make sure correction is same as that obtained by using LT only and then applying STLABX. ", ctx)?;

    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT+S",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XLT",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::STLABX(
        STATE.as_slice(),
        SOBS.subarray(4),
        STATE2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE2.as_slice(),
        b"=",
        ESTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Repeat, using mixed case and white space in flag.", ctx)?;
    spicelib::SPKSSB(399, ET, b"DE-125", SOBS.as_slice_mut(), ctx)?;
    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b" X lT+ s",
        ESTATE.as_slice_mut(),
        &mut ELT,
        ctx,
    )?;

    spicelib::SPKAPO(
        799,
        ET,
        b"DE-125",
        SOBS.as_slice(),
        b"XlT",
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    spicelib::STLABX(
        STATE.as_slice(),
        SOBS.subarray(4),
        STATE2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKSD(b"LT", LT, b"=", ELT, 0.0, OK, ctx)?;
    testutil::CHCKAD(
        b"STATE",
        STATE2.as_slice(),
        b"=",
        ESTATE.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // That's all folks.
    //
    spicelib::SPKUEF(SPKHAN, ctx)?;
    testutil::KILFIL(b"test_spk.bsp", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
