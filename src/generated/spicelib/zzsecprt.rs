//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const G22: f64 = 5.7686396;
const G32: f64 = 0.95240898;
const G44: f64 = 1.8014998;
const G52: f64 = 1.050833;
const G54: f64 = 4.4108898;
const FASX2: f64 = 0.13130908;
const FASX4: f64 = 2.8843198;
const FASX6: f64 = 0.37448087;

//$Procedure ZZSECPRT ( Calculate dot terms for DPSPCE perturbation )
pub fn ZZSECPRT(
    ISYNFL: i32,
    DG: &[f64],
    DEL: &[f64],
    XNI: f64,
    OMEGAO: f64,
    ATIME: f64,
    OMGDOT: f64,
    XLI: f64,
    XFACT: f64,
    XLDOT: &mut f64,
    XNDOT: &mut f64,
    XNDDT: &mut f64,
) {
    let DG = DummyArray::new(DG, 1..=10);
    let DEL = DummyArray::new(DEL, 1..=3);
    let mut X2LI: f64 = 0.0;
    let mut X2OMI: f64 = 0.0;
    let mut XOMI: f64 = 0.0;

    //
    // Local variables.
    //

    //
    // Calculate the dot terms with respect to the state of the
    // resonance flag.
    //

    if (ISYNFL == 0) {
        //
        // Resonance flag set.
        //

        XOMI = (OMEGAO + (OMGDOT * ATIME));
        X2OMI = (XOMI + XOMI);
        X2LI = (XLI + XLI);

        *XNDOT = ((((((((((DG[1] * f64::sin(((X2OMI + XLI) - G22)))
            + (DG[2] * f64::sin((XLI - G22))))
            + (DG[3] * f64::sin(((XOMI + XLI) - G32))))
            + (DG[4] * f64::sin(((-XOMI + XLI) - G32))))
            + (DG[5] * f64::sin(((X2OMI + X2LI) - G44))))
            + (DG[6] * f64::sin((X2LI - G44))))
            + (DG[7] * f64::sin(((XOMI + XLI) - G52))))
            + (DG[8] * f64::sin(((-XOMI + XLI) - G52))))
            + (DG[9] * f64::sin(((XOMI + X2LI) - G54))))
            + (DG[10] * f64::sin(((-XOMI + X2LI) - G54))));

        *XNDDT = (((((((DG[1] * f64::cos(((X2OMI + XLI) - G22)))
            + (DG[2] * f64::cos((XLI - G22))))
            + (DG[3] * f64::cos(((XOMI + XLI) - G32))))
            + (DG[4] * f64::cos(((-XOMI + XLI) - G32))))
            + (DG[7] * f64::cos(((XOMI + XLI) - G52))))
            + (DG[8] * f64::cos(((-XOMI + XLI) - G52))))
            + (2.0
                * ((((DG[5] * f64::cos(((X2OMI + X2LI) - G44)))
                    + (DG[6] * f64::cos((X2LI - G44))))
                    + (DG[9] * f64::cos(((XOMI + X2LI) - G54))))
                    + (DG[10] * f64::cos(((XOMI + X2LI) - G54))))));
    } else {
        //
        // Resonance flag not set
        //

        *XNDOT = (((DEL[1] * f64::sin((XLI - FASX2)))
            + (DEL[2] * f64::sin((2.0 * (XLI - FASX4)))))
            + (DEL[3] * f64::sin((3.0 * (XLI - FASX6)))));

        *XNDDT = (((DEL[1] * f64::cos((XLI - FASX2)))
            + ((2.0 * DEL[2]) * f64::cos((2.0 * (XLI - FASX4)))))
            + ((3.0 * DEL[3]) * f64::cos((3.0 * (XLI - FASX6)))));
    }

    *XLDOT = (XNI + XFACT);
    *XNDDT = (*XNDDT * *XLDOT);

    //
    // Hi!  What are you doing way down here?  Did you bring pizza?
    //
}
