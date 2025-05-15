//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DSKSHP: i32 = 2;
const ELLSHP: i32 = 1;
const MTHLEN: i32 = 500;
const SUBLEN: i32 = 20;
const CVTLEN: i32 = 20;
const TANGNT: i32 = 1;
const GUIDED: i32 = 2;
const TMTLEN: i32 = 20;
const LMBCRV: i32 = 0;
const UMBRAL: i32 = 1;
const PNMBRL: i32 = 2;
const ACLLEN: i32 = 25;
const CTRCOR: i32 = 1;
const ELLCOR: i32 = 2;
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK0: &[u8] = b"tangpt_test0.tpc";
const PCK1: &[u8] = b"tangpt_test0.tpc";
const SPK0: &[u8] = b"tangpt_test0.bsp";
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LOCLEN: i32 = 20;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORLOC: Vec<u8>,
    DREF: Vec<u8>,
    FIXREF: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    ALT: f64,
    DVALS: StackArray<f64, 4>,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    RADII: StackArray<f64, 3>,
    RANGE: f64,
    SRFPT: StackArray<f64, 3>,
    SRFVEC: StackArray<f64, 3>,
    TANPT: StackArray<f64, 3>,
    TRGEPC: f64,
    XALT: f64,
    XRANGE: f64,
    XTANPT: StackArray<f64, 3>,
    XTRGEP: f64,
    XSRFPT: StackArray<f64, 3>,
    XSRFVC: StackArray<f64, 3>,
    HANDLE: i32,
    NRAD: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; CORLEN as usize];
        let mut CORLOC = vec![b' '; LOCLEN as usize];
        let mut DREF = vec![b' '; FRNMLN as usize];
        let mut FIXREF = vec![b' '; FRNMLN as usize];
        let mut METHOD = vec![b' '; MTHLEN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut ALT: f64 = 0.0;
        let mut DVALS = StackArray::<f64, 4>::new(1..=4);
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RANGE: f64 = 0.0;
        let mut SRFPT = StackArray::<f64, 3>::new(1..=3);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut TANPT = StackArray::<f64, 3>::new(1..=3);
        let mut TRGEPC: f64 = 0.0;
        let mut XALT: f64 = 0.0;
        let mut XRANGE: f64 = 0.0;
        let mut XTANPT = StackArray::<f64, 3>::new(1..=3);
        let mut XTRGEP: f64 = 0.0;
        let mut XSRFPT = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut HANDLE: i32 = 0;
        let mut NRAD: i32 = 0;

        Self {
            ABCORR,
            CORLOC,
            DREF,
            FIXREF,
            METHOD,
            OBSRVR,
            TARGET,
            ALT,
            DVALS,
            DVEC,
            ET,
            RADII,
            RANGE,
            SRFPT,
            SRFVEC,
            TANPT,
            TRGEPC,
            XALT,
            XRANGE,
            XTANPT,
            XTRGEP,
            XSRFPT,
            XSRFVC,
            HANDLE,
            NRAD,
        }
    }
}

