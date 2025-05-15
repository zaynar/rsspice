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
const CTRSIZ: i32 = 2;
const BTSIZE: i32 = 100;
const STSIZE: i32 = 10000;
const FTSIZE: i32 = 5000;
const FILSIZ: i32 = 255;
const TYPLEN: i32 = 4;
const MAXN: i32 = (FTSIZE + 4);
const FRNMLN: i32 = 32;
const KERSIZ: i32 = 12;
const LNSIZE: i32 = 80;
const LBLSIZ: i32 = 40;

struct SaveVars {
    BTDSK: Vec<u8>,
    DSKS: ActualCharArray,
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    OVRDSK: Vec<u8>,
    DSKDSC: StackArray<f64, 24>,
    XDSKDS: StackArray<f64, 24>,
    BODYID: i32,
    BTHAN: i32,
    CENTRS: StackArray<i32, 101>,
    CLASS: i32,
    CLSSID: i32,
    DLADSC: StackArray<i32, 8>,
    DSKCTR: StackArray<i32, 2>,
    FILENO: i32,
    FRAMID: i32,
    HANDLE: i32,
    HANDLS: ActualArray<i32>,
    I: i32,
    J: i32,
    K: i32,
    N: i32,
    NBOD: i32,
    NEARTH: i32,
    NSAT: i32,
    NSEG: i32,
    NXTDSC: StackArray<i32, 8>,
    OVRHAN: i32,
    PRVDSC: StackArray<i32, 8>,
    SURFID: i32,
    XDLADS: StackArray<i32, 8>,
    FND: bool,
    FOUND: bool,
    RETVAL: bool,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BTDSK = vec![b' '; FILSIZ as usize];
        let mut DSKS = ActualCharArray::new(FILSIZ, 1..=MAXN);
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LBLSIZ as usize];
        let mut OVRDSK = vec![b' '; FILSIZ as usize];
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut XDSKDS = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut BODYID: i32 = 0;
        let mut BTHAN: i32 = 0;
        let mut CENTRS = StackArray::<i32, 101>::new(1..=(BTSIZE + 1));
        let mut CLASS: i32 = 0;
        let mut CLSSID: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut DSKCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut FILENO: i32 = 0;
        let mut FRAMID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HANDLS = ActualArray::<i32>::new(1..=MAXN);
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NBOD: i32 = 0;
        let mut NEARTH: i32 = 0;
        let mut NSAT: i32 = 0;
        let mut NSEG: i32 = 0;
        let mut NXTDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut OVRHAN: i32 = 0;
        let mut PRVDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut SURFID: i32 = 0;
        let mut XDLADS = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut FND: bool = false;
        let mut FOUND: bool = false;
        let mut RETVAL: bool = false;
        let mut UPDATE: bool = false;

        Self {
            BTDSK,
            DSKS,
            FRAME,
            LABEL,
            OVRDSK,
            DSKDSC,
            XDSKDS,
            BODYID,
            BTHAN,
            CENTRS,
            CLASS,
            CLSSID,
            DLADSC,
            DSKCTR,
            FILENO,
            FRAMID,
            HANDLE,
            HANDLS,
            I,
            J,
            K,
            N,
            NBOD,
            NEARTH,
            NSAT,
            NSEG,
            NXTDSC,
            OVRHAN,
            PRVDSC,
            SURFID,
            XDLADS,
            FND,
            FOUND,
            RETVAL,
            UPDATE,
        }
    }
}

