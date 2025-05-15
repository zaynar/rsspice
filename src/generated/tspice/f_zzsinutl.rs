//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSRF: i32 = 100;
const DSK0: &[u8] = b"zzsinutl_0.bds";
const DSK1: &[u8] = b"zzsinutl_1.bds";
const PCK0: &[u8] = b"zzsinutl.tpc";
const VTIGHT: f64 = 0.00000000000001;
const NAMLEN: i32 = 32;

struct SaveVars {
    FIXREF: Vec<u8>,
    KVNAME: Vec<u8>,
    TARGET: Vec<u8>,
    DIST: f64,
    ET: f64,
    MAXRAD: f64,
    MINRAD: f64,
    PNEAR: StackArray<f64, 3>,
    RADII: StackArray<f64, 3>,
    RAYDIR: StackArray<f64, 3>,
    SPOINT: StackArray<f64, 3>,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    XDIST: f64,
    XMAX: f64,
    XMIN: f64,
    XRADII: StackArray<f64, 3>,
    XPNEAR: StackArray<f64, 3>,
    XPT: StackArray<f64, 3>,
    XXPT: StackArray<f64, 3>,
    BODYID: i32,
    FIXFID: i32,
    N: i32,
    NLAT: i32,
    NLON: i32,
    NSURF: i32,
    SRFLST: StackArray<i32, 100>,
    SURFID: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIXREF = vec![b' '; NAMLEN as usize];
        let mut KVNAME = vec![b' '; NAMLEN as usize];
        let mut TARGET = vec![b' '; NAMLEN as usize];
        let mut DIST: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut MAXRAD: f64 = 0.0;
        let mut MINRAD: f64 = 0.0;
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut RADII = StackArray::<f64, 3>::new(1..=3);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut XDIST: f64 = 0.0;
        let mut XMAX: f64 = 0.0;
        let mut XMIN: f64 = 0.0;
        let mut XRADII = StackArray::<f64, 3>::new(1..=3);
        let mut XPNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut XPT = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT = StackArray::<f64, 3>::new(1..=3);
        let mut BODYID: i32 = 0;
        let mut FIXFID: i32 = 0;
        let mut N: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut SURFID: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            FIXREF,
            KVNAME,
            TARGET,
            DIST,
            ET,
            MAXRAD,
            MINRAD,
            PNEAR,
            RADII,
            RAYDIR,
            SPOINT,
            TOL,
            VERTEX,
            XDIST,
            XMAX,
            XMIN,
            XRADII,
            XPNEAR,
            XPT,
            XXPT,
            BODYID,
            FIXFID,
            N,
            NLAT,
            NLON,
            NSURF,
            SRFLST,
            SURFID,
            FOUND,
        }
    }
}

