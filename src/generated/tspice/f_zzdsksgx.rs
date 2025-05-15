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
const DSK0: &[u8] = b"zzdsksgx_0.bds";
const PCK0: &[u8] = b"zzdsksgx.tpc";
const NAMLEN: i32 = 32;

struct SaveVars {
    FRNAME: Vec<u8>,
    D: f64,
    DC: StackArray<f64, 1>,
    DSKDSC: StackArray<f64, 24>,
    ET: f64,
    RAYDIR: StackArray<f64, 3>,
    VERTEX: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    BODYID: i32,
    DLADSC: StackArray<i32, 8>,
    DTYPE: i32,
    HANDLE: i32,
    IC: StackArray<i32, 1>,
    NLAT: i32,
    NLON: i32,
    PLID: i32,
    SURFID: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRNAME = vec![b' '; NAMLEN as usize];
        let mut D: f64 = 0.0;
        let mut DC = StackArray::<f64, 1>::new(1..=1);
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut ET: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut BODYID: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut DTYPE: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut IC = StackArray::<i32, 1>::new(1..=1);
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut PLID: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            FRNAME,
            D,
            DC,
            DSKDSC,
            ET,
            RAYDIR,
            VERTEX,
            XPT,
            XXPT,
            BODYID,
            DLADSC,
            DTYPE,
            HANDLE,
            IC,
            NLAT,
            NLON,
            PLID,
            SURFID,
            FOUND,
        }
    }
}

//$Procedure F_ZZDSKSGX ( ZZDSKSGX tests )
pub fn F_ZZDSKSGX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZDSKSGX", ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // The following simple cases are meant to assist debugging.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create tessellated ellipsoid DSK.", ctx)?;

    //
    // Create, load, and discard generic test PCK.
    //
    testutil::TSTPCK(PCK0, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create DSK.
    //
    // TARGET = 'MARS'
    fstr::assign(&mut save.FRNAME, b"IAU_MARS");
    save.BODYID = 499;
    save.SURFID = 1;

    save.NLON = 40;
    save.NLAT = 20;

    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FRNAME,
        save.NLON,
        save.NLAT,
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars segment: check intercept.", ctx)?;

    spicelib::DASOPR(DSK0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DLABFS(
        save.HANDLE,
        save.DLADSC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Descr FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::DSKGD(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DTYPE = intrinsics::IDNINT(save.DSKDSC[TYPIDX]);
    save.ET = ((save.DSKDSC[BTMIDX] + save.DSKDSC[ETMIDX]) / 2 as f64);

    save.D = 1000000.0;
    spicelib::VPACK(
        save.D,
        ((2 as f64) * save.D),
        ((3 as f64) * save.D),
        save.VERTEX.as_slice_mut(),
    );
    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    spicelib::ZZDSKSGX(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DTYPE,
        save.ET,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"xpt FOUND", save.FOUND, true, OK, ctx)?;

    //
    // Compare results to those from DSKX02.
    //
    spicelib::DSKX02(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        &mut save.PLID,
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"xxpt FOUND", save.FOUND, true, OK, ctx)?;

    //
    // We expect an exact match for the intercept.
    //
    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"=",
        save.XXPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check the integer info component. (The d.p. component
    // is undefined.)
    //
    testutil::CHCKAI(b"IC", save.IC.as_slice(), b"=", &[save.PLID], 1, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Mars segment: non-intercept case.", ctx)?;

    //
    // Just point the ray away from the target.
    //
    spicelib::ZZDSKSGX(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DTYPE,
        save.ET,
        save.VERTEX.as_slice(),
        save.VERTEX.as_slice(),
        save.XPT.as_slice_mut(),
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"xpt FOUND", save.FOUND, false, OK, ctx)?;

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Invalid file handle", ctx)?;

    spicelib::ZZDSKSGX(
        (save.HANDLE + 1),
        save.DLADSC.as_slice(),
        save.DTYPE,
        save.ET,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDHANDLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad data type", ctx)?;

    spicelib::ZZDSKSGX(
        (save.HANDLE + 1),
        save.DLADSC.as_slice(),
        -1,
        save.ET,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPT.as_slice_mut(),
        save.DC.as_slice_mut(),
        save.IC.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(TYPENOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    //
    // Close DSK file; open was performed using DASOPR.
    //
    spicelib::DASCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
