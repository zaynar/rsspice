//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CK02T1: &[u8] = b"ckw02.bc";
const VTIGHT: f64 = 0.00000000000001;
const COUNT: i32 = 4;
const SIDLEN: i32 = 42;
const FRNMLN: i32 = 32;

//$Procedure F_CK02 ( CK data type 02 tests )
pub fn F_CK02(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut REF = [b' '; FRNMLN as usize];
    let mut SEGID = [b' '; SIDLEN as usize];
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut AVVS = StackArray2D::<f64, 12>::new(1..=3, 1..=COUNT);
    let mut BEGTIM: f64 = 0.0;
    let mut CLKOUT: f64 = 0.0;
    let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut EMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ENDTIM: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=5);
    let mut DSCTMP = StackArray::<f64, 5>::new(1..=5);
    let mut RECRD = StackArray::<f64, 10>::new(1..=10);
    let mut START = StackArray::<f64, 4>::new(1..=COUNT);
    let mut STOPT = StackArray::<f64, 4>::new(1..=COUNT);
    let mut QUATS = StackArray2D::<f64, 16>::new(0..=3, 1..=COUNT);
    let mut RATES = StackArray::<f64, 4>::new(1..=COUNT);
    let mut HANDLE: i32 = 0;
    let mut INST: i32 = 0;
    let mut NREC: i32 = 0;
    let mut FOUND: bool = false;

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_CK02", ctx)?;

    testutil::KILFIL(CK02T1, ctx)?;

    spicelib::CKOPN(CK02T1, b"TestCK02", 1000, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    BEGTIM = 2f32 as f64;
    ENDTIM = 12f32 as f64;

    START[1] = 2.0;
    START[2] = 4.0;
    START[3] = 8.0;
    START[4] = 10.0;

    STOPT[1] = START[2];
    STOPT[2] = START[3];
    STOPT[3] = START[4];
    STOPT[4] = ENDTIM;

    RATES[1] = 1.0;
    RATES[2] = 0.75;
    RATES[3] = 0.5;
    RATES[4] = 0.25;

    INST = -1002;
    fstr::assign(&mut REF, b"J2000");
    fstr::assign(&mut SEGID, b"TEST CKTYPE 02");
    NREC = COUNT;

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

    //
    // *****************************************************************
    //
    // CKW02 error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case 1 with invalid number of records.",
        ctx,
    )?;

    NREC = 0;

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMREC)", OK, ctx)?;

    NREC = 4;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case 2 with invalid number of records.",
        ctx,
    )?;

    NREC = -1;

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMREC)", OK, ctx)?;

    NREC = 4;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case with invalid descriptor begin time.",
        ctx,
    )?;

    BEGTIM = 3.0;

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDESCRTIME)", OK, ctx)?;

    BEGTIM = 2.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case with invalid descriptor end time.",
        ctx,
    )?;

    ENDTIM = 11f32 as f64;

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDESCRTIME)", OK, ctx)?;

    ENDTIM = 12f32 as f64;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case with invalid reference frame name.",
        ctx,
    )?;

    fstr::assign(&mut REF, b"MY_FRAME");

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    fstr::assign(&mut REF, b"J2000");

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case with non-printing characters in SEGID.",
        ctx,
    )?;

    fstr::assign(fstr::substr_mut(&mut SEGID, 5..=5), &intrinsics::CHAR(7));

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    fstr::assign(fstr::substr_mut(&mut SEGID, 5..=5), b" ");

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW02 error case: Case with SEGID that is too long.", ctx)?;

    fstr::assign(&mut SEGID, b"12345678901234567890123456789012345678901");

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    fstr::assign(&mut SEGID, b"TEST CKTYPE 02");

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case with negative first SCLK time.",
        ctx,
    )?;

    START[1] = -2.0;
    BEGTIM = -2.0;

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKTIME)", OK, ctx)?;

    START[1] = 2.0;
    BEGTIM = 2.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case 1 with degenerate first interval.",
        ctx,
    )?;

    STOPT[1] = START[1];

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATEINTERVAL)", OK, ctx)?;

    STOPT[1] = START[2];

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW02 error case: Case 2 with degenerate first interval.",
        ctx,
    )?;

    STOPT[1] = (START[1] - 1.0);

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATEINTERVAL)", OK, ctx)?;

    STOPT[1] = START[2];

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW02 error case: Case 1 with degenerate interval.", ctx)?;

    STOPT[2] = START[2];

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATEINTERVAL)", OK, ctx)?;

    STOPT[2] = START[3];

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW02 error case: Case 2 with degenerate interval.", ctx)?;

    STOPT[2] = (START[2] - 1.0);

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DEGENERATEINTERVAL)", OK, ctx)?;

    STOPT[2] = START[3];

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW02 error case: Case 1 with times out of order.", ctx)?;

    START[3] = START[2];

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    START[3] = 8.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW02 error case: Case 2 with times out of order.", ctx)?;

    START[3] = (START[2] - 1.0);

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    START[3] = 8.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW02 error case: Case with bad stop time.", ctx)?;

    STOPT[2] = (START[3] + 1.0);

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADSTOPTIME)", OK, ctx)?;

    STOPT[2] = START[3];

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW02 error case: Case with a zero quaternion.", ctx)?;

    spicelib::CLEARD(4, QUATS.subarray_mut([0, 4]));

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROQUATERNION)", OK, ctx)?;

    QUATS[[0, 4]] = f64::cos(0.25);

    //
    // *****************************************************************
    //
    // CKW02 Non-error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    // Trivial case: write a CK containing all quaternions of
    // unit length.
    //
    testutil::TCASE(b"Test CKW02: All quaternions unit length.", ctx)?;

    spicelib::CKW02(
        HANDLE,
        BEGTIM,
        ENDTIM,
        INST,
        &REF,
        &SEGID,
        NREC,
        START.as_slice(),
        STOPT.as_slice(),
        QUATS.as_slice(),
        AVVS.as_slice(),
        RATES.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKCLS(HANDLE, ctx)?;
    spicelib::CKLPF(CK02T1, &mut HANDLE, ctx)?;

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
        b"~",
        EMAT.as_slice(),
        9,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"AV",
        AV.as_slice(),
        b"~",
        AVVS.subarray([1, 2]),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::UNLOAD(CK02T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // CKNR02 Error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKNR02 error case: segment not of data type 2.", ctx)?;

    //
    // Load the a CK file and get the handle.
    //
    spicelib::DAFOPR(CK02T1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the first segment.
    //
    spicelib::DAFBFS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the data type to anything other than 2, e.g. 0
    // Data type and angular velocity flag are given by DESCR(4).
    // Save the value in order to restore it.
    //
    spicelib::MOVED(DESCR.as_slice(), 5, DSCTMP.as_slice_mut());
    DSCTMP[4] = 0.0;

    spicelib::CKNR02(HANDLE, DSCTMP.as_slice(), &mut NREC, ctx)?;
    testutil::CHCKXC(true, b"SPICE(CKWRONGDATATYPE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // CKGR02 Error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKGR02 error case: segment not of data type 2.", ctx)?;

    //
    // Call CKGR02 using the "corrupted" DESCR from previous test.
    // Note that the CK file we are using has one segment with 4
    // records.
    //
    spicelib::CKGR02(HANDLE, DSCTMP.as_slice(), 1, RECRD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(CKWRONGDATATYPE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKGR02 error case: case 1 with record which does not exist.",
        ctx,
    )?;

    spicelib::CKGR02(HANDLE, DESCR.as_slice(), 0, RECRD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(CKNONEXISTREC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKGR02 error case: case 2 with record which does not exist.",
        ctx,
    )?;

    spicelib::CKGR02(HANDLE, DESCR.as_slice(), 10, RECRD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(CKNONEXISTREC)", OK, ctx)?;

    //
    // Close the CK file.
    //
    spicelib::CKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // CKNR02 and CKGR02 non-error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Examine the segment in the file CK02T1.", ctx)?;

    //
    // Load the a CK file and get the handle.
    //
    spicelib::DAFOPR(CK02T1, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the first (only) segment.
    //
    spicelib::DAFBFS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // How many records does this segment contain?
    //
    spicelib::CKNR02(HANDLE, DESCR.as_slice(), &mut NREC, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that CKNR02 returns the expected number of records.
    //
    testutil::CHCKSI(b"NREC", NREC, b"=", COUNT, 0, OK, ctx)?;

    for I in 1..=NREC {
        //
        // Get the Ith pointing instance in the segment.
        //
        spicelib::CKGR02(HANDLE, DESCR.as_slice(), I, RECRD.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the times, rate, quaternion and av.
        //
        testutil::CHCKSD(b"BEGTIM", RECRD[1], b"=", START[I], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"ENDTIM", RECRD[2], b"=", STOPT[I], 0.0, OK, ctx)?;
        testutil::CHCKSD(b"RATE", RECRD[3], b"=", RATES[I], 0.0, OK, ctx)?;

        testutil::CHCKAD(
            b"QUATS",
            RECRD.subarray(4),
            b"=",
            QUATS.subarray([0, I]),
            4,
            0.0,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"AVVS",
            RECRD.subarray(8),
            b"=",
            AVVS.subarray([1, I]),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Close the CK file.
    //
    spicelib::CKCLS(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Clean up
    //
    // *****************************************************************
    //
    spicelib::CKUPF(HANDLE, ctx)?;
    testutil::KILFIL(CK02T1, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
