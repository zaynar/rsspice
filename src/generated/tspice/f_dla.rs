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
const FTSIZE: i32 = 5000;
const NWD: i32 = 128;
const NWI: i32 = 256;
const NWC: i32 = 1024;
const CHARDT: i32 = 1;
const DPDT: i32 = 2;
const INTDT: i32 = 3;
const DLA0: &[u8] = b"dlatest0.dla";
const DLA1: &[u8] = b"dlatest1.dla";
const CBUFLN: i32 = 80;
const CBUFSZ: i32 = 100;
const IBUFSZ: i32 = 100;
const DBUFSZ: i32 = 100;
const LNSIZE: i32 = 80;

struct SaveVars {
    CDATA: ActualCharArray,
    LABEL: Vec<u8>,
    XDATAC: ActualCharArray,
    DDATA: StackArray<f64, 100>,
    XDATAD: StackArray<f64, 100>,
    DLADSC: StackArray<i32, 8>,
    HAN0: i32,
    HAN1: i32,
    HANDLE: i32,
    IDATA: StackArray<i32, 100>,
    NCADD: i32,
    NCOMCH: i32,
    NFOUND: i32,
    NSEG: i32,
    NXTDSC: StackArray<i32, 8>,
    PRVDSC: StackArray<i32, 8>,
    XBASEC: i32,
    XBASED: i32,
    XBASEI: i32,
    XDATAI: StackArray<i32, 100>,
    XBWD: i32,
    XDSC: StackArray<i32, 8>,
    XFWD: i32,
    XSIZEC: i32,
    XSIZED: i32,
    XSIZEI: i32,
    FOUND: bool,
    ISSAME: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CDATA = ActualCharArray::new(CBUFLN, 1..=CBUFSZ);
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut XDATAC = ActualCharArray::new(CBUFLN, 1..=CBUFSZ);
        let mut DDATA = StackArray::<f64, 100>::new(1..=DBUFSZ);
        let mut XDATAD = StackArray::<f64, 100>::new(1..=DBUFSZ);
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut HAN0: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut IDATA = StackArray::<i32, 100>::new(1..=IBUFSZ);
        let mut NCADD: i32 = 0;
        let mut NCOMCH: i32 = 0;
        let mut NFOUND: i32 = 0;
        let mut NSEG: i32 = 0;
        let mut NXTDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut PRVDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut XBASEC: i32 = 0;
        let mut XBASED: i32 = 0;
        let mut XBASEI: i32 = 0;
        let mut XDATAI = StackArray::<i32, 100>::new(1..=IBUFSZ);
        let mut XBWD: i32 = 0;
        let mut XDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut XFWD: i32 = 0;
        let mut XSIZEC: i32 = 0;
        let mut XSIZED: i32 = 0;
        let mut XSIZEI: i32 = 0;
        let mut FOUND: bool = false;
        let mut ISSAME: bool = false;

        Self {
            CDATA,
            LABEL,
            XDATAC,
            DDATA,
            XDATAD,
            DLADSC,
            HAN0,
            HAN1,
            HANDLE,
            IDATA,
            NCADD,
            NCOMCH,
            NFOUND,
            NSEG,
            NXTDSC,
            PRVDSC,
            XBASEC,
            XBASED,
            XBASEI,
            XDATAI,
            XBWD,
            XDSC,
            XFWD,
            XSIZEC,
            XSIZED,
            XSIZEI,
            FOUND,
            ISSAME,
        }
    }
}

