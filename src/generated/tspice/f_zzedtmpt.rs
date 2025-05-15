//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const TIGHT: f64 = 0.00000000001;
const VTIGHT: f64 = 0.0000000000001;
const LNSIZE: i32 = 255;
const UBPL: i32 = 4;

struct SaveVars {
    TITLE: Vec<u8>,
    LCHAR: ActualCharArray,
    A: f64,
    ALPHA: f64,
    AXIOFF: StackArray<f64, 3>,
    AXIS: StackArray<f64, 3>,
    B: f64,
    BETA: f64,
    C: f64,
    DIST: f64,
    DLAT: f64,
    DLON: f64,
    DOT: f64,
    LAT: f64,
    LON: f64,
    ORIGIN: StackArray<f64, 3>,
    NORMAL: StackArray<f64, 3>,
    PLNVEC: StackArray<f64, 3>,
    POINT: StackArray<f64, 3>,
    PROJ: StackArray<f64, 3>,
    REFPLN: StackArray<f64, 4>,
    SCALE: f64,
    SRCRAD: f64,
    TANPLN: StackArray<f64, 4>,
    THETA: f64,
    TOL: f64,
    X: StackArray<f64, 3>,
    XPOINT: StackArray<f64, 3>,
    Y: StackArray<f64, 3>,
    Z: StackArray<f64, 3>,
    NLAT: i32,
    NLON: i32,
    NSHAPE: i32,
    SEED: i32,
    UMBRAL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut LCHAR = ActualCharArray::new(1, 1..=2);
        let mut A: f64 = 0.0;
        let mut ALPHA: f64 = 0.0;
        let mut AXIOFF = StackArray::<f64, 3>::new(1..=3);
        let mut AXIS = StackArray::<f64, 3>::new(1..=3);
        let mut B: f64 = 0.0;
        let mut BETA: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut DIST: f64 = 0.0;
        let mut DLAT: f64 = 0.0;
        let mut DLON: f64 = 0.0;
        let mut DOT: f64 = 0.0;
        let mut LAT: f64 = 0.0;
        let mut LON: f64 = 0.0;
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
        let mut PLNVEC = StackArray::<f64, 3>::new(1..=3);
        let mut POINT = StackArray::<f64, 3>::new(1..=3);
        let mut PROJ = StackArray::<f64, 3>::new(1..=3);
        let mut REFPLN = StackArray::<f64, 4>::new(1..=UBPL);
        let mut SCALE: f64 = 0.0;
        let mut SRCRAD: f64 = 0.0;
        let mut TANPLN = StackArray::<f64, 4>::new(1..=UBPL);
        let mut THETA: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut X = StackArray::<f64, 3>::new(1..=3);
        let mut XPOINT = StackArray::<f64, 3>::new(1..=3);
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);
        let mut NLAT: i32 = 0;
        let mut NLON: i32 = 0;
        let mut NSHAPE: i32 = 0;
        let mut SEED: i32 = 0;
        let mut UMBRAL: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"T"), Val::C(b"F")].into_iter();
            LCHAR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TITLE,
            LCHAR,
            A,
            ALPHA,
            AXIOFF,
            AXIS,
            B,
            BETA,
            C,
            DIST,
            DLAT,
            DLON,
            DOT,
            LAT,
            LON,
            ORIGIN,
            NORMAL,
            PLNVEC,
            POINT,
            PROJ,
            REFPLN,
            SCALE,
            SRCRAD,
            TANPLN,
            THETA,
            TOL,
            X,
            XPOINT,
            Y,
            Z,
            NLAT,
            NLON,
            NSHAPE,
            SEED,
            UMBRAL,
        }
    }
}

