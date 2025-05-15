//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ZNS: f64 = 0.0000119459;
const C1SS: f64 = 0.0000029864797;
const ZES: f64 = 0.01675;
const ZNL: f64 = 0.00015835218;
const C1L: f64 = 0.00000047968065;
const ZEL: f64 = 0.0549;
const ZCOSIS: f64 = 0.91744867;
const ZSINIS: f64 = 0.39785416;
const ZSINGS: f64 = -0.98088458;
const ZCOSGS: f64 = 0.1945905;
const Q22: f64 = 0.0000017891679;
const Q31: f64 = 0.0000021460748;
const Q33: f64 = 0.00000022123015;
const ROOT22: f64 = 0.0000017891679;
const ROOT32: f64 = 0.00000037393792;
const ROOT44: f64 = 0.0000000073636953;
const ROOT52: f64 = 0.00000011428639;
const ROOT54: f64 = 0.0000000021765803;
const THDT: f64 = 0.0043752691;
const STEP2: f64 = 259200.0;
const LUNAR: i32 = 1;
const ZERO: f64 = 0.0;
const ONE: f64 = 1.0;

struct SaveVars {
    A1: f64,
    A10: f64,
    A2: f64,
    A3: f64,
    A4: f64,
    A5: f64,
    A6: f64,
    A7: f64,
    A8: f64,
    A9: f64,
    AINV2: f64,
    ALFDP: f64,
    AQNV: f64,
    ATIME: f64,
    BETDP: f64,
    BFACT: f64,
    BSQ: f64,
    C: f64,
    CC: f64,
    COSIQ: f64,
    COSIS: f64,
    COSOK: f64,
    COSOMO: f64,
    COSQ: f64,
    COSQ2: f64,
    CTEM: f64,
    DAY: f64,
    DELT: f64,
    DS50: f64,
    E3: f64,
    EE2: f64,
    EO: f64,
    EOC: f64,
    EQ: f64,
    EQSQ: f64,
    ET: f64,
    F2: f64,
    F220: f64,
    F221: f64,
    F3: f64,
    F311: f64,
    F321: f64,
    F322: f64,
    F330: f64,
    F441: f64,
    F442: f64,
    F522: f64,
    F523: f64,
    F542: f64,
    F543: f64,
    FT: f64,
    G200: f64,
    G201: f64,
    G211: f64,
    G300: f64,
    G310: f64,
    G322: f64,
    G410: f64,
    G422: f64,
    G520: f64,
    G521: f64,
    G532: f64,
    G533: f64,
    GAM: f64,
    OMEGAO: f64,
    OXNODE: f64,
    PE: f64,
    PE0: f64,
    PGH: f64,
    PGH0: f64,
    PH: f64,
    PH0: f64,
    PINC: f64,
    PINC0: f64,
    PIX1: f64,
    PIX2: f64,
    PL: f64,
    PL0: f64,
    PREEP: f64,
    RTEQSQ: f64,
    S1: f64,
    S2: f64,
    S3: f64,
    S4: f64,
    S5: f64,
    S6: f64,
    S7: f64,
    SE: f64,
    SE2: f64,
    SE3: f64,
    SEL: f64,
    SES: f64,
    SGH: f64,
    SGH2: f64,
    SGH3: f64,
    SGH4: f64,
    SGHL: f64,
    SGHS: f64,
    SH: f64,
    SH2: f64,
    SH3: f64,
    SHL: f64,
    SHS: f64,
    SI: f64,
    SI2: f64,
    SI3: f64,
    SIL: f64,
    SINI2: f64,
    SINIQ: f64,
    SINIS: f64,
    SINOK: f64,
    SINOMO: f64,
    SINQ: f64,
    SINZF: f64,
    SIS: f64,
    SL: f64,
    SL2: f64,
    SL3: f64,
    SL4: f64,
    SLL: f64,
    SLS: f64,
    STEM: f64,
    STEPN: f64,
    STEPP: f64,
    TEMP: f64,
    TEMP1: f64,
    THETA: f64,
    THGR: f64,
    X1: f64,
    X2: f64,
    X3: f64,
    X4: f64,
    X5: f64,
    X6: f64,
    X7: f64,
    X8: f64,
    XFACT: f64,
    XGH2: f64,
    XGH3: f64,
    XGH4: f64,
    XH2: f64,
    XH3: f64,
    XI2: f64,
    XI3: f64,
    XINCL: f64,
    XL: f64,
    XL2: f64,
    XL3: f64,
    XL4: f64,
    XLAMO: f64,
    XLDOT: f64,
    XLI: f64,
    XLS: f64,
    XMAO: f64,
    XMO: f64,
    XNDDT: f64,
    XNDOT: f64,
    XNI: f64,
    XNO2: f64,
    XNODCE: f64,
    XNODEO: f64,
    XNOI: f64,
    XNQ: f64,
    XQNCL: f64,
    Z1: f64,
    Z11: f64,
    Z12: f64,
    Z13: f64,
    Z2: f64,
    Z21: f64,
    Z22: f64,
    Z23: f64,
    Z3: f64,
    Z31: f64,
    Z32: f64,
    Z33: f64,
    ZCOSG: f64,
    ZCOSGL: f64,
    ZCOSH: f64,
    ZCOSHL: f64,
    ZCOSI: f64,
    ZCOSIL: f64,
    ZE: f64,
    ZF: f64,
    ZM: f64,
    ZMOL: f64,
    ZMOS: f64,
    ZN: f64,
    ZSING: f64,
    ZSINGL: f64,
    ZSINH: f64,
    ZSINHL: f64,
    ZSINI: f64,
    ZSINIL: f64,
    ZX: f64,
    ZY: f64,
    DEL: StackArray<f64, 3>,
    DG: StackArray<f64, 10>,
    SSX: StackArray<f64, 5>,
    JDTDB: f64,
    JDUT50: f64,
    IRESFL: i32,
    ISYNFL: i32,
    CONT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut A1: f64 = 0.0;
        let mut A10: f64 = 0.0;
        let mut A2: f64 = 0.0;
        let mut A3: f64 = 0.0;
        let mut A4: f64 = 0.0;
        let mut A5: f64 = 0.0;
        let mut A6: f64 = 0.0;
        let mut A7: f64 = 0.0;
        let mut A8: f64 = 0.0;
        let mut A9: f64 = 0.0;
        let mut AINV2: f64 = 0.0;
        let mut ALFDP: f64 = 0.0;
        let mut AQNV: f64 = 0.0;
        let mut ATIME: f64 = 0.0;
        let mut BETDP: f64 = 0.0;
        let mut BFACT: f64 = 0.0;
        let mut BSQ: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut CC: f64 = 0.0;
        let mut COSIQ: f64 = 0.0;
        let mut COSIS: f64 = 0.0;
        let mut COSOK: f64 = 0.0;
        let mut COSOMO: f64 = 0.0;
        let mut COSQ: f64 = 0.0;
        let mut COSQ2: f64 = 0.0;
        let mut CTEM: f64 = 0.0;
        let mut DAY: f64 = 0.0;
        let mut DELT: f64 = 0.0;
        let mut DS50: f64 = 0.0;
        let mut E3: f64 = 0.0;
        let mut EE2: f64 = 0.0;
        let mut EO: f64 = 0.0;
        let mut EOC: f64 = 0.0;
        let mut EQ: f64 = 0.0;
        let mut EQSQ: f64 = 0.0;
        let mut ET: f64 = 0.0;
        let mut F2: f64 = 0.0;
        let mut F220: f64 = 0.0;
        let mut F221: f64 = 0.0;
        let mut F3: f64 = 0.0;
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
        let mut FT: f64 = 0.0;
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
        let mut GAM: f64 = 0.0;
        let mut OMEGAO: f64 = 0.0;
        let mut OXNODE: f64 = 0.0;
        let mut PE: f64 = 0.0;
        let mut PE0: f64 = 0.0;
        let mut PGH: f64 = 0.0;
        let mut PGH0: f64 = 0.0;
        let mut PH: f64 = 0.0;
        let mut PH0: f64 = 0.0;
        let mut PINC: f64 = 0.0;
        let mut PINC0: f64 = 0.0;
        let mut PIX1: f64 = 0.0;
        let mut PIX2: f64 = 0.0;
        let mut PL: f64 = 0.0;
        let mut PL0: f64 = 0.0;
        let mut PREEP: f64 = 0.0;
        let mut RTEQSQ: f64 = 0.0;
        let mut S1: f64 = 0.0;
        let mut S2: f64 = 0.0;
        let mut S3: f64 = 0.0;
        let mut S4: f64 = 0.0;
        let mut S5: f64 = 0.0;
        let mut S6: f64 = 0.0;
        let mut S7: f64 = 0.0;
        let mut SE: f64 = 0.0;
        let mut SE2: f64 = 0.0;
        let mut SE3: f64 = 0.0;
        let mut SEL: f64 = 0.0;
        let mut SES: f64 = 0.0;
        let mut SGH: f64 = 0.0;
        let mut SGH2: f64 = 0.0;
        let mut SGH3: f64 = 0.0;
        let mut SGH4: f64 = 0.0;
        let mut SGHL: f64 = 0.0;
        let mut SGHS: f64 = 0.0;
        let mut SH: f64 = 0.0;
        let mut SH2: f64 = 0.0;
        let mut SH3: f64 = 0.0;
        let mut SHL: f64 = 0.0;
        let mut SHS: f64 = 0.0;
        let mut SI: f64 = 0.0;
        let mut SI2: f64 = 0.0;
        let mut SI3: f64 = 0.0;
        let mut SIL: f64 = 0.0;
        let mut SINI2: f64 = 0.0;
        let mut SINIQ: f64 = 0.0;
        let mut SINIS: f64 = 0.0;
        let mut SINOK: f64 = 0.0;
        let mut SINOMO: f64 = 0.0;
        let mut SINQ: f64 = 0.0;
        let mut SINZF: f64 = 0.0;
        let mut SIS: f64 = 0.0;
        let mut SL: f64 = 0.0;
        let mut SL2: f64 = 0.0;
        let mut SL3: f64 = 0.0;
        let mut SL4: f64 = 0.0;
        let mut SLL: f64 = 0.0;
        let mut SLS: f64 = 0.0;
        let mut STEM: f64 = 0.0;
        let mut STEPN: f64 = 0.0;
        let mut STEPP: f64 = 0.0;
        let mut TEMP: f64 = 0.0;
        let mut TEMP1: f64 = 0.0;
        let mut THETA: f64 = 0.0;
        let mut THGR: f64 = 0.0;
        let mut X1: f64 = 0.0;
        let mut X2: f64 = 0.0;
        let mut X3: f64 = 0.0;
        let mut X4: f64 = 0.0;
        let mut X5: f64 = 0.0;
        let mut X6: f64 = 0.0;
        let mut X7: f64 = 0.0;
        let mut X8: f64 = 0.0;
        let mut XFACT: f64 = 0.0;
        let mut XGH2: f64 = 0.0;
        let mut XGH3: f64 = 0.0;
        let mut XGH4: f64 = 0.0;
        let mut XH2: f64 = 0.0;
        let mut XH3: f64 = 0.0;
        let mut XI2: f64 = 0.0;
        let mut XI3: f64 = 0.0;
        let mut XINCL: f64 = 0.0;
        let mut XL: f64 = 0.0;
        let mut XL2: f64 = 0.0;
        let mut XL3: f64 = 0.0;
        let mut XL4: f64 = 0.0;
        let mut XLAMO: f64 = 0.0;
        let mut XLDOT: f64 = 0.0;
        let mut XLI: f64 = 0.0;
        let mut XLS: f64 = 0.0;
        let mut XMAO: f64 = 0.0;
        let mut XMO: f64 = 0.0;
        let mut XNDDT: f64 = 0.0;
        let mut XNDOT: f64 = 0.0;
        let mut XNI: f64 = 0.0;
        let mut XNO2: f64 = 0.0;
        let mut XNODCE: f64 = 0.0;
        let mut XNODEO: f64 = 0.0;
        let mut XNOI: f64 = 0.0;
        let mut XNQ: f64 = 0.0;
        let mut XQNCL: f64 = 0.0;
        let mut Z1: f64 = 0.0;
        let mut Z11: f64 = 0.0;
        let mut Z12: f64 = 0.0;
        let mut Z13: f64 = 0.0;
        let mut Z2: f64 = 0.0;
        let mut Z21: f64 = 0.0;
        let mut Z22: f64 = 0.0;
        let mut Z23: f64 = 0.0;
        let mut Z3: f64 = 0.0;
        let mut Z31: f64 = 0.0;
        let mut Z32: f64 = 0.0;
        let mut Z33: f64 = 0.0;
        let mut ZCOSG: f64 = 0.0;
        let mut ZCOSGL: f64 = 0.0;
        let mut ZCOSH: f64 = 0.0;
        let mut ZCOSHL: f64 = 0.0;
        let mut ZCOSI: f64 = 0.0;
        let mut ZCOSIL: f64 = 0.0;
        let mut ZE: f64 = 0.0;
        let mut ZF: f64 = 0.0;
        let mut ZM: f64 = 0.0;
        let mut ZMOL: f64 = 0.0;
        let mut ZMOS: f64 = 0.0;
        let mut ZN: f64 = 0.0;
        let mut ZSING: f64 = 0.0;
        let mut ZSINGL: f64 = 0.0;
        let mut ZSINH: f64 = 0.0;
        let mut ZSINHL: f64 = 0.0;
        let mut ZSINI: f64 = 0.0;
        let mut ZSINIL: f64 = 0.0;
        let mut ZX: f64 = 0.0;
        let mut ZY: f64 = 0.0;
        let mut DEL = StackArray::<f64, 3>::new(1..=3);
        let mut DG = StackArray::<f64, 10>::new(1..=10);
        let mut SSX = StackArray::<f64, 5>::new(1..=5);
        let mut JDTDB: f64 = 0.0;
        let mut JDUT50: f64 = 0.0;
        let mut IRESFL: i32 = 0;
        let mut ISYNFL: i32 = 0;
        let mut CONT: bool = false;

