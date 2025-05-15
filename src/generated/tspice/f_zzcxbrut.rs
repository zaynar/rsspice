//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;

//$Procedure      F_ZZCXBRUT ( Test brute force cone-segment intercept )
pub fn F_ZZCXBRUT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ANGLE: f64 = 0.0;
    let mut APEX = StackArray::<f64, 3>::new(1..=3);
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut ENDPT1 = StackArray::<f64, 3>::new(1..=3);
    let mut ENDPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut TOL: f64 = 0.0;
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut XXPT = StackArray::<f64, 3>::new(1..=3);
    let mut ISBRCK: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    // DOUBLE PRECISION      TIGHT
    // PARAMETER           ( TIGHT  = 1.D-12 )

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZCXBRUT", ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"AXIS is +Z; line segment is parallel to X-Y plane. First endpoint is outside cone.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, 1.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(1.0, 0.0, 1.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"AXIS is +Z; line segment is parallel to X-Y plane. First endpoint is inside cone.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, 1.0, ENDPT2.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 1.0, ENDPT1.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(1.0, 0.0, 1.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"AXIS is +Z; line segment is parallel to Z axis. First endpoint is outside cone.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, 0.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 3.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(2.0, 0.0, 2.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"AXIS is +Z; line segment is parallel to Z axis. First endpoint is inside cone.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, 0.0, ENDPT2.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 3.0, ENDPT1.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(2.0, 0.0, 2.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"AXIS is +Z; line segment is parallel to Z axis; first endpoint is far below X-Y plane.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, -10000000000.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(2.0, 0.0, 3.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(2.0, 0.0, 2.0, XXPT.as_slice_mut());

    TOL = 0.00001;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"AXIS is +Z; line segment is on the Z axis;first endpoint is below X-Y plane.",
        ctx,
    )?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(0.0, 0.0, -1.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 3.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(0.0, 0.0, 0.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    //
    // Use absolute test since the expected result is the
    // zero vector.
    //
    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"AXIS is +Z; line segment is parallel to X-Y plane. First endpoint is outside cone. Angle > pi/2.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (((3 as f64) * spicelib::PI(ctx)) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, -1.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, -1.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(1.0, 0.0, -1.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"AXIS is +Z; line segment is parallel to X-Y plane. First endpoint is outside cone. Angle > pi/2. APEX is non-zero.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(3.0, 4.0, 5.0, APEX.as_slice_mut());

    ANGLE = (((3 as f64) * spicelib::PI(ctx)) / 4 as f64);

    spicelib::VPACK(5.0, 4.0, 4.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(3.0, 4.0, 4.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(4.0, 4.0, 4.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"AXIS is -Z; line segment is parallel to X-Y plane. First endpoint is outside cone. Angle > pi/2. APEX is non-zero.", ctx)?;

    spicelib::VPACK(0.0, 0.0, -1.0, AXIS.as_slice_mut());
    spicelib::VPACK(3.0, 4.0, 5.0, APEX.as_slice_mut());

    ANGLE = (((3 as f64) * spicelib::PI(ctx)) / 4 as f64);

    spicelib::VPACK(5.0, 4.0, 6.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(3.0, 4.0, 6.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VPACK(4.0, 4.0, 6.0, XXPT.as_slice_mut());

    TOL = VTIGHT;

    testutil::CHCKAD(
        b"XPT",
        XPT.as_slice(),
        b"~~/",
        XXPT.as_slice(),
        3,
        TOL,
        OK,
        ctx,
    )?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, true, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Root is not bracketed.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 1.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, 1.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(3.0, 0.0, 1.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISBRCK", ISBRCK, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Error: axis is zero vector.", ctx)?;

    spicelib::VPACK(0.0, 0.0, 0.0, AXIS.as_slice_mut());
    spicelib::VPACK(0.0, 0.0, 0.0, APEX.as_slice_mut());

    ANGLE = (spicelib::PI(ctx) / 4 as f64);

    spicelib::VPACK(2.0, 0.0, 1.0, ENDPT1.as_slice_mut());
    spicelib::VPACK(3.0, 0.0, 1.0, ENDPT2.as_slice_mut());

    spicelib::ZZCXBRUT(
        APEX.as_slice(),
        AXIS.as_slice(),
        ANGLE,
        ENDPT1.as_slice(),
        ENDPT2.as_slice(),
        XPT.as_slice_mut(),
        &mut ISBRCK,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
