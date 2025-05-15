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
const MAXSRF: i32 = 100;
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
const PCK0: &[u8] = b"sbf_test0.tpc";
const TIGHT: f64 = 0.000000000001;
const STSIZE: i32 = 10000;
const BTSIZE: i32 = 100;
const MNSTSZ: i32 = 6;
const MNBTSZ: i32 = 3;
const MAXDC: i32 = 1;
const MAXIC: i32 = 1;
const NAMLEN: i32 = 32;
const MAXN: i32 = 100;
const FILSIZ: i32 = 255;
const LBLSIZ: i32 = 25;
const NDSKS: i32 = 4;
const NSITES: i32 = 3;

struct SaveVars {
    BTNBOD: i32,
    BTBODY: StackArray<i32, 100>,
    BTSEGP: StackArray<i32, 100>,
    BTSTSZ: StackArray<i32, 100>,
    STHAN: ActualArray<i32>,
    STDSCR: ActualArray2D<f64>,
    STDLAD: ActualArray2D<i32>,
    STFREE: i32,
    STOFF: ActualArray2D<f64>,
    STCTR: ActualArray2D<f64>,
    STRAD: ActualArray<f64>,
    DSKS: ActualCharArray,
    FRAME: Vec<u8>,
    FRAMES: ActualCharArray,
    LABEL: Vec<u8>,
    SEGFRM: Vec<u8>,
    SITFNM: ActualCharArray,
    SITNMS: ActualCharArray,
    TARGET: Vec<u8>,
    TOPFK: ActualCharArray,
    TOPSPK: ActualCharArray,
    ANGLES: StackArray2D<f64, 9>,
    DC: StackArray<f64, 1>,
    DSKDSC: StackArray<f64, 24>,
    ET: f64,
    FIRST: f64,
    LAT: f64,
    LAST: f64,
    LON: f64,
    LT: f64,
    NORMAL: StackArray<f64, 3>,
    POINT: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    SITPOS: StackArray2D<f64, 9>,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    XCTR: StackArray<f64, 3>,
    XDPAR: ActualArray<f64>,
    XDSKDS: StackArray<f64, 24>,
    XFORM: StackArray2D<f64, 9>,
    XOFF: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XRAD: f64,
    AXES: StackArray2D<i32, 9>,
    BIDS: StackArray<i32, 100>,
    BODYID: i32,
    BTCARD: i32,
    CLASS: i32,
    CLSSID: i32,
    DLADSC: StackArray<i32, 8>,
    FIXFID: i32,
    FRMCTR: i32,
    HANDLE: i32,
    HANDLS: StackArray<i32, 100>,
    IC: StackArray<i32, 1>,
    J: i32,
    K: i32,
    MAXBOD: i32,
    NEEDED: i32,
    NSURF: i32,
    P: i32,
    PRVDSC: StackArray<i32, 8>,
    SEGCTR: i32,
    SEGFID: i32,
    SITFID: StackArray<i32, 3>,
    SITIDS: StackArray<i32, 3>,
    SRFLST: StackArray<i32, 100>,
    STCARD: i32,
    SURFID: i32,
    UB: i32,
    XBOD: StackArray<i32, 100>,
    XDLADS: StackArray<i32, 8>,
    XHAN: i32,
    XINTAR: ActualArray<i32>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BTNBOD: i32 = 0;
        let mut BTBODY = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTSEGP = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut BTSTSZ = StackArray::<i32, 100>::new(1..=BTSIZE);
        let mut STHAN = ActualArray::<i32>::new(1..=STSIZE);
        let mut STDSCR = ActualArray2D::<f64>::new(1..=DSKDSZ, 1..=STSIZE);
        let mut STDLAD = ActualArray2D::<i32>::new(1..=DLADSZ, 1..=STSIZE);
        let mut STFREE: i32 = 0;
        let mut STOFF = ActualArray2D::<f64>::new(1..=3, 1..=STSIZE);
        let mut STCTR = ActualArray2D::<f64>::new(1..=3, 1..=STSIZE);
        let mut STRAD = ActualArray::<f64>::new(1..=STSIZE);
        let mut DSKS = ActualCharArray::new(FILSIZ, 1..=MAXN);
        let mut FRAME = vec![b' '; NAMLEN as usize];
        let mut FRAMES = ActualCharArray::new(NAMLEN, 1..=MAXN);
        let mut LABEL = vec![b' '; LBLSIZ as usize];
        let mut SEGFRM = vec![b' '; NAMLEN as usize];
        let mut SITFNM = ActualCharArray::new(NAMLEN, 1..=NSITES);
        let mut SITNMS = ActualCharArray::new(NAMLEN, 1..=NSITES);
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut TOPFK = ActualCharArray::new(FILSIZ, 1..=NSITES);
        let mut TOPSPK = ActualCharArray::new(FILSIZ, 1..=NSITES);
        let mut ANGLES = StackArray2D::<f64, 9>::new(1..=3, 1..=NSITES);
        let mut DC = StackArray::<f64, 1>::new(1..=MAXDC);
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut ET: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut POINT = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SITPOS = StackArray2D::<f64, 9>::new(1..=3, 1..=NSITES);
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XCTR = StackArray::<f64, 3>::new(1..=3);
        let mut XDPAR = ActualArray::<f64>::new(1..=(STSIZE * DSKDSZ));
        let mut XDSKDS = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XOFF = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XRAD: f64 = 0.0;
        let mut AXES = StackArray2D::<i32, 9>::new(1..=3, 1..=NSITES);
        let mut BIDS = StackArray::<i32, 100>::new(1..=MAXN);
        let mut BODYID: i32 = 0;
        let mut BTCARD: i32 = 0;
        let mut CLASS: i32 = 0;
        let mut CLSSID: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut FIXFID: i32 = 0;
        let mut FRMCTR: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HANDLS = StackArray::<i32, 100>::new(1..=MAXN);
        let mut IC = StackArray::<i32, 1>::new(1..=MAXIC);
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut MAXBOD: i32 = 0;
        let mut NEEDED: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut P: i32 = 0;
        let mut PRVDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut SEGCTR: i32 = 0;
        let mut SEGFID: i32 = 0;
        let mut SITFID = StackArray::<i32, 3>::new(1..=NSITES);
        let mut SITIDS = StackArray::<i32, 3>::new(1..=NSITES);
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut STCARD: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut UB: i32 = 0;
        let mut XBOD = StackArray::<i32, 100>::new(1..=MAXN);
        let mut XDLADS = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut XHAN: i32 = 0;
        let mut XINTAR = ActualArray::<i32>::new(1..=(STSIZE * DLADSZ));
        let mut FOUND: bool = false;