//$Procedure F_TANGPT0 ( Test tangent point routine, part 0 )
pub fn F_TANGPT0(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Saved variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_TANGPT0", ctx)?;

    //*****************************************************************
    //
    //     Error cases
    //
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup", ctx)?;

    //
    // Create and load LSK, then delete LSK.
    //
    testutil::TSTLSK(ctx)?;

    //
    // Create default test PCK.
    //
    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTPCK(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create test SPK.
    //
    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::TSTSPK(SPK0, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: no ID code for target.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"XMOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: no ID code for observer.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"XEARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: no ID code for observer.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"XEARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: zero-length normal vector.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::CLEARD(3, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: invalid aberration correction.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN-S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error: consecutive calls with blank aberration correction.",
        ctx,
    )?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b" ");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Make sure matching but invalid value of ABCORR is not
    // considered a recognized value.
    //
    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: invalid aberration correction locus.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGPT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTSUPPORTED)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: observer is target.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: target frame not recognized.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOOOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Error: no orientation data available for target frame.",
        ctx,
    )?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"EARTH");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"ITRF93");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: target frame center is not target.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: target radii unavailable.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_EARTH");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::DVPOOL(b"BODY301_RADII", ctx)?;

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //
    // Restore Moon radii.
    //
    spicelib::LDPOOL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: too few target radii; 3 required.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::BODVCD(
        301,
        b"RADII",
        3,
        &mut save.NRAD,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PDPOOL(b"BODY301_RADII", 2, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // Restore Moon radii.
    //
    spicelib::LDPOOL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: too many target radii; 3 required.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    for I in 1..=4 {
        save.DVALS[I] = (I as f64);
    }

    spicelib::PDPOOL(b"BODY301_RADII", 4, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //
    // Restore Moon radii.
    //
    spicelib::LDPOOL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: ray frame not recognized.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"XYZ");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOFRAME)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: no orientation data available for ray frame.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"ITRF93");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(FRAMEDATANOTFOUND)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: no ephemeris data available for observer.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"GASPRA");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: no ephemeris data available for target.", ctx)?;

    //
    // We need to have a target frame available, but no ephemeris data
    // for the target. We need to have ephemeris data for the observer.
    // Pick a target for which we have frame orientation in a generic
    // PCK file.
    //
    // Create test PCK containing data from pck00010.tpc. Clear out
    // existing PCK data first.
    //
    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::CLPOOL(ctx)?;
    testutil::T_PCK10(PCK1, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"GASPRA");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_GASPRA");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // Restore default text kernels.
    //
    testutil::TSTLSK(ctx)?;
    spicelib::LDPOOL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Error: observer is inside target ellipsoid.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    //
    // Make the Moon's radii long enough so the Moon envelops the Earth.
    //
    spicelib::FILLD(1000000.0, 3, save.DVALS.as_slice_mut());

    spicelib::PDPOOL(b"BODY301_RADII", 3, save.DVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDGEOMETRY)", OK, ctx)?;

    //
    // Restore target radii.
    //
    spicelib::LDPOOL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*****************************************************************
    //
    //     Non-error exception cases
    //
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Change observer name-ID association.", ctx)?;

    //
    // Perform normal computation with the Earth as observer.
    //
    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");
    save.ET = 0.0;
    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");
    spicelib::VPACK(1.0, 0.0, 0.0, save.DVEC.as_slice_mut());

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.XTANPT.as_slice_mut(),
        &mut save.XALT,
        &mut save.XRANGE,
        save.XSRFPT.as_slice_mut(),
        &mut save.XTRGEP,
        save.XSRFVC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now map the name JUPITER to the ID code 399.
    //
    spicelib::PCPOOL(b"NAIF_BODY_NAME", 1, CharArray::from_ref(b"JUPITER"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(b"NAIF_BODY_CODE", 1, &[399], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Repeat the call, using JUPITER as the observer name.
    //
    fstr::assign(&mut save.OBSRVR, b"EARTH");

    spicelib::TANGPT(
        &save.METHOD,
        &save.TARGET,
        save.ET,
        &save.FIXREF,
        &save.ABCORR,
        &save.CORLOC,
        &save.OBSRVR,
        &save.DREF,
        save.DVEC.as_slice(),
        save.TANPT.as_slice_mut(),
        &mut save.ALT,
        &mut save.RANGE,
        save.SRFPT.as_slice_mut(),
        &mut save.TRGEPC,
        save.SRFVEC.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ALT", save.ALT, b"=", save.XALT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"RANGE", save.RANGE, b"=", save.XRANGE, 0.0, OK, ctx)?;
    testutil::CHCKAD(
        b"TANPT",
        save.TANPT.as_slice(),
        b"=",
        save.XTANPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"SRFPT",
        save.SRFPT.as_slice(),
        b"=",
        save.XSRFPT.as_slice(),
        3,
        0.0,
        OK,
        ctx,
    )?;

    //
    // TODO: Check handling of invalidation of cached ID codes and other
    // saved values.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(PCK1, ctx)? {
        spicelib::DELFIL(PCK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    if spicelib::EXISTS(SPK0, ctx)? {
        spicelib::SPKUEF(save.HANDLE, ctx)?;
        spicelib::DELFIL(SPK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