//$Procedure F_ZZSINUTL ( ZZSINUTL tests )
pub fn F_ZZSINUTL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    // DOUBLE PRECISION      TIGHT
    // PARAMETER           ( TIGHT  = 1.D-11 )

    //
    // Local Variables
    //
    // CHARACTER*(LNSIZE)    TITLE

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
    testutil::TOPEN(b"F_ZZSINUTL", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create PCK.", ctx)?;

    if spicelib::EXISTS(PCK0, ctx)? {
        spicelib::DELFIL(PCK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::T_PCK08(PCK0, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create Mars DSK.", ctx)?;

    if spicelib::EXISTS(DSK0, ctx)? {
        spicelib::DELFIL(DSK0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    save.BODYID = 499;
    save.SURFID = 1;
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    spicelib::NAMFRM(&save.FIXREF, &mut save.FIXFID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NLON = 40;
    save.NLAT = 20;

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Set-up: create scaled-down Mars DSK.", ctx)?;

    if spicelib::EXISTS(DSK1, ctx)? {
        spicelib::DELFIL(DSK1, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Temporarily replace radii in pool with scaled
    // radii.
    //
    spicelib::VSCL(0.5, save.XRADII.as_slice(), save.RADII.as_slice_mut());

    fstr::assign(&mut save.KVNAME, b"BODY499_RADII");

    spicelib::PDPOOL(&save.KVNAME, 3, save.RADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // This file contains "surface 2" for Mars.
    //
    save.SURFID = 2;
    save.BODYID = 499;
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");

    save.NLON = 40;
    save.NLAT = 20;

    testutil::T_ELDS2Z(
        save.BODYID,
        save.SURFID,
        &save.FIXREF,
        save.NLON,
        save.NLAT,
        DSK1,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore original Mars radii.
    //
    spicelib::PDPOOL(&save.KVNAME, 3, save.XRADII.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZSINUTL error case
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSINUTL error: direct call to ZZSINUTL.", ctx)?;

    spicelib::ZZSINUTL(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        save.FIXFID,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.SPOINT.as_slice(),
        save.FOUND,
        save.MINRAD,
        save.MAXRAD,
        save.PNEAR.as_slice(),
        save.DIST,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BOGUSENTRY)", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZMAXRAD / ZZSUDSKI / ZZSUELIN tests
    //
    //**********************************************************************

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMAXRAD: ellipsoid shape.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    spicelib::ZZSUELIN(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMAX = intrinsics::DMAX1(&[save.XRADII[1], save.XRADII[2], save.XRADII[3]]);

    //
    // Get the max target radius.
    //
    spicelib::ZZMAXRAD(&mut save.MAXRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~", save.XMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMAXRAD: DSK shape; use two surfaces.", ctx)?;

    //
    // Load DSKs.
    //
    spicelib::FURNSH(DSK0, ctx)?;
    spicelib::FURNSH(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMAX = intrinsics::DMAX1(&[save.XRADII[1], save.XRADII[2], save.XRADII[3]]);

    //
    // Get the max target radius. We expect surface 2 not to
    // contribute.
    //
    // In principle, the maximum radius should match that of the
    // ellipsoid, since the tessellation, which uses an even number of
    // latitude bands, places some vertices on the target body's
    // equator. However, round-off error should be expected.
    //
    spicelib::ZZMAXRAD(&mut save.MAXRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMAXRAD: DSK shape; use empty surface list.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMAX = intrinsics::DMAX1(&[save.XRADII[1], save.XRADII[2], save.XRADII[3]]);

    //
    // Get the max target radius. We expect surface 2 not to
    // contribute.
    //
    spicelib::ZZMAXRAD(&mut save.MAXRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAX, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMAXRAD: DSK shape; use surface 2 only.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMAX = (0.5 * intrinsics::DMAX1(&[save.XRADII[1], save.XRADII[2], save.XRADII[3]]));

    //
    // Get the max target radius. We expect surface 2 not to
    // contribute.
    //
    spicelib::ZZMAXRAD(&mut save.MAXRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MAXRAD", save.MAXRAD, b"~/", save.XMAX, save.TOL, OK, ctx)?;

    //**********************************************************************
    //
    //     ZZSUELIN error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSUELIN: ellipsoid radii are not available.", ctx)?;

    //
    // Radii for the target are not loaded.
    //
    save.BODYID = -777;

    spicelib::ZZSUELIN(save.BODYID, ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERNELVARNOTFOUND)", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZSUDSKI error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSUDSKI: surface count too large.", ctx)?;

    save.NSURF = (MAXSRF + 1);

    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZSUDSKI: surface count too small.", ctx)?;

    save.NSURF = -1;

    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCOUNT)", OK, ctx)?;

    //**********************************************************************
    //
    //     ZZMINRAD / ZZSUDSKI / ZZSUELIN tests
    //
    //**********************************************************************

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMINRAD: ellipsoid shape.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    spicelib::ZZSUELIN(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMIN = intrinsics::DMIN1(&[save.XRADII[1], save.XRADII[2], save.XRADII[3]]);

    //
    // Get the min target radius.
    //
    spicelib::ZZMINRAD(&mut save.MINRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~", save.XMIN, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMINRAD: DSK shape; use two surfaces.", ctx)?;

    //
    // The DSKs are already loaded.
    //

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    //
    // Get the expected minimum radius via a brute force
    // computation which considers each plate.
    //
    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.XMIN,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZMINRAD(&mut save.MINRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMIN, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMINRAD: DSK shape; use empty surface list.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the expected minimum radius via a brute force
    // computation which considers each plate.
    //
    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.XMIN,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the min target radius. We expect surface 2 to
    // contribute this value.
    //
    spicelib::ZZMINRAD(&mut save.MINRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMIN, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZMINRAD: DSK shape; use surface 2 only.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Get the expected minimum radius via a brute force
    // computation which considers each plate.
    //
    spicelib::ZZDSKSPH(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        &mut save.XMIN,
        &mut save.MAXRAD,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the min target radius. We expect surface 2 to
    // contribute this value.

    spicelib::ZZMINRAD(&mut save.MINRAD, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"MINRAD", save.MINRAD, b"~/", save.XMIN, save.TOL, OK, ctx)?;

    //**********************************************************************
    //
    //     ZZRAYSFX tests
    //
    //**********************************************************************

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYSFX: Ellipsoid intercept.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    spicelib::ZZSUELIN(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMIN = intrinsics::DMIN1(&[save.XRADII[1], save.XRADII[2], save.XRADII[3]]);

    //
    // Get expected intercept.
    //
    spicelib::SURFPT(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XRADII[1],
        save.XRADII[2],
        save.XRADII[3],
        save.XXPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"SURFPT found", save.FOUND, true, OK, ctx)?;

    //
    // The epoch is unused for the ellipsoid case.
    //
    save.ET = 0.0;

    //
    // Now compute the intercept using the generalized callback.
    //

    spicelib::ZZRAYSFX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ZZRAYSFX found", save.FOUND, true, OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYSFX: Ellipsoid non-intercept.", ctx)?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    spicelib::ZZSUELIN(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VEQU(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.XMIN = intrinsics::DMIN1(&[save.XRADII[1], save.XRADII[2], save.XRADII[3]]);

    //
    // Now try to compute the intercept using the generalized callback.
    //

    spicelib::ZZRAYSFX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ZZRAYSFX found", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYSFX: DSK intercept; use 2 surfaces.", ctx)?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch IS used for the DSK case; it's used to
    // select applicable segments.
    //
    save.ET = 0.0;

    //
    // Compute the expected intercept.
    //
    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XXPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DSKXV found", save.FOUND, true, OK, ctx)?;

    //
    // Now compute the intercept using the generalized callback.
    //

    spicelib::ZZRAYSFX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ZZRAYSFX found", save.FOUND, true, OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYSFX: DSK intercept; use an empty surface list.", ctx)?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 0;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch IS used for the DSK case; it's used to
    // select applicable segments.
    //
    save.ET = 0.0;

    //
    // Compute the expected intercept.
    //
    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XXPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DSKXV found", save.FOUND, true, OK, ctx)?;

    //
    // Now compute the intercept using the generalized callback.
    //

    spicelib::ZZRAYSFX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ZZRAYSFX found", save.FOUND, true, OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZRAYSFX: DSK intercept; use the second surface only.",
        ctx,
    )?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch IS used for the DSK case; it's used to
    // select applicable segments.
    //
    save.ET = 0.0;

    //
    // Compute the expected intercept.
    //
    spicelib::DSKXV(
        false,
        &save.TARGET,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.ET,
        &save.FIXREF,
        1,
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XXPT.as_slice_mut(),
        std::slice::from_mut(&mut save.FOUND),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"DSKXV found", save.FOUND, true, OK, ctx)?;

    //
    // Now compute the intercept using the generalized callback.
    //

    spicelib::ZZRAYSFX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ZZRAYSFX found", save.FOUND, true, OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        save.XPT.as_slice(),
        b"~~/",
        save.XXPT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZRAYSFX: DSK non-intercept due to ray direction; use 2 surfaces.",
        ctx,
    )?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VEQU(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch IS used for the DSK case; it's used to
    // select applicable segments.
    //
    save.ET = 0.0;

    //
    // Now try to compute the intercept using the generalized callback.
    //
    spicelib::ZZRAYSFX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ZZRAYSFX found", save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZRAYSFX: DSK non-intercept due to epoch; use 2 surfaces.",
        ctx,
    )?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    //
    // Set ray vertex and direction so that an intercept
    // will occur if the required surface data are
    // available.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VMINUS(save.VERTEX.as_slice(), save.RAYDIR.as_slice_mut());

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch IS used for the DSK case; it's used to
    // select applicable segments.
    //
    save.ET = -((500 as f64) * spicelib::JYEAR());

    //
    // Now try to compute the intercept using the generalized callback.
    //
    spicelib::ZZRAYSFX(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.XPT.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ZZRAYSFX found", save.FOUND, false, OK, ctx)?;

    //**********************************************************************
    //
    //     ZZRAYNP tests
    //
    //**********************************************************************

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZRAYNP: find near point to ray on ellipsoid. Non-intercept case.",
        ctx,
    )?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    spicelib::ZZSUELIN(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(5000.0, -6000.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected near point.
    //
    spicelib::NPEDLN(
        save.XRADII[1],
        save.XRADII[2],
        save.XRADII[3],
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPNEAR.as_slice_mut(),
        &mut save.XDIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch is unused for the ellipsoid case.
    //
    save.ET = 0.0;

    //
    // Now compute the near point using the generalized callback.
    //

    spicelib::ZZRAYNP(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"ZZRAYNP: find near point to ray on ellipsoid. Intercept case.",
        ctx,
    )?;

    //
    // Set target parameters.
    //
    save.BODYID = 499;
    spicelib::ZZSUELIN(save.BODYID, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 0.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(5000.0, -6000.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Get target radii from PCK file.
    //
    spicelib::BODVCD(
        save.BODYID,
        b"RADII",
        3,
        &mut save.N,
        save.XRADII.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected near point.
    //
    spicelib::NPEDLN(
        save.XRADII[1],
        save.XRADII[2],
        save.XRADII[3],
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPNEAR.as_slice_mut(),
        &mut save.XDIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch is unused for the ellipsoid case.
    //
    save.ET = 0.0;

    //
    // Now compute the near point using the generalized callback.
    //

    spicelib::ZZRAYNP(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    //
    // For the intercept cases, use an absolute error test for
    // distance, rather than a relative error test.
    //
    testutil::CHCKSD(b"DIST", save.DIST, b"~", save.XDIST, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYNP: DSK case: find near point to ray on bounding ellipsoid. Use two surfaces. Non-intercept case.", ctx)?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 2;
    save.SRFLST[1] = 1;
    save.SRFLST[2] = 2;

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 0.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(5000.0, -6000.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Get maximum bounding radius.
    //
    spicelib::ZZMAXRAD(&mut save.XRADII[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected near point. Note that we have a spherical
    // bounding surface.
    //
    spicelib::NPEDLN(
        save.XRADII[1],
        save.XRADII[1],
        save.XRADII[1],
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPNEAR.as_slice_mut(),
        &mut save.XDIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch is unused for the ellipsoid case.
    //
    save.ET = 0.0;

    //
    // Now compute the near point using the generalized callback.
    //

    spicelib::ZZRAYNP(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYNP: DSK case: find near point to ray on bounding ellipsoid. Use two surfaces, this time with an empty surface list. Non-intercept case.", ctx)?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 0;

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(5000.0, -6000.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Get maximum bounding radius.
    //
    spicelib::ZZMAXRAD(&mut save.XRADII[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected near point. Note that we have a spherical
    // bounding surface.
    //
    spicelib::NPEDLN(
        save.XRADII[1],
        save.XRADII[1],
        save.XRADII[1],
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPNEAR.as_slice_mut(),
        &mut save.XDIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch is unused for the ellipsoid case.
    //
    save.ET = 0.0;

    //
    // Now compute the near point using the generalized callback.
    //

    spicelib::ZZRAYNP(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYNP: DSK case: find near point to ray on bounding ellipsoid. Use the second surface only. Non-intercept case.", ctx)?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 10000.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(5000.0, -6000.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Get maximum bounding radius.
    //
    spicelib::ZZMAXRAD(&mut save.XRADII[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected near point. Note that we have a spherical
    // bounding surface.
    //
    spicelib::NPEDLN(
        save.XRADII[1],
        save.XRADII[1],
        save.XRADII[1],
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPNEAR.as_slice_mut(),
        &mut save.XDIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch is unused for the ellipsoid case.
    //
    save.ET = 0.0;

    //
    // Now compute the near point using the generalized callback.
    //

    spicelib::ZZRAYNP(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"ZZRAYNP: DSK case: find near point to ray on bounding ellipsoid. Use the second surface only. Intercept case.", ctx)?;

    //
    // Set target parameters.
    //
    fstr::assign(&mut save.TARGET, b"MARS");
    fstr::assign(&mut save.FIXREF, b"IAU_MARS");
    save.BODYID = 499;
    save.NSURF = 1;
    save.SRFLST[1] = 2;

    //
    // Set the target parameters.
    //
    spicelib::ZZSUDSKI(
        save.BODYID,
        save.NSURF,
        save.SRFLST.as_slice(),
        save.FIXFID,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set ray vertex and direction.
    //
    spicelib::VPACK(-5000.0, 6000.0, 0.0, save.VERTEX.as_slice_mut());

    spicelib::VPACK(5000.0, -6000.0, 0.0, save.RAYDIR.as_slice_mut());

    //
    // Get maximum bounding radius.
    //
    spicelib::ZZMAXRAD(&mut save.XRADII[1], ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get expected near point. Note that we have a spherical
    // bounding surface.
    //
    spicelib::NPEDLN(
        save.XRADII[1],
        save.XRADII[1],
        save.XRADII[1],
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.XPNEAR.as_slice_mut(),
        &mut save.XDIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The epoch is unused for the ellipsoid case.
    //
    save.ET = 0.0;

    //
    // Now compute the near point using the generalized callback.
    //

    spicelib::ZZRAYNP(
        save.VERTEX.as_slice(),
        save.RAYDIR.as_slice(),
        save.ET,
        save.PNEAR.as_slice_mut(),
        &mut save.DIST,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~", save.XDIST, save.TOL, OK, ctx)?;
    testutil::CHCKAD(
        b"PNEAR",
        save.PNEAR.as_slice(),
        b"~~/",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up.", ctx)?;

    spicelib::DELFIL(PCK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(DSK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
