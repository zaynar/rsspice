//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CK03E: &[u8] = b"test03err.bc";
const CK03T1: &[u8] = b"ck03t1.bc";
const VTIGHT: f64 = 0.00000000000001;
const SIDLEN: i32 = 60;
const FRNMLN: i32 = 32;
const COUNT: i32 = 4;

struct SaveVars {
    SEGID: Vec<u8>,
    REF: Vec<u8>,
    AV: StackArray<f64, 3>,
    AVVS: StackArray2D<f64, 12>,
    BEGTIM: f64,
    CLKOUT: f64,
    CMAT: StackArray2D<f64, 9>,
    DESCR: StackArray<f64, 5>,
    DSCTMP: StackArray<f64, 5>,
    EMAT: StackArray2D<f64, 9>,
    ENDTIM: f64,
    QUATS: StackArray2D<f64, 16>,
    RECRD: StackArray<f64, 8>,
    SCLKDP: StackArray<f64, 4>,
    STARTS: StackArray<f64, 4>,
    HANDLE: i32,
    INST: i32,
    NINTS: i32,
    NREC: i32,
    AVFLAG: bool,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEGID = vec![b' '; SIDLEN as usize];
        let mut REF = vec![b' '; FRNMLN as usize];
        let mut AV = StackArray::<f64, 3>::new(1..=3);
        let mut AVVS = StackArray2D::<f64, 12>::new(1..=3, 1..=COUNT);
        let mut BEGTIM: f64 = 0.0;
        let mut CLKOUT: f64 = 0.0;
        let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut DESCR = StackArray::<f64, 5>::new(1..=5);
        let mut DSCTMP = StackArray::<f64, 5>::new(1..=5);
        let mut EMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut ENDTIM: f64 = 0.0;
        let mut QUATS = StackArray2D::<f64, 16>::new(0..=3, 1..=COUNT);
        let mut RECRD = StackArray::<f64, 8>::new(1..=8);
        let mut SCLKDP = StackArray::<f64, 4>::new(1..=COUNT);
        let mut STARTS = StackArray::<f64, 4>::new(1..=COUNT);
        let mut HANDLE: i32 = 0;
        let mut INST: i32 = 0;
        let mut NINTS: i32 = 0;
        let mut NREC: i32 = 0;
        let mut AVFLAG: bool = false;
        let mut FOUND: bool = false;

        Self {
            SEGID,
            REF,
            AV,
            AVVS,
            BEGTIM,
            CLKOUT,
            CMAT,
            DESCR,
            DSCTMP,
            EMAT,
            ENDTIM,
            QUATS,
            RECRD,
            SCLKDP,
            STARTS,
            HANDLE,
            INST,
            NINTS,
            NREC,
            AVFLAG,
            FOUND,
        }
    }
}

