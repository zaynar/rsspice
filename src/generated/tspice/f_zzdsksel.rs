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
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;

struct SaveVars {
    BTIME: f64,
    COR1: f64,
    COR2: f64,
    CORPAR: StackArray<f64, 10>,
    DSKDSC: StackArray<f64, 24>,
    ET: f64,
    ETIME: f64,
    POS: StackArray<f64, 3>,
    BODYID: i32,
    CORSYS: i32,
    DCLASS: i32,
    DLADSC: StackArray<i32, 8>,
    FRAMID: i32,
    HANDLE: i32,
    NSURF: i32,
    SURFID: i32,
    SRFLST: StackArray<i32, 100>,
    ISMTCH: bool,
    LGRETV: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BTIME: f64 = 0.0;
        let mut COR1: f64 = 0.0;
        let mut COR2: f64 = 0.0;
        let mut CORPAR = StackArray::<f64, 10>::new(1..=NSYPAR);
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut ET: f64 = 0.0;
        let mut ETIME: f64 = 0.0;
        let mut POS = StackArray::<f64, 3>::new(1..=3);
        let mut BODYID: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut DCLASS: i32 = 0;
        let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut FRAMID: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut SURFID: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut ISMTCH: bool = false;
        let mut LGRETV: bool = false;

        Self {
            BTIME,
            COR1,
            COR2,
            CORPAR,
            DSKDSC,
            ET,
            ETIME,
            POS,
            BODYID,
            CORSYS,
            DCLASS,
            DLADSC,
            FRAMID,
            HANDLE,
            NSURF,
            SURFID,
            SRFLST,
            ISMTCH,
            LGRETV,
        }
    }
}