        Self {
            BTNBOD,
            BTBODY,
            BTSEGP,
            BTSTSZ,
            STHAN,
            STDSCR,
            STDLAD,
            STFREE,
            STOFF,
            STCTR,
            STRAD,
            DSKS,
            FRAME,
            FRAMES,
            LABEL,
            SEGFRM,
            SITFNM,
            SITNMS,
            TARGET,
            TOPFK,
            TOPSPK,
            ANGLES,
            DC,
            DSKDSC,
            ET,
            FIRST,
            LAT,
            LAST,
            LON,
            LT,
            NORMAL,
            POINT,
            RAYDIR,
            SITPOS,
            TOL,
            VERTEX,
            XCTR,
            XDPAR,
            XDSKDS,
            XFORM,
            XOFF,
            XPT,
            XRAD,
            AXES,
            BIDS,
            BODYID,
            BTCARD,
            CLASS,
            CLSSID,
            DLADSC,
            FIXFID,
            FRMCTR,
            HANDLE,
            HANDLS,
            IC,
            J,
            K,
            MAXBOD,
            NEEDED,
            NSURF,
            P,
            PRVDSC,
            SEGCTR,
            SEGFID,
            SITFID,
            SITIDS,
            SRFLST,
            STCARD,
            SURFID,
            UB,
            XBOD,
            XDLADS,
            XHAN,
            XINTAR,
            FOUND,
        }
    }
}

