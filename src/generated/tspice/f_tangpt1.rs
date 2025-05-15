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
const SPK0: &[u8] = b"tangpt_test0.bsp";
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LOCLEN: i32 = 20;
const TIGHT: f64 = 0.000000000001;
const VTIGHT: f64 = 0.00000000000001;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORLOC: Vec<u8>,
    DREF: Vec<u8>,
    FIXREF: Vec<u8>,
    METHOD: Vec<u8>,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    ALT: f64,
    DVEC: StackArray<f64, 3>,
    ET: f64,
    FIXDIR: StackArray<f64, 3>,
    FIXOBS: StackArray<f64, 3>,
    FIXSTL: StackArray<f64, 3>,
    FIXTAN: StackArray<f64, 3>,
    FIXTRG: StackArray<f64, 3>,
    H: f64,
    J2OBS: StackArray<f64, 3>,
    LT: f64,
    NORMAL: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    RANGE: f64,
    SEP: f64,
    SRFPT: StackArray<f64, 3>,
    SRFTAN: StackArray<f64, 3>,
    SRFVEC: StackArray<f64, 3>,
    SSBOBS: StackArray<f64, 6>,
    SSBTRG: StackArray<f64, 6>,
    STLOFF: StackArray<f64, 3>,
    TANPT: StackArray<f64, 3>,
    TANSTA: StackArray<f64, 6>,
    TANST1: StackArray<f64, 6>,
    TMPPOS: StackArray<f64, 3>,
    TOL: f64,
    TRGEPC: f64,
    TRGSTA: StackArray<f64, 6>,
    XALT: f64,
    XEPOCH: f64,
    XFORM: StackArray2D<f64, 9>,
    XRANGE: f64,
    XSRFPT: StackArray<f64, 3>,
    XSRFVC: StackArray<f64, 3>,
    XTANPT: StackArray<f64, 3>,
    ZVEC: StackArray<f64, 3>,
    HANDLE: i32,
    NRAD: i32,
    OBSCDE: i32,
    TRGCDE: i32,
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
        let mut DVEC = StackArray::<f64, 3>::new(1..=3);
        let mut ET: f64 = 0.0;
        let mut FIXDIR = StackArray::<f64, 3>::new(1..=3);
        let mut FIXOBS = StackArray::<f64, 3>::new(1..=3);
        let mut FIXSTL = StackArray::<f64, 3>::new(1..=3);
        let mut FIXTAN = StackArray::<f64, 3>::new(1..=3);
        let mut FIXTRG = StackArray::<f64, 3>::new(1..=3);
        let mut H: f64 = 0.0;
        let mut J2OBS = StackArray::<f64, 3>::new(1..=3);
        let mut LT: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RANGE: f64 = 0.0;
        let mut SEP: f64 = 0.0;
        let mut SRFPT = StackArray::<f64, 3>::new(1..=3);
        let mut SRFTAN = StackArray::<f64, 3>::new(1..=3);
        let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
        let mut SSBOBS = StackArray::<f64, 6>::new(1..=6);
        let mut SSBTRG = StackArray::<f64, 6>::new(1..=6);
        let mut STLOFF = StackArray::<f64, 3>::new(1..=3);
        let mut TANPT = StackArray::<f64, 3>::new(1..=3);
        let mut TANSTA = StackArray::<f64, 6>::new(1..=6);
        let mut TANST1 = StackArray::<f64, 6>::new(1..=6);
        let mut TMPPOS = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut TRGEPC: f64 = 0.0;
        let mut TRGSTA = StackArray::<f64, 6>::new(1..=6);
        let mut XALT: f64 = 0.0;
        let mut XEPOCH: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XRANGE: f64 = 0.0;
        let mut XSRFPT = StackArray::<f64, 3>::new(1..=3);
        let mut XSRFVC = StackArray::<f64, 3>::new(1..=3);
        let mut XTANPT = StackArray::<f64, 3>::new(1..=3);
        let mut ZVEC = StackArray::<f64, 3>::new(1..=3);
        let mut HANDLE: i32 = 0;
        let mut NRAD: i32 = 0;
        let mut OBSCDE: i32 = 0;
        let mut TRGCDE: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            ZVEC.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORLOC,
            DREF,
            FIXREF,
            METHOD,
            OBSRVR,
            TARGET,
            ALT,
            DVEC,
            ET,
            FIXDIR,
            FIXOBS,
            FIXSTL,
            FIXTAN,
            FIXTRG,
            H,
            J2OBS,
            LT,
            NORMAL,
            RADII,
            RANGE,
            SEP,
            SRFPT,
            SRFTAN,
            SRFVEC,
            SSBOBS,
            SSBTRG,
            STLOFF,
            TANPT,
            TANSTA,
            TANST1,
            TMPPOS,
            TOL,
            TRGEPC,
            TRGSTA,
            XALT,
            XEPOCH,
            XFORM,
            XRANGE,
            XSRFPT,
            XSRFVC,
            XTANPT,
            ZVEC,
            HANDLE,
            NRAD,
            OBSCDE,
            TRGCDE,
        }
    }
}

