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

//$Procedure ZZDSPR ( SGP4 deep space long period )
pub fn ZZDSPR(
    OPMODE: i32,
    E3: f64,
    EE2: f64,
    PEO: f64,
    PGHO: f64,
    PHO: f64,
    PINCO: f64,
    PLO: f64,
    SE2: f64,
    SE3: f64,
    SGH2: f64,
    SGH3: f64,
    SGH4: f64,
    SH2: f64,
    SH3: f64,
    SI2: f64,
    SI3: f64,
    SL2: f64,
    SL3: f64,
    SL4: f64,
    T: f64,
    XGH2: f64,
    XGH3: f64,
    XGH4: f64,
    XH2: f64,
    XH3: f64,
    XI2: f64,
    XI3: f64,
    XL2: f64,
    XL3: f64,
    XL4: f64,
    ZMOL: f64,
    ZMOS: f64,
    INCLO: f64,
    DOINIT: bool,
    ECCP: &mut f64,
    INCLP: &mut f64,
    NODEP: &mut f64,
    ARGPP: &mut f64,
    MP: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ALFDP: f64 = 0.0;
    let mut BETDP: f64 = 0.0;
    let mut COSIP: f64 = 0.0;
    let mut COSOP: f64 = 0.0;
    let mut DALF: f64 = 0.0;
    let mut DBET: f64 = 0.0;
    let mut DLS: f64 = 0.0;
    let mut F2: f64 = 0.0;
    let mut F3: f64 = 0.0;
    let mut PE: f64 = 0.0;
    let mut PGH: f64 = 0.0;
    let mut PH: f64 = 0.0;
    let mut PINC: f64 = 0.0;
    let mut PL: f64 = 0.0;
    let mut SEL: f64 = 0.0;
    let mut SES: f64 = 0.0;
    let mut SGHL: f64 = 0.0;
    let mut SGHS: f64 = 0.0;
    let mut SHL: f64 = 0.0;
    let mut SHS: f64 = 0.0;
    let mut SIL: f64 = 0.0;
    let mut SINIP: f64 = 0.0;
    let mut SINOP: f64 = 0.0;
    let mut SINZF: f64 = 0.0;
    let mut SIS: f64 = 0.0;
    let mut SLL: f64 = 0.0;
    let mut SLS: f64 = 0.0;
    let mut XLS: f64 = 0.0;
    let mut XNOH: f64 = 0.0;
    let mut ZEL: f64 = 0.0;
    let mut ZES: f64 = 0.0;
    let mut ZF: f64 = 0.0;
    let mut ZM: f64 = 0.0;
    let mut ZNL: f64 = 0.0;
    let mut ZNS: f64 = 0.0;

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

    CHKIN(b"ZZDSPR", ctx)?;

    //
    // Local constants.
    //
    ZES = 0.01675;
    ZEL = 0.0549;
    ZNS = 0.0000119459;
    ZNL = 0.00015835218;

    //
    // Calculate time varying periodics.
    //
    ZM = (ZMOS + (ZNS * T));

    if DOINIT {
        ZM = ZMOS;
    }

    ZF = (ZM + ((2.0 * ZES) * f64::sin(ZM)));
    SINZF = f64::sin(ZF);
    F2 = (((0.5 * SINZF) * SINZF) - 0.25);
    F3 = -((0.5 * SINZF) * f64::cos(ZF));
    SES = ((SE2 * F2) + (SE3 * F3));
    SIS = ((SI2 * F2) + (SI3 * F3));
    SLS = (((SL2 * F2) + (SL3 * F3)) + (SL4 * SINZF));
    SGHS = (((SGH2 * F2) + (SGH3 * F3)) + (SGH4 * SINZF));
    SHS = ((SH2 * F2) + (SH3 * F3));
    ZM = (ZMOL + (ZNL * T));

    if DOINIT {
        ZM = ZMOL;
    }

    ZF = (ZM + ((2.0 * ZEL) * f64::sin(ZM)));
    SINZF = f64::sin(ZF);
    F2 = (((0.5 * SINZF) * SINZF) - 0.25);
    F3 = -((0.5 * SINZF) * f64::cos(ZF));
    SEL = ((EE2 * F2) + (E3 * F3));
    SIL = ((XI2 * F2) + (XI3 * F3));
    SLL = (((XL2 * F2) + (XL3 * F3)) + (XL4 * SINZF));
    SGHL = (((XGH2 * F2) + (XGH3 * F3)) + (XGH4 * SINZF));
    SHL = ((XH2 * F2) + (XH3 * F3));
    PE = (SES + SEL);
    PINC = (SIS + SIL);
    PL = (SLS + SLL);
    PGH = (SGHS + SGHL);
    PH = (SHS + SHL);

    if !DOINIT {
        PE = (PE - PEO);
        PINC = (PINC - PINCO);
        PL = (PL - PLO);
        PGH = (PGH - PGHO);
        PH = (PH - PHO);
        *INCLP = (*INCLP + PINC);
        *ECCP = (*ECCP + PE);
        SINIP = f64::sin(*INCLP);
        COSIP = f64::cos(*INCLP);

        //
        // Apply periodics directly.
        //
        // sgp4fix for lyddane choice
        //
        // strn3 used original inclination - this is technically
        // feasible
        //
        // gsfc used perturbed inclination - also technically feasible
        // probably best to readjust the 0.2 limit value and limit
        // discontinuity
        //
        // 0.2 rad = 11.45916 deg
        //
        // use next line for original strn3 approach and original
        // inclination
        //
        //     IF (INCLO.GE.0.2D0) THEN
        //
        // use next line for gsfc version and perturbed inclination
        //
        if (*INCLP >= 0.2) {
            PH = (PH / SINIP);
            PGH = (PGH - (COSIP * PH));
            *ARGPP = (*ARGPP + PGH);
            *NODEP = (*NODEP + PH);
            *MP = (*MP + PL);
        } else {
            //
            // Apply periodics with Lyddane modification.
            //
            SINOP = f64::sin(*NODEP);
            COSOP = f64::cos(*NODEP);
            ALFDP = (SINIP * SINOP);
            BETDP = (SINIP * COSOP);
            DALF = ((PH * COSOP) + ((PINC * COSIP) * SINOP));
            DBET = (-(PH * SINOP) + ((PINC * COSIP) * COSOP));
            ALFDP = (ALFDP + DALF);
            BETDP = (BETDP + DBET);
            *NODEP = intrinsics::DMOD(*NODEP, TWOPI(ctx));

            //
            // sgp4fix for afspc written intrinsic functions
            // NODEP used without a trigonometric function ahead
            //

            if ((*NODEP < 0.0) && (OPMODE == AFSPC)) {
                *NODEP = (*NODEP + TWOPI(ctx));
            }

            XLS = ((*MP + *ARGPP) + (COSIP * *NODEP));
            DLS = ((PL + PGH) - ((PINC * *NODEP) * SINIP));
            XLS = (XLS + DLS);
            XNOH = *NODEP;
            *NODEP = f64::atan2(ALFDP, BETDP);

            //
            // sgp4fix for afspc written intrinsic functions
            // NODEP used without a trigonometric function ahead
            //

            if ((*NODEP < 0.0) && (OPMODE == AFSPC)) {
                *NODEP = (*NODEP + TWOPI(ctx));
            }

            if (f64::abs((XNOH - *NODEP)) > PI(ctx)) {
                if (*NODEP < XNOH) {
                    *NODEP = (*NODEP + TWOPI(ctx));
                } else {
                    *NODEP = (*NODEP - TWOPI(ctx));
                }
            }

            *MP = (*MP + PL);
            *ARGPP = ((XLS - *MP) - (COSIP * *NODEP));
        }
    }

    CHKOUT(b"ZZDSPR", ctx)?;

    Ok(())
}