        Self {
            A1,
            A10,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            AINV2,
            ALFDP,
            AQNV,
            ATIME,
            BETDP,
            BFACT,
            BSQ,
            C,
            CC,
            COSIQ,
            COSIS,
            COSOK,
            COSOMO,
            COSQ,
            COSQ2,
            CTEM,
            DAY,
            DELT,
            DS50,
            E3,
            EE2,
            EO,
            EOC,
            EQ,
            EQSQ,
            ET,
            F2,
            F220,
            F221,
            F3,
            F311,
            F321,
            F322,
            F330,
            F441,
            F442,
            F522,
            F523,
            F542,
            F543,
            FT,
            G200,
            G201,
            G211,
            G300,
            G310,
            G322,
            G410,
            G422,
            G520,
            G521,
            G532,
            G533,
            GAM,
            OMEGAO,
            OXNODE,
            PE,
            PE0,
            PGH,
            PGH0,
            PH,
            PH0,
            PINC,
            PINC0,
            PIX1,
            PIX2,
            PL,
            PL0,
            PREEP,
            RTEQSQ,
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            SE,
            SE2,
            SE3,
            SEL,
            SES,
            SGH,
            SGH2,
            SGH3,
            SGH4,
            SGHL,
            SGHS,
            SH,
            SH2,
            SH3,
            SHL,
            SHS,
            SI,
            SI2,
            SI3,
            SIL,
            SINI2,
            SINIQ,
            SINIS,
            SINOK,
            SINOMO,
            SINQ,
            SINZF,
            SIS,
            SL,
            SL2,
            SL3,
            SL4,
            SLL,
            SLS,
            STEM,
            STEPN,
            STEPP,
            TEMP,
            TEMP1,
            THETA,
            THGR,
            X1,
            X2,
            X3,
            X4,
            X5,
            X6,
            X7,
            X8,
            XFACT,
            XGH2,
            XGH3,
            XGH4,
            XH2,
            XH3,
            XI2,
            XI3,
            XINCL,
            XL,
            XL2,
            XL3,
            XL4,
            XLAMO,
            XLDOT,
            XLI,
            XLS,
            XMAO,
            XMO,
            XNDDT,
            XNDOT,
            XNI,
            XNO2,
            XNODCE,
            XNODEO,
            XNOI,
            XNQ,
            XQNCL,
            Z1,
            Z11,
            Z12,
            Z13,
            Z2,
            Z21,
            Z22,
            Z23,
            Z3,
            Z31,
            Z32,
            Z33,
            ZCOSG,
            ZCOSGL,
            ZCOSH,
            ZCOSHL,
            ZCOSI,
            ZCOSIL,
            ZE,
            ZF,
            ZM,
            ZMOL,
            ZMOS,
            ZN,
            ZSING,
            ZSINGL,
            ZSINH,
            ZSINHL,
            ZSINI,
            ZSINIL,
            ZX,
            ZY,
            DEL,
            DG,
            SSX,
            JDTDB,
            JDUT50,
            IRESFL,
            ISYNFL,
            CONT,
        }
    }
}

