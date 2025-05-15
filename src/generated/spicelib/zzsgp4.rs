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

struct SaveVars {
    A: f64,
    ALTA: f64,
    ALTP: f64,
    ARGPDOT: f64,
    AYCOF: f64,
    CC1: f64,
    CC4: f64,
    CC5: f64,
    CON41: f64,
    ETA: f64,
    INCLO: f64,
    MDOT: f64,
    NODEDOT: f64,
    OMGCOF: f64,
    SINMAO: f64,
    T2COF: f64,
    T3COF: f64,
    T4COF: f64,
    T5COF: f64,
    X1MTH2: f64,
    X7THM1: f64,
    XLCOF: f64,
    XMCOF: f64,
    XNODCF: f64,
    E3: f64,
    EE2: f64,
    PEO: f64,
    PGHO: f64,
    PHO: f64,
    PINCO: f64,
    PLO: f64,
    GSTO: f64,
    XFACT: f64,
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
    XLAMO: f64,
    ZMOL: f64,
    ZMOS: f64,
    ATIME: f64,
    XLI: f64,
    XNI: f64,
    ARGPO: f64,
    BSTAR: f64,
    D2: f64,
    D2201: f64,
    D2211: f64,
    D3: f64,
    D3210: f64,
    D3222: f64,
    D4: f64,
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
    DELMO: f64,
    DIDT: f64,
    DMDT: f64,
    DNODT: f64,
    DOMDT: f64,
    ECCO: f64,
    ER: f64,
    J2: f64,
    J3OJ2: f64,
    MO: f64,
    NO: f64,
    NODEO: f64,
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
    XKE: f64,
    SVMODE: i32,
    IREZ: i32,
    DOSIMP: bool,
    DODEEP: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut A: f64 = 0.0;
        let mut ALTA: f64 = 0.0;
        let mut ALTP: f64 = 0.0;
        let mut ARGPDOT: f64 = 0.0;
        let mut AYCOF: f64 = 0.0;
        let mut CC1: f64 = 0.0;
        let mut CC4: f64 = 0.0;
        let mut CC5: f64 = 0.0;
        let mut CON41: f64 = 0.0;
        let mut ETA: f64 = 0.0;
        let mut INCLO: f64 = 0.0;
        let mut MDOT: f64 = 0.0;
        let mut NODEDOT: f64 = 0.0;
        let mut OMGCOF: f64 = 0.0;
        let mut SINMAO: f64 = 0.0;
        let mut T2COF: f64 = 0.0;
        let mut T3COF: f64 = 0.0;
        let mut T4COF: f64 = 0.0;
        let mut T5COF: f64 = 0.0;
        let mut X1MTH2: f64 = 0.0;
        let mut X7THM1: f64 = 0.0;
        let mut XLCOF: f64 = 0.0;
        let mut XMCOF: f64 = 0.0;
        let mut XNODCF: f64 = 0.0;
        let mut E3: f64 = 0.0;
        let mut EE2: f64 = 0.0;
        let mut PEO: f64 = 0.0;
        let mut PGHO: f64 = 0.0;
        let mut PHO: f64 = 0.0;
        let mut PINCO: f64 = 0.0;
        let mut PLO: f64 = 0.0;
        let mut GSTO: f64 = 0.0;
        let mut XFACT: f64 = 0.0;
        let mut XGH2: f64 = 0.0;
        let mut XGH3: f64 = 0.0;
        let mut XGH4: f64 = 0.0;
        let mut XH2: f64 = 0.0;
        let mut XH3: f64 = 0.0;
        let mut XI2: f64 = 0.0;
        let mut XI3: f64 = 0.0;
        let mut XL2: f64 = 0.0;
        let mut XL3: f64 = 0.0;
        let mut XL4: f64 = 0.0;
        let mut XLAMO: f64 = 0.0;
        let mut ZMOL: f64 = 0.0;
        let mut ZMOS: f64 = 0.0;
        let mut ATIME: f64 = 0.0;
        let mut XLI: f64 = 0.0;
        let mut XNI: f64 = 0.0;
        let mut ARGPO: f64 = 0.0;
        let mut BSTAR: f64 = 0.0;
        let mut D2: f64 = 0.0;
        let mut D2201: f64 = 0.0;
        let mut D2211: f64 = 0.0;
        let mut D3: f64 = 0.0;
        let mut D3210: f64 = 0.0;
        let mut D3222: f64 = 0.0;
        let mut D4: f64 = 0.0;
        let mut D4410: f64 = 0.0;
        let mut D4422: f64 = 0.0;
        let mut D5220: f64 = 0.0;
        let mut D5232: f64 = 0.0;
        let mut D5421: f64 = 0.0;
        let mut D5433: f64 = 0.0;
        let mut DEDT: f64 = 0.0;
        let mut DEL1: f64 = 0.0;
        let mut DEL2: f64 = 0.0;
        let mut DEL3: f64 = 0.0;
        let mut DELMO: f64 = 0.0;
        let mut DIDT: f64 = 0.0;
        let mut DMDT: f64 = 0.0;
        let mut DNODT: f64 = 0.0;
        let mut DOMDT: f64 = 0.0;
        let mut ECCO: f64 = 0.0;
        let mut ER: f64 = 0.0;
        let mut J2: f64 = 0.0;
        let mut J3OJ2: f64 = 0.0;
        let mut MO: f64 = 0.0;
        let mut NO: f64 = 0.0;
        let mut NODEO: f64 = 0.0;
        let mut SE2: f64 = 0.0;
        let mut SE3: f64 = 0.0;
        let mut SGH2: f64 = 0.0;
        let mut SGH3: f64 = 0.0;
        let mut SGH4: f64 = 0.0;
        let mut SH2: f64 = 0.0;
        let mut SH3: f64 = 0.0;
        let mut SI2: f64 = 0.0;
        let mut SI3: f64 = 0.0;
        let mut SL2: f64 = 0.0;
        let mut SL3: f64 = 0.0;
        let mut SL4: f64 = 0.0;
        let mut XKE: f64 = 0.0;
        let mut SVMODE: i32 = 0;
        let mut IREZ: i32 = 0;
        let mut DOSIMP: bool = false;
        let mut DODEEP: bool = false;

