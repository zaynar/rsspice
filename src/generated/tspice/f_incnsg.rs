//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 320;
const VTIGHT: f64 = 0.000000000001;
const TIGHT: f64 = 0.0000000001;
const MED: f64 = 0.000000001;

struct SaveVars {
    TITLE: Vec<u8>,
    ANGLE: f64,
    APEX: StackArray<f64, 3>,
    AXIS: StackArray<f64, 3>,
    CONDIR: StackArray<f64, 3>,
    CONPT1: StackArray<f64, 3>,
    CONPT2: StackArray<f64, 3>,
    CONVTX: StackArray<f64, 3>,
    CONX: StackArray<f64, 3>,
    CONZ: StackArray<f64, 3>,
    DALPHA: f64,
    DIST: f64,
    DLAT: f64,
    DP: f64,
    DSPAN: f64,
    ENDPT1: StackArray<f64, 3>,
    ENDPT2: StackArray<f64, 3>,
    EP1: StackArray<f64, 3>,
    EP1OFF: StackArray<f64, 3>,
    EP2: StackArray<f64, 3>,
    EP2OFF: StackArray<f64, 3>,
    H: f64,
    LAT: f64,
    MARGIN: f64,
    OFFSET: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    R: f64,
    R2: f64,
    RAYDIR: StackArray<f64, 3>,
    S: f64,
    SEP: f64,
    SPAN: f64,
    TARG: StackArray<f64, 3>,
    TARGY: f64,
    THETA: f64,
    TOL: f64,
    VERTEX: StackArray<f64, 3>,
    VTEMP: StackArray<f64, 3>,
    XFORM: StackArray2D<f64, 9>,
    XPT1: StackArray<f64, 3>,
    XPT2: StackArray<f64, 3>,
    XVEC: StackArray<f64, 3>,
    XXPT1: StackArray<f64, 3>,
    XXPT2: StackArray<f64, 3>,
    XZ: f64,
    YVEC: StackArray<f64, 3>,
    Z: f64,
    K: i32,
    NCONE: i32,
    NLAT: i32,
    NSPAN: i32,
    NXPTS: i32,
    XNXPTS: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut ANGLE: f64 = 0.0;
        let mut APEX = StackArray::<f64, 3>::new(1..=3);
        let mut AXIS = StackArray::<f64, 3>::new(1..=3);
        let mut CONDIR = StackArray::<f64, 3>::new(1..=3);
        let mut CONPT1 = StackArray::<f64, 3>::new(1..=3);
        let mut CONPT2 = StackArray::<f64, 3>::new(1..=3);
        let mut CONVTX = StackArray::<f64, 3>::new(1..=3);
        let mut CONX = StackArray::<f64, 3>::new(1..=3);
        let mut CONZ = StackArray::<f64, 3>::new(1..=3);
        let mut DALPHA: f64 = 0.0;
        let mut DIST: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DP: f64 = 0.0;
        let mut DSPAN: f64 = 0.0;
        let mut ENDPT1 = StackArray::<f64, 3>::new(1..=3);
        let mut ENDPT2 = StackArray::<f64, 3>::new(1..=3);
        let mut EP1 = StackArray::<f64, 3>::new(1..=3);
        let mut EP1OFF = StackArray::<f64, 3>::new(1..=3);
        let mut EP2 = StackArray::<f64, 3>::new(1..=3);
        let mut EP2OFF = StackArray::<f64, 3>::new(1..=3);
        let mut H: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut MARGIN: f64 = 0.0;
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut R2: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut S: f64 = 0.0;
        let mut SEP: f64 = 0.0;
        let mut SPAN: f64 = 0.0;
        let mut TARG = StackArray::<f64, 3>::new(1..=3);
        let mut TARGY: f64 = 0.0;
        let mut THETA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut VERTEX = StackArray::<f64, 3>::new(1..=3);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XPT1 = StackArray::<f64, 3>::new(1..=3);
        let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
        let mut XVEC = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT1 = StackArray::<f64, 3>::new(1..=3);
        let mut XXPT2 = StackArray::<f64, 3>::new(1..=3);
        let mut XZ: f64 = 0.0;
        let mut YVEC = StackArray::<f64, 3>::new(1..=3);
        let mut Z: f64 = 0.0;
        let mut K: i32 = 0;
        let mut NCONE: i32 = 0;
        let mut NLAT: i32 = 0;
        let mut NSPAN: i32 = 0;
        let mut NXPTS: i32 = 0;
        let mut XNXPTS: i32 = 0;

        Self {
            TITLE,
            ANGLE,
            APEX,
            AXIS,
            CONDIR,
            CONPT1,
            CONPT2,
            CONVTX,
            CONX,
            CONZ,
            DALPHA,
            DIST,
            DLAT,
            DP,
            DSPAN,
            ENDPT1,
            ENDPT2,
            EP1,
            EP1OFF,
            EP2,
            EP2OFF,
            H,
            LAT,
            MARGIN,
            OFFSET,
            PNEAR,
            R,
            R2,
            RAYDIR,
            S,
            SEP,
            SPAN,
            TARG,
            TARGY,
            THETA,
            TOL,
            VERTEX,
            VTEMP,
            XFORM,
            XPT1,
            XPT2,
            XVEC,
            XXPT1,
            XXPT2,
            XZ,
            YVEC,
            Z,
            K,
            NCONE,
            NLAT,
            NSPAN,
            NXPTS,
            XNXPTS,
        }
    }
}

