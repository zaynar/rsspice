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
const DSK0: &[u8] = b"dsk02_test0.bds";
const DSK1: &[u8] = b"dsk02_test1.bds";
const VTIGHT: f64 = 0.00000000000001;
const DTYPE: i32 = 2;
const FRNMLN: i32 = 32;
const IBUFSZ: i32 = MAXCGR;
const LNSIZE: i32 = 80;
const MAXP: i32 = 20000;
const MAXV: i32 = 10000;
const VOXPSZ: i32 = 100000;
const VOXNPL: i32 = 200000;
const MXIXSZ: i32 = 1000000;
const WORKSZ: i32 = 1000000;
const DBUFSZ: i32 = (3 * MAXV);

struct SaveVars {
    FRAME: Vec<u8>,
    LABEL: Vec<u8>,
    A: f64,
    B: f64,
    C: f64,
    CORPAR: StackArray<f64, 10>,
    DBUFF: ActualArray<f64>,
    DSKDSC: StackArray<f64, 24>,
    FINSCL: f64,
    FIRST: f64,
    LAST: f64,
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
    NORMAL: StackArray<f64, 3>,
    OVTBDS: StackArray2D<f64, 6>,
    OVXORI: StackArray<f64, 3>,
    OVXSIZ: f64,
    SPAIXD: StackArray<f64, 10>,
    SPAXD2: StackArray<f64, 10>,
    SPAXD3: StackArray<f64, 10>,
    VARRAY: StackArray2D<f64, 9>,
    VRTCES: ActualArray<f64>,
    VRTCS2: ActualArray<f64>,
    VRTCS3: ActualArray<f64>,
    XFORM: StackArray2D<f64, 9>,
    XNORML: StackArray<f64, 3>,
    ADDR: i32,
    BODYID: i32,
    CORSCL: i32,
    DCLASS: i32,
    DLADS2: StackArray<i32, 8>,
    DLADSC: StackArray<i32, 8>,
    FRAMID: i32,
    HANDLE: i32,
    HAN1: i32,
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
    OCRSCL: i32,
    ONP: i32,
    ONV: i32,
    ONVXTT: i32,
    OVGRXT: StackArray<i32, 3>,
    OVLSIZ: i32,
    OVPSIZ: i32,
    OVTLSZ: i32,
    PLATE: StackArray<i32, 3>,
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
    WORK: ActualArray2D<i32>,
    XNCGR: i32,
    XNP: i32,
    XNV: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRAME = vec![b' '; FRNMLN as usize];
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DBUFF = ActualArray::<f64>::new(1..=DBUFSZ);
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut FINSCL: f64 = 0.0;
        let mut FIRST: f64 = 0.0;
        let mut LAST: f64 = 0.0;
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
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut OVTBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut OVXORI = StackArray::<f64, 3>::new(1..=3);
        let mut OVXSIZ: f64 = 0.0;
        let mut SPAIXD = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut SPAXD2 = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut SPAXD3 = StackArray::<f64, 10>::new(1..=IXDFIX);
        let mut VARRAY = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut VRTCES = ActualArray::<f64>::new(1..=(3 * MAXV));
        let mut VRTCS2 = ActualArray::<f64>::new(1..=(3 * MAXV));
        let mut VRTCS3 = ActualArray::<f64>::new(1..=(3 * MAXV));
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XNORML = StackArray::<f64, 3>::new(1..=3);
        let mut ADDR: i32 = 0;
        let mut BODYID: i32 = 0;
        let mut CORSCL: i32 = 0;
        let mut DCLASS: i32 = 0;
        let mut DLADS2 = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut FRAMID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HAN1: i32 = 0;
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
        let mut OCRSCL: i32 = 0;
        let mut ONP: i32 = 0;
        let mut ONV: i32 = 0;
        let mut ONVXTT: i32 = 0;
        let mut OVGRXT = StackArray::<i32, 3>::new(1..=3);
        let mut OVLSIZ: i32 = 0;
        let mut OVPSIZ: i32 = 0;
        let mut OVTLSZ: i32 = 0;
        let mut PLATE = StackArray::<i32, 3>::new(1..=3);
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
        let mut WORK = ActualArray2D::<i32>::new(1..=2, 1..=WORKSZ);
        let mut XNCGR: i32 = 0;
        let mut XNP: i32 = 0;
        let mut XNV: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            FRAME,
            LABEL,
            A,
            B,
            C,
            CORPAR,
            DBUFF,
            DSKDSC,
            FINSCL,
            FIRST,
            LAST,
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
            NORMAL,
            OVTBDS,
            OVXORI,
            OVXSIZ,
            SPAIXD,
            SPAXD2,
            SPAXD3,
            VARRAY,
            VRTCES,
            VRTCS2,
            VRTCS3,
            XFORM,
            XNORML,
            ADDR,
            BODYID,
            CORSCL,
            DCLASS,
            DLADS2,
            DLADSC,
            FRAMID,
            HANDLE,
            HAN1,
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
            OCRSCL,
            ONP,
            ONV,
            ONVXTT,
            OVGRXT,
            OVLSIZ,
            OVPSIZ,
            OVTLSZ,
            PLATE,
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
            WORK,
            XNCGR,
            XNP,
            XNV,
            FOUND,
        }
    }
}