        Self {
            A,
            ALTA,
            ALTP,
            ARGPDOT,
            AYCOF,
            CC1,
            CC4,
            CC5,
            CON41,
            ETA,
            INCLO,
            MDOT,
            NODEDOT,
            OMGCOF,
            SINMAO,
            T2COF,
            T3COF,
            T4COF,
            T5COF,
            X1MTH2,
            X7THM1,
            XLCOF,
            XMCOF,
            XNODCF,
            E3,
            EE2,
            PEO,
            PGHO,
            PHO,
            PINCO,
            PLO,
            GSTO,
            XFACT,
            XGH2,
            XGH3,
            XGH4,
            XH2,
            XH3,
            XI2,
            XI3,
            XL2,
            XL3,
            XL4,
            XLAMO,
            ZMOL,
            ZMOS,
            ATIME,
            XLI,
            XNI,
            ARGPO,
            BSTAR,
            D2,
            D2201,
            D2211,
            D3,
            D3210,
            D3222,
            D4,
            D4410,
            D4422,
            D5220,
            D5232,
            D5421,
            D5433,
            DEDT,
            DEL1,
            DEL2,
            DEL3,
            DELMO,
            DIDT,
            DMDT,
            DNODT,
            DOMDT,
            ECCO,
            ER,
            J2,
            J3OJ2,
            MO,
            NO,
            NODEO,
            SE2,
            SE3,
            SGH2,
            SGH3,
            SGH4,
            SH2,
            SH3,
            SI2,
            SI3,
            SL2,
            SL3,
            SL4,
            XKE,
            SVMODE,
            IREZ,
            DOSIMP,
            DODEEP,
        }
    }
}

//$Procedure ZZSGP4 ( SGP4 wrapper )
pub fn ZZSGP4(
    GEOPHS: &[f64],
    ELEMS: &[f64],
    OPMODE: i32,
    T: f64,
    STATE: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Local Variables
    //

    //
    // DS values
    //

    //
    // SPICELIB routines.
    //

    CHKIN(b"ZZSGP4", ctx)?;
    SETMSG(b"The routine ZZSGP4 is an umbrella for the SGP4 initializer and propagator entry points. Do not call ZZSGP4. It is likely that a programming error has been made.", ctx);
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZSGP4", ctx)?;

    Ok(())
}

