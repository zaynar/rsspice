//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZDSPC ( SGP4 deep space routine )
pub fn ZZDSPC(
    IREZ: i32,
    D2201: f64,
    D2211: f64,
    D3210: f64,
    D3222: f64,
    D4410: f64,
    D4422: f64,
    D5220: f64,
    D5232: f64,
    D5421: f64,
    D5433: f64,
    DEDT: f64,
    DEL1: f64,
    DEL2: f64,
    DEL3: f64,
    DIDT: f64,
    DMDT: f64,
    DNODT: f64,
    DOMDT: f64,
    ARGPO: f64,
    ARGPDOT: f64,
    T: f64,
    TC: f64,
    GSTO: f64,
    XFACT: f64,
    XLAMO: f64,
    NO: f64,
    ATIME: &mut f64,
    ECCM: &mut f64,
    ARGPM: &mut f64,
    INCLM: &mut f64,
    XLI: &mut f64,
    MM: &mut f64,
    XNI: &mut f64,
    NODEM: &mut f64,
    DNDT: &mut f64,
    XN: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DELT: f64 = 0.0;
    let mut FASX2: f64 = 0.0;
    let mut FASX4: f64 = 0.0;
    let mut FASX6: f64 = 0.0;
    let mut FT: f64 = 0.0;
    let mut G22: f64 = 0.0;
    let mut G32: f64 = 0.0;
    let mut G44: f64 = 0.0;
    let mut G52: f64 = 0.0;
    let mut G54: f64 = 0.0;
    let mut RPTIM: f64 = 0.0;
    let mut STEP2: f64 = 0.0;
    let mut STEPN: f64 = 0.0;
    let mut STEPP: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut X2LI: f64 = 0.0;
    let mut X2OMI: f64 = 0.0;
    let mut XL: f64 = 0.0;
    let mut XLDOT: f64 = 0.0;
    let mut XNDDT: f64 = 0.0;
    let mut XNDT: f64 = 0.0;
    let mut XOMI: f64 = 0.0;
    let mut IRET: i32 = 0;
    let mut IRETN: i32 = 0;

    //
    // Local Variables
    //

    //
    // SPICELIB routines.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSPC", ctx)?;

    //
    // Constants
    //
    FASX2 = 0.13130908;
    FASX4 = 2.8843198;
    FASX6 = 0.37448087;
    G22 = 5.7686396;
    G32 = 0.95240898;
    G44 = 1.8014998;
    G52 = 1.050833;
    G54 = 4.4108898;
    RPTIM = 0.0043752690880113;
    STEPP = 720.0;
    STEPN = -720.0;
    STEP2 = 259200.0;

    //
    // Calculate deep space resonance effects.
    //
    *DNDT = 0.0;
    THETA = intrinsics::DMOD((GSTO + (TC * RPTIM)), TWOPI(ctx));
    *ECCM = (*ECCM + (DEDT * T));

    *INCLM = (*INCLM + (DIDT * T));
    *ARGPM = (*ARGPM + (DOMDT * T));
    *NODEM = (*NODEM + (DNODT * T));
    *MM = (*MM + (DMDT * T));

    //
    // sgp4fix for negative inclinations
    // the following if statement should be commented out
    //
    //      IF( INCLM .LT. 0.0D0) THEN
    //          INCLM  = -INCLM
    //          ARGPM  = ARGPM-PI
    //          NODEM  = NODEM+PI
    //      END IF
    //

    //
    // sgp4fix for propagator problems
    //
    // The following integration works for negative time steps and
    // periods. The specific changes are unknown because the original
    // code was so convoluted
    //
    // sgp4fix Take out atime = 0.0 and fix for faster operation
    //

    //
    // Just in case - should be set in loops if used.
    //
    FT = 0.0;

    if (IREZ != 0) {
        //
        // UPDATE RESONANCES : NUMERICAL (EULER-MACLAURIN) INTEGRATION
        //
        // EPOCH RESTART
        //

        //
        // sgp4fix streamline check
        //
        if (((*ATIME == 0.0) || ((T * *ATIME) <= 0.0)) || (f64::abs(T) < f64::abs(*ATIME))) {
            *ATIME = 0.0;
            *XNI = NO;
            *XLI = XLAMO;
        }

        //
        // sgp4fix move check outside loop
        //
        if (T > 0.0) {
            DELT = STEPP;
        } else {
            DELT = STEPN;
        }

        //
        // ADDED FOR DO LOOP
        //
        IRETN = 381;

        //
        // ADDED FOR LOOP
        //
        IRET = 0;

        while (IRETN == 381) {
            //
            // DOT TERMS CALCULATED
            //
            // NEAR - SYNCHRONOUS RESONANCE TERMS
            //
            if (IREZ != 2) {
                XNDT = (((DEL1 * f64::sin((*XLI - FASX2)))
                    + (DEL2 * f64::sin((2.0 * (*XLI - FASX4)))))
                    + (DEL3 * f64::sin((3.0 * (*XLI - FASX6)))));
                XLDOT = (*XNI + XFACT);
                XNDDT = (((DEL1 * f64::cos((*XLI - FASX2)))
                    + ((2.0 * DEL2) * f64::cos((2.0 * (*XLI - FASX4)))))
                    + ((3.0 * DEL3) * f64::cos((3.0 * (*XLI - FASX6)))));
                XNDDT = (XNDDT * XLDOT);
            } else {
                //
                // NEAR - HALF-DAY RESONANCE TERMS
                //

                XOMI = (ARGPO + (ARGPDOT * *ATIME));
                X2OMI = (XOMI + XOMI);
                X2LI = (*XLI + *XLI);
                XNDT = ((((((((((D2201 * f64::sin(((X2OMI + *XLI) - G22)))
                    + (D2211 * f64::sin((*XLI - G22))))
                    + (D3210 * f64::sin(((XOMI + *XLI) - G32))))
                    + (D3222 * f64::sin(((-XOMI + *XLI) - G32))))
                    + (D4410 * f64::sin(((X2OMI + X2LI) - G44))))
                    + (D4422 * f64::sin((X2LI - G44))))
                    + (D5220 * f64::sin(((XOMI + *XLI) - G52))))
                    + (D5232 * f64::sin(((-XOMI + *XLI) - G52))))
                    + (D5421 * f64::sin(((XOMI + X2LI) - G54))))
                    + (D5433 * f64::sin(((-XOMI + X2LI) - G54))));
                XLDOT = (*XNI + XFACT);
                XNDDT = (((((((D2201 * f64::cos(((X2OMI + *XLI) - G22)))
                    + (D2211 * f64::cos((*XLI - G22))))
                    + (D3210 * f64::cos(((XOMI + *XLI) - G32))))
                    + (D3222 * f64::cos(((-XOMI + *XLI) - G32))))
                    + (D5220 * f64::cos(((XOMI + *XLI) - G52))))
                    + (D5232 * f64::cos(((-XOMI + *XLI) - G52))))
                    + (2.0
                        * ((((D4410 * f64::cos(((X2OMI + X2LI) - G44)))
                            + (D4422 * f64::cos((X2LI - G44))))
                            + (D5421 * f64::cos(((XOMI + X2LI) - G54))))
                            + (D5433 * f64::cos(((-XOMI + X2LI) - G54))))));
                XNDDT = (XNDDT * XLDOT);
            }

            //
            // INTEGRATOR
            //
            // sgp4fix move end checks to end of routine
            //

            if (f64::abs((T - *ATIME)) >= STEPP) {
                IRET = 0;
                IRETN = 381;
            } else {
                FT = (T - *ATIME);
                IRETN = 0;
            }

            if (IRETN == 381) {
                *XLI = ((*XLI + (XLDOT * DELT)) + (XNDT * STEP2));
                *XNI = ((*XNI + (XNDT * DELT)) + (XNDDT * STEP2));
                *ATIME = (*ATIME + DELT);
            }
        }

        *XN = ((*XNI + (XNDT * FT)) + (((XNDDT * FT) * FT) * 0.5));
        XL = ((*XLI + (XLDOT * FT)) + (((XNDT * FT) * FT) * 0.5));

        if (IREZ != 1) {
            *MM = ((XL - (2.0 * *NODEM)) + (2.0 * THETA));
            *DNDT = (*XN - NO);
        } else {
            *MM = (((XL - *NODEM) - *ARGPM) + THETA);
            *DNDT = (*XN - NO);
        }

        *XN = (NO + *DNDT);
    }

    CHKOUT(b"ZZDSPC", ctx)?;

    Ok(())
}
