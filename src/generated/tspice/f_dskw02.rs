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
const IXNV: i32 = 1;
const IXNP: i32 = (IXNV + 1);
const IXNVXT: i32 = (IXNP + 1);
const IXVGRX: i32 = (IXNVXT + 1);
const IXCGSC: i32 = (IXVGRX + 3);
const IXVXPS: i32 = (IXCGSC + 1);
const IXVXLS: i32 = (IXVXPS + 1);
const IXVTLS: i32 = (IXVXLS + 1);
const IXPLAT: i32 = (IXVTLS + 1);
const IXDSCR: i32 = 1;
const DSCSZ2: i32 = 24;
const IXVTBD: i32 = (IXDSCR + DSCSZ2);
const IXVXOR: i32 = (IXVTBD + 6);
const IXVXSZ: i32 = (IXVXOR + 3);
const IXVERT: i32 = (IXVXSZ + 1);
const KWNV: i32 = 1;
const KWNP: i32 = (KWNV + 1);
const KWNVXT: i32 = (KWNP + 1);
const KWVGRX: i32 = (KWNVXT + 1);
const KWCGSC: i32 = (KWVGRX + 1);
const KWVXPS: i32 = (KWCGSC + 1);
const KWVXLS: i32 = (KWVXPS + 1);
const KWVTLS: i32 = (KWVXLS + 1);
const KWPLAT: i32 = (KWVTLS + 1);
const KWVXPT: i32 = (KWPLAT + 1);
const KWVXPL: i32 = (KWVXPT + 1);
const KWVTPT: i32 = (KWVXPL + 1);
const KWVTPL: i32 = (KWVTPT + 1);
const KWCGPT: i32 = (KWVTPL + 1);
const KWDSC: i32 = (KWCGPT + 1);
const KWVTBD: i32 = (KWDSC + 1);
const KWVXOR: i32 = (KWVTBD + 1);
const KWVXSZ: i32 = (KWVXOR + 1);
const KWVERT: i32 = (KWVXSZ + 1);
const MAXVRT: i32 = 16000002;
const MAXPLT: i32 = (2 * (MAXVRT - 2));
const MAXNPV: i32 = (((3 * MAXPLT) / 2) + 1);
const MAXVOX: i32 = 100000000;
const MAXCGR: i32 = 100000;
const MAXEDG: i32 = 120;
const SIVGRX: i32 = 1;
const SICGSC: i32 = (SIVGRX + 3);
const SIVXNP: i32 = (SICGSC + 1);
const SIVXNL: i32 = (SIVXNP + 1);
const SIVTNL: i32 = (SIVXNL + 1);
const SICGRD: i32 = (SIVTNL + 1);
const IXIFIX: i32 = (MAXCGR + 7);
const SIVTBD: i32 = 1;
const SIVXOR: i32 = (SIVTBD + 6);
const SIVXSZ: i32 = (SIVXOR + 3);
const IXDFIX: i32 = 10;
const MAXVXP: i32 = (MAXPLT / 2);
const MAXCEL: i32 = 60000000;
const MXNVLS: i32 = (MAXCEL + (MAXVXP / 2));
const SPAISZ: i32 = ((((IXIFIX + MAXVXP) + MXNVLS) + MAXVRT) + MAXNPV);
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
const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);
const DSK0: &[u8] = b"dskw02_test0.bds";
const DSK1: &[u8] = b"dskw02_test1.bds";
const DSK2: &[u8] = b"dskw02_test2.bds";
const DSK3: &[u8] = b"dskw02_test3.bds";
const VTIGHT: f64 = 0.00000000000001;
const DTYPE: i32 = 2;
const FRNMLN: i32 = 32;
const IBUFSZ: i32 = MAXCGR;
const MAXP: i32 = 20000;
const MAXV: i32 = 10000;
const VOXPSZ: i32 = 100000;
const VOXNPL: i32 = 200000;
const MXIXSZ: i32 = 1000000;
const WORKSZ: i32 = 1000000;
const DBUFSZ: i32 = (3 * MAXV);

