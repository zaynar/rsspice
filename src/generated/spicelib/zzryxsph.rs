//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZRYXSPH ( Intersection of ray and sphere )
pub fn ZZRYXSPH(VERTEX: &[f64], UDIR: &[f64], R: f64, XPT: &mut [f64], FOUND: &mut bool) {
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let UDIR = DummyArray::new(UDIR, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut CPAR: f64 = 0.0;
    let mut PERP = StackArray::<f64, 3>::new(1..=3);
    let mut PMAG2: f64 = 0.0;
    let mut R2: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut VMAG2: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    *FOUND = false;

    //
    // Find the component of VERTEX orthogonal to UDIR. If the magnitude
    // of this component exceeds R, there's no intercept.
    //
    CPAR = VDOT(VERTEX.as_slice(), UDIR.as_slice());

    VLCOM(
        1.0,
        VERTEX.as_slice(),
        -CPAR,
        UDIR.as_slice(),
        PERP.as_slice_mut(),
    );

    PMAG2 = VDOT(PERP.as_slice(), PERP.as_slice());
    R2 = (R * R);

    //
    // Compare squares of magnitudes, rather than magnitudes, for
    // efficiency.
    //
    if (PMAG2 > R2) {
        return;
    }

    S = f64::sqrt(intrinsics::DMAX1(&[0.0, (R2 - PMAG2)]));

    VMAG2 = VDOT(VERTEX.as_slice(), VERTEX.as_slice());

    if (VMAG2 > R2) {
        //
        // If the magnitude of the vertex exceeds R, the vertex is
        // outside the sphere. Above, we have compared squares of
        // magnitudes for efficiency.
        //
        if (CPAR > 0.0) {
            //
            // The ray points away from the sphere; there can be no
            // intersection.
            //
            return;
        }
        //
        // Given that an intercept exists, we can find it between VERTEX
        // and VPERP by following -UDIR from PERP towards VERTEX.
        //
        XPT[1] = (PERP[1] - (S * UDIR[1]));
        XPT[2] = (PERP[2] - (S * UDIR[2]));
        XPT[3] = (PERP[3] - (S * UDIR[3]));
    } else if (VMAG2 < R2) {
        //
        // The vertex is inside the sphere. We can calculate the exit
        // point by using PERP as a vertex.
        //
        XPT[1] = (PERP[1] + (S * UDIR[1]));
        XPT[2] = (PERP[2] + (S * UDIR[2]));
        XPT[3] = (PERP[3] + (S * UDIR[3]));
    } else {
        //
        // PERP is the sole intercept.
        //
        XPT[1] = PERP[1];
        XPT[2] = PERP[2];
        XPT[3] = PERP[3];
    }

    *FOUND = true;
}