//$Procedure F_ZZEDTMPT ( ZZEDTMPT tests )
pub fn F_ZZEDTMPT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //
    //    T_RANDD ( LOWER, UPPER, SEED )
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
    testutil::TOPEN(b"F_ZZEDTMPT", ctx)?;

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
    testutil::TCASE(b"Light source and target are spheres of the same size: A. Distance between centers is 3*A. Umbral case.", ctx)?;

    save.A = 1000.0;
    save.B = save.A;
    save.C = save.A;
    save.SRCRAD = save.A;

    save.UMBRAL = true;

    spicelib::VPACK(((3 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSCL(
        ((0.5 * f64::sqrt(2.0)) * save.A),
        save.PLNVEC.as_slice(),
        save.XPOINT.as_slice_mut(),
    );

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Light source and target are spheres of the same size: A. Distance between centers is 3*A. Penumbral case.", ctx)?;

    save.A = 1000.0;
    save.B = save.A;
    save.C = save.A;
    save.SRCRAD = save.A;

    save.UMBRAL = false;

    spicelib::VPACK(((3 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // A tangent segment connecting the sphere and target,
    // in the plane defined by PLNVEC, crosses the axis with
    // angle ALPHA, where
    //
    //    sin( ALPHA ) = A / (1.5 * A)
    //
    // The complementary angle is BETA.
    //
    save.ALPHA = f64::asin((1.0 / 1.5));

    save.BETA = (spicelib::HALFPI(ctx) - save.ALPHA);

    spicelib::VSCL(
        (((f64::sin(save.BETA) * save.A) * f64::sqrt(2.0)) / 2 as f64),
        save.PLNVEC.as_slice(),
        save.XPOINT.as_slice_mut(),
    );

    save.XPOINT[1] = (save.A * f64::cos(save.BETA));

    save.TOL = TIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Triaxial target. Source is larger than longest target axis. Umbral case.",
        ctx,
    )?;

    save.A = 1000.0;
    save.B = ((2 as f64) * save.A);
    save.C = (save.A / 2 as f64);
    save.SRCRAD = ((10 as f64) * save.A);

    save.UMBRAL = true;

    spicelib::VPACK(((20 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure POINT is on the target. Find the scaled version
    // of POINT on the target surface and compare.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is in the reference plane. Project POINT
    // onto the plane and compare.
    //
    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.REFPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.REFPLN.as_slice(),
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is on the correct side of the axis with respect
    // to PLNVEC.
    //
    save.DOT = spicelib::VDOT(save.POINT.as_slice(), save.PLNVEC.as_slice());

    testutil::CHCKSD(b"point DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // Create a tangent plane at the terminator point. The distance
    // between this plane and the center of the source should be
    // the radius of the source.
    //
    spicelib::SURFNM(
        save.A,
        save.B,
        save.C,
        save.POINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::NVP2PL(
        save.NORMAL.as_slice(),
        save.POINT.as_slice(),
        save.TANPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.AXIS.as_slice(),
        save.TANPLN.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DIST = spicelib::VDIST(save.AXIS.as_slice(), save.PROJ.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.SRCRAD, save.TOL, OK, ctx)?;

    //
    // Make sure the source center is on the correct side of the
    // tangent plane.
    //
    spicelib::VSUB(
        save.AXIS.as_slice(),
        save.PROJ.as_slice(),
        save.AXIOFF.as_slice_mut(),
    );

    save.DOT = spicelib::VDOT(save.AXIOFF.as_slice(), save.NORMAL.as_slice());

    testutil::CHCKSD(b"axis DOT", save.DOT, b"<", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Triaxial target. Source is smaller than shortest target axis. Umbral case.",
        ctx,
    )?;

    save.A = 1000.0;
    save.B = ((2 as f64) * save.A);
    save.C = (save.A / 2 as f64);
    save.SRCRAD = (save.A / 10 as f64);

    save.UMBRAL = true;

    spicelib::VPACK(((40 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure POINT is on the target. Find the scaled version
    // of POINT on the target surface and compare.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is in the reference plane. Project POINT
    // onto the plane and compare.
    //
    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.REFPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.REFPLN.as_slice(),
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is on the correct side of the axis with respect
    // to PLNVEC.
    //
    save.DOT = spicelib::VDOT(save.POINT.as_slice(), save.PLNVEC.as_slice());

    testutil::CHCKSD(b"DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // Create a tangent plane at the terminator point. The distance
    // between this plane and the center of the source should be
    // the radius of the source.
    //
    spicelib::SURFNM(
        save.A,
        save.B,
        save.C,
        save.POINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::NVP2PL(
        save.NORMAL.as_slice(),
        save.POINT.as_slice(),
        save.TANPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.AXIS.as_slice(),
        save.TANPLN.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DIST = spicelib::VDIST(save.AXIS.as_slice(), save.PROJ.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.SRCRAD, save.TOL, OK, ctx)?;

    //
    // Make sure the source center is on the correct side of the
    // tangent plane.
    //
    spicelib::VSUB(
        save.AXIS.as_slice(),
        save.PROJ.as_slice(),
        save.AXIOFF.as_slice_mut(),
    );

    save.DOT = spicelib::VDOT(save.AXIOFF.as_slice(), save.NORMAL.as_slice());

    testutil::CHCKSD(b"axis DOT", save.DOT, b"<", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Triaxial target. Source is larger than longest target axis. Penumbral case.",
        ctx,
    )?;

    save.A = 1000.0;
    save.B = ((2 as f64) * save.A);
    save.C = (save.A / 2 as f64);
    save.SRCRAD = ((10 as f64) * save.A);

    save.UMBRAL = false;

    spicelib::VPACK(((30 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure POINT is on the target. Find the scaled version
    // of POINT on the target surface and compare.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is in the reference plane. Project POINT
    // onto the plane and compare.
    //
    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.REFPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.REFPLN.as_slice(),
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is on the correct side of the axis with respect
    // to PLNVEC.
    //
    save.DOT = spicelib::VDOT(save.POINT.as_slice(), save.PLNVEC.as_slice());

    testutil::CHCKSD(b"DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // Create a tangent plane at the terminator point. The distance
    // between this plane and the center of the source should be
    // the radius of the source.
    //
    spicelib::SURFNM(
        save.A,
        save.B,
        save.C,
        save.POINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::NVP2PL(
        save.NORMAL.as_slice(),
        save.POINT.as_slice(),
        save.TANPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.AXIS.as_slice(),
        save.TANPLN.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DIST = spicelib::VDIST(save.AXIS.as_slice(), save.PROJ.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.SRCRAD, save.TOL, OK, ctx)?;

    //
    // Make sure the source center is on the correct side of the
    // tangent plane.
    //
    spicelib::VSUB(
        save.AXIS.as_slice(),
        save.PROJ.as_slice(),
        save.AXIOFF.as_slice_mut(),
    );

    save.DOT = spicelib::VDOT(save.AXIOFF.as_slice(), save.NORMAL.as_slice());

    testutil::CHCKSD(b"axis DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Triaxial target. Source is smaller than shortest target axis. Penumbral case.",
        ctx,
    )?;

    save.A = 1000.0;
    save.B = ((2 as f64) * save.A);
    save.C = (save.A / 2 as f64);
    save.SRCRAD = (save.A / 10 as f64);

    save.UMBRAL = false;

    spicelib::VPACK(((30 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure POINT is on the target. Find the scaled version
    // of POINT on the target surface and compare.
    //
    spicelib::EDPNT(
        save.POINT.as_slice(),
        save.A,
        save.B,
        save.C,
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is in the reference plane. Project POINT
    // onto the plane and compare.
    //
    spicelib::PSV2PL(
        save.ORIGIN.as_slice(),
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.REFPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.POINT.as_slice(),
        save.REFPLN.as_slice(),
        save.XPOINT.as_slice_mut(),
        ctx,
    )?;

    save.TOL = VTIGHT;

    testutil::CHCKAD(
        b"POINT",
        save.POINT.as_slice(),
        b"~~/",
        save.XPOINT.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Make sure POINT is on the correct side of the axis with respect
    // to PLNVEC.
    //
    save.DOT = spicelib::VDOT(save.POINT.as_slice(), save.PLNVEC.as_slice());

    testutil::CHCKSD(b"DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;

    //
    // Create a tangent plane at the terminator point. The distance
    // between this plane and the center of the source should be
    // the radius of the source.
    //
    spicelib::SURFNM(
        save.A,
        save.B,
        save.C,
        save.POINT.as_slice(),
        save.NORMAL.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::NVP2PL(
        save.NORMAL.as_slice(),
        save.POINT.as_slice(),
        save.TANPLN.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPRJP(
        save.AXIS.as_slice(),
        save.TANPLN.as_slice(),
        save.PROJ.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.DIST = spicelib::VDIST(save.AXIS.as_slice(), save.PROJ.as_slice());

    save.TOL = TIGHT;

    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.SRCRAD, save.TOL, OK, ctx)?;

    //
    // Make sure the source center is on the correct side of the
    // tangent plane.
    //
    spicelib::VSUB(
        save.AXIS.as_slice(),
        save.PROJ.as_slice(),
        save.AXIOFF.as_slice_mut(),
    );

    save.DOT = spicelib::VDOT(save.AXIOFF.as_slice(), save.NORMAL.as_slice());

    testutil::CHCKSD(b"axis DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;

    //
    //
    // The following cases use a variety of target shapes, scales, and
    // light source positions
    //
    //
    save.NLAT = 11;
    save.NLON = 10;

    save.DLAT = (spicelib::PI(ctx) / (save.NLAT - 1) as f64);
    save.DLON = (((2 as f64) * spicelib::PI(ctx)) / save.NLON as f64);

    save.NSHAPE = 10;

    save.SEED = -100;

    for SHADIX in 1..=2 {
        //
        // Set the shadow type flag.
        //
        save.UMBRAL = (SHADIX == 1);

        for I in 1..=save.NLON {
            save.LON = (((I - 1) as f64) * save.DLON);

            for J in 1..=save.NLAT {
                save.LAT = (spicelib::HALFPI(ctx) - (((J - 1) as f64) * save.DLAT));

                for K in 1..=save.NSHAPE {
                    //
                    // Pick scale factor.
                    //
                    save.SCALE =
                        f64::powf(10.0, testutil::T_RANDD(-100.0, 100.0, &mut save.SEED, ctx)?);

                    save.A = (save.SCALE * testutil::T_RANDD(1.0, 10.0, &mut save.SEED, ctx)?);
                    save.B = (save.SCALE * testutil::T_RANDD(1.0, 10.0, &mut save.SEED, ctx)?);
                    save.C = (save.SCALE * testutil::T_RANDD(1.0, 10.0, &mut save.SEED, ctx)?);
                    save.SRCRAD = (save.SCALE * testutil::T_RANDD(1.0, 10.0, &mut save.SEED, ctx)?);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Pick a rotation angle for the cutting half plane.
                    //
                    save.THETA = testutil::T_RANDD(
                        0.0,
                        ((2 as f64) * spicelib::PI(ctx)),
                        &mut save.SEED,
                        ctx,
                    )?;

                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // --- Case: ------------------------------------------------------
                    //
                    fstr::assign(&mut save.TITLE, b"Random case. SCALE = #; A = #; B = #; C = #; SRCRAD = #, THETA (deg) = #; UMBRAL = #.");

                    spicelib::REPMD(
                        &save.TITLE.to_vec(),
                        b"#",
                        save.SCALE,
                        14,
                        &mut save.TITLE,
                        ctx,
                    );
                    spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.A, 14, &mut save.TITLE, ctx);
                    spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.B, 14, &mut save.TITLE, ctx);
                    spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.C, 14, &mut save.TITLE, ctx);
                    spicelib::REPMD(
                        &save.TITLE.to_vec(),
                        b"#",
                        save.SRCRAD,
                        14,
                        &mut save.TITLE,
                        ctx,
                    );
                    spicelib::REPMD(
                        &save.TITLE.to_vec(),
                        b"#",
                        (save.THETA * spicelib::DPR(ctx)),
                        14,
                        &mut save.TITLE,
                        ctx,
                    );
                    spicelib::REPMC(
                        &save.TITLE.to_vec(),
                        b"#",
                        &save.LCHAR[SHADIX],
                        &mut save.TITLE,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::TCASE(&save.TITLE, ctx)?;

                    spicelib::LATREC(
                        (1000.0 * save.SCALE),
                        save.LON,
                        save.LAT,
                        save.AXIS.as_slice_mut(),
                    );

                    spicelib::VEQU(save.AXIS.as_slice(), save.X.as_slice_mut());
                    spicelib::FRAME(
                        save.X.as_slice_mut(),
                        save.Y.as_slice_mut(),
                        save.Z.as_slice_mut(),
                    );

                    spicelib::VROTV(
                        save.Z.as_slice(),
                        save.AXIS.as_slice(),
                        save.THETA,
                        save.PLNVEC.as_slice_mut(),
                    );

                    spicelib::ZZEDTMPT(
                        save.UMBRAL,
                        save.A,
                        save.B,
                        save.C,
                        save.SRCRAD,
                        save.AXIS.as_slice(),
                        save.PLNVEC.as_slice(),
                        save.POINT.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Make sure POINT is on the target. Find the scaled
                    // version of POINT on the target surface and compare.
                    //
                    spicelib::EDPNT(
                        save.POINT.as_slice(),
                        save.A,
                        save.B,
                        save.C,
                        save.XPOINT.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.TOL = VTIGHT;

                    testutil::CHCKAD(
                        b"POINT",
                        save.POINT.as_slice(),
                        b"~~/",
                        save.XPOINT.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;

                    //
                    // Make sure POINT is in the reference plane. Project
                    // POINT onto the plane and compare.
                    //
                    spicelib::PSV2PL(
                        save.ORIGIN.as_slice(),
                        save.AXIS.as_slice(),
                        save.PLNVEC.as_slice(),
                        save.REFPLN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::VPRJP(
                        save.POINT.as_slice(),
                        save.REFPLN.as_slice(),
                        save.XPOINT.as_slice_mut(),
                        ctx,
                    )?;

                    save.TOL = VTIGHT;

                    testutil::CHCKAD(
                        b"POINT",
                        save.POINT.as_slice(),
                        b"~~/",
                        save.XPOINT.as_slice(),
                        3,
                        save.TOL,
                        OK,
                        ctx,
                    )?;
                    //
                    // Make sure POINT is on the correct side of the axis
                    // with respect to PLNVEC.
                    //
                    save.DOT = spicelib::VDOT(save.POINT.as_slice(), save.PLNVEC.as_slice());

                    testutil::CHCKSD(b"DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;

                    //
                    // Create a tangent plane at the terminator point. The
                    // distance between this plane and the center of the
                    // source should be the radius of the source.
                    //
                    spicelib::SURFNM(
                        save.A,
                        save.B,
                        save.C,
                        save.POINT.as_slice(),
                        save.NORMAL.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::NVP2PL(
                        save.NORMAL.as_slice(),
                        save.POINT.as_slice(),
                        save.TANPLN.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::VPRJP(
                        save.AXIS.as_slice(),
                        save.TANPLN.as_slice(),
                        save.PROJ.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.DIST = spicelib::VDIST(save.AXIS.as_slice(), save.PROJ.as_slice());

                    save.TOL = TIGHT;

                    testutil::CHCKSD(b"DIST", save.DIST, b"~/", save.SRCRAD, save.TOL, OK, ctx)?;

                    //
                    // Make sure the source center is on the correct side of
                    // the tangent plane.
                    //
                    spicelib::VSUB(
                        save.AXIS.as_slice(),
                        save.PROJ.as_slice(),
                        save.AXIOFF.as_slice_mut(),
                    );

                    save.DOT = spicelib::VDOT(save.AXIOFF.as_slice(), save.NORMAL.as_slice());

                    if save.UMBRAL {
                        testutil::CHCKSD(b"axis DOT", save.DOT, b"<", 0.0, save.TOL, OK, ctx)?;
                    } else {
                        testutil::CHCKSD(b"axis DOT", save.DOT, b">", 0.0, save.TOL, OK, ctx)?;
                    }
                }
            }
        }
    }

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad target radii.", ctx)?;

    save.A = 1000.0;
    save.B = save.A;
    save.C = save.A;
    save.SRCRAD = save.A;

    save.UMBRAL = false;

    spicelib::VPACK(((3 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        0.0,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        -1.0,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        0.0,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        -1.0,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        0.0,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        -1.0,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad source radius", ctx)?;

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        0.0,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADIUS)", OK, ctx)?;

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        -1.0,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADIUS)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad axis vector", ctx)?;

    spicelib::CLEARD(3, save.AXIS.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    spicelib::VPACK(((3 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Source and target are too close.", ctx)?;

    spicelib::VPACK(save.A, 0.0, 0.0, save.AXIS.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(OBJECTSTOOCLOSE)", OK, ctx)?;

    spicelib::VPACK(((3 as f64) * save.A), 0.0, 0.0, save.AXIS.as_slice_mut());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad plane reference vector", ctx)?;

    spicelib::CLEARD(3, save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    spicelib::VPACK(0.0, 1.0, 1.0, save.PLNVEC.as_slice_mut());

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Axis and plane reference vector are linearly dependent.",
        ctx,
    )?;

    spicelib::VEQU(save.AXIS.as_slice(), save.PLNVEC.as_slice_mut());

    spicelib::ZZEDTMPT(
        save.UMBRAL,
        save.A,
        save.B,
        save.C,
        save.SRCRAD,
        save.AXIS.as_slice(),
        save.PLNVEC.as_slice(),
        save.POINT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