//$Procedure ZZNRDDP ( Shell for deep space entry points )
pub fn ZZNRDDP(
    AO: f64,
    ELEMS: &[f64],
    EM: f64,
    OMGASM: f64,
    OMGDOT: f64,
    T: f64,
    XINC: f64,
    XLL: f64,
    XLLDOT: f64,
    XN: f64,
    XNODES: f64,
    XNODOT: f64,
    XNODP: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Local variables
    //

    //
    // SPICELIB functions
    //

    //
    // Define rather a large number of local parameters.
    //

    //
    // Save everything just to be sure.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZNRDDP", ctx)?;
    }

    //
    // This routine should never be called. If this routine is called,
    // an error is signaled.
    //
    SETMSG(b"NRDDP: You called an entry which performs no run-time function. This may indicate a bug. Please check the documentation for the subroutine ZZNRDDP.", ctx);

    SIGERR(b"SPICE(EVILBOGUSENTRY)", ctx)?;

    CHKOUT(b"ZZNRDDP", ctx)?;

    Ok(())
}

//$Procedure ZZDPINIT (Initialize deep space algorithm and variables )
pub fn ZZDPINIT(
    AO: f64,
    XLLDOT: f64,
    OMGDOT: f64,
    XNODOT: f64,
    XNODP: f64,
    ELEMS: &[f64],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let ELEMS = DummyArray::new(ELEMS, 1..=10);

    save.PIX1 = PI(ctx);
    save.PIX2 = TWOPI(ctx);

    //
    // Unpack the elements array.
    //
    save.XINCL = ELEMS[4];
    save.XNODEO = ELEMS[5];
    save.EO = ELEMS[6];
    save.OMEGAO = ELEMS[7];
    save.XMO = ELEMS[8];

    //
    // Calculate intermediate values
    //
    save.EQSQ = f64::powi(save.EO, 2);
    save.BSQ = (ONE - save.EQSQ);
    save.RTEQSQ = f64::sqrt(save.BSQ);
    save.SINIQ = f64::sin(save.XINCL);
    save.COSIQ = f64::cos(save.XINCL);
    save.COSQ2 = f64::powi(save.COSIQ, 2);
    save.SINOMO = f64::sin(save.OMEGAO);
    save.COSOMO = f64::cos(save.OMEGAO);

    //
    // This section of code was previously performed by the THETAG
    // function.  The epoch of the elements is defined in seconds since
    // J2000.  It is necessary to calculate the number of days which have
    // elapsed since the Jan 0.0 1950 reference date which is
    // Dec 31 1949 00:00:00 UTC ( J1950 - 1 ).  First extract the epoch
    // from the ELEMS array and place it in the first entry of a working
    // array.
    //
    save.ET = ELEMS[10];

    //
    // Convert the ET seconds past the J2000 epoch to the Julian
    // date TDB.
    //
    save.JDTDB = (J2000() + (save.ET / SPD()));

    //
    // How many days since the 1950 reference? Using SPICE standard
    // leapseconds the difference between TDB and UTC in 1950 is 32.184
    // seconds.  So we compute JDTDB corresponding to the UTC 1950
    // epoch.  We call this JDTDB epoch ---JDUT50. Then we get the days
    // since 1950 by simple arithmetic.
    //
    save.JDUT50 = ((J1950() - ONE) + (32.184 / SPD()));
    save.DS50 = (save.JDTDB - save.JDUT50);

    //
    // What is the Earth's right ascension of the epoch?  We know the
    // value at the JD1950-1 reference date, so add the number of radians
    // the Earth has rotated through since then.  MOD this value with
    // 2*PI to get the right ascension for the epoch.  This technique may
    // not be the best way to get this value.
    //
    save.THETA = (1.72944494 + (6.3003880987 * save.DS50));
    save.THGR = intrinsics::DMOD(save.THETA, save.PIX2);

    //
    // THGR should have a domain between 0 and 2 Pi.
    //
    if (save.THGR < ZERO) {
        save.THGR = (save.THGR + save.PIX2);
    }

    //
    // Set some operation variables.
    //
    save.EQ = save.EO;
    save.XNQ = XNODP;
    save.AQNV = (ONE / AO);
    save.XQNCL = save.XINCL;
    save.XMAO = save.XMO;
    save.SINQ = f64::sin(save.XNODEO);
    save.COSQ = f64::cos(save.XNODEO);

    //
    // Initialize lunar solar terms
    //

    save.DAY = (save.DS50 + 18261.5);

    if (save.DAY != save.PREEP) {
        save.PREEP = save.DAY;
        save.XNODCE = (4.523602 - (0.00092422029 * save.DAY));
        save.STEM = f64::sin(save.XNODCE);
        save.CTEM = f64::cos(save.XNODCE);
        save.ZCOSIL = (0.91375164 - (0.03568096 * save.CTEM));
        save.ZSINIL = f64::sqrt((ONE - f64::powi(save.ZCOSIL, 2)));
        save.ZSINHL = ((0.089683511 * save.STEM) / save.ZSINIL);
        save.ZCOSHL = f64::sqrt((ONE - f64::powi(save.ZSINHL, 2)));
        save.C = (4.7199672 + (0.2299715 * save.DAY));
        save.GAM = (5.8351514 + (0.001944368 * save.DAY));
        save.ZMOL = intrinsics::DMOD((save.C - save.GAM), save.PIX2);

        if (save.ZMOL < ZERO) {
            save.ZMOL = (save.ZMOL + save.PIX2);
        }

        save.ZX = ((0.39785416 * save.STEM) / save.ZSINIL);
        save.ZY = ((save.ZCOSHL * save.CTEM) + ((0.91744867 * save.ZSINHL) * save.STEM));

        //
        // Compute the angle from the x-axis of the point
        //
        if ((save.ZX != ZERO) || (save.ZY != ZERO)) {
            save.ZX = f64::atan2(save.ZX, save.ZY);

            if (save.ZX < ZERO) {
                save.ZX = (save.ZX + save.PIX2);
            }
        } else {
            save.ZX = ZERO;
        }

        save.ZX = ((save.GAM + save.ZX) - save.XNODCE);
        save.ZCOSGL = f64::cos(save.ZX);
        save.ZSINGL = f64::sin(save.ZX);
        save.ZMOS = (6.2565837 + (0.017201977 * save.DAY));
        save.ZMOS = intrinsics::DMOD(save.ZMOS, save.PIX2);

        if (save.ZMOS < ZERO) {
            save.ZMOS = (save.ZMOS + save.PIX2);
        }
    }

    //
    // Do solar terms.  Start with the constant values.
    //

    save.ZCOSG = ZCOSGS;
    save.ZSING = ZSINGS;
    save.ZCOSI = ZCOSIS;
    save.ZSINI = ZSINIS;
    save.ZCOSH = save.COSQ;
    save.ZSINH = save.SINQ;
    save.CC = C1SS;
    save.ZN = ZNS;
    save.ZE = ZES;
    save.XNOI = (ONE / save.XNQ);

    //
    // Initialize solar and lunar terms.  The procedure will
    // first initialize just the solar, then the lunar, then
    // reinitialize the solar with the added lunar effect.
    //
    for I in 1..=2 {
        //
        // Solar.
        //
        save.A1 = ((save.ZCOSG * save.ZCOSH) + ((save.ZSING * save.ZCOSI) * save.ZSINH));
        save.A3 = (-(save.ZSING * save.ZCOSH) + ((save.ZCOSG * save.ZCOSI) * save.ZSINH));
        save.A7 = (-(save.ZCOSG * save.ZSINH) + ((save.ZSING * save.ZCOSI) * save.ZCOSH));
        save.A8 = (save.ZSING * save.ZSINI);
        save.A9 = ((save.ZSING * save.ZSINH) + ((save.ZCOSG * save.ZCOSI) * save.ZCOSH));
        save.A10 = (save.ZCOSG * save.ZSINI);
        save.A2 = ((save.COSIQ * save.A7) + (save.SINIQ * save.A8));
        save.A4 = ((save.COSIQ * save.A9) + (save.SINIQ * save.A10));
        save.A5 = (-(save.SINIQ * save.A7) + (save.COSIQ * save.A8));
        save.A6 = (-(save.SINIQ * save.A9) + (save.COSIQ * save.A10));

        save.X1 = ((save.A1 * save.COSOMO) + (save.A2 * save.SINOMO));
        save.X2 = ((save.A3 * save.COSOMO) + (save.A4 * save.SINOMO));
        save.X3 = (-(save.A1 * save.SINOMO) + (save.A2 * save.COSOMO));
        save.X4 = (-(save.A3 * save.SINOMO) + (save.A4 * save.COSOMO));
        save.X5 = (save.A5 * save.SINOMO);
        save.X6 = (save.A6 * save.SINOMO);
        save.X7 = (save.A5 * save.COSOMO);
        save.X8 = (save.A6 * save.COSOMO);

        save.Z31 = ((12.0 * f64::powi(save.X1, 2)) - (3.0 * f64::powi(save.X3, 2)));
        save.Z32 = (((24.0 * save.X1) * save.X2) - ((6.0 * save.X3) * save.X4));
        save.Z33 = ((12.0 * f64::powi(save.X2, 2)) - (3.0 * f64::powi(save.X4, 2)));

        save.Z1 =
            ((3.0 * (f64::powi(save.A1, 2) + f64::powi(save.A2, 2))) + (save.Z31 * save.EQSQ));
        save.Z2 = ((6.0 * ((save.A1 * save.A3) + (save.A2 * save.A4))) + (save.Z32 * save.EQSQ));
        save.Z3 =
            ((3.0 * (f64::powi(save.A3, 2) + f64::powi(save.A4, 2))) + (save.Z33 * save.EQSQ));

        save.Z11 = (-((6.0 * save.A1) * save.A5)
            + (save.EQSQ * (-((24.0 * save.X1) * save.X7) - ((6.0 * save.X3) * save.X5))));
        save.Z12 = (-(6.0 * ((save.A1 * save.A6) + (save.A3 * save.A5)))
            + (save.EQSQ
                * (-(24.0 * ((save.X2 * save.X7) + (save.X1 * save.X8)))
                    - (6.0 * ((save.X3 * save.X6) + (save.X4 * save.X5))))));
        save.Z13 = (-((6.0 * save.A3) * save.A6)
            + (save.EQSQ * (-((24.0 * save.X2) * save.X8) - ((6.0 * save.X4) * save.X6))));
        save.Z21 = (((6.0 * save.A2) * save.A5)
            + (save.EQSQ * (((24.0 * save.X1) * save.X5) - ((6.0 * save.X3) * save.X7))));
        save.Z22 = ((6.0 * ((save.A4 * save.A5) + (save.A2 * save.A6)))
            + (save.EQSQ
                * ((24.0 * ((save.X2 * save.X5) + (save.X1 * save.X6)))
                    - (6.0 * ((save.X4 * save.X7) + (save.X3 * save.X8))))));
        save.Z23 = (((6.0 * save.A4) * save.A6)
            + (save.EQSQ * (((24.0 * save.X2) * save.X6) - ((6.0 * save.X4) * save.X8))));

        save.Z1 = ((save.Z1 + save.Z1) + (save.BSQ * save.Z31));
        save.Z2 = ((save.Z2 + save.Z2) + (save.BSQ * save.Z32));
        save.Z3 = ((save.Z3 + save.Z3) + (save.BSQ * save.Z33));

        save.S3 = (save.CC * save.XNOI);
        save.S2 = -((0.5 * save.S3) / save.RTEQSQ);
        save.S4 = (save.S3 * save.RTEQSQ);
        save.S1 = -((15.0 * save.EQ) * save.S4);
        save.S5 = ((save.X1 * save.X3) + (save.X2 * save.X4));
        save.S6 = ((save.X2 * save.X3) + (save.X1 * save.X4));
        save.S7 = ((save.X2 * save.X4) - (save.X1 * save.X3));

        save.SE = ((save.S1 * save.ZN) * save.S5);
        save.SI = ((save.S2 * save.ZN) * (save.Z11 + save.Z13));
        save.SL = -((save.ZN * save.S3) * (((save.Z1 + save.Z3) - 14.0) - (6.0 * save.EQSQ)));
        save.SGH = ((save.S4 * save.ZN) * ((save.Z31 + save.Z33) - 6.0));
        save.SH = -((save.ZN * save.S2) * (save.Z21 + save.Z23));

        //
        // Check for, and adjust SH, at inclinations near 0 and 180 degs.
        //
        if ((save.XQNCL < 0.052359877) || (save.XQNCL > (PI(ctx) - 0.052359877))) {
            save.SH = ZERO;
        }

        //
        // Secondary check, J.I.C.
        //
        if (save.SINIQ == 0.0) {
            save.SH = ZERO;
        }

        save.EE2 = ((2.0 * save.S1) * save.S6);
        save.E3 = ((2.0 * save.S1) * save.S7);
        save.XI2 = ((2.0 * save.S2) * save.Z12);
        save.XI3 = ((2.0 * save.S2) * (save.Z13 - save.Z11));
        save.XL2 = -((2.0 * save.S3) * save.Z2);
        save.XL3 = -((2.0 * save.S3) * (save.Z3 - save.Z1));
        save.XL4 = -(((2.0 * save.S3) * (-21.0 - (9.0 * save.EQSQ))) * save.ZE);
        save.XGH2 = ((2.0 * save.S4) * save.Z32);
        save.XGH3 = ((2.0 * save.S4) * (save.Z33 - save.Z31));
        save.XGH4 = -((18.0 * save.S4) * save.ZE);
        save.XH2 = -((2.0 * save.S2) * save.Z22);
        save.XH3 = -((2.0 * save.S2) * (save.Z23 - save.Z21));

        if (I == LUNAR) {
            //
            // Do lunar terms after solar terms, but only once.
            //
            save.SSX[1] = save.SL;

            //
            // Prevent evaluation of 1/SINIQ for SH = 0.
            //
            if (save.SH == 0.0) {
                save.SSX[3] = 0.0;
            }
            if (save.SH != 0.0) {
                save.SSX[3] = (save.SH / save.SINIQ);
            }

            save.SSX[2] = (save.SGH - (save.COSIQ * save.SSX[3]));
            save.SSX[4] = save.SE;
            save.SSX[5] = save.SI;
            save.SE2 = save.EE2;
            save.SI2 = save.XI2;
            save.SL2 = save.XL2;

            save.SGH2 = save.XGH2;
            save.SH2 = save.XH2;
            save.SE3 = save.E3;
            save.SI3 = save.XI3;
            save.SL3 = save.XL3;

            save.SGH3 = save.XGH3;
            save.SH3 = save.XH3;
            save.SL4 = save.XL4;
            save.SGH4 = save.XGH4;

            save.ZCOSG = save.ZCOSGL;
            save.ZSING = save.ZSINGL;
            save.ZCOSI = save.ZCOSIL;
            save.ZSINI = save.ZSINIL;
            save.ZCOSH = ((save.ZCOSHL * save.COSQ) + (save.ZSINHL * save.SINQ));
            save.ZSINH = ((save.SINQ * save.ZCOSHL) - (save.COSQ * save.ZSINHL));

            save.ZN = ZNL;
            save.CC = C1L;
            save.ZE = ZEL;
        }
    }

    save.SSX[1] = (save.SSX[1] + save.SL);

    //
    // Prevent evaluation of 1/SINIQ for SH = 0.
    //
    if (save.SH == 0.0) {
        save.SSX[2] = (save.SSX[2] + save.SGH);
    }
    if (save.SH != 0.0) {
        save.SSX[2] = ((save.SSX[2] + save.SGH) - ((save.COSIQ / save.SINIQ) * save.SH));
    }

    if (save.SH == 0.0) {
        save.SSX[3] = save.SSX[3];
    }
    if (save.SH != 0.0) {
        save.SSX[3] = (save.SSX[3] + (save.SH / save.SINIQ));
    }

    save.SSX[4] = (save.SSX[4] + save.SE);
    save.SSX[5] = (save.SSX[5] + save.SI);

    //
    // Geopotential resonance initialization for 12 hour orbits
    //

    save.IRESFL = 0;
    save.ISYNFL = 0;

    if ((save.XNQ < 0.0052359877) && (save.XNQ > 0.0034906585)) {
        //
        // Synchronous resonance terms initialization
        //

        save.IRESFL = 1;
        save.ISYNFL = 1;
        save.G200 = (ONE + (save.EQSQ * (-2.5 + (0.8125 * save.EQSQ))));
        save.G310 = (ONE + (2.0 * save.EQSQ));
        save.G300 = (ONE + (save.EQSQ * (-6.0 + (6.60937 * save.EQSQ))));
        save.F220 = (0.75 * f64::powi((ONE + save.COSIQ), 2));
        save.F311 = ((((0.9375 * save.SINIQ) * save.SINIQ) * (ONE + (3.0 * save.COSIQ)))
            - (0.75 * (ONE + save.COSIQ)));
        save.F330 = (1.875 * f64::powi((ONE + save.COSIQ), 3));
        save.DEL[1] = ((3.0 * f64::powi(save.XNQ, 2)) * f64::powi(save.AQNV, 2));
        save.DEL[2] = ((((2.0 * save.DEL[1]) * save.F220) * save.G200) * Q22);
        save.DEL[3] = (((((3.0 * save.DEL[1]) * save.F330) * save.G300) * Q33) * save.AQNV);
        save.DEL[1] = ((((save.DEL[1] * save.F311) * save.G310) * Q31) * save.AQNV);
        save.XLAMO = (((save.XMAO + save.XNODEO) + save.OMEGAO) - save.THGR);
        save.BFACT = (((XLLDOT + OMGDOT) + XNODOT) - THDT);
        save.BFACT = (((save.BFACT + save.SSX[1]) + save.SSX[2]) + save.SSX[3]);
    } else {
        if (((save.XNQ < 0.00826) || (save.XNQ > 0.00924)) || (save.EQ < 0.5)) {
            return;
        }

        save.IRESFL = 1;
        save.EOC = (save.EQ * save.EQSQ);
        save.G201 = (-0.306 - ((save.EQ - 0.64) * 0.44));

        //
        // Looks icky doesn't it?
        //

        if (save.EQ > 0.65) {
            save.G211 =
                (((-72.099 + (331.819 * save.EQ)) - (508.738 * save.EQSQ)) + (266.724 * save.EOC));
            save.G310 = (((-346.844 + (1582.851 * save.EQ)) - (2415.925 * save.EQSQ))
                + (1246.113 * save.EOC));
            save.G322 = (((-342.585 + (1554.908 * save.EQ)) - (2366.899 * save.EQSQ))
                + (1215.972 * save.EOC));
            save.G410 = (((-1052.797 + (4758.686 * save.EQ)) - (7193.992 * save.EQSQ))
                + (3651.957 * save.EOC));
            save.G422 = (((-3581.69 + (16178.11 * save.EQ)) - (24462.77 * save.EQSQ))
                + (12422.52 * save.EOC));

            //
            // Decide on the G520 coefficient.
            //
            if (save.EQ > 0.715) {
                save.G520 = (((-5149.66 + (29936.92 * save.EQ)) - (54087.36 * save.EQSQ))
                    + (31324.56 * save.EOC));
            } else {
                save.G520 = ((1464.74 - (4664.75 * save.EQ)) + (3763.64 * save.EQSQ));
            }
        } else {
            save.G211 = ((3.616 - (13.247 * save.EQ)) + (16.29 * save.EQSQ));
            save.G310 =
                (((-19.302 + (117.39 * save.EQ)) - (228.419 * save.EQSQ)) + (156.591 * save.EOC));
            save.G322 = (((-18.9068 + (109.7927 * save.EQ)) - (214.6334 * save.EQSQ))
                + (146.5816 * save.EOC));
            save.G410 =
                (((-41.122 + (242.694 * save.EQ)) - (471.094 * save.EQSQ)) + (313.953 * save.EOC));
            save.G422 = (((-146.407 + (841.88 * save.EQ)) - (1629.014 * save.EQSQ))
                + (1083.435 * save.EOC));
            save.G520 = (((-532.114 + (3017.977 * save.EQ)) - (5740.0 * save.EQSQ))
                + (3708.276 * save.EOC));
        }

        if (save.EQ >= 0.7) {
            save.G533 = (((-37995.78 + (161616.52 * save.EQ)) - (229838.2 * save.EQSQ))
                + (109377.94 * save.EOC));
            save.G521 = (((-51752.104 + (218913.95 * save.EQ)) - (309468.16 * save.EQSQ))
                + (146349.42 * save.EOC));
            save.G532 = (((-40023.88 + (170470.89 * save.EQ)) - (242699.48 * save.EQSQ))
                + (115605.82 * save.EOC));
        } else {
            save.G533 = (((-919.2277 + (4988.61 * save.EQ)) - (9064.77 * save.EQSQ))
                + (5542.21 * save.EOC));
            save.G521 = (((-822.71072 + (4568.6173 * save.EQ)) - (8491.4146 * save.EQSQ))
                + (5337.524 * save.EOC));
            save.G532 =
                (((-853.666 + (4690.25 * save.EQ)) - (8624.77 * save.EQSQ)) + (5341.4 * save.EOC));
        }

        save.SINI2 = (save.SINIQ * save.SINIQ);

        save.F220 = (0.75 * ((ONE + (2.0 * save.COSIQ)) + save.COSQ2));
        save.F221 = (1.5 * save.SINI2);
        save.F321 = ((1.875 * save.SINIQ) * ((ONE - (2.0 * save.COSIQ)) - (3.0 * save.COSQ2)));
        save.F322 = -((1.875 * save.SINIQ) * ((ONE + (2.0 * save.COSIQ)) - (3.0 * save.COSQ2)));
        save.F441 = ((35.0 * save.SINI2) * save.F220);
        save.F442 = ((39.375 * save.SINI2) * save.SINI2);

        save.F522 = ((9.84375 * save.SINIQ)
            * ((save.SINI2 * ((ONE - (2.0 * save.COSIQ)) - (5.0 * save.COSQ2)))
                + (0.33333333 * ((-2.0 + (4.0 * save.COSIQ)) + (6.0 * save.COSQ2)))));

        save.F523 = (save.SINIQ
            * (((4.92187512 * save.SINI2) * ((-2.0 - (4.0 * save.COSIQ)) + (10.0 * save.COSQ2)))
                + (6.56250012 * ((ONE + (2.0 * save.COSIQ)) - (3.0 * save.COSQ2)))));

        save.F542 = ((29.53125 * save.SINIQ)
            * ((2.0 - (8.0 * save.COSIQ))
                + (save.COSQ2 * ((-12.0 + (8.0 * save.COSIQ)) + (10.0 * save.COSQ2)))));

        save.F543 = ((29.53125 * save.SINIQ)
            * ((-2.0 - (8.0 * save.COSIQ))
                + (save.COSQ2 * ((12.0 + (8.0 * save.COSIQ)) - (10.0 * save.COSQ2)))));

        save.XNO2 = (save.XNQ * save.XNQ);
        save.AINV2 = (save.AQNV * save.AQNV);
        save.TEMP1 = ((3.0 * save.XNO2) * save.AINV2);
        save.TEMP = (save.TEMP1 * ROOT22);
        save.DG[1] = ((save.TEMP * save.F220) * save.G201);
        save.DG[2] = ((save.TEMP * save.F221) * save.G211);
        save.TEMP1 = (save.TEMP1 * save.AQNV);
        save.TEMP = (save.TEMP1 * ROOT32);
        save.DG[3] = ((save.TEMP * save.F321) * save.G310);
        save.DG[4] = ((save.TEMP * save.F322) * save.G322);
        save.TEMP1 = (save.TEMP1 * save.AQNV);
        save.TEMP = ((2.0 * save.TEMP1) * ROOT44);
        save.DG[5] = ((save.TEMP * save.F441) * save.G410);
        save.DG[6] = ((save.TEMP * save.F442) * save.G422);
        save.TEMP1 = (save.TEMP1 * save.AQNV);
        save.TEMP = (save.TEMP1 * ROOT52);
        save.DG[7] = ((save.TEMP * save.F522) * save.G520);
        save.DG[8] = ((save.TEMP * save.F523) * save.G532);
        save.TEMP = ((2.0 * save.TEMP1) * ROOT54);
        save.DG[9] = ((save.TEMP * save.F542) * save.G521);
        save.DG[10] = ((save.TEMP * save.F543) * save.G533);

        save.XLAMO = ((((save.XMAO + save.XNODEO) + save.XNODEO) - save.THGR) - save.THGR);
        save.BFACT = ((((XLLDOT + XNODOT) + XNODOT) - THDT) - THDT);
        save.BFACT = (((save.BFACT + save.SSX[1]) + save.SSX[3]) + save.SSX[3]);
    }

    save.XFACT = (save.BFACT - save.XNQ);

    //
    // Initialize integrator
    //
    save.XLI = save.XLAMO;
    save.XNI = save.XNQ;
    save.ATIME = ZERO;
}