//$Procedure F_ZZDSKBSR ( ZZDSKBSR tests )
pub fn F_ZZDSKBSR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL functions
    //

    //
    // ZZDSKBSR parameters
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
    // Save everything in order to avoid stack problems on
    // some platforms.
    //

    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZDSKBSR", ctx)?;

    //***********************************************************************
    //
    //     ZZDSKBSR entry point tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKBSR error: direct call to ZZDSKBSR.", ctx)?;

    spicelib::ZZDSKBSR(
        b" ",
        1,
        1,
        spicelib::ZZDSKBDC,
        save.DSKCTR.as_slice(),
        save.UPDATE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DSKBOGUSENTRY)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Setup: create two DSK files, each containing data for bodies 499 and 599.",
        ctx,
    )?;

    fstr::assign(save.DSKS.get_mut(1), b"zzdskbsr_test_1.bds");
    fstr::assign(save.DSKS.get_mut(2), b"zzdskbsr_test_2.bds");

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if spicelib::EXISTS(&save.DSKS[save.I], ctx)? {
                spicelib::DELFIL(&save.DSKS[save.I], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            save.BODYID = 499;
            fstr::assign(&mut save.FRAME, b"IAU_MARS");
            save.SURFID = save.I;
            testutil::T_SMLDSK(
                save.BODYID,
                save.SURFID,
                &save.FRAME,
                &save.DSKS[save.I],
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Append segment.
            //
            save.BODYID = 599;
            fstr::assign(&mut save.FRAME, b"IAU_JUPITER");
            save.SURFID = save.I;

            testutil::T_SMLDSK(
                save.BODYID,
                save.SURFID,
                &save.FRAME,
                &save.DSKS[save.I],
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //***********************************************************************
    //
    //     ZZDSKLSF/ZZDSKUSF/ZZDSKBSS/ZZDSKSNS basic tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKSNS Error case: search for segments for body 499 without first calling ZZDSKBSS.",
        ctx,
    )?;

    save.BODYID = 499;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(CALLZZDSKBSSFIRST)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKBSS Error case: search for segments for body 499. No DSKs have been loaded yet.",
        ctx,
    )?;

    save.BODYID = 499;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: Load first DSK; search for segments for body 499.",
        ctx,
    )?;

    //
    // Initialize the local DSK state counter.
    //
    spicelib::ZZCTRUIN(save.DSKCTR.as_slice_mut(), ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the first DSK file.
    //
    spicelib::ZZDSKLSF(&save.DSKS[1], &mut save.HANDLS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.BODYID = 499;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find one segment.
    //
    testutil::CHCKSL(b"FOUND (1)", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        //
        // Check the other outputs.
        //
        testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HANDLS[1], 0, OK, ctx)?;

        spicelib::DLABFS(
            save.HANDLS[1],
            save.XDLADS.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLABFS FOUND", save.FND, true, OK, ctx)?;

        spicelib::DSKGD(
            save.HANDLS[1],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Continue the search. We should not find another segment.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (2)", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKLSF: Search for segments for body 599.", ctx)?;

    save.BODYID = 599;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find one segment.
    //
    testutil::CHCKSL(b"FOUND (1)", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        //
        // Check the other outputs. We need to locate the second
        // segment in the file first.
        //
        testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HANDLS[1], 0, OK, ctx)?;

        spicelib::DLABFS(
            save.HANDLS[1],
            save.XDLADS.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DLAFNS(
            save.HANDLS[1],
            save.XDLADS.as_slice(),
            save.NXTDSC.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLAFNS FOUND", save.FND, true, OK, ctx)?;

        spicelib::DSKGD(
            save.HANDLS[1],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Continue the search. We should not find another segment.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (2)", save.FOUND, false, OK, ctx)?;

    //
    // Check the state change status. We expect that no update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Search for segments for body 699. None should be found. No error should be signaled.",
        ctx,
    )?;

    save.BODYID = 699;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find...nothing.
    //
    testutil::CHCKSL(b"FOUND (1)", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: Load second DSK; search for segments for body 499.",
        ctx,
    )?;

    spicelib::ZZDSKLSF(&save.DSKS[2], &mut save.HANDLS[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.BODYID = 499;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = 2;
    save.I = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find two segments. The first one found should
        // be in the file loaded last.
        //
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.I = (save.I + 1);
        save.J = ((save.NSEG + 1) - save.I);

        if save.FOUND {
            //
            // Check the other outputs.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HANDLS[save.J],
                0,
                OK,
                ctx,
            )?;
            //
            // Search backwards in the file for the expected segment.
            // In each case we're looking for the first segment in
            // the file.
            //
            spicelib::DLABBS(
                save.HANDLS[save.J],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DLAFPS(
                save.HANDLS[save.J],
                save.XDLADS.as_slice(),
                save.PRVDSC.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"DLAFPS FOUND", save.FND, true, OK, ctx)?;

            spicelib::DSKGD(
                save.HANDLS[save.J],
                save.XDLADS.as_slice(),
                save.XDSKDS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                b"DLADSC",
                save.DLADSC.as_slice(),
                b"=",
                save.XDLADS.as_slice(),
                DLADSZ,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"DSKDSC",
                save.DSKDSC.as_slice(),
                b"=",
                save.XDSKDS.as_slice(),
                DSKDSZ,
                0.0,
                OK,
                ctx,
            )?;
        }

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: Keep both files loaded; search for segments for body 599.",
        ctx,
    )?;

    save.BODYID = 599;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = 2;
    save.I = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find two segments. The first one found should
        // be in the file loaded last.
        //
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.I = (save.I + 1);
        save.J = ((save.NSEG + 1) - save.I);

        if save.FOUND {
            //
            // Check the other outputs.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HANDLS[save.J],
                0,
                OK,
                ctx,
            )?;
            //
            // Search backwards in the file for the expected segment.
            // In each case we're looking for the second segment in
            // the file.
            //
            spicelib::DLABBS(
                save.HANDLS[save.J],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"DLABBS FOUND", save.FND, true, OK, ctx)?;

            spicelib::DSKGD(
                save.HANDLS[save.J],
                save.XDLADS.as_slice(),
                save.XDSKDS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                b"DLADSC",
                save.DLADSC.as_slice(),
                b"=",
                save.XDLADS.as_slice(),
                DLADSZ,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"DSKDSC",
                save.DSKDSC.as_slice(),
                b"=",
                save.XDSKDS.as_slice(),
                DSKDSZ,
                0.0,
                OK,
                ctx,
            )?;
        }

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: Load first DSK a second time; verify that the handle is unchanged.",
        ctx,
    )?;

    spicelib::ZZDSKLSF(&save.DSKS[1], &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HANDLS[1], 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: Load second DSK a second time; verify that the handle is unchanged.",
        ctx,
    )?;

    spicelib::ZZDSKLSF(&save.DSKS[2], &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HANDLS[2], 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKUSF: Unload the first file; keep the second file loaded; search for segments for body 499.", ctx)?;

    spicelib::ZZDSKUSF(save.HANDLS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.BODYID = 499;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find one segment.
    //
    testutil::CHCKSL(b"FOUND (1)", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        //
        // Check the other outputs.
        //
        testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HANDLS[2], 0, OK, ctx)?;

        spicelib::DLABFS(
            save.HANDLS[2],
            save.XDLADS.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLABFS FOUND", save.FND, true, OK, ctx)?;

        spicelib::DSKGD(
            save.HANDLS[2],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Continue the search. We should not find another segment.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (2)", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKUSF: Continue the previous case; search for segments for body 599.",
        ctx,
    )?;

    save.BODYID = 599;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find one segment.
    //
    testutil::CHCKSL(b"FOUND (1)", save.FOUND, true, OK, ctx)?;

    if save.FOUND {
        //
        // Check the other outputs.
        //
        testutil::CHCKSI(b"HANDLE", save.HANDLE, b"=", save.HANDLS[2], 0, OK, ctx)?;

        //
        // Use a backward search here.
        //
        spicelib::DLABBS(
            save.HANDLS[2],
            save.XDLADS.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLABFS FOUND", save.FND, true, OK, ctx)?;

        spicelib::DSKGD(
            save.HANDLS[2],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // Continue the search. We should not find another segment.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND (2)", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKUSF: unload the second file. Call ZZDSKBSS to try to start a search for segments for body 499.", ctx)?;

    spicelib::ZZDSKUSF(save.HANDLS[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.BODYID = 499;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOLOADEDDSKFILES)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: Load first DSK a third time; verify that the handle is new.",
        ctx,
    )?;

    spicelib::ZZDSKLSF(&save.DSKS[1], &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        (save.HANDLS[2] + 1),
        0,
        OK,
        ctx,
    )?;

    save.HANDLS[1] = save.HANDLE;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: Load second DSK a third time; verify that the handle is unchanged.",
        ctx,
    )?;

    spicelib::ZZDSKLSF(&save.DSKS[2], &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        (save.HANDLS[1] + 1),
        0,
        OK,
        ctx,
    )?;

    save.HANDLS[2] = save.HANDLE;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: After reload: search for segments for body 499.",
        ctx,
    )?;

    //
    // We must first build a segment list for body 499, so the segments
    // will be found in the expected order.
    //
    save.BODYID = 499;

    spicelib::ZZDSKBBL(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = 2;
    save.I = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find two segments. The first one found should
        // be in the file loaded last.
        //
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.I = (save.I + 1);
        save.J = ((save.NSEG + 1) - save.I);

        if save.FOUND {
            //
            // Check the other outputs.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HANDLS[save.J],
                0,
                OK,
                ctx,
            )?;
            //
            // Search backwards in the file for the expected segment.
            // In each case we're looking for the first segment in
            // the file.
            //
            spicelib::DLABBS(
                save.HANDLS[save.J],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DLAFPS(
                save.HANDLS[save.J],
                save.XDLADS.as_slice(),
                save.PRVDSC.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"DLAFPS FOUND", save.FND, true, OK, ctx)?;

            spicelib::DSKGD(
                save.HANDLS[save.J],
                save.XDLADS.as_slice(),
                save.XDSKDS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                b"DLADSC",
                save.DLADSC.as_slice(),
                b"=",
                save.XDLADS.as_slice(),
                DLADSZ,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"DSKDSC",
                save.DSKDSC.as_slice(),
                b"=",
                save.XDSKDS.as_slice(),
                DSKDSZ,
                0.0,
                OK,
                ctx,
            )?;
        }

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: After reload: search for segments for body 599.",
        ctx,
    )?;

    save.BODYID = 599;
    //
    // Rebuild the segment list for 599.
    //
    spicelib::ZZDSKBBL(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Start a search.
    //
    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = 2;
    save.I = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find two segments. The first one found should
        // be in the file loaded last.
        //
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.I = (save.I + 1);
        save.J = ((save.NSEG + 1) - save.I);

        if save.FOUND {
            //
            // Check the other outputs.
            //
            testutil::CHCKSI(
                b"HANDLE",
                save.HANDLE,
                b"=",
                save.HANDLS[save.J],
                0,
                OK,
                ctx,
            )?;
            //
            // Search backwards in the file for the expected segment.
            // In each case we're looking for the second segment in
            // the file.
            //
            spicelib::DLABBS(
                save.HANDLS[save.J],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"DLABBS FOUND", save.FND, true, OK, ctx)?;

            spicelib::DSKGD(
                save.HANDLS[save.J],
                save.XDLADS.as_slice(),
                save.XDSKDS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKAI(
                b"DLADSC",
                save.DLADSC.as_slice(),
                b"=",
                save.XDLADS.as_slice(),
                DLADSZ,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"DSKDSC",
                save.DSKDSC.as_slice(),
                b"=",
                save.XDSKDS.as_slice(),
                DSKDSZ,
                0.0,
                OK,
                ctx,
            )?;
        }

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //***********************************************************************
    //
    //
    //     ZZDSKLSF/ZZDSKBSS/ZZDSKSNS full capability tests
    //
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Setup: create DSK containing data for BTSIZE+1bodies. There are two segments per body.",
        ctx,
    )?;

    fstr::assign(&mut save.BTDSK, b"zzdskbsr_body_table.bds");

    if spicelib::EXISTS(&save.BTDSK, ctx)? {
        spicelib::DELFIL(&save.BTDSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // We want to take advantage of the SPICE built-in non-inertial
    // frames. We need BTSIZE+1 of them, so we must include the nonsense
    // frames such as those associated with planetary barycenters.
    // This doesn't matter from the perspective of this test.
    //
    //
    save.NBOD = (BTSIZE + 1);

    save.N = 0;
    save.I = 1;

    while ((save.N < save.NBOD) && (save.I < (BTSIZE + 10))) {
        save.FRAMID = (10000 + save.I);

        spicelib::FRINFO(
            save.FRAMID,
            &mut save.BODYID,
            &mut save.CLASS,
            &mut save.CLSSID,
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Note that there's a gap in the sequence for class ID 3000;
        // there may be other gaps.
        //
        // Also, we want to avoid using the earth, since IAU_EARTH
        // and EARTH_FIXED are both built-in frames centered on
        // the earth. This is an unnecessary complication.
        //
        if (save.FOUND && (save.BODYID != 399)) {
            save.N = (save.N + 1);

            spicelib::FRMNAM(save.FRAMID, &mut save.FRAME, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if fstr::eq(&save.FRAME, b" ") {
                spicelib::SETMSG(b"Frame name for ID # is blank.", ctx);
                spicelib::ERRINT(b"#", save.FRAMID, ctx);
                spicelib::SIGERR(b"SPICE(NOTRANSLATION)", ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            save.SURFID = save.N;

            testutil::T_SMLDSK(save.BODYID, save.SURFID, &save.FRAME, &save.BTDSK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SURFID = -save.N;

            testutil::T_SMLDSK(save.BODYID, save.SURFID, &save.FRAME, &save.BTDSK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.CENTRS[save.N] = save.BODYID;
        }

        save.I = (save.I + 1);
    }

    testutil::CHCKSI(b"N", save.N, b"=", save.NBOD, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Load the DSK that contains BTSIZE+1 bodies. Look up data for each one.",
        ctx,
    )?;

    //
    // This test will force one body out of the body list.
    //
    spicelib::ZZDSKLSF(&save.BTDSK, &mut save.BTHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = save.NBOD;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.BODYID = save.CENTRS[save.I];

            spicelib::ZZDSKBSS(save.BODYID, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Prepare segment selection test function for use in BSR search.
            //
            save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Perform the search. We expect to find two segments per body.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = 2;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    spicelib::ZZDSKSNS(
                        spicelib::ZZDSKBDC,
                        &mut save.HANDLE,
                        save.DLADSC.as_slice_mut(),
                        save.DSKDSC.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"ZZDSKSNS FOUND", save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        //
                        // Check the other outputs.
                        //
                        fstr::assign(&mut save.LABEL, b"HANDLE I=@; J=@");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.J, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSI(&save.LABEL, save.HANDLE, b"=", save.BTHAN, 0, OK, ctx)?;

                        //
                        // Search backwards in the file for the expected segment.
                        //
                        if ((save.I == save.NBOD) && (save.J == 1)) {
                            spicelib::DLABBS(
                                save.BTHAN,
                                save.XDLADS.as_slice_mut(),
                                &mut save.FND,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSL(b"DLABBS FOUND", save.FND, true, OK, ctx)?;
                        } else {
                            spicelib::DLAFPS(
                                save.BTHAN,
                                save.XDLADS.as_slice(),
                                save.PRVDSC.as_slice_mut(),
                                &mut save.FND,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::CHCKSL(b"DLAFPS FOUND", save.FND, true, OK, ctx)?;

                            spicelib::MOVEI(
                                save.PRVDSC.as_slice(),
                                DLADSZ,
                                save.XDLADS.as_slice_mut(),
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }

                        spicelib::DSKGD(
                            save.BTHAN,
                            save.XDLADS.as_slice(),
                            save.XDSKDS.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        fstr::assign(&mut save.LABEL, b"DLADSC I=@; J=@");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.J, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKAI(
                            &save.LABEL,
                            save.DLADSC.as_slice(),
                            b"=",
                            save.XDLADS.as_slice(),
                            DLADSZ,
                            OK,
                            ctx,
                        )?;

                        fstr::assign(&mut save.LABEL, b"DSKDSC I=@; J=@");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.I, &mut save.LABEL, ctx);
                        spicelib::REPMI(&save.LABEL.to_vec(), b"@", save.J, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKAD(
                            &save.LABEL,
                            save.DSKDSC.as_slice(),
                            b"=",
                            save.XDSKDS.as_slice(),
                            DSKDSZ,
                            0.0,
                            OK,
                            ctx,
                        )?;
                    }

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Unload this file to simplify the later tests.
    //
    spicelib::ZZDSKUSF(save.BTHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create FTSIZE+1 DSKs containing data for the earth. Each file but the last contains two segments. The last one contains five segments.", ctx)?;

    save.NEARTH = (FTSIZE + 1);

    {
        let m1__: i32 = 3;
        let m2__: i32 = (2 + save.NEARTH);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(save.DSKS.get_mut(save.I), b"zzdskbsr_test_#.bds");
            spicelib::REPMI(
                &save.DSKS[save.I].to_vec(),
                b"#",
                save.I,
                &mut save.DSKS[save.I],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if spicelib::EXISTS(&save.DSKS[save.I], ctx)? {
                spicelib::DELFIL(&save.DSKS[save.I], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            save.BODYID = 399;
            fstr::assign(&mut save.FRAME, b"IAU_EARTH");
            save.SURFID = save.I;
            testutil::T_SMLDSK(
                save.BODYID,
                save.SURFID,
                &save.FRAME,
                &save.DSKS[save.I],
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.SURFID = -save.I;
            testutil::T_SMLDSK(
                save.BODYID,
                save.SURFID,
                &save.FRAME,
                &save.DSKS[save.I],
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    save.I = (2 + save.NEARTH);

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        save.J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SURFID = save.J;
            testutil::T_SMLDSK(
                save.BODYID,
                save.SURFID,
                &save.FRAME,
                &save.DSKS[save.I],
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKLSF: load FTSIZE-2 earth DSKs.", ctx)?;

    //
    // Note that DSKs 1 and 2 are still loaded.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = FTSIZE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKLSF(&save.DSKS[save.I], &mut save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the state change status. We expect that an update is
            // indicated.
            //
            spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKLSF error case: try to load one more earth DSK.", ctx)?;

    save.I = (FTSIZE + 1);

    spicelib::ZZDSKLSF(&save.DSKS[save.I], &mut save.HANDLS[save.I], ctx)?;
    testutil::CHCKXC(true, b"SPICE(FTFULL)", OK, ctx)?;
    //
    // The error message above brought to you by ZZDDHOPN.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKLSF: unload first two DSKs and load remaining earth DSKs, except for the last one.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKUSF(save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Check the state change status. We expect that an update is
            // indicated.
            //
            spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = (FTSIZE + save.I);

            spicelib::ZZDSKLSF(&save.DSKS[save.J], &mut save.HANDLS[save.J], ctx)?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Check the state change status. We expect that an update is
            // indicated.
            //
            spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKBSS/ZZDSKSNS: prepare to look up all earth segments but those in the last earth file. Force ZZDSKBSR to build a segment list for the earth via a call to ZZDSKBBL.", ctx)?;

    save.BODYID = 399;

    spicelib::ZZDSKBBL(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that no update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKBSS/ZZDSKSNS: look up all loaded earth segments ",
        ctx,
    )?;

    //
    // Note that the last earth file is not loaded.
    //

    save.BODYID = 399;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that no update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, false, OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = STSIZE;
    save.I = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find STSIZE segments.
        //
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.I = (save.I + 1);
        save.J = ((save.NSEG + 1) - save.I);
        save.FILENO = ((2 + ((save.J - 1) / 2)) + 1);

        //
        // Check the state change status. We expect that no update is
        // indicated.
        //
        spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"UPDATE", save.UPDATE, false, OK, ctx)?;

        //
        // Check the other outputs.
        //
        testutil::CHCKSI(
            b"HANDLE",
            save.HANDLE,
            b"=",
            save.HANDLS[save.FILENO],
            0,
            OK,
            ctx,
        )?;

        if spicelib::ODD(save.I) {
            //
            // Start a backward search for the expected
            // segment.
            //
            spicelib::DLABBS(
                save.HANDLS[save.FILENO],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"(1) DLABFS FOUND", save.FND, true, OK, ctx)?;
        } else {
            //
            // Continue the backward search for the
            // expected segment.
            //
            spicelib::DLAFPS(
                save.HANDLS[save.FILENO],
                save.DLADSC.as_slice(),
                save.PRVDSC.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::DSKGD(
            save.HANDLS[save.FILENO],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"(1) DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"(1) DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // We should have found STSIZE segments.
    //
    testutil::CHCKSI(b"num. segments found", save.I, b"=", STSIZE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKBSS/ZZDSKSNS: repeat look up.", ctx)?;

    save.BODYID = 399;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that no update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, false, OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = STSIZE;
    save.I = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find STSIZE segments.
        //
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.I = (save.I + 1);
        save.J = ((save.NSEG + 1) - save.I);
        save.FILENO = ((2 + ((save.J - 1) / 2)) + 1);

        //
        // Check the state change status. We expect that no update is
        // indicated.
        //
        spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"UPDATE", save.UPDATE, false, OK, ctx)?;

        //
        // Check the other outputs.
        //
        testutil::CHCKSI(
            b"HANDLE",
            save.HANDLE,
            b"=",
            save.HANDLS[save.FILENO],
            0,
            OK,
            ctx,
        )?;

        if spicelib::ODD(save.I) {
            //
            // Start a backward search for the expected
            // segment.
            //
            spicelib::DLABBS(
                save.HANDLS[save.FILENO],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"(1) DLABFS FOUND", save.FND, true, OK, ctx)?;
        } else {
            //
            // Continue the backward search for the
            // expected segment.
            //
            spicelib::DLAFPS(
                save.HANDLS[save.FILENO],
                save.DLADSC.as_slice(),
                save.PRVDSC.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::DSKGD(
            save.HANDLS[save.FILENO],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"(1) DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"(1) DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // We should have found STSIZE segments.
    //
    testutil::CHCKSI(b"num. segments found", save.I, b"=", STSIZE, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"ZZDSKBSS/ZZDSKSNS: unload even-indexed earth DSKs. Look up remaining segments. ",
        ctx,
    )?;

    {
        let m1__: i32 = 4;
        let m2__: i32 = (2 + FTSIZE);
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKUSF(save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // Start a new search.
    //
    save.BODYID = 399;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = STSIZE;

    //
    // I is the segment offset from the head of the list. I is 1-based.
    // K is the iteration count.

    save.I = 0;
    save.K = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find STSIZE/2 segments.
        //
        save.K = (save.K + 1);

        if spicelib::EVEN(save.I) {
            save.I = (save.I + 3);
        } else {
            save.I = (save.I + 1);
        }

        save.J = ((save.NSEG + 1) - save.I);
        save.FILENO = ((2 + ((save.J - 1) / 2)) + 1);

        //
        // Check the other outputs.
        //
        testutil::CHCKSI(
            b"HANDLE",
            save.HANDLE,
            b"=",
            save.HANDLS[save.FILENO],
            0,
            OK,
            ctx,
        )?;

        if spicelib::EVEN(save.J) {
            //
            // Start a backward search in the file at index FILENO for the
            // expected segment.
            //
            spicelib::DLABBS(
                save.HANDLS[save.FILENO],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"(1) DLABFS FOUND", save.FND, true, OK, ctx)?;
        } else {
            //
            // Continue the backward search for the
            // expected segment.
            //
            spicelib::DLAFPS(
                save.HANDLS[save.FILENO],
                save.DLADSC.as_slice(),
                save.PRVDSC.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::DSKGD(
            save.HANDLS[save.FILENO],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"(1) DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"(1) DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // We should have found STSIZE/2 segments.
    //
    testutil::CHCKSI(
        b"num. segments found",
        save.K,
        b"=",
        (STSIZE / 2),
        0,
        OK,
        ctx,
    )?;

    //
    // Reload the earth kernels indexed 1 through FTSIZE to restore
    // the original file priorities.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = (FTSIZE + 2);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKLSF(&save.DSKS[save.I], &mut save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Force construction of a new segment list.
    //
    spicelib::ZZDSKBBL(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(
        b"ZZDSKBSS/ZZDSKSNS: unload odd-indexed earth DSKs. Look up remaining segments. ",
        ctx,
    )?;

    {
        let m1__: i32 = 3;
        let m2__: i32 = (2 + FTSIZE);
        let m3__: i32 = 2;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKUSF(save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    //
    // Start a new search.
    //
    save.BODYID = 399;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = STSIZE;

    //
    // I is the segment offset from the head of the list. I is 1-based.
    // K is the iteration count.

    save.I = -2;
    save.K = 0;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while save.FOUND {
        //
        // We should find STSIZE/2 segments.
        //
        save.K = (save.K + 1);

        if spicelib::EVEN(save.I) {
            save.I = (save.I + 3);
        } else {
            save.I = (save.I + 1);
        }

        save.J = ((save.NSEG + 1) - save.I);
        save.FILENO = ((2 + ((save.J - 1) / 2)) + 1);

        //
        // Check the other outputs.
        //
        testutil::CHCKSI(
            b"HANDLE",
            save.HANDLE,
            b"=",
            save.HANDLS[save.FILENO],
            0,
            OK,
            ctx,
        )?;

        if spicelib::EVEN(save.J) {
            //
            // Start a backward search in the file at index FILENO for the
            // expected segment.
            //
            spicelib::DLABBS(
                save.HANDLS[save.FILENO],
                save.XDLADS.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(b"(1) DLABFS FOUND", save.FND, true, OK, ctx)?;
        } else {
            //
            // Continue the backward search for the
            // expected segment.
            //
            spicelib::DLAFPS(
                save.HANDLS[save.FILENO],
                save.DLADSC.as_slice(),
                save.PRVDSC.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::DSKGD(
            save.HANDLS[save.FILENO],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"(1) DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"(1) DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Fetch information for the next segment.
        //
        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // We should have found STSIZE/2 segments.
    //
    testutil::CHCKSI(
        b"num. segments found",
        save.K,
        b"=",
        (STSIZE / 2),
        0,
        OK,
        ctx,
    )?;

    //
    // Reload the earth kernels indexed 1 through FTSIZE to restore
    // the original file priorities.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = (FTSIZE + 2);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKLSF(&save.DSKS[save.I], &mut save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Force construction of a new segment list.
    //
    spicelib::ZZDSKBBL(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Unload the first earth DSK. Load the final earth DSK. Attempt a search.",
        ctx,
    )?;
    //
    // The segment list should run out of room while the front of the
    // list is built. This should force the reconstruction of the list,
    // with eventual failure when the tail of the list cannot be
    // accommodated.
    //
    spicelib::ZZDSKUSF(save.HANDLS[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.K = (2 + save.NEARTH);

    spicelib::ZZDSKLSF(&save.DSKS[save.K], &mut save.HANDLS[save.K], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that no update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.BODYID = 399;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search over all segments. We expect a failure.
    //
    // Use ZZDSKNOT to force a complete search.
    //

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKNOT,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BUFFEROVERFLOW)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Search for one earth segment. It should be located in the last file loaded.",
        ctx,
    )?;

    save.K = (2 + save.NEARTH);

    spicelib::ZZDSKLSF(&save.DSKS[save.K], &mut save.HANDLS[save.K], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.BODYID = 399;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that no update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, false, OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    save.NSEG = STSIZE;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Fetch data to compare.
    //
    spicelib::DLABBS(
        save.HANDLS[save.K],
        save.XDLADS.as_slice_mut(),
        &mut save.FND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLABFS FOUND", save.FND, true, OK, ctx)?;

    //
    // Check the other outputs.
    //
    testutil::CHCKSI(
        b"HANDLE",
        save.HANDLE,
        b"=",
        save.HANDLS[save.K],
        0,
        OK,
        ctx,
    )?;

    spicelib::DSKGD(
        save.HANDLS[save.K],
        save.XDLADS.as_slice(),
        save.XDSKDS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"DLADSC",
        save.DLADSC.as_slice(),
        b"=",
        save.XDLADS.as_slice(),
        DLADSZ,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"DSKDSC",
        save.DSKDSC.as_slice(),
        b"=",
        save.XDSKDS.as_slice(),
        DSKDSZ,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Look up earth segments until we encounter a failure. We don\'t have enough room in the segment table for all earth segments.", ctx)?;

    save.BODYID = 399;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKNOT,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BUFFEROVERFLOW)", OK, ctx)?;

    //
    // Unload the last earth DSK.
    //
    spicelib::ZZDSKUSF(save.HANDLS[(2 + save.NEARTH)], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the first FTSIZE earth DSKs.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = (FTSIZE + 2);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKLSF(&save.DSKS[save.I], &mut save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Force construction of a new segment list.
    //
    spicelib::ZZDSKBBL(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Setup: create a DSK file containing 10 segments for body 699.",
        ctx,
    )?;

    //
    // To even create a new DSK, we need to unload one of them,
    // since we'll open a new DSK. We normally would need to
    // unload two files to accommodate the scratch DAS used for
    // DAS segregation, but T_SMLDSK doesn't segregate its output
    // file.
    //
    spicelib::ZZDSKUSF(save.HANDLS[(FTSIZE + 2)], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.NSAT = 10;

    save.I = (FTSIZE + 3);

    fstr::assign(save.DSKS.get_mut(save.I), b"zzdskbsr_test_#.bds");
    spicelib::REPMI(
        &save.DSKS[save.I].to_vec(),
        b"#",
        save.I,
        &mut save.DSKS[save.I],
        ctx,
    );

    if spicelib::EXISTS(&save.DSKS[save.I], ctx)? {
        spicelib::DELFIL(&save.DSKS[save.I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    save.BODYID = 699;
    fstr::assign(&mut save.FRAME, b"IAU_SATURN");

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NSAT;
        let m3__: i32 = 1;
        save.J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SURFID = save.J;
            testutil::T_SMLDSK(
                save.BODYID,
                save.SURFID,
                &save.FRAME,
                &save.DSKS[save.I],
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.J += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKBSS/ZZDSKSNS: load the Saturn DSK. Look up the Saturn segments. This look-up should cause the 399, 499, and 599 segment lists to be dumped.", ctx)?;

    //
    // Load the first DSK and find data for 499 and 599, so we have
    // segment lists that need to be removed. We have too many files
    // loaded, so unload an earth file first.
    //
    spicelib::ZZDSKUSF(save.HANDLS[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKLSF(&save.DSKS[1], &mut save.HANDLS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKBBL(499, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKBBL(599, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.I = (FTSIZE + 3);

    spicelib::ZZDSKLSF(&save.DSKS[save.I], &mut save.HANDLS[save.I], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the state change status. We expect that an update is
    // indicated.
    //
    spicelib::ZZDSKCHK(save.DSKCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE", save.UPDATE, true, OK, ctx)?;

    save.BODYID = 699;

    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Prepare segment selection test function for use in BSR search.
    //
    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We should find NSAT segments from DSK #(FTSIZE+3).
    //
    testutil::CHCKSL(b"FOUND (1)", save.FOUND, true, OK, ctx)?;
    spicelib::DLABBS(
        save.HANDLS[save.I],
        save.XDLADS.as_slice_mut(),
        &mut save.FND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DLABFS FOUND", save.FND, true, OK, ctx)?;

    save.J = 0;

    while ((save.FOUND && save.FND) && (save.J < save.NSAT)) {
        save.J = (save.J + 1);
        //
        // Check the other outputs.
        //
        testutil::CHCKSI(
            b"HANDLE",
            save.HANDLE,
            b"=",
            save.HANDLS[save.I],
            0,
            OK,
            ctx,
        )?;

        spicelib::DSKGD(
            save.HANDLS[save.I],
            save.XDLADS.as_slice(),
            save.XDSKDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            b"DLADSC",
            save.DLADSC.as_slice(),
            b"=",
            save.XDLADS.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"DSKDSC",
            save.DSKDSC.as_slice(),
            b"=",
            save.XDSKDS.as_slice(),
            DSKDSZ,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Continue to search backwards.
        //
        spicelib::DLAFPS(
            save.HANDLS[save.I],
            save.XDLADS.as_slice(),
            save.PRVDSC.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ZZDSKSNS(
            spicelib::ZZDSKBDC,
            &mut save.HANDLE,
            save.DLADSC.as_slice_mut(),
            save.DSKDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKUSF test: cause 699 body table entry to be deleted, when 699 is not at index NBT",
        ctx,
    )?;

    //
    // To set this up, we'll put the earth body table entry at
    // the end of the table. We need to do an earth lookup to
    // get the earth back into the body table, while avoiding
    // removal of the Saturn list. Make the set of earth segments
    // smaller by unloading some earth files.
    //
    {
        let m1__: i32 = 3;
        let m2__: i32 = 12;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKUSF(save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            save.I += m3__;
        }
    }
    //
    // Now do an earth search.
    //
    save.BODYID = 399;
    spicelib::ZZDSKBSS(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RETVAL = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSNS(
        spicelib::ZZDSKBDC,
        &mut save.HANDLE,
        save.DLADSC.as_slice_mut(),
        save.DSKDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // All of the preceding was to get the earth into the last
    // position of the body table. Now unload the Saturn file.
    //
    save.K = (FTSIZE + 3);

    spicelib::ZZDSKUSF(save.HANDLS[save.K], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKSNS test: search a file that has more than STSIZE segments.",
        ctx,
    )?;

    //
    // Create a monster DSK.
    //
    fstr::assign(&mut save.OVRDSK, b"zzdskbsr_toobig.bds");

    if spicelib::EXISTS(&save.OVRDSK, ctx)? {
        spicelib::DELFIL(&save.OVRDSK, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    save.BODYID = 999;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (STSIZE + 1);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.FRAME, b"IAU_PLUTO");
            save.SURFID = save.I;
            testutil::T_SMLDSK(save.BODYID, save.SURFID, &save.FRAME, &save.OVRDSK, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Try to build a segment list for 999.
    //
    spicelib::ZZDSKLSF(&save.OVRDSK, &mut save.OVRHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKBBL(save.BODYID, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BUFFEROVERFLOW)", OK, ctx)?;

    //
    //     Clean up.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = (FTSIZE + 3);
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZDSKUSF(save.HANDLS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DELFIL(&save.DSKS[save.I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::ZZDSKUSF(save.BTHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(&save.BTDSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKUSF(save.OVRHAN, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(&save.OVRDSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
