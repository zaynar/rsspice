//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NGRAVS: i32 = 3;
const NGRAVC: i32 = 8;
const WGS721: i32 = 1;
const WGS72: i32 = 2;
const WGS84: i32 = 3;
const P_RAD: i32 = 1;
const P_XKE: i32 = 2;
const P_MU: i32 = 3;
const P_TUMN: i32 = 4;
const P_J2: i32 = 5;
const P_J3: i32 = 6;
const P_J4: i32 = 7;
const P_J3J2: i32 = 8;
const K_J2: i32 = 1;
const K_J3: i32 = 2;
const K_J4: i32 = 3;
const K_KE: i32 = 4;
const K_QO: i32 = 5;
const K_SO: i32 = 6;
const K_ER: i32 = 7;
const K_AE: i32 = 8;
const NGEO: i32 = K_AE;
const AFSPC: i32 = 1;
const IMPRVD: i32 = 2;
const KNDT20: i32 = 1;
const KNDD60: i32 = 2;
const KBSTAR: i32 = 3;
const KINCL: i32 = 4;
const KNODE0: i32 = 5;
const KECC: i32 = 6;
const KOMEGA: i32 = 7;
const KMO: i32 = 8;
const KNO: i32 = 9;
const KEPOCH: i32 = 10;
const NELEMS: i32 = KEPOCH;

//$Procedure ZZGRAV ( SGP4 gravitational constants )
pub fn ZZGRAV(GRAV: &mut [f64]) {
    let mut GRAV = DummyArrayMut2D::new(GRAV, 1..=NGRAVS, 1..=NGRAVC);

    //
    // The definition of the MU constant is to allow calculation of
    // those constants which are functions of MU in the WGS72 and WGS84
    // assignments.
    //

    //
    // WGS72 low precision Spacetrack #3 constants. These values should
    // correspond to those in geophysical.ker. If not, something is
    // very wrong.
    //

    GRAV[[WGS721, P_RAD]] = 6378.135;

    GRAV[[WGS721, P_XKE]] = 0.0743669161;

    GRAV[[WGS721, P_MU]] = 398600.79964;

    GRAV[[WGS721, P_TUMN]] = (1.0 / GRAV[[WGS721, P_XKE]]);

    GRAV[[WGS721, P_J2]] = 0.001082616;

    GRAV[[WGS721, P_J3]] = -0.00000253881;

    GRAV[[WGS721, P_J4]] = -0.00000165597;

    GRAV[[WGS721, P_J3J2]] = (GRAV[[WGS721, P_J3]] / GRAV[[WGS721, P_J2]]);

    //
    // WGS 72 constants.
    //

    GRAV[[WGS72, P_MU]] = 398600.8;

    GRAV[[WGS72, P_RAD]] = 6378.135;

    GRAV[[WGS72, P_XKE]] =
        (60.0 / f64::sqrt((f64::powi(GRAV[[WGS72, P_RAD]], 3) / GRAV[[WGS72, P_MU]])));

    GRAV[[WGS72, P_TUMN]] = (1.0 / GRAV[[WGS72, P_XKE]]);

    GRAV[[WGS72, P_J2]] = 0.001082616;

    GRAV[[WGS72, P_J3]] = -0.00000253881;

    GRAV[[WGS72, P_J4]] = -0.00000165597;

    GRAV[[WGS72, P_J3J2]] = (GRAV[[WGS72, P_J3]] / GRAV[[WGS72, P_J2]]);

    //
    // WGS 84 constants.
    //

    GRAV[[WGS84, P_MU]] = 398600.5;

    GRAV[[WGS84, P_RAD]] = 6378.137;

    GRAV[[WGS84, P_XKE]] =
        (60.0 / f64::sqrt((f64::powi(GRAV[[WGS84, P_RAD]], 3) / GRAV[[WGS84, P_MU]])));

    GRAV[[WGS84, P_TUMN]] = (1.0 / GRAV[[WGS84, P_XKE]]);

    GRAV[[WGS84, P_J2]] = 0.00108262998905;

    GRAV[[WGS84, P_J3]] = -0.00000253215306;

    GRAV[[WGS84, P_J4]] = -0.00000161098761;

    GRAV[[WGS84, P_J3J2]] = (GRAV[[WGS84, P_J3]] / GRAV[[WGS84, P_J2]]);
}
