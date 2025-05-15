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

//$Procedure ZZDSIN ( SGP4 deep space initialization )
pub fn ZZDSIN(
    GEOPHS: &[f64],
    COSIM: f64,
    EMSQ: &mut f64,
    ARGPO: f64,
    S1: f64,
    S2: f64,
    S3: f64,
    S4: f64,
    S5: f64,
    SINIM: f64,
    SS1: f64,
    SS2: f64,
    SS3: f64,
    SS4: f64,
    SS5: f64,
    SZ1: f64,
    SZ3: f64,
    SZ11: f64,
    SZ13: f64,
    SZ21: f64,
    SZ23: f64,
    SZ31: f64,
    SZ33: f64,
    T: f64,
    TC: f64,
    GSTO: f64,
    MO: f64,
    MDOT: f64,
    NO: f64,
    NODEO: f64,
    NODEDOT: f64,
    XPIDOT: f64,
    Z1: f64,
    Z3: f64,
    Z11: f64,
    Z13: f64,
    Z21: f64,
    Z23: f64,
    Z31: f64,
    Z33: f64,
    ECCO: f64,
    ECCSQ: f64,
    ECCM: &mut f64,
    ARGPM: &mut f64,
    INCLM: &mut f64,
    MM: &mut f64,
    XN: &mut f64,
    NODEM: &mut f64,
    IREZ: &mut i32,
    ATIME: &mut f64,
    D2201: &mut f64,
    D2211: &mut f64,
    D3210: &mut f64,
    D3222: &mut f64,
    D4410: &mut f64,
    D4422: &mut f64,
    D5220: &mut f64,
    D5232: &mut f64,
    D5421: &mut f64,
    D5433: &mut f64,
    DEDT: &mut f64,
    DIDT: &mut f64,
    DMDT: &mut f64,
    DNDT: &mut f64,
    DNODT: &mut f64,
    DOMDT: &mut f64,
    DEL1: &mut f64,
    DEL2: &mut f64,
    DEL3: &mut f64,
    XFACT: &mut f64,
    XLAMO: &mut f64,
    XLI: &mut f64,
    XNI: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let GEOPHS = DummyArray::new(GEOPHS, 1..=8);
    let mut AINV2: f64 = 0.0;
    let mut AONV: f64 = 0.0;
    let mut COSISQ: f64 = 0.0;
    let mut EMO: f64 = 0.0;
    let mut EMSQO: f64 = 0.0;
    let mut EOC: f64 = 0.0;
    let mut F220: f64 = 0.0;
    let mut F221: f64 = 0.0;
    let mut F311: f64 = 0.0;
    let mut F321: f64 = 0.0;
    let mut F322: f64 = 0.0;
    let mut F330: f64 = 0.0;
    let mut F441: f64 = 0.0;
    let mut F442: f64 = 0.0;
    let mut F522: f64 = 0.0;
    let mut F523: f64 = 0.0;
    let mut F542: f64 = 0.0;
    let mut F543: f64 = 0.0;
    let mut G200: f64 = 0.0;
    let mut G201: f64 = 0.0;
    let mut G211: f64 = 0.0;
    let mut G300: f64 = 0.0;
    let mut G310: f64 = 0.0;
    let mut G322: f64 = 0.0;
    let mut G410: f64 = 0.0;
    let mut G422: f64 = 0.0;
    let mut G520: f64 = 0.0;
    let mut G521: f64 = 0.0;
    let mut G532: f64 = 0.0;
    let mut G533: f64 = 0.0;
    let mut Q22: f64 = 0.0;
    let mut Q31: f64 = 0.0;
    let mut Q33: f64 = 0.0;
    let mut ROOT22: f64 = 0.0;
    let mut ROOT32: f64 = 0.0;
    let mut ROOT44: f64 = 0.0;
    let mut ROOT52: f64 = 0.0;
    let mut ROOT54: f64 = 0.0;
    let mut RPTIM: f64 = 0.0;
    let mut SES: f64 = 0.0;
    let mut SGHL: f64 = 0.0;
    let mut SGHS: f64 = 0.0;
    let mut SGS: f64 = 0.0;
    let mut SHL: f64 = 0.0;
    let mut SHS: f64 = 0.0;
    let mut SINI2: f64 = 0.0;
    let mut SIS: f64 = 0.0;
    let mut SLS: f64 = 0.0;
    let mut TEMP: f64 = 0.0;
    let mut TEMP1: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut X2O3: f64 = 0.0;
    let mut XKE: f64 = 0.0;
    let mut XNO2: f64 = 0.0;
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

    CHKIN(b"ZZDSIN", ctx)?;

    //
    // Constants
    //
    Q22 = 0.0000017891679;
    Q31 = 0.0000021460748;
    Q33 = 0.00000022123015;
    ROOT22 = 0.0000017891679;
    ROOT44 = 0.0000000073636953;
    ROOT54 = 0.0000000021765803;

    //
    // This equates to 7.29211514668855e-5 rad/sec
    //
    RPTIM = 0.0043752690880113;

    ROOT32 = 0.00000037393792;
    ROOT52 = 0.00000011428639;
    X2O3 = (2.0 / 3.0);
    ZNL = 0.00015835218;
    ZNS = 0.0000119459;

    //
    // This code block replaces the call:
    //
    // sgp4fix identify constants and allow alternate values.
    //
    // CALL getgravconst( whichconst, tumin,
    // .                  mu, radiusearthkm, xke,
    // .                  j2, j3, j4, j3oj2 )
    //
    XKE = GEOPHS[K_KE];

    //
    // DEEP SPACE INITIALIZATION
    //
    *IREZ = 0;
    if ((*XN < 0.0052359877) && (*XN > 0.0034906585)) {
        *IREZ = 1;
    }

    if (((*XN >= 0.00826) && (*XN <= 0.00924)) && (*ECCM >= 0.5)) {
        *IREZ = 2;
    }

    //
    // DO SOLAR TERMS
    //
    SES = ((SS1 * ZNS) * SS5);
    SIS = ((SS2 * ZNS) * (SZ11 + SZ13));
    SLS = -((ZNS * SS3) * (((SZ1 + SZ3) - 14.0) - (6.0 * *EMSQ)));
    SGHS = ((SS4 * ZNS) * ((SZ31 + SZ33) - 6.0));
    SHS = -((ZNS * SS2) * (SZ21 + SZ23));

    //
    // sgp4fix for 180 deg incl
    //
    if ((*INCLM < 0.052359877) || (*INCLM > (PI(ctx) - 0.052359877))) {
        SHS = 0.0;
    }

    if (SINIM != 0.0) {
        SHS = (SHS / SINIM);
    }

    SGS = (SGHS - (COSIM * SHS));

    //
    // DO LUNAR TERMS
    //
    *DEDT = (SES + ((S1 * ZNL) * S5));
    *DIDT = (SIS + ((S2 * ZNL) * (Z11 + Z13)));
    *DMDT = (SLS - ((ZNL * S3) * (((Z1 + Z3) - 14.0) - (6.0 * *EMSQ))));
    SGHL = ((S4 * ZNL) * ((Z31 + Z33) - 6.0));
    SHL = -((ZNL * S2) * (Z21 + Z23));

    //
    // sgp4fix for 180 deg incl
    //

    if ((*INCLM < 0.052359877) || (*INCLM > (PI(ctx) - 0.052359877))) {
        SHL = 0.0;
    }

    *DOMDT = (SGS + SGHL);
    *DNODT = SHS;

    if (SINIM != 0.0) {
        *DOMDT = (*DOMDT - ((COSIM / SINIM) * SHL));
        *DNODT = (*DNODT + (SHL / SINIM));
    }

    //
    // CALCULATE DEEP SPACE RESONANCE EFFECTS
    //
    *DNDT = 0.0;
    THETA = intrinsics::DMOD((GSTO + (TC * RPTIM)), TWOPI(ctx));
    *ECCM = (*ECCM + (*DEDT * T));
    *EMSQ = f64::powi(*ECCM, 2);
    *INCLM = (*INCLM + (*DIDT * T));
    *ARGPM = (*ARGPM + (*DOMDT * T));
    *NODEM = (*NODEM + (*DNODT * T));
    *MM = (*MM + (*DMDT * T));

    //
    // sgp4fix for negative inclinations
    // the following if statement should be commented out
    //
    //         IF(Inclm .lt. 0.0D0) THEN
    //           Inclm  = -Inclm
    //           Argpm  = Argpm-PI
    //           nodem = nodem+PI
    //         END IF
    //

    //
    // Initialize the resonance terms
    //
    if (*IREZ != 0) {
        AONV = f64::powf((*XN / XKE), X2O3);

        //
        // GEOPOTENTIAL RESONANCE FOR 12 HOUR ORBITS
        //
        if (*IREZ == 2) {
            COSISQ = (COSIM * COSIM);
            EMO = *ECCM;
            EMSQO = *EMSQ;
            *ECCM = ECCO;
            *EMSQ = ECCSQ;
            EOC = (*ECCM * *EMSQ);
            G201 = (-0.306 - ((*ECCM - 0.64) * 0.44));

            if (*ECCM <= 0.65) {
                G211 = ((3.616 - (13.247 * *ECCM)) + (16.29 * *EMSQ));
                G310 = (((-19.302 + (117.39 * *ECCM)) - (228.419 * *EMSQ)) + (156.591 * EOC));
                G322 = (((-18.9068 + (109.7927 * *ECCM)) - (214.6334 * *EMSQ)) + (146.5816 * EOC));
                G410 = (((-41.122 + (242.694 * *ECCM)) - (471.094 * *EMSQ)) + (313.953 * EOC));
                G422 = (((-146.407 + (841.88 * *ECCM)) - (1629.014 * *EMSQ)) + (1083.435 * EOC));
                G520 = (((-532.114 + (3017.977 * *ECCM)) - (5740.032 * *EMSQ)) + (3708.276 * EOC));
            } else {
                G211 = (((-72.099 + (331.819 * *ECCM)) - (508.738 * *EMSQ)) + (266.724 * EOC));
                G310 = (((-346.844 + (1582.851 * *ECCM)) - (2415.925 * *EMSQ)) + (1246.113 * EOC));
                G322 = (((-342.585 + (1554.908 * *ECCM)) - (2366.899 * *EMSQ)) + (1215.972 * EOC));
                G410 = (((-1052.797 + (4758.686 * *ECCM)) - (7193.992 * *EMSQ)) + (3651.957 * EOC));
                G422 = (((-3581.69 + (16178.11 * *ECCM)) - (24462.77 * *EMSQ)) + (12422.52 * EOC));

                if (*ECCM > 0.715) {
                    G520 =
                        (((-5149.66 + (29936.92 * *ECCM)) - (54087.36 * *EMSQ)) + (31324.56 * EOC));
                } else {
                    G520 = ((1464.74 - (4664.75 * *ECCM)) + (3763.64 * *EMSQ));
                }
            }

            if (*ECCM < 0.7) {
                G533 = (((-919.2277 + (4988.61 * *ECCM)) - (9064.77 * *EMSQ)) + (5542.21 * EOC));
                G521 =
                    (((-822.71072 + (4568.6173 * *ECCM)) - (8491.4146 * *EMSQ)) + (5337.524 * EOC));
                G532 = (((-853.666 + (4690.25 * *ECCM)) - (8624.77 * *EMSQ)) + (5341.4 * EOC));
            } else {
                G533 =
                    (((-37995.78 + (161616.52 * *ECCM)) - (229838.2 * *EMSQ)) + (109377.94 * EOC));
                G521 = (((-51752.104 + (218913.95 * *ECCM)) - (309468.16 * *EMSQ))
                    + (146349.42 * EOC));
                G532 =
                    (((-40023.88 + (170470.89 * *ECCM)) - (242699.48 * *EMSQ)) + (115605.82 * EOC));
            }

            SINI2 = (SINIM * SINIM);
            F220 = (0.75 * ((1.0 + (2.0 * COSIM)) + COSISQ));
            F221 = (1.5 * SINI2);
            F321 = ((1.875 * SINIM) * ((1.0 - (2.0 * COSIM)) - (3.0 * COSISQ)));
            F322 = -((1.875 * SINIM) * ((1.0 + (2.0 * COSIM)) - (3.0 * COSISQ)));
            F441 = ((35.0 * SINI2) * F220);
            F442 = ((39.375 * SINI2) * SINI2);
            F522 = ((9.84375 * SINIM)
                * ((SINI2 * ((1.0 - (2.0 * COSIM)) - (5.0 * COSISQ)))
                    + (0.33333333 * ((-2.0 + (4.0 * COSIM)) + (6.0 * COSISQ)))));
            F523 = (SINIM
                * (((4.92187512 * SINI2) * ((-2.0 - (4.0 * COSIM)) + (10.0 * COSISQ)))
                    + (6.56250012 * ((1.0 + (2.0 * COSIM)) - (3.0 * COSISQ)))));
            F542 = ((29.53125 * SINIM)
                * ((2.0 - (8.0 * COSIM)) + (COSISQ * ((-12.0 + (8.0 * COSIM)) + (10.0 * COSISQ)))));
            F543 = ((29.53125 * SINIM)
                * ((-2.0 - (8.0 * COSIM)) + (COSISQ * ((12.0 + (8.0 * COSIM)) - (10.0 * COSISQ)))));

            XNO2 = (*XN * *XN);
            AINV2 = (AONV * AONV);
            TEMP1 = ((3.0 * XNO2) * AINV2);
            TEMP = (TEMP1 * ROOT22);
            *D2201 = ((TEMP * F220) * G201);
            *D2211 = ((TEMP * F221) * G211);
            TEMP1 = (TEMP1 * AONV);
            TEMP = (TEMP1 * ROOT32);
            *D3210 = ((TEMP * F321) * G310);
            *D3222 = ((TEMP * F322) * G322);
            TEMP1 = (TEMP1 * AONV);
            TEMP = ((2.0 * TEMP1) * ROOT44);
            *D4410 = ((TEMP * F441) * G410);
            *D4422 = ((TEMP * F442) * G422);
            TEMP1 = (TEMP1 * AONV);
            TEMP = (TEMP1 * ROOT52);
            *D5220 = ((TEMP * F522) * G520);
            *D5232 = ((TEMP * F523) * G532);
            TEMP = ((2.0 * TEMP1) * ROOT54);
            *D5421 = ((TEMP * F542) * G521);
            *D5433 = ((TEMP * F543) * G533);
            *XLAMO = intrinsics::DMOD(((((MO + NODEO) + NODEO) - THETA) - THETA), TWOPI(ctx));
            *XFACT = (((MDOT + *DMDT) + (2.0 * ((NODEDOT + *DNODT) - RPTIM))) - NO);

            *ECCM = EMO;
            *EMSQ = EMSQO;
        }

        if (*IREZ == 1) {
            //
            // SYNCHRONOUS RESONANCE TERMS
            //
            G200 = (1.0 + (*EMSQ * (-2.5 + (0.8125 * *EMSQ))));
            G310 = (1.0 + (2.0 * *EMSQ));
            G300 = (1.0 + (*EMSQ * (-6.0 + (6.60937 * *EMSQ))));
            F220 = ((0.75 * (1.0 + COSIM)) * (1.0 + COSIM));
            F311 = ((((0.9375 * SINIM) * SINIM) * (1.0 + (3.0 * COSIM))) - (0.75 * (1.0 + COSIM)));
            F330 = (1.0 + COSIM);
            F330 = (((1.875 * F330) * F330) * F330);
            *DEL1 = ((((3.0 * *XN) * *XN) * AONV) * AONV);
            *DEL2 = ((((2.0 * *DEL1) * F220) * G200) * Q22);
            *DEL3 = (((((3.0 * *DEL1) * F330) * G300) * Q33) * AONV);
            *DEL1 = ((((*DEL1 * F311) * G310) * Q31) * AONV);
            *XLAMO = intrinsics::DMOD((((MO + NODEO) + ARGPO) - THETA), TWOPI(ctx));
            *XFACT = ((((((MDOT + XPIDOT) - RPTIM) + *DMDT) + *DOMDT) + *DNODT) - NO);
        }

        //
        // FOR SGP4, INITIALIZE THE INTEGRATOR
        //
        *XLI = *XLAMO;
        *XNI = NO;
        *ATIME = 0.0;
        *XN = (NO + *DNDT);
    }

    CHKOUT(b"ZZDSIN", ctx)?;

    Ok(())
}