//$Procedure F_SBF ( SBF tests )
pub fn F_SBF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Body table variables
    // --------------------
    //
    //    BTNBOD  is the number of bodies in the body table.
    //
    //    BTBODY  is an array of body ID codes.
    //
    //    BTSEGP  is an array of pointers (start indices) to entries in
    //            the segment table. The Ith pointer indicates the start
    //            index for entries for the Ith body.
    //
    //    BTSTSZ  is an array of segment table entry counts. The Ith
    //            element of BTSTSZ is the number of entries in the
    //            segment table for the Ith body.
    //
    //

    //
    // Segment table variables
    //

    //
    // Miniature buffer sizes
    //

    //
    // Local Parameters
    //

    // INTEGER               TITLEN
    // PARAMETER           ( TITLEN = 160 )

    //
    // Local Variables
    //

    // CHARACTER*(TITLEN)    TITLE

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SBF", ctx)?;

    //***********************************************************************
    //
    //     Test ZZDSKSBI
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBI initialization call", ctx)?;

    save.MAXBOD = BTSIZE;

    //
    // Give the data structure arrays some initial values, so we
    // can verify the initialization procedure.
    //
    save.BTNBOD = BTSIZE;

    for I in 1..=BTSIZE {
        save.BTBODY[I] = -I;
        save.BTSTSZ[I] = I;
    }

    for I in 1..=STSIZE {
        save.STHAN[I] = -I;

        spicelib::FILLD((I as f64), DSKDSZ, save.STDSCR.subarray_mut([1, I]));
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::FILLI(I, DLADSZ, save.STDLAD.subarray_mut([1, I]));
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::FILLD((I as f64), 3, save.STOFF.subarray_mut([1, I]));
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::FILLD((I as f64), 3, save.STCTR.subarray_mut([1, I]));
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.STRAD[I] = (I as f64);
    }

    save.STFREE = 0;

    //
    // The call:
    //
    spicelib::ZZDSKSBI(
        save.MAXBOD,
        STSIZE,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the body ID table.
    //
    spicelib::CLEARI(BTSIZE, save.XINTAR.as_slice_mut());

    testutil::CHCKAI(
        b"BTBODY",
        save.BTBODY.as_slice(),
        b"=",
        save.XINTAR.as_slice(),
        BTSIZE,
        OK,
        ctx,
    )?;

    //
    // Check the body table count.
    //
    testutil::CHCKSI(b"BTNBOD", save.BTNBOD, b"=", 0, 0, OK, ctx)?;

    //
    // Check the body table segment list sizes.
    //
    testutil::CHCKAI(
        b"BTSTSZ",
        save.BTSTSZ.as_slice(),
        b"=",
        save.XINTAR.as_slice(),
        BTSIZE,
        OK,
        ctx,
    )?;

    //
    // Check the segment table handle array.
    //
    testutil::CHCKAI(
        b"STHAN",
        save.STHAN.as_slice(),
        b"=",
        save.XINTAR.as_slice(),
        STSIZE,
        OK,
        ctx,
    )?;

    //
    // Check the segment table DSK descriptor array.
    //
    testutil::CHCKAD(
        b"STDSCR",
        save.STDSCR.as_slice(),
        b"=",
        save.XDPAR.as_slice(),
        (STSIZE * DSKDSZ),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the segment table DLA descriptor array.
    //
    testutil::CHCKAI(
        b"STDLAD",
        save.STDLAD.as_slice(),
        b"=",
        save.XINTAR.as_slice(),
        (STSIZE * DLADSZ),
        OK,
        ctx,
    )?;

    //
    // Check the segment table free element index. The first
    // free element is at index 1.
    //
    testutil::CHCKSI(b"STFREE", save.STFREE, b"=", 1, 0, OK, ctx)?;

    //
    // Check the segment table bounding sphere center array.
    //
    testutil::CHCKAD(
        b"STCTR",
        save.STCTR.as_slice(),
        b"=",
        save.XDPAR.as_slice(),
        (STSIZE * 3),
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the segment table bounding sphere radius array.
    //
    testutil::CHCKAD(
        b"STRAD",
        save.STRAD.as_slice(),
        b"=",
        save.XDPAR.as_slice(),
        STSIZE,
        0.0,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //     Test ZZDSKSBA
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple case: add data for a body having one segment.", ctx)?;

    fstr::assign(save.DSKS.get_mut(1), b"zzdsksbf_test_1.bds");

    if spicelib::EXISTS(&save.DSKS[1], ctx)? {
        spicelib::DELFIL(&save.DSKS[1], ctx)?;
    }
    //
    // Create a trivial DSK for Mars.
    //
    save.BODYID = 499;
    save.SURFID = 1;
    fstr::assign(&mut save.FRAME, b"IAU_MARS");

    testutil::T_SMLDSK(save.BODYID, save.SURFID, &save.FRAME, &save.DSKS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the DSK.
    //
    spicelib::FURNSH(&save.DSKS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the SBF data structures. Use the miniature buffers.
    //
    spicelib::ZZDSKSBI(
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Add data for Mars to the buffer.
    //
    spicelib::ZZDSKSBA(
        save.BODYID,
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now check the buffer contents.
    //
    // The body table should have just one entry.
    //
    testutil::CHCKSI(b"BTNBOD", save.BTNBOD, b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSI(b"BTBODY(1)", save.BTBODY[1], b"=", 499, 0, OK, ctx)?;

    //
    // The segment table should contain one entry for Mars.
    //
    testutil::CHCKSI(b"BTSTSZ(1)", save.BTSTSZ[1], b"=", 1, 0, OK, ctx)?;

    //
    // The segment table pointer should point to the first
    // element of the segment table.
    //
    testutil::CHCKSI(b"BTSEGP(1)", save.BTSEGP[1], b"=", 1, 0, OK, ctx)?;

    //
    // Get the expected handle and descriptors for the Mars segment.
    //
    spicelib::DASOPR(&save.DSKS[1], &mut save.HANDLS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XHAN = save.HANDLS[1];

    spicelib::DLABFS(save.XHAN, save.XDLADS.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DSKGD(
        save.XHAN,
        save.XDLADS.as_slice(),
        save.XDSKDS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check these items in the buffers.
    //
    testutil::CHCKSI(b"STHAN(1)", save.STHAN[1], b"=", save.XHAN, 0, OK, ctx)?;

    testutil::CHCKAI(
        b"STDLAD(*,1)",
        save.STDLAD.subarray([1, 1]),
        b"=",
        save.XDLADS.as_slice(),
        DLADSZ,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STDSKD(*,1)",
        save.STDSCR.subarray([1, 1]),
        b"=",
        save.XDSKDS.as_slice(),
        DSKDSZ,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the segment table "free" index.
    //
    testutil::CHCKSI(b"STFREE", save.STFREE, b"=", 2, 0, OK, ctx)?;

    //
    // Check the segment frame center offset. The body and frame
    // center coincide.
    //
    spicelib::CLEARD(3, save.XOFF.as_slice_mut());

    testutil::CHCKAD(
        b"STOFF(*,1)",
        save.STOFF.subarray([1, 1]),
        b"=",
        save.XOFF.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Get the expected bounding sphere's center and radius. Check the
    // buffered values.
    //
    spicelib::ZZSEGBOX(
        save.XDSKDS.as_slice(),
        save.XCTR.as_slice_mut(),
        &mut save.XRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"STCTR(*,1)",
        save.STCTR.subarray([1, 1]),
        b"=",
        save.XCTR.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"STRAD(1)", save.STRAD[1], b"=", save.XRAD, 0.0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error case: try to buffer Mars data again, without re-initializing first.",
        ctx,
    )?;

    spicelib::ZZDSKSBA(
        save.BODYID,
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDADD)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Remove the Mars entry from the buffers.", ctx)?;
    //
    // The removal is accomplished by "making room" for the
    // maximum number of segment table entries.
    //
    save.NEEDED = MNSTSZ;

    spicelib::ZZDSKSBR(
        save.NEEDED,
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The body table should be empty.
    //
    testutil::CHCKSI(b"BTNBOD", save.BTNBOD, b"=", 0, 0, OK, ctx)?;

    //
    // Check the segment table "free" index.
    //
    testutil::CHCKSI(b"STFREE", save.STFREE, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create DSKs for Venus, Jupiter and Saturn.", ctx)?;
    //
    // We need to have a DSK for a body other Mars loaded
    // in order to call ZZDSKSBA in the next test.
    //
    //
    // These files will contain two segments each.
    //
    save.BIDS[2] = 299;
    save.BIDS[3] = 599;
    save.BIDS[4] = 699;

    fstr::assign(save.FRAMES.get_mut(2), b"IAU_VENUS");
    fstr::assign(save.FRAMES.get_mut(3), b"IAU_JUPITER");
    fstr::assign(save.FRAMES.get_mut(4), b"IAU_SATURN");

    //
    // We'll use a topocentric frame for one segment for each
    // body; the other segments will use body-centered frames.
    //
    // We'll need a generic PCK:
    //
    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTPCK(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Create the FKs and SPKs needed to support topocentric frames.
    //
    save.SITIDS[1] = 299001;
    save.SITIDS[2] = 599001;
    save.SITIDS[3] = 699001;

    fstr::assign(save.SITNMS.get_mut(1), b"VENUS_SURFACE_SITE");
    fstr::assign(save.SITNMS.get_mut(2), b"JUPITER_SURFACE_SITE");
    fstr::assign(save.SITNMS.get_mut(3), b"SATURN_SURFACE_SITE");

    for I in 1..=NSITES {
        save.LON = ((30.0 + (3 * (I - 1)) as f64) * spicelib::RPD(ctx));
        save.LAT = ((45.0 + (2 * (I - 1)) as f64) * spicelib::RPD(ctx));

        spicelib::SRFREC(
            save.BIDS[(I + 1)],
            save.LON,
            save.LAT,
            save.SITPOS.subarray_mut([1, I]),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    fstr::assign(save.SITFNM.get_mut(1), b"VENUS_TOPO");
    fstr::assign(save.SITFNM.get_mut(2), b"JUPITER_TOPO");
    fstr::assign(save.SITFNM.get_mut(3), b"SATURN_TOPO");

    for I in 1..=NSITES {
        save.SITFID[I] = save.SITIDS[I];
    }

    save.FIRST = -((100 as f64) * spicelib::JYEAR());
    save.LAST = ((100 as f64) * spicelib::JYEAR());

    for I in 1..=NSITES {
        save.AXES[[1, I]] = 3;
        save.AXES[[2, I]] = 2;
        save.AXES[[3, I]] = 3;

        save.LON = ((30.0 + (3 * (I - 1)) as f64) * spicelib::RPD(ctx));
        save.LAT = ((45.0 + (2 * (I - 1)) as f64) * spicelib::RPD(ctx));

        save.ANGLES[[1, I]] = -save.LON;
        save.ANGLES[[2, I]] = (save.LAT - (spicelib::PI(ctx) / 2 as f64));
        save.ANGLES[[3, I]] = spicelib::PI(ctx);
    }

    //
    // Create topocentric kernels. We'll need to create
    // separate kernels for the different target bodies.
    //
    for I in 1..=3 {
        fstr::assign(save.TOPFK.get_mut(I), b"sbf_test_topo_#.tf");
        spicelib::REPMI(&save.TOPFK[I].to_vec(), b"#", I, &mut save.TOPFK[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(save.TOPSPK.get_mut(I), b"sbf_test_topo_#.bsp");
        spicelib::REPMI(&save.TOPSPK[I].to_vec(), b"#", I, &mut save.TOPSPK[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::BODC2N(save.BIDS[(I + 1)], &mut save.TARGET, &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"body name found", save.FOUND, true, OK, ctx)?;

        if spicelib::EXISTS(&save.TOPFK[I], ctx)? {
            spicelib::DELFIL(&save.TOPFK[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        if spicelib::EXISTS(&save.TOPSPK[I], ctx)? {
            spicelib::DELFIL(&save.TOPSPK[I], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        testutil::T_TOPKER(
            &save.TOPFK[I],
            &save.TOPSPK[I],
            &save.TARGET,
            &save.FRAMES[(I + 1)],
            1,
            save.SITIDS.subarray(I),
            save.SITNMS.subarray(I),
            save.SITPOS.subarray([1, I]),
            save.SITFNM.subarray(I),
            save.SITFID.subarray(I),
            save.FIRST,
            save.LAST,
            save.AXES.subarray([1, I]),
            save.ANGLES.subarray([1, I]),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Load the kernels.
        //
        spicelib::FURNSH(&save.TOPFK[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::FURNSH(&save.TOPSPK[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Now create the DSK files.
    //
    for I in 2..=4 {
        fstr::assign(save.DSKS.get_mut(I), b"zzdsksbf_test_#.bds");
        spicelib::REPMI(&save.DSKS[I].to_vec(), b"#", I, &mut save.DSKS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if spicelib::EXISTS(&save.DSKS[I], ctx)? {
            spicelib::DELFIL(&save.DSKS[I], ctx)?;
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.SURFID = save.J;
                save.BODYID = save.BIDS[I];

                if (save.J == 1) {
                    fstr::assign(&mut save.FRAME, save.FRAMES.get(I));
                } else {
                    fstr::assign(&mut save.FRAME, save.SITFNM.get((I - 1)));
                }

                testutil::T_SMLDSK(save.BODYID, save.SURFID, &save.FRAME, &save.DSKS[I], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unload the Mars DSK. Try to populate buffers. This is not an error, but there should be no Mars data in the buffer.", ctx)?;

    spicelib::UNLOAD(&save.DSKS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load a Jupiter DSK so the BSR subsystem will agree to function.
    //
    spicelib::FURNSH(&save.DSKS[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSBA(
        save.BODYID,
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The body table should contain an entry for Mars, but
    // there should be no segment data.
    //
    testutil::CHCKSI(b"BTNBOD", save.BTNBOD, b"=", 1, 0, OK, ctx)?;

    testutil::CHCKSI(b"BTSTSZ", save.BTSTSZ[1], b"=", 0, 0, OK, ctx)?;

    //
    // Check the segment table "free" index.
    //
    testutil::CHCKSI(b"STFREE", save.STFREE, b"=", 1, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Re-initialize, then Buffer data for Venus, Jupiter, and Saturn.",
        ctx,
    )?;

    //
    // Load kernels.
    //
    for I in 2..=4 {
        spicelib::FURNSH(&save.DSKS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Reinitialize the buffers.
    //
    save.BIDS[2] = 299;
    save.BIDS[3] = 599;
    save.BIDS[4] = 699;

    spicelib::ZZDSKSBI(
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 2..=4 {
        spicelib::ZZDSKSBA(
            save.BIDS[I],
            MNBTSZ,
            MNSTSZ,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // The body table should have three entries.
    //
    testutil::CHCKSI(b"BTNBOD", save.BTNBOD, b"=", 3, 0, OK, ctx)?;

    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"BTBODY(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            &save.LABEL,
            save.BTBODY[I],
            b"=",
            save.BIDS[(I + 1)],
            0,
            OK,
            ctx,
        )?;
    }

    //
    // The segment table should contain two entries for each body.
    //
    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"BTSTSZ(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.BTSTSZ[I], b"=", 2, 0, OK, ctx)?;
    }

    //
    // The segment table pointers should point to the successive
    // odd-indexed elements of the table.
    //
    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"BTSEGP(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"BTSEGP", save.BTSEGP[I], b"=", ((2 * I) - 1), 0, OK, ctx)?;
    }

    //
    // Check the segment table.
    //
    for I in 1..=3 {
        //
        // Check segment data for body I. Open the DSK for this body.
        //
        save.K = (I + 1);

        spicelib::DASOPR(&save.DSKS[save.K], &mut save.HANDLS[save.K], ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Check the entry for the Jth segment.
                //
                // Get the expected DLA descriptor from the DSK file.
                //
                if (save.J == 1) {
                    spicelib::DLABBS(
                        save.HANDLS[save.K],
                        save.XDLADS.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                } else {
                    spicelib::DLAFPS(
                        save.HANDLS[save.K],
                        save.XDLADS.as_slice(),
                        save.PRVDSC.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                    spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
                }

                //
                // Let P be the start index of the current entry in the
                // segment table.
                //
                save.P = ((save.BTSEGP[I] - 1) + save.J);

                //
                // Check the file handle in the segment table.
                //
                testutil::CHCKSI(
                    b"HANDLE",
                    save.STHAN[save.P],
                    b"=",
                    save.HANDLS[save.K],
                    0,
                    OK,
                    ctx,
                )?;

                //
                // Get the expected DSK descriptor.
                //
                spicelib::DSKGD(
                    save.HANDLS[save.K],
                    save.XDLADS.as_slice(),
                    save.XDSKDS.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the buffered DLA descriptor for the current segment.
                //

                testutil::CHCKAI(
                    b"DLADSC",
                    save.STDLAD.subarray([1, save.P]),
                    b"=",
                    save.XDLADS.as_slice(),
                    DLADSZ,
                    OK,
                    ctx,
                )?;

                //
                // Check the buffered DSK descriptor for the current segment.
                //
                testutil::CHCKAD(
                    b"DSKDSC",
                    save.STDSCR.subarray([1, save.P]),
                    b"=",
                    save.XDSKDS.as_slice(),
                    DSKDSZ,
                    0.0,
                    OK,
                    ctx,
                )?;

                //
                // Check the segment frame center offset.
                //
                if (save.J == 1) {
                    //
                    // The segment uses a topocentric frame. We must
                    // convert the site position into this frame in
                    // order to generate the buffered vector.
                    //
                    // The rotation between the frames is time-independent.
                    //
                    spicelib::PXFORM(
                        &save.FRAMES[save.K],
                        &save.SITFNM[I],
                        0.0,
                        save.XFORM.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::MXV(
                        save.XFORM.as_slice(),
                        save.SITPOS.subarray([1, I]),
                        save.XOFF.as_slice_mut(),
                    );
                } else {
                    //
                    // The body and frame center coincide.
                    //
                    spicelib::CLEARD(3, save.XOFF.as_slice_mut());
                }

                save.TOL = TIGHT;

                testutil::CHCKAD(
                    b"STOFF(*,P)",
                    save.STOFF.subarray([1, save.P]),
                    b"~~/",
                    save.XOFF.as_slice(),
                    3,
                    save.TOL,
                    OK,
                    ctx,
                )?;

                //
                // Get the expected bounding sphere's center and radius. Check
                // the buffered values.
                //
                spicelib::ZZSEGBOX(
                    save.XDSKDS.as_slice(),
                    save.XCTR.as_slice_mut(),
                    &mut save.XRAD,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"STCTR(*,P)",
                    save.STCTR.subarray([1, save.P]),
                    b"=",
                    save.XCTR.as_slice(),
                    3,
                    0.0,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(
                    b"STRAD(P)",
                    save.STRAD[save.P],
                    b"=",
                    save.XRAD,
                    0.0,
                    OK,
                    ctx,
                )?;
                save.J += m3__;
            }
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Buffer data for Mars. This should cause data for Venus to be deleted.",
        ctx,
    )?;

    spicelib::FURNSH(&save.DSKS[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BIDS[1] = 499;

    spicelib::ZZDSKSBA(
        save.BIDS[1],
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The body table should have three entries.
    //
    testutil::CHCKSI(b"BTNBOD", save.BTNBOD, b"=", 3, 0, OK, ctx)?;

    //
    // Check the body IDs in the body table.
    //
    save.XBOD[1] = 599;
    save.XBOD[2] = 699;
    save.XBOD[3] = 499;

    for I in 1..=save.BTNBOD {
        fstr::assign(&mut save.LABEL, b"BTBODY(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.BTBODY[I], b"=", save.XBOD[I], 0, OK, ctx)?;
    }

    //
    // The segment table should contain two entries for each body
    // except for Mars.
    //
    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"BTSTSZ(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (I == 3) {
            save.J = 1;
        } else {
            save.J = 2;
        }

        testutil::CHCKSI(&save.LABEL, save.BTSTSZ[I], b"=", save.J, 0, OK, ctx)?;
    }

    //
    // The segment table pointers should point to the successive
    // odd-indexed elements of the table.
    //
    for I in 1..=3 {
        fstr::assign(&mut save.LABEL, b"BTSEGP(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"BTSEGP", save.BTSEGP[I], b"=", ((2 * I) - 1), 0, OK, ctx)?;
    }

    //
    // Check the segment table.
    //
    for I in 1..=3 {
        //
        // Check segment data for body I. Open the DSK for this body.
        //
        if (I == 3) {
            save.K = 1;
        } else {
            save.K = (I + 2);
        }

        spicelib::DASOPR(&save.DSKS[save.K], &mut save.HANDLS[save.K], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Let UB be the upper bound of the segment loop.
        //
        if (I == 3) {
            save.UB = 1;
        } else {
            save.UB = 2;
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.UB;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Check the entry for the Jth segment.
                //
                // Get the expected DLA descriptor from the DSK file.
                //
                if (save.J == 1) {
                    spicelib::DLABBS(
                        save.HANDLS[save.K],
                        save.XDLADS.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                } else {
                    spicelib::DLAFPS(
                        save.HANDLS[save.K],
                        save.XDLADS.as_slice(),
                        save.PRVDSC.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                    spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
                }

                //
                // Let P be the start index of the current entry in the
                // segment table.
                //
                save.P = ((save.BTSEGP[I] - 1) + save.J);

                //
                // Check the file handle in the segment table.
                //
                testutil::CHCKSI(
                    b"HANDLE",
                    save.STHAN[save.P],
                    b"=",
                    save.HANDLS[save.K],
                    0,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    ctx.stop()?;
                }

                //
                // Get the expected DSK descriptor.
                //
                spicelib::DSKGD(
                    save.HANDLS[save.K],
                    save.XDLADS.as_slice(),
                    save.XDSKDS.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the buffered DLA descriptor for the current segment.
                //

                testutil::CHCKAI(
                    b"DLADSC",
                    save.STDLAD.subarray([1, save.P]),
                    b"=",
                    save.XDLADS.as_slice(),
                    DLADSZ,
                    OK,
                    ctx,
                )?;

                //
                // Check the buffered DSK descriptor for the current segment.
                //
                testutil::CHCKAD(
                    b"DSKDSC",
                    save.STDSCR.subarray([1, save.P]),
                    b"=",
                    save.XDSKDS.as_slice(),
                    DSKDSZ,
                    0.0,
                    OK,
                    ctx,
                )?;

                //
                // Check the segment frame center offset. Get the expected
                // offset first.
                //
                save.SEGCTR = intrinsics::IDNINT(save.XDSKDS[CTRIDX]);
                save.SEGFID = intrinsics::IDNINT(save.XDSKDS[FRMIDX]);

                spicelib::FRINFO(
                    save.SEGFID,
                    &mut save.FRMCTR,
                    &mut save.CLASS,
                    &mut save.CLSSID,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"info found", save.FOUND, true, OK, ctx)?;

                if (save.FRMCTR != save.SEGCTR) {
                    spicelib::FRMNAM(save.SEGFID, &mut save.SEGFRM, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSC(b"SEGFRM", &save.SEGFRM, b"!=", b" ", OK, ctx)?;

                    spicelib::SPKGPS(
                        save.FRMCTR,
                        0.0,
                        &save.SEGFRM,
                        save.SEGCTR,
                        save.XOFF.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;

                    testutil::CHCKAD(
                        b"STOFF(*,P)",
                        save.STOFF.subarray([1, save.P]),
                        b"=",
                        save.XOFF.as_slice(),
                        3,
                        0.0,
                        OK,
                        ctx,
                    )?;
                } else {
                    //
                    // The frame center and segment's central body match.
                    //
                    spicelib::CLEARD(3, save.XOFF.as_slice_mut());
                }

                //
                // Get the expected bounding sphere's center and radius. Check
                // the buffered values.
                //
                spicelib::ZZSEGBOX(
                    save.XDSKDS.as_slice(),
                    save.XCTR.as_slice_mut(),
                    &mut save.XRAD,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"STCTR(*,P)",
                    save.STCTR.subarray([1, save.P]),
                    b"=",
                    save.XCTR.as_slice(),
                    3,
                    0.0,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(
                    b"STRAD(P)",
                    save.STRAD[save.P],
                    b"=",
                    save.XRAD,
                    0.0,
                    OK,
                    ctx,
                )?;
                save.J += m3__;
            }
        }
    }

    //
    // Close files opened by DASOPR.
    //
    for I in 1..=NDSKS {
        spicelib::DASCLS(save.HANDLS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //*********************************************************************
    //*
    //*    ZZDSKSBA Error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBA: insufficient room in segment table.", ctx)?;

    spicelib::ZZDSKSBI(
        MNBTSZ,
        1,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BIDS[1] = 599;

    spicelib::ZZDSKSBA(
        save.BIDS[1],
        MNBTSZ,
        1,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SEGMENTTABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBA: missing SPK data; cannot create offsets.", ctx)?;

    spicelib::UNLOAD(&save.TOPSPK[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSBI(
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BIDS[1] = 299;

    spicelib::ZZDSKSBA(
        save.BIDS[1],
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // Restore SPK file.
    //
    spicelib::FURNSH(&save.TOPSPK[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBA: missing FK data; cannot create offsets.", ctx)?;

    spicelib::UNLOAD(&save.TOPFK[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unloading the file has the side effect of unloading the
    // test PCK as well. Restore this file.
    //
    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSBI(
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BIDS[1] = 299;

    spicelib::ZZDSKSBA(
        save.BIDS[1],
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOFRAMEINFO)", OK, ctx)?;

    //
    // Restore FK file.
    //
    spicelib::FURNSH(&save.TOPFK[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBA: missing PCK data; cannot create offsets.", ctx)?;

    spicelib::UNLOAD(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Unloading the file has the side effect of unloading the
    // FK as well. Restore this file.
    //
    spicelib::FURNSH(&save.TOPFK[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZDSKSBI(
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BIDS[1] = 299;

    spicelib::ZZDSKSBA(
        save.BIDS[1],
        MNBTSZ,
        MNSTSZ,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // Restore FK file.
    //
    spicelib::FURNSH(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //***********************************************************************
    //
    //     Test ZZDSKSBR
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     Normal cases
    //
    //***********************************************************************

    testutil::TCASE(b"Store 7 segments; make room for 2.", ctx)?;

    //
    // Re-initialize.
    //
    save.BTCARD = 4;
    save.STCARD = 7;

    spicelib::ZZDSKSBI(
        save.BTCARD,
        save.STCARD,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Insert segment data for bodies 499, 299, 599, and 699.
    //
    save.BIDS[1] = 499;
    save.BIDS[2] = 299;
    save.BIDS[3] = 599;
    save.BIDS[4] = 699;

    for I in 1..=NDSKS {
        spicelib::ZZDSKSBA(
            save.BIDS[I],
            save.BTCARD,
            save.STCARD,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Make room for 2 segments.
    //
    save.NEEDED = 2;

    spicelib::ZZDSKSBR(
        save.NEEDED,
        save.BTCARD,
        save.STCARD,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The tables should contain data only for 599 and 699.
    //

    //
    // The body table should have two entries.
    //
    testutil::CHCKSI(b"BTNBOD", save.BTNBOD, b"=", 2, 0, OK, ctx)?;

    //
    // Check the body IDs in the body table.
    //
    save.XBOD[1] = 599;
    save.XBOD[2] = 699;

    for I in 1..=save.BTNBOD {
        fstr::assign(&mut save.LABEL, b"BTBODY(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.BTBODY[I], b"=", save.XBOD[I], 0, OK, ctx)?;
    }

    //
    // The segment table should contain two entries for each body.
    //
    for I in 1..=2 {
        fstr::assign(&mut save.LABEL, b"BTSTSZ(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(&save.LABEL, save.BTSTSZ[I], b"=", 2, 0, OK, ctx)?;
    }

    //
    // The segment table pointers should point to the successive
    // odd-indexed elements of the table.
    //
    for I in 1..=2 {
        fstr::assign(&mut save.LABEL, b"BTSEGP(@)");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"BTSEGP", save.BTSEGP[I], b"=", ((2 * I) - 1), 0, OK, ctx)?;
    }

    //
    // Check the segment table.
    //
    for I in 1..=save.BTNBOD {
        //
        // Check segment data for body I. Open the DSK for this body.
        //
        save.K = (I + 2);

        spicelib::DASOPR(&save.DSKS[save.K], &mut save.HANDLS[save.K], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Check the entry for the Jth segment.
                //
                // Get the expected DLA descriptor from the DSK file.
                //
                if (save.J == 1) {
                    spicelib::DLABBS(
                        save.HANDLS[save.K],
                        save.XDLADS.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
                } else {
                    spicelib::DLAFPS(
                        save.HANDLS[save.K],
                        save.XDLADS.as_slice(),
                        save.PRVDSC.as_slice_mut(),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                    spicelib::MOVEI(save.PRVDSC.as_slice(), DLADSZ, save.XDLADS.as_slice_mut());
                }

                //
                // Let P be the start index of the current entry in the
                // segment table.
                //
                save.P = ((save.BTSEGP[I] - 1) + save.J);

                //
                // Check the file handle in the segment table.
                //
                testutil::CHCKSI(
                    b"HANDLE",
                    save.STHAN[save.P],
                    b"=",
                    save.HANDLS[save.K],
                    0,
                    OK,
                    ctx,
                )?;

                if !*OK {
                    ctx.stop()?;
                }

                //
                // Get the expected DSK descriptor.
                //
                spicelib::DSKGD(
                    save.HANDLS[save.K],
                    save.XDLADS.as_slice(),
                    save.XDSKDS.as_slice_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Check the buffered DLA descriptor for the current segment.
                //

                testutil::CHCKAI(
                    b"DLADSC",
                    save.STDLAD.subarray([1, save.P]),
                    b"=",
                    save.XDLADS.as_slice(),
                    DLADSZ,
                    OK,
                    ctx,
                )?;

                //
                // Check the buffered DSK descriptor for the current segment.
                //
                testutil::CHCKAD(
                    b"DSKDSC",
                    save.STDSCR.subarray([1, save.P]),
                    b"=",
                    save.XDSKDS.as_slice(),
                    DSKDSZ,
                    0.0,
                    OK,
                    ctx,
                )?;

                //
                // Check the segment frame center offset. Get the expected
                // offset first.
                //
                save.SEGCTR = intrinsics::IDNINT(save.XDSKDS[CTRIDX]);
                save.SEGFID = intrinsics::IDNINT(save.XDSKDS[FRMIDX]);

                spicelib::FRINFO(
                    save.SEGFID,
                    &mut save.FRMCTR,
                    &mut save.CLASS,
                    &mut save.CLSSID,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(b"info found", save.FOUND, true, OK, ctx)?;

                if (save.FRMCTR != save.SEGCTR) {
                    spicelib::FRMNAM(save.SEGFID, &mut save.SEGFRM, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSC(b"SEGFRM", &save.SEGFRM, b"!=", b" ", OK, ctx)?;

                    spicelib::SPKGPS(
                        save.FRMCTR,
                        0.0,
                        &save.SEGFRM,
                        save.SEGCTR,
                        save.XOFF.as_slice_mut(),
                        &mut save.LT,
                        ctx,
                    )?;

                    testutil::CHCKAD(
                        b"STOFF(*,P)",
                        save.STOFF.subarray([1, save.P]),
                        b"=",
                        save.XOFF.as_slice(),
                        3,
                        0.0,
                        OK,
                        ctx,
                    )?;
                } else {
                    //
                    // The frame center and segment's central body match.
                    //
                    spicelib::CLEARD(3, save.XOFF.as_slice_mut());
                }

                //
                // Check the segment frame center offset. The body and frame
                // center coincide.
                //
                testutil::CHCKAD(
                    b"STOFF(*,P)",
                    save.STOFF.subarray([1, save.P]),
                    b"=",
                    save.XOFF.as_slice(),
                    3,
                    0.0,
                    OK,
                    ctx,
                )?;

                //
                // Get the expected bounding sphere's center and radius. Check
                // the buffered values.
                //
                spicelib::ZZSEGBOX(
                    save.XDSKDS.as_slice(),
                    save.XCTR.as_slice_mut(),
                    &mut save.XRAD,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKAD(
                    b"STCTR(*,P)",
                    save.STCTR.subarray([1, save.P]),
                    b"=",
                    save.XCTR.as_slice(),
                    3,
                    0.0,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSD(
                    b"STRAD(P)",
                    save.STRAD[save.P],
                    b"=",
                    save.XRAD,
                    0.0,
                    OK,
                    ctx,
                )?;
                save.J += m3__;
            }
        }
    }

    //
    // Close files opened by DASOPR.
    //
    for I in 1..=NDSKS {
        spicelib::DASCLS(save.HANDLS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //***********************************************************************
    //
    //     ZZDSKSBR error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Ask for more room than the segment table has.", ctx)?;

    save.NEEDED = (save.STCARD + 1);

    spicelib::ZZDSKSBR(
        save.NEEDED,
        save.BTCARD,
        save.STCARD,
        save.BTBODY.as_slice_mut(),
        &mut save.BTNBOD,
        save.BTSEGP.as_slice_mut(),
        save.BTSTSZ.as_slice_mut(),
        save.STHAN.as_slice_mut(),
        save.STDSCR.as_slice_mut(),
        save.STDLAD.as_slice_mut(),
        &mut save.STFREE,
        save.STOFF.as_slice_mut(),
        save.STCTR.as_slice_mut(),
        save.STRAD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SEGTABLETOOSMALL)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    ZZDSKSBF Error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Direct ZZDSKSBF call.", ctx)?;

    spicelib::ZZDSKSBF(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.POINT.as_slice(),
        save.XPT.as_slice(),
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        save.DC.as_slice(),
        save.IC.as_slice(),
        save.FOUND,
        save.NORMAL.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Clean up
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NDSKS {
        spicelib::DASCLS(save.HANDLS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(&save.DSKS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=3 {
        spicelib::DELFIL(&save.TOPFK[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(&save.TOPSPK[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