struct SaveVars {
    FRAME: Vec<u8>,
    A: f64,
    ALTLIM: f64,
    B: f64,
    C: f64,
    CORPAR: StackArray<f64, 10>,
    DBUFF: ActualArray<f64>,
    DELTA: f64,
    DSKDSC: StackArray<f64, 24>,
    F: f64,
    FINSCL: f64,
    FIRST: f64,
    LAST: f64,
    MAXLON: f64,
    MINLON: f64,
    MN32: f64,
    MN33: f64,
    MNCOR1: f64,
    MNCOR2: f64,
    MNCOR3: f64,
    MX32: f64,
    MX33: f64,
    MXCOR1: f64,
    MXCOR2: f64,
    MXCOR3: f64,
    RE: f64,
    RP: f64,
    SPAIXD: StackArray<f64, 10>,
    SPAXD2: StackArray<f64, 10>,
    SPAXD3: StackArray<f64, 10>,
    TOL: f64,
    VRTCES: ActualArray<f64>,
    VRTCS2: ActualArray<f64>,
    VRTCS3: ActualArray<f64>,
    X: f64,
    XFORM: StackArray2D<f64, 9>,
    ADDR: i32,
    BODYID: i32,
    CORSCL: i32,
    CORSYS: i32,
    DCLASS: i32,
    DLADS2: StackArray<i32, 8>,
    DLADSC: StackArray<i32, 8>,
    FRAMID: i32,
    HANDLE: i32,
    HAN1: i32,
    HAN2: i32,
    HAN3: i32,
    I: i32,
    IBUFF: ActualArray<i32>,
    J: i32,
    K: i32,
    N: i32,
    NLAT: i32,
    NLON: i32,
    NP: i32,
    NP2: i32,
    NV: i32,
    NV2: i32,
    NVXTOT: i32,
    NXTDSC: StackArray<i32, 8>,
    PLATES: ActualArray<i32>,
    PLATS2: ActualArray<i32>,
    SPAIXI: ActualArray<i32>,
    SPAXI2: ActualArray<i32>,
    SPAXI3: ActualArray<i32>,
    SPXISZ: i32,
    SURFID: i32,
    SURF2: i32,
    VGREXT: StackArray<i32, 3>,
    VPSIZ3: i32,
    VPSIZE: i32,
    VLSIZE: i32,
    VLSIZ3: i32,
    VTXLSZ: i32,
    W: i32,
    WORK: ActualArray2D<i32>,
    XNCGR: i32,
    XNP: i32,
    XNV: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut A: f64 = 0.0;
        let mut ALTLIM: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DBUFF = ActualArray::<f64>::new(1..=DBUFSZ);
        let mut DELTA: f64 = 0.0;
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut F: f64 = 0.0;
        let mut FINSCL: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
        let mut MAXLON: f64 = 0.0;
        let mut MINLON: f64 = 0.0;
        let mut MN32: f64 = 0.0;
        let mut MN33: f64 = 0.0;
        let mut MNCOR1: f64 = 0.0;
        let mut MNCOR2: f64 = 0.0;
        let mut MNCOR3: f64 = 0.0;
        let mut MX32: f64 = 0.0;
        let mut MX33: f64 = 0.0;
        let mut MXCOR1: f64 = 0.0;
        let mut MXCOR2: f64 = 0.0;
        let mut MXCOR3: f64 = 0.0;
        let mut RE: f64 = 0.0;
        let mut RP: f64 = 0.0;
        let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut SPAXD2 = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut SPAXD3 = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut TOL: f64 = 0.0;
        let mut VRTCES = ActualArray::<f64>::new(1..=(3 * MAXV));
        let mut VRTCS2 = ActualArray::<f64>::new(1..=(3 * MAXV));
        let mut VRTCS3 = ActualArray::<f64>::new(1..=(3 * MAXV));
        let mut X: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut ADDR: i32 = 0;
        let mut BODYID: i32 = 0;
        let mut CORSCL: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut DCLASS: i32 = 0;
        let mut DLADS2 = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut FRAMID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HAN1: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut HAN3: i32 = 0;
        let mut I: i32 = 0;
        let mut IBUFF = ActualArray::<i32>::new(1..=IBUFSZ);
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NP: i32 = 0;
        let mut NP2: i32 = 0;
        let mut NV: i32 = 0;
        let mut NV2: i32 = 0;
        let mut NVXTOT: i32 = 0;
        let mut NXTDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut PLATES = ActualArray::<i32>::new(1..=(3 * MAXP));
        let mut PLATS2 = ActualArray::<i32>::new(1..=(3 * MAXP));
        let mut SPAIXI = ActualArray::<i32>::new(1..=MXIXSZ);
        let mut SPAXI2 = ActualArray::<i32>::new(1..=MXIXSZ);
        let mut SPAXI3 = ActualArray::<i32>::new(1..=MXIXSZ);
        let mut SPXISZ: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut SURF2: i32 = 0;
        let mut VGREXT = StackArray::<i32, 3>::new(1..=3);
        let mut VPSIZ3: i32 = 0;
        let mut VPSIZE: i32 = 0;
        let mut VLSIZE: i32 = 0;
        let mut VLSIZ3: i32 = 0;
        let mut VTXLSZ: i32 = 0;
        let mut W: i32 = 0;
        let mut WORK = ActualArray2D::<i32>::new(1..=2, 1..=WORKSZ);
        let mut XNCGR: i32 = 0;
        let mut XNP: i32 = 0;
        let mut XNV: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            FRAME,
            A,
            ALTLIM,
            B,
            C,
            CORPAR,
            DBUFF,
            DELTA,
            DSKDSC,
            F,
            FINSCL,
            FIRST,
            LAST,
            MAXLON,
            MINLON,
            MN32,
            MN33,
            MNCOR1,
            MNCOR2,
            MNCOR3,
            MX32,
            MX33,
            MXCOR1,
            MXCOR2,
            MXCOR3,
            RE,
            RP,
            SPAIXD,
            SPAXD2,
            SPAXD3,
            TOL,
            VRTCES,
            VRTCS2,
            VRTCS3,
            X,
            XFORM,
            ADDR,
            BODYID,
            CORSCL,
            CORSYS,
            DCLASS,
            DLADS2,
            DLADSC,
            FRAMID,
            HANDLE,
            HAN1,
            HAN2,
            HAN3,
            I,
            IBUFF,
            J,
            K,
            N,
            NLAT,
            NLON,
            NP,
            NP2,
            NV,
            NV2,
            NVXTOT,
            NXTDSC,
            PLATES,
            PLATS2,
            SPAIXI,
            SPAXI2,
            SPAXI3,
            SPXISZ,
            SURFID,
            SURF2,
            VGREXT,
            VPSIZ3,
            VPSIZE,
            VLSIZE,
            VLSIZ3,
            VTXLSZ,
            W,
            WORK,
            XNCGR,
            XNP,
            XNV,
            FOUND,
        }
    }
}

