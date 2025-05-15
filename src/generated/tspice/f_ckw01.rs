//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const COUNT: i32 = 4;

//$Procedure F_CKW01 ( Family of checks for CKW01 )
pub fn F_CKW01(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut HANDLE: i32 = 0;
    let mut BEGTIM: f64 = 0.0;
    let mut ENDTIM: f64 = 0.0;
    let mut INST: i32 = 0;
    let mut REF = [b' '; 8 as usize];
    let mut AVFLAG: bool = false;
    let mut SEGID = [b' '; 42 as usize];
    let mut NREC: i32 = 0;
    let mut SCLKDP = StackArray::<f64, 4>::new(1..=COUNT);
    let mut QUATS = StackArray2D::<f64, 16>::new(0..=3, 1..=COUNT);
    let mut AVVS = StackArray2D::<f64, 12>::new(1..=3, 1..=COUNT);
    let mut EMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CLKOUT: f64 = 0.0;
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
    testutil::TOPEN(b"F_CKW01", ctx)?;
    testutil::KILFIL(b"ckw01.bc", ctx)?;
    spicelib::CKOPN(b"ckw01.bc", b"TestCK01", 1000, &mut HANDLE, ctx)?;

    BEGTIM = 1.0;
    ENDTIM = 12.0;

    SCLKDP[1] = 2.0;
    SCLKDP[2] = 4.0;
    SCLKDP[3] = 8.0;
    SCLKDP[4] = 10.0;

    INST = -1001;
    fstr::assign(&mut REF, b"J2000");
    fstr::assign(&mut SEGID, b"TEST CKTYPE 01");
    AVFLAG = true;

    QUATS[[0, 1]] = 1.0;
    QUATS[[1, 1]] = 0.0;
    QUATS[[2, 1]] = 0.0;
    QUATS[[3, 1]] = 0.0;

    QUATS[[0, 2]] = f64::cos(0.1);
    QUATS[[1, 2]] = f64::sin(0.1);
    QUATS[[2, 2]] = 0.0;
    QUATS[[3, 2]] = 0.0;

    QUATS[[0, 3]] = f64::cos(0.15);
    QUATS[[1, 3]] = f64::sin(0.15);
    QUATS[[2, 3]] = 0.0;
    QUATS[[3, 3]] = 0.0;

    QUATS[[0, 4]] = f64::cos(0.25);
    QUATS[[1, 4]] = f64::sin(0.25);
    QUATS[[2, 4]] = 0.0;
    QUATS[[3, 4]] = 0.0;

    AVVS[[1, 1]] = 1.0;
    AVVS[[2, 1]] = 0.0;
    AVVS[[3, 1]] = 0.0;

    AVVS[[1, 2]] = 0.5;
    AVVS[[2, 2]] = 0.0;
    AVVS[[3, 2]] = 0.0;

    AVVS[[1, 3]] = 0.3;
    AVVS[[2, 3]] = 0.0;
    AVVS[[3, 3]] = 0.0;

    AVVS[[1, 4]] = 0.2;
    AVVS[[2, 4]] = 0.0;
    AVVS[[3, 4]] = 0.0;

    NREC = 4;

    //
    // Check all exceptions.
    //
    testutil::TCASE(b"Case 1 with invalid number of records.", ctx)?;

    NREC = 0;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMREC)", OK, ctx)?;

    NREC = 4;

    testutil::TCASE(b"Case 2 with invalid number of records.", ctx)?;

    NREC = -1;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMREC)", OK, ctx)?;

    NREC = 4;

    testutil::TCASE(b"Case with invalid descriptor begin time.", ctx)?;

    BEGTIM = 3.0;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDESCRTIME)", OK, ctx)?;

    BEGTIM = 2.0;

    testutil::TCASE(b"Case with invalid descriptor end time.", ctx)?;

    ENDTIM = 9.0;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDESCRTIME)", OK, ctx)?;

    ENDTIM = 10.0;

    testutil::TCASE(b"Case with invalid reference frame name.", ctx)?;

    fstr::assign(&mut REF, b"MY_FRAME");

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    testutil::TCASE(b"Case with non-printing characters in SEGID.", ctx)?;

    fstr::assign(fstr::substr_mut(&mut SEGID, 5..=5), &intrinsics::CHAR(7));

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    fstr::assign(fstr::substr_mut(&mut SEGID, 5..=5), b" ");

    testutil::TCASE(b"Case with SEGID that is too long.", ctx)?;

    fstr::assign(&mut SEGID, b"12345678901234567890123456789012345678901");

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    fstr::assign(&mut SEGID, b"TEST CKTYPE 03");

    testutil::TCASE(b"Case with negative first SCLK time.", ctx)?;

    SCLKDP[1] = -2.0;
    BEGTIM = -2.0;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKTIME)", OK, ctx)?;

    SCLKDP[1] = 2.0;
    BEGTIM = 2.0;

    testutil::TCASE(b"Case 1 with SCLK times out of order.", ctx)?;

    SCLKDP[2] = 8.0;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    SCLKDP[2] = 4.0;

    testutil::TCASE(b"Case 2 with SCLK times out of order.", ctx)?;

    SCLKDP[2] = 9.0;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    SCLKDP[2] = 4.0;

    testutil::TCASE(b"Case with a zero quaternion.", ctx)?;

    spicelib::CLEARD(4, QUATS.subarray_mut([0, 4]));

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROQUATERNION)", OK, ctx)?;

    QUATS[[0, 4]] = f64::cos(0.25);

    //
    // Finally, check one good case.
    //
    testutil::TCASE(b"All quaternions unit length.", ctx)?;

    spicelib::CKW01(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        AVFLAG,
        &SEGID,
        NREC,
        SCLKDP.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKCLS(HANDLE, ctx)?;
    spicelib::CKLPF(b"ckw01.bc", &mut HANDLE, ctx)?;

    spicelib::CKGPAV(
        INST,
        4.0,
        1.0,
        b"J2000",
        CMAT.as_slice_mut(),
        AV.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"CLKOUT", CLKOUT, b"=", 4.0, 0.0, OK, ctx)?;

    spicelib::Q2M(QUATS.subarray([0, 2]), EMAT.as_slice_mut());
    testutil::CHCKAD(
        b"CMAT",
        CMAT.as_slice(),
        b"=",
        EMAT.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"AV",
        AV.as_slice(),
        b"=",
        AVVS.subarray([1, 2]),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::CKUPF(HANDLE, ctx)?;
    testutil::KILFIL(b"ckw01.bc", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