//$Procedure      F_ZZDSKSEL ( Test the entry points of ZZDSKSEL )
pub fn F_ZZDSKSEL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZDSKSEL", ctx)?;

    //***********************************************************************
    //
    //     ZZDSKSEL umbrella tests
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Direct call to ZZDSKSEL", ctx)?;

    save.LGRETV = spicelib::ZZDSKSEL(
        save.SURFID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.BODYID,
        save.DCLASS,
        save.CORSYS,
        save.CORPAR.as_slice(),
        save.COR1,
        save.COR2,
        save.FRAMID,
        save.POS.as_slice(),
        save.ET,
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //***********************************************************************
    //
    //     ZZDSKSBD/ZZDSKBDC tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZDSKSBD/ZZDSKBDC normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBD/ZZDSKBDC: match case", ctx)?;

    //
    // Initialize handle and DLA descriptor. Neither are
    // used for this comparison.
    //
    save.HANDLE = 0;
    spicelib::CLEARI(DLADSZ, save.DLADSC.as_slice_mut());

    save.BODYID = 599;

    save.LGRETV = spicelib::ZZDSKSBD(save.BODYID, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The DSK descriptor must have a matching body ID code.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    //
    // The body ID index in a DSK descriptor is called 'CTRIDX'.
    //
    save.DSKDSC[CTRIDX] = (save.BODYID as f64);

    save.ISMTCH = spicelib::ZZDSKBDC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBD/ZZDSKBDC: non-match case", ctx)?;

    save.DSKDSC[CTRIDX] = ((save.BODYID + 1) as f64);

    save.ISMTCH = spicelib::ZZDSKBDC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // No error cases exist for these routines.
    //

    //***********************************************************************
    //
    //     ZZDSKSIT/ZZDSKCIT tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZDSKSIT/ZZDSKCIT normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBD/ZZDSKBDC: match case, empty surface list.", ctx)?;

    //
    // Initialize handle and DLA descriptor. Neither are
    // used for this comparison.
    //
    save.HANDLE = 0;
    spicelib::CLEARI(DLADSZ, save.DLADSC.as_slice_mut());

    save.BODYID = 599;
    save.NSURF = 0;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());

    save.ET = 10.0;

    save.LGRETV = spicelib::ZZDSKSIT(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The DSK descriptor must have a matching body ID code.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    save.DSKDSC[CTRIDX] = (save.BODYID as f64);

    //
    // Set time bounds. The bounds must bracket ET.
    //
    save.BTIME = -((50 as f64) * spicelib::JYEAR());
    save.ETIME = ((50 as f64) * spicelib::JYEAR());

    save.DSKDSC[BTMIDX] = save.BTIME;
    save.DSKDSC[ETMIDX] = save.ETIME;

    //
    // The body ID index in a DSK descriptor is called 'CTRIDX'.
    //

    save.ISMTCH = spicelib::ZZDSKCIT(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKSBD/ZZDSKBDC: match case, non-empty surface list.",
        ctx,
    )?;

    //
    // Initialize handle and DLA descriptor. Neither are
    // used for this comparison.
    //
    save.HANDLE = 0;
    spicelib::CLEARI(DLADSZ, save.DLADSC.as_slice_mut());

    save.BODYID = 599;

    spicelib::CLEARI(MAXSRF, save.SRFLST.as_slice_mut());
    save.NSURF = 2;
    save.SRFLST[1] = -1;
    save.SRFLST[2] = -2;

    save.ET = 10.0;

    save.LGRETV = spicelib::ZZDSKSIT(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The DSK descriptor must have a matching body ID code.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    //
    // Set the surface ID code.
    //
    save.DSKDSC[SRFIDX] = save.SRFLST[1] as f64;

    //
    // Set time bounds. The bounds must bracket ET.
    //
    save.DSKDSC[CTRIDX] = (save.BODYID as f64);

    save.BTIME = -((50 as f64) * spicelib::JYEAR());
    save.ETIME = ((50 as f64) * spicelib::JYEAR());

    save.DSKDSC[BTMIDX] = save.BTIME;
    save.DSKDSC[ETMIDX] = save.ETIME;

    //
    // Look for a match.
    //
    save.ISMTCH = spicelib::ZZDSKCIT(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH (-1)", save.ISMTCH, true, OK, ctx)?;

    //
    // Try again with the other surface.
    //
    save.DSKDSC[SRFIDX] = save.SRFLST[2] as f64;

    save.ISMTCH = spicelib::ZZDSKCIT(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH (-2)", save.ISMTCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBD/ZZDSKBDC: non-match case, body mismatch.", ctx)?;

    //
    // Save body ID.
    //
    save.BODYID = intrinsics::IDNINT(save.DSKDSC[CTRIDX]);

    save.DSKDSC[CTRIDX] = 0 as f64;

    save.ISMTCH = spicelib::ZZDSKCIT(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;
    //
    // Restore body ID.
    //
    save.DSKDSC[CTRIDX] = (save.BODYID as f64);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBD/ZZDSKBDC: non-match case, time too early.", ctx)?;

    save.LGRETV = spicelib::ZZDSKSIT(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        (save.BTIME - 1.0),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKCIT(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBD/ZZDSKBDC: non-match case, time too late.", ctx)?;

    save.LGRETV = spicelib::ZZDSKSIT(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        (save.ETIME + 1.0),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKCIT(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKSBD/ZZDSKBDC: non-match case, surface not in list.",
        ctx,
    )?;

    save.LGRETV = spicelib::ZZDSKSIT(save.BODYID, 1, &[-3], save.ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKCIT(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //***********************************************************************
    //
    //     ZZDSKSIT/ZZDSKCIT error cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKSBD/ZZDSKBDC: surface list too big.", ctx)?;

    save.LGRETV = spicelib::ZZDSKSIT(
        save.BODYID,
        (MAXSRF + 1),
        save.SRFLST.as_slice(),
        save.ET,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //***********************************************************************
    //
    //     ZZDSKUSC/ZZDSKUMC tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZDSKUSC/ZZDSKUMC normal cases
    //
    //***********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKUSC/ZZDSKUMC: match case.", ctx)?;

    //
    // Initialize handle and DLA descriptor. Neither are
    // used for this comparison.
    //
    save.HANDLE = 0;
    spicelib::CLEARI(DLADSZ, save.DLADSC.as_slice_mut());

    save.BODYID = 599;

    save.ET = 10.0;

    save.COR1 = (30.0 * spicelib::RPD(ctx));
    save.COR2 = -(5.0 * spicelib::RPD(ctx));

    save.LGRETV = spicelib::ZZDSKUSC(save.BODYID, save.ET, save.COR1, save.COR2, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The DSK descriptor must have a matching body ID code.
    //
    spicelib::CLEARD(DSKDSZ, save.DSKDSC.as_slice_mut());

    save.DSKDSC[CTRIDX] = (save.BODYID as f64);

    //
    // Set time bounds. The bounds must bracket ET.
    //
    save.BTIME = -((50 as f64) * spicelib::JYEAR());
    save.ETIME = ((50 as f64) * spicelib::JYEAR());

    save.DSKDSC[BTMIDX] = save.BTIME;
    save.DSKDSC[ETMIDX] = save.ETIME;

    //
    // Set coordinate system and coordinate bounds.
    //
    save.DSKDSC[SYSIDX] = (LATSYS as f64);

    save.DSKDSC[MN1IDX] = -(90.0 * spicelib::RPD(ctx));
    save.DSKDSC[MX1IDX] = (90.0 * spicelib::RPD(ctx));

    save.DSKDSC[MN2IDX] = -(45.0 * spicelib::RPD(ctx));
    save.DSKDSC[MX2IDX] = (45.0 * spicelib::RPD(ctx));

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKUSC/ZZDSKUMC: non-match case, body mismatch.", ctx)?;

    //
    // Save body ID.
    //
    save.BODYID = intrinsics::IDNINT(save.DSKDSC[CTRIDX]);

    save.DSKDSC[CTRIDX] = 0 as f64;

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;
    //
    // Restore body ID.
    //
    save.DSKDSC[CTRIDX] = (save.BODYID as f64);

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKUSC/ZZDSKUMC: non-match case, time too early.", ctx)?;

    save.LGRETV = spicelib::ZZDSKUSC(
        save.BODYID,
        (save.BTIME - 1 as f64),
        save.COR1,
        save.COR2,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZDSKUSC/ZZDSKUMC: non-match case, time too late.", ctx)?;

    save.LGRETV = spicelib::ZZDSKUSC(
        save.BODYID,
        (save.ETIME + 1 as f64),
        save.COR1,
        save.COR2,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKUSC/ZZDSKUMC: non-match case, coordinate 1 too small",
        ctx,
    )?;

    save.LGRETV = spicelib::ZZDSKUSC(
        save.BODYID,
        save.ET,
        (save.DSKDSC[MN1IDX] - 0.1),
        save.COR2,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKUSC/ZZDSKUMC: non-match case, coordinate 1 too large",
        ctx,
    )?;

    save.LGRETV = spicelib::ZZDSKUSC(
        save.BODYID,
        save.ET,
        (save.DSKDSC[MX1IDX] + 0.1),
        save.COR2,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKUSC/ZZDSKUMC: non-match case, coordinate 2 too small",
        ctx,
    )?;

    save.LGRETV = spicelib::ZZDSKUSC(
        save.BODYID,
        save.ET,
        save.COR1,
        (save.DSKDSC[MN2IDX] - 0.1),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZDSKUSC/ZZDSKUMC: non-match case, coordinate 2 too large",
        ctx,
    )?;

    save.LGRETV = spicelib::ZZDSKUSC(
        save.BODYID,
        save.ET,
        save.COR1,
        (save.DSKDSC[MX2IDX] + 0.1),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ISMTCH = spicelib::ZZDSKUMC(
        save.HANDLE,
        save.DLADSC.as_slice(),
        save.DSKDSC.as_slice(),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISMTCH", save.ISMTCH, false, OK, ctx)?;

    //
    // No error cases exist for these routines.
    //

    //***********************************************************************
    //
    //     ZZDSKMSC/ZZDSKMMC tests
    //
    //***********************************************************************

    //***********************************************************************
    //
    //     ZZDSKMSC/ZZDSKMMC normal cases
    //
    //***********************************************************************

    //
    // TBD
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