//$Procedure XXSGP4I ( SGP4 initializer )
pub fn XXSGP4I(
    GEOPHS: &[f64],
    ELEMS: &[f64],
    OPMODE: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let GEOPHS = DummyArray::new(GEOPHS, 1..=8);
    let ELEMS = DummyArray::new(ELEMS, 1..=10);
    let mut AINV: f64 = 0.0;
    let mut AO: f64 = 0.0;
    let mut ARGPM: f64 = 0.0;
    let mut CC1SQ: f64 = 0.0;
    let mut CC2: f64 = 0.0;
    let mut CC3: f64 = 0.0;
    let mut CNODM: f64 = 0.0;
    let mut COEF: f64 = 0.0;
    let mut COEF1: f64 = 0.0;
    let mut CON42: f64 = 0.0;
    let mut COSIM: f64 = 0.0;
    let mut COSIO: f64 = 0.0;
    let mut COSIO2: f64 = 0.0;
    let mut COSIO4: f64 = 0.0;
    let mut COSOMM: f64 = 0.0;
    let mut DAY: f64 = 0.0;
    let mut DNDT: f64 = 0.0;
    let mut ECCM: f64 = 0.0;
    let mut ECCSQ: f64 = 0.0;
    let mut EETA: f64 = 0.0;
    let mut EMSQ: f64 = 0.0;
    let mut EPOCH: f64 = 0.0;
    let mut ETASQ: f64 = 0.0;
    let mut GAM: f64 = 0.0;
    let mut INCLM: f64 = 0.0;
    let mut J3: f64 = 0.0;
    let mut J4: f64 = 0.0;
    let mut MM: f64 = 0.0;
    let mut NODEM: f64 = 0.0;
    let mut OMEOSQ: f64 = 0.0;
    let mut PERIGE: f64 = 0.0;
    let mut PINVSQ: f64 = 0.0;
    let mut POSQ: f64 = 0.0;
    let mut PSISQ: f64 = 0.0;
    let mut QZMS24: f64 = 0.0;
    let mut QZMS2T: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut RTEMSQ: f64 = 0.0;
    let mut RTEOSQ: f64 = 0.0;
    let mut S1: f64 = 0.0;
    let mut S2: f64 = 0.0;
    let mut S3: f64 = 0.0;
    let mut S4: f64 = 0.0;
    let mut S5: f64 = 0.0;
    let mut S6: f64 = 0.0;
    let mut S7: f64 = 0.0;
    let mut SFOUR: f64 = 0.0;
    let mut SINIM: f64 = 0.0;
    let mut SINIO: f64 = 0.0;
    let mut SINOMM: f64 = 0.0;
    let mut SNODM: f64 = 0.0;
    let mut SS: f64 = 0.0;
    let mut SS1: f64 = 0.0;
    let mut SS2: f64 = 0.0;
    let mut SS3: f64 = 0.0;
    let mut SS4: f64 = 0.0;
    let mut SS5: f64 = 0.0;
    let mut SS6: f64 = 0.0;
    let mut SS7: f64 = 0.0;
    let mut SZ1: f64 = 0.0;
    let mut SZ11: f64 = 0.0;
    let mut SZ12: f64 = 0.0;
    let mut SZ13: f64 = 0.0;
    let mut SZ2: f64 = 0.0;
    let mut SZ21: f64 = 0.0;
    let mut SZ22: f64 = 0.0;
    let mut SZ23: f64 = 0.0;
    let mut SZ3: f64 = 0.0;
    let mut SZ31: f64 = 0.0;
    let mut SZ32: f64 = 0.0;
    let mut SZ33: f64 = 0.0;
    let mut TC: f64 = 0.0;
    let mut TEMP: f64 = 0.0;
    let mut TEMP1: f64 = 0.0;
    let mut TEMP2: f64 = 0.0;
    let mut TEMP3: f64 = 0.0;
    let mut TEMP4: f64 = 0.0;
    let mut TSI: f64 = 0.0;
    let mut TUMIN: f64 = 0.0;
    let mut TVEC = StackArray::<f64, 8>::new(1..=8);
    let mut TZERO: f64 = 0.0;
    let mut X2O3: f64 = 0.0;
    let mut XHDOT1: f64 = 0.0;
    let mut XN: f64 = 0.0;
    let mut XPIDOT: f64 = 0.0;
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
    let mut DOINIT: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"XXSGP4I", ctx)?;

    //
    // Initialize.
    //
    save.DODEEP = false;
    save.DOSIMP = false;
    save.SVMODE = OPMODE;

    //
    // This code block replaces the call:
    //
    // sgp4fix - note the following variables are also passed directly
    // via sgp4 common. It is possible to streamline the XXSGP4I call
    // by deleting the "x" variables, but the user would need to set
    // the common values first. we include the additional assignment
    // in case twoline2rv is not used.
    //
    //    bstar  = xbstar
    //    ecco   = xecco
    //    argpo  = xargpo
    //    inclo  = xinclo
    //    mo     = xmo
    //    no     = xno
    //    nodeo  = xnodeo

    save.BSTAR = ELEMS[KBSTAR];
    save.INCLO = ELEMS[KINCL];
    save.NODEO = ELEMS[KNODE0];
    save.ECCO = ELEMS[KECC];
    save.ARGPO = ELEMS[KOMEGA];
    save.MO = ELEMS[KMO];
    save.NO = ELEMS[KNO];

    //
    // Remember that sgp4 uses units of days from 0 jan 1950
    // (sgp4epoch) and minutes from the epoch (time)
    //
    // 2433281.5 JD TDB = 1949-12-31 00:00:00.000000 TDB
    // 2400000.5 JD TDB = 1858-11-17 00:00:00.000000 TDB
    //
    // 2433281.5 - 2400000.5 = 33281.0
    //

    //
    // Convert the J2000 TDB representation of the epoch to
    // JD UTC then calculate the offset from the JD 2433281.5 UTC
    // reference.
    //

    TVEC[1] = ELEMS[KEPOCH];
    TTRANS(b"TDB", b"JDUTC", TVEC.as_slice_mut(), ctx)?;
    EPOCH = (TVEC[1] - 2433281.5);

    if FAILED(ctx) {
        CHKOUT(b"XXSGP4I", ctx)?;
        return Ok(());
    }

    //
    // This code block replaces the call:
    //
    // CALL getgravconst( whichconst, tumin,
    // .                  mu, radiusearthkm, xke,
    // .                  j2, j3, j4, j3oj2 )
    //
    save.J2 = GEOPHS[K_J2];
    J3 = GEOPHS[K_J3];
    J4 = GEOPHS[K_J4];
    save.ER = GEOPHS[K_ER];
    save.XKE = GEOPHS[K_KE];
    TUMIN = (1.0 / GEOPHS[K_KE]);
    save.J3OJ2 = (J3 / save.J2);

    //
    // The following assignment and IF block is taken
    // from TWOLINE2RVSGP4.
    //
    save.A = f64::powf((save.NO * TUMIN), -(2.0 / 3.0));

    if (f64::abs((save.ECCO - 1.0)) > 0.000001) {
        save.ALTP = ((save.A * (1.0 - save.ECCO)) - 1.0);
        save.ALTA = ((save.A * (1.0 + save.ECCO)) - 1.0);
    } else {
        save.ALTA = 999999.9;
        save.ALTP = (2.0 * (4.0 / f64::powf((save.NO * save.NO), (1.0 / 3.0))));
    }

    SS = ((78.0 / save.ER) + 1.0);
    QZMS2T = f64::powi(((120.0 - 78.0) / save.ER), 4);
    X2O3 = (2.0 / 3.0);

    //
    // sgp4fix divisor for divide by zero check on inclination
    // the old check used 1.0D0 + cos(pi-1.0D-9), but then compared
    // it to 1.5D-12, so the threshold was changed to 1.5D-12 for
    // consistency.
    //
    TEMP4 = 0.0000000000015;
    TZERO = 0.0;
    DOINIT = true;

    ZZINIL(
        GEOPHS.as_slice(),
        OPMODE,
        save.ECCO,
        EPOCH,
        save.INCLO,
        &mut save.NO,
        &mut AINV,
        &mut AO,
        &mut save.CON41,
        &mut CON42,
        &mut COSIO,
        &mut COSIO2,
        &mut ECCSQ,
        &mut OMEOSQ,
        &mut POSQ,
        &mut RP,
        &mut RTEOSQ,
        &mut SINIO,
        &mut save.GSTO,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"XXSGP4I", ctx)?;
        return Ok(());
    }

    //
    // Check RP for a reasonable value. The propagator may not
    // calculate correct state values for RP < 1.
    //
    if (RP < 1.0) {
        SETMSG(b"TLE elements suborbital.", ctx);
        SIGERR(b"SPICE(SUBORBITAL)", ctx)?;
        CHKOUT(b"XXSGP4I", ctx)?;
        return Ok(());
    }

    //
    // If nodeo and No are gtr 0
    //
    if ((OMEOSQ >= 0.0) || (save.NO >= 0.0)) {
        save.DOSIMP = false;

        if (RP < ((220.0 / save.ER) + 1.0)) {
            save.DOSIMP = true;
        }

        SFOUR = SS;
        QZMS24 = QZMS2T;
        PERIGE = ((RP - 1.0) * save.ER);

        //
        // For perigees below 156 km, S and Qoms2t are altered.
        //
        if (PERIGE < 156.0) {
            SFOUR = (PERIGE - 78.0);

            if (PERIGE <= 98.0) {
                SFOUR = 20.0;
            }

            QZMS24 = f64::powi(((120.0 - SFOUR) / save.ER), 4);
            SFOUR = ((SFOUR / save.ER) + 1.0);
        }

        PINVSQ = (1.0 / POSQ);

        TSI = (1.0 / (AO - SFOUR));
        save.ETA = ((AO * save.ECCO) * TSI);
        ETASQ = (save.ETA * save.ETA);
        EETA = (save.ECCO * save.ETA);
        PSISQ = f64::abs((1.0 - ETASQ));
        COEF = (QZMS24 * f64::powi(TSI, 4));
        COEF1 = (COEF / f64::powf(PSISQ, 3.5));
        CC2 = ((COEF1 * save.NO)
            * ((AO * ((1.0 + (1.5 * ETASQ)) + (EETA * (4.0 + ETASQ))))
                + (((((0.375 * save.J2) * TSI) / PSISQ) * save.CON41)
                    * (8.0 + ((3.0 * ETASQ) * (8.0 + ETASQ))))));
        save.CC1 = (save.BSTAR * CC2);
        CC3 = 0.0;

        if (save.ECCO > 0.0001) {
            CC3 = -((((((2.0 * COEF) * TSI) * save.J3OJ2) * save.NO) * SINIO) / save.ECCO);
        }

        save.X1MTH2 = (1.0 - COSIO2);
        save.CC4 = (((((2.0 * save.NO) * COEF1) * AO) * OMEOSQ)
            * (((save.ETA * (2.0 + (0.5 * ETASQ))) + (save.ECCO * (0.5 + (2.0 * ETASQ))))
                - (((save.J2 * TSI) / (AO * PSISQ))
                    * (-((3.0 * save.CON41)
                        * ((1.0 - (2.0 * EETA)) + (ETASQ * (1.5 - (0.5 * EETA)))))
                        + (((0.75 * save.X1MTH2) * ((2.0 * ETASQ) - (EETA * (1.0 + ETASQ))))
                            * f64::cos((2.0 * save.ARGPO)))))));
        save.CC5 =
            ((((2.0 * COEF1) * AO) * OMEOSQ) * ((1.0 + (2.75 * (ETASQ + EETA))) + (EETA * ETASQ)));
        COSIO4 = (COSIO2 * COSIO2);
        TEMP1 = (((1.5 * save.J2) * PINVSQ) * save.NO);
        TEMP2 = (((0.5 * TEMP1) * save.J2) * PINVSQ);
        TEMP3 = -((((0.46875 * J4) * PINVSQ) * PINVSQ) * save.NO);
        save.MDOT = ((save.NO + (((0.5 * TEMP1) * RTEOSQ) * save.CON41))
            + (((0.0625 * TEMP2) * RTEOSQ) * ((13.0 - (78.0 * COSIO2)) + (137.0 * COSIO4))));
        save.ARGPDOT = ((-((0.5 * TEMP1) * CON42)
            + ((0.0625 * TEMP2) * ((7.0 - (114.0 * COSIO2)) + (395.0 * COSIO4))))
            + (TEMP3 * ((3.0 - (36.0 * COSIO2)) + (49.0 * COSIO4))));
        XHDOT1 = -(TEMP1 * COSIO);
        save.NODEDOT = (XHDOT1
            + ((((0.5 * TEMP2) * (4.0 - (19.0 * COSIO2)))
                + ((2.0 * TEMP3) * (3.0 - (7.0 * COSIO2))))
                * COSIO));
        XPIDOT = (save.ARGPDOT + save.NODEDOT);
        save.OMGCOF = ((save.BSTAR * CC3) * f64::cos(save.ARGPO));
        save.XMCOF = 0.0;

        if (save.ECCO > 0.0001) {
            save.XMCOF = -(((X2O3 * COEF) * save.BSTAR) / EETA);
        }

        save.XNODCF = (((3.5 * OMEOSQ) * XHDOT1) * save.CC1);
        save.T2COF = (1.5 * save.CC1);

        //
        // sgp4fix for divide by zero with xinco = 180 deg.
        //
        if (f64::abs((COSIO + 1.0)) > 0.0000000000015) {
            save.XLCOF = -((((0.25 * save.J3OJ2) * SINIO) * (3.0 + (5.0 * COSIO))) / (1.0 + COSIO));
        } else {
            save.XLCOF = -((((0.25 * save.J3OJ2) * SINIO) * (3.0 + (5.0 * COSIO))) / TEMP4);
        }

        save.AYCOF = -((0.5 * save.J3OJ2) * SINIO);
        save.DELMO = f64::powi((1.0 + (save.ETA * f64::cos(save.MO))), 3);
        save.SINMAO = f64::sin(save.MO);
        save.X7THM1 = ((7.0 * COSIO2) - 1.0);

        //
        // Deep Space Initialization
        //
        if ((TWOPI(ctx) / save.NO) >= 225.0) {
            save.DODEEP = true;
            save.DOSIMP = true;
            TC = 0.0;
            INCLM = save.INCLO;

            //
            // Common.
            //
            ZZDSCM(
                EPOCH,
                save.ECCO,
                save.ARGPO,
                TC,
                save.INCLO,
                save.NODEO,
                save.NO,
                &mut SNODM,
                &mut CNODM,
                &mut SINIM,
                &mut COSIM,
                &mut SINOMM,
                &mut COSOMM,
                &mut DAY,
                &mut save.E3,
                &mut save.EE2,
                &mut ECCM,
                &mut EMSQ,
                &mut GAM,
                &mut save.PEO,
                &mut save.PGHO,
                &mut save.PHO,
                &mut save.PINCO,
                &mut save.PLO,
                &mut RTEMSQ,
                &mut save.SE2,
                &mut save.SE3,
                &mut save.SGH2,
                &mut save.SGH3,
                &mut save.SGH4,
                &mut save.SH2,
                &mut save.SH3,
                &mut save.SI2,
                &mut save.SI3,
                &mut save.SL2,
                &mut save.SL3,
                &mut save.SL4,
                &mut S1,
                &mut S2,
                &mut S3,
                &mut S4,
                &mut S5,
                &mut S6,
                &mut S7,
                &mut SS1,
                &mut SS2,
                &mut SS3,
                &mut SS4,
                &mut SS5,
                &mut SS6,
                &mut SS7,
                &mut SZ1,
                &mut SZ2,
                &mut SZ3,
                &mut SZ11,
                &mut SZ12,
                &mut SZ13,
                &mut SZ21,
                &mut SZ22,
                &mut SZ23,
                &mut SZ31,
                &mut SZ32,
                &mut SZ33,
                &mut save.XGH2,
                &mut save.XGH3,
                &mut save.XGH4,
                &mut save.XH2,
                &mut save.XH3,
                &mut save.XI2,
                &mut save.XI3,
                &mut save.XL2,
                &mut save.XL3,
                &mut save.XL4,
                &mut XN,
                &mut Z1,
                &mut Z2,
                &mut Z3,
                &mut Z11,
                &mut Z12,
                &mut Z13,
                &mut Z21,
                &mut Z22,
                &mut Z23,
                &mut Z31,
                &mut Z32,
                &mut Z33,
                &mut save.ZMOL,
                &mut save.ZMOS,
                ctx,
            )?;

            //
            // Long period perturbations.
            //
            ZZDSPR(
                OPMODE,
                save.E3,
                save.EE2,
                save.PEO,
                save.PGHO,
                save.PHO,
                save.PINCO,
                save.PLO,
                save.SE2,
                save.SE3,
                save.SGH2,
                save.SGH3,
                save.SGH4,
                save.SH2,
                save.SH3,
                save.SI2,
                save.SI3,
                save.SL2,
                save.SL3,
                save.SL4,
                TZERO,
                save.XGH2,
                save.XGH3,
                save.XGH4,
                save.XH2,
                save.XH3,
                save.XI2,
                save.XI3,
                save.XL2,
                save.XL3,
                save.XL4,
                save.ZMOL,
                save.ZMOS,
                INCLM,
                DOINIT,
                &mut save.ECCO,
                &mut save.INCLO,
                &mut save.NODEO,
                &mut save.ARGPO,
                &mut save.MO,
                ctx,
            )?;

            ARGPM = 0.0;
            NODEM = 0.0;
            MM = 0.0;

            //
            // Initialization
            //
            ZZDSIN(
                GEOPHS.as_slice(),
                COSIM,
                &mut EMSQ,
                save.ARGPO,
                S1,
                S2,
                S3,
                S4,
                S5,
                SINIM,
                SS1,
                SS2,
                SS3,
                SS4,
                SS5,
                SZ1,
                SZ3,
                SZ11,
                SZ13,
                SZ21,
                SZ23,
                SZ31,
                SZ33,
                TZERO,
                TC,
                save.GSTO,
                save.MO,
                save.MDOT,
                save.NO,
                save.NODEO,
                save.NODEDOT,
                XPIDOT,
                Z1,
                Z3,
                Z11,
                Z13,
                Z21,
                Z23,
                Z31,
                Z33,
                save.ECCO,
                ECCSQ,
                &mut ECCM,
                &mut ARGPM,
                &mut INCLM,
                &mut MM,
                &mut XN,
                &mut NODEM,
                &mut save.IREZ,
                &mut save.ATIME,
                &mut save.D2201,
                &mut save.D2211,
                &mut save.D3210,
                &mut save.D3222,
                &mut save.D4410,
                &mut save.D4422,
                &mut save.D5220,
                &mut save.D5232,
                &mut save.D5421,
                &mut save.D5433,
                &mut save.DEDT,
                &mut save.DIDT,
                &mut save.DMDT,
                &mut DNDT,
                &mut save.DNODT,
                &mut save.DOMDT,
                &mut save.DEL1,
                &mut save.DEL2,
                &mut save.DEL3,
                &mut save.XFACT,
                &mut save.XLAMO,
                &mut save.XLI,
                &mut save.XNI,
                ctx,
            )?;
        }

        //
        // Set variables if not deep space or rp < 220
        //
        if !save.DOSIMP {
            CC1SQ = (save.CC1 * save.CC1);
            save.D2 = (((4.0 * AO) * TSI) * CC1SQ);
            TEMP = (((save.D2 * TSI) * save.CC1) / 3.0);
            save.D3 = (((17.0 * AO) + SFOUR) * TEMP);
            save.D4 = (((((0.5 * TEMP) * AO) * TSI) * ((221.0 * AO) + (31.0 * SFOUR))) * save.CC1);
            save.T3COF = (save.D2 + (2.0 * CC1SQ));
            save.T4COF =
                (0.25 * ((3.0 * save.D3) + (save.CC1 * ((12.0 * save.D2) + (10.0 * CC1SQ)))));
            save.T5COF = (0.2
                * ((((3.0 * save.D4) + ((12.0 * save.CC1) * save.D3))
                    + ((6.0 * save.D2) * save.D2))
                    + ((15.0 * CC1SQ) * ((2.0 * save.D2) + CC1SQ))));
        }
    }

    DOINIT = false;

    CHKOUT(b"XXSGP4I", ctx)?;
    Ok(())
}

