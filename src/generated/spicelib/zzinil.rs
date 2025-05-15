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

//$Procedure ZZINIL ( SGP4 initializer )
pub fn ZZINIL(
    GEOPHS: &[f64],
    OPMODE: i32,
    ECCO: f64,
    EPOCH: f64,
    INCLO: f64,
    NO: &mut f64,
    AINV: &mut f64,
    AO: &mut f64,
    CON41: &mut f64,
    CON42: &mut f64,
    COSIO: &mut f64,
    COSIO2: &mut f64,
    ECCSQ: &mut f64,
    OMEOSQ: &mut f64,
    POSQ: &mut f64,
    RP: &mut f64,
    RTEOSQ: &mut f64,
    SINIO: &mut f64,
    GSTO: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let GEOPHS = DummyArray::new(GEOPHS, 1..=8);
    let mut ADEL: f64 = 0.0;
    let mut AK: f64 = 0.0;
    let mut C1: f64 = 0.0;
    let mut C1P2P: f64 = 0.0;
    let mut D1: f64 = 0.0;
    let mut DEL: f64 = 0.0;
    let mut FK5R: f64 = 0.0;
    let mut J2: f64 = 0.0;
    let mut PO: f64 = 0.0;
    let mut RADDAY: f64 = 0.0;
    let mut TEMP: f64 = 0.0;
    let mut TFRAC: f64 = 0.0;
    let mut THGR70: f64 = 0.0;
    let mut TS70: f64 = 0.0;
    let mut TUT1: f64 = 0.0;
    let mut X2O3: f64 = 0.0;
    let mut XKE: f64 = 0.0;
    let mut IDS70: i32 = 0;

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

    CHKIN(b"ZZINIL", ctx)?;

    //
    // This code block replaces the call:
    //
    // sgp4fix identify constants and allow alternate values.
    //
    // CALL getgravconst( whichconst, tumin,
    // .                  mu, radiusearthkm, xke,
    // .                  j2, j3, j4, j3oj2 )
    //
    J2 = GEOPHS[K_J2];
    XKE = GEOPHS[K_KE];
    X2O3 = (2.0 / 3.0);

    //
    // Calculate auxillary epoch quantities
    //
    *ECCSQ = (ECCO * ECCO);
    *OMEOSQ = (1.0 - *ECCSQ);
    *RTEOSQ = f64::sqrt(*OMEOSQ);
    *COSIO = f64::cos(INCLO);
    *COSIO2 = (*COSIO * *COSIO);

    //
    // Un-KOZAI the mean motion
    //
    AK = f64::powf((XKE / *NO), X2O3);
    D1 = (((0.75 * J2) * ((3.0 * *COSIO2) - 1.0)) / (*RTEOSQ * *OMEOSQ));
    DEL = (D1 / (AK * AK));
    ADEL = (AK * ((1.0 - (DEL * DEL)) - (DEL * ((1.0 / 3.0) + (((134.0 * DEL) * DEL) / 81.0)))));
    DEL = (D1 / (ADEL * ADEL));
    *NO = (*NO / (1.0 + DEL));

    *AO = f64::powf((XKE / *NO), X2O3);
    *SINIO = f64::sin(INCLO);
    PO = (*AO * *OMEOSQ);
    *CON42 = (1.0 - (5.0 * *COSIO2));
    *CON41 = ((-*CON42 - *COSIO2) - *COSIO2);
    *AINV = (1.0 / *AO);
    *POSQ = (PO * PO);
    *RP = (*AO * (1.0 - ECCO));

    //
    // Calculate greenwich location at epoch
    //

    //
    // sgp4fix Modern approach to finding sidereal time
    //

    if (OPMODE == IMPRVD) {
        //
        // Radians per day, earth rotation, 6.30038809866574D0.
        //
        RADDAY = (TWOPI(ctx) * 1.002737909350795);
        TEMP = (EPOCH + 2433281.5);
        TUT1 = (((f64::trunc((TEMP - 0.5)) + 0.5) - 2451545.0) / 36525.0);
        *GSTO = ((((1.75336855923327 + (628.331970688841 * TUT1))
            + ((0.00000677071394490334 * TUT1) * TUT1))
            - (((0.000000000450876723431868 * TUT1) * TUT1) * TUT1))
            + (RADDAY * ((TEMP - 0.5) - f64::trunc((TEMP - 0.5)))));
    } else if (OPMODE == AFSPC) {
        //
        // sgp4fix Use old way of finding GST
        //
        // Count integer number of days from 0 jan 1970
        //
        TS70 = (EPOCH - 7305.0);
        IDS70 = ((TS70 + 0.00000001) as i32);
        TFRAC = (TS70 - IDS70 as f64);

        //
        // Find greenwich location at epoch
        //
        C1 = 0.017202791694070362;
        THGR70 = 1.7321343856509375;
        FK5R = 0.000000000000005075514194322695;
        C1P2P = (C1 + TWOPI(ctx));
        *GSTO = (((THGR70 + (C1 * IDS70 as f64)) + (C1P2P * TFRAC)) + ((TS70 * TS70) * FK5R));
    } else {
        SETMSG(
            b"Unknown value for OPMODE. Value # not coded in zzsgp4.inc.",
            ctx,
        );
        ERRINT(b"#", OPMODE, ctx);
        SIGERR(b"SPICE(UNKNOWNMODE)", ctx)?;
        CHKOUT(b"ZZINIL", ctx)?;
        return Ok(());
    }

    //
    // Check quadrants
    //
    *GSTO = intrinsics::DMOD(*GSTO, TWOPI(ctx));

    if (*GSTO < 0.0) {
        *GSTO = (*GSTO + TWOPI(ctx));
    }

    CHKOUT(b"ZZINIL", ctx)?;

    Ok(())
}
