//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;
const MEDTOL: f64 = 0.000000001;
const LOOSE: f64 = 0.00001;
const LNSIZE: i32 = 255;
const NSHAPE: i32 = 5;
const NALT: i32 = 5;
const MINALT: i32 = 2;
const NSCALE: i32 = 8;
const MINSCL: i32 = 4;
const NU: i32 = 5;
const NR: i32 = 6;
const MINR: i32 = 3;

struct SaveVars {
    TITLE: Vec<u8>,
    ALT: f64,
    ANGLE: f64,
    DIST: f64,
    DIR: StackArray<f64, 3>,
    EDGE: StackArray2D<f64, 9>,
    EDGNML: StackArray2D<f64, 9>,
    L: f64,
    NORMAL: StackArray<f64, 3>,
    PNEAR: StackArray<f64, 3>,
    OFFSET: StackArray<f64, 3>,
    POINT: StackArray<f64, 3>,
    R: f64,
    S: f64,
    SCALE: f64,
    SEP: f64,
    SHAPES: StackArray3D<f64, 45>,
    TOL: f64,
    UEDGE: StackArray<f64, 3>,
    V1: StackArray<f64, 3>,
    V2: StackArray<f64, 3>,
    V3: StackArray<f64, 3>,
    VTEMP: StackArray<f64, 3>,
    XDIST: f64,
    XFORM: StackArray2D<f64, 9>,
    XPNEAR: StackArray<f64, 3>,
    XSHAPE: StackArray2D<f64, 9>,
    NEXT: StackArray<i32, 3>,
    PRED: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut ALT: f64 = 0.0;
        let mut ANGLE: f64 = 0.0;
        let mut DIST: f64 = 0.0;
        let mut DIR = StackArray::<f64, 3>::new(1..=3);
        let mut EDGE = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut EDGNML = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut L: f64 = 0.0;
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut POINT = StackArray::<f64, 3>::new(1..=3);
        let mut R: f64 = 0.0;
        let mut S: f64 = 0.0;
        let mut SCALE: f64 = 0.0;
        let mut SEP: f64 = 0.0;
        let mut SHAPES = StackArray3D::<f64, 45>::new(1..=3, 1..=3, 1..=NSHAPE);
        let mut TOL: f64 = 0.0;
        let mut UEDGE = StackArray::<f64, 3>::new(1..=3);
        let mut V1 = StackArray::<f64, 3>::new(1..=3);
        let mut V2 = StackArray::<f64, 3>::new(1..=3);
        let mut V3 = StackArray::<f64, 3>::new(1..=3);
        let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
        let mut XDIST: f64 = 0.0;
        let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XPNEAR = StackArray::<f64, 3>::new(1..=3);
        let mut XSHAPE = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut PRED = StackArray::<i32, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(3), Val::I(1), Val::I(2)].into_iter();
            PRED.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TITLE,
            ALT,
            ANGLE,
            DIST,
            DIR,
            EDGE,
            EDGNML,
            L,
            NORMAL,
            PNEAR,
            OFFSET,
            POINT,
            R,
            S,
            SCALE,
            SEP,
            SHAPES,
            TOL,
            UEDGE,
            V1,
            V2,
            V3,
            VTEMP,
            XDIST,
            XFORM,
            XPNEAR,
            XSHAPE,
            NEXT,
            PRED,
        }
    }
}