//$Procedure XXSGP4E ( SGP4 evaluator )
pub fn XXSGP4E(T: f64, STATE: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATE = DummyArrayMut::new(STATE, 1..=6);
    let mut AM: f64 = 0.0;
    let mut ARGPM: f64 = 0.0;
    let mut ARGPP: f64 = 0.0;
    let mut AXNL: f64 = 0.0;
    let mut AYNL: f64 = 0.0;
    let mut BETAL: f64 = 0.0;
    let mut CNOD: f64 = 0.0;
    let mut COS2U: f64 = 0.0;
    let mut COSEO1: f64 = 0.0;
    let mut COSI: f64 = 0.0;
    let mut COSIM: f64 = 0.0;
    let mut COSIP: f64 = 0.0;
    let mut COSISQ: f64 = 0.0;
    let mut COSSU: f64 = 0.0;
    let mut COSU: f64 = 0.0;
    let mut DELM: f64 = 0.0;
    let mut DELOMG: f64 = 0.0;
    let mut DNDT: f64 = 0.0;
    let mut ECCM: f64 = 0.0;
    let mut ECCP: f64 = 0.0;
    let mut ECOSE: f64 = 0.0;
    let mut EL2: f64 = 0.0;
    let mut EMSQ: f64 = 0.0;
    let mut EO1: f64 = 0.0;
    let mut ESINE: f64 = 0.0;
    let mut INCLM: f64 = 0.0;
    let mut KPS: f64 = 0.0;
    let mut MM: f64 = 0.0;
    let mut MP: f64 = 0.0;
    let mut MR: f64 = 0.0;
    let mut MV: f64 = 0.0;
    let mut NODEM: f64 = 0.0;
    let mut NODEP: f64 = 0.0;
    let mut OMGADF: f64 = 0.0;
    let mut PL: f64 = 0.0;
    let mut RDOTL: f64 = 0.0;
    let mut RL: f64 = 0.0;
    let mut RVDOT: f64 = 0.0;
    let mut RVDOTL: f64 = 0.0;
    let mut SIN2U: f64 = 0.0;
    let mut SINEO1: f64 = 0.0;
    let mut SINI: f64 = 0.0;
    let mut SINIM: f64 = 0.0;
    let mut SINIP: f64 = 0.0;
    let mut SINSU: f64 = 0.0;
    let mut SINU: f64 = 0.0;
    let mut SNOD: f64 = 0.0;
    let mut SU: f64 = 0.0;
    let mut T2: f64 = 0.0;
    let mut T3: f64 = 0.0;
    let mut T4: f64 = 0.0;
    let mut TC: f64 = 0.0;
    let mut TEM5: f64 = 0.0;
    let mut TEMP: f64 = 0.0;
    let mut TEMP1: f64 = 0.0;
    let mut TEMP2: f64 = 0.0;
    let mut TEMP4: f64 = 0.0;
    let mut TEMPA: f64 = 0.0;
    let mut TEMPE: f64 = 0.0;
    let mut TEMPL: f64 = 0.0;
    let mut U: f64 = 0.0;
    let mut UX: f64 = 0.0;
    let mut UY: f64 = 0.0;
    let mut UZ: f64 = 0.0;
    let mut VX: f64 = 0.0;
    let mut VY: f64 = 0.0;
    let mut VZ: f64 = 0.0;
    let mut X2O3: f64 = 0.0;
    let mut XINC: f64 = 0.0;
    let mut XINCP: f64 = 0.0;
    let mut XL: f64 = 0.0;
    let mut XLM: f64 = 0.0;
    let mut XMDF: f64 = 0.0;
    let mut XMX: f64 = 0.0;
    let mut XMY: f64 = 0.0;
    let mut XN: f64 = 0.0;
    let mut XNODDF: f64 = 0.0;
    let mut XNODE: f64 = 0.0;
    let mut ITER: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"XXSGP4E", ctx)?;

    //
    // Local constants. Keep compiler ok for warnings on
    // uninitialized variables
    //
    X2O3 = (2.0 / 3.0);
    MR = 0.0;
    COSEO1 = 1.0;
    SINEO1 = 0.0;

    //
    // Set mathematical constants.
    //
    // This code block replaces the call:
    //
    // sgp4fix identify constants and allow alternate values.
    //
    // CALL getgravconst( whichconst, tumin,
    // .                  mu, radiusearthkm, xke,
    // .                  j2, j3, j4, j3oj2 )
    //

    //
    // sgp4fix divisor for divide by zero check on inclination
    // the old check used 1.0D0 + cos(pi-1.0D-9), but then compared it to
    // 1.5D-12, so the threshold was changed to 1.5D-12 for consistency.
    //
    TEMP4 = 0.0000000000015;
    KPS = ((save.ER * save.XKE) / 60.0);

    //
    // UPDATE FOR SECULAR GRAVITY AND ATMOSPHERIC DRAG
    //
    XMDF = (save.MO + (save.MDOT * T));
    OMGADF = (save.ARGPO + (save.ARGPDOT * T));
    XNODDF = (save.NODEO + (save.NODEDOT * T));
    ARGPM = OMGADF;
    MM = XMDF;
    T2 = (T * T);
    NODEM = (XNODDF + (save.XNODCF * T2));
    TEMPA = (1.0 - (save.CC1 * T));
    TEMPE = ((save.BSTAR * save.CC4) * T);
    TEMPL = (save.T2COF * T2);

    if !save.DOSIMP {
        DELOMG = (save.OMGCOF * T);
        DELM = (save.XMCOF * (f64::powi((1.0 + (save.ETA * f64::cos(XMDF))), 3) - save.DELMO));
        TEMP = (DELOMG + DELM);
        MM = (XMDF + TEMP);
        ARGPM = (OMGADF - TEMP);
        T3 = (T2 * T);
        T4 = (T3 * T);
        TEMPA = (((TEMPA - (save.D2 * T2)) - (save.D3 * T3)) - (save.D4 * T4));
        TEMPE = (TEMPE + ((save.BSTAR * save.CC5) * (f64::sin(MM) - save.SINMAO)));
        TEMPL = ((TEMPL + (save.T3COF * T3)) + (T4 * (save.T4COF + (T * save.T5COF))));
    }

    XN = save.NO;
    ECCM = save.ECCO;
    INCLM = save.INCLO;

    if save.DODEEP {
        TC = T;

        ZZDSPC(
            save.IREZ,
            save.D2201,
            save.D2211,
            save.D3210,
            save.D3222,
            save.D4410,
            save.D4422,
            save.D5220,
            save.D5232,
            save.D5421,
            save.D5433,
            save.DEDT,
            save.DEL1,
            save.DEL2,
            save.DEL3,
            save.DIDT,
            save.DMDT,
            save.DNODT,
            save.DOMDT,
            save.ARGPO,
            save.ARGPDOT,
            T,
            TC,
            save.GSTO,
            save.XFACT,
            save.XLAMO,
            save.NO,
            &mut save.ATIME,
            &mut ECCM,
            &mut ARGPM,
            &mut INCLM,
            &mut save.XLI,
            &mut MM,
            &mut save.XNI,
            &mut NODEM,
            &mut DNDT,
            &mut XN,
            ctx,
        )?;
    }

    //
    // Mean motion less than 0.0.
    //
    if (XN <= 0.0) {
        SETMSG(
            b"Mean motion less-than zero. This error may indicate a bad TLE set.",
            ctx,
        );
        SIGERR(b"SPICE(BADMEANMOTION)", ctx)?;
        CHKOUT(b"XXSGP4E", ctx)?;
        return Ok(());
    }

    AM = (f64::powf((save.XKE / XN), X2O3) * f64::powi(TEMPA, 2));
    XN = (save.XKE / f64::powf(AM, 1.5));
    ECCM = (ECCM - TEMPE);

    //
    // Fix tolerance for error recognition. Vallado code used
    // a lower limit of -0.001. This value apparently prevents
    // an error signal due to roundoff error.
    //
    if ((ECCM >= 1.0) || (ECCM < -0.001)) {
        SETMSG(b"Mean eccentricity value, #, beyond allowed bounds [-0.001,1.0). This error may indicate a bad TLE set.", ctx);
        ERRDP(b"#", ECCM, ctx);
        SIGERR(b"SPICE(BADMECCENTRICITY)", ctx)?;
        CHKOUT(b"XXSGP4E", ctx)?;
        return Ok(());
    }

    if (AM < 0.95) {
        SETMSG(b"Mean semi-major axis value, #, below allowed minimum of 0.95. This error may indicate a bad TLE set or a decayed orbit.", ctx);
        ERRDP(b"#", ECCM, ctx);
        SIGERR(b"SPICE(BADMSEMIMAJOR)", ctx)?;
        CHKOUT(b"XXSGP4E", ctx)?;
        return Ok(());
    }

    //
    // sgp4fix change test condition for eccentricity
    //
    if (ECCM < 0.000001) {
        ECCM = 0.000001;
    }

    MM = (MM + (save.NO * TEMPL));
    XLM = ((MM + ARGPM) + NODEM);
    EMSQ = (ECCM * ECCM);
    TEMP = (1.0 - EMSQ);
    NODEM = intrinsics::DMOD(NODEM, TWOPI(ctx));
    ARGPM = intrinsics::DMOD(ARGPM, TWOPI(ctx));
    XLM = intrinsics::DMOD(XLM, TWOPI(ctx));
    MM = intrinsics::DMOD(((XLM - ARGPM) - NODEM), TWOPI(ctx));

    //
    // Compute extra mean quantities
    //
    SINIM = f64::sin(INCLM);
    COSIM = f64::cos(INCLM);

    //
    // Add lunar-solar periodics
    //
    ECCP = ECCM;
    XINCP = INCLM;
    ARGPP = ARGPM;
    NODEP = NODEM;
    MP = MM;
    SINIP = SINIM;
    COSIP = COSIM;

    //
    // Use deep space perturbation if indicated.
    //

    if save.DODEEP {
        ZZDSPR(
            save.SVMODE,
            save.E3,
            save.EE2,
            save.PEO,
            save.PGHO,
            save.PHO,
            save.PINCO,
            save.PLO,
            save.SE2,
            save.SE3,
            save.SGH2,
            save.SGH3,
            save.SGH4,
            save.SH2,
            save.SH3,
            save.SI2,
            save.SI3,
            save.SL2,
            save.SL3,
            save.SL4,
            T,
            save.XGH2,
            save.XGH3,
            save.XGH4,
            save.XH2,
            save.XH3,
            save.XI2,
            save.XI3,
            save.XL2,
            save.XL3,
            save.XL4,
            save.ZMOL,
            save.ZMOS,
            save.INCLO,
            false,
            &mut ECCP,
            &mut XINCP,
            &mut NODEP,
            &mut ARGPP,
            &mut MP,
            ctx,
        )?;

        if (XINCP < 0.0) {
            XINCP = -XINCP;
            NODEP = (NODEP + PI(ctx));
            ARGPP = (ARGPP - PI(ctx));
        }

        if ((ECCP < 0.0) || (ECCP > 1.0)) {
            SETMSG(b"Perturbed eccentricity value, #, beyond allowed bounds [0,1]. This error may indicate a bad TLE set.", ctx);
            ERRDP(b"#", ECCP, ctx);
            SIGERR(b"SPICE(BADPECCENTRICITY)", ctx)?;
            CHKOUT(b"XXSGP4E", ctx)?;
            return Ok(());
        }
    }

    //
    // Update for long period periodics if a deep space trajectory.
    //

    if save.DODEEP {
        SINIP = f64::sin(XINCP);
        COSIP = f64::cos(XINCP);
        save.AYCOF = -((0.5 * save.J3OJ2) * SINIP);

        //
        // sgp4fix for divide by zero with xincp = 180 deg
        //

        if (f64::abs((COSIP + 1.0)) > 0.0000000000015) {
            save.XLCOF = -((((0.25 * save.J3OJ2) * SINIP) * (3.0 + (5.0 * COSIP))) / (1.0 + COSIP));
        } else {
            save.XLCOF = -((((0.25 * save.J3OJ2) * SINIP) * (3.0 + (5.0 * COSIP))) / TEMP4);
        }
    }

    AXNL = (ECCP * f64::cos(ARGPP));
    TEMP = (1.0 / (AM * (1.0 - (ECCP * ECCP))));
    AYNL = ((ECCP * f64::sin(ARGPP)) + (TEMP * save.AYCOF));
    XL = (((MP + ARGPP) + NODEP) + ((TEMP * save.XLCOF) * AXNL));

    //
    // Solve Kepler's equation.
    //
    U = intrinsics::DMOD((XL - NODEP), TWOPI(ctx));
    EO1 = U;
    ITER = 0;

    //
    // sgp4fix for Kepler iteration the following iteration needs
    // better limits on corrections
    //

    TEMP = 9999.9;

    while ((TEMP >= 0.000000000001) && (ITER < 10)) {
        ITER = (ITER + 1);
        SINEO1 = f64::sin(EO1);
        COSEO1 = f64::cos(EO1);
        TEM5 = ((1.0 - (COSEO1 * AXNL)) - (SINEO1 * AYNL));
        TEM5 = ((((U - (AYNL * COSEO1)) + (AXNL * SINEO1)) - EO1) / TEM5);
        TEMP = f64::abs(TEM5);

        //
        // Stop excessive correction.
        //
        if (TEMP > 1.0) {
            TEM5 = (TEM5 / TEMP);
        }

        EO1 = (EO1 + TEM5);
    }

    //
    // Short period preliminary quantities.
    //
    ECOSE = ((AXNL * COSEO1) + (AYNL * SINEO1));
    ESINE = ((AXNL * SINEO1) - (AYNL * COSEO1));
    EL2 = ((AXNL * AXNL) + (AYNL * AYNL));
    PL = (AM * (1.0 - EL2));

    //
    // Error check for semi-latus rectum < 0.0
    //
    if (PL < 0.0) {
        SETMSG(b"Semi-latus rectum less-than zero.", ctx);
        SIGERR(b"SPICE(BADSEMILATUS)", ctx)?;
        CHKOUT(b"XXSGP4E", ctx)?;
        return Ok(());
    }

    RL = (AM * (1.0 - ECOSE));
    RDOTL = ((f64::sqrt(AM) * ESINE) / RL);
    RVDOTL = (f64::sqrt(PL) / RL);
    BETAL = f64::sqrt((1.0 - EL2));
    TEMP = (ESINE / (1.0 + BETAL));
    SINU = ((AM / RL) * ((SINEO1 - AYNL) - (AXNL * TEMP)));
    COSU = ((AM / RL) * ((COSEO1 - AXNL) + (AYNL * TEMP)));
    SU = f64::atan2(SINU, COSU);
    SIN2U = ((COSU + COSU) * SINU);
    COS2U = (1.0 - ((2.0 * SINU) * SINU));
    TEMP = (1.0 / PL);
    TEMP1 = ((0.5 * save.J2) * TEMP);
    TEMP2 = (TEMP1 * TEMP);

    //
    // Update for short period periodics if a deep space trajectory.
    //
    if save.DODEEP {
        COSISQ = (COSIP * COSIP);
        save.CON41 = ((3.0 * COSISQ) - 1.0);
        save.X1MTH2 = (1.0 - COSISQ);
        save.X7THM1 = ((7.0 * COSISQ) - 1.0);
    }

    MR = ((RL * (1.0 - (((1.5 * TEMP2) * BETAL) * save.CON41)))
        + (((0.5 * TEMP1) * save.X1MTH2) * COS2U));
    SU = (SU - (((0.25 * TEMP2) * save.X7THM1) * SIN2U));
    XNODE = (NODEP + (((1.5 * TEMP2) * COSIP) * SIN2U));
    XINC = (XINCP + ((((1.5 * TEMP2) * COSIP) * SINIP) * COS2U));
    MV = (RDOTL - ((((XN * TEMP1) * save.X1MTH2) * SIN2U) / save.XKE));
    RVDOT = (RVDOTL + (((XN * TEMP1) * ((save.X1MTH2 * COS2U) + (1.5 * save.CON41))) / save.XKE));

    //
    // Orientation vectors.
    //

    SINSU = f64::sin(SU);
    COSSU = f64::cos(SU);
    SNOD = f64::sin(XNODE);
    CNOD = f64::cos(XNODE);
    SINI = f64::sin(XINC);
    COSI = f64::cos(XINC);
    XMX = -(SNOD * COSI);
    XMY = (CNOD * COSI);
    UX = ((XMX * SINSU) + (CNOD * COSSU));
    UY = ((XMY * SINSU) + (SNOD * COSSU));
    UZ = (SINI * SINSU);
    VX = ((XMX * COSSU) - (CNOD * SINSU));
    VY = ((XMY * COSSU) - (SNOD * SINSU));
    VZ = (SINI * COSSU);

    //
    // Position and velocity.
    //

    STATE[1] = ((MR * UX) * save.ER);
    STATE[2] = ((MR * UY) * save.ER);
    STATE[3] = ((MR * UZ) * save.ER);
    STATE[4] = (((MV * UX) + (RVDOT * VX)) * KPS);
    STATE[5] = (((MV * UY) + (RVDOT * VY)) * KPS);
    STATE[6] = (((MV * UZ) + (RVDOT * VZ)) * KPS);

    //
    // sgp4fix for decaying satellites
    //
    // Place this test here to ensure evaluation of STATE.
    // The result may be physically invalid.
    //
    if (MR < 1.0) {
        SETMSG(b"Satellite has decayed.", ctx);
        SIGERR(b"SPICE(ORBITDECAY)", ctx)?;
        CHKOUT(b"XXSGP4E", ctx)?;
        return Ok(());
    }

    CHKOUT(b"XXSGP4E", ctx)?;

    Ok(())
}
