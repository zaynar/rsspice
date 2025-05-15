//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVLIM: f64 = 0.000000000000001;
const MAXITR: i32 = 1000;

//$Procedure  ZZCXBRUT ( Cone-segment intersection by brute force )
pub fn ZZCXBRUT(
    APEX: &[f64],
    AXIS: &[f64],
    ANGLE: f64,
    ENDPT1: &[f64],
    ENDPT2: &[f64],
    XPT: &mut [f64],
    ISBRCK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let APEX = DummyArray::new(APEX, 1..=3);
    let AXIS = DummyArray::new(AXIS, 1..=3);
    let ENDPT1 = DummyArray::new(ENDPT1, 1..=3);
    let ENDPT2 = DummyArray::new(ENDPT2, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut STATE1: bool = false;
    let mut STATE2: bool = false;
    let mut COSANG: f64 = 0.0;
    let mut DELTA: f64 = 0.0;
    let mut DP: f64 = 0.0;
    let mut DP1: f64 = 0.0;
    let mut DP2: f64 = 0.0;
    let mut HIGH: f64 = 0.0;
    let mut LOCANG: f64 = 0.0;
    let mut LOCAXI = StackArray::<f64, 3>::new(1..=3);
    let mut LOW: f64 = 0.0;
    let mut MIDPT: f64 = 0.0;
    let mut OFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut OFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut PRVDLT: f64 = 0.0;
    let mut SEG = StackArray::<f64, 3>::new(1..=3);
    let mut UOFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut UOFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut UX = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut NITR: i32 = 0;
    let mut STATE: bool = false;

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
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Check the axis.
    //
    if VZERO(AXIS.as_slice()) {
        CHKIN(b"ZZCXBRUT", ctx)?;
        SETMSG(b"Cone axis is the zero vector", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZCXBRUT", ctx)?;
        return Ok(());
    }

    //
    // Make a local version of the cone's axis and angle. The
    // angle will be less than or equal to pi/2 radians.
    //
    if (ANGLE > HALFPI(ctx)) {
        LOCANG = (PI(ctx) - ANGLE);
        VMINUS(AXIS.as_slice(), LOCAXI.as_slice_mut());
    } else {
        LOCANG = ANGLE;
        VEQU(AXIS.as_slice(), LOCAXI.as_slice_mut());
    }

    VHATIP(LOCAXI.as_slice_mut());

    COSANG = f64::cos(LOCANG);

    //
    // Calculate the offsets of the endpoints from the apex,
    // and get unit-length versions of these.
    //
    VSUB(ENDPT1.as_slice(), APEX.as_slice(), OFF1.as_slice_mut());
    VSUB(ENDPT2.as_slice(), APEX.as_slice(), OFF2.as_slice_mut());

    VHAT(OFF1.as_slice(), UOFF1.as_slice_mut());
    VHAT(OFF2.as_slice(), UOFF2.as_slice_mut());

    //
    // Get the dot products of the unit offsets with the axis.
    // These will serve as proxies for latitude.
    //
    DP1 = VDOT(UOFF1.as_slice(), LOCAXI.as_slice());
    DP2 = VDOT(UOFF2.as_slice(), LOCAXI.as_slice());

    //
    // The "state" variables at the endpoints are .TRUE. if
    // the endpoints are on or inside the cone.
    //
    STATE1 = (DP1 >= COSANG);
    STATE2 = (DP2 >= COSANG);

    //
    // The intersection is supposed to be bracketed. Return
    // if not, indicating the situation via ISBRCK.
    //
    *ISBRCK = (STATE1 != STATE2);

    if !*ISBRCK {
        return Ok(());
    }

    //
    // Prepare for a solution by bisection.
    //
    VSUB(OFF2.as_slice(), OFF1.as_slice(), SEG.as_slice_mut());

    LOW = 0.0;
    HIGH = 1.0;
    DELTA = f64::abs((HIGH - LOW));
    PRVDLT = 2.0;
    NITR = 0;

    while (((DELTA > CNVLIM) && (DELTA < PRVDLT)) && (NITR < MAXITR)) {
        MIDPT = ((LOW + HIGH) / 2 as f64);

        VLCOM(
            1.0,
            OFF1.as_slice(),
            MIDPT,
            SEG.as_slice(),
            X.as_slice_mut(),
        );

        VHAT(X.as_slice(), UX.as_slice_mut());

        DP = VDOT(UX.as_slice(), LOCAXI.as_slice());

        STATE = (DP >= COSANG);

        if (STATE == STATE1) {
            //
            // There has been no state change between OFF1
            // and XPT.
            //
            LOW = MIDPT;
        } else {
            HIGH = MIDPT;
        }

        PRVDLT = DELTA;
        DELTA = f64::abs((HIGH - LOW));

        NITR = (NITR + 1);
    }
    //
    // X is an offset from APEX. The solution is an offset from the
    // origin.
    //
    VADD(APEX.as_slice(), X.as_slice(), XPT.as_slice_mut());

    Ok(())
}
