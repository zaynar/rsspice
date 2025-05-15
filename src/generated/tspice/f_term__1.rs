//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.000000000000001;
const TOL: f64 = VTIGHT;
const UBPL: i32 = 4;
const LNSIZE: i32 = 80;

//
//
// Utility functions:
//
//
//
// Check a terminator point on an ellipsoid.
//
pub fn T_ETERM(
    TITLE: &[u8],
    UMBRAL: bool,
    SRCPOS: &[f64],
    SRCRAD: f64,
    A: f64,
    B: f64,
    C: f64,
    X: &[f64],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SRCPOS = DummyArray::new(SRCPOS, 1..=3);
    let X = DummyArray::new(X, 1..=3);
    let mut LABEL = [b' '; LNSIZE as usize];
    let mut D: f64 = 0.0;
    let mut H: f64 = 0.0;
    let mut N = StackArray::<f64, 3>::new(1..=3);
    let mut NEARP = StackArray::<f64, 3>::new(1..=3);
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut SCALE: f64 = 0.0;

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
    // Get the ellipsoid's outward normal vector at X.
    //
    spicelib::SURFNM(A, B, C, X.as_slice(), N.as_slice_mut(), ctx)?;

    //
    // Form the tangent plane at X.
    //
    spicelib::NVP2PL(N.as_slice(), X.as_slice(), PLANE.as_slice_mut(), ctx)?;

    //
    // The center of the source should be below the tangent
    // plane in the umbral case and above the plane in the
    // penumbral case.
    //
    fstr::assign(&mut LABEL, TITLE);
    spicelib::SUFFIX(b"<N,SRCPOS>", 1, &mut LABEL);

    spicelib::VSUB(SRCPOS.as_slice(), X.as_slice(), OFFSET.as_slice_mut());

    if UMBRAL {
        testutil::CHCKSD(
            &LABEL,
            spicelib::VDOT(N.as_slice(), OFFSET.as_slice()),
            b"<",
            0.0,
            TOL,
            OK,
            ctx,
        )?;
    } else {
        testutil::CHCKSD(
            &LABEL,
            spicelib::VDOT(N.as_slice(), OFFSET.as_slice()),
            b">",
            0.0,
            TOL,
            OK,
            ctx,
        )?;
    }

    if !*OK {
        return Ok(());
    }

    //
    // Find the nearest point on the plane to the center
    // of the light source.
    //
    spicelib::VPRJP(
        SRCPOS.as_slice(),
        PLANE.as_slice(),
        NEARP.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the plane's height relative to the source's
    // surface.
    //
    H = (spicelib::VDIST(NEARP.as_slice(), SRCPOS.as_slice()) - SRCRAD);

    //
    // Determine the scale that should be used for the
    // coming relative error computation.
    //
    D = spicelib::VNORM(SRCPOS.as_slice());

    SCALE = intrinsics::DMAX1(&[D, SRCRAD, A, B, C]);

    fstr::assign(&mut LABEL, TITLE);
    spicelib::SUFFIX(b"ABS(H)/SCALE", 1, &mut LABEL);

    testutil::CHCKSD(TITLE, (f64::abs(H) / SCALE), b"~", 0.0, TOL, OK, ctx)?;

    Ok(())
}