//$Procedure F_INCNSG ( INCNSG tests )
pub fn F_INCNSG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_INCNSG", ctx)?;

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Axis is zero vector.", ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, save.AXIS.as_slice_mut());

    save.ANGLE = (30.0 * spicelib::RPD(ctx));

    save.XZ = f64::sqrt(3.0);

    spicelib::VPACK(-1.0, 0.0, (save.XZ / 2 as f64), save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 2.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Angular radius of cone is negative.", ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = -0.000000000001;

    save.XZ = f64::sqrt(3.0);

    spicelib::VPACK(-1.0, 0.0, (save.XZ / 2 as f64), save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 2.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDANGLE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Line segment has zero length.", ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = (30.0 * spicelib::RPD(ctx));

    save.XZ = f64::sqrt(3.0);

    spicelib::VPACK(-1.0, 0.0, (save.XZ / 2 as f64), save.ENDPT1.as_slice_mut());
    spicelib::VEQU(save.ENDPT1.as_slice(), save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ENDPOINTSMATCH)", OK, ctx)?;

    //
    //     To be completed:
    //
    // --- Case: ------------------------------------------------------
    //
    //      TITLE =  'Coefficients have absolute values that are too large.'
    //
    //      CALL TCASE ( TITLE )

    //**********************************************************************
    //
    //     Non-error exceptional cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Cone is the half-space {Z>=0}. Segment is vertical and intersects cone.",
        ctx,
    )?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = (90.0 * spicelib::RPD(ctx));

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 2.0, 3.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(-1.0, 2.0, 0.0, save.XXPT1.as_slice_mut());

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(-1.0, 2.0, 0.0, save.XXPT1.as_slice_mut());

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Cone is the half-space {Z>=0}. Segment is vertical and misses cone.",
        ctx,
    )?;

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 2.0, -2.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stretch case: cone is ALMOST the half-space {Z>=0}; cone angle is pi/2 - 1.e-9. Segment is vertical and intersects cone.", ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = ((90.0 * spicelib::RPD(ctx)) - 0.000000001);

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 2.0, 3.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(-1.0, 2.0, 0.0, save.XXPT1.as_slice_mut());

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(-1.0, 2.0, 0.0, save.XXPT1.as_slice_mut());

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stretch case: cone is ALMOST the half-space {Z>=0}; cone angle is pi/2 - 1.e-9. Segment is vertical and misses cone.", ctx)?;

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 2.0, -2.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stretch case: cone is ALMOST the half-space { X: <X-APEX,AXIS>  >  0}; cone angle is pi/2 - 1.e-9. Segment is parallel to AXIS and intersects cone.", ctx)?;

    spicelib::VPACK(100.0, -200.0, 300.0, save.APEX.as_slice_mut());

    spicelib::VPACK(-7.0, 5.0, 1.0, save.AXIS.as_slice_mut());

    spicelib::FRAME(
        save.AXIS.as_slice_mut(),
        save.XVEC.as_slice_mut(),
        save.YVEC.as_slice_mut(),
    );

    save.ANGLE = ((90.0 * spicelib::RPD(ctx)) - 0.000000001);

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        -3.0,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );

    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT1.as_slice_mut(),
    );

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        3.0,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );

    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT2.as_slice_mut(),
    );

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (up)", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    save.Z = (f64::sqrt(5.0) * f64::tan((spicelib::HALFPI(ctx) - save.ANGLE)));

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (up)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (down)", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        -save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (down)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stretch case: cone is ALMOST the half-space { X: <X-APEX,AXIS>  <  0}; cone angle is pi/2 + 1.e-9. Segment is parallel to AXIS and intersects cone.", ctx)?;

    spicelib::VPACK(100.0, -200.0, 300.0, save.APEX.as_slice_mut());

    spicelib::VPACK(-7.0, 5.0, 1.0, save.AXIS.as_slice_mut());

    spicelib::FRAME(
        save.AXIS.as_slice_mut(),
        save.XVEC.as_slice_mut(),
        save.YVEC.as_slice_mut(),
    );

    save.ANGLE = ((90.0 * spicelib::RPD(ctx)) + 0.000000001);

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        -3.0,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );

    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT1.as_slice_mut(),
    );

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        3.0,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );

    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT2.as_slice_mut(),
    );

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (up)", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    save.Z = (f64::sqrt(5.0) * f64::tan((spicelib::HALFPI(ctx) - save.ANGLE)));

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (up)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (down)", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        -save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (down)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stretch case: cone is ALMOST the half-space { X: <X-APEX,AXIS>  <  0}; cone angle is pi/2 - 1.e-9. Segment is orthogonal to cone\'s axis and intersects cone twice.", ctx)?;

    spicelib::VPACK(100.0, -200.0, 300.0, save.APEX.as_slice_mut());

    spicelib::VPACK(-7.0, 5.0, 1.0, save.AXIS.as_slice_mut());

    spicelib::FRAME(
        save.AXIS.as_slice_mut(),
        save.XVEC.as_slice_mut(),
        save.YVEC.as_slice_mut(),
    );

    save.ANGLE = ((90.0 * spicelib::RPD(ctx)) - 0.000000001);

    //
    // Z is the magnitude of the projections of the endpoints
    // onto the cone's axis.
    //
    save.Z = 0.0000000001;

    spicelib::VLCOM3(
        1.0,
        save.XVEC.as_slice(),
        -2.0,
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT1.as_slice_mut(),
    );

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT2.as_slice_mut(),
    );

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (1)", save.NXPTS, b"=", 2, 0, OK, ctx)?;

    //
    // Compute expected value of XPT1.
    //
    save.R = f64::sqrt(5.0);
    save.Z = 0.0000000001;
    save.S = (save.Z / f64::tan(f64::abs((spicelib::HALFPI(ctx) - save.ANGLE))));

    spicelib::VLCOM3(
        ((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        -((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (1)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    spicelib::VLCOM3(
        -((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        ((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT2.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT2 (1)",
        save.XPT2.as_slice(),
        b"~~/",
        save.XXPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (2)", save.NXPTS, b"=", 2, 0, OK, ctx)?;

    spicelib::VLCOM3(
        -((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        ((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (2)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    spicelib::VLCOM3(
        ((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        -((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT2.as_slice_mut(),
    );

    testutil::CHCKAD(
        b"XPT2 (2)",
        save.XPT2.as_slice(),
        b"~~/",
        save.XXPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Stretch case: cone is ALMOST the half-space { X: <X-APEX,AXIS>  <  0}; cone angle is pi/2 + 1.e-9. Segment is orthogonal to cone\'s axis and intersects cone twice.", ctx)?;

    spicelib::VPACK(100.0, -200.0, 300.0, save.APEX.as_slice_mut());

    spicelib::VPACK(-7.0, 5.0, 1.0, save.AXIS.as_slice_mut());

    spicelib::FRAME(
        save.AXIS.as_slice_mut(),
        save.XVEC.as_slice_mut(),
        save.YVEC.as_slice_mut(),
    );

    save.ANGLE = ((90.0 * spicelib::RPD(ctx)) + 0.000000001);

    //
    // Z is the magnitude of the projections of the endpoints
    // onto the cone's axis.
    //
    save.Z = -0.0000000001;

    spicelib::VLCOM3(
        1.0,
        save.XVEC.as_slice(),
        -2.0,
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT1.as_slice_mut(),
    );

    spicelib::VLCOM3(
        -1.0,
        save.XVEC.as_slice(),
        2.0,
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.ENDPT2.as_slice_mut(),
    );

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (1)", save.NXPTS, b"=", 2, 0, OK, ctx)?;

    //
    // Compute expected value of XPT1.
    //
    save.R = f64::sqrt(5.0);
    save.Z = 0.0000000001;
    save.S = (f64::abs(save.Z) / f64::tan(f64::abs((spicelib::HALFPI(ctx) - save.ANGLE))));

    spicelib::VLCOM3(
        ((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        -((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (1)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    spicelib::VLCOM3(
        -((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        ((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT2.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT2 (1)",
        save.XPT2.as_slice(),
        b"~~/",
        save.XXPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS (2)", save.NXPTS, b"=", 2, 0, OK, ctx)?;

    spicelib::VLCOM3(
        -((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        ((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.0000001;

    testutil::CHCKAD(
        b"XPT1 (2)",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    spicelib::VLCOM3(
        ((1.0 * save.S) / save.R),
        save.XVEC.as_slice(),
        -((2.0 * save.S) / save.R),
        save.YVEC.as_slice(),
        save.Z,
        save.AXIS.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VADD(
        save.VTEMP.as_slice(),
        save.APEX.as_slice(),
        save.XXPT2.as_slice_mut(),
    );

    testutil::CHCKAD(
        b"XPT2 (2)",
        save.XPT2.as_slice(),
        b"~~/",
        save.XXPT2.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Cone is ALMOST the half-space {Z>=0}; cone angle is pi/2 - 1.e-15. Segment is vertical and intersects cone.", ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = ((90.0 * spicelib::RPD(ctx)) - 0.000000000000001);

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 2.0, 3.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(-1.0, 2.0, 0.0, save.XXPT1.as_slice_mut());

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(-1.0, 2.0, 0.0, save.XXPT1.as_slice_mut());

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Cone is ALMOST the half-space {Z>=0}; cone angle is pi/2 - 1.e-15. Segment is vertical and misses cone.", ctx)?;

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 2.0, -2.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Cone is the half-space {X<=5}. Segment is parallel to X axis and intersects cone.",
        ctx,
    )?;

    spicelib::VPACK(5.0, 3.0, 4.0, save.APEX.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 0.0, save.AXIS.as_slice_mut());

    save.ANGLE = (90.0 * spicelib::RPD(ctx));

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(6.0, 2.0, -3.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(5.0, 2.0, -3.0, save.XXPT1.as_slice_mut());

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(5.0, 2.0, -3.0, save.XXPT1.as_slice_mut());

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Cone is the half-space {X<=5}. Segment is parallel to X axis and misses cone.",
        ctx,
    )?;

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-4.0, 2.0, -3.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Cone is ALMOST the half-space {Z>=0}; cone angle is pi/2 - 1.e-7. Segment is vertical and intersects cone.", ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = ((90.0 * spicelib::RPD(ctx)) - 0.0000001);

    spicelib::VPACK(-1.0, 2.0, -3.0, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 2.0, 3.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    spicelib::VPACK(
        -1.0,
        2.0,
        (f64::tan((spicelib::HALFPI(ctx) - save.ANGLE)) * f64::sqrt(5.0)),
        save.XXPT1.as_slice_mut(),
    );

    save.TOL = 0.00000001;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Repeat call with segment endpoints reversed.
    //
    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT2.as_slice(),
        save.ENDPT1.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

    testutil::CHCKAD(
        b"XPT1",
        save.XPT1.as_slice(),
        b"~~/",
        save.XXPT1.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Trivial case: cone apex is origin; axis is +Z; angular radius is 30 degrees; segment\'s first endpoint is (-1,0,sqrt(3)/2); second endpoint is (0,0,sqrt(3)/2).");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = (30.0 * spicelib::RPD(ctx));

    save.XZ = (f64::sqrt(3.0) / 2 as f64);

    spicelib::VPACK(-1.0, 0.0, save.XZ, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, save.XZ, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect one point of intersection to be found.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    if *OK {
        spicelib::VPACK(-0.5, 0.0, save.XZ, save.XXPT1.as_slice_mut());
        spicelib::CLEARD(3, save.XXPT2.as_slice_mut());

        testutil::CHCKAD(
            b"XPT1",
            save.XPT1.as_slice(),
            b"~~/",
            save.XXPT1.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Trivial case: cone apex is origin; axis is +Z; angular radius is 30 degrees; segment\'s first endpoint is (-1,0,sqrt(3)/2); second endpoint is (-1,0,2).");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = (30.0 * spicelib::RPD(ctx));

    save.XZ = f64::sqrt(3.0);

    spicelib::VPACK(-1.0, 0.0, (save.XZ / 2 as f64), save.ENDPT1.as_slice_mut());
    spicelib::VPACK(-1.0, 0.0, 2.0, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect one point of intersection to be found.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    if *OK {
        spicelib::VPACK(-1.0, 0.0, save.XZ, save.XXPT1.as_slice_mut());
        spicelib::CLEARD(3, save.XXPT2.as_slice_mut());

        testutil::CHCKAD(
            b"XPT1",
            save.XPT1.as_slice(),
            b"~~/",
            save.XXPT1.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Trivial case: cone apex is origin; axis is +Z; angular radius is 30 degrees; segment\'s first endpoint is (-1,0,sqrt(3)/2); second endpoint is (1,0,sqrt(3)/2).");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = (30.0 * spicelib::RPD(ctx));

    save.XZ = (f64::sqrt(3.0) / 2 as f64);

    spicelib::VPACK(-1.0, 0.0, save.XZ, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, save.XZ, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect two points of intersection to be found.
    //
    save.XNXPTS = 2;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    if *OK {
        spicelib::VPACK(-0.5, 0.0, save.XZ, save.XXPT1.as_slice_mut());
        spicelib::VPACK(0.5, 0.0, save.XZ, save.XXPT2.as_slice_mut());

        testutil::CHCKAD(
            b"XPT1",
            save.XPT1.as_slice(),
            b"~~/",
            save.XXPT1.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"XPT2",
            save.XPT2.as_slice(),
            b"~~/",
            save.XXPT2.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Repeat previous case but offset both the apex and ray\'s vertex.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, -5.0, save.APEX.as_slice_mut());

    save.ANGLE = (30.0 * spicelib::RPD(ctx));

    save.XZ = (f64::sqrt(3.0) / 2 as f64);

    spicelib::VPACK(-1.0, 0.0, save.XZ, save.ENDPT1.as_slice_mut());
    spicelib::VADD(
        save.ENDPT1.as_slice(),
        save.APEX.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.ENDPT1.as_slice_mut());

    spicelib::VPACK(1.0, 0.0, save.XZ, save.ENDPT2.as_slice_mut());
    spicelib::VADD(
        save.ENDPT2.as_slice(),
        save.APEX.as_slice(),
        save.VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(save.VTEMP.as_slice(), save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect two points of intersection to be found.
    //
    save.XNXPTS = 2;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    if *OK {
        spicelib::VPACK(-0.5, 0.0, save.XZ, save.XXPT1.as_slice_mut());
        spicelib::VADD(
            save.XXPT1.as_slice(),
            save.APEX.as_slice(),
            save.VTEMP.as_slice_mut(),
        );
        spicelib::VEQU(save.VTEMP.as_slice(), save.XXPT1.as_slice_mut());

        spicelib::VPACK(0.5, 0.0, save.XZ, save.XXPT2.as_slice_mut());
        spicelib::VADD(
            save.XXPT2.as_slice(),
            save.APEX.as_slice(),
            save.VTEMP.as_slice_mut(),
        );
        spicelib::VEQU(save.VTEMP.as_slice(), save.XXPT2.as_slice_mut());

        testutil::CHCKAD(
            b"XPT1",
            save.XPT1.as_slice(),
            b"~~/",
            save.XXPT1.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"XPT2",
            save.XPT2.as_slice(),
            b"~~/",
            save.XXPT2.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Cone angle exceeds pi/2: cone apex is origin; axis is +Z; angular radius is 150 degrees; segment\'s first endpoint is (-1,0,-sqrt(3)/2); second endpoint is (1,0,-sqrt(3)/2).");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = (150.0 * spicelib::RPD(ctx));

    save.XZ = -(f64::sqrt(3.0) / 2 as f64);

    spicelib::VPACK(-1.0, 0.0, save.XZ, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(1.0, 0.0, save.XZ, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect two points of intersection to be found.
    //
    save.XNXPTS = 2;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    if *OK {
        spicelib::VPACK(-0.5, 0.0, save.XZ, save.XXPT1.as_slice_mut());
        spicelib::VPACK(0.5, 0.0, save.XZ, save.XXPT2.as_slice_mut());

        testutil::CHCKAD(
            b"XPT1",
            save.XPT1.as_slice(),
            b"~~/",
            save.XXPT1.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
        testutil::CHCKAD(
            b"XPT2",
            save.XPT2.as_slice(),
            b"~~/",
            save.XXPT2.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Cone angle exceeds pi/2: cone apex is origin; axis is +Z; angular radius is 150 degrees; segment\'s first endpoint is (-1,0,-sqrt(3)/2); second endpoint is (0,0,-sqrt(3)/2).");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::CLEARD(3, save.APEX.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    save.ANGLE = (150.0 * spicelib::RPD(ctx));

    save.XZ = -(f64::sqrt(3.0) / 2 as f64);

    spicelib::VPACK(-1.0, 0.0, save.XZ, save.ENDPT1.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, save.XZ, save.ENDPT2.as_slice_mut());

    spicelib::INCNSG(
        save.APEX.as_slice(),
        save.AXIS.as_slice(),
        save.ANGLE,
        save.ENDPT1.as_slice(),
        save.ENDPT2.as_slice(),
        &mut save.NXPTS,
        save.XPT1.as_slice_mut(),
        save.XPT2.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect one point of intersection to be found.
    //
    save.XNXPTS = 1;

    testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", save.XNXPTS, 0, OK, ctx)?;

    if *OK {
        spicelib::VPACK(-0.5, 0.0, save.XZ, save.XXPT1.as_slice_mut());

        testutil::CHCKAD(
            b"XPT1",
            save.XPT1.as_slice(),
            b"~~/",
            save.XXPT1.as_slice(),
            3,
            VTIGHT,
            OK,
            ctx,
        )?;
    }

    //**********************************************************************
    //
    //     Normal cases --- varied geometry
    //
    //**********************************************************************

    //
    // The following are intersection cases.
    //
    // All intersection cases for lines that are not orthogonal to the
    // cone's axis can be characterized by:
    //
    //    1) The line's intersection with the plane containing the apex
    //       and normal to the axis. This point of intersection is the
    //       vertex of a ray contained in the line and intersecting the
    //       cone.
    //
    //    2) The ray's angle relative to the plane described in (1).
    //
    //    3) The azimuth of the ray relative to the line segment starting
    //       at the ray's vertex and ending at the cone's apex.
    //
    //
    // Pick an arbitrary cone axis and apex.
    //
    //
    spicelib::VPACK(3.0, -2.0, 7.0, save.APEX.as_slice_mut());

    spicelib::VPACK(2.0, 3.0, -2.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 0.0, 1.0, save.AXIS.as_slice_mut());

    spicelib::VHAT(save.AXIS.as_slice(), save.CONZ.as_slice_mut());

    //
    // Pick an arbitrary ray vertex in the X-Y plane.
    //
    save.THETA = (27.0 * spicelib::RPD(ctx));

    save.R = 1000.0;

    spicelib::VPACK(
        (save.R * f64::cos(save.THETA)),
        (save.R * f64::sin(save.THETA)),
        0.0,
        save.VTEMP.as_slice_mut(),
    );
    //
    // Create the transformation matrix for mapping vectors
    // from the standard frame to the cone frame.
    //
    spicelib::TWOVEC(
        save.CONZ.as_slice(),
        3,
        save.VTEMP.as_slice(),
        1,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;

    for I in 1..=3 {
        save.CONX[I] = save.XFORM[[1, I]];
    }

    spicelib::VSCL(save.R, save.CONX.as_slice(), save.CONVTX.as_slice_mut());

    //
    // Loop over cone angles.
    //
    save.NCONE = 10;

    save.DALPHA = (spicelib::HALFPI(ctx) / (save.NCONE + 1) as f64);

    //
    // Note: NLAT can be increased to 100 without an excessive
    // increase in run time. However, the value 10 exercises a
    // difficult numerical case: that in which the ray is normal
    // to the cone at the first intercept.
    //
    save.NLAT = 10;

    save.DLAT = (spicelib::HALFPI(ctx) / (save.NLAT + 1) as f64);

    //
    // Let the cone angles range from DALPHA to pi/2 - DALPHA.
    //
    for I in 1..=save.NCONE {
        save.ANGLE = ((I as f64) * save.DALPHA);

        //
        // Let the ray "latitude" vary from DLAT to pi/2 - DLAT. Note
        // that this is an approximation unless the ray's direction
        // vector has a zero Y component.
        //
        for J in 1..=save.NLAT {
            save.LAT = ((J as f64) * save.DLAT);

            //
            // Let H be the Z intercept of a ray emanating from CONVTX,
            // pointing toward the +Z axis, and making angle LAT relative
            // to the X-Y plane.
            //
            save.H = (save.R * f64::tan(save.LAT));
            //
            // Let SPAN be the length of a line segment parallel to the Y
            // axis and passing through the +Z axis at height H.
            //
            save.SPAN = ((2.0 * save.H) * f64::tan(save.ANGLE));

            //
            // We'll place a "target" on the segment from which SPAN was
            // derived.
            //
            save.NSPAN = 11;

            save.DSPAN = (save.SPAN / (save.NSPAN - 1) as f64);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NSPAN;
                let m3__: i32 = 1;
                save.K = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.TARGY = (-(save.SPAN / 2 as f64) + (((save.K - 1) as f64) * save.DSPAN));

                    if ((save.K == 1) || (save.K == save.NSPAN)) {
                        //
                        // Contract TARGY a bit, since these are formally
                        // tangent cases, and we want to ensure a hit.
                        //
                        save.TARGY = (save.TARGY * (1.0 - 0.001));
                    }

                    spicelib::VPACK(0.0, save.TARGY, save.H, save.TARG.as_slice_mut());

                    spicelib::VSUB(
                        save.TARG.as_slice(),
                        save.CONVTX.as_slice(),
                        save.CONDIR.as_slice_mut(),
                    );

                    spicelib::VHATIP(save.CONDIR.as_slice_mut());

                    spicelib::VEQU(save.CONVTX.as_slice(), save.CONPT1.as_slice_mut());

                    //
                    // Let R2 be the distance of the second endpoint from
                    // the vertex.
                    //
                    save.R2 = 100000.0;
                    spicelib::VLCOM(
                        1.0,
                        save.CONPT1.as_slice(),
                        save.R2,
                        save.CONDIR.as_slice(),
                        save.CONPT2.as_slice_mut(),
                    );

                    //
                    // Shift the endpoints to make them relative to the origin
                    // of the standard frame. Convert the endpoints back to the
                    // standard basis.
                    //
                    spicelib::MTXV(
                        save.XFORM.as_slice(),
                        save.CONPT1.as_slice(),
                        save.VTEMP.as_slice_mut(),
                    );
                    spicelib::VADD(
                        save.VTEMP.as_slice(),
                        save.APEX.as_slice(),
                        save.ENDPT1.as_slice_mut(),
                    );

                    spicelib::MTXV(
                        save.XFORM.as_slice(),
                        save.CONPT2.as_slice(),
                        save.VTEMP.as_slice_mut(),
                    );
                    spicelib::VADD(
                        save.VTEMP.as_slice(),
                        save.APEX.as_slice(),
                        save.ENDPT2.as_slice_mut(),
                    );

                    spicelib::MTXV(
                        save.XFORM.as_slice(),
                        save.CONVTX.as_slice(),
                        save.VTEMP.as_slice_mut(),
                    );
                    spicelib::VADD(
                        save.VTEMP.as_slice(),
                        save.APEX.as_slice(),
                        save.VERTEX.as_slice_mut(),
                    );

                    spicelib::MTXV(
                        save.XFORM.as_slice(),
                        save.TARG.as_slice(),
                        save.VTEMP.as_slice_mut(),
                    );
                    spicelib::VADD(
                        save.VTEMP.as_slice(),
                        save.APEX.as_slice(),
                        save.TARG.as_slice_mut(),
                    );

                    spicelib::MTXV(
                        save.XFORM.as_slice(),
                        save.CONDIR.as_slice(),
                        save.VTEMP.as_slice_mut(),
                    );
                    spicelib::VEQU(save.VTEMP.as_slice(), save.RAYDIR.as_slice_mut());

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    fstr::assign(&mut save.TITLE, b"Intercept case: I = #; J = #; K = #");
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

                    testutil::TCASE(&save.TITLE, ctx)?;

                    spicelib::INCNSG(
                        save.APEX.as_slice(),
                        save.AXIS.as_slice(),
                        save.ANGLE,
                        save.ENDPT1.as_slice(),
                        save.ENDPT2.as_slice(),
                        &mut save.NXPTS,
                        save.XPT1.as_slice_mut(),
                        save.XPT2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSI(b"NXPTS", save.NXPTS, b">", 0, 0, OK, ctx)?;

                    if (save.NXPTS >= 1) {
                        //
                        // Check XPT1.
                        //
                        // Make sure XPT1 is on the ray containing the
                        // line segment.
                        //
                        spicelib::NPLNPT(
                            save.VERTEX.as_slice(),
                            save.RAYDIR.as_slice(),
                            save.XPT1.as_slice(),
                            save.PNEAR.as_slice_mut(),
                            &mut save.DIST,
                            ctx,
                        )?;

                        save.TOL = TIGHT;

                        testutil::CHCKAD(
                            b"PNEAR",
                            save.PNEAR.as_slice(),
                            b"~~/",
                            save.XPT1.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        //
                        // Make sure XPT1 is on the correct side of the ray's
                        // vertex.
                        //
                        spicelib::VSUB(
                            save.XPT1.as_slice(),
                            save.VERTEX.as_slice(),
                            save.EP1OFF.as_slice_mut(),
                        );

                        save.DP = spicelib::VDOT(save.EP1OFF.as_slice(), save.RAYDIR.as_slice());

                        testutil::CHCKSD(b"DP 1", save.DP, b">", 0.0, 0.0, OK, ctx)?;

                        //
                        // Make sure XPT1 is on the cone's surface.
                        //
                        spicelib::VSUB(
                            save.XPT1.as_slice(),
                            save.APEX.as_slice(),
                            save.OFFSET.as_slice_mut(),
                        );

                        save.SEP =
                            spicelib::VSEP(save.OFFSET.as_slice(), save.AXIS.as_slice(), ctx);

                        save.TOL = TIGHT;

                        testutil::CHCKSD(
                            b"XPT1 SEP",
                            save.SEP,
                            b"~",
                            save.ANGLE,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        if (save.NXPTS == 2) {
                            //
                            // Check XPT2.
                            //
                            // Make sure XPT2 is on the ray containing the
                            // line segment.
                            //
                            spicelib::NPLNPT(
                                save.VERTEX.as_slice(),
                                save.RAYDIR.as_slice(),
                                save.XPT2.as_slice(),
                                save.PNEAR.as_slice_mut(),
                                &mut save.DIST,
                                ctx,
                            )?;

                            save.TOL = TIGHT;

                            testutil::CHCKAD(
                                b"PNEAR",
                                save.PNEAR.as_slice(),
                                b"~~/",
                                save.XPT2.as_slice(),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;

                            //
                            // Make sure XPT2 is on the correct side of the ray's
                            // vertex.
                            //
                            spicelib::VSUB(
                                save.XPT2.as_slice(),
                                save.VERTEX.as_slice(),
                                save.EP2OFF.as_slice_mut(),
                            );

                            save.DP =
                                spicelib::VDOT(save.EP2OFF.as_slice(), save.RAYDIR.as_slice());

                            testutil::CHCKSD(b"DP 2", save.DP, b">", 0.0, 0.0, OK, ctx)?;

                            //
                            // Check that XPT2 is farther from VERTEX than XPT1.
                            //
                            testutil::CHCKSD(
                                b"XPT2 offset",
                                spicelib::VNORM(save.EP2OFF.as_slice()),
                                b">",
                                spicelib::VNORM(save.EP1OFF.as_slice()),
                                0.0,
                                OK,
                                ctx,
                            )?;

                            //
                            // Make sure XPT2 is on the cone's surface.
                            //
                            spicelib::VSUB(
                                save.XPT2.as_slice(),
                                save.APEX.as_slice(),
                                save.OFFSET.as_slice_mut(),
                            );

                            save.SEP =
                                spicelib::VSEP(save.OFFSET.as_slice(), save.AXIS.as_slice(), ctx);

                            save.TOL = TIGHT;

                            testutil::CHCKSD(
                                b"XPT2 SEP",
                                save.SEP,
                                b"~",
                                save.ANGLE,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }
                    }

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    fstr::assign(
                        &mut save.TITLE,
                        b"Intercept case with reversed endpoints: I = #; J = #; K = #",
                    );
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

                    testutil::TCASE(&save.TITLE, ctx)?;

                    spicelib::INCNSG(
                        save.APEX.as_slice(),
                        save.AXIS.as_slice(),
                        save.ANGLE,
                        save.ENDPT2.as_slice(),
                        save.ENDPT1.as_slice(),
                        &mut save.NXPTS,
                        save.XPT1.as_slice_mut(),
                        save.XPT2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSI(b"NXPTS", save.NXPTS, b">", 0, 0, OK, ctx)?;

                    if (save.NXPTS >= 1) {
                        //
                        // Check XPT1.
                        //
                        // Make sure XPT1 is on the ray containing the
                        // line segment.
                        //
                        spicelib::NPLNPT(
                            save.VERTEX.as_slice(),
                            save.RAYDIR.as_slice(),
                            save.XPT1.as_slice(),
                            save.PNEAR.as_slice_mut(),
                            &mut save.DIST,
                            ctx,
                        )?;

                        save.TOL = MED;

                        testutil::CHCKAD(
                            b"PNEAR",
                            save.PNEAR.as_slice(),
                            b"~~/",
                            save.XPT1.as_slice(),
                            3,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        //
                        // Make sure XPT1 is on the correct side of the ray's
                        // vertex.
                        //
                        spicelib::VSUB(
                            save.XPT1.as_slice(),
                            save.VERTEX.as_slice(),
                            save.EP1OFF.as_slice_mut(),
                        );

                        save.DP = spicelib::VDOT(save.EP1OFF.as_slice(), save.RAYDIR.as_slice());

                        testutil::CHCKSD(b"DP 1", save.DP, b">", 0.0, 0.0, OK, ctx)?;

                        //
                        // Make sure XPT1 is on the cone's surface.
                        //
                        spicelib::VSUB(
                            save.XPT1.as_slice(),
                            save.APEX.as_slice(),
                            save.OFFSET.as_slice_mut(),
                        );

                        save.SEP =
                            spicelib::VSEP(save.OFFSET.as_slice(), save.AXIS.as_slice(), ctx);

                        save.TOL = MED;

                        testutil::CHCKSD(
                            b"XPT1 SEP",
                            save.SEP,
                            b"~",
                            save.ANGLE,
                            save.TOL,
                            OK,
                            ctx,
                        )?;

                        if (save.NXPTS == 2) {
                            //
                            // Check XPT2.
                            //
                            // Make sure XPT2 is on the ray containing the
                            // line segment.
                            //
                            spicelib::NPLNPT(
                                save.VERTEX.as_slice(),
                                save.RAYDIR.as_slice(),
                                save.XPT2.as_slice(),
                                save.PNEAR.as_slice_mut(),
                                &mut save.DIST,
                                ctx,
                            )?;

                            save.TOL = TIGHT;

                            testutil::CHCKAD(
                                b"PNEAR",
                                save.PNEAR.as_slice(),
                                b"~~/",
                                save.XPT2.as_slice(),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;

                            //
                            // Make sure XPT2 is on the correct side of the ray's
                            // vertex.
                            //
                            spicelib::VSUB(
                                save.XPT2.as_slice(),
                                save.VERTEX.as_slice(),
                                save.EP2OFF.as_slice_mut(),
                            );

                            save.DP =
                                spicelib::VDOT(save.EP2OFF.as_slice(), save.RAYDIR.as_slice());

                            testutil::CHCKSD(b"DP 2", save.DP, b">", 0.0, 0.0, OK, ctx)?;

                            //
                            // Check that XPT1 is farther from VERTEX than XPT2.
                            //
                            testutil::CHCKSD(
                                b"XPT1 offset",
                                spicelib::VNORM(save.EP1OFF.as_slice()),
                                b">",
                                spicelib::VNORM(save.EP2OFF.as_slice()),
                                0.0,
                                OK,
                                ctx,
                            )?;

                            //
                            // Make sure XPT2 is on the cone's surface.
                            //
                            spicelib::VSUB(
                                save.XPT2.as_slice(),
                                save.APEX.as_slice(),
                                save.OFFSET.as_slice_mut(),
                            );

                            save.SEP =
                                spicelib::VSEP(save.OFFSET.as_slice(), save.AXIS.as_slice(), ctx);

                            save.TOL = MED;

                            testutil::CHCKSD(
                                b"XPT2 SEP",
                                save.SEP,
                                b"~",
                                save.ANGLE,
                                save.TOL,
                                OK,
                                ctx,
                            )?;
                        }
                    }

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    fstr::assign(&mut save.TITLE, b"Intercept case with second endpoint in cone\'s interior. I = #; J = #; K = #");
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

                    testutil::TCASE(&save.TITLE, ctx)?;

                    //
                    // Find the first intercept. Extend the segment slightly
                    // to create the second endpoint.
                    //
                    spicelib::INCNSG(
                        save.APEX.as_slice(),
                        save.AXIS.as_slice(),
                        save.ANGLE,
                        save.ENDPT1.as_slice(),
                        save.ENDPT2.as_slice(),
                        &mut save.NXPTS,
                        save.XPT1.as_slice_mut(),
                        save.XPT2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSI(b"(0) NXPTS", save.NXPTS, b">", 0, 0, OK, ctx)?;

                    spicelib::VSUB(
                        save.XPT1.as_slice(),
                        save.ENDPT1.as_slice(),
                        save.EP1OFF.as_slice_mut(),
                    );

                    spicelib::VLCOM(
                        1.0,
                        save.ENDPT1.as_slice(),
                        (1.0 + 0.001),
                        save.EP1OFF.as_slice(),
                        save.EP2.as_slice_mut(),
                    );

                    //
                    // On this second call to INCNSG, use the new value
                    // of the second endpoint.
                    //
                    spicelib::INCNSG(
                        save.APEX.as_slice(),
                        save.AXIS.as_slice(),
                        save.ANGLE,
                        save.ENDPT1.as_slice(),
                        save.EP2.as_slice(),
                        &mut save.NXPTS,
                        save.XPT1.as_slice_mut(),
                        save.XPT2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // We expect exactly one hit.
                    //
                    testutil::CHCKSI(b"(1) NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    fstr::assign(&mut save.TITLE, b"Intercept case with first endpoint in cone\'s interior. The endpoints are reversed relative to those of the previous case. I = #; J = #; K = #.");
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

                    testutil::TCASE(&save.TITLE, ctx)?;

                    //
                    // Find the first intercept. Extend the segment slightly
                    // to create the second endpoint.
                    //
                    spicelib::INCNSG(
                        save.APEX.as_slice(),
                        save.AXIS.as_slice(),
                        save.ANGLE,
                        save.ENDPT1.as_slice(),
                        save.ENDPT2.as_slice(),
                        &mut save.NXPTS,
                        save.XPT1.as_slice_mut(),
                        save.XPT2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSI(b"(0) NXPTS", save.NXPTS, b">", 0, 0, OK, ctx)?;

                    spicelib::VSUB(
                        save.XPT1.as_slice(),
                        save.ENDPT1.as_slice(),
                        save.EP1OFF.as_slice_mut(),
                    );

                    spicelib::VLCOM(
                        1.0,
                        save.ENDPT1.as_slice(),
                        (1.0 + 0.001),
                        save.EP1OFF.as_slice(),
                        save.EP2.as_slice_mut(),
                    );

                    //
                    // On this second call to INCNSG, use the new value
                    // of the second endpoint as the first endpoint.
                    //
                    spicelib::INCNSG(
                        save.APEX.as_slice(),
                        save.AXIS.as_slice(),
                        save.ANGLE,
                        save.EP2.as_slice(),
                        save.ENDPT1.as_slice(),
                        &mut save.NXPTS,
                        save.XPT1.as_slice_mut(),
                        save.XPT2.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // We expect exactly one hit.
                    //
                    testutil::CHCKSI(b"(1) NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    if (save.K == ((save.NSPAN + 1) / 2)) {
                        //
                        // Perform this test only for rays contained in the
                        // plane containing ENDPT1 and AXIS; this simplifies
                        // the geometry.
                        //

                        fstr::assign(&mut save.TITLE, b"Intercept case with first endpoint in cone\'s interior. I = #; J = #; K = #");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

                        testutil::TCASE(&save.TITLE, ctx)?;

                        //
                        // Find the first intercept. Extend the segment slightly
                        // to create the new value of the first endpoint.
                        //
                        spicelib::INCNSG(
                            save.APEX.as_slice(),
                            save.AXIS.as_slice(),
                            save.ANGLE,
                            save.ENDPT1.as_slice(),
                            save.ENDPT2.as_slice(),
                            &mut save.NXPTS,
                            save.XPT1.as_slice_mut(),
                            save.XPT2.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSI(b"(0) NXPTS", save.NXPTS, b">", 0, 0, OK, ctx)?;

                        spicelib::VSUB(
                            save.XPT1.as_slice(),
                            save.ENDPT1.as_slice(),
                            save.EP1OFF.as_slice_mut(),
                        );

                        spicelib::VLCOM(
                            1.0,
                            save.ENDPT1.as_slice(),
                            (1.0 + 0.001),
                            save.EP1OFF.as_slice(),
                            save.EP1.as_slice_mut(),
                        );

                        //
                        // On this second call to INCNSG, use the new value of
                        // the first endpoint.
                        //
                        spicelib::INCNSG(
                            save.APEX.as_slice(),
                            save.AXIS.as_slice(),
                            save.ANGLE,
                            save.EP1.as_slice(),
                            save.ENDPT2.as_slice(),
                            &mut save.NXPTS,
                            save.XPT1.as_slice_mut(),
                            save.XPT2.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        save.MARGIN = 0.0000000001;

                        if (save.LAT < ((spicelib::HALFPI(ctx) - save.ANGLE) - save.MARGIN)) {
                            //
                            // We expect exactly one hit.
                            //
                            testutil::CHCKSI(b"(1) NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;
                        } else if (save.LAT > ((spicelib::HALFPI(ctx) - save.ANGLE) + save.MARGIN))
                        {
                            //
                            // There should be no hits; the ray rises more steeply
                            // than the cone.
                            //
                            testutil::CHCKSI(b"(1) NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;
                        }

                        //
                        // --- Case: ------------------------------------------------------
                        //
                        //                 Perform this test only for rays contained in the
                        //                 plane containing ENDPT1 and AXIS; this simplifies
                        //                 the geometry.
                        //

                        fstr::assign(&mut save.TITLE, b"Intercept case with second endpoint in cone\'s interior. This is a repeat of the previous case, with endpoints switched. I = #; J = #; K = #");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

                        testutil::TCASE(&save.TITLE, ctx)?;

                        //
                        // On this second call to INCNSG, use the new value of
                        // the first endpoint.
                        //
                        spicelib::INCNSG(
                            save.APEX.as_slice(),
                            save.AXIS.as_slice(),
                            save.ANGLE,
                            save.ENDPT2.as_slice(),
                            save.EP1.as_slice(),
                            &mut save.NXPTS,
                            save.XPT1.as_slice_mut(),
                            save.XPT2.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        save.MARGIN = 0.0000000001;

                        if (save.LAT < ((spicelib::HALFPI(ctx) - save.ANGLE) - save.MARGIN)) {
                            //
                            // We expect exactly one hit.
                            //
                            testutil::CHCKSI(b"(1) NXPTS", save.NXPTS, b"=", 1, 0, OK, ctx)?;
                        } else if (save.LAT > ((spicelib::HALFPI(ctx) - save.ANGLE) + save.MARGIN))
                        {
                            //
                            // There should be no hits; the ray rises more steeply
                            // than the cone.
                            //
                            testutil::CHCKSI(b"(1) NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;
                        }
                    }

                    save.K += m3__;
                }
            }

            //
            // --- Case: ------------------------------------------------------
            //
            //           Miss case: make the target too far off in the Y direction
            //           for an intercept to exist.
            //
            save.K = 0;
            save.TARGY = (-(save.SPAN / 2 as f64) + (((save.K - 1) as f64) * save.DSPAN));

            fstr::assign(
                &mut save.TITLE,
                b"Non-intercept case: Y < -SPAN/2. I = #; J = #; K = #",
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::VPACK(0.0, save.TARGY, save.H, save.TARG.as_slice_mut());

            spicelib::VSUB(
                save.TARG.as_slice(),
                save.CONVTX.as_slice(),
                save.CONDIR.as_slice_mut(),
            );

            spicelib::VHATIP(save.CONDIR.as_slice_mut());

            spicelib::VEQU(save.CONVTX.as_slice(), save.CONPT1.as_slice_mut());

            //
            // Let R2 be the distance of the second endpoint from
            // the vertex.
            //
            save.R2 = 100000.0;
            spicelib::VLCOM(
                1.0,
                save.CONPT1.as_slice(),
                save.R2,
                save.CONDIR.as_slice(),
                save.CONPT2.as_slice_mut(),
            );

            //
            // Shift the endpoints to make them relative to the origin
            // of the standard frame. Convert the endpoints back to the
            // standard basis.
            //
            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.CONPT1.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.ENDPT1.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.CONPT2.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.ENDPT2.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.TARG.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.TARG.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.CONVTX.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.VERTEX.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.RAYDIR.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VEQU(save.VTEMP.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::INCNSG(
                save.APEX.as_slice(),
                save.AXIS.as_slice(),
                save.ANGLE,
                save.ENDPT1.as_slice(),
                save.ENDPT2.as_slice(),
                &mut save.NXPTS,
                save.XPT1.as_slice_mut(),
                save.XPT2.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect a miss.
            //
            testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;

            //
            // --- Case: ------------------------------------------------------
            //
            //           Miss case: make the target too far off in the Y direction
            //           for an intercept to exist.
            //
            save.K = (save.NSPAN + 1);
            save.TARGY = (((save.NSPAN / 2) as f64) * save.DSPAN);

            fstr::assign(
                &mut save.TITLE,
                b"Non-intercept case: Y > SPAN/2. I = #; J = #; K = #",
            );
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.K, &mut save.TITLE, ctx);

            testutil::TCASE(&save.TITLE, ctx)?;

            spicelib::VPACK(0.0, save.TARGY, save.H, save.TARG.as_slice_mut());

            spicelib::VSUB(
                save.TARG.as_slice(),
                save.CONVTX.as_slice(),
                save.RAYDIR.as_slice_mut(),
            );

            spicelib::VHATIP(save.RAYDIR.as_slice_mut());

            spicelib::VEQU(save.CONVTX.as_slice(), save.CONPT1.as_slice_mut());

            //
            // Let R2 be the distance of the second endpoint from
            // the vertex.
            //
            save.R2 = 100000.0;
            spicelib::VLCOM(
                1.0,
                save.CONPT1.as_slice(),
                save.R2,
                save.CONDIR.as_slice(),
                save.CONPT2.as_slice_mut(),
            );

            //
            // Shift the endpoints to make them relative to the origin
            // of the standard frame. Convert the endpoints back to the
            // standard basis.
            //
            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.CONPT1.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.ENDPT1.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.CONPT2.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.ENDPT2.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.TARG.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.TARG.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.CONVTX.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VADD(
                save.VTEMP.as_slice(),
                save.APEX.as_slice(),
                save.VERTEX.as_slice_mut(),
            );

            spicelib::MTXV(
                save.XFORM.as_slice(),
                save.RAYDIR.as_slice(),
                save.VTEMP.as_slice_mut(),
            );
            spicelib::VEQU(save.VTEMP.as_slice(), save.RAYDIR.as_slice_mut());

            spicelib::INCNSG(
                save.APEX.as_slice(),
                save.AXIS.as_slice(),
                save.ANGLE,
                save.ENDPT1.as_slice(),
                save.ENDPT2.as_slice(),
                &mut save.NXPTS,
                save.XPT1.as_slice_mut(),
                save.XPT2.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We expect a miss.
            //
            testutil::CHCKSI(b"NXPTS", save.NXPTS, b"=", 0, 0, OK, ctx)?;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
