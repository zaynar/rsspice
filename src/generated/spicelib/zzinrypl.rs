//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZINRYPL ( Simplified intersection of ray and plane )
pub fn ZZINRYPL(
    VERTEX: &[f64],
    UDIR: &[f64],
    UPLNML: &[f64],
    CONST: f64,
    MAXD: f64,
    NXPTS: &mut i32,
    XPT: &mut [f64],
) {
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let UDIR = DummyArray::new(UDIR, 1..=3);
    let UPLNML = DummyArray::new(UPLNML, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut DIRCON: f64 = 0.0;
    let mut H: f64 = 0.0;
    let mut LPAR: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut VTXCON: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Start out indicating no intersection.
    //
    *NXPTS = 0;

    //
    // VTXCON is the plane constant of the ray's vertex.
    //
    VTXCON = VDOT(VERTEX.as_slice(), UPLNML.as_slice());
    //
    // DIRCON is the length of the component of the ray's
    // direction vector in the direction of UPLNML.
    //
    DIRCON = VDOT(UDIR.as_slice(), UPLNML.as_slice());

    //
    // Dispose of the easy non-intersection cases. (The ray
    // lying in the plane is considered a non-intersection case,
    // by the way.)
    //
    if ((VTXCON > CONST) && (DIRCON > 0.0)) {
        return;
    }

    if ((VTXCON < CONST) && (DIRCON < 0.0)) {
        return;
    }

    if (VTXCON == CONST) {
        //
        // The ray's vertex lies in the plane.
        //
        if (DIRCON != 0.0) {
            //
            // The ray does not lie in the plane. The
            // intercept is the ray's vertex.
            //
            *NXPTS = 1;
            VEQU(VERTEX.as_slice(), XPT.as_slice_mut());
        }

        return;
    }

    //
    // Let UPAR and UPERP be, respectively, the components of UDIR
    // parallel to and perpendicular to UPLNML.
    //
    // Compute the maximum allowed length of UPERP.
    //
    //
    H = f64::abs((VTXCON - CONST));
    LPAR = f64::abs(DIRCON);

    //
    // To prevent overflow, we require
    //
    //      H
    //    ----  <= MAXD
    //    LPAR
    //
    // or equivalently
    //
    //    H  <=  MAXD * LPAR
    //
    if (H > (MAXD * LPAR)) {
        return;
    }

    //
    // For safety, return if we could have a divide-by-zero error.
    //
    if (LPAR == 0.0) {
        return;
    }

    //
    // Still being here means we can compute XPT, provided
    // the given value of MAXD was reasonable.
    //
    // Note that the earlier tests we performed should
    // rule out the case
    //
    //    DIRCON = 0
    //
    // We have also ruled out overflow in the computation below.
    //
    S = (H / LPAR);

    XPT[1] = (VERTEX[1] + (S * UDIR[1]));
    XPT[2] = (VERTEX[2] + (S * UDIR[2]));
    XPT[3] = (VERTEX[3] + (S * UDIR[3]));

    *NXPTS = 1;
}
