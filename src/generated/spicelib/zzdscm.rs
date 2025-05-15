//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZDSCM ( SGP4 deep space common calculations )
pub fn ZZDSCM(
    EPOCH: f64,
    ECCP: f64,
    ARGPP: f64,
    TC: f64,
    INCLP: f64,
    NODEP: f64,
    NP: f64,
    SNODM: &mut f64,
    CNODM: &mut f64,
    SINIM: &mut f64,
    COSIM: &mut f64,
    SINOMM: &mut f64,
    COSOMM: &mut f64,
    DAY: &mut f64,
    E3: &mut f64,
    EE2: &mut f64,
    ECCM: &mut f64,
    EMSQ: &mut f64,
    GAM: &mut f64,
    PEO: &mut f64,
    PGHO: &mut f64,
    PHO: &mut f64,
    PINCO: &mut f64,
    PLO: &mut f64,
    RTEMSQ: &mut f64,
    SE2: &mut f64,
    SE3: &mut f64,
    SGH2: &mut f64,
    SGH3: &mut f64,
    SGH4: &mut f64,
    SH2: &mut f64,
    SH3: &mut f64,
    SI2: &mut f64,
    SI3: &mut f64,
    SL2: &mut f64,
    SL3: &mut f64,
    SL4: &mut f64,
    S1: &mut f64,
    S2: &mut f64,
    S3: &mut f64,
    S4: &mut f64,
    S5: &mut f64,
    S6: &mut f64,
    S7: &mut f64,
    SS1: &mut f64,
    SS2: &mut f64,
    SS3: &mut f64,
    SS4: &mut f64,
    SS5: &mut f64,
    SS6: &mut f64,
    SS7: &mut f64,
    SZ1: &mut f64,
    SZ2: &mut f64,
    SZ3: &mut f64,
    SZ11: &mut f64,
    SZ12: &mut f64,
    SZ13: &mut f64,
    SZ21: &mut f64,
    SZ22: &mut f64,
    SZ23: &mut f64,
    SZ31: &mut f64,
    SZ32: &mut f64,
    SZ33: &mut f64,
    XGH2: &mut f64,
    XGH3: &mut f64,
    XGH4: &mut f64,
    XH2: &mut f64,
    XH3: &mut f64,
    XI2: &mut f64,
    XI3: &mut f64,
    XL2: &mut f64,
    XL3: &mut f64,
    XL4: &mut f64,
    XN: &mut f64,
    Z1: &mut f64,
    Z2: &mut f64,
    Z3: &mut f64,
    Z11: &mut f64,
    Z12: &mut f64,
    Z13: &mut f64,
    Z21: &mut f64,
    Z22: &mut f64,
    Z23: &mut f64,
    Z31: &mut f64,
    Z32: &mut f64,
    Z33: &mut f64,
    ZMOL: &mut f64,
    ZMOS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ZES: f64 = 0.0;
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
    let mut BETASQ: f64 = 0.0;
    let mut C1L: f64 = 0.0;
    let mut C1SS: f64 = 0.0;
    let mut CC: f64 = 0.0;
    let mut CTEM: f64 = 0.0;
    let mut STEM: f64 = 0.0;
    let mut X1: f64 = 0.0;
    let mut X2: f64 = 0.0;
    let mut X3: f64 = 0.0;
    let mut X4: f64 = 0.0;
    let mut X5: f64 = 0.0;
    let mut X6: f64 = 0.0;
    let mut X7: f64 = 0.0;
    let mut X8: f64 = 0.0;
    let mut XNODCE: f64 = 0.0;
    let mut XNOI: f64 = 0.0;
    let mut ZCOSG: f64 = 0.0;
    let mut ZCOSGL: f64 = 0.0;
    let mut ZCOSGS: f64 = 0.0;
    let mut ZCOSH: f64 = 0.0;
    let mut ZCOSHL: f64 = 0.0;
    let mut ZCOSI: f64 = 0.0;
    let mut ZCOSIL: f64 = 0.0;
    let mut ZCOSIS: f64 = 0.0;
    let mut ZEL: f64 = 0.0;
    let mut ZSING: f64 = 0.0;
    let mut ZSINGL: f64 = 0.0;
    let mut ZSINGS: f64 = 0.0;
    let mut ZSINH: f64 = 0.0;
    let mut ZSINHL: f64 = 0.0;
    let mut ZSINI: f64 = 0.0;
    let mut ZSINIL: f64 = 0.0;
    let mut ZSINIS: f64 = 0.0;
    let mut ZX: f64 = 0.0;
    let mut ZY: f64 = 0.0;

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

    CHKIN(b"ZZDSCM", ctx)?;

    //
    // Constants
    //
    ZES = 0.01675;
    ZEL = 0.0549;
    C1SS = 0.0000029864797;
    C1L = 0.00000047968065;
    ZSINIS = 0.39785416;
    ZCOSIS = 0.91744867;
    ZCOSGS = 0.1945905;
    ZSINGS = -0.98088458;

    //
    // DEEP SPACE PERIODICS INITIALIZATION
    //
    *XN = NP;
    *ECCM = ECCP;
    *SNODM = f64::sin(NODEP);
    *CNODM = f64::cos(NODEP);
    *SINOMM = f64::sin(ARGPP);
    *COSOMM = f64::cos(ARGPP);
    *SINIM = f64::sin(INCLP);
    *COSIM = f64::cos(INCLP);
    *EMSQ = (*ECCM * *ECCM);
    BETASQ = (1.0 - *EMSQ);
    *RTEMSQ = f64::sqrt(BETASQ);

    //
    // INITIALIZE LUNAR SOLAR TERMS
    //
    // Note, EPOCH + 18261.5D0 corresponds to JD days since
    // 1899-12-31 12:00:00.
    //
    *PEO = 0.0;
    *PINCO = 0.0;
    *PLO = 0.0;
    *PGHO = 0.0;
    *PHO = 0.0;
    *DAY = ((EPOCH + 18261.5) + (TC / 1440.0));
    XNODCE = intrinsics::DMOD((4.523602 - (0.00092422029 * *DAY)), TWOPI(ctx));
    STEM = f64::sin(XNODCE);
    CTEM = f64::cos(XNODCE);
    ZCOSIL = (0.91375164 - (0.03568096 * CTEM));
    ZSINIL = f64::sqrt((1.0 - (ZCOSIL * ZCOSIL)));
    ZSINHL = ((0.089683511 * STEM) / ZSINIL);
    ZCOSHL = f64::sqrt((1.0 - (ZSINHL * ZSINHL)));
    *GAM = (5.8351514 + (0.001944368 * *DAY));
    ZX = ((0.39785416 * STEM) / ZSINIL);
    ZY = ((ZCOSHL * CTEM) + ((0.91744867 * ZSINHL) * STEM));
    ZX = f64::atan2(ZX, ZY);
    ZX = ((*GAM + ZX) - XNODCE);
    ZCOSGL = f64::cos(ZX);
    ZSINGL = f64::sin(ZX);

    //
    // DO SOLAR TERMS
    //
    ZCOSG = ZCOSGS;
    ZSING = ZSINGS;
    ZCOSI = ZCOSIS;
    ZSINI = ZSINIS;
    ZCOSH = *CNODM;
    ZSINH = *SNODM;
    CC = C1SS;
    XNOI = (1.0 / *XN);

    //
    // Loop over the lunar and solar term flags.
    //
    for LSFLG in 1..=2 {
        A1 = ((ZCOSG * ZCOSH) + ((ZSING * ZCOSI) * ZSINH));
        A3 = (-(ZSING * ZCOSH) + ((ZCOSG * ZCOSI) * ZSINH));
        A7 = (-(ZCOSG * ZSINH) + ((ZSING * ZCOSI) * ZCOSH));
        A8 = (ZSING * ZSINI);
        A9 = ((ZSING * ZSINH) + ((ZCOSG * ZCOSI) * ZCOSH));
        A10 = (ZCOSG * ZSINI);
        A2 = ((*COSIM * A7) + (*SINIM * A8));
        A4 = ((*COSIM * A9) + (*SINIM * A10));
        A5 = (-(*SINIM * A7) + (*COSIM * A8));
        A6 = (-(*SINIM * A9) + (*COSIM * A10));

        X1 = ((A1 * *COSOMM) + (A2 * *SINOMM));
        X2 = ((A3 * *COSOMM) + (A4 * *SINOMM));
        X3 = (-(A1 * *SINOMM) + (A2 * *COSOMM));
        X4 = (-(A3 * *SINOMM) + (A4 * *COSOMM));
        X5 = (A5 * *SINOMM);
        X6 = (A6 * *SINOMM);
        X7 = (A5 * *COSOMM);
        X8 = (A6 * *COSOMM);

        *Z31 = (((12.0 * X1) * X1) - ((3.0 * X3) * X3));
        *Z32 = (((24.0 * X1) * X2) - ((6.0 * X3) * X4));
        *Z33 = (((12.0 * X2) * X2) - ((3.0 * X4) * X4));
        *Z1 = ((3.0 * ((A1 * A1) + (A2 * A2))) + (*Z31 * *EMSQ));
        *Z2 = ((6.0 * ((A1 * A3) + (A2 * A4))) + (*Z32 * *EMSQ));
        *Z3 = ((3.0 * ((A3 * A3) + (A4 * A4))) + (*Z33 * *EMSQ));
        *Z11 = (-((6.0 * A1) * A5) + (*EMSQ * (-((24.0 * X1) * X7) - ((6.0 * X3) * X5))));
        *Z12 = (-(6.0 * ((A1 * A6) + (A3 * A5)))
            + (*EMSQ * (-(24.0 * ((X2 * X7) + (X1 * X8))) - (6.0 * ((X3 * X6) + (X4 * X5))))));
        *Z13 = (-((6.0 * A3) * A6) + (*EMSQ * (-((24.0 * X2) * X8) - ((6.0 * X4) * X6))));
        *Z21 = (((6.0 * A2) * A5) + (*EMSQ * (((24.0 * X1) * X5) - ((6.0 * X3) * X7))));
        *Z22 = ((6.0 * ((A4 * A5) + (A2 * A6)))
            + (*EMSQ * ((24.0 * ((X2 * X5) + (X1 * X6))) - (6.0 * ((X4 * X7) + (X3 * X8))))));
        *Z23 = (((6.0 * A4) * A6) + (*EMSQ * (((24.0 * X2) * X6) - ((6.0 * X4) * X8))));
        *Z1 = ((*Z1 + *Z1) + (BETASQ * *Z31));
        *Z2 = ((*Z2 + *Z2) + (BETASQ * *Z32));
        *Z3 = ((*Z3 + *Z3) + (BETASQ * *Z33));
        *S3 = (CC * XNOI);
        *S2 = -((0.5 * *S3) / *RTEMSQ);
        *S4 = (*S3 * *RTEMSQ);
        *S1 = -((15.0 * *ECCM) * *S4);
        *S5 = ((X1 * X3) + (X2 * X4));
        *S6 = ((X2 * X3) + (X1 * X4));
        *S7 = ((X2 * X4) - (X1 * X3));

        //
        // DO LUNAR TERMS
        //
        if (LSFLG == 1) {
            *SS1 = *S1;
            *SS2 = *S2;
            *SS3 = *S3;
            *SS4 = *S4;
            *SS5 = *S5;
            *SS6 = *S6;
            *SS7 = *S7;
            *SZ1 = *Z1;
            *SZ2 = *Z2;
            *SZ3 = *Z3;
            *SZ11 = *Z11;
            *SZ12 = *Z12;
            *SZ13 = *Z13;
            *SZ21 = *Z21;
            *SZ22 = *Z22;
            *SZ23 = *Z23;
            *SZ31 = *Z31;
            *SZ32 = *Z32;
            *SZ33 = *Z33;
            ZCOSG = ZCOSGL;
            ZSING = ZSINGL;
            ZCOSI = ZCOSIL;
            ZSINI = ZSINIL;
            ZCOSH = ((ZCOSHL * *CNODM) + (ZSINHL * *SNODM));
            ZSINH = ((*SNODM * ZCOSHL) - (*CNODM * ZSINHL));
            CC = C1L;
        }
    }

    *ZMOL = intrinsics::DMOD(((4.7199672 + (0.2299715 * *DAY)) - *GAM), TWOPI(ctx));
    *ZMOS = intrinsics::DMOD((6.2565837 + (0.017201977 * *DAY)), TWOPI(ctx));

    //
    // DO SOLAR TERMS
    //
    *SE2 = ((2.0 * *SS1) * *SS6);
    *SE3 = ((2.0 * *SS1) * *SS7);
    *SI2 = ((2.0 * *SS2) * *SZ12);
    *SI3 = ((2.0 * *SS2) * (*SZ13 - *SZ11));
    *SL2 = -((2.0 * *SS3) * *SZ2);
    *SL3 = -((2.0 * *SS3) * (*SZ3 - *SZ1));
    *SL4 = -(((2.0 * *SS3) * (-21.0 - (9.0 * *EMSQ))) * ZES);
    *SGH2 = ((2.0 * *SS4) * *SZ32);
    *SGH3 = ((2.0 * *SS4) * (*SZ33 - *SZ31));
    *SGH4 = -((18.0 * *SS4) * ZES);
    *SH2 = -((2.0 * *SS2) * *SZ22);
    *SH3 = -((2.0 * *SS2) * (*SZ23 - *SZ21));

    //
    // DO LUNAR TERMS
    //
    *EE2 = ((2.0 * *S1) * *S6);
    *E3 = ((2.0 * *S1) * *S7);
    *XI2 = ((2.0 * *S2) * *Z12);
    *XI3 = ((2.0 * *S2) * (*Z13 - *Z11));
    *XL2 = -((2.0 * *S3) * *Z2);
    *XL3 = -((2.0 * *S3) * (*Z3 - *Z1));
    *XL4 = -(((2.0 * *S3) * (-21.0 - (9.0 * *EMSQ))) * ZEL);
    *XGH2 = ((2.0 * *S4) * *Z32);
    *XGH3 = ((2.0 * *S4) * (*Z33 - *Z31));
    *XGH4 = -((18.0 * *S4) * ZEL);
    *XH2 = -((2.0 * *S2) * *Z22);
    *XH3 = -((2.0 * *S2) * (*Z23 - *Z21));

    CHKOUT(b"ZZDSCM", ctx)?;

    Ok(())
}