//$Procedure F_DLA ( Test DLA routines )
pub fn F_DLA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Save all variables to avoid stack problems on some
    // platforms.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DLA", ctx)?;

    // *****************************************************************
    //
    // DLAOPN tests
    //
    // *****************************************************************
    //
    // DLAOPN error cases:
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Blank DLA file name", ctx)?;

    if spicelib::EXISTS(DLA0, ctx)? {
        spicelib::DELFIL(DLA0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    save.NCOMCH = 0;

    spicelib::DLAOPN(b" ", b"DSK", DLA0, save.NCOMCH, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BLANKFILENAME)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"File exists", ctx)?;

    save.NCOMCH = 0;

    spicelib::DLAOPN(DLA0, b"DSK", DLA0, save.NCOMCH, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLAOPN(DLA0, b"DSK", DLA0, save.NCOMCH, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILEOPENFAIL)", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DLA0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Blank file type", ctx)?;

    save.NCOMCH = 0;

    spicelib::DLAOPN(DLA0, b" ", DLA0, save.NCOMCH, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BLANKFILETYPE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Non-printing character in file type", ctx)?;

    save.NCOMCH = 0;

    spicelib::DLAOPN(
        DLA0,
        &intrinsics::CHAR(0),
        DLA0,
        save.NCOMCH,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ILLEGALCHARACTER)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Bad comment record count", ctx)?;

    save.NCOMCH = -1;

    spicelib::DLAOPN(DLA0, b"DSK", DLA0, save.NCOMCH, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADRECORDCOUNT)", OK, ctx)?;

    //
    // DLAOPN normal cases:
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Create DLA file without data", ctx)?;

    save.NCOMCH = 0;

    spicelib::DLAOPN(DLA0, b"DSK", DLA0, save.NCOMCH, &mut save.HAN0, ctx)?;

    //
    // Close file and re-open for read access.
    //
    spicelib::DASCLS(save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DLA0, &mut save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check DLA version.
    //
    spicelib::DASRDI(save.HAN0, VERIDX, VERIDX, save.IDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"DLA version", save.IDATA[1], b"=", FMTVER, 0, OK, ctx)?;

    //
    // Check forward and backward pointers.
    //
    spicelib::DASRDI(save.HAN0, LLBIDX, LLBIDX, save.IDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"Begin ptr", save.IDATA[1], b"=", NULPTR, 0, OK, ctx)?;

    spicelib::DASRDI(save.HAN0, LLEIDX, LLEIDX, save.IDATA.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"End ptr", save.IDATA[1], b"=", NULPTR, 0, OK, ctx)?;

    //
    // Leave file open for read access for further tests.
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Create DLA file with 10 segments.", ctx)?;

    //
    // Note that we can't use a DSK for these tests, since DSKs don't
    // contain character data.
    //
    if spicelib::EXISTS(DLA1, ctx)? {
        spicelib::DELFIL(DLA1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Add space for comment characters, just to make sure this
    // doesn't cause problems.
    //
    save.NCOMCH = 4096;

    spicelib::DLAOPN(DLA1, b"DSK", DLA1, save.NCOMCH, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Populate data buffers.
    //
    save.NSEG = 10;

    for I in 1..=save.NSEG {
        spicelib::DLABNS(save.HAN1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=(IBUFSZ - I) {
            save.IDATA[J] = ((1000 * I) + J);
        }

        for J in 1..=(DBUFSZ - I) {
            save.DDATA[J] = (((1000 * I) + J) as f64);
        }

        for J in 1..=(CBUFSZ - I) {
            fstr::assign(save.CDATA.get_mut(J), b"Segment # line #.");

            spicelib::REPMI(&save.CDATA[J].to_vec(), b"#", I, &mut save.CDATA[J], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.CDATA[J].to_vec(), b"#", J, &mut save.CDATA[J], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                fstr::substr_mut(save.CDATA.get_mut(J), (CBUFLN - 2)..),
                b"-->",
            );
        }

        spicelib::DASADI(save.HAN1, (IBUFSZ - I), save.IDATA.as_slice(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DASADD(save.HAN1, (DBUFSZ - I), save.DDATA.as_slice(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.NCADD = ((CBUFSZ - I) * CBUFLN);

        spicelib::DASADC(save.HAN1, save.NCADD, 1, CBUFLN, save.CDATA.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DLAENS(save.HAN1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close file and re-open for read access.
    //
    spicelib::DASCLS(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASOPR(DLA1, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // *****************************************************************
    //
    // DLABFS/DLAFNS tests
    //
    // *****************************************************************
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DLABFS: bad handle", ctx)?;

    spicelib::DLABFS(-999, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DLAFNS: bad handle", ctx)?;

    spicelib::DLAFNS(
        -999,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DLABFS: Start forward search on empty list", ctx)?;

    //
    // DLA0 is assumed to be open at this point.
    //

    spicelib::CLEARI(DLADSZ, save.DLADSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI(DLADSZ, save.XDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(save.HAN0, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::CHCKAI(
        b"DLADSC",
        save.DLADSC.as_slice(),
        b"=",
        save.XDSC.as_slice(),
        DLADSZ,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DLABFS/DLAFNS: perform forward search on 10-segment DLA.",
        ctx,
    )?;

    //
    // DLA1 is assumed to be open at this point.
    //
    spicelib::CLEARI(DLADSZ, save.PRVDSC.as_slice_mut());

    save.NFOUND = 0;

    spicelib::DLABFS(save.HAN1, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    save.XBASEI = 3;
    save.XBASEC = 0;
    save.XBASED = 0;

    while save.FOUND {
        save.NFOUND = (save.NFOUND + 1);
        //
        // Check out the descriptor components.
        //
        // Base addresses precede the first address in use by
        // a DLA segment.
        //
        // The integer base is shifted by the amount of integer
        // data in the previous segment, plus the size of a DLA
        // descriptor, since DLA descriptors reside in DAS integer
        // address space.
        //
        save.XBASEI = ((save.XBASEI + DLADSZ) + save.PRVDSC[ISZIDX]);

        fstr::assign(&mut save.LABEL, b"IBASE segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[IBSIDX],
            b"=",
            save.XBASEI,
            0,
            OK,
            ctx,
        )?;

        //
        // Set the expected size based on the formula used
        // for constructing the file.
        //
        save.XSIZEI = (IBUFSZ - save.NFOUND);

        fstr::assign(&mut save.LABEL, b"ISIZE segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[ISZIDX],
            b"=",
            save.XSIZEI,
            0,
            OK,
            ctx,
        )?;

        //
        // Check d.p. base and size.
        //
        save.XBASED = (save.XBASED + save.PRVDSC[DSZIDX]);

        fstr::assign(&mut save.LABEL, b"DBASE segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[DBSIDX],
            b"=",
            save.XBASED,
            0,
            OK,
            ctx,
        )?;

        //
        // Set the expected size based on the formula used
        // for constructing the file.
        //
        save.XSIZED = (DBUFSZ - save.NFOUND);

        fstr::assign(&mut save.LABEL, b"DSIZE segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[DSZIDX],
            b"=",
            save.XSIZED,
            0,
            OK,
            ctx,
        )?;

        //
        // Check character base and size.
        //
        save.XBASEC = (save.XBASEC + save.PRVDSC[CSZIDX]);

        fstr::assign(&mut save.LABEL, b"CBASE segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[CBSIDX],
            b"=",
            save.XBASEC,
            0,
            OK,
            ctx,
        )?;

        //
        // Set the expected size based on the formula used
        // for constructing the file.
        //
        save.XSIZEC = ((CBUFSZ - save.NFOUND) * CBUFLN);

        fstr::assign(&mut save.LABEL, b"CSIZE segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[CSZIDX],
            b"=",
            save.XSIZEC,
            0,
            OK,
            ctx,
        )?;

        //
        // Check the forward pointer. The pointer should contain
        // the DAS integer address of the first element of
        // the next descriptor, or NULPTR if the current segment
        // is the last.
        //
        if (save.NFOUND < save.NSEG) {
            save.XFWD = ((save.DLADSC[IBSIDX] + save.DLADSC[ISZIDX]) + 1);
        } else {
            save.XFWD = NULPTR;
        }

        fstr::assign(&mut save.LABEL, b"FWD PTR segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[FWDIDX],
            b"=",
            save.XFWD,
            0,
            OK,
            ctx,
        )?;

        //
        // Check the backward pointer. The pointer should contain
        // the DAS integer address of the first element of
        // the previous descriptor, or NULPTR if the current segment
        // is the first.
        //
        if (save.NFOUND > 1) {
            save.XBWD = ((save.PRVDSC[IBSIDX] + 1) - DLADSZ);
        } else {
            save.XBWD = NULPTR;
        }

        fstr::assign(&mut save.LABEL, b"BWD PTR segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.DLADSC[BWDIDX],
            b"=",
            save.XBWD,
            0,
            OK,
            ctx,
        )?;

        //
        // Prepare to check data. Fill buffers with expected
        // values.
        //
        for J in 1..=(IBUFSZ - save.NFOUND) {
            save.XDATAI[J] = ((1000 * save.NFOUND) + J);
        }

        for J in 1..=(DBUFSZ - save.NFOUND) {
            save.XDATAD[J] = (((1000 * save.NFOUND) + J) as f64);
        }

        for J in 1..=(CBUFSZ - save.NFOUND) {
            fstr::assign(save.XDATAC.get_mut(J), b"Segment # line #.");

            spicelib::REPMI(
                &save.XDATAC[J].to_vec(),
                b"#",
                save.NFOUND,
                &mut save.XDATAC[J],
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.XDATAC[J].to_vec(), b"#", J, &mut save.XDATAC[J], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                fstr::substr_mut(save.XDATAC.get_mut(J), (CBUFLN - 2)..),
                b"-->",
            );
        }

        //
        // Check integer data.
        //
        spicelib::DASRDI(
            save.HAN1,
            (save.XBASEI + 1),
            (save.XBASEI + save.XSIZEI),
            save.IDATA.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"IDATA segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.IDATA.as_slice(),
            b"=",
            save.XDATAI.as_slice(),
            save.XSIZEI,
            OK,
            ctx,
        )?;

        //
        // Check d.p. data.
        //
        spicelib::DASRDD(
            save.HAN1,
            (save.XBASED + 1),
            (save.XBASED + save.XSIZED),
            save.DDATA.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"DDATA segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAD(
            &save.LABEL,
            save.DDATA.as_slice(),
            b"=",
            save.XDATAD.as_slice(),
            save.XSIZED,
            0.0,
            OK,
            ctx,
        )?;

        //
        // Check character data.
        //
        spicelib::DASRDC(
            save.HAN1,
            (save.XBASEC + 1),
            (save.XBASEC + save.XSIZEC),
            1,
            CBUFLN,
            save.CDATA.as_arg_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.LABEL, b"CDATA segment #");
        spicelib::REPMI(
            &save.LABEL.to_vec(),
            b"#",
            save.NFOUND,
            &mut save.LABEL,
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // For this check we must convert the character count to
        // an array element count.
        //
        testutil::CHCKAC(
            &save.LABEL,
            save.CDATA.as_arg(),
            b"=",
            save.XDATAC.as_arg(),
            (save.XSIZEC / CBUFLN),
            OK,
            ctx,
        )?;

        //
        // Fetch next segment.
        //
        spicelib::MOVEI(save.DLADSC.as_slice(), DLADSZ, save.PRVDSC.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DLAFNS(
            save.HAN1,
            save.PRVDSC.as_slice(),
            save.DLADSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (save.NFOUND > save.NSEG) {
            //
            // Escape from loop if necessary.
            //
            save.FOUND = false;
        }
    }

    testutil::CHCKSI(b"NFOUND", save.NFOUND, b"=", save.NSEG, 0, OK, ctx)?;

    // *****************************************************************
    //
    // DLABBS/DLAFPS tests
    //
    // *****************************************************************
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DLABBS: bad handle", ctx)?;

    spicelib::DLABBS(-999, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DLAFPS: bad handle", ctx)?;

    spicelib::DLAFPS(
        -999,
        save.DLADSC.as_slice(),
        save.PRVDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DLABBS: Start backward search on empty list", ctx)?;

    spicelib::CLEARI(DLADSZ, save.DLADSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI(DLADSZ, save.XDSC.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABBS(save.HAN0, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::CHCKAI(
        b"DLADSC",
        save.DLADSC.as_slice(),
        b"=",
        save.XDSC.as_slice(),
        DLADSZ,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DLABBS/DLAFPS: perform backward search on 10-segment DLA.",
        ctx,
    )?;

    //
    // DLA1 is assumed to be open at this point.
    //
    save.NFOUND = 0;

    spicelib::DLABBS(save.HAN1, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We're going to assume at this point that the forward
    // search routines are working. We'll use them to produce
    // the expected descriptors.
    //
    for I in 1..=save.NSEG {
        //
        // Produce the expected descriptor for the Ith segment.
        //
        spicelib::DLABFS(save.HAN1, save.XDSC.as_slice_mut(), &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for J in 1..=(I - 1) {
            spicelib::MOVEI(save.XDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::DLAFNS(
                save.HAN1,
                save.DLADSC.as_slice(),
                save.XDSC.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Do a backward search to locate the Ith segment.
        //
        spicelib::DLABBS(save.HAN1, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        for J in 1..=(save.NSEG - I) {
            spicelib::DLAFPS(
                save.HAN1,
                save.DLADSC.as_slice(),
                save.PRVDSC.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Compare descriptors.
        //
        fstr::assign(&mut save.LABEL, b"DLADSC segment #");
        spicelib::REPMI(&save.LABEL.to_vec(), b"#", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.LABEL,
            save.DLADSC.as_slice(),
            b"=",
            save.XDSC.as_slice(),
            DLADSZ,
            OK,
            ctx,
        )?;
    }

    // *****************************************************************
    //
    // DLASSG tests
    //
    // *****************************************************************
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DLASSG: compare DLA descriptors in 10-segment DLA file.",
        ctx,
    )?;

    spicelib::DLABFS(save.HAN1, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    for I in 2..=10 {
        spicelib::DLAFNS(
            save.HAN1,
            save.DLADSC.as_slice(),
            save.NXTDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        save.ISSAME = spicelib::DLASSG(
            save.HAN1,
            save.HAN1,
            save.DLADSC.as_slice(),
            save.NXTDSC.as_slice(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"ISSAME", save.ISSAME, false, OK, ctx)?;

        spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DLASSG: compare matching descriptors and handles.", ctx)?;

    save.ISSAME = spicelib::DLASSG(
        save.HAN1,
        save.HAN1,
        save.DLADSC.as_slice(),
        save.DLADSC.as_slice(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISSAME", save.ISSAME, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DLASSG: compare matching descriptors and different handles.",
        ctx,
    )?;

    save.ISSAME = spicelib::DLASSG(
        save.HAN1,
        (save.HAN1 + 1),
        save.DLADSC.as_slice(),
        save.DLADSC.as_slice(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISSAME", save.ISSAME, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DLASSG: compare matching handles and different descriptors.",
        ctx,
    )?;

    //
    // Make sure that a difference in any descriptor element is
    // detected.
    //
    spicelib::MOVEI(save.DLADSC.as_slice(), DLADSZ, save.NXTDSC.as_slice_mut());

    for I in 1..=DLADSZ {
        //
        // Modify the Ith element of NXTDSC.
        //
        save.NXTDSC[I] = -save.NXTDSC[I];

        save.ISSAME = spicelib::DLASSG(
            save.HAN1,
            save.HAN1,
            save.DLADSC.as_slice(),
            save.NXTDSC.as_slice(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"ISSAME", save.ISSAME, false, OK, ctx)?;
        //
        // Restore original element value.
        //
        save.NXTDSC[I] = -save.NXTDSC[I];
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Clean up DLA files", ctx)?;

    spicelib::DASCLS(save.HAN0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(DLA0, ctx)? {
        spicelib::DELFIL(DLA0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DASCLS(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(DLA1, ctx)? {
        spicelib::DELFIL(DLA1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