//$Procedure ZZDPSEC (Calculate secular perturbations )
pub fn ZZDPSEC(
    XLL: &mut f64,
    OMGASM: &mut f64,
    XNODES: &mut f64,
    EM: &mut f64,
    XINC: &mut f64,
    XN: &mut f64,
    T: f64,
    ELEMS: &[f64],
    OMGDOT: f64,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let ELEMS = DummyArray::new(ELEMS, 1..=10);

    save.STEPP = 720.0;
    save.STEPN = -720.0;

    save.XINCL = ELEMS[4];
    save.EO = ELEMS[6];
    *XLL = (*XLL + (save.SSX[1] * T));
    *OMGASM = (*OMGASM + (save.SSX[2] * T));
    *XNODES = (*XNODES + (save.SSX[3] * T));
    *EM = (save.EO + (save.SSX[4] * T));
    *XINC = (save.XINCL + (save.SSX[5] * T));

    //
    // Check for the state of the resonance flag.
    //
    if (save.IRESFL == 0) {
        return;
    }

    //
    // If we got down here then the resonance effects need to be
    // calculated.  Continue to loop until the CONT flag is set to false.
    //
    save.CONT = true;

    while save.CONT {
        if (((save.ATIME == ZERO) || ((T >= ZERO) && (save.ATIME < ZERO)))
            || ((T < ZERO) && (save.ATIME >= ZERO)))
        {
            //
            // Epoch restart
            //
            if (T >= ZERO) {
                save.DELT = save.STEPP;
            } else {
                save.DELT = save.STEPN;
            }

            save.ATIME = ZERO;
            save.XNI = save.XNQ;
            save.XLI = save.XLAMO;

            save.CONT = false;
        } else if (f64::abs(T) >= f64::abs(save.ATIME)) {
            save.DELT = save.STEPN;

            if (T > ZERO) {
                save.DELT = save.STEPP;
            }

            save.CONT = false;
        } else {
            save.DELT = save.STEPP;

            if (T >= ZERO) {
                save.DELT = save.STEPN;
            }

            ZZSECPRT(
                save.ISYNFL,
                save.DG.as_slice(),
                save.DEL.as_slice(),
                save.XNI,
                save.OMEGAO,
                save.ATIME,
                OMGDOT,
                save.XLI,
                save.XFACT,
                &mut save.XLDOT,
                &mut save.XNDOT,
                &mut save.XNDDT,
            );

            save.XLI = ((save.XLI + (save.XLDOT * save.DELT)) + (save.XNDOT * STEP2));
            save.XNI = ((save.XNI + (save.XNDOT * save.DELT)) + (save.XNDDT * STEP2));
            save.ATIME = (save.ATIME + save.DELT);

            save.CONT = true;
        }
    }

    //
    // Do this loop while the time interval is greater than STEPP
    //

    while (f64::abs((T - save.ATIME)) >= save.STEPP) {
        ZZSECPRT(
            save.ISYNFL,
            save.DG.as_slice(),
            save.DEL.as_slice(),
            save.XNI,
            save.OMEGAO,
            save.ATIME,
            OMGDOT,
            save.XLI,
            save.XFACT,
            &mut save.XLDOT,
            &mut save.XNDOT,
            &mut save.XNDDT,
        );

        save.XLI = ((save.XLI + (save.XLDOT * save.DELT)) + (save.XNDOT * STEP2));
        save.XNI = ((save.XNI + (save.XNDOT * save.DELT)) + (save.XNDDT * STEP2));
        save.ATIME = (save.ATIME + save.DELT);
    }

    //
    // Calculate the time interval and determine the secular
    // perturbations
    //

    save.FT = (T - save.ATIME);

    ZZSECPRT(
        save.ISYNFL,
        save.DG.as_slice(),
        save.DEL.as_slice(),
        save.XNI,
        save.OMEGAO,
        save.ATIME,
        OMGDOT,
        save.XLI,
        save.XFACT,
        &mut save.XLDOT,
        &mut save.XNDOT,
        &mut save.XNDDT,
    );

    *XN = ((save.XNI + (save.XNDOT * save.FT)) + (((save.XNDDT * save.FT) * save.FT) * 0.5));
    save.XL = ((save.XLI + (save.XLDOT * save.FT)) + (((save.XNDOT * save.FT) * save.FT) * 0.5));
    save.TEMP = ((-*XNODES + save.THGR) + (T * THDT));
    *XLL = ((save.XL - *OMGASM) + save.TEMP);

    if (save.ISYNFL == 0) {
        *XLL = ((save.XL + save.TEMP) + save.TEMP);
    }
}