//$Procedure F_TANGPT1 ( Test tangent point routine, part 1 )
pub fn F_TANGPT1(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Elevation units are radians.
    //
    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_TANGPT1", ctx)?;

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

    //*****************************************************************
    //
    //     Normal cases
    //
    //*****************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple case 1: Earth observes Moon, abcorr = NONE. Ray points toward a point 100 km above the Moon\'s north pole. Use tangent point locus. Use modified Moon radii: make the shape a true triaxial ellipsoid.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");

    //
    // Don't use ET = 0.D0 for testing expected results; that time
    // value could hide uninitialized time errors.
    //
    save.ET = 31557600.0;

    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");

    //
    // Get radii of Moon. Replace radii with values giving a true
    // triaxial shape: radii are a, a/2, a/3.
    //
    spicelib::BODVRD(
        b"MOON",
        b"RADII",
        3,
        &mut save.NRAD,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RADII[2] = (save.RADII[1] / 2 as f64);
    save.RADII[3] = (save.RADII[1] / 3 as f64);

    spicelib::PDPOOL(b"BODY301_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create target point FIXTRG in the body-fixed frame.
    //
    save.H = 100.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.ZVEC.as_slice_mut());
    spicelib::VSCL(
        (save.RADII[3] + save.H),
        save.ZVEC.as_slice(),
        save.FIXTRG.as_slice_mut(),
    );

    //
    // Compute the state of the target point relative to the observer
    // in the target body-fixed frame, evaluated at the target epoch.
    // Return the state expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.FIXTRG.as_slice(),
        b"MOON",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TRGSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.TRGSTA.as_slice(), save.DVEC.as_slice_mut());

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

    //
    // Convert the ray direction from the J2000 frame to the body-fixed
    // frame at the target epoch.
    //
    spicelib::PXFORM(
        &save.DREF,
        &save.FIXREF,
        save.TRGEPC,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.DVEC.as_slice(),
        save.FIXDIR.as_slice_mut(),
    );

    //
    // Compute the vector from the observer to the tangent point in
    // the body-fixed frame at the target epoch.
    //
    spicelib::VLCOM3(
        1.0,
        save.SRFVEC.as_slice(),
        -1.0,
        save.SRFPT.as_slice(),
        1.0,
        save.TANPT.as_slice(),
        save.FIXTAN.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check angular separation of ray direction and direction
    // to tangent point.
    //
    save.SEP = spicelib::VSEP(save.FIXDIR.as_slice(), save.FIXTAN.as_slice(), ctx);

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"RAY-OBS-TO-TANPT SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Verify the zenith direction is normal to the ray direction.
    //
    save.SEP = spicelib::VSEP(save.NORMAL.as_slice(), save.FIXDIR.as_slice(), ctx);

    testutil::CHCKSD(
        b"RAY ZENITH SEP",
        save.SEP,
        b"~",
        spicelib::HALFPI(ctx),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the altitude of the tangent point ALT.
    //
    save.XALT = spicelib::VDIST(save.SRFPT.as_slice(), save.TANPT.as_slice());

    //
    // Use the range from the observer to the tangent point to scale
    // the tolerance for the altitude check.
    //
    save.TOL = (TIGHT * save.RANGE);

    testutil::CHCKSD(b"ALT", save.ALT, b"~", save.XALT, save.TOL, OK, ctx)?;

    //
    // Get the position of the observer relative to the target center,
    // expressed in the target body-fixed frame. This call works only
    // for the geometric case.
    //
    spicelib::SPKPOS(
        &save.OBSRVR,
        save.ET,
        &save.FIXREF,
        b"NONE",
        &save.TARGET,
        save.FIXOBS.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check SRFVEC.
    //
    spicelib::VSUB(
        save.SRFPT.as_slice(),
        save.FIXOBS.as_slice(),
        save.XSRFVC.as_slice_mut(),
    );

    save.TOL = 0.0000000000001;

    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"~~/",
        save.XSRFVC.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check range to the tangent point.
    //
    save.XRANGE = spicelib::VDIST(save.TANPT.as_slice(), save.FIXOBS.as_slice());

    save.TOL = 0.0000000000001;

    testutil::CHCKSD(b"RANGE", save.RANGE, b"~/", save.XRANGE, save.TOL, OK, ctx)?;

    //
    // Check the target epoch. In this case, it should match ET.
    //
    save.XEPOCH = save.ET;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"~", save.XEPOCH, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple case 2: Earth observes Moon, abcorr = NONE. Ray points toward a point 100 km above the Moon\'s north pole. Use surface point locus.", ctx)?;

    //
    // Because we're using geometric computations, all output quantities
    // should match those from the previous case.
    //
    save.XEPOCH = save.TRGEPC;
    save.XALT = save.ALT;
    save.XRANGE = save.RANGE;

    spicelib::VEQU(save.TANPT.as_slice(), save.XTANPT.as_slice_mut());
    spicelib::VEQU(save.SRFPT.as_slice(), save.XSRFPT.as_slice_mut());
    spicelib::VEQU(save.SRFVEC.as_slice(), save.XSRFVC.as_slice_mut());

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");

    //
    // Don't use ET = 0.D0 for testing expected results; that time
    // value could hide uninitialized time errors.
    //
    save.ET = 31557600.0;

    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.CORLOC, b"SURFACE POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");

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

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"RANGE", save.RANGE, b"~/", save.XRANGE, save.TOL, OK, ctx)?;
    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"~", save.XEPOCH, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"~~/",
        save.XSRFVC.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    save.TOL = (TIGHT * save.RANGE);

    testutil::CHCKSD(b"ALT", save.ALT, b"~", save.XALT, save.TOL, OK, ctx)?;

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"TANPT",
        save.TANPT.as_slice(),
        b"~~/",
        save.XTANPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"SRFPT",
        save.SRFPT.as_slice(),
        b"~~/",
        save.XSRFPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple case 3: Earth observes Moon. ABCORR = CN. Ray points toward a point 100 km above the Moon\'s north pole. Use tangent point locus. Use modified Moon radii: make the shape a true triaxial ellipsoid. The geometric position of the Moon is used to compute the ray\'s direction vector. Use surface point locus.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");

    //
    // Don't use ET = 0.D0 for testing expected results; that could hide
    // uninitialized time errors.
    //
    save.ET = 31557600.0;

    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");

    //
    // Get radii of Moon. Replace radii with values giving a true
    // triaxial shape: radii are a, a/2, a/3.
    //
    spicelib::BODVRD(
        b"MOON",
        b"RADII",
        3,
        &mut save.NRAD,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RADII[2] = (save.RADII[1] / 2 as f64);
    save.RADII[3] = (save.RADII[1] / 3 as f64);

    spicelib::PDPOOL(b"BODY301_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create target point FIXTRG in the body-fixed frame.
    //
    save.H = 100.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.ZVEC.as_slice_mut());
    spicelib::VSCL(
        (save.RADII[3] + save.H),
        save.ZVEC.as_slice(),
        save.FIXTRG.as_slice_mut(),
    );

    //
    // Compute the state of the target point relative to the observer
    // in the target body-fixed frame, evaluated at the target epoch.
    // Return the state expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.FIXTRG.as_slice(),
        b"MOON",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TRGSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.TRGSTA.as_slice(), save.DVEC.as_slice_mut());

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

    //
    // Convert the ray direction from the J2000 frame to the body-fixed
    // frame at the target epoch.
    //
    spicelib::PXFORM(
        &save.DREF,
        &save.FIXREF,
        save.TRGEPC,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.DVEC.as_slice(),
        save.FIXDIR.as_slice_mut(),
    );

    //
    // Compute the vector from the observer to the tangent point in
    // the body-fixed frame at the target epoch.
    //
    spicelib::VLCOM3(
        1.0,
        save.SRFVEC.as_slice(),
        -1.0,
        save.SRFPT.as_slice(),
        1.0,
        save.TANPT.as_slice(),
        save.FIXTAN.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check angular separation of ray direction and direction
    // to tangent point.
    //
    save.SEP = spicelib::VSEP(save.FIXDIR.as_slice(), save.FIXTAN.as_slice(), ctx);
    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"RAY-OBS-TO-TANPT SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Verify the zenith direction is normal to the ray direction.
    //
    save.SEP = spicelib::VSEP(save.NORMAL.as_slice(), save.FIXDIR.as_slice(), ctx);

    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"RAY ZENITH SEP",
        save.SEP,
        b"~",
        spicelib::HALFPI(ctx),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check ALT.
    //
    save.XALT = spicelib::VDIST(save.SRFPT.as_slice(), save.TANPT.as_slice());

    //
    // Use the range from the observer to the tangent point to scale
    // the tolerance for the altitude check.
    //
    save.TOL = (TIGHT * save.RANGE);

    testutil::CHCKSD(b"ALT", save.ALT, b"~", save.XALT, save.TOL, OK, ctx)?;

    //
    // Get the position of the observer relative to the target center,
    // expressed in the target body-fixed frame. We must compute this
    // vector using the position of the observer relative to the solar
    // system barycenter at ET and the position of the target relative
    // to the solar system barycenter at TRGEPC.
    //
    save.OBSCDE = 399;
    save.TRGCDE = 301;

    spicelib::SPKSSB(
        save.OBSCDE,
        save.ET,
        &save.DREF,
        save.SSBOBS.as_slice_mut(),
        ctx,
    )?;
    spicelib::SPKSSB(
        save.TRGCDE,
        save.TRGEPC,
        &save.DREF,
        save.SSBTRG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.SSBOBS.as_slice(),
        save.SSBTRG.as_slice(),
        save.J2OBS.as_slice_mut(),
    );
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.J2OBS.as_slice(),
        save.FIXOBS.as_slice_mut(),
    );

    //
    // Check SRFVEC.
    //
    spicelib::VSUB(
        save.SRFPT.as_slice(),
        save.FIXOBS.as_slice(),
        save.XSRFVC.as_slice_mut(),
    );

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"~~/",
        save.XSRFVC.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;
    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"~~",
        save.XSRFVC.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check range to the tangent point.
    //
    save.XRANGE = spicelib::VDIST(save.TANPT.as_slice(), save.FIXOBS.as_slice());

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"RANGE", save.RANGE, b"~/", save.XRANGE, save.TOL, OK, ctx)?;

    //
    // Check the target epoch.
    //
    save.XEPOCH = (save.ET - (save.RANGE / spicelib::CLIGHT()));

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"~", save.XEPOCH, save.TOL, OK, ctx)?;

    //
    // Find the aberration-corrected position of the tangent point
    // relative to the observer, expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.TANPT.as_slice(),
        b"MOON",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TANSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the angular separation of the observer-tangent point
    // vector with the original ray direction.
    //
    save.SEP = spicelib::VSEP(save.TANSTA.as_slice(), save.DVEC.as_slice(), ctx);

    save.TOL = 0.0000000000001;

    testutil::CHCKSD(b"TANSTA-DVEC SEP", save.SEP, b"~", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple case 4: Earth observes Moon, abcorr = CN+S. Ray points toward a point 100 km above the Moon\'s north pole. Use tangent point locus. Use modified Moon radii: make the shape a true triaxial ellipsoid.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"MOON");

    //
    // Don't use ET = 0.D0 for testing expected results; that could hide
    // uninitialized time errors.
    //
    save.ET = 31557600.0;

    fstr::assign(&mut save.FIXREF, b"IAU_MOON");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.DREF, b"J2000");

    //
    // Get radii of Moon. Replace radii with values giving a true
    // triaxial shape: radii are a, a/2, a/3.
    //
    spicelib::BODVRD(
        b"MOON",
        b"RADII",
        3,
        &mut save.NRAD,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RADII[2] = (save.RADII[1] / 2 as f64);
    save.RADII[3] = (save.RADII[1] / 3 as f64);

    spicelib::PDPOOL(b"BODY301_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create target point FIXTRG in the body-fixed frame.
    //
    save.H = 100.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.ZVEC.as_slice_mut());
    spicelib::VSCL(
        (save.RADII[3] + save.H),
        save.ZVEC.as_slice(),
        save.FIXTRG.as_slice_mut(),
    );

    //
    // Compute the state of the target point relative to the observer
    // in the target body-fixed frame, evaluated at the target epoch.
    // Return the state expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.FIXTRG.as_slice(),
        b"MOON",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TRGSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.TRGSTA.as_slice(), save.DVEC.as_slice_mut());

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

    //
    // Convert the ray direction from the J2000 frame to the body-fixed
    // frame at the target epoch.
    //
    spicelib::PXFORM(
        &save.DREF,
        &save.FIXREF,
        save.TRGEPC,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.DVEC.as_slice(),
        save.FIXDIR.as_slice_mut(),
    );

    //
    // Compute the vector from the observer to the tangent point in
    // the body-fixed frame at the target epoch.
    //
    spicelib::VLCOM3(
        1.0,
        save.SRFVEC.as_slice(),
        -1.0,
        save.SRFPT.as_slice(),
        1.0,
        save.TANPT.as_slice(),
        save.FIXTAN.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check angular separation of ray direction and direction
    // to tangent point.
    //
    save.SEP = spicelib::VSEP(save.FIXDIR.as_slice(), save.FIXTAN.as_slice(), ctx);

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"RAY-OBS-TO-TANPT SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Verify the zenith direction is normal to the ray direction.
    //
    save.SEP = spicelib::VSEP(save.NORMAL.as_slice(), save.FIXDIR.as_slice(), ctx);

    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"RAY ZENITH SEP",
        save.SEP,
        b"~",
        spicelib::HALFPI(ctx),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check ALT.
    //
    save.XALT = spicelib::VDIST(save.SRFPT.as_slice(), save.TANPT.as_slice());

    //
    // Use the range from the observer to the tangent point to scale
    // the tolerance for the altitude check.
    //
    save.TOL = (TIGHT * save.RANGE);

    testutil::CHCKSD(b"ALT", save.ALT, b"~", save.XALT, save.TOL, OK, ctx)?;

    //
    // Prepare to perform a consistency check on SRFVEC. We'll
    // use the target epoch associated with the tangent point to
    // compute the target body's position and orientation. We'll also
    // use the stellar aberration correction of the tangent point to
    // shift the target position. The target body's position and
    // orientation, along with the surface point, will be used to
    // compute an expected value of SRFVEC.
    //
    // Get the position of the observer relative to the target center,
    // expressed in the target body-fixed frame. We must compute this
    // vector using the position of the observer relative to the solar
    // system barycenter at ET and the position of the target relative
    // to the solar system barycenter at TRGEPC.
    //
    // This computation accounts for light time between the observer
    // and the tangent point. It does not account for stellar
    // aberration. That will be done separately.
    //
    save.OBSCDE = 399;
    save.TRGCDE = 301;

    spicelib::SPKSSB(
        save.OBSCDE,
        save.ET,
        &save.DREF,
        save.SSBOBS.as_slice_mut(),
        ctx,
    )?;
    spicelib::SPKSSB(
        save.TRGCDE,
        save.TRGEPC,
        &save.DREF,
        save.SSBTRG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.SSBOBS.as_slice(),
        save.SSBTRG.as_slice(),
        save.J2OBS.as_slice_mut(),
    );
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.J2OBS.as_slice(),
        save.FIXOBS.as_slice_mut(),
    );

    //
    // Now adjust the position of the observer relative to the target
    // to take into account the shift of the target by the stellar
    // aberration offset of the tangent point. We can get this offset
    // by computing the state of the tangent point with and without
    // stellar aberration correction.
    //
    // Find the aberration-corrected position of the tangent point
    // relative to the observer, expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.TANPT.as_slice(),
        b"MOON",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TANSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the light time-corrected position of the tangent point
    // relative to the observer, expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.TANPT.as_slice(),
        b"MOON",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        b"CN",
        &save.OBSRVR,
        save.TANST1.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the stellar aberration offset of the tangent point,
    // expressed in the J2000 frame. Convert to the body-fixed frame
    // at epoch TRGEPC.
    //
    spicelib::VSUB(
        save.TANSTA.as_slice(),
        save.TANST1.as_slice(),
        save.STLOFF.as_slice_mut(),
    );

    spicelib::MXV(
        save.XFORM.as_slice(),
        save.STLOFF.as_slice(),
        save.FIXSTL.as_slice_mut(),
    );

    //
    // Now compute the expected observer-to-surface vector, accounting
    // for the aberration corrections applicable to the tangent point.
    //
    spicelib::VSUB(
        save.SRFPT.as_slice(),
        save.FIXOBS.as_slice(),
        save.TMPPOS.as_slice_mut(),
    );
    spicelib::VADD(
        save.FIXSTL.as_slice(),
        save.TMPPOS.as_slice(),
        save.XSRFVC.as_slice_mut(),
    );

    //
    // Check SRFVEC.
    //
    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"~~/",
        save.XSRFVC.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check range to the tangent point using the tangent point state.
    //
    save.XRANGE = spicelib::VDIST(save.TANPT.as_slice(), save.FIXOBS.as_slice());

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"RANGE", save.RANGE, b"~/", save.XRANGE, save.TOL, OK, ctx)?;

    //
    // Check the target epoch using the tangent point state.
    // It should match ET-LT, where LT is light time between the
    // tangent point and the observer.
    //
    save.XEPOCH = (save.ET - (save.RANGE / spicelib::CLIGHT()));

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"~", save.XEPOCH, save.TOL, OK, ctx)?;

    //
    // Compute the angular separation of the observer-tangent point
    // vector with the original ray direction. This differs from the
    // earlier test in that this time, the vector from the observer
    // to the tangent point was computed by SPKCPT.
    //
    save.SEP = spicelib::VSEP(save.TANSTA.as_slice(), save.DVEC.as_slice(), ctx);

    save.TOL = 0.0000000000001;

    testutil::CHCKSD(b"TANSTA-DVEC SEP", save.SEP, b"~", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple case 5: Phobos observes Mars, abcorr = NONE. Ray points toward a point 100 km above Mars\'s north pole. Use tangent point locus. Use modified Mars radii: make the shape a true triaxial ellipsoid.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"Mars");

    //
    // Don't use ET = 0.D0 for testing expected results; that could hide
    // uninitialized time errors.
    //
    save.ET = 31557600.0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"Phobos");
    fstr::assign(&mut save.DREF, b"J2000");

    //
    // Get radii of Mars. Replace radii with values giving a true
    // triaxial shape: radii are a, a/2, a/3.
    //
    spicelib::BODVRD(
        b"MARS",
        b"RADII",
        3,
        &mut save.NRAD,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RADII[2] = (save.RADII[1] / 2 as f64);
    save.RADII[3] = (save.RADII[1] / 3 as f64);

    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create target point FIXTRG in the body-fixed frame.
    //
    save.H = 100.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.ZVEC.as_slice_mut());
    spicelib::VSCL(
        (save.RADII[3] + save.H),
        save.ZVEC.as_slice(),
        save.FIXTRG.as_slice_mut(),
    );

    //
    // Compute the state of the target point relative to the observer
    // in the target body-fixed frame, evaluated at the target epoch.
    // Return the state expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.FIXTRG.as_slice(),
        b"MARS",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TRGSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.TRGSTA.as_slice(), save.DVEC.as_slice_mut());

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

    //
    // Convert the ray direction from the J2000 frame to the body-fixed
    // frame at the target epoch.
    //
    spicelib::PXFORM(
        &save.DREF,
        &save.FIXREF,
        save.TRGEPC,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.DVEC.as_slice(),
        save.FIXDIR.as_slice_mut(),
    );

    //
    // Compute the vector from the observer to the tangent point in
    // the body-fixed frame at the target epoch.
    //
    spicelib::VLCOM3(
        1.0,
        save.SRFVEC.as_slice(),
        -1.0,
        save.SRFPT.as_slice(),
        1.0,
        save.TANPT.as_slice(),
        save.FIXTAN.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check angular separation of ray direction and direction
    // to tangent point.
    //
    save.SEP = spicelib::VSEP(save.FIXDIR.as_slice(), save.FIXTAN.as_slice(), ctx);

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"RAY-OBS-TO-TANPT SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Verify the zenith direction is normal to the ray direction.
    //
    save.SEP = spicelib::VSEP(save.NORMAL.as_slice(), save.FIXDIR.as_slice(), ctx);

    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"RAY ZENITH SEP",
        save.SEP,
        b"~",
        spicelib::HALFPI(ctx),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check ALT.
    //
    save.XALT = spicelib::VDIST(save.SRFPT.as_slice(), save.TANPT.as_slice());

    //
    // Use the range from the observer to the tangent point to scale
    // the tolerance for the altitude check.
    //
    save.TOL = (TIGHT * save.RANGE);

    testutil::CHCKSD(b"ALT", save.ALT, b"~", save.XALT, save.TOL, OK, ctx)?;

    //
    // Get the position of the observer relative to the target center,
    // expressed in the target body-fixed frame. This call works only
    // for the geometric case.
    //
    spicelib::SPKPOS(
        &save.OBSRVR,
        save.ET,
        &save.FIXREF,
        b"NONE",
        &save.TARGET,
        save.FIXOBS.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check SRFVEC.
    //
    spicelib::VSUB(
        save.SRFPT.as_slice(),
        save.FIXOBS.as_slice(),
        save.XSRFVC.as_slice_mut(),
    );

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"~~/",
        save.XSRFVC.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check range to the tangent point.
    //
    save.XRANGE = spicelib::VDIST(save.TANPT.as_slice(), save.FIXOBS.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"RANGE", save.RANGE, b"~/", save.XRANGE, save.TOL, OK, ctx)?;

    //
    // Check the target epoch. In this case, it should match ET.
    //
    save.XEPOCH = save.ET;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"~", save.XEPOCH, save.TOL, OK, ctx)?;

    //
    // Compute the angular separation of the observer-tangent point
    // vector with the original ray direction.
    //
    spicelib::SPKCPT(
        save.TANPT.as_slice(),
        &save.TARGET,
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TANSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.SEP = spicelib::VSEP(save.TANSTA.as_slice(), save.DVEC.as_slice(), ctx);

    save.TOL = TIGHT;

    testutil::CHCKSD(b"TANSTA-DVEC SEP", save.SEP, b"~", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Simple case 6: Phobos observes Mars, abcorr = CN+S. Ray points toward a point 100 km above Mars\'s north pole. Use tangent point locus. Use modified Mars radii: make the shape a true triaxial ellipsoid.", ctx)?;

    fstr::assign(&mut save.METHOD, b"ELLIPSOID");
    fstr::assign(&mut save.TARGET, b"Mars");

    //
    // Don't use ET = 0.D0 for testing expected results; that could hide
    // uninitialized time errors.
    //
    save.ET = 31557600.0;

    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    fstr::assign(&mut save.ABCORR, b"CN+S");
    fstr::assign(&mut save.CORLOC, b"TANGENT POINT");
    fstr::assign(&mut save.OBSRVR, b"Phobos");
    fstr::assign(&mut save.DREF, b"J2000");

    //
    // Get radii of Mars. Replace radii with values giving a true
    // triaxial shape: radii are a, a/2, a/3.
    //
    spicelib::BODVRD(
        b"MARS",
        b"RADII",
        3,
        &mut save.NRAD,
        save.RADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.RADII[2] = (save.RADII[1] / 2 as f64);
    save.RADII[3] = (save.RADII[1] / 3 as f64);

    spicelib::PDPOOL(b"BODY499_RADII", 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create target point FIXTRG in the body-fixed frame.
    //
    save.H = 100.0;

    spicelib::VPACK(0.0, 0.0, 1.0, save.ZVEC.as_slice_mut());
    spicelib::VSCL(
        (save.RADII[3] + save.H),
        save.ZVEC.as_slice(),
        save.FIXTRG.as_slice_mut(),
    );

    //
    // Compute the state of the target point relative to the observer
    // in the target body-fixed frame, evaluated at the target epoch.
    // Return the state expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.FIXTRG.as_slice(),
        b"MARS",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TRGSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VEQU(save.TRGSTA.as_slice(), save.DVEC.as_slice_mut());

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

    //
    // Convert the ray direction from the J2000 frame to the body-fixed
    // frame at the target epoch.
    //
    spicelib::PXFORM(
        &save.DREF,
        &save.FIXREF,
        save.TRGEPC,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.DVEC.as_slice(),
        save.FIXDIR.as_slice_mut(),
    );

    //
    // Compute the vector from the observer to the tangent point in
    // the body-fixed frame at the target epoch.
    //
    spicelib::VLCOM3(
        1.0,
        save.SRFVEC.as_slice(),
        -1.0,
        save.SRFPT.as_slice(),
        1.0,
        save.TANPT.as_slice(),
        save.FIXTAN.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check angular separation of ray direction and direction
    // to tangent point.
    //
    save.SEP = spicelib::VSEP(save.FIXDIR.as_slice(), save.FIXTAN.as_slice(), ctx);

    save.TOL = VTIGHT;

    testutil::CHCKSD(
        b"RAY-OBS-TO-TANPT SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);
    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Verify the zenith direction is normal to the ray direction.
    //
    save.SEP = spicelib::VSEP(save.NORMAL.as_slice(), save.FIXDIR.as_slice(), ctx);

    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"RAY ZENITH SEP",
        save.SEP,
        b"~",
        spicelib::HALFPI(ctx),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the position of the tangent point above the surface point.
    //
    spicelib::SURFNM(
        save.RADII[1],
        save.RADII[2],
        save.RADII[3],
        save.SRFPT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.TANPT.as_slice(),
        save.SRFPT.as_slice(),
        save.SRFTAN.as_slice_mut(),
    );

    save.SEP = spicelib::VSEP(save.SRFTAN.as_slice(), save.NORMAL.as_slice(), ctx);

    //
    // Use looser tolerance here since the relative errors in TANPT
    // and SRFPT are larger compared to the lengths of those vectors
    // than they are relative to the observer-target distance.
    //
    save.TOL = TIGHT;

    testutil::CHCKSD(
        b"SRFPT-TANPT ZENITH SEP",
        save.SEP,
        b"~",
        0.0,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check ALT.
    //
    save.XALT = spicelib::VDIST(save.SRFPT.as_slice(), save.TANPT.as_slice());

    save.TOL = (TIGHT * save.RANGE);

    testutil::CHCKSD(b"ALT", save.ALT, b"~", save.XALT, save.TOL, OK, ctx)?;

    //
    // Prepare to perform a consistency check on SRFVEC. We'll
    // use the target epoch associated with the tangent point to
    // compute the target body's position and orientation. We'll also
    // use the stellar aberration correction of the tangent point to
    // shift the target position. The target body's position and
    // orientation, along with the surface point, will be used to
    // compute an expected value of SRFVEC.
    //
    // Get the position of the observer relative to the target center,
    // expressed in the target body-fixed frame. We must compute this
    // vector using the position of the observer relative to the solar
    // system barycenter at ET and the position of the target relative
    // to the solar system barycenter at TRGEPC.
    //
    // This computation accounts for light time between the observer
    // and the tangent point. It does not account for stellar
    // aberration. That will be done separately.
    //
    save.OBSCDE = 401;
    save.TRGCDE = 499;

    spicelib::SPKSSB(
        save.OBSCDE,
        save.ET,
        &save.DREF,
        save.SSBOBS.as_slice_mut(),
        ctx,
    )?;
    spicelib::SPKSSB(
        save.TRGCDE,
        save.TRGEPC,
        &save.DREF,
        save.SSBTRG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSUB(
        save.SSBOBS.as_slice(),
        save.SSBTRG.as_slice(),
        save.J2OBS.as_slice_mut(),
    );
    spicelib::MXV(
        save.XFORM.as_slice(),
        save.J2OBS.as_slice(),
        save.FIXOBS.as_slice_mut(),
    );

    //
    // Now adjust the position of the observer relative to the target
    // to take into account the shift of the target by the stellar
    // aberration offset of the tangent point. We can get this offset
    // by computing the state of the tangent point with and without
    // stellar aberration correction.
    //
    // Find the aberration-corrected position of the tangent point
    // relative to the observer, expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.TANPT.as_slice(),
        b"MARS",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        &save.ABCORR,
        &save.OBSRVR,
        save.TANSTA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the light time-corrected position of the tangent point
    // relative to the observer, expressed in the J2000 frame.
    //
    spicelib::SPKCPT(
        save.TANPT.as_slice(),
        b"MARS",
        &save.FIXREF,
        save.ET,
        &save.DREF,
        b"TARGET",
        b"CN",
        &save.OBSRVR,
        save.TANST1.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find the stellar aberration offset of the tangent point,
    // expressed in the J2000 frame. Convert to the body-fixed frame
    // at epoch TRGEPC.
    //
    spicelib::VSUB(
        save.TANSTA.as_slice(),
        save.TANST1.as_slice(),
        save.STLOFF.as_slice_mut(),
    );

    spicelib::MXV(
        save.XFORM.as_slice(),
        save.STLOFF.as_slice(),
        save.FIXSTL.as_slice_mut(),
    );

    //
    // Now compute the expected observer-to-surface vector, accounting
    // for the aberration corrections applicable to the tangent point.
    //
    spicelib::VSUB(
        save.SRFPT.as_slice(),
        save.FIXOBS.as_slice(),
        save.TMPPOS.as_slice_mut(),
    );
    spicelib::VADD(
        save.FIXSTL.as_slice(),
        save.TMPPOS.as_slice(),
        save.XSRFVC.as_slice_mut(),
    );

    //
    // Check SRFVEC.
    //
    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"SRFVEC",
        save.SRFVEC.as_slice(),
        b"~~/",
        save.XSRFVC.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check range to the tangent point using the tangent point state.
    //
    save.XRANGE = spicelib::VNORM(save.TANSTA.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"RANGE", save.RANGE, b"~/", save.XRANGE, save.TOL, OK, ctx)?;

    //
    // Check the target epoch.
    //
    save.XEPOCH = (save.ET - (save.RANGE / spicelib::CLIGHT()));

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"TRGEPC", save.TRGEPC, b"~", save.XEPOCH, save.TOL, OK, ctx)?;

    //
    // Compute the angular separation of the observer-tangent point
    // vector with the original ray direction.
    //
    save.SEP = spicelib::VSEP(save.TANSTA.as_slice(), save.DVEC.as_slice(), ctx);

    //
    // Use a tolerance approximately equal to the angular extent of
    // 0.1 mm at the range of the tangent point.
    //
    save.TOL = (0.0000001 / save.RANGE);

    testutil::CHCKSD(b"TANSTA-DVEC SEP", save.SEP, b"~", 0.0, save.TOL, OK, ctx)?;

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