//$Procedure      F_DSK02 ( Test DSK type 2 low-level routines )
pub fn F_DSK02(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_DSK02", ctx)?;

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

    for I in 1..=save.NV {
        save.K = ((3 * (I - 1)) + 1);

        spicelib::MXV(
            save.XFORM.as_slice(),
            save.VRTCES.subarray(save.K),
            save.VRTCS3.subarray_mut(save.K),
        );
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
    //     DSKI02 tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKI02: Check DSK segment\'s vertex and plate counts.",
        ctx,
    )?;

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
    testutil::TCASE(b"DSKI02: Check voxel grid extents.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check coarse voxel grid scale.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check voxel pointer count.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check voxel-plate correspondence list size.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check vertex-plate correspondence list size.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check coarse grid", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check plates.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check voxel-plate pointer array.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check voxel-plate correspondence list.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check vertex-plate pointer array.", ctx)?;

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
    testutil::TCASE(b"DSKI02: Check vertex-plate correspondence list.", ctx)?;

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
    testutil::TCASE(b"DSKI02: read from second segment.", ctx)?;

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
    testutil::TCASE(b"DSKI02: read from first segment of first file again.", ctx)?;

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
    testutil::TCASE(b"DSKI02: read from first segment of second file.", ctx)?;

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
    testutil::TCASE(b"DSKI02: read from first segment of first file again.", ctx)?;

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
    testutil::TCASE(b"DSKI02 error: invalid keyword", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        -1,
        1,
        1,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKI02 error: invalid room value", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWPLAT,
        1,
        0,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWPLAT,
        1,
        -1,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKI02 error: invalid start value", ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWPLAT,
        -1,
        1,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKI02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWPLAT,
        ((3 * save.NP) + 1),
        save.NP,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     DSKD02 tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKD02: check DSK descriptor", ctx)?;

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
    testutil::TCASE(b"DSKD02: check vertex bounds", ctx)?;

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
    testutil::TCASE(b"DSKD02: check voxel origin", ctx)?;

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
    testutil::TCASE(b"DSKD02: check voxel size", ctx)?;

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
    testutil::TCASE(b"DSKD02: read from second segment.", ctx)?;

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
    testutil::TCASE(b"DSKD02: read from first segment of first file again.", ctx)?;

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

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKD02 error: invalid keyword", ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        -1,
        1,
        1,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKD02 error: invalid room value", ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWDSC,
        1,
        0,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWDSC,
        1,
        -1,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKD02 error: invalid start value", ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWDSC,
        -1,
        1,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKD02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        KWDSC,
        (DSKDSZ + 1),
        DSKDSZ,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     DSKB02 tests
    //
    //***********************************************************************

    testutil::TCASE(
        b"DSKB02: Check parameters from first segment of first file",
        ctx,
    )?;

    spicelib::DSKB02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        &mut save.ONV,
        &mut save.ONP,
        &mut save.ONVXTT,
        save.OVTBDS.as_slice_mut(),
        &mut save.OVXSIZ,
        save.OVXORI.as_slice_mut(),
        save.OVGRXT.as_slice_mut(),
        &mut save.OCRSCL,
        &mut save.OVTLSZ,
        &mut save.OVPSIZ,
        &mut save.OVLSIZ,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NV", save.ONV, b"=", save.XNV, 0, OK, ctx)?;
    testutil::CHCKSI(b"NP", save.ONP, b"=", save.XNP, 0, OK, ctx)?;

    //
    // Check the voxel grid extent out of order so it can be used to
    // check the total count.
    //
    testutil::CHCKAI(
        b"VGREXT",
        save.OVGRXT.as_slice(),
        b"=",
        save.SPAIXI.subarray(SIVGRX),
        3,
        OK,
        ctx,
    )?;

    save.J = ((save.VGREXT[1] * save.VGREXT[2]) * save.VGREXT[3]);

    testutil::CHCKSI(b"NVXTOT", save.ONVXTT, b"=", save.J, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"VTXBDS",
        save.OVTBDS.as_slice(),
        b"=",
        save.SPAIXD.subarray(SIVTBD),
        6,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"VOXSIZ",
        save.OVXSIZ,
        b"=",
        save.SPAIXD[SIVXSZ],
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"VOXORI",
        save.OVXORI.as_slice(),
        b"=",
        save.SPAIXD.subarray(SIVXOR),
        3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSI(
        b"CORSCL",
        save.OCRSCL,
        b"=",
        save.SPAIXI[SICGSC],
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKSI(
        b"VTXLSZ",
        save.OVTLSZ,
        b"=",
        save.SPAIXI[SIVTNL],
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKSI(
        b"VOXPSZ",
        save.OVPSIZ,
        b"=",
        save.SPAIXI[SIVXNP],
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKSI(
        b"VOXLSZ",
        save.OVLSIZ,
        b"=",
        save.SPAIXI[SIVXNL],
        0,
        OK,
        ctx,
    )?;

    //***********************************************************************
    //
    //     DSKZ02 tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKZ02: Check DSK segment\'s vertex and plate counts.",
        ctx,
    )?;

    spicelib::DSKZ02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        &mut save.NV,
        &mut save.NP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NV", save.NV, b"=", save.XNV, 0, OK, ctx)?;
    testutil::CHCKSI(b"NP", save.NP, b"=", save.XNP, 0, OK, ctx)?;

    //***********************************************************************
    //
    //     DSKV02 tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKV02: get vertices from first segment of first file in one call.",
        ctx,
    )?;

    spicelib::DSKV02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        1,
        save.NV,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"VRTCES",
        save.DBUFF.as_slice(),
        b"=",
        save.VRTCES.as_slice(),
        (3 * save.NV),
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKV02: get vertices from first segment of first file one at a time.",
        ctx,
    )?;

    for I in 1..=save.NV {
        spicelib::DSKV02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.DBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = ((3 * (I - 1)) + 1);

        testutil::CHCKAD(
            b"VRTCES",
            save.DBUFF.as_slice(),
            b"=",
            save.VRTCES.subarray(save.J),
            3,
            0.0,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKV02 error: bad ROOM value", ctx)?;

    spicelib::DSKV02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        1,
        0,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKV02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        1,
        -1,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKV02 error: bad START value", ctx)?;

    spicelib::DSKV02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        0,
        save.NV,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKV02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        (save.NV + 1),
        save.NV,
        &mut save.N,
        save.DBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     DSKP02 tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKP02: get plates from first segment of first file in one call.",
        ctx,
    )?;

    spicelib::DSKP02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        1,
        save.NP,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAI(
        b"PLATES",
        save.IBUFF.as_slice(),
        b"=",
        save.PLATES.as_slice(),
        (3 * save.NP),
        OK,
        ctx,
    )?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKP02: get plates from first segment of first file one at a time",
        ctx,
    )?;

    for I in 1..=save.NP {
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.IBUFF.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.J = ((3 * (I - 1)) + 1);

        testutil::CHCKAI(
            b"PLATES",
            save.IBUFF.as_slice(),
            b"=",
            save.PLATES.subarray(save.J),
            3,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKP02 error: bad room value", ctx)?;

    spicelib::DSKP02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        1,
        0,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKP02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        1,
        -1,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"DSKP02 error: bad START value", ctx)?;

    spicelib::DSKP02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        0,
        save.NP,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    spicelib::DSKP02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        (save.NP + 1),
        save.NP,
        &mut save.N,
        save.IBUFF.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INDEXOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     DSKN02 tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKN02: check normal vectors for all plates in first segment.",
        ctx,
    )?;

    for I in 1..=save.NP {
        //
        // Get the normal vector for the Ith plate.
        //
        spicelib::DSKN02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            save.NORMAL.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Get the Ith plate; look up its vertices.
        //
        spicelib::DSKP02(
            save.HANDLE,
            save.DLADSC.as_slice(),
            I,
            1,
            &mut save.N,
            save.PLATE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.K = save.PLATE[save.J];

                spicelib::DSKV02(
                    save.HANDLE,
                    save.DLADSC.as_slice(),
                    save.K,
                    1,
                    &mut save.N,
                    save.VARRAY.subarray_mut([1, save.J]),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Compute the expected normal vector. Note that the output
        // of PLTNRM does not have unit length.
        //
        spicelib::PLTNRM(
            save.VARRAY.subarray([1, 1]),
            save.VARRAY.subarray([1, 2]),
            save.VARRAY.subarray([1, 3]),
            save.XNORML.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VHATIP(save.XNORML.as_slice_mut());

        fstr::assign(&mut save.LABEL, b"Normal @");
        spicelib::REPMI(&save.LABEL.to_vec(), b"@", I, &mut save.LABEL, ctx);
        testutil::CHCKAD(
            &save.LABEL,
            save.NORMAL.as_slice(),
            b"~~/",
            save.XNORML.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //***********************************************************************
    //
    //     DSKGD tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"DSKGD: check descriptor of first segment of first file.",
        ctx,
    )?;

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Check descriptor elements.
    //
    testutil::CHCKSI(
        b"BODYID",
        intrinsics::IDNINT(save.DSKDSC[CTRIDX]),
        b"=",
        save.BODYID,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"SURFID",
        intrinsics::IDNINT(save.DSKDSC[SRFIDX]),
        b"=",
        save.SURFID,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"FRAMID",
        intrinsics::IDNINT(save.DSKDSC[FRMIDX]),
        b"=",
        save.FRAMID,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"DCLASS",
        intrinsics::IDNINT(save.DSKDSC[CLSIDX]),
        b"=",
        save.DCLASS,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"DTYPE",
        intrinsics::IDNINT(save.DSKDSC[TYPIDX]),
        b"=",
        DTYPE,
        0,
        OK,
        ctx,
    )?;
    testutil::CHCKSI(
        b"CORSYS",
        intrinsics::IDNINT(save.DSKDSC[SYSIDX]),
        b"=",
        LATSYS,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"MNCOR1",
        save.DSKDSC[MN1IDX],
        b"=",
        save.MNCOR1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MXCOR1",
        save.DSKDSC[MX1IDX],
        b"=",
        save.MXCOR1,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MNCOR2",
        save.DSKDSC[MN2IDX],
        b"=",
        save.MNCOR2,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MXCOR2",
        save.DSKDSC[MX2IDX],
        b"=",
        save.MXCOR2,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MNCOR3",
        save.DSKDSC[MN3IDX],
        b"=",
        save.MNCOR3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"MXCOR3",
        save.DSKDSC[MX3IDX],
        b"=",
        save.MXCOR3,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"CORPAR",
        save.DSKDSC.subarray(PARIDX),
        b"=",
        save.CORPAR.as_slice(),
        NSYPAR,
        0.0,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"FIRST",
        save.DSKDSC[BTMIDX],
        b"=",
        save.FIRST,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(b"LAST", save.DSKDSC[ETMIDX], b"=", save.LAST, 0.0, OK, ctx)?;

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

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