//$Procedure ZZDPPER ( Calculate periodic perturbations )
pub fn ZZDPPER(
    T: f64,
    EM: &mut f64,
    XINC: &mut f64,
    OMGASM: &mut f64,
    XNODES: &mut f64,
    XLL: &mut f64,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Time varying periodic terms.
    //

    //
    // Update for solar perts at time T.
    //
    save.ZM = (save.ZMOS + (ZNS * T));
    save.ZF = (save.ZM + ((2.0 * ZES) * f64::sin(save.ZM)));
    save.SINZF = f64::sin(save.ZF);
    save.F2 = (((0.5 * save.SINZF) * save.SINZF) - 0.25);
    save.F3 = -((0.5 * save.SINZF) * f64::cos(save.ZF));
    save.SES = ((save.SE2 * save.F2) + (save.SE3 * save.F3));
    save.SIS = ((save.SI2 * save.F2) + (save.SI3 * save.F3));
    save.SLS = (((save.SL2 * save.F2) + (save.SL3 * save.F3)) + (save.SL4 * save.SINZF));
    save.SGHS = (((save.SGH2 * save.F2) + (save.SGH3 * save.F3)) + (save.SGH4 * save.SINZF));
    save.SHS = ((save.SH2 * save.F2) + (save.SH3 * save.F3));

    //
    // Update for lunar perts at time T.
    //
    save.ZM = (save.ZMOL + (ZNL * T));
    save.ZF = (save.ZM + ((2.0 * ZEL) * f64::sin(save.ZM)));
    save.SINZF = f64::sin(save.ZF);
    save.F2 = (((0.5 * save.SINZF) * save.SINZF) - 0.25);
    save.F3 = -((0.5 * save.SINZF) * f64::cos(save.ZF));
    save.SEL = ((save.EE2 * save.F2) + (save.E3 * save.F3));
    save.SIL = ((save.XI2 * save.F2) + (save.XI3 * save.F3));
    save.SLL = (((save.XL2 * save.F2) + (save.XL3 * save.F3)) + (save.XL4 * save.SINZF));
    save.SGHL = (((save.XGH2 * save.F2) + (save.XGH3 * save.F3)) + (save.XGH4 * save.SINZF));
    save.SHL = ((save.XH2 * save.F2) + (save.XH3 * save.F3));

    //
    // Sum of solar and lunar perts
    //
    save.PE = (save.SES + save.SEL);
    save.PINC = (save.SIS + save.SIL);
    save.PL = (save.SLS + save.SLL);
    save.PGH = (save.SGHS + save.SGHL);
    save.PH = (save.SHS + save.SHL);

    //
    // Subtract the epoch perturbations off the calculated values.
    //
    save.PE = (save.PE - save.PE0);
    save.PINC = (save.PINC - save.PINC0);
    save.PL = (save.PL - save.PL0);
    save.PGH = (save.PGH - save.PGH0);
    save.PH = (save.PH - save.PH0);

    *XINC = (*XINC + save.PINC);
    *EM = (*EM + save.PE);

    //
    // Sin and Cos of the perturbed inclination.  The original
    // Spacetrack 3 report calculated the values before the
    // perturbation.
    //
    save.SINIS = f64::sin(*XINC);
    save.COSIS = f64::cos(*XINC);

    if (save.XQNCL > 0.2) {
        save.PH = (save.PH / save.SINIQ);
        save.PGH = (save.PGH - (save.COSIQ * save.PH));
        *OMGASM = (*OMGASM + save.PGH);
        *XNODES = (*XNODES + save.PH);
        *XLL = (*XLL + save.PL);
    } else {
        //
        // Apply periodics with Lyddane modification
        //
        save.SINOK = f64::sin(*XNODES);
        save.COSOK = f64::cos(*XNODES);
        save.ALFDP = (save.SINIS * save.SINOK);
        save.BETDP = (save.SINIS * save.COSOK);
        save.ALFDP =
            ((save.ALFDP + (save.PH * save.COSOK)) + ((save.PINC * save.COSIS) * save.SINOK));
        save.BETDP =
            ((save.BETDP - (save.PH * save.SINOK)) + ((save.PINC * save.COSIS) * save.COSOK));

        //
        // Force a 0 - 2Pi domain on XNODES.
        //
        if (*XNODES < ZERO) {
            *XNODES = (*XNODES + save.PIX2);
        }

        save.XLS = (((((*XLL + *OMGASM) + save.PL) + save.PGH) + (save.COSIS * *XNODES))
            - ((save.SINIS * *XNODES) * save.PINC));

        //
        // Compute the angle from the x-axis of the point
        //
        if ((save.ALFDP != ZERO) || (save.BETDP != ZERO)) {
            //
            // Save the old value of XNODES, then compute the current value
            // From ALFDP and BETDP
            //
            save.OXNODE = *XNODES;
            *XNODES = f64::atan2(save.ALFDP, save.BETDP);

            //
            // Force a 0 - 2Pi domain on XNODES
            //
            if (*XNODES < ZERO) {
                *XNODES = (*XNODES + save.PIX2);
            }

            //
            // XNODES should be the angular difference between the previous
            // value of XNODES and that just calculated.  This is a
            // correction to the standard SDP4 routine which did not
            // calculate this term correctly if XNODES passes over the
            // branch cut at 2*Pi.
            //
            if (f64::abs((*XNODES - save.OXNODE)) > save.PIX1) {
                if (*XNODES > save.OXNODE) {
                    *XNODES = (*XNODES - save.PIX2);
                } else {
                    *XNODES = (*XNODES + save.PIX2);
                }
            }
        } else {
            *XNODES = ZERO;
        }

        *XLL = (*XLL + save.PL);
        *OMGASM = ((save.XLS - *XLL) - (*XNODES * f64::cos(*XINC)));
    }
}