//$Procedure F_CK03 ( CK data type 03 tests )
pub fn F_CK03(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Open the test family.
    //
    testutil::TOPEN(b"F_CK03", ctx)?;

    //
    // *****************************************************************
    //
    // CKW03 error cases:
    //
    // *****************************************************************
    //
    // Test CKW03:  start out with error handling.
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 setup", ctx)?;

    //
    // Create a segment identifier.
    //
    fstr::assign(&mut save.SEGID, b"CK type 03 test segment");

    //
    // Open a new CK file.
    //
    if spicelib::EXISTS(CK03E, ctx)? {
        spicelib::DELFIL(CK03E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK03E, b"Type 03 CK error file", 4, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BEGTIM = 2.0;
    save.ENDTIM = 10.0;

    save.SCLKDP[1] = 2.0;
    save.SCLKDP[2] = 4.0;
    save.SCLKDP[3] = 8.0;
    save.SCLKDP[4] = 10.0;

    save.INST = -1003;
    fstr::assign(&mut save.REF, b"J2000");
    save.AVFLAG = true;
    save.NINTS = 1;
    save.STARTS[1] = save.BEGTIM;
    save.NREC = 4;

    save.QUATS[[0, 1]] = 1.0;
    save.QUATS[[1, 1]] = 0.0;
    save.QUATS[[2, 1]] = 0.0;
    save.QUATS[[3, 1]] = 0.0;

    save.QUATS[[0, 2]] = f64::cos(0.1);
    save.QUATS[[1, 2]] = f64::sin(0.1);
    save.QUATS[[2, 2]] = 0.0;
    save.QUATS[[3, 2]] = 0.0;

    save.QUATS[[0, 3]] = f64::cos(0.15);
    save.QUATS[[1, 3]] = f64::sin(0.15);
    save.QUATS[[2, 3]] = 0.0;
    save.QUATS[[3, 3]] = 0.0;

    save.QUATS[[0, 4]] = f64::cos(0.25);
    save.QUATS[[1, 4]] = f64::sin(0.25);
    save.QUATS[[2, 4]] = 0.0;
    save.QUATS[[3, 4]] = 0.0;

    save.AVVS[[1, 1]] = 1.0;
    save.AVVS[[2, 1]] = 0.0;
    save.AVVS[[3, 1]] = 0.0;

    save.AVVS[[1, 2]] = 0.5;
    save.AVVS[[2, 2]] = 0.0;
    save.AVVS[[3, 2]] = 0.0;

    save.AVVS[[1, 3]] = 0.3;
    save.AVVS[[2, 3]] = 0.0;
    save.AVVS[[3, 3]] = 0.0;

    save.AVVS[[1, 4]] = 0.2;
    save.AVVS[[2, 4]] = 0.0;
    save.AVVS[[3, 4]] = 0.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 1 with invalid number of records.",
        ctx,
    )?;

    save.NREC = 0;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMREC)", OK, ctx)?;

    save.NREC = 4;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 2 with invalid number of records.",
        ctx,
    )?;

    save.NREC = -1;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMREC)", OK, ctx)?;

    save.NREC = 4;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 1 with invalid number of intervals.",
        ctx,
    )?;

    save.NINTS = 0;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMINT)", OK, ctx)?;

    save.NINTS = 1;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 2 with invalid number of intervals.",
        ctx,
    )?;

    save.NINTS = -1;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDNUMINT)", OK, ctx)?;

    save.NINTS = 1;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: invalid descriptor begin time.", ctx)?;

    save.BEGTIM = 3.0;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDESCRTIME)", OK, ctx)?;

    save.BEGTIM = 2.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: invalid descriptor end time.", ctx)?;

    save.ENDTIM = 9.0;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDESCRTIME)", OK, ctx)?;

    save.ENDTIM = 10.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: invalid reference frame name.", ctx)?;

    fstr::assign(&mut save.REF, b"MY_FRAME");

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDREFFRAME)", OK, ctx)?;

    fstr::assign(&mut save.REF, b"J2000");

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: non-printing characters in SEGID.", ctx)?;

    fstr::assign(
        fstr::substr_mut(&mut save.SEGID, 3..=3),
        &intrinsics::CHAR(7),
    );

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NONPRINTABLECHARS)", OK, ctx)?;

    fstr::assign(fstr::substr_mut(&mut save.SEGID, 3..=3), b" ");

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: SEGID that is too long.", ctx)?;

    fstr::assign(
        &mut save.SEGID,
        b"12345678901234567890123456789012345678901",
    );

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SEGIDTOOLONG)", OK, ctx)?;

    fstr::assign(&mut save.SEGID, b"CK type 03 test segment");

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: negative first SCLK time.", ctx)?;

    save.SCLKDP[1] = -2.0;
    save.BEGTIM = -2.0;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSCLKTIME)", OK, ctx)?;

    save.SCLKDP[1] = 2.0;
    save.BEGTIM = 2.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 1 with SCLK times out of order.",
        ctx,
    )?;

    save.SCLKDP[2] = 8.0;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    save.SCLKDP[2] = 4.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 2 with SCLK times out of order.",
        ctx,
    )?;

    save.SCLKDP[2] = 9.0;

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    save.SCLKDP[2] = 4.0;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: mismatching first times.", ctx)?;

    save.STARTS[1] = save.SCLKDP[2];

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESDONTMATCH)", OK, ctx)?;

    save.STARTS[1] = save.BEGTIM;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 1 with interval starts out of order.",
        ctx,
    )?;

    save.NINTS = 3;
    save.STARTS[1] = save.BEGTIM;
    save.STARTS[2] = save.SCLKDP[2];
    save.STARTS[3] = save.SCLKDP[2];

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    save.NINTS = 1;
    save.STARTS[1] = save.BEGTIM;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKW03 error case: case 2 with interval starts out of order.",
        ctx,
    )?;

    save.NINTS = 3;
    save.STARTS[1] = save.BEGTIM;
    save.STARTS[2] = save.SCLKDP[3];
    save.STARTS[3] = save.SCLKDP[2];

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;

    save.NINTS = 1;
    save.STARTS[1] = save.BEGTIM;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: invalid interval start times.", ctx)?;

    save.NINTS = 2;
    save.STARTS[1] = save.BEGTIM;
    save.STARTS[2] = ((save.SCLKDP[2] + save.SCLKDP[3]) / 2.0);

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTARTTIME)", OK, ctx)?;

    save.NINTS = 1;
    save.STARTS[1] = save.BEGTIM;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKW03 error case: zero quaternion.", ctx)?;

    spicelib::CLEARD(4, save.QUATS.subarray_mut([0, 4]));

    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROQUATERNION)", OK, ctx)?;

    save.QUATS[[0, 4]] = f64::cos(0.25);

    //
    // *****************************************************************
    //
    // End of CKW03 error cases:
    //
    // *****************************************************************
    //
    // Close the CK file at the DAF level; CKCLS won't close
    // a file without segments.
    //
    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    //  CKW03 Non-error cases:
    //
    // *****************************************************************
    //
    // Trivial case: write an CK containing all quaternions of
    // unit length.
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Test CKW03: All quaternions unit length.", ctx)?;

    //
    // Open new C-kernel.
    //
    if spicelib::EXISTS(CK03T1, ctx)? {
        spicelib::DELFIL(CK03T1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CKOPN(CK03T1, CK03T1, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write segment.
    //
    spicelib::CKW03(
        save.HANDLE,
        save.BEGTIM,
        save.ENDTIM,
        save.INST,
        &save.REF,
        save.AVFLAG,
        &save.SEGID,
        save.NREC,
        save.SCLKDP.as_slice(),
        save.QUATS.as_slice(),
        save.AVVS.as_slice(),
        save.NINTS,
        save.STARTS.as_slice(),
        ctx,
    )?;

    //
    // Close kernel.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the kernel.
    //
    spicelib::FURNSH(CK03T1, ctx)?;

    spicelib::CKGPAV(
        save.INST,
        4.0,
        1.0,
        b"J2000",
        save.CMAT.as_slice_mut(),
        save.AV.as_slice_mut(),
        &mut save.CLKOUT,
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"CLKOUT", save.CLKOUT, b"=", 4.0, 0.0, OK, ctx)?;

    spicelib::Q2M(save.QUATS.subarray([0, 2]), save.EMAT.as_slice_mut());
    testutil::CHCKAD(
        b"CMAT",
        save.CMAT.as_slice(),
        b"~",
        save.EMAT.as_slice(),
        9,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"AV",
        save.AV.as_slice(),
        b"~",
        save.AVVS.subarray([1, 2]),
        3,
        VTIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::UNLOAD(CK03T1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    //  CKNR03 Error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKNR03 error case: segment not of data type 3.", ctx)?;

    //
    // Load the a CK file and get the handle.
    //
    spicelib::DAFOPR(CK03T1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the first segment.
    //
    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the data type to anything other than 3, e.g. 0
    // Data type and angular velocity flag are given by DESCR(4).
    // Save the value in order to restore it.
    //
    spicelib::MOVED(save.DESCR.as_slice(), 5, save.DSCTMP.as_slice_mut());
    save.DSCTMP[4] = 0.0;

    spicelib::CKNR03(save.HANDLE, save.DSCTMP.as_slice(), &mut save.NREC, ctx)?;
    testutil::CHCKXC(true, b"SPICE(CKWRONGDATATYPE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    //  CKGR03 Error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"CKGR03 error case: segment not of data type 3.", ctx)?;

    //
    // Call CKGR03 using the "corrupted" DESCR from previous test.
    // Note that the CK file we are using has one segment with 4
    // records.
    //
    spicelib::CKGR03(
        save.HANDLE,
        save.DSCTMP.as_slice(),
        1,
        save.RECRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CKWRONGDATATYPE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKGR03 error case: case 1 with record which does not exist.",
        ctx,
    )?;

    spicelib::CKGR03(
        save.HANDLE,
        save.DESCR.as_slice(),
        0,
        save.RECRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CKNONEXISTREC)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"CKGR03 error case: case 2 with record which does not exist.",
        ctx,
    )?;

    spicelib::CKGR03(
        save.HANDLE,
        save.DESCR.as_slice(),
        10,
        save.RECRD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CKNONEXISTREC)", OK, ctx)?;

    //
    // Close the CK file.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    //  CKNR03 and CKGR03 non-error cases:
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Examine the segment in the file CK03T1.", ctx)?;

    //
    // Load the a CK file and get the handle.
    //
    spicelib::DAFOPR(CK03T1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the first (only) segment.
    //
    spicelib::DAFBFS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DAFGS(save.DESCR.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // How many records does this segment contain?
    //
    spicelib::CKNR03(save.HANDLE, save.DESCR.as_slice(), &mut save.NREC, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check that CKNR03 returns the expected number of records.
    //
    testutil::CHCKSI(b"NREC", save.NREC, b"=", COUNT, 0, OK, ctx)?;

    for I in 1..=save.NREC {
        //
        // Get the Ith pointing instance in the segment.
        //
        spicelib::CKGR03(
            save.HANDLE,
            save.DESCR.as_slice(),
            I,
            save.RECRD.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the time, quaternion and av.
        //
        testutil::CHCKSD(b"SCLKDP", save.RECRD[1], b"=", save.SCLKDP[I], 0.0, OK, ctx)?;

        testutil::CHCKAD(
            b"QUATS",
            save.RECRD.subarray(2),
            b"=",
            save.QUATS.subarray([0, I]),
            4,
            0.0,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"AVVS",
            save.RECRD.subarray(6),
            b"=",
            save.AVVS.subarray([1, I]),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Close the CK file.
    //
    spicelib::CKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Clean up
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete CK files.", ctx)?;

    spicelib::DAFCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(CK03E, ctx)? {
        spicelib::DELFIL(CK03E, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(CK03T1, ctx)? {
        spicelib::DELFIL(CK03T1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
