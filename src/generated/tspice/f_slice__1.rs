//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBEL: i32 = 9;
const UBPL: i32 = 4;
const IPTOL: f64 = 0.00000000000005;
const LVLTOL: f64 = 0.00000000000005;
const NPT: i32 = 4;

//
// Supporting function T_ISLIMB
//
pub fn T_ISLIMB(
    LIMB: &[f64],
    A: f64,
    B: f64,
    C: f64,
    VIEWPT: &[f64],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let LIMB = DummyArray::new(LIMB, 1..=UBEL);
    let VIEWPT = DummyArray::new(VIEWPT, 1..=3);
    let mut T_ISLIMB: bool = false;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut DIFF = StackArray::<f64, 3>::new(1..=3);
    let mut ELPT = StackArray2D::<f64, 12>::new(1..=3, 1..=NPT);
    let mut LEVEL: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SCLA: f64 = 0.0;
    let mut SCLB: f64 = 0.0;
    let mut SCLC: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut VP = StackArray::<f64, 3>::new(1..=3);

    //
    // Test whether an ellipse is the limb of an ellipsoid as seen
    // from a specified viewing location.
    //

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
    // Scale ellipse and viewing point.
    //
    SCALE = intrinsics::DMAX1(&[
        f64::abs(A),
        f64::abs(B),
        f64::abs(C),
        spicelib::VNORM(VIEWPT.as_slice()),
    ]);

    SCLA = (A / SCALE);
    SCLB = (B / SCALE);
    SCLC = (C / SCALE);

    spicelib::VSCL((1.0 / SCALE), VIEWPT.as_slice(), VP.as_slice_mut());

    spicelib::EL2CGV(
        LIMB.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VSCLIP((1.0 / SCALE), CENTER.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMAJOR.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMINOR.as_slice_mut());

    //
    // Construct the endpoints of the major and minor axes.
    //
    spicelib::VADD(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        ELPT.subarray_mut([1, 1]),
    );
    spicelib::VSUB(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        ELPT.subarray_mut([1, 2]),
    );
    spicelib::VADD(
        CENTER.as_slice(),
        SMINOR.as_slice(),
        ELPT.subarray_mut([1, 3]),
    );
    spicelib::VSUB(
        CENTER.as_slice(),
        SMINOR.as_slice(),
        ELPT.subarray_mut([1, 4]),
    );

    //
    // Check the `level' of each point.
    //
    for I in 1..=NPT {
        LEVEL = (((f64::powi(ELPT[[1, I]], 2) / f64::powi(SCLA, 2))
            + (f64::powi(ELPT[[2, I]], 2) / f64::powi(SCLB, 2)))
            + (f64::powi(ELPT[[3, I]], 2) / f64::powi(SCLC, 2)));

        testutil::CHCKSD(b"LEVEL", LEVEL, b"~", 1.0, LVLTOL, OK, ctx)?;

        if !*OK {
            //
            // The putative limb point is not on the ellipsoid.
            //
            T_ISLIMB = false;
        }
    }

    //
    // For each endpoint, find the surface normal and its
    // angular separation from the difference vector between it and
    // the viewing point.
    //
    for I in 1..=NPT {
        spicelib::VSUB(VP.as_slice(), ELPT.subarray([1, I]), DIFF.as_slice_mut());

        spicelib::SURFNM(A, B, C, ELPT.subarray([1, I]), NORMAL.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        SEP = spicelib::VSEP(NORMAL.as_slice(), DIFF.as_slice(), ctx);

        testutil::CHCKSD(b"SEP", SEP, b"~", spicelib::HALFPI(ctx), IPTOL, OK, ctx)?;

        if !*OK {
            return Ok(T_ISLIMB);
        }
    }

    T_ISLIMB = true;

    Ok(T_ISLIMB)
}