//$Procedure      F_DSKW02 ( Test DSK type 2 writer )
pub fn F_DSKW02(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

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
    // Saved variables
    //
    // Save everything to avoid stack problems.
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DSKW02", ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Create new Mars DSK files.", ctx)?;

    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    //
    // Create vertices and plates.
    //
    // The Mars radii used here need not be consistent with
    // the current generic PCK.
    //
    save.A = 3396.19;
    save.B = save.A;
    save.C = 3376.2;

    save.NLON = 20;
    save.NLAT = 10;

    support::ZZELLPLT(
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        MAXV,
        MAXP,
        &mut save.NV,
        save.VRTCES.as_slice_mut(),
        &mut save.NP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    save.FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(save.NP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the coarse scale.
    //
    save.CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    save.VPSIZE = VOXPSZ;
    save.VLSIZE = VOXNPL;
    save.SPXISZ = MXIXSZ;

    if *OK {
        //
        // Create a spatial index that includes a vertex-plate mapping.
        //
        spicelib::DSKMI2(
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.FINSCL,
            save.CORSCL,
            WORKSZ,
            save.VPSIZE,
            save.VLSIZE,
            true,
            save.SPXISZ,
            save.WORK.as_slice_mut(),
            save.SPAIXD.as_slice_mut(),
            save.SPAIXI.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Generate bounds for the 3rd coordinate.
    //
    if *OK {
        spicelib::DSKRB2(
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            LATSYS,
            save.CORPAR.as_slice(),
            &mut save.MNCOR3,
            &mut save.MXCOR3,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Set segment attribute inputs.
    //
    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);

    save.FIRST = -(spicelib::JYEAR() * 100 as f64);
    save.LAST = (spicelib::JYEAR() * 100 as f64);

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.DCLASS = 2;
    save.BODYID = 499;
    save.SURFID = 1;
    fstr::assign(&mut save.FRAME, b"IAU_MARS");

    spicelib::NAMFRM(&save.FRAME, &mut save.FRAMID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write the file.
    //
    spicelib::DSKOPN(DSK0, DSK0, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HANDLE,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create a second segment having higher resolution.
    //
    save.NLON = 30;
    save.NLAT = 15;

    support::ZZELLPLT(
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        MAXV,
        MAXP,
        &mut save.NV2,
        save.VRTCS2.as_slice_mut(),
        &mut save.NP2,
        save.PLATS2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    save.FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(save.NP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the coarse scale.
    //
    save.CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    save.VPSIZE = VOXPSZ;
    save.VLSIZE = VOXNPL;
    save.SPXISZ = MXIXSZ;

    if *OK {
        //
        // Create a spatial index that includes a vertex-plate mapping.
        //
        spicelib::DSKMI2(
            save.NV2,
            save.VRTCS2.as_slice(),
            save.NP2,
            save.PLATS2.as_slice(),
            save.FINSCL,
            save.CORSCL,
            WORKSZ,
            save.VPSIZE,
            save.VLSIZE,
            true,
            save.SPXISZ,
            save.WORK.as_slice_mut(),
            save.SPAXD2.as_slice_mut(),
            save.SPAXI2.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Generate bounds for the 3rd coordinate.
    //
    if *OK {
        spicelib::DSKRB2(
            save.NV2,
            save.VRTCS2.as_slice(),
            save.NP2,
            save.PLATS2.as_slice(),
            LATSYS,
            save.CORPAR.as_slice(),
            &mut save.MN32,
            &mut save.MX32,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // This segment has its own surface ID.
    //
    save.SURF2 = 2;

    if *OK {
        spicelib::DSKW02(
            save.HANDLE,
            save.BODYID,
            save.SURF2,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MN32,
            save.MX32,
            save.FIRST,
            save.LAST,
            save.NV2,
            save.VRTCS2.as_slice(),
            save.NP2,
            save.PLATS2.as_slice(),
            save.SPAXD2.as_slice(),
            save.SPAXI2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close the file.
    //
    if *OK {
        spicelib::DSKCLS(save.HANDLE, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create a second DSK file containing data similar to that
    // of the first segment. We want the segment's DAS address
    // ranges to be identical to those of the first segment,
    // but both the integer and d.p. data to be different. To
    // achieve this, we'll rotate the vertices to a different
    // frame. We'll still label the frame as IAU_MARS.
    //
    //
    // Let XFORM be a matrix that permutes the standard basis
    // vectors.
    //
    spicelib::CLEARD(9, save.XFORM.as_slice_mut());

    save.XFORM[[1, 2]] = 1.0;
    save.XFORM[[2, 3]] = 1.0;
    save.XFORM[[3, 1]] = 1.0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NV;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.K = ((3 * (save.I - 1)) + 1);

            spicelib::MXV(
                save.XFORM.as_slice(),
                save.VRTCES.subarray(save.K),
                save.VRTCS3.subarray_mut(save.K),
            );

            save.I += m3__;
        }
    }

    //
    // Create a spatial index for this data set.
    //
    //
    // Set the spatial index integer component size.
    //
    save.VPSIZE = VOXPSZ;
    save.VLSIZE = VOXNPL;
    save.SPXISZ = MXIXSZ;

    if *OK {
        //
        // Create a spatial index that includes a vertex-plate mapping.
        //
        spicelib::DSKMI2(
            save.NV,
            save.VRTCS3.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.FINSCL,
            save.CORSCL,
            WORKSZ,
            save.VPSIZE,
            save.VLSIZE,
            true,
            save.SPXISZ,
            save.WORK.as_slice_mut(),
            save.SPAXD3.as_slice_mut(),
            save.SPAXI3.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Generate bounds for the 3rd coordinate.
    //
    if *OK {
        spicelib::DSKRB2(
            save.NV,
            save.VRTCS3.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            LATSYS,
            save.CORPAR.as_slice(),
            &mut save.MN33,
            &mut save.MX33,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::DSKOPN(DSK1, DSK1, 0, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN1,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MN33,
            save.MX33,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCS3.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAXD3.as_slice(),
            save.SPAXI3.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if *OK {
        spicelib::DSKCLS(save.HAN1, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //***********************************************************************
    //
    //
    //     The following tests examine the files that were created.
    //
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check DSK segment\'s vertex and plate counts.", ctx)?;

    spicelib::DASOPR(DSK0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XNV = save.NV;
    save.XNP = save.NP;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWNV,
        1,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.NV),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NV", save.NV, b"=", save.XNV, 0, OK, ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWNP,
        1,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.NP),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NP", save.NP, b"=", save.XNP, 0, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel grid extents.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVGRX,
        1,
        3,
        &mut save.N,
        save.VGREXT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the voxel grid extent sub-array of the
    // integer spatial index component.
    //
    testutil::CHCKAI(
        b"VGREXT",
        save.VGREXT.as_slice(),
        b"=",
        save.SPAIXI.subarray(SIVGRX),
        3,
        OK,
        ctx,
    )?;

    //
    // We'll use the total voxel count later.
    //
    save.NVXTOT = ((save.VGREXT[1] * save.VGREXT[2]) * save.VGREXT[3]);

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check coarse voxel grid scale.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWCGSC,
        1,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.CORSCL),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the coarse voxel scale sub-array of the
    // integer spatial index component.
    //
    testutil::CHCKSI(
        b"CORSCL",
        save.CORSCL,
        b"=",
        save.SPAIXI[SICGSC],
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel pointer count.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVXPS,
        1,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.VPSIZE),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the corresponding sub-array of the
    // integer spatial index component.
    //
    testutil::CHCKSI(
        b"VOXNPT",
        save.VPSIZE,
        b"=",
        save.SPAIXI[SIVXNP],
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel-plate correspondence list size.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVXLS,
        1,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.VLSIZE),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the corresponding sub-array of the
    // integer spatial index component.
    //
    testutil::CHCKSI(
        b"VOXNPL",
        save.VLSIZE,
        b"=",
        save.SPAIXI[SIVXNL],
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check vertex-plate correspondence list size.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVTLS,
        1,
        1,
        &mut save.N,
        std::slice::from_mut(&mut save.VTXLSZ),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the corresponding sub-array of the
    // integer spatial index component.
    //
    testutil::CHCKSI(
        b"VTXNPL",
        save.VTXLSZ,
        b"=",
        save.SPAIXI[SIVTNL],
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check coarse grid", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWCGPT,
        1,
        MAXCGR,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the coarse grid size first.
    //
    save.XNCGR = (save.NVXTOT / intrinsics::pow(save.CORSCL, 3));

    testutil::CHCKSI(b"NCGR", save.N, b"=", save.XNCGR, 0, OK, ctx)?;

    if *OK {
        testutil::CHCKAI(
            b"CGRPTR",
            save.IBUFF.as_slice(),
            b"=",
            save.SPAIXI.subarray(SICGRD),
            save.N,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check plates.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWPLAT,
        1,
        (3 * save.NP),
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"PLATES",
        save.IBUFF.subarray(1),
        b"=",
        save.PLATES.as_slice(),
        (3 * save.NP),
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel-plate pointer array.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVXPT,
        1,
        save.VPSIZE,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the corresponding sub-array of the
    // integer spatial index component.
    //
    save.ADDR = (SICGRD + MAXCGR);

    testutil::CHCKAI(
        b"VOXPTR",
        save.IBUFF.subarray(1),
        b"=",
        save.SPAIXI.subarray(save.ADDR),
        save.VPSIZE,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel-plate correspondence list.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVXPL,
        1,
        save.VLSIZE,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the corresponding sub-array of the
    // integer spatial index component.
    //
    save.ADDR = ((SICGRD + MAXCGR) + save.VPSIZE);

    testutil::CHCKAI(
        b"VOXLST",
        save.IBUFF.subarray(1),
        b"=",
        save.SPAIXI.subarray(save.ADDR),
        save.VLSIZE,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check vertex-plate pointer array.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVTPT,
        1,
        save.NV,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the corresponding sub-array of the
    // integer spatial index component.
    //
    save.ADDR = (((SICGRD + MAXCGR) + save.VPSIZE) + save.VLSIZE);

    testutil::CHCKAI(
        b"VRTPTR",
        save.IBUFF.subarray(1),
        b"=",
        save.SPAIXI.subarray(save.ADDR),
        save.NV,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check vertex-plate correspondence list.", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVTPL,
        1,
        save.VTXLSZ,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare against the corresponding sub-array of the
    // integer spatial index component.
    //
    save.ADDR = ((((SICGRD + MAXCGR) + save.VPSIZE) + save.VLSIZE) + save.NV);

    testutil::CHCKAI(
        b"VRTLST",
        save.IBUFF.subarray(1),
        b"=",
        save.SPAIXI.subarray(save.ADDR),
        save.VTXLSZ,
        OK,
        ctx,
    )?;

    //
    //     The following cases exercise the logic for use of saved values.
    //
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from second segment.", ctx)?;

    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if *OK {
        spicelib::DSKI02(
            save.HANDLE,
            save.NXTDSC.as_slice(),
            KWNV,
            1,
            1,
            &mut save.N,
            save.IBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NV2", save.IBUFF[1], b"=", save.NV2, 0, OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from first segment of first file again.", ctx)?;

    //
    // This call resets the previous DLA descriptor to the first one of
    // the first file. This sets up the next test, which shows that
    // DSKI02 can detect a segment change when the DLA segment
    // descriptor start addresses match those of the previous segment,
    // but the handle changes.
    //
    if *OK {
        spicelib::DSKI02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            KWNV,
            1,
            1,
            &mut save.N,
            save.IBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NV", save.IBUFF[1], b"=", save.NV, 0, OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from first segment of second file.", ctx)?;

    spicelib::DASOPR(DSK1, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(save.HAN1, save.DLADS2.as_slice_mut(), &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if *OK {
        //
        // Check voxel-plate correspondence list. This is the call
        // to DSKI02 where the input handle changes. We use IBUFSZ
        // as the "room" argument so we don't need to look up the
        // list size. We want to look up the voxel-plate list at
        // this point because this is an integer structure that
        // differs depending on which file we're reading.
        //
        spicelib::DSKI02(
            save.HAN1,
            save.DLADS2.as_slice(),
            KWVXPL,
            1,
            IBUFSZ,
            &mut save.N,
            save.IBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We need to look up the size of the voxel-plate pointer
        // array in order to get the correct index of the voxel-plate
        // list in the spatial index.
        //
        spicelib::DSKI02(
            save.HAN1,
            save.DLADS2.as_slice(),
            KWVXPS,
            1,
            1,
            &mut save.N,
            std::slice::from_mut(&mut save.VPSIZ3),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the voxel-plate list size as well.
        //
        spicelib::DSKI02(
            save.HAN1,
            save.DLADS2.as_slice(),
            KWVXLS,
            1,
            1,
            &mut save.N,
            std::slice::from_mut(&mut save.VLSIZ3),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Compare against the corresponding sub-array of the
        // integer spatial index component.
        //
        save.ADDR = ((SICGRD + MAXCGR) + save.VPSIZ3);

        testutil::CHCKAI(
            b"VOXLST",
            save.IBUFF.subarray(1),
            b"=",
            save.SPAXI3.subarray(save.ADDR),
            save.VLSIZ3,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from first segment of first file again.", ctx)?;

    if *OK {
        spicelib::DSKI02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            KWNV,
            1,
            1,
            &mut save.N,
            save.IBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"NV", save.IBUFF[1], b"=", save.NV, 0, OK, ctx)?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check DSK descriptor", ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWDSC,
        1,
        DSKDSZ,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check descriptor elements.
    //
    testutil::CHCKSI(
        b"BODYID",
        intrinsics::IDNINT(save.DBUFF[CTRIDX]),
        b"=",
        save.BODYID,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"SURFID",
        intrinsics::IDNINT(save.DBUFF[SRFIDX]),
        b"=",
        save.SURFID,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"FRAMID",
        intrinsics::IDNINT(save.DBUFF[FRMIDX]),
        b"=",
        save.FRAMID,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"DCLASS",
        intrinsics::IDNINT(save.DBUFF[CLSIDX]),
        b"=",
        save.DCLASS,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"DTYPE",
        intrinsics::IDNINT(save.DBUFF[TYPIDX]),
        b"=",
        DTYPE,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"CORSYS",
        intrinsics::IDNINT(save.DBUFF[SYSIDX]),
        b"=",
        LATSYS,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"MNCOR1",
        save.DBUFF[MN1IDX],
        b"=",
        save.MNCOR1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MXCOR1",
        save.DBUFF[MX1IDX],
        b"=",
        save.MXCOR1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MNCOR2",
        save.DBUFF[MN2IDX],
        b"=",
        save.MNCOR2,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MXCOR2",
        save.DBUFF[MX2IDX],
        b"=",
        save.MXCOR2,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MNCOR3",
        save.DBUFF[MN3IDX],
        b"=",
        save.MNCOR3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MXCOR3",
        save.DBUFF[MX3IDX],
        b"=",
        save.MXCOR3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"CORPAR",
        save.DBUFF.subarray(PARIDX),
        b"=",
        save.CORPAR.as_slice(),
        NSYPAR,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(b"FIRST", save.DBUFF[BTMIDX], b"=", save.FIRST, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"LAST", save.DBUFF[ETMIDX], b"=", save.LAST, 0.0, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check vertex bounds", ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVTBD,
        1,
        6,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"VTXBDS",
        save.DBUFF.as_slice(),
        b"=",
        save.SPAIXD.subarray(SIVTBD),
        6,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel origin", ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVXOR,
        1,
        3,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"VOXORI",
        save.DBUFF.as_slice(),
        b"=",
        save.SPAIXD.subarray(SIVXOR),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check voxel size", ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWVXSZ,
        1,
        1,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(
        b"VOXSIZ",
        *save.DBUFF.first(),
        b"=",
        save.SPAIXD[SIVXSZ],
        0.0,
        OK,
        ctx,
    )?;

    //
    //     The following cases exercise the logic for use of saved values.
    //
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from second segment.", ctx)?;

    spicelib::DLAFNS(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.NXTDSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if *OK {
        spicelib::DSKD02(
            save.HANDLE,
            save.NXTDSC.as_slice(),
            KWVXSZ,
            1,
            1,
            &mut save.N,
            save.DBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(
            b"VOXSIZ",
            save.DBUFF[1],
            b"=",
            save.SPAXD2[SIVXSZ],
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKD02: read from first segment of second file.", ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    if *OK {
        spicelib::DSKD02(
            save.HAN1,
            save.DLADS2.as_slice(),
            KWVXSZ,
            1,
            1,
            &mut save.N,
            save.DBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(
            b"VOXSIZ",
            save.DBUFF[1],
            b"=",
            save.SPAXD3[SIVXSZ],
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Read from first segment of first file again.", ctx)?;

    if *OK {
        spicelib::DSKD02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            KWVXSZ,
            1,
            1,
            &mut save.N,
            save.DBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(
            b"VOXSIZ",
            save.DBUFF[1],
            b"=",
            save.SPAIXD[SIVXSZ],
            0.0,
            OK,
            ctx,
        )?;
    }

    //***********************************************************************
    //
    //     DSKW02 non-error, exceptional cases
    //
    //***********************************************************************

    //***********************************************************************
    //
    //
    //     Cases relating to longitude/latitude bounds are performed for
    //     both latitudinal and planetodetic coordinate systems.
    //
    //
    //***********************************************************************

    for CORIX in 1..=2 {
        if (CORIX == 1) {
            save.CORSYS = LATSYS;
            spicelib::CLEARD(3, save.CORPAR.as_slice_mut());
        } else {
            save.CORSYS = PDTSYS;
            save.CORPAR[1] = save.A;
            save.CORPAR[2] = ((save.A - save.C) / save.A);
        }
        //
        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(
            b"Create a segment with longitude bounds slightly beyond -pi:pi.",
            ctx,
        )?;

        //
        // Prepare to write a normal one-segment file.
        //
        if spicelib::EXISTS(DSK3, ctx)? {
            spicelib::DELFIL(DSK3, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Create vertices and plates.
        //
        // The Mars radii used here need not be consistent with
        // the current generic PCK.
        //
        save.A = 3396.19;
        save.B = save.A;
        save.C = 3376.2;

        save.NLON = 20;
        save.NLAT = 10;

        support::ZZELLPLT(
            save.A,
            save.B,
            save.C,
            save.NLON,
            save.NLAT,
            MAXV,
            MAXP,
            &mut save.NV,
            save.VRTCES.as_slice_mut(),
            &mut save.NP,
            save.PLATES.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Create a spatial index for the plate set.
        //
        // Use a heuristic formula for the fine scale.
        //
        save.FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(save.NP as f64, 0.23) / 8 as f64)]);

        //
        // Pick a one-size-fits-all value for the coarse scale.
        //
        save.CORSCL = 10;

        //
        // Set the spatial index integer component size.
        //
        save.VPSIZE = VOXPSZ;
        save.VLSIZE = VOXNPL;
        save.SPXISZ = MXIXSZ;

        if *OK {
            //
            // Create a spatial index that includes a vertex-plate mapping.
            //
            spicelib::DSKMI2(
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.FINSCL,
                save.CORSCL,
                WORKSZ,
                save.VPSIZE,
                save.VLSIZE,
                true,
                save.SPXISZ,
                save.WORK.as_slice_mut(),
                save.SPAIXD.as_slice_mut(),
                save.SPAIXI.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Generate bounds for the 3rd coordinate.
        //
        if *OK {
            spicelib::DSKRB2(
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                LATSYS,
                save.CORPAR.as_slice(),
                &mut save.MNCOR3,
                &mut save.MXCOR3,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Set segment attribute inputs.
        //
        save.DELTA = 0.0000000000001;

        save.MNCOR1 = (-spicelib::PI(ctx) - save.DELTA);
        save.MXCOR1 = (spicelib::PI(ctx) + save.DELTA);
        save.MNCOR2 = 0.0;
        save.MXCOR2 = spicelib::HALFPI(ctx);

        save.FIRST = -(spicelib::JYEAR() * 100 as f64);
        save.LAST = (spicelib::JYEAR() * 100 as f64);

        spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

        save.DCLASS = 2;
        save.BODYID = 499;
        save.SURFID = 1;
        fstr::assign(&mut save.FRAME, b"IAU_MARS");

        spicelib::NAMFRM(&save.FRAME, &mut save.FRAMID, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Write the file.
        //
        spicelib::DSKOPN(DSK3, DSK3, 0, &mut save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            spicelib::DSKW02(
                save.HAN3,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                LATSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Close the file.
        //
        spicelib::DSKCLS(save.HAN3, true, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Examine the longitude bounds in the DSK descriptor of the
        // file's first and only segment.
        //
        spicelib::DASOPR(DSK3, &mut save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DLABFS(save.HAN3, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLABFS found", save.FOUND, true, OK, ctx)?;

        spicelib::DSKGD(
            save.HAN3,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.MINLON = save.DSKDSC[MN1IDX];
        save.MAXLON = save.DSKDSC[MX1IDX];

        //
        // The longitude bounds of the segment should have been trimmed
        // so the total extent is 2*pi.
        //
        save.TOL = VTIGHT;

        testutil::CHCKSD(b"MINLON", save.MINLON, b"~", save.MNCOR1, save.TOL, OK, ctx)?;

        //
        // We expect the upper bound to be shifted left to make
        // the total extent 2*pi.
        //
        save.X = (save.MXCOR1 - ((2 as f64) * save.DELTA));

        testutil::CHCKSD(b"MAXLON", save.MAXLON, b"~", save.X, save.TOL, OK, ctx)?;

        //
        // Close the file.
        //
        spicelib::DASCLS(save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(
            b"Create a segment with the lower longitude bound slightly beyond -2*pi.",
            ctx,
        )?;

        save.DELTA = 0.0000000000001;

        save.MNCOR1 = (-((2 as f64) * spicelib::PI(ctx)) - save.DELTA);
        save.MXCOR1 = 0.0;

        //
        // Prepare to write to a new file.
        //
        //
        // Prepare to write a normal one-segment file.
        //
        if spicelib::EXISTS(DSK3, ctx)? {
            spicelib::DELFIL(DSK3, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::DSKOPN(DSK3, DSK3, 0, &mut save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            //
            // Write the segment.
            //

            spicelib::DSKW02(
                save.HAN3,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                LATSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Close the file.
        //
        spicelib::DSKCLS(save.HAN3, true, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Examine the longitude bounds in the DSK descriptor of the
        // file's first segment.
        //
        spicelib::DASOPR(DSK3, &mut save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DLABFS(save.HAN3, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLABFS found", save.FOUND, true, OK, ctx)?;

        spicelib::DSKGD(
            save.HAN3,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.MINLON = save.DSKDSC[MN1IDX];
        save.MAXLON = save.DSKDSC[MX1IDX];

        //
        // The longitude bounds of the segment should have been trimmed
        // so the total extent is 2*pi.
        //
        save.TOL = VTIGHT;

        //
        // We expect the upper bound to be shifted right to move it into
        // the valid range [-2*pi, 2*pi].
        //
        testutil::CHCKSD(
            b"MINLON",
            save.MINLON,
            b"~",
            -((2 as f64) * spicelib::PI(ctx)),
            save.TOL,
            OK,
            ctx,
        )?;

        testutil::CHCKSD(b"MAXLON", save.MAXLON, b"~", save.MXCOR1, save.TOL, OK, ctx)?;

        //
        // Close the file.
        //
        spicelib::DASCLS(save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(
            b"Create a segment with the upper longitude bound slightly beyond 2*pi.",
            ctx,
        )?;

        save.DELTA = 0.0000000000001;

        save.MNCOR1 = 0.0;
        save.MXCOR1 = (((2 as f64) * spicelib::PI(ctx)) + save.DELTA);

        //
        // Prepare to write to an existing file.
        //
        spicelib::DASOPW(DSK3, &mut save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            //
            // Write the segment.
            //

            spicelib::DSKW02(
                save.HAN3,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                LATSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        //
        // Close the file.
        //
        spicelib::DSKCLS(save.HAN3, true, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Examine the longitude bounds in the DSK descriptor of the
        // file's second segment.
        //
        spicelib::DASOPR(DSK3, &mut save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DLABFS(save.HAN3, save.DLADSC.as_slice_mut(), &mut save.FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLABFS found", save.FOUND, true, OK, ctx)?;

        spicelib::DLAFNS(
            save.HAN3,
            save.DLADSC.as_slice(),
            save.NXTDSC.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"DLAFNS found", save.FOUND, true, OK, ctx)?;

        spicelib::MOVEI(save.NXTDSC.as_slice(), DLADSZ, save.DLADSC.as_slice_mut());

        spicelib::DSKGD(
            save.HAN3,
            save.DLADSC.as_slice(),
            save.DSKDSC.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.MINLON = save.DSKDSC[MN1IDX];
        save.MAXLON = save.DSKDSC[MX1IDX];

        //
        // The longitude bounds of the segment should have been trimmed
        // so the total extent is 2*pi.
        //
        save.TOL = VTIGHT;

        //
        // We expect the upper bound to be shifted left to move it into
        // the valid range [-2*pi, 2*pi].
        //
        testutil::CHCKSD(b"MINLON", save.MINLON, b"~", save.MNCOR1, save.TOL, OK, ctx)?;

        testutil::CHCKSD(
            b"MAXLON",
            save.MAXLON,
            b"~",
            ((2 as f64) * spicelib::PI(ctx)),
            save.TOL,
            OK,
            ctx,
        )?;

        //
        // Close the file.
        //
        spicelib::DASCLS(save.HAN3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    //
    //
    // This is the end of the latitudinal/planetodetic non-error,
    // exceptional case loop.
    //
    //
    //

    //***********************************************************************
    //
    //     DSKW02 error cases
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: frame cannot be mapped to ID code.", ctx)?;

    //
    // Prepare to write a normal one-segment file.
    //
    if spicelib::EXISTS(DSK2, ctx)? {
        spicelib::DELFIL(DSK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create vertices and plates.
    //
    // The Mars radii used here need not be consistent with
    // the current generic PCK.
    //
    save.A = 3396.19;
    save.B = save.A;
    save.C = 3376.2;

    save.NLON = 20;
    save.NLAT = 10;

    support::ZZELLPLT(
        save.A,
        save.B,
        save.C,
        save.NLON,
        save.NLAT,
        MAXV,
        MAXP,
        &mut save.NV,
        save.VRTCES.as_slice_mut(),
        &mut save.NP,
        save.PLATES.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a spatial index for the plate set.
    //
    // Use a heuristic formula for the fine scale.
    //
    save.FINSCL = intrinsics::DMAX1(&[1.0, (f64::powf(save.NP as f64, 0.23) / 8 as f64)]);

    //
    // Pick a one-size-fits-all value for the coarse scale.
    //
    save.CORSCL = 10;

    //
    // Set the spatial index integer component size.
    //
    save.VPSIZE = VOXPSZ;
    save.VLSIZE = VOXNPL;
    save.SPXISZ = MXIXSZ;

    if *OK {
        //
        // Create a spatial index that includes a vertex-plate mapping.
        //
        spicelib::DSKMI2(
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.FINSCL,
            save.CORSCL,
            WORKSZ,
            save.VPSIZE,
            save.VLSIZE,
            true,
            save.SPXISZ,
            save.WORK.as_slice_mut(),
            save.SPAIXD.as_slice_mut(),
            save.SPAIXI.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Generate bounds for the 3rd coordinate.
    //
    if *OK {
        spicelib::DSKRB2(
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            LATSYS,
            save.CORPAR.as_slice(),
            &mut save.MNCOR3,
            &mut save.MXCOR3,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Set segment attribute inputs.
    //
    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);

    save.FIRST = -(spicelib::JYEAR() * 100 as f64);
    save.LAST = (spicelib::JYEAR() * 100 as f64);

    spicelib::CLEARD(NSYPAR, save.CORPAR.as_slice_mut());

    save.DCLASS = 2;
    save.BODYID = 499;
    save.SURFID = 1;
    fstr::assign(&mut save.FRAME, b"IAU_XXX");

    spicelib::NAMFRM(&save.FRAME, &mut save.FRAMID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Write the file.
    //
    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(FRAMEIDNOTFOUND)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the frame name.
    //
    fstr::assign(&mut save.FRAME, b"IAU_MARS");

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: time bounds are out of order.", ctx)?;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.LAST,
            save.FIRST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(TIMESOUTOFORDER)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //***********************************************************************
    //
    //
    //     Cases relating to longitude/latitude bounds are performed for
    //     both latitudinal and planetodetic coordinate systems.
    //
    //
    //***********************************************************************

    for CORIX in 1..=2 {
        if (CORIX == 1) {
            save.CORSYS = LATSYS;
            spicelib::CLEARD(3, save.CORPAR.as_slice_mut());
        } else {
            save.CORSYS = PDTSYS;
            save.CORPAR[1] = save.A;
            save.CORPAR[2] = ((save.A - save.C) / save.A);
        }

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(b"DSKW02 error: longitude bounds out of bounds.", ctx)?;

        save.MNCOR1 = (-spicelib::TWOPI(ctx) - ((2 as f64) * ANGMRG));

        spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
        }

        save.MNCOR1 = 0.0;
        save.MXCOR1 = (spicelib::TWOPI(ctx) + ((2 as f64) * ANGMRG));

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            save.CORSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        spicelib::DSKCLS(save.HAN2, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(DSK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Restore longitude bounds.
        //
        save.MNCOR1 = 0.0;
        save.MXCOR1 = spicelib::TWOPI(ctx);

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(b"DSKW02 error: maximum longitude bound exceeds minimum longitude by an excessive amount.", ctx)?;

        save.MNCOR1 = -spicelib::PI(ctx);
        save.MXCOR1 = (spicelib::PI(ctx) + ((2 as f64) * ANGMRG));

        spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(INVALIDLONEXTENT)", OK, ctx)?;
        }

        spicelib::DSKCLS(save.HAN2, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(DSK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(b"DSKW02 error: minimum longitude bound exceeds maximum longitude by an excessive amount.", ctx)?;

        //
        // This is the case where the maximum is less than the
        // minimum by enough so that adding 2*pi to the maximum
        // doesn't make it greater than the minimum.
        //

        save.MNCOR1 = spicelib::PI(ctx);
        save.MXCOR1 = (-spicelib::PI(ctx) - ((2 as f64) * ANGMRG));

        spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(INVALIDLONEXTENT)", OK, ctx)?;
        }

        spicelib::DSKCLS(save.HAN2, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(DSK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Restore longitude bounds.
        //
        save.MNCOR1 = 0.0;
        save.MXCOR1 = spicelib::TWOPI(ctx);

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(b"DSKW02 error: latitude bounds out of range.", ctx)?;

        spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            //
            // Lower bound is too low.
            //
            save.MNCOR2 = (-spicelib::HALFPI(ctx) - ((2 as f64) * ANGMRG));

            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

            //
            // Lower bound is too high.
            //
            save.MNCOR2 = (spicelib::HALFPI(ctx) - (ANGMRG / 2 as f64));
            save.MXCOR2 = spicelib::HALFPI(ctx);

            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

            //
            // Upper bound is too high.
            //
            save.MNCOR2 = -spicelib::HALFPI(ctx);
            save.MXCOR2 = (spicelib::HALFPI(ctx) + ((2 as f64) * ANGMRG));

            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

            //
            // Upper bound is too low.
            //
            save.MNCOR2 = -spicelib::HALFPI(ctx);
            save.MXCOR2 = (-spicelib::HALFPI(ctx) + (ANGMRG / 2 as f64));

            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
        }

        spicelib::DSKCLS(save.HAN2, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(DSK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // --- Case --------------------------------------------------------
        //
        testutil::TCASE(b"DSKW02 error: latitude bounds out of order.", ctx)?;

        save.MNCOR2 = (spicelib::PI(ctx) / 4 as f64);
        save.MXCOR2 = -(spicelib::PI(ctx) / 4 as f64);

        spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if *OK {
            spicelib::DSKW02(
                save.HAN2,
                save.BODYID,
                save.SURFID,
                save.DCLASS,
                &save.FRAME,
                save.CORSYS,
                save.CORPAR.as_slice(),
                save.MNCOR1,
                save.MXCOR1,
                save.MNCOR2,
                save.MXCOR2,
                save.MNCOR3,
                save.MXCOR3,
                save.FIRST,
                save.LAST,
                save.NV,
                save.VRTCES.as_slice(),
                save.NP,
                save.PLATES.as_slice(),
                save.SPAIXD.as_slice(),
                save.SPAIXI.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;
        }

        spicelib::DSKCLS(save.HAN2, true, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DELFIL(DSK2, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.MNCOR2 = -(spicelib::PI(ctx) / 2 as f64);
        save.MXCOR2 = (spicelib::PI(ctx) / 2 as f64);
    }
    //
    //
    //
    // This is the end of the latitudinal/planetodetic error case loop.
    //
    //
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: radius bounds out of range.", ctx)?;

    save.CORSYS = LATSYS;

    save.MNCOR3 = -0.0000000000000001;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            save.CORSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
    }

    save.MNCOR3 = 0.0;
    save.X = save.MXCOR3;

    save.MXCOR3 = 0.0;

    spicelib::DSKW02(
        save.HAN2,
        save.BODYID,
        save.SURFID,
        save.DCLASS,
        &save.FRAME,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MNCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MXCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    save.MXCOR3 = -0.0000000000000001;

    spicelib::DSKW02(
        save.HAN2,
        save.BODYID,
        save.SURFID,
        save.DCLASS,
        &save.FRAME,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MNCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MXCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore MXCOR3.
    //
    save.MXCOR3 = save.X;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: longitude bounds are equal.", ctx)?;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MNCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(ZEROBOUNDSEXTENT)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: radius bounds out of order.", ctx)?;

    save.MNCOR3 = 10000.0;
    save.MXCOR3 = 1000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: radius bounds are equal.", ctx)?;

    save.MNCOR3 = 10000.0;
    save.MXCOR3 = save.MNCOR3;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(ZEROBOUNDSEXTENT)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: altitude bounds out of order.", ctx)?;

    save.RE = 10000.0;
    save.RP = 5000.0;
    save.F = 0.5;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.MNCOR3 = 10.0;
    save.MXCOR3 = -10.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            PDTSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: altitude bounds are equal.", ctx)?;

    save.RE = 10000.0;
    save.RP = 5000.0;
    save.F = 0.5;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.MNCOR3 = 10.0;
    save.MXCOR3 = save.MNCOR3;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            PDTSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(ZEROBOUNDSEXTENT)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: lower altitude bound less than or equal to min{ -a**2/b, -b**2/a }. Oblate case.", ctx)?;

    save.RE = 10000.0;
    save.RP = 5000.0;
    save.F = 0.5;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.ALTLIM = intrinsics::DMIN1(&[
        -(f64::powi(save.RE, 2) / save.RP),
        -(f64::powi(save.RP, 2) / save.RE),
    ]);
    save.MNCOR3 = (save.ALTLIM - 0.000001);
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            PDTSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(DEGENERATESURFACE)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: lower altitude bound less than or equal to min{ -a**2/b, -b**2/a }. Prolate case.", ctx)?;

    save.RE = 10000.0;
    save.RP = 15000.0;
    save.F = -0.5;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.ALTLIM = intrinsics::DMIN1(&[
        -(f64::powi(save.RE, 2) / save.RP),
        -(f64::powi(save.RP, 2) / save.RE),
    ]);
    save.MNCOR3 = (save.ALTLIM - 0.000001);
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            PDTSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(DEGENERATESURFACE)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: invalid equatorial radius.", ctx)?;

    save.RE = -10000.0;
    save.F = 0.5;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.MNCOR3 = -10.0;
    save.MXCOR3 = 10.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            PDTSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
    }

    save.RE = 0.0;
    save.CORPAR[1] = save.RE;

    spicelib::DSKW02(
        save.HAN2,
        save.BODYID,
        save.SURFID,
        save.DCLASS,
        &save.FRAME,
        PDTSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MNCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MXCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: invalid flattening coefficient.", ctx)?;

    save.RE = 10000.0;
    save.F = 1.5;

    save.CORPAR[1] = save.RE;
    save.CORPAR[2] = save.F;

    save.MNCOR3 = -10.0;
    save.MXCOR3 = 10.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            PDTSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKW02 error: bounds are equal. Rectangular coordinate case.",
        ctx,
    )?;

    save.MNCOR1 = -10.0;
    save.MXCOR1 = 10.0;

    save.MNCOR2 = -20.0;
    save.MXCOR2 = 20.0;

    save.MNCOR3 = -30.0;
    save.MXCOR3 = 30.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        //
        // Set X bounds equal.
        //
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            RECSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MNCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;
    }

    //
    // Set Y bounds equal.
    //
    spicelib::DSKW02(
        save.HAN2,
        save.BODYID,
        save.SURFID,
        save.DCLASS,
        &save.FRAME,
        RECSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MXCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MXCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    //
    // Set Z bounds equal.
    //
    spicelib::DSKW02(
        save.HAN2,
        save.BODYID,
        save.SURFID,
        save.DCLASS,
        &save.FRAME,
        RECSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MNCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MNCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: bad coordinate system.", ctx)?;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CORSYS = -1;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            save.CORSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: bad vertex index within plate.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the index range of the vertices within the integer
    // spatial index.
    //
    save.I = (save.NP / 2);
    save.J = (2 + (3 * (save.I - 1)));
    save.K = save.PLATES[save.J];

    save.PLATES[save.J] = 0;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(BADVERTEXINDEX)", OK, ctx)?;
    }

    //
    // Restore the plate.
    //
    save.PLATES[save.J] = save.K;

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: bad vertex count.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            0,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            -1,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        //     CALL DSKW02 ( HAN2,        BODYID,      SURFID,
        // .                 DCLASS,      FRAME,       LATSYS,
        // .                 CORPAR,      MNCOR1,      MXCOR1,
        // .                 MNCOR2,      MXCOR2,      MNCOR3,
        // .                 MXCOR3,      FIRST,       LAST,
        // .                 MAXVRT+1,    VRTCES,      NP,
        // .                 PLATES,      SPAIXD,      SPAIXI )
        //     CALL CHCKXC ( .TRUE., 'SPICE(VALUEOUTOFRANGE)', OK )
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: bad plate count", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            0,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            -1,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        //     CALL DSKW02 ( HAN2,        BODYID,      SURFID,
        // .                 DCLASS,      FRAME,       LATSYS,
        // .                 CORPAR,      MNCOR1,      MXCOR1,
        // .                 MNCOR2,      MXCOR2,      MNCOR3,
        // .                 MXCOR3,      FIRST,       LAST,
        // .                 NV,          VRTCES,      MAXPLT+1,
        // .                 PLATES,      SPAIXD,      SPAIXI )
        //     CALL CHCKXC ( .TRUE., 'SPICE(VALUEOUTOFRANGE)', OK )
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: bad voxel grid extents.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the extents in the integer spatial index.
    //
    if *OK {
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.W = save.SPAIXI[save.I];

                save.SPAIXI[save.I] = 0;

                spicelib::DSKW02(
                    save.HAN2,
                    save.BODYID,
                    save.SURFID,
                    save.DCLASS,
                    &save.FRAME,
                    LATSYS,
                    save.CORPAR.as_slice(),
                    save.MNCOR1,
                    save.MXCOR1,
                    save.MNCOR2,
                    save.MXCOR2,
                    save.MNCOR3,
                    save.MXCOR3,
                    save.FIRST,
                    save.LAST,
                    save.NV,
                    save.VRTCES.as_slice(),
                    save.NP,
                    save.PLATES.as_slice(),
                    save.SPAIXD.as_slice(),
                    save.SPAIXI.as_slice(),
                    ctx,
                )?;
                testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

                save.SPAIXI[save.I] = (MAXVOX + 1);

                spicelib::DSKW02(
                    save.HAN2,
                    save.BODYID,
                    save.SURFID,
                    save.DCLASS,
                    &save.FRAME,
                    LATSYS,
                    save.CORPAR.as_slice(),
                    save.MNCOR1,
                    save.MXCOR1,
                    save.MNCOR2,
                    save.MXCOR2,
                    save.MNCOR3,
                    save.MXCOR3,
                    save.FIRST,
                    save.LAST,
                    save.NV,
                    save.VRTCES.as_slice(),
                    save.NP,
                    save.PLATES.as_slice(),
                    save.SPAIXD.as_slice(),
                    save.SPAIXI.as_slice(),
                    ctx,
                )?;
                testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

                //
                // Restore the extent.
                //
                save.SPAIXI[save.I] = save.W;

                save.I += m3__;
            }
        }
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: fine voxel count too large.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the extents in the integer spatial index.
    //
    if *OK {
        spicelib::MOVEI(save.SPAIXI.as_slice(), 3, save.IBUFF.as_slice_mut());

        save.SPAIXI[1] = 10000;
        save.SPAIXI[2] = 100000;
        save.SPAIXI[3] = 1;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the voxel grid extents.
    //
    spicelib::MOVEI(save.IBUFF.as_slice(), 3, save.SPAIXI.as_slice_mut());

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: coarse voxel scale out of range.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the extents in the integer spatial index.
    //
    if *OK {
        //
        // Save the coarse scale and the grid extents.
        //
        save.W = save.SPAIXI[SICGSC];

        spicelib::MOVEI(save.SPAIXI.as_slice(), 3, save.IBUFF.as_slice_mut());

        save.SPAIXI[SICGSC] = 2;

        save.SPAIXI[1] = 100;
        save.SPAIXI[2] = 100;
        save.SPAIXI[3] = 100;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        save.SPAIXI[SICGSC] = 0;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        save.SPAIXI[SICGSC] = -1;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

        //
        // Make the scale greater than the cube root of the
        // total fine voxel count.
        //
        save.SPAIXI[SICGSC] = 101;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the voxel grid extents and the coarse scale.
    //
    spicelib::MOVEI(save.IBUFF.as_slice(), 3, save.SPAIXI.as_slice_mut());

    save.SPAIXI[SICGSC] = save.W;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKW02 error: cube of coarse voxel scale does not evenly divide fine voxel count.",
        ctx,
    )?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the extents in the integer spatial index.
    //
    if *OK {
        //
        // Save the coarse scale and the grid extents.
        //
        save.W = save.SPAIXI[SICGSC];

        spicelib::MOVEI(save.SPAIXI.as_slice(), 3, save.IBUFF.as_slice_mut());

        save.SPAIXI[SICGSC] = 3;

        save.SPAIXI[1] = 10;
        save.SPAIXI[2] = 10;
        save.SPAIXI[3] = 10;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INCOMPATIBLESCALE)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the voxel grid extents and the coarse scale.
    //
    spicelib::MOVEI(save.IBUFF.as_slice(), 3, save.SPAIXI.as_slice_mut());

    save.SPAIXI[SICGSC] = save.W;

    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: coarse voxel count too large.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the extents in the integer spatial index.
    //
    if *OK {
        //
        // Save the coarse scale and the grid extents.
        //
        save.W = save.SPAIXI[SICGSC];

        spicelib::MOVEI(save.SPAIXI.as_slice(), 3, save.IBUFF.as_slice_mut());

        save.SPAIXI[SICGSC] = 1;

        save.SPAIXI[1] = 100;
        save.SPAIXI[2] = 100;
        save.SPAIXI[3] = 100;

        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            save.DCLASS,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the voxel grid extents and the coarse scale.
    //
    spicelib::MOVEI(save.IBUFF.as_slice(), 3, save.SPAIXI.as_slice_mut());

    save.SPAIXI[SICGSC] = save.W;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: bad data class.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKOPN(DSK2, DSK2, 0, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if *OK {
        spicelib::DSKW02(
            save.HAN2,
            save.BODYID,
            save.SURFID,
            -1,
            &save.FRAME,
            LATSYS,
            save.CORPAR.as_slice(),
            save.MNCOR1,
            save.MXCOR1,
            save.MNCOR2,
            save.MXCOR2,
            save.MNCOR3,
            save.MXCOR3,
            save.FIRST,
            save.LAST,
            save.NV,
            save.VRTCES.as_slice(),
            save.NP,
            save.PLATES.as_slice(),
            save.SPAIXD.as_slice(),
            save.SPAIXI.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;
    }

    spicelib::DSKCLS(save.HAN2, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKW02 error: write to closed file.", ctx)?;

    save.MNCOR1 = 0.0;
    save.MXCOR1 = spicelib::TWOPI(ctx);
    save.MNCOR2 = -spicelib::HALFPI(ctx);
    save.MXCOR2 = spicelib::HALFPI(ctx);
    save.MNCOR3 = 0.0;
    save.MXCOR3 = 10000.0;

    spicelib::DSKW02(
        save.HAN2,
        save.BODYID,
        save.SURFID,
        save.DCLASS,
        &save.FRAME,
        LATSYS,
        save.CORPAR.as_slice(),
        save.MNCOR1,
        save.MXCOR1,
        save.MNCOR2,
        save.MXCOR2,
        save.MNCOR3,
        save.MXCOR3,
        save.FIRST,
        save.LAST,
        save.NV,
        save.VRTCES.as_slice(),
        save.NP,
        save.PLATES.as_slice(),
        save.SPAIXD.as_slice(),
        save.SPAIXI.as_slice(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DASNOSUCHHANDLE)", OK, ctx)?;

    //
    //
    //     Clean up.
    //
    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Unload and delete DSK files.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DASCLS(save.HAN3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
