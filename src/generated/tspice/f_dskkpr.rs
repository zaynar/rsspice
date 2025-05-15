//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const FILSIZ: i32 = 255;
const TYPLEN: i32 = 4;
const MAXN: i32 = 5000;
const FRNMLN: i32 = 32;
const KERSIZ: i32 = 12;
const LNSIZE: i32 = 80;

//$Procedure F_DSKKPR ( DSK KEEPER tests )
pub fn F_DSKKPR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DSK = [b' '; FILSIZ as usize];
    let mut DSKS = ActualCharArray::new(FILSIZ, 1..=MAXN);
    let mut FILTYP = [b' '; TYPLEN as usize];
    let mut FNAME = [b' '; FILSIZ as usize];
    let mut FRAME = [b' '; FRNMLN as usize];
    let mut KERTXT = ActualCharArray::new(LNSIZE, 1..=KERSIZ);
    let mut META = [b' '; FILSIZ as usize];
    let mut SOURCE = [b' '; FILSIZ as usize];
    let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut BODYID: i32 = 0;
    let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut HANDLE: i32 = 0;
    let mut HANS = ActualArray::<i32>::new(1..=MAXN);
    let mut N: i32 = 0;
    let mut NSMALL: i32 = 0;
    let mut SURFID: i32 = 0;
    let mut TOTAL: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut FND: bool = false;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Test utility functions
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
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DSKKPR", ctx)?;

    //***********************************************************************
    //
    //     KEEPER entry point tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create a DSK file.", ctx)?;

    BODYID = 499;
    SURFID = 1;
    fstr::assign(&mut FRAME, b"IAU_MARS");

    fstr::assign(&mut DSK, b"keeper_test_0.bds");

    if spicelib::EXISTS(&DSK, ctx)? {
        spicelib::DELFIL(&DSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_SMLDSK(BODYID, SURFID, &FRAME, &DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FURNSH test: load a DSK file.", ctx)?;

    spicelib::FURNSH(&DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KTOTAL test: count loaded DSK files.", ctx)?;

    spicelib::KTOTAL(b"DSK", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (dsk)", TOTAL, b"=", 1, 0, OK, ctx)?;

    spicelib::KTOTAL(b"ALL", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (all)", TOTAL, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"UNLOAD test: unload the DSK file.", ctx)?;

    spicelib::UNLOAD(&DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of loaded DSKs.
    //
    spicelib::KTOTAL(b"DSK", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (dsk)", TOTAL, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KCLEAR test: load and then unload the DSK file.", ctx)?;

    spicelib::FURNSH(&DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of loaded DSKs.
    //
    spicelib::KTOTAL(b"DSK", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (dsk)", TOTAL, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FURNSH test: load DSKs via meta-kernel.", ctx)?;

    fstr::assign(KERTXT.get_mut(1), b"KPL/MK");
    testutil::BEGDAT(&mut KERTXT[2]);
    fstr::assign(KERTXT.get_mut(3), b"KERNELS_TO_LOAD = ( ");
    fstr::assign(KERTXT.get_mut(4), b" ");
    fstr::assign(KERTXT.get_mut(5), b"   \'keeper_test_1.bds\'");
    fstr::assign(KERTXT.get_mut(6), b"   \'keeper_test_2.bds\'");
    fstr::assign(KERTXT.get_mut(7), b"   \'keeper_test_3.bds\'");
    fstr::assign(KERTXT.get_mut(8), b"   \'keeper_test_4.bds\'");
    fstr::assign(KERTXT.get_mut(9), b"   \'keeper_test_5.bds\'");
    fstr::assign(KERTXT.get_mut(10), b" ");
    fstr::assign(KERTXT.get_mut(11), b" ) ");
    testutil::BEGTXT(&mut KERTXT[12]);

    NSMALL = 5;

    for I in 1..=NSMALL {
        fstr::assign(DSKS.get_mut(I), b"keeper_test_#.bds");
        spicelib::REPMI(&DSKS[I].to_vec(), b"#", I, &mut DSKS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_SMLDSK(BODYID, I, &FRAME, &DSKS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create meta-kernel.
    //
    fstr::assign(&mut META, b"keeper_meta.tm");

    if spicelib::EXISTS(&META, ctx)? {
        spicelib::DELFIL(&META, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::TXTOPN(&META, &mut UNIT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=KERSIZ {
        spicelib::WRITLN(&KERTXT[I], UNIT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    //
    // Load meta-kernel.
    //
    spicelib::FURNSH(&META, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KTOTAL test (MK): count loaded DSK files.", ctx)?;

    spicelib::KTOTAL(b"DSK", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (dsk)", TOTAL, b"=", NSMALL, 0, OK, ctx)?;

    spicelib::KTOTAL(b"ALL", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (all)", TOTAL, b"=", (NSMALL + 1), 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KINFO (MK): check info on loaded DSKs.", ctx)?;

    //
    // Check info on meta-kernel.
    //
    spicelib::KINFO(
        &META,
        &mut FILTYP,
        &mut SOURCE,
        &mut HANDLE,
        &mut FOUND,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FILTYP", &FILTYP, b"=", b"META", OK, ctx)?;

    //
    // Check info on DSKs.
    //
    for I in 1..=NSMALL {
        spicelib::KINFO(
            &DSKS[I],
            &mut FILTYP,
            &mut SOURCE,
            &mut HANDLE,
            &mut FOUND,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        if *OK {
            testutil::CHCKSC(b"FILTYP", &FILTYP, b"=", b"DSK", OK, ctx)?;
            testutil::CHCKSC(b"SOURCE", &SOURCE, b"=", &META, OK, ctx)?;

            //
            // Check the handle of the Ith DSK.
            //
            // Use the handle to extract the surface ID of the
            // sole segment of the current file. This ID is unique
            // among the loaded DSKs.
            //
            spicelib::DLABFS(HANDLE, DLADSC.as_slice_mut(), &mut FND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DSKGD(HANDLE, DLADSC.as_slice(), DSKDSC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            SURFID = intrinsics::IDNINT(DSKDSC[SRFIDX]);

            testutil::CHCKSI(b"SURFID", SURFID, b"=", I, 0, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KDATA (MK): check info on loaded DSKs.", ctx)?;

    //
    // Check info on meta-kernel.
    //
    spicelib::KDATA(
        1,
        b"META",
        &mut FNAME,
        &mut FILTYP,
        &mut SOURCE,
        &mut HANDLE,
        &mut FOUND,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"FNAME", &FNAME, b"=", &META, OK, ctx)?;
    testutil::CHCKSC(b"FILTYP", &FILTYP, b"=", b"META", OK, ctx)?;

    //
    // Check info on DSKs.
    //
    for I in 1..=NSMALL {
        spicelib::KDATA(
            I,
            b"DSK",
            &mut FNAME,
            &mut FILTYP,
            &mut SOURCE,
            &mut HANS[I],
            &mut FOUND,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        if *OK {
            testutil::CHCKSC(b"FILE", &FNAME, b"=", &DSKS[I], OK, ctx)?;
            testutil::CHCKSC(b"FILTYP", &FILTYP, b"=", b"DSK", OK, ctx)?;
            testutil::CHCKSC(b"SOURCE", &SOURCE, b"=", &META, OK, ctx)?;

            //
            // Check the handle of the Ith DSK.
            //
            // Use the handle to extract the surface ID of the
            // sole segment of the current file. This ID is unique
            // among the loaded DSKs.
            //
            spicelib::DLABFS(HANS[I], DLADSC.as_slice_mut(), &mut FND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DSKGD(HANS[I], DLADSC.as_slice(), DSKDSC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            SURFID = intrinsics::IDNINT(DSKDSC[SRFIDX]);

            testutil::CHCKSI(b"SURFID", SURFID, b"=", I, 0, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"UNLOAD: unload meta-kernel.", ctx)?;

    spicelib::UNLOAD(&META, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"DSK", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (dsk)", TOTAL, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FURNSH: reload meta-kernel.", ctx)?;

    spicelib::FURNSH(&META, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Tests using the maximum number of loaded DSKs follow.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"FURNSH test: load 5000 DSK files.", ctx)?;

    N = 5000;

    //
    // Create the DSKs that weren't there already from the
    // meta-kernel test cases.
    //
    for I in (NSMALL + 1)..=N {
        fstr::assign(DSKS.get_mut(I), b"keeper_test_#.bds");
        spicelib::REPMI(&DSKS[I].to_vec(), b"#", I, &mut DSKS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_SMLDSK(BODYID, I, &FRAME, &DSKS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=N {
        spicelib::FURNSH(&DSKS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KTOTAL: check loaded DSK count.", ctx)?;

    spicelib::KTOTAL(b"DSK", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (dsk)", TOTAL, b"=", (NSMALL + N), 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KINFO: check info on loaded DSKs.", ctx)?;

    for I in 1..=N {
        spicelib::KINFO(
            &DSKS[I],
            &mut FILTYP,
            &mut SOURCE,
            &mut HANDLE,
            &mut FOUND,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        if *OK {
            testutil::CHCKSC(b"FILTYP", &FILTYP, b"=", b"DSK", OK, ctx)?;

            if (I > NSMALL) {
                testutil::CHCKSC(b"SOURCE", &SOURCE, b"=", b" ", OK, ctx)?;
            }

            //
            // Check the handle of the Ith DSK.
            //
            // Use the handle to extract the surface ID of the
            // sole segment of the current file. This ID is unique
            // among the loaded DSKs.
            //
            spicelib::DLABFS(HANDLE, DLADSC.as_slice_mut(), &mut FND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DSKGD(HANDLE, DLADSC.as_slice(), DSKDSC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            SURFID = intrinsics::IDNINT(DSKDSC[SRFIDX]);

            testutil::CHCKSI(b"SURFID", SURFID, b"=", I, 0, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KCLEAR: unload all 5000 DSKs.", ctx)?;

    //
    // Unload all files.
    //
    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: unloading via UNLOAD is still an n**2 process: it's
    // too slow with 5000 files.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KDATA: check info on loaded DSKs.", ctx)?;

    //
    // Reload DSKs.
    //
    for I in 1..=N {
        spicelib::FURNSH(&DSKS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=N {
        spicelib::KDATA(
            I,
            b"DSK",
            &mut FNAME,
            &mut FILTYP,
            &mut SOURCE,
            &mut HANS[I],
            &mut FOUND,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        if *OK {
            testutil::CHCKSC(b"FILE", &FNAME, b"=", &DSKS[I], OK, ctx)?;
            testutil::CHCKSC(b"FILTYP", &FILTYP, b"=", b"DSK", OK, ctx)?;
            testutil::CHCKSC(b"SOURCE", &SOURCE, b"=", b" ", OK, ctx)?;

            //
            // Check the handle of the Ith DSK.
            //
            // Use the handle to extract the surface ID of the
            // sole segment of the current file. This ID is unique
            // among the loaded DSKs.
            //
            spicelib::DLABFS(HANS[I], DLADSC.as_slice_mut(), &mut FND, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DSKGD(HANS[I], DLADSC.as_slice(), DSKDSC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            SURFID = intrinsics::IDNINT(DSKDSC[SRFIDX]);

            testutil::CHCKSI(b"SURFID", SURFID, b"=", I, 0, OK, ctx)?;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"KCLEAR/KTOTAL: verify keeper clear operation.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"ALL", &mut TOTAL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"TOTAL (ALL)", TOTAL, b"=", 0, 0, OK, ctx)?;

    //
    //     Clean up.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::DELFIL(&DSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=N {
        spicelib::DELFIL(&DSKS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DELFIL(&META, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