//$Procedure F_PLTNP ( PLTNP tests )
pub fn F_PLTNP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_PLTNP", ctx)?;

    //**********************************************************************
    //
    //     Non-error exceptional cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Degenerate case: plate is a single point.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V1.as_slice_mut());
    spicelib::VEQU(save.V1.as_slice(), save.V2.as_slice_mut());
    spicelib::VEQU(save.V1.as_slice(), save.V3.as_slice_mut());

    spicelib::VPACK(0.0, 0.0, 5.0, save.POINT.as_slice_mut());

    spicelib::VEQU(save.V1.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 5.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(
        &mut save.TITLE,
        b"Degenerate case: plate is a single point. Input point coincides with plate.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V1.as_slice_mut());
    spicelib::VEQU(save.V1.as_slice(), save.V2.as_slice_mut());
    spicelib::VEQU(save.V1.as_slice(), save.V3.as_slice_mut());

    spicelib::VEQU(save.V1.as_slice(), save.POINT.as_slice_mut());

    spicelib::VEQU(save.V1.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 0.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
        b"~~",
        save.XPNEAR.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Degenerate case: plate is a line segment bounded by V1 and V2.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V1.as_slice_mut());
    spicelib::VPACK(6.0, 8.0, 10.0, save.V2.as_slice_mut());

    spicelib::VLCOM(
        0.5,
        save.V1.as_slice(),
        0.5,
        save.V2.as_slice(),
        save.V3.as_slice_mut(),
    );

    spicelib::VPACK(0.0, 0.0, 5.0, save.POINT.as_slice_mut());

    spicelib::VEQU(save.V1.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 5.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Degenerate case: plate is a line segment bounded by V1 and V2. Input point is interior of segment. ");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V1.as_slice_mut());
    spicelib::VPACK(6.0, 8.0, 10.0, save.V2.as_slice_mut());

    spicelib::VEQU(save.V2.as_slice(), save.V3.as_slice_mut());

    spicelib::VLCOM(
        0.5,
        save.V1.as_slice(),
        0.5,
        save.V2.as_slice(),
        save.POINT.as_slice_mut(),
    );

    spicelib::VEQU(save.POINT.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 0.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(
        &mut save.TITLE,
        b"Degenerate case: plate is a line segment bounded by V1 and V3.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V1.as_slice_mut());
    spicelib::VPACK(6.0, 8.0, 10.0, save.V3.as_slice_mut());

    spicelib::VLCOM(
        0.5,
        save.V1.as_slice(),
        0.5,
        save.V3.as_slice(),
        save.V2.as_slice_mut(),
    );

    spicelib::VPACK(0.0, 0.0, 5.0, save.POINT.as_slice_mut());

    spicelib::VEQU(save.V1.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 5.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Degenerate case: plate is a line segment bounded by V1 and V3. Input point is interior of segment. ");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V1.as_slice_mut());
    spicelib::VPACK(6.0, 8.0, 10.0, save.V3.as_slice_mut());

    spicelib::VLCOM(
        0.5,
        save.V1.as_slice(),
        0.5,
        save.V3.as_slice(),
        save.V2.as_slice_mut(),
    );

    spicelib::VLCOM(
        0.5,
        save.V1.as_slice(),
        0.5,
        save.V2.as_slice(),
        save.POINT.as_slice_mut(),
    );

    spicelib::VEQU(save.POINT.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 0.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(
        &mut save.TITLE,
        b"Degenerate case: plate is a line segment bounded by V2 and V3.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V2.as_slice_mut());
    spicelib::VPACK(6.0, 8.0, 10.0, save.V3.as_slice_mut());

    spicelib::VLCOM(
        0.5,
        save.V2.as_slice(),
        0.5,
        save.V3.as_slice(),
        save.V1.as_slice_mut(),
    );

    spicelib::VPACK(0.0, 0.0, 5.0, save.POINT.as_slice_mut());

    spicelib::VEQU(save.V2.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 5.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Degenerate case: plate is a line segment bounded by V2 and V3. Input point is interior of segment. ");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 4.0, 5.0, save.V2.as_slice_mut());
    spicelib::VPACK(6.0, 8.0, 10.0, save.V3.as_slice_mut());

    spicelib::VLCOM(
        0.5,
        save.V2.as_slice(),
        0.5,
        save.V3.as_slice(),
        save.V1.as_slice_mut(),
    );

    spicelib::VLCOM(
        0.5,
        save.V2.as_slice(),
        0.5,
        save.V3.as_slice(),
        save.POINT.as_slice_mut(),
    );

    spicelib::VEQU(save.POINT.as_slice(), save.XPNEAR.as_slice_mut());
    save.XDIST = 0.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    //
    // Simple tests with easily verified results:
    //
    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(
        &mut save.TITLE,
        b"Trivial case: plate is in X-Y plane; point is on +Z axis.",
    );

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(-1.0, -1.0, 0.0, save.V1.as_slice_mut());
    spicelib::VPACK(1.0, -1.0, 0.0, save.V2.as_slice_mut());
    spicelib::VPACK(0.0, 1.0, 0.0, save.V3.as_slice_mut());

    spicelib::VPACK(0.0, 0.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, save.XPNEAR.as_slice_mut());

    save.XDIST = 3.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    // The following cases use the triangle defined above.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is below (in the -Y sense) the bottom edge of the plate.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(0.0, -2.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VPACK(0.0, -1.0, 0.0, save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(10.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is below the bottom edge of the plate and shifted in the -X direction.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(-2.0, -2.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VEQU(save.V1.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(11.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is below the bottom edge of the plate and shifted in the +X direction.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(2.0, -2.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VEQU(save.V2.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(11.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is above (in the +Y sense) the bottom edge of the plate but below outward normal to the left edge emanating from the lower left corner.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(-4.0, 0.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VEQU(save.V1.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(19.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is above (in the +Y sense) the bottom edge of the plate but below outward normal to the right edge emanating from the lower right corner.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(4.0, 0.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VEQU(save.V2.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(19.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is on the outward normal to the left edge emanating from the lower left corner.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(-3.0, 0.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VEQU(save.V1.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(14.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is on the outward normal to the right edge emanating from the lower right corner.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(3.0, 0.0, 3.0, save.POINT.as_slice_mut());
    spicelib::VEQU(save.V2.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(14.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is above (in the +Y sense) the outward normal to the left edge emanating from the lower left corner.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(0.5, 1.0, 0.0, save.OFFSET.as_slice_mut());

    spicelib::VADD(
        save.V1.as_slice(),
        save.OFFSET.as_slice(),
        save.XPNEAR.as_slice_mut(),
    );

    spicelib::VPACK(-1.0, 0.5, 0.0, save.NORMAL.as_slice_mut());

    spicelib::VLCOM3(
        1.0,
        save.V1.as_slice(),
        1.0,
        save.OFFSET.as_slice(),
        2.0,
        save.NORMAL.as_slice(),
        save.POINT.as_slice_mut(),
    );

    save.POINT[3] = 3.0;

    save.XDIST = f64::sqrt(14.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is above (in the +Y sense) the outward normal to the right edge emanating from the lower right corner.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(-0.5, 1.0, 0.0, save.OFFSET.as_slice_mut());

    spicelib::VADD(
        save.V2.as_slice(),
        save.OFFSET.as_slice(),
        save.XPNEAR.as_slice_mut(),
    );

    spicelib::VPACK(1.0, 0.5, 0.0, save.NORMAL.as_slice_mut());

    spicelib::VLCOM3(
        1.0,
        save.V2.as_slice(),
        1.0,
        save.OFFSET.as_slice(),
        2.0,
        save.NORMAL.as_slice(),
        save.POINT.as_slice_mut(),
    );

    save.POINT[3] = 3.0;

    save.XDIST = f64::sqrt(14.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is on the outward normal to the left edge emanating from the top (in the +Y sense) vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VEQU(save.V3.as_slice(), save.XPNEAR.as_slice_mut());

    spicelib::VPACK(-1.0, 0.5, 0.0, save.NORMAL.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.V3.as_slice(),
        2.0,
        save.NORMAL.as_slice(),
        save.POINT.as_slice_mut(),
    );

    save.POINT[3] = 3.0;

    save.XDIST = f64::sqrt(14.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is on the outward normal to the right edge emanating from the top (in the +Y sense) vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VEQU(save.V3.as_slice(), save.XPNEAR.as_slice_mut());

    spicelib::VPACK(1.0, 0.5, 0.0, save.NORMAL.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.V3.as_slice(),
        2.0,
        save.NORMAL.as_slice(),
        save.POINT.as_slice_mut(),
    );

    save.POINT[3] = 3.0;

    save.XDIST = f64::sqrt(14.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is above (in the +Y sense)the outward normal to the left edge emanating from the top (in the +Y sense) vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VEQU(save.V3.as_slice(), save.XPNEAR.as_slice_mut());

    spicelib::VPACK(-1.0, 0.5, 0.0, save.NORMAL.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.V3.as_slice(),
        2.0,
        save.NORMAL.as_slice(),
        save.POINT.as_slice_mut(),
    );

    save.POINT[2] = (save.POINT[2] + 1.0);
    save.POINT[3] = 3.0;

    save.XDIST = f64::sqrt(17.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is above (in the +Y sense)the outward normal to the right edge emanating from the top (in the +Y sense) vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VEQU(save.V3.as_slice(), save.XPNEAR.as_slice_mut());

    spicelib::VPACK(1.0, 0.5, 0.0, save.NORMAL.as_slice_mut());

    spicelib::VLCOM(
        1.0,
        save.V3.as_slice(),
        2.0,
        save.NORMAL.as_slice(),
        save.POINT.as_slice_mut(),
    );

    save.POINT[2] = (save.POINT[2] + 1.0);
    save.POINT[3] = 3.0;

    save.XDIST = f64::sqrt(17.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Exterior case: plate is in X-Y plane; X-Y plane projection of point is above (in the +Y sense)the top vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(0.0, 3.0, 3.0, save.POINT.as_slice_mut());

    spicelib::VEQU(save.V3.as_slice(), save.XPNEAR.as_slice_mut());

    save.XDIST = f64::sqrt(13.0);

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    // "Interior" cases follow. For these cases, the projection
    // of the point onto the plane containing the plate is a point
    // on the plate.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Interior case: plate is in X-Y plane; X-Y plane projection of point is below (in the -Y sense)the top vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(0.0, 0.9, 3.0, save.POINT.as_slice_mut());

    spicelib::VEQU(save.POINT.as_slice(), save.XPNEAR.as_slice_mut());
    save.XPNEAR[3] = 0.0;

    save.XDIST = 3.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Interior case: plate is in X-Y plane; X-Y plane projection of point is above (in the -Y sense)and to the right of the lower left vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(0.1, 0.1, 3.0, save.OFFSET.as_slice_mut());

    spicelib::VADD(
        save.V1.as_slice(),
        save.OFFSET.as_slice(),
        save.POINT.as_slice_mut(),
    );

    spicelib::VEQU(save.POINT.as_slice(), save.XPNEAR.as_slice_mut());
    save.XPNEAR[3] = 0.0;

    save.XDIST = 3.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    fstr::assign(&mut save.TITLE, b"Interior case: plate is in X-Y plane; X-Y plane projection of point is above (in the -Y sense)and to the left of the lower right vertex.");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::VPACK(-0.1, 0.1, 3.0, save.OFFSET.as_slice_mut());

    spicelib::VADD(
        save.V2.as_slice(),
        save.OFFSET.as_slice(),
        save.POINT.as_slice_mut(),
    );

    spicelib::VEQU(save.POINT.as_slice(), save.XPNEAR.as_slice_mut());
    save.XPNEAR[3] = 0.0;

    save.XDIST = 3.0;

    spicelib::PLTNP(
        save.POINT.as_slice(),
        save.V1.as_slice(),
        save.V2.as_slice(),
        save.V3.as_slice(),
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
    //
    //
    // Systematic tests:
    //
    //
    //

    //
    // --- Case --------------------------------------------------------
    //
    fstr::assign(&mut save.TITLE, b"Setup");

    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Create basic shapes. All shapes are variants of triangles.
    //
    //    Shape 1 is an acute triangle.
    //
    spicelib::VPACK(-1.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 1, 1]));
    spicelib::VPACK(1.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 2, 1]));
    spicelib::VPACK(0.0, 1.0, 0.0, save.SHAPES.subarray_mut([1, 3, 1]));

    //
    // Shape 2 is a right triangle.
    //
    spicelib::VPACK(-1.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 1, 2]));
    spicelib::VPACK(1.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 2, 2]));
    spicelib::VPACK(-1.0, 1.0, 0.0, save.SHAPES.subarray_mut([1, 3, 2]));

    //
    // Shape 3 is an obtuse triangle.
    //
    spicelib::VPACK(-4.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 1, 3]));
    spicelib::VPACK(4.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 2, 3]));
    spicelib::VPACK(0.0, 1.0, 0.0, save.SHAPES.subarray_mut([1, 3, 3]));

    //
    // Shape 4 is a needle-like, acute triangle.
    //
    spicelib::VPACK(-1.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 1, 4]));
    spicelib::VPACK(1.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 2, 4]));
    spicelib::VPACK(0.0, 10000.0, 0.0, save.SHAPES.subarray_mut([1, 3, 4]));

    //
    // Shape 5 is an almost flat, obtuse triangle.
    //
    spicelib::VPACK(-10000.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 1, 5]));
    spicelib::VPACK(10000.0, -1.0, 0.0, save.SHAPES.subarray_mut([1, 2, 5]));
    spicelib::VPACK(0.0, 1.0, 0.0, save.SHAPES.subarray_mut([1, 3, 5]));

    //
    // Create an offset vector and a transformation; we'll use
    // these to move the shapes away from the origin and from
    // the X-Y plane.
    //
    spicelib::VPACK(-17.0, 13.0, 23.0, save.OFFSET.as_slice_mut());

    spicelib::EUL2M(
        (30.0 * spicelib::RPD(ctx)),
        -(10.0 * spicelib::RPD(ctx)),
        (70.0 * spicelib::RPD(ctx)),
        3,
        2,
        3,
        save.XFORM.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //

    //
    // Loop over the triangles.
    //
    for SHAPIX in 1..=NSHAPE {
        //
        // Loop over the scales.
        //
        for SCALIX in 1..=NSCALE {
            //
            // Translate and transform the triangle.
            //
            for I in 1..=3 {
                spicelib::VADD(
                    save.OFFSET.as_slice(),
                    save.SHAPES.subarray([1, I, SHAPIX]),
                    save.VTEMP.as_slice_mut(),
                );
                spicelib::MXV(
                    save.XFORM.as_slice(),
                    save.VTEMP.as_slice(),
                    save.XSHAPE.subarray_mut([1, I]),
                );
            }

            save.SCALE = f64::powf(10.0, (4.0 * (SCALIX - MINSCL) as f64));

            //
            // Scale the triangle.
            //
            for I in 1..=3 {
                spicelib::VSCLIP(save.SCALE, save.XSHAPE.subarray_mut([1, I]));
            }

            //
            // Compute the triangle's edges.
            //
            spicelib::VSUB(
                save.XSHAPE.subarray([1, 2]),
                save.XSHAPE.subarray([1, 1]),
                save.EDGE.subarray_mut([1, 1]),
            );
            spicelib::VSUB(
                save.XSHAPE.subarray([1, 3]),
                save.XSHAPE.subarray([1, 2]),
                save.EDGE.subarray_mut([1, 2]),
            );
            spicelib::VSUB(
                save.XSHAPE.subarray([1, 1]),
                save.XSHAPE.subarray([1, 3]),
                save.EDGE.subarray_mut([1, 3]),
            );

            //
            // Compute a normal to the plane of the triangle. Pick the
            // normal so the vertices are ordered in the positive sense
            // about the normal.
            //
            spicelib::UCRSS(
                save.EDGE.subarray([1, 1]),
                save.EDGE.subarray([1, 2]),
                save.NORMAL.as_slice_mut(),
            );

            //
            // Compute the outward normal vectors for the edges, where the
            // normal vectors lie in the plane of the triangle.
            //
            spicelib::UCRSS(
                save.EDGE.subarray([1, 1]),
                save.NORMAL.as_slice(),
                save.EDGNML.subarray_mut([1, 1]),
            );
            spicelib::UCRSS(
                save.EDGE.subarray([1, 2]),
                save.NORMAL.as_slice(),
                save.EDGNML.subarray_mut([1, 2]),
            );
            spicelib::UCRSS(
                save.EDGE.subarray([1, 3]),
                save.NORMAL.as_slice(),
                save.EDGNML.subarray_mut([1, 3]),
            );

            //
            // We're going to create points, the distance of which from
            // the current triangle is to be computed.
            //
            // Loop over the point's height values.
            //
            for ALTIX in 1..=NALT {
                save.ALT = (save.SCALE * f64::powf(10.0, ((ALTIX - MINALT) as f64)));

                //
                // Loop over the triangles' vertices.
                //
                for VIX in 1..=3 {
                    //
                    // Compute the angular separation between the
                    // rays emanating from vertex VIX in the directions
                    // of the outward normal to edge VIX and the
                    // normal to edge PRED(VIX). We'll use this later.
                    //
                    save.SEP = spicelib::VSEP(
                        save.EDGNML.subarray([1, VIX]),
                        save.EDGNML.subarray([1, save.PRED[VIX]]),
                        ctx,
                    );
                    //
                    // Compute the length of the edge between vertex
                    // VIX and vertex NEXT(VIX). We'll use this later.
                    //
                    save.L = spicelib::VNORM(save.EDGE.subarray([1, VIX]));

                    for UIX in 0..=NU {
                        //
                        // UIX controls the angular separation of the point
                        // from the normal rays emanating from vertex VIX.
                        //
                        save.ANGLE = (save.SEP * ((UIX as f64) / (NU as f64)));

                        spicelib::VROTV(
                            save.EDGNML.subarray([1, save.PRED[VIX]]),
                            save.NORMAL.as_slice(),
                            save.ANGLE,
                            save.DIR.as_slice_mut(),
                        );
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        for RIX in 1..=NR {
                            //
                            // --- Case --------------------------------------------------------
                            //
                            //
                            //                       Create a point in the plane of the triangle,
                            //                       lying in the sector having apex at vertex VIX
                            //                       and bounded by rays emanating from the apex in
                            //                       directions EDGNML(1,VIX) and
                            //                       EDGNML(1,PRED(VIX)). For such a point, the
                            //                       nearest point on the triangle is the vertex
                            //                       itself. We'll then add an "altitude" to the
                            //                       point.
                            //
                            //                       In the case immediately below, RIX controls the
                            //                       distance of the point from the vertex.
                            //
                            save.R = (save.SCALE * f64::powf(10.0, (2.0 * (RIX - MINR) as f64)));

                            fstr::assign(&mut save.TITLE, b"Near point should be vertex #: SHAPIX = #; SCALIX = #; ALTIX = #; VIX = #; UIX = #, RIX = #; R   = #; ALT = #.");

                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", VIX, &mut save.TITLE, ctx);
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                SHAPIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                SCALIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                ALTIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", VIX, &mut save.TITLE, ctx);
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", UIX, &mut save.TITLE, ctx);
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", RIX, &mut save.TITLE, ctx);
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.R,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.ALT,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::TCASE(&save.TITLE, ctx)?;

                            //
                            // Create the point.
                            //
                            spicelib::VLCOM3(
                                1.0,
                                save.XSHAPE.subarray([1, VIX]),
                                save.R,
                                save.DIR.as_slice(),
                                save.ALT,
                                save.NORMAL.as_slice(),
                                save.POINT.as_slice_mut(),
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Compute the near point on the triangle and
                            // the distance between this point and POINT.
                            //
                            spicelib::PLTNP(
                                save.POINT.as_slice(),
                                save.XSHAPE.subarray([1, 1]),
                                save.XSHAPE.subarray([1, 2]),
                                save.XSHAPE.subarray([1, 3]),
                                save.PNEAR.as_slice_mut(),
                                &mut save.DIST,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Set tolerances based on the numerical
                            // difficulty of the case.
                            //
                            if (SHAPIX <= 3) {
                                save.TOL = TIGHT;
                            } else {
                                save.TOL = LOOSE;
                            }

                            save.XDIST = spicelib::VDIST(
                                save.XSHAPE.subarray([1, VIX]),
                                save.POINT.as_slice(),
                            );

                            testutil::CHCKSD(
                                b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx,
                            )?;

                            if (SHAPIX <= 3) {
                                if (RIX <= (NR - 3)) {
                                    save.TOL = TIGHT;
                                } else {
                                    save.TOL = MEDTOL;
                                }
                            } else {
                                save.TOL = LOOSE;
                            }

                            testutil::CHCKAD(
                                b"PNEAR",
                                save.PNEAR.as_slice(),
                                b"~~/",
                                save.XSHAPE.subarray([1, VIX]),
                                3,
                                save.TOL,
                                OK,
                                ctx,
                            )?;

                            //
                            // --- Case --------------------------------------------------------
                            //
                            //
                            //                       Test points whose near points lie on the
                            //                       edge between vertices VIX and NEXT(VIX).
                            //
                            //                       We can use the values of ALT and R we've
                            //                       already computed. R will be the distance
                            //                       from the edge of the projection of the
                            //                       point onto the plane of the triangle.
                            //
                            //                       S will replace ANGLE in this computation: S
                            //                       will be the distance from the outward normal
                            //                       ray of the edge, emanating from vertex VIX,
                            //                       of the projection of the point onto the plane
                            //                       of the triangle.
                            //
                            save.S = (save.L * ((UIX as f64) / (NU as f64)));

                            fstr::assign(&mut save.TITLE, b"Near point should be on the edge: SHAPIX = #; SCALIX = #; ALTIX = #; VIX = #; UIX = #, RIX = #; R   = #; S = #; ALT = #.");

                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                SHAPIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                SCALIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                ALTIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", VIX, &mut save.TITLE, ctx);
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", UIX, &mut save.TITLE, ctx);
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", RIX, &mut save.TITLE, ctx);
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.R,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.S,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.ALT,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::TCASE(&save.TITLE, ctx)?;

                            //
                            // Generate the point.
                            //
                            // We'll need the unit vector in the direction
                            // of the edge.
                            //
                            spicelib::VHAT(save.EDGE.subarray([1, VIX]), save.UEDGE.as_slice_mut());
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Create the point.
                            //
                            spicelib::VLCOM3(
                                1.0,
                                save.XSHAPE.subarray([1, VIX]),
                                save.R,
                                save.EDGNML.subarray([1, VIX]),
                                save.S,
                                save.UEDGE.as_slice(),
                                save.VTEMP.as_slice_mut(),
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::VLCOM(
                                1.0,
                                save.VTEMP.as_slice(),
                                save.ALT,
                                save.NORMAL.as_slice(),
                                save.POINT.as_slice_mut(),
                            );

                            //
                            // Compute the near point on the triangle and
                            // the distance between this point and POINT.
                            //
                            spicelib::PLTNP(
                                save.POINT.as_slice(),
                                save.XSHAPE.subarray([1, 1]),
                                save.XSHAPE.subarray([1, 2]),
                                save.XSHAPE.subarray([1, 3]),
                                save.PNEAR.as_slice_mut(),
                                &mut save.DIST,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // The expected near point lies on the line
                            // containing the edge.
                            //
                            spicelib::NPLNPT(
                                save.XSHAPE.subarray([1, VIX]),
                                save.UEDGE.as_slice(),
                                save.POINT.as_slice(),
                                save.XPNEAR.as_slice_mut(),
                                &mut save.XDIST,
                                ctx,
                            )?;

                            //
                            // Set tolerances based on the numerical
                            // difficulty of the case.
                            //
                            if (SHAPIX <= 3) {
                                if ((RIX <= (NR - 3)) && (ALTIX <= (NALT - 3))) {
                                    save.TOL = MEDTOL;
                                } else {
                                    save.TOL = LOOSE;
                                }
                            } else {
                                save.TOL = LOOSE;
                            }

                            testutil::CHCKSD(
                                b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx,
                            )?;

                            if (SHAPIX <= 3) {
                                if ((RIX <= (NR - 3)) && (ALTIX <= (NALT - 3))) {
                                    save.TOL = TIGHT;
                                } else {
                                    save.TOL = LOOSE;
                                }
                            } else {
                                save.TOL = LOOSE;
                            }

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
                            // --- Case --------------------------------------------------------
                            //
                            //
                            //                       Test points whose near points lie in the
                            //                       interior of the triangle.
                            //
                            //                       We can use the values of ALT we've already
                            //                       computed. We'll create the projection of the
                            //                       point using linear combinations of the vertices.
                            //
                            fstr::assign(&mut save.TITLE, b"Near point should be in the interior of the triangle: SHAPIX = #; SCALIX = #; ALTIX = #; VIX = #; UIX = #, RIX = #; ALT = #. ");

                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                SHAPIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                SCALIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                ALTIX,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", VIX, &mut save.TITLE, ctx);
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", UIX, &mut save.TITLE, ctx);
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", RIX, &mut save.TITLE, ctx);
                            spicelib::REPMD(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.ALT,
                                14,
                                &mut save.TITLE,
                                ctx,
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            testutil::TCASE(&save.TITLE, ctx)?;

                            //
                            // S ranges from 0 to 1.
                            //
                            save.S = ((UIX as f64) / (NU as f64));

                            //
                            // Make R range from 0 to 1.
                            //
                            save.R = (((RIX - 1) as f64) / ((NR - 1) as f64));

                            //
                            // Generate the projection of the point. This
                            // is the expected near point. Use a convex
                            // combination of the vertices.
                            //
                            spicelib::VLCOM3(
                                (1.0 - save.S),
                                save.XSHAPE.subarray([1, VIX]),
                                (save.S * (1.0 - save.R)),
                                save.XSHAPE.subarray([1, save.NEXT[VIX]]),
                                (save.S * save.R),
                                save.XSHAPE.subarray([1, save.PRED[VIX]]),
                                save.XPNEAR.as_slice_mut(),
                            );

                            //
                            // Generate the point.
                            //
                            spicelib::VLCOM(
                                1.0,
                                save.XPNEAR.as_slice(),
                                save.ALT,
                                save.NORMAL.as_slice(),
                                save.POINT.as_slice_mut(),
                            );

                            //
                            // Compute the near point on the triangle and
                            // the distance between this point and POINT.
                            //
                            spicelib::PLTNP(
                                save.POINT.as_slice(),
                                save.XSHAPE.subarray([1, 1]),
                                save.XSHAPE.subarray([1, 2]),
                                save.XSHAPE.subarray([1, 3]),
                                save.PNEAR.as_slice_mut(),
                                &mut save.DIST,
                                ctx,
                            )?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // The expected distance is just the altitude
                            // of the point with respect to the plane
                            // containing the triangle.
                            //
                            save.XDIST = save.ALT;

                            //
                            // Set tolerances based on the numerical
                            // difficulty of the case.
                            //
                            if (SHAPIX <= 3) {
                                if ((RIX <= (NR - 3)) && (ALTIX <= (NALT - 3))) {
                                    save.TOL = MEDTOL;
                                } else {
                                    save.TOL = LOOSE;
                                }
                            } else {
                                save.TOL = LOOSE;
                            }

                            testutil::CHCKSD(
                                b"DIST", save.DIST, b"~/", save.XDIST, save.TOL, OK, ctx,
                            )?;

                            if (SHAPIX <= 3) {
                                if ((RIX <= (NR - 3)) && (ALTIX <= (NALT - 3))) {
                                    save.TOL = TIGHT;
                                } else {
                                    save.TOL = LOOSE;
                                }
                            } else {
                                save.TOL = LOOSE;
                            }

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
                        }
                        //
                        // End of "RIX" loop.
                        //
                    }
                    //
                    // End of "UIX" loop.
                    //
                }
                //
                // End of vertex loop.
                //
            }
            //
            // End of altitude loop.
            //
        }
        //
        // End of scale loop.
        //
    }
    //
    // End of shape loop.
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
