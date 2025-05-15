//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const NMODL: i32 = 6;
const KJ2: i32 = 1;
const KJ3: i32 = 2;
const KJ4: i32 = 3;
const KKE: i32 = 4;
const KQO: i32 = 5;
const KSO: i32 = 6;
const KER: i32 = 7;
const KAE: i32 = 8;
const NGEOCN: i32 = KAE;
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
const NEXT: i32 = 1;
const PREV: i32 = 2;
const NIL: i32 = 0;
const PRSIZE: i32 = 29;
const MXLOOP: i32 = 20;
const HALF: f64 = 0.5;
const ONE: f64 = 1.0;
const F3OV8: f64 = (3.0 / 8.0);
const F3OV2: f64 = 1.5;
const TOTHRD: f64 = (2.0 / 3.0);
const ONETHD: f64 = (1.0 / 3.0);

struct SaveVars {
    J2: f64,
    J3: f64,
    J4: f64,
    KE: f64,
    QO: f64,
    SO: f64,
    ER: f64,
    AE: f64,
    AE2: f64,
    AE3: f64,
    AE4: f64,
    CK2: f64,
    A3OVK2: f64,
    CK4: f64,
    QOMSO: f64,
    Q1: f64,
    Q2: f64,
    QOMS2T: f64,
    S: f64,
    EPOCH: f64,
    XNDT2O: f64,
    XNDD6O: f64,
    BSTAR: f64,
    XINCL: f64,
    XNODEO: f64,
    EO: f64,
    OMEGAO: f64,
    XMO: f64,
    XNO: f64,
    A: f64,
    A1: f64,
    AO: f64,
    AODP: f64,
    AXN: f64,
    AYCOF: f64,
    AYN: f64,
    AYNL: f64,
    BETAL: f64,
    BETAO: f64,
    BETAO2: f64,
    BETAO3: f64,
    BETAO4: f64,
    C1: f64,
    C1SQ: f64,
    C2: f64,
    C3: f64,
    C4: f64,
    C5: f64,
    CAPU: f64,
    COEF: f64,
    COEF1: f64,
    COS2U: f64,
    COSEPW: f64,
    COSIK: f64,
    COSIO: f64,
    COSNOK: f64,
    COSU: f64,
    COSUK: f64,
    D2: f64,
    D3: f64,
    D4: f64,
    DEL1: f64,
    DELM: f64,
    DELMO: f64,
    DELO: f64,
    DELOMG: f64,
    E: f64,
    ECOSE: f64,
    EETA: f64,
    ELEMNT: StackArray2D<f64, 60>,
    ELSQ: f64,
    EPSILN: f64,
    EPW: f64,
    EPWNXT: f64,
    ESINE: f64,
    EST: f64,
    ETA: f64,
    ETASQ: f64,
    F: f64,
    FL: f64,
    FMOD2P: f64,
    FPRIME: f64,
    FU: f64,
    LOWER: f64,
    LSTGEO: StackArray<f64, 8>,
    M: f64,
    MOV1M: f64,
    OMEGA: f64,
    OMGADF: f64,
    OMGCOF: f64,
    OMGDOT: f64,
    PERIGE: f64,
    PINVSQ: f64,
    PIX2: f64,
    PL: f64,
    PRELIM: StackArray2D<f64, 174>,
    PSISQ: f64,
    QOMS24: f64,
    R: f64,
    RDOT: f64,
    RDOTK: f64,
    RFDOT: f64,
    RFDOTK: f64,
    RK: f64,
    S4: f64,
    SIN2U: f64,
    SINEPW: f64,
    SINIK: f64,
    SINIO: f64,
    SINMO: f64,
    SINNOK: f64,
    SINU: f64,
    SINUK: f64,
    T2COF: f64,
    T3COF: f64,
    T4COF: f64,
    T5COF: f64,
    TCUBE: f64,
    TEMP: f64,
    TEMP1: f64,
    TEMP2: f64,
    TEMP3: f64,
    TEMP4: f64,
    TEMP5: f64,
    TEMP6: f64,
    TEMPA: f64,
    TEMPE: f64,
    TEMPL: f64,
    TFOUR: f64,
    THETA2: f64,
    THETA4: f64,
    TOKM: f64,
    TOKMPS: f64,
    TSI: f64,
    TSINCE: f64,
    TSQ: f64,
    U: f64,
    UK: f64,
    UPPER: f64,
    UX: f64,
    UY: f64,
    UZ: f64,
    VX: f64,
    VY: f64,
    VZ: f64,
    X1M5TH: f64,
    X1MTH2: f64,
    X3THM1: f64,
    X7THM1: f64,
    XHDOT1: f64,
    XINCK: f64,
    XL: f64,
    XLCOF: f64,
    XLL: f64,
    XLT: f64,
    XMCOF: f64,
    XMDF: f64,
    XMDOT: f64,
    XMP: f64,
    XMX: f64,
    XMY: f64,
    XN: f64,
    XNODCF: f64,
    XNODDF: f64,
    XNODE: f64,
    XNODEK: f64,
    XNODOT: f64,
    XNODP: f64,
    BEFORE: i32,
    COUNT: i32,
    AFTER: i32,
    HEAD: i32,
    I: i32,
    LAST: i32,
    LIST: StackArray2D<i32, 12>,
    N: i32,
    DOINIT: bool,
    NEWGEO: bool,
    RECOG: bool,
    UNREC: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut J2: f64 = 0.0;
        let mut J3: f64 = 0.0;
        let mut J4: f64 = 0.0;
        let mut KE: f64 = 0.0;
        let mut QO: f64 = 0.0;
        let mut SO: f64 = 0.0;
        let mut ER: f64 = 0.0;
        let mut AE: f64 = 0.0;
        let mut AE2: f64 = 0.0;
        let mut AE3: f64 = 0.0;
        let mut AE4: f64 = 0.0;
        let mut CK2: f64 = 0.0;
        let mut A3OVK2: f64 = 0.0;
        let mut CK4: f64 = 0.0;
        let mut QOMSO: f64 = 0.0;
        let mut Q1: f64 = 0.0;
        let mut Q2: f64 = 0.0;
        let mut QOMS2T: f64 = 0.0;
        let mut S: f64 = 0.0;
        let mut EPOCH: f64 = 0.0;
        let mut XNDT2O: f64 = 0.0;
        let mut XNDD6O: f64 = 0.0;
        let mut BSTAR: f64 = 0.0;
        let mut XINCL: f64 = 0.0;
        let mut XNODEO: f64 = 0.0;
        let mut EO: f64 = 0.0;
        let mut OMEGAO: f64 = 0.0;
        let mut XMO: f64 = 0.0;
        let mut XNO: f64 = 0.0;
        let mut A: f64 = 0.0;
        let mut A1: f64 = 0.0;
        let mut AO: f64 = 0.0;
        let mut AODP: f64 = 0.0;
        let mut AXN: f64 = 0.0;
        let mut AYCOF: f64 = 0.0;
        let mut AYN: f64 = 0.0;
        let mut AYNL: f64 = 0.0;
        let mut BETAL: f64 = 0.0;
        let mut BETAO: f64 = 0.0;
        let mut BETAO2: f64 = 0.0;
        let mut BETAO3: f64 = 0.0;
        let mut BETAO4: f64 = 0.0;
        let mut C1: f64 = 0.0;
        let mut C1SQ: f64 = 0.0;
        let mut C2: f64 = 0.0;
        let mut C3: f64 = 0.0;
        let mut C4: f64 = 0.0;
        let mut C5: f64 = 0.0;
        let mut CAPU: f64 = 0.0;
        let mut COEF: f64 = 0.0;
        let mut COEF1: f64 = 0.0;
        let mut COS2U: f64 = 0.0;
        let mut COSEPW: f64 = 0.0;
        let mut COSIK: f64 = 0.0;
        let mut COSIO: f64 = 0.0;
        let mut COSNOK: f64 = 0.0;
        let mut COSU: f64 = 0.0;
        let mut COSUK: f64 = 0.0;
        let mut D2: f64 = 0.0;
        let mut D3: f64 = 0.0;
        let mut D4: f64 = 0.0;
        let mut DEL1: f64 = 0.0;
        let mut DELM: f64 = 0.0;
        let mut DELMO: f64 = 0.0;
        let mut DELO: f64 = 0.0;
        let mut DELOMG: f64 = 0.0;
        let mut E: f64 = 0.0;
        let mut ECOSE: f64 = 0.0;
        let mut EETA: f64 = 0.0;
        let mut ELEMNT = StackArray2D::<f64, 60>::new(1..=NELEMS, 1..=NMODL);
        let mut ELSQ: f64 = 0.0;
        let mut EPSILN: f64 = 0.0;
        let mut EPW: f64 = 0.0;
        let mut EPWNXT: f64 = 0.0;
        let mut ESINE: f64 = 0.0;
        let mut EST: f64 = 0.0;
        let mut ETA: f64 = 0.0;
        let mut ETASQ: f64 = 0.0;
        let mut F: f64 = 0.0;
        let mut FL: f64 = 0.0;
        let mut FMOD2P: f64 = 0.0;
        let mut FPRIME: f64 = 0.0;
        let mut FU: f64 = 0.0;
        let mut LOWER: f64 = 0.0;
        let mut LSTGEO = StackArray::<f64, 8>::new(1..=NGEOCN);
        let mut M: f64 = 0.0;
        let mut MOV1M: f64 = 0.0;
        let mut OMEGA: f64 = 0.0;
        let mut OMGADF: f64 = 0.0;
        let mut OMGCOF: f64 = 0.0;
        let mut OMGDOT: f64 = 0.0;
        let mut PERIGE: f64 = 0.0;
        let mut PINVSQ: f64 = 0.0;
        let mut PIX2: f64 = 0.0;
        let mut PL: f64 = 0.0;
        let mut PRELIM = StackArray2D::<f64, 174>::new(1..=PRSIZE, 1..=NMODL);
        let mut PSISQ: f64 = 0.0;
        let mut QOMS24: f64 = 0.0;
        let mut R: f64 = 0.0;
        let mut RDOT: f64 = 0.0;
        let mut RDOTK: f64 = 0.0;
        let mut RFDOT: f64 = 0.0;
        let mut RFDOTK: f64 = 0.0;
        let mut RK: f64 = 0.0;
        let mut S4: f64 = 0.0;
        let mut SIN2U: f64 = 0.0;
        let mut SINEPW: f64 = 0.0;
        let mut SINIK: f64 = 0.0;
        let mut SINIO: f64 = 0.0;
        let mut SINMO: f64 = 0.0;
        let mut SINNOK: f64 = 0.0;
        let mut SINU: f64 = 0.0;
        let mut SINUK: f64 = 0.0;
        let mut T2COF: f64 = 0.0;
        let mut T3COF: f64 = 0.0;
        let mut T4COF: f64 = 0.0;
        let mut T5COF: f64 = 0.0;
        let mut TCUBE: f64 = 0.0;
        let mut TEMP: f64 = 0.0;
        let mut TEMP1: f64 = 0.0;
        let mut TEMP2: f64 = 0.0;
        let mut TEMP3: f64 = 0.0;
        let mut TEMP4: f64 = 0.0;
        let mut TEMP5: f64 = 0.0;
        let mut TEMP6: f64 = 0.0;
        let mut TEMPA: f64 = 0.0;
        let mut TEMPE: f64 = 0.0;
        let mut TEMPL: f64 = 0.0;
        let mut TFOUR: f64 = 0.0;
        let mut THETA2: f64 = 0.0;
        let mut THETA4: f64 = 0.0;
        let mut TOKM: f64 = 0.0;
        let mut TOKMPS: f64 = 0.0;
        let mut TSI: f64 = 0.0;
        let mut TSINCE: f64 = 0.0;
        let mut TSQ: f64 = 0.0;
        let mut U: f64 = 0.0;
        let mut UK: f64 = 0.0;
        let mut UPPER: f64 = 0.0;
        let mut UX: f64 = 0.0;
        let mut UY: f64 = 0.0;
        let mut UZ: f64 = 0.0;
        let mut VX: f64 = 0.0;
        let mut VY: f64 = 0.0;
        let mut VZ: f64 = 0.0;
        let mut X1M5TH: f64 = 0.0;
        let mut X1MTH2: f64 = 0.0;
        let mut X3THM1: f64 = 0.0;
        let mut X7THM1: f64 = 0.0;
        let mut XHDOT1: f64 = 0.0;
        let mut XINCK: f64 = 0.0;
        let mut XL: f64 = 0.0;
        let mut XLCOF: f64 = 0.0;
        let mut XLL: f64 = 0.0;
        let mut XLT: f64 = 0.0;
        let mut XMCOF: f64 = 0.0;
        let mut XMDF: f64 = 0.0;
        let mut XMDOT: f64 = 0.0;
        let mut XMP: f64 = 0.0;
        let mut XMX: f64 = 0.0;
        let mut XMY: f64 = 0.0;
        let mut XN: f64 = 0.0;
        let mut XNODCF: f64 = 0.0;
        let mut XNODDF: f64 = 0.0;
        let mut XNODE: f64 = 0.0;
        let mut XNODEK: f64 = 0.0;
        let mut XNODOT: f64 = 0.0;
        let mut XNODP: f64 = 0.0;
        let mut BEFORE: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut AFTER: i32 = 0;
        let mut HEAD: i32 = 0;
        let mut I: i32 = 0;
        let mut LAST: i32 = 0;
        let mut LIST = StackArray2D::<i32, 12>::new(1..=2, 1..=NMODL);
        let mut N: i32 = 0;
        let mut DOINIT: bool = false;
        let mut NEWGEO: bool = false;
        let mut RECOG: bool = false;
        let mut UNREC: bool = false;

        DOINIT = true;

        Self {
            J2,
            J3,
            J4,
            KE,
            QO,
            SO,
            ER,
            AE,
            AE2,
            AE3,
            AE4,
            CK2,
            A3OVK2,
            CK4,
            QOMSO,
            Q1,
            Q2,
            QOMS2T,
            S,
            EPOCH,
            XNDT2O,
            XNDD6O,
            BSTAR,
            XINCL,
            XNODEO,
            EO,
            OMEGAO,
            XMO,
            XNO,
            A,
            A1,
            AO,
            AODP,
            AXN,
            AYCOF,
            AYN,
            AYNL,
            BETAL,
            BETAO,
            BETAO2,
            BETAO3,
            BETAO4,
            C1,
            C1SQ,
            C2,
            C3,
            C4,
            C5,
            CAPU,
            COEF,
            COEF1,
            COS2U,
            COSEPW,
            COSIK,
            COSIO,
            COSNOK,
            COSU,
            COSUK,
            D2,
            D3,
            D4,
            DEL1,
            DELM,
            DELMO,
            DELO,
            DELOMG,
            E,
            ECOSE,
            EETA,
            ELEMNT,
            ELSQ,
            EPSILN,
            EPW,
            EPWNXT,
            ESINE,
            EST,
            ETA,
            ETASQ,
            F,
            FL,
            FMOD2P,
            FPRIME,
            FU,
            LOWER,
            LSTGEO,
            M,
            MOV1M,
            OMEGA,
            OMGADF,
            OMGCOF,
            OMGDOT,
            PERIGE,
            PINVSQ,
            PIX2,
            PL,
            PRELIM,
            PSISQ,
            QOMS24,
            R,
            RDOT,
            RDOTK,
            RFDOT,
            RFDOTK,
            RK,
            S4,
            SIN2U,
            SINEPW,
            SINIK,
            SINIO,
            SINMO,
            SINNOK,
            SINU,
            SINUK,
            T2COF,
            T3COF,
            T4COF,
            T5COF,
            TCUBE,
            TEMP,
            TEMP1,
            TEMP2,
            TEMP3,
            TEMP4,
            TEMP5,
            TEMP6,
            TEMPA,
            TEMPE,
            TEMPL,
            TFOUR,
            THETA2,
            THETA4,
            TOKM,
            TOKMPS,
            TSI,
            TSINCE,
            TSQ,
            U,
            UK,
            UPPER,
            UX,
            UY,
            UZ,
            VX,
            VY,
            VZ,
            X1M5TH,
            X1MTH2,
            X3THM1,
            X7THM1,
            XHDOT1,
            XINCK,
            XL,
            XLCOF,
            XLL,
            XLT,
            XMCOF,
            XMDF,
            XMDOT,
            XMP,
            XMX,
            XMY,
            XN,
            XNODCF,
            XNODDF,
            XNODE,
            XNODEK,
            XNODOT,
            XNODP,
            BEFORE,
            COUNT,
            AFTER,
            HEAD,
            I,
            LAST,
            LIST,
            N,
            DOINIT,
            NEWGEO,
            RECOG,
            UNREC,
        }
    }
}

/// Evaluate "two-line" element data, near-earth
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine EVSGP4. *ALL* TLE evaluations should use that routine.
/// NAIF supports EV2LIN only for backward compatibility.
///
/// Evaluate NORAD two-line element data for near-earth, earth
/// orbiting spacecraft (that is spacecraft with orbital periods
/// less-than 225 minutes). This evaluator uses algorithms as
/// described in Hoots 1980 \[1].
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Epoch in seconds past ephemeris epoch J2000.
///  GEOPHS     I   Geophysical constants
///  ELEMS      I   Two-line element data
///  STATE      O   Evaluated state
///  NMODL      P   Parameter controlling number of buffered elements.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch in TDB seconds past ephemeris epoch J2000
///           at which a state should be produced from the
///           input elements.
///
///  GEOPHS   is a collection of 8 geophysical constants needed
///           for computing a state. The order of these
///           constants must be:
///
///              GEOPHS(1) = J2 gravitational harmonic for Earth.
///              GEOPHS(2) = J3 gravitational harmonic for Earth.
///              GEOPHS(3) = J4 gravitational harmonic for Earth.
///
///           These first three constants are dimensionless.
///
///              GEOPHS(4) = KE: Square root of the GM for Earth where
///                          GM is expressed in Earth radii cubed per
///                          minutes squared.
///
///              GEOPHS(5) = QO: High altitude bound for atmospheric
///                          model in km.
///
///              GEOPHS(6) = SO: Low altitude bound for atmospheric
///                          model in km.
///
///              GEOPHS(7) = RE: Equatorial radius of the earth in km.
///
///              GEOPHS(8) = AE: Distance units/earth radius
///                          (normally 1)
///
///           Below are currently recommended values for these
///           items:
///
///              J2 =    1.082616D-3
///              J3 =   -2.53881D-6
///              J4 =   -1.65597D-6
///
///           The next item is the square root of GM for the Earth
///           given in units of earth-radii**1.5/Minute
///
///              KE =    7.43669161D-2
///
///           The next two items define the top and bottom of the
///           atmospheric drag model used by the type 10 ephemeris
///           type. Don't adjust these unless you understand the full
///           implications of such changes.
///
///              QO =  120.0D0
///              SO =   78.0D0
///
///           The ER value is the equatorial radius in km of the Earth
///           as used by NORAD.
///
///              ER = 6378.135D0
///
///           The value of AE is the number of distance units per
///           Earth radii used by the NORAD state propagation
///           software. The value should be 1 unless you've got a very
///           good understanding of the NORAD routine SGP4 and the
///           affect of changing this value.
///
///              AE =    1.0D0
///
///  ELEMS    is an array containing two-line element data
///           as prescribed below. The elements NDD6O and BSTAR
///           must already be scaled by the proper exponent stored
///           in the two line elements set. Moreover, the
///           various items must be converted to the units shown
///           here.
///
///              ELEMS (  1 ) = NDT20 in radians/minute**2
///              ELEMS (  2 ) = NDD60 in radians/minute**3
///              ELEMS (  3 ) = BSTAR
///              ELEMS (  4 ) = INCL  in radians
///              ELEMS (  5 ) = NODE0 in radians
///              ELEMS (  6 ) = ECC
///              ELEMS (  7 ) = OMEGA in radians
///              ELEMS (  8 ) = M0    in radians
///              ELEMS (  9 ) = N0    in radians/minute
///              ELEMS ( 10 ) = EPOCH of the elements in seconds
///                             past ephemeris epoch J2000.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STATE    is the state produced by evaluating the input elements
///           at the input epoch ET. Units are km and km/sec relative
///           to the TEME reference frame.
/// ```
///
/// # Parameters
///
/// ```text
///  NMODL    is a parameter that controls how many element sets
///           can be buffered internally. Since there are a lot
///           of computations that are independent of time these
///           are buffered and only computed if an unbuffered
///           model is supplied. This value should always
///           be at least 2. Increasing it a great deal is not
///           advised since the time needed to search the
///           buffered elements for a match increases linearly
///           with the NMODL. Empirically, 6 seems to be a good
///           break even value for NMODL.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  No checks are made on the reasonableness of the inputs.
///
///  2)  If the number of iterations for the calculation exceeds the
///      limit for this value, the error SPICE(ITERATIONEXCEEDED) is
///      signaled. This error should signal only for bad (nonphysical)
///      TLEs.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine evaluates NORAD two-line element sets for
///  near-earth orbiting satellites using the algorithms described in
///  Hoots 1980 [1]. This code is an adaptation of
///  the NORAD routine SGP4 to eliminate common blocks, allow
///  buffering of models and intermediate parameters and double
///  precision calculations.
///
///  Near earth is as an orbital period of less than 225
///  minutes.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose you have a set of two-line elements for the LUME 1
///     cubesat. This example shows how you can use this routine
///     together with the routine GETELM to propagate a state to an
///     epoch of interest.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ev2lin_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name           Contents
///           ---------           ------------------------------------
///           naif0012.tls        Leapseconds
///           geophysical.ker     geophysical constants for evaluation
///                               of two-line element sets.
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'naif0012.tls',
///                               'geophysical.ker'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM EV2LIN_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         TIMSTR
///           PARAMETER           ( TIMSTR = '2020-05-26 02:25:00' )
///
///           INTEGER               PNAMLN
///           PARAMETER           ( PNAMLN = 2  )
///
///           INTEGER               TLELLN
///           PARAMETER           ( TLELLN = 69 )
///
///     C
///     C     The LUME-1 cubesat is an Earth orbiting object; set
///     C     the center ID to the Earth ID.
///     C
///           INTEGER               CENTER
///           PARAMETER           ( CENTER  = 399     )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(PNAMLN)    NOADPN ( 8  )
///           CHARACTER*(TLELLN)    TLE    ( 2  )
///
///           DOUBLE PRECISION      ELEMS  ( 10 )
///           DOUBLE PRECISION      EPOCH
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      GEOPHS ( 8  )
///           DOUBLE PRECISION      STATE  ( 6  )
///
///           INTEGER               I
///           INTEGER               N
///
///     C
///     C     These are the variables that will hold the constants
///     C     required by EV2LIN. These constants are available from
///     C     the loaded PCK file, which provides the actual values
///     C     and units as used by NORAD propagation model.
///     C
///     C        Constant   Meaning
///     C        --------   ------------------------------------------
///     C        J2         J2 gravitational harmonic for Earth.
///     C        J3         J3 gravitational harmonic for Earth.
///     C        J4         J4 gravitational harmonic for Earth.
///     C        KE         Square root of the GM for Earth.
///     C        QO         High altitude bound for atmospheric model.
///     C        SO         Low altitude bound for atmospheric model.
///     C        ER         Equatorial radius of the Earth.
///     C        AE         Distance units/earth radius.
///     C
///           DATA          NOADPN  /  'J2', 'J3', 'J4', 'KE',
///          .                         'QO', 'SO', 'ER', 'AE'  /
///
///     C
///     C     Define the Two-Line Element set for LUME-1.
///     C
///           TLE(1)  = '1 43908U 18111AJ  20146.60805006  .00000806'
///          .      //                   '  00000-0  34965-4 0  9999'
///           TLE(2)  = '2 43908  97.2676  47.2136 0020001 220.6050 '
///          .      //                   '139.3698 15.24999521 78544'
///
///     C
///     C     Load the PCK file that provides the geophysical
///     C     constants required for the evaluation of the two-line
///     C     elements sets. Load also an LSK, as it is required by
///     C     GETELM to perform time conversions. Use a meta-kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'ev2lin_ex1.tm' )
///
///     C
///     C     Retrieve the data from the kernel, and place it on
///     C     the GEOPHS array.
///     C
///           DO I = 1, 8
///
///              CALL BODVCD ( CENTER, NOADPN(I), 1, N, GEOPHS(I) )
///
///           END DO
///
///     C
///     C     Convert the Two Line Elements lines to the element sets.
///     C     Set the lower bound for the years to be the beginning
///     C     of the space age.
///     C
///           CALL GETELM ( 1957, TLE, EPOCH, ELEMS )
///
///     C
///     C     Now propagate the state using EV2LIN to the epoch of
///     C     interest.
///     C
///           CALL STR2ET ( TIMSTR, ET )
///           CALL EV2LIN ( ET, GEOPHS, ELEMS, STATE )
///
///     C
///     C     Display the results.
///     C
///           WRITE(*,'(2A)')       'Epoch   : ', TIMSTR
///           WRITE(*,'(A,3F16.8)') 'Position:', (STATE(I), I=1,3)
///           WRITE(*,'(A,3F16.8)') 'Velocity:', (STATE(I), I=4,6)
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Epoch   : 2020-05-26 02:25:00
///     Position:  -4644.60403398  -5038.95025540   -337.27141117
///     Velocity:     -0.45719025      0.92884817     -7.55917355
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  F. Hoots and R. Roehrich, "Spacetrack Report #3: Models for
///       Propagation of the NORAD Element Sets," U.S. Air Force
///       Aerospace Defense Command, Colorado Springs, CO, 1980.
///
///  [2]  F. Hoots, "Spacetrack Report #6: Models for Propagation of
///       Space Command Element Sets,"  U.S. Air Force Aerospace
///       Defense Command, Colorado Springs, CO, 1986.
///
///  [3]  F. Hoots, P. Schumacher and R. Glover, "History of Analytical
///       Orbit Modeling in the U. S. Space Surveillance System,"
///       Journal of Guidance, Control, and Dynamics. 27(2):174-185,
///       2004.
///
///  [4]  D. Vallado, P. Crawford, R. Hujsak and T. Kelso, "Revisiting
///       Spacetrack Report #3," paper AIAA 2006-6753 presented at the
///       AIAA/AAS Astrodynamics Specialist Conference, Keystone, CO.,
///       August 21-24, 2006.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 01-NOV-2021 (JDR) (EDW)
///
///         Declared routine as deprecated.
///
///         Corrected the description of QO and SO constants in the
///         detailed description of the input argument GEOPHS and the input
///         element names in ELEMS.
///
///         Added Spacetrack Report #3 to literature references.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragments.
///
/// -    SPICELIB Version 1.1.0, 15-SEP-2014 (EDW)
///
///         Added error check to prevent infinite loop in
///         calculation of EST.
///
/// -    SPICELIB Version 1.0.3, 02-JAN-2008 (EDW)
///
///         Corrected error in the calculation of the C4 term
///         identified by Curtis Haase.
///
///         Minor edit to the COEF1 declaration strictly
///         identifying the constant as a double.
///
///         From:
///
///            COEF1  = COEF  / PSISQ**3.5
///
///         To:
///
///            COEF1  = COEF  / PSISQ**3.5D0
///
/// -    SPICELIB Version 1.0.2, 08-JUL-2004 (EDW)
///
///         Corrected error in the calculation of the C2 term.
///         Reordered C1, C2 calculations to avoid division
///         by BSTAR.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1998 (EDW)
///
///         Corrected error in header describing the GEOPHS array.
///
/// -    SPICELIB Version 1.0.0, 14-JAN-1994 (WLT)
/// ```
pub fn ev2lin(
    ctx: &mut SpiceContext,
    et: f64,
    geophs: &[f64],
    elems: &[f64],
    state: &mut [f64],
) -> crate::Result<()> {
    EV2LIN(et, geophs, elems, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EV2LIN ( Evaluate "two-line" element data, near-earth )
pub fn EV2LIN(
    ET: f64,
    GEOPHS: &[f64],
    ELEMS: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let GEOPHS = DummyArray::new(GEOPHS, 1..);
    let ELEMS = DummyArray::new(ELEMS, 1..);
    let mut STATE = DummyArrayMut::new(STATE, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    //
    // The following parameters give the location of the various
    // geophysical parameters needed for the two line element
    // sets.
    //
    // KJ2  --- location of J2
    // KJ3  --- location of J3
    // KJ4  --- location if J4
    // KKE  --- location of KE = sqrt(GM) in earth-radii**1.5/MIN
    // KQO  --- upper bound of atmospheric model in KM
    // KSO  --- lower bound of atmospheric model in KM
    // KER  --- earth equatorial radius in KM.
    // KAE  --- distance units/earth radius
    //
    //

    //
    // An enumeration of the various components of the
    // elements array---ELEMS
    //
    // KNDT20
    // KNDD60
    // KBSTAR
    // KINCL
    // KNODE0
    // KECC
    // KOMEGA
    // KMO
    // KNO
    //

    //
    // The parameters NEXT and PREV are used in our linked list
    // LIST(NEXT,I) points to the list item the occurs after
    // list item I.  LIST ( PREV, I ) points to the list item
    // that precedes list item I.
    // NEXT
    // PREV
    //

    //
    // There are a number of preliminary quantities that are needed
    // to compute the state.  Those that are not time dependent and
    // depend only upon the elements are stored in a buffer so that
    // if an element set matches a saved set, these preliminary
    // quantities  will not be recomputed.  PRSIZE is the parameter
    // used to declare the needed room.
    //
    //
    // When we perform bisection in the solution of Kepler's equation
    // we don't want to bisect too many times.
    //

    //
    // Numerical Constants
    //

    //
    // Local variables
    //
    // Geophysical Quantities
    //

    //
    // Elements
    //

    //
    // Intermediate quantities. The time independent quantities
    // are calculated only as new elements come into the routine.
    //

    CHKIN(b"EV2LIN", ctx)?;

    //
    // Rather than always making function calls we store the
    // values of the PI dependent constants the first time
    // through the routine.
    //
    if save.DOINIT {
        save.DOINIT = false;
        save.PIX2 = TWOPI(ctx);

        {
            let m1__: i32 = 1;
            let m2__: i32 = NGEOCN;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.LSTGEO[save.I] = 0.0;
                save.I += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = NMODL;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                for J in 1..=NELEMS {
                    save.ELEMNT[[J, save.I]] = 0.0;
                }
                save.I += m3__;
            }
        }

        //
        // Set up our doubly linked list of most recently used
        // models.  Here's how things are supposed to be arranged:
        //
        // LIST(NEXT,I)   points to the ephemeris model that was used
        //                most recently after ephemeris model I.
        // LIST(PREV,I)   points to the latest ephemeris model used
        //                that was used more recently than I.
        //
        // HEAD           points to the most recently used ephemeris
        //                model.
        //
        //
        save.HEAD = 1;

        save.LIST[[PREV, save.HEAD]] = NIL;
        save.LIST[[NEXT, 1]] = 2;

        {
            let m1__: i32 = 2;
            let m2__: i32 = (NMODL - 1);
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.LIST[[NEXT, save.I]] = (save.I + 1);
                save.LIST[[PREV, save.I]] = (save.I - 1);
                save.I += m3__;
            }
        }

        save.LIST[[NEXT, NMODL]] = NIL;
        save.LIST[[PREV, NMODL]] = (NMODL - 1);
    }
    //
    // We update the geophysical parameters only if there
    // has been a change from the last time they were
    // supplied.
    //
    if ((((((((save.LSTGEO[KAE] != GEOPHS[KAE]) || (save.LSTGEO[KER] != GEOPHS[KER]))
        || (save.LSTGEO[KJ2] != GEOPHS[KJ2]))
        || (save.LSTGEO[KJ3] != GEOPHS[KJ3]))
        || (save.LSTGEO[KJ4] != GEOPHS[KJ4]))
        || (save.LSTGEO[KKE] != GEOPHS[KKE]))
        || (save.LSTGEO[KQO] != GEOPHS[KQO]))
        || (save.LSTGEO[KSO] != GEOPHS[KSO]))
    {
        {
            let m1__: i32 = 1;
            let m2__: i32 = NGEOCN;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.LSTGEO[save.I] = GEOPHS[save.I];
                save.I += m3__;
            }
        }

        save.J2 = GEOPHS[KJ2];
        save.J3 = GEOPHS[KJ3];
        save.J4 = GEOPHS[KJ4];
        save.KE = GEOPHS[KKE];
        save.QO = GEOPHS[KQO];
        save.SO = GEOPHS[KSO];
        save.ER = GEOPHS[KER];
        save.AE = GEOPHS[KAE];

        save.AE2 = (save.AE * save.AE);
        save.AE3 = (save.AE * save.AE2);
        save.AE4 = (save.AE * save.AE3);
        save.CK2 = ((HALF * save.J2) * save.AE2);
        save.A3OVK2 = -(((2.0 * save.J3) * save.AE) / save.J2);
        save.CK4 = -((F3OV8 * save.J4) * save.AE4);
        save.QOMSO = (save.QO - save.SO);
        save.Q1 = ((save.QOMSO * save.AE) / save.ER);
        save.Q2 = (save.Q1 * save.Q1);
        save.QOMS2T = (save.Q2 * save.Q2);
        save.S = (save.AE * (ONE + (save.SO / save.ER)));
        //
        // When we've finished up we will need to convert everything
        // back to KM and KM/SEC  the two variables below give the
        // factors we shall need to do this.
        //
        save.TOKM = (save.ER / save.AE);
        save.TOKMPS = (save.TOKM / 60.0);

        save.NEWGEO = true;
    } else {
        save.NEWGEO = false;
    }

    //
    // Fetch all of the pieces of this model.
    //
    save.EPOCH = ELEMS[KEPOCH];
    save.XNDT2O = ELEMS[KNDT20];
    save.XNDD6O = ELEMS[KNDD60];
    save.BSTAR = ELEMS[KBSTAR];
    save.XINCL = ELEMS[KINCL];
    save.XNODEO = ELEMS[KNODE0];
    save.EO = ELEMS[KECC];
    save.OMEGAO = ELEMS[KOMEGA];
    save.XMO = ELEMS[KMO];
    save.XNO = ELEMS[KNO];

    //
    // See if this model is already buffered, start at the first
    // model in the list (the most recently used model).
    //

    save.UNREC = true;
    save.N = save.HEAD;

    while ((save.N != NIL) && save.UNREC) {
        //
        // The actual order of the elements is such that we can
        // usually tell that a stored model is different from
        // the one under consideration by looking at the
        // end of the list first.  Hence we start with I = NELEMS
        // and decrement I until we have looked at everything
        // or found a mismatch.
        //
        save.RECOG = true;
        save.I = NELEMS;

        while (save.RECOG && (save.I > 0)) {
            save.RECOG = (save.RECOG && (save.ELEMNT[[save.I, save.N]] == ELEMS[save.I]));
            save.I = (save.I - 1);
        }

        save.UNREC = !save.RECOG;

        if save.UNREC {
            save.LAST = save.N;
            save.N = save.LIST[[NEXT, save.N]];
        }
    }

    if (save.N == NIL) {
        save.N = save.LAST;
    }
    //
    // Either N points to a recognized item or it points to the
    // tail of the list where the least recently used items is
    // located.  In either case N must be made the head of the
    // list.  (If it is already the head of the list we don't
    // have to bother with anything.)
    //
    if (save.N != save.HEAD) {
        //
        // Find the items that come before and after N and
        // link them together.
        //
        save.BEFORE = save.LIST[[PREV, save.N]];
        save.AFTER = save.LIST[[NEXT, save.N]];

        save.LIST[[NEXT, save.BEFORE]] = save.AFTER;

        if (save.AFTER != NIL) {
            save.LIST[[PREV, save.AFTER]] = save.BEFORE;
        }
        //
        // Now the guy that will come after N is the current
        // head of the list.  N will have no predecessor.
        //
        save.LIST[[NEXT, save.N]] = save.HEAD;
        save.LIST[[PREV, save.N]] = NIL;
        //
        // The predecessor the current head of the list becomes N
        //
        save.LIST[[PREV, save.HEAD]] = save.N;
        //
        // and finally, N becomes the head of the list.
        //
        save.HEAD = save.N;
    }

    if (save.RECOG && !save.NEWGEO) {
        //
        // We can just look up the intermediate values from
        // computations performed on a previous call to this
        // routine.
        //
        save.AODP = save.PRELIM[[1, save.N]];
        save.AYCOF = save.PRELIM[[2, save.N]];
        save.C1 = save.PRELIM[[3, save.N]];
        save.C4 = save.PRELIM[[4, save.N]];
        save.C5 = save.PRELIM[[5, save.N]];
        save.COSIO = save.PRELIM[[6, save.N]];
        save.D2 = save.PRELIM[[7, save.N]];
        save.D3 = save.PRELIM[[8, save.N]];
        save.D4 = save.PRELIM[[9, save.N]];
        save.DELMO = save.PRELIM[[10, save.N]];
        save.ETA = save.PRELIM[[11, save.N]];
        save.OMGCOF = save.PRELIM[[12, save.N]];
        save.OMGDOT = save.PRELIM[[13, save.N]];
        save.PERIGE = save.PRELIM[[14, save.N]];
        save.SINIO = save.PRELIM[[15, save.N]];
        save.SINMO = save.PRELIM[[16, save.N]];
        save.T2COF = save.PRELIM[[17, save.N]];
        save.T3COF = save.PRELIM[[18, save.N]];
        save.T4COF = save.PRELIM[[19, save.N]];
        save.T5COF = save.PRELIM[[20, save.N]];
        save.X1MTH2 = save.PRELIM[[21, save.N]];
        save.X3THM1 = save.PRELIM[[22, save.N]];
        save.X7THM1 = save.PRELIM[[23, save.N]];
        save.XLCOF = save.PRELIM[[24, save.N]];
        save.XMCOF = save.PRELIM[[25, save.N]];
        save.XMDOT = save.PRELIM[[26, save.N]];
        save.XNODCF = save.PRELIM[[27, save.N]];
        save.XNODOT = save.PRELIM[[28, save.N]];
        save.XNODP = save.PRELIM[[29, save.N]];
    } else {
        //
        // Compute all of the intermediate items needed.
        // First, the inclination dependent constants.
        //
        save.COSIO = f64::cos(save.XINCL);
        save.SINIO = f64::sin(save.XINCL);
        save.THETA2 = (save.COSIO * save.COSIO);

        save.THETA4 = (save.THETA2 * save.THETA2);
        save.X3THM1 = ((3.0 * save.THETA2) - ONE);
        save.X7THM1 = ((7.0 * save.THETA2) - ONE);

        save.X1MTH2 = (ONE - save.THETA2);
        save.X1M5TH = (ONE - (5.0 * save.THETA2));
        //
        // Eccentricity dependent constants
        //
        save.BETAO = f64::sqrt((ONE - (save.EO * save.EO)));
        save.BETAO2 = (ONE - (save.EO * save.EO));
        save.BETAO3 = (save.BETAO * save.BETAO2);
        save.BETAO4 = (save.BETAO2 * save.BETAO2);
        //
        // Semi-major axis and ascending node related constants.
        //
        save.A1 = f64::powf((save.KE / save.XNO), TOTHRD);
        save.DEL1 = (((1.5 * save.CK2) * save.X3THM1) / ((save.A1 * save.A1) * save.BETAO3));

        save.AO = (save.A1
            * (ONE - (save.DEL1 * (ONETHD + (save.DEL1 * (ONE + ((save.DEL1 * 134.0) / 81.0)))))));

        save.DELO = (((1.5 * save.CK2) * save.X3THM1) / ((save.AO * save.AO) * save.BETAO3));

        save.XNODP = (save.XNO / (ONE + save.DELO));
        save.AODP = (save.AO / (ONE - save.DELO));

        save.S4 = save.S;
        save.QOMS24 = save.QOMS2T;
        save.PERIGE = (save.ER * ((save.AODP * (ONE - save.EO)) - save.AE));

        //
        // For perigee below 156 km, the values of S and QOMS2T are
        // altered.
        //
        if (save.PERIGE < 156.0) {
            save.S4 = (save.PERIGE - 78.0);

            if (save.PERIGE <= 98.0) {
                save.S4 = 20.0;
            }

            save.QOMS24 = f64::powi((((120.0 - save.S4) * save.AE) / save.ER), 4);
            save.S4 = (save.AE + (save.S4 / save.ER));
        }
        //
        // The next block is simply a pretty print of the code in
        // sgp4 from label number 10 through the label 90.
        //
        save.PINVSQ = (ONE / ((save.AODP * save.AODP) * save.BETAO4));
        save.TSI = (ONE / (save.AODP - save.S4));

        save.ETA = ((save.AODP * save.EO) * save.TSI);
        save.ETASQ = (save.ETA * save.ETA);
        save.EETA = (save.EO * save.ETA);
        save.COEF = (save.QOMS24 * f64::powi(save.TSI, 4));

        save.PSISQ = f64::abs((ONE - save.ETASQ));

        save.COEF1 = (save.COEF / f64::powf(save.PSISQ, 3.5));
        save.C2 = ((save.COEF1 * save.XNODP)
            * ((save.AODP * ((ONE + (F3OV2 * save.ETASQ)) + (save.EETA * (4.0 + save.ETASQ))))
                + ((((0.75 * save.CK2) * (save.TSI / save.PSISQ)) * save.X3THM1)
                    * (8.0 + (save.ETASQ * (24.0 + (3.0 * save.ETASQ)))))));

        save.C1 = (save.C2 * save.BSTAR);

        save.C3 = ((((((save.COEF * save.TSI) * save.A3OVK2) * save.XNODP) * save.AE)
            * save.SINIO)
            / save.EO);

        save.C4 = (((((2.0 * save.XNODP) * save.COEF1) * save.AODP) * save.BETAO2)
            * (((save.ETA * (2.0 + (0.5 * save.ETASQ)))
                + (save.EO * (HALF + (2.0 * save.ETASQ))))
                - ((2.0 * ((save.CK2 * save.TSI) / (save.AODP * save.PSISQ)))
                    * (-((3.0 * save.X3THM1)
                        * ((ONE - (save.EETA * 2.0))
                            + (save.ETASQ * (F3OV2 - (HALF * save.EETA)))))
                        + (((0.75 * f64::cos((2.0 * save.OMEGAO))) * save.X1MTH2)
                            * ((save.ETASQ * 2.0) - (save.EETA * (ONE + save.ETASQ))))))));

        save.C5 = ((((2.0 * save.COEF1) * save.AODP) * save.BETAO2)
            * ((ONE + (2.75 * (save.ETASQ + save.EETA))) + (save.EETA * save.ETASQ)));

        save.TEMP1 = (((3.0 * save.CK2) * save.PINVSQ) * save.XNODP);
        save.TEMP2 = ((save.TEMP1 * save.CK2) * save.PINVSQ);
        save.TEMP3 = ((((1.25 * save.CK4) * save.PINVSQ) * save.PINVSQ) * save.XNODP);

        save.XMDOT = ((save.XNODP + (((HALF * save.TEMP1) * save.BETAO) * save.X3THM1))
            + (((0.0625 * save.TEMP2) * save.BETAO)
                * ((13.0 - (78.0 * save.THETA2)) + (137.0 * save.THETA4))));

        save.OMGDOT = ((-((HALF * save.TEMP1) * save.X1M5TH)
            + ((0.0625 * save.TEMP2) * ((7.0 - (114.0 * save.THETA2)) + (395.0 * save.THETA4))))
            + (save.TEMP3 * ((3.0 - (36.0 * save.THETA2)) + (49.0 * save.THETA4))));

        save.XHDOT1 = -(save.TEMP1 * save.COSIO);

        save.XNODOT = (save.XHDOT1
            + (save.COSIO
                * (((HALF * save.TEMP2) * (4.0 - (19.0 * save.THETA2)))
                    + ((2.0 * save.TEMP3) * (3.0 - (7.0 * save.THETA2))))));

        save.OMGCOF = ((save.BSTAR * save.C3) * f64::cos(save.OMEGAO));
        save.XMCOF = -((((save.BSTAR * TOTHRD) * save.COEF) * save.AE) / save.EETA);
        save.XNODCF = (((3.5 * save.BETAO2) * save.XHDOT1) * save.C1);
        save.T2COF = (F3OV2 * save.C1);
        save.AYCOF = ((0.25 * save.A3OVK2) * save.SINIO);
        save.XLCOF = (((HALF * save.AYCOF) * (3.0 + (5.0 * save.COSIO))) / (ONE + save.COSIO));

        save.DELMO = f64::powi((ONE + (save.ETA * f64::cos(save.XMO))), 3);
        save.SINMO = f64::sin(save.XMO);
        //
        // For perigee less than 220 kilometers, the ISIMP flag is set
        // and the equations are truncated to linear variation in SQRT
        // A and quadratic variation in mean anomaly.  Also, the C3
        // term, the Delta OMEGA term, and the Delta M term are
        // dropped.  (Note: Normally we would just use
        //
        if (save.PERIGE >= 220.0) {
            save.C1SQ = (save.C1 * save.C1);
            save.D2 = (((4.0 * save.TSI) * save.C1SQ) * save.AODP);
            save.TEMP = (((save.D2 * save.TSI) * save.C1) * ONETHD);
            save.D3 = (save.TEMP * (save.S4 + (17.0 * save.AODP)));
            save.D4 = (((((save.TEMP * save.TSI) * save.C1) * save.AODP) * HALF)
                * ((221.0 * save.AODP) + (31.0 * save.S4)));

            save.T3COF = (save.D2 + (2.0 * save.C1SQ));

            save.T4COF =
                (0.25 * ((3.0 * save.D3) + (save.C1 * ((12.0 * save.D2) + (10.0 * save.C1SQ)))));

            save.T5COF = (0.2
                * ((((3.0 * save.D4) + ((12.0 * save.C1) * save.D3))
                    + ((6.0 * save.D2) * save.D2))
                    + ((15.0 * save.C1SQ) * ((2.0 * save.D2) + save.C1SQ))));
        }
        //
        // Now store the intermediate computations so that if we
        // should hit this model again we can just look up the needed
        // results from the above computations.
        //
        save.PRELIM[[1, save.N]] = save.AODP;
        save.PRELIM[[2, save.N]] = save.AYCOF;
        save.PRELIM[[3, save.N]] = save.C1;
        save.PRELIM[[4, save.N]] = save.C4;
        save.PRELIM[[5, save.N]] = save.C5;
        save.PRELIM[[6, save.N]] = save.COSIO;
        save.PRELIM[[7, save.N]] = save.D2;
        save.PRELIM[[8, save.N]] = save.D3;
        save.PRELIM[[9, save.N]] = save.D4;
        save.PRELIM[[10, save.N]] = save.DELMO;
        save.PRELIM[[11, save.N]] = save.ETA;
        save.PRELIM[[12, save.N]] = save.OMGCOF;
        save.PRELIM[[13, save.N]] = save.OMGDOT;
        save.PRELIM[[14, save.N]] = save.PERIGE;
        save.PRELIM[[15, save.N]] = save.SINIO;
        save.PRELIM[[16, save.N]] = save.SINMO;
        save.PRELIM[[17, save.N]] = save.T2COF;
        save.PRELIM[[18, save.N]] = save.T3COF;
        save.PRELIM[[19, save.N]] = save.T4COF;
        save.PRELIM[[20, save.N]] = save.T5COF;
        save.PRELIM[[21, save.N]] = save.X1MTH2;
        save.PRELIM[[22, save.N]] = save.X3THM1;
        save.PRELIM[[23, save.N]] = save.X7THM1;
        save.PRELIM[[24, save.N]] = save.XLCOF;
        save.PRELIM[[25, save.N]] = save.XMCOF;
        save.PRELIM[[26, save.N]] = save.XMDOT;
        save.PRELIM[[27, save.N]] = save.XNODCF;
        save.PRELIM[[28, save.N]] = save.XNODOT;
        save.PRELIM[[29, save.N]] = save.XNODP;
        //
        // Finally, move these elements in the storage area
        // for checking the next time through.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = NELEMS;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.ELEMNT[[save.I, save.N]] = ELEMS[save.I];
                save.I += m3__;
            }
        }
    }
    //
    // Now that all of the introductions are out of the way
    // we can get down to business.
    //
    // Compute the time since the epoch for this model.
    //
    save.TSINCE = (ET - save.EPOCH);
    //
    // and convert it to minutes
    //
    save.TSINCE = (save.TSINCE / 60.0);

    save.XMDF = (save.XMO + (save.XMDOT * save.TSINCE));
    save.OMGADF = (save.OMEGAO + (save.OMGDOT * save.TSINCE));
    save.XNODDF = (save.XNODEO + (save.XNODOT * save.TSINCE));
    save.OMEGA = save.OMGADF;
    save.XMP = save.XMDF;

    save.TSQ = (save.TSINCE * save.TSINCE);
    save.XNODE = (save.XNODDF + (save.XNODCF * save.TSQ));
    save.TEMPA = (1.0 - (save.C1 * save.TSINCE));
    save.TEMPE = ((save.BSTAR * save.C4) * save.TSINCE);
    save.TEMPL = (save.T2COF * save.TSQ);

    if (save.PERIGE > 220.0) {
        save.TCUBE = (save.TSQ * save.TSINCE);
        save.TFOUR = (save.TCUBE * save.TSINCE);

        save.DELOMG = (save.OMGCOF * save.TSINCE);
        save.DELM =
            (save.XMCOF * (f64::powi((1.0 + (save.ETA * f64::cos(save.XMDF))), 3) - save.DELMO));
        save.TEMP = (save.DELOMG + save.DELM);
        save.XMP = (save.XMDF + save.TEMP);
        save.OMEGA = (save.OMGADF - save.TEMP);

        save.TEMPA = (((save.TEMPA - (save.D2 * save.TSQ)) - (save.D3 * save.TCUBE))
            - (save.D4 * save.TFOUR));

        save.TEMPE = (save.TEMPE + ((save.BSTAR * save.C5) * (f64::sin(save.XMP) - save.SINMO)));
        save.TEMPL = ((save.TEMPL + (save.TCUBE * save.T3COF))
            + (save.TFOUR * (save.T4COF + (save.TSINCE * save.T5COF))));
    }

    save.A = (save.AODP * f64::powi(save.TEMPA, 2));
    save.XL = (((save.XMP + save.OMEGA) + save.XNODE) + (save.XNODP * save.TEMPL));

    save.E = (save.EO - save.TEMPE);

    //
    // The parameter BETA used to be needed, but it's only use
    // was in the computation of TEMP below where it got squared
    // so we'll remove it from the list of things to compute.
    //
    // BETA =  DSQRT( 1.0D0 - E*E )
    //

    save.XN = (save.KE / f64::powf(save.A, 1.5));

    //
    // Long period periodics
    //
    save.TEMP = (1.0 / (save.A * (1.0 - (save.E * save.E))));
    save.AYNL = (save.TEMP * save.AYCOF);
    save.AYN = ((save.E * f64::sin(save.OMEGA)) + save.AYNL);
    save.AXN = (save.E * f64::cos(save.OMEGA));

    save.XLL = ((save.TEMP * save.XLCOF) * save.AXN);
    save.XLT = (save.XL + save.XLL);

    //
    // Solve Kepler's equation.
    //
    // We are going to solve for the roots of this equation by
    // using a mixture of Newton's method and the prescription for
    // root finding outlined in the SPICE routine UNITIM.
    //
    // We are going to solve the equation
    //
    //       U = EPW - AXN * SIN(EPW)  +  AYN * COS(EPW)
    //
    // Where
    //
    //    AYN  = E    * SIN(OMEGA)   +    AYNL
    //    AXN  = E    * COS(OMEGA)
    //
    // And
    //
    //    AYNL =  -0.50D0  * SINIO * AE * J3   / (J2*A*(1.0D0 - E*E))
    //
    // Since this is a low earth orbiter (period less than 225 minutes)
    // The maximum value E can take (without having the orbiter
    // plowing fields) is approximately 0.47 and AYNL will not be
    // more than about .01.  ( Typically E will be much smaller
    // on the order of about .1 )  Thus we can initially
    // view the problem of solving the equation for EPW as a
    // function of the form
    //
    // U = EPW + F ( EPW )                                   (1)
    //
    // Where F( EPW ) = -AXN*SIN(EPW) + AYN*COS(EPW)
    //
    // Note that | F'(EPW) | < M =  DSQRT( AXN**2 + AYN**2 ) < 0.48
    //
    // From the above discussion it is evident that F is a contraction
    // mapping.  So that we can employ the same techniques as were
    // used in the routine UNITIM to get our first approximations of
    // the root.  Once we have some good first approximations, we
    // will speed up the root finding by using Newton's method for
    // finding a zero of a function.  The function we will work on
    // is
    //
    //    f  (x) = x - U - AXN*SIN(x) + AYN*COS(x)         (2)
    //
    // By applying Newton's method we will go from linear to
    // quadratic convergence.
    //
    // We will keep track of our error bounds along the way so
    // that we will know how many iterations to perform in each
    // phase of the root extraction.
    //
    // few steps using bisection.
    //
    //
    // For the benefit of those interested
    // here's the basics of what we'll do.
    //
    //    Whichever EPW satisfies equation (1) will be
    //    unique. The uniqueness of the solution is ensured because the
    //    expression on the right-hand side of the equation is
    //    monotone increasing in EPW.
    //
    //    Let's suppose that EPW is the solution, then the following
    //    is true.
    //
    //       EPW = U - F(EPW)
    //
    //    but we can also replace the EPW on the right hand side of the
    //    equation by U - F(EPW).  Thus
    //
    //       EPW = U - F( U - F(EPW))
    //
    //           = U - F( U - F( U - F(EPW)))
    //
    //           = U - F( U - F( U - F( U - F(EPW))))
    //
    //           = U - F( U - F( U - F( U - F( U - F(EPW)))))
    //           .
    //           .
    //           .
    //           = U - F( U - F( U - F( U - F( U - F(U - ... )))
    //
    //    and so on, for as long as we have patience to perform the
    //    substitutions.
    //
    //    The point of doing this recursive substitution is that we
    //    hope to move EPW to an insignificant part of the computation.
    //    This would seem to have a reasonable chance of success since
    //    F is a bounded and has a small derivative.
    //
    //    Following this idea, we will attempt to solve for EPW using
    //    the recursive method outlined below.
    //
    // We will make our first guess at EPW, call it EPW_0.
    //
    //    EPW_0 = U
    //
    // Our next guess, EPW_1, is given by:
    //
    //    EPW_1 = U - F(EPW_0)
    //
    // And so on:
    //
    //    EPW_2 = U - F(EPW_1)        [ = U - F(U - F(U))          ]
    //    EPW_3 = U - F(EPW_2)        [ = U - F(U - F(U - F(U)))   ]
    //       .
    //       .
    //       .
    //    EPW_n = U - F(EPW_(n-1))    [ = U - F(U - F(U - F(U...)))]
    //
    //    The questions to ask at this point are:
    //
    //       1) Do the EPW_i's converge?
    //       2) If they converge, do they converge to EPW?
    //       3) If they converge to EPW, how fast do they get there?
    //
    //    1) The sequence of approximations converges.
    //
    //       | EPW_n - EPW_(n-1) | =  [ U - F( EPW_(n-1) ) ]
    //                             -  [ U - F( EPW_(n-2) ) ]
    //
    //                             =  [ F( EPW_(n-2) ) - F( EPW_(n-1)) ]
    //
    // The function F has an important property. The absolute
    // value of its derivative is always less than M.
    // This means that for any pair of real numbers s,t
    //
    //    | F(t) - F(s) |  < M*| t - s |.
    //
    // From this observation, we can see that
    //
    //    | EPW_n - EPW_(n-1) | < M*| EPW_(n-1) - EPW_(n-2) |
    //
    // With this fact available, we could (with a bit more work)
    // conclude that the sequence of EPW_i's converges and that
    // it converges at a rate that is at least as fast as the
    // sequence M, M**2, M**3.  In fact the difference
    //    |EPW - EPW_N| < M/(1-M) * | EPW_N - EPW_(N-1) |
    //
    //                   < M/(1-M) * M**N | EPW_1 - EPW_0 |
    //
    // 2) If we let EPW be the limit of the EPW_i's then it follows
    //    that
    //
    //           EPW = U - F(EPW).
    //
    //
    // or that
    //
    //           U = EPW + F(EPW).
    //
    // We will use this technique to get an approximation that
    // is within a tolerance of EPW and then switch to
    // a Newton's method. (We'll compute the tolerance using
    // the value of M given above).
    //
    //
    // For the Newton's method portion of the problem, recall
    // from Taylor's formula that:
    //
    //    f(x) = f(x_0) + f'(x_0)(x-x_0) +  f''(c)/2 (x-x_0)**2
    //
    // for some c between x and x_0
    //
    // If x happens to be a zero of f then we can rearrange the
    // terms above to get
    //
    //                   f(x_0)       f''(c)
    //       x = x_0 -   -------  +  -------- ( x - x_0)**2
    //                   f'(x_0)      f'(x_0)
    //
    // Thus the error in the Newton approximation
    //
    //
    //                   f(x_0)
    //       x = x_0  -  -------
    //                   f'(x_0)
    //
    // is
    //
    //                 f''(c)
    //                -------- ( x - x_0)**2
    //                 f'(x_0)
    //
    // Thus if we can bound f'' and pick a good first
    // choice for x_0 (using the first method outlined
    // above we can get quadratic convergence.)
    //
    // In our case we have
    //
    //    f  (x) = x - U - AXN*SIN(x) + AYN*COS(x)
    //    f' (x) = 1     - AXN*COS(x) - AYN*SIN(x)
    //    f''(x) =         AXN*SIN(x) - AYN*COS(x)
    //
    // So that:
    //
    //    f' (x) >  1 - M
    //
    //    f''(x) <  M
    //
    // Thus the error in the Newton's approximation is
    // at most
    //
    //    M/(1-M) * ( x - x_0 )**2
    //
    // Thus as long as our original estimate (determined using
    // the contraction method) gets within a reasonable tolerance
    // of x, we can use Newton's method to achieve faster
    // convergence.
    //

    save.M = f64::sqrt(((save.AXN * save.AXN) + (save.AYN * save.AYN)));
    save.MOV1M = f64::abs((save.M / (1.0 - save.M)));

    save.FMOD2P = intrinsics::DMOD((save.XLT - save.XNODE), save.PIX2);

    if (save.FMOD2P < 0 as f64) {
        save.FMOD2P = (save.FMOD2P + save.PIX2);
    }

    save.CAPU = save.FMOD2P;
    save.EPW = save.CAPU;
    save.EST = 1.0;

    save.COUNT = 0;

    while (save.EST > 0.125) {
        save.COUNT = (save.COUNT + 1);

        if (save.COUNT > MXLOOP) {
            SETMSG(b"EST iteration count of #1 exceeded at time ET #2. This error may indicate a bad TLE set.", ctx);
            ERRINT(b"#1", MXLOOP, ctx);
            ERRDP(b"#2", ET, ctx);
            SIGERR(b"SPICE(ITERATIONEXCEEDED)", ctx)?;
            CHKOUT(b"EV2LIN", ctx)?;
            return Ok(());
        }

        save.EPWNXT =
            ((save.CAPU - (save.AXN * f64::sin(save.EPW))) + (save.AYN * f64::cos(save.EPW)));
        save.EST = (save.MOV1M * f64::abs((save.EPWNXT - save.EPW)));
        save.EPW = save.EPWNXT;
    }

    //
    // We need to be able to add something to EPW and not
    // get EPW (but not too much).
    //
    save.EPSILN = save.EST;

    if ((save.EPSILN + save.EPW) != save.EPW) {
        //
        // Now we switch over to Newton's method.  Note that
        // since our error estimate is less than 1/8, six iterations
        // of Newton's method should get us to within 1/2**96 of
        // the correct answer (If there were no round off to contend
        // with).
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 5;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.SINEPW = f64::sin(save.EPW);
                save.COSEPW = f64::cos(save.EPW);

                save.F = (((save.EPW - save.CAPU) - (save.AXN * save.SINEPW))
                    + (save.AYN * save.COSEPW));
                save.FPRIME = ((1.0 - (save.AXN * save.COSEPW)) - (save.AYN * save.SINEPW));

                save.EPWNXT = (save.EPW - (save.F / save.FPRIME));
                //
                // Our new error estimate comes from the discussion
                // of convergence of Newton's method.
                //
                save.EPW = save.EPWNXT;

                if ((save.EPW + save.EST) != save.EPW) {
                    save.EPSILN = save.EST;
                    save.EST = ((save.MOV1M * save.EST) * save.EST);
                }

                save.I += m3__;
            }
        }
    }
    //
    // Finally, we use bisection to avoid the problems of
    // round-off that may be present in Newton's method.  Since
    // we've gotten quite close to the answer (theoretically
    // anyway)  we won't have to perform many bisection passes.
    //
    // First we must bracket the root.  Note that we will
    // increase EPSILN so that we don't spend much time
    // determining the bracketing interval.  Also if the first
    // addition of EPSILN to EPW doesn't modify it, were set up
    // to just quit.  This happens only if F is sufficiently
    // close to zero that it can't alter EPW by adding it to
    // or subtracting it from EPW.
    //

    save.SINEPW = f64::sin(save.EPW);
    save.COSEPW = f64::cos(save.EPW);

    save.F = (((save.EPW - save.CAPU) - (save.AXN * save.SINEPW)) + (save.AYN * save.COSEPW));
    save.EPSILN = intrinsics::DMAX1(&[f64::abs(save.F), save.EPSILN]);

    if (save.F == 0 as f64) {
        save.LOWER = save.EPW;
        save.UPPER = save.EPW;
    } else if (save.F > 0 as f64) {
        save.FU = save.F;
        save.UPPER = save.EPW;
        save.LOWER = (save.EPW - save.EPSILN);
        save.EPW = save.LOWER;

        while ((save.F > 0 as f64) && (save.LOWER != save.UPPER)) {
            save.EPW = (save.EPW - save.EPSILN);
            save.F = (((save.EPW - save.CAPU) - (save.AXN * f64::sin(save.EPW)))
                + (save.AYN * f64::cos(save.EPW)));
            save.EPSILN = (save.EPSILN * 2.0);
        }

        save.LOWER = save.EPW;
        save.FL = save.F;

        if (save.F == 0 as f64) {
            save.UPPER = save.LOWER;
        }
    } else if (save.F < 0 as f64) {
        save.FL = save.F;
        save.LOWER = save.EPW;
        save.UPPER = (save.EPW + save.EPSILN);
        save.EPW = save.UPPER;

        while ((save.F < 0 as f64) && (save.LOWER != save.UPPER)) {
            save.EPW = (save.EPW + save.EPSILN);
            save.F = (((save.EPW - save.CAPU) - (save.AXN * f64::sin(save.EPW)))
                + (save.AYN * f64::cos(save.EPW)));
            save.EPSILN = (save.EPSILN * 2.0);
        }

        save.UPPER = save.EPW;
        save.FU = save.F;

        if (save.F == 0 as f64) {
            save.LOWER = save.EPW;
        }
    }

    //
    // Finally, bisect until we can do no more.
    //
    save.COUNT = 0;

    while ((save.UPPER > save.LOWER) && (save.COUNT < MXLOOP)) {
        save.COUNT = (save.COUNT + 1);
        save.EPW = BRCKTD((0.5 * (save.UPPER + save.LOWER)), save.LOWER, save.UPPER);
        //
        // EPW eventually will not be different from one of the
        // two bracketing values.  If this is the time, we need
        // to decide on a value for EPW.  That's done below.
        //
        if ((save.EPW == save.UPPER) || (save.EPW == save.LOWER)) {
            if (-save.FL < save.FU) {
                save.EPW = save.LOWER;
                save.UPPER = save.LOWER;
            } else {
                save.EPW = save.UPPER;
                save.LOWER = save.UPPER;
            }
        } else {
            save.F = (((save.EPW - save.CAPU) - (save.AXN * f64::sin(save.EPW)))
                + (save.AYN * f64::cos(save.EPW)));

            if (save.F > 0 as f64) {
                save.UPPER = save.EPW;
                save.FU = save.F;
            } else if (save.F < 0 as f64) {
                save.LOWER = save.EPW;
                save.FL = save.F;
            } else {
                save.LOWER = save.EPW;
                save.UPPER = save.EPW;
            }
        }
    }

    //
    // Short period preliminary quantities
    //

    save.SINEPW = f64::sin(save.EPW);
    save.COSEPW = f64::cos(save.EPW);

    save.TEMP3 = (save.AXN * save.SINEPW);
    save.TEMP4 = (save.AYN * save.COSEPW);

    save.TEMP5 = (save.AXN * save.COSEPW);
    save.TEMP6 = (save.AYN * save.SINEPW);

    save.ECOSE = (save.TEMP5 + save.TEMP6);
    save.ESINE = (save.TEMP3 - save.TEMP4);
    save.ELSQ = ((save.AXN * save.AXN) + (save.AYN * save.AYN));
    save.TEMP = (1.0 - save.ELSQ);
    save.PL = (save.A * save.TEMP);
    save.R = (save.A * (1.0 - save.ECOSE));

    save.TEMP1 = (1.0 / save.R);
    save.RDOT = (((save.KE * save.TEMP1) * f64::sqrt(save.A)) * save.ESINE);
    save.RFDOT = ((save.KE * save.TEMP1) * f64::sqrt(save.PL));
    save.TEMP2 = (save.A * save.TEMP1);
    save.BETAL = f64::sqrt(save.TEMP);
    save.TEMP3 = (1.0 / (1.0 + save.BETAL));

    save.COSU = (save.TEMP2 * ((save.COSEPW - save.AXN) + ((save.AYN * save.ESINE) * save.TEMP3)));
    save.SINU = (save.TEMP2 * ((save.SINEPW - save.AYN) - ((save.AXN * save.ESINE) * save.TEMP3)));
    //
    // Compute the angle from the x-axis of the point ( COSU, SINU )
    //
    if ((save.SINU != 0.0) || (save.COSU != 0.0)) {
        save.U = f64::atan2(save.SINU, save.COSU);

        if (save.U < 0 as f64) {
            save.U = (save.U + save.PIX2);
        }
    } else {
        save.U = 0.0;
    }

    save.SIN2U = ((2.0 * save.SINU) * save.COSU);
    save.COS2U = (((2.0 * save.COSU) * save.COSU) - 1.0);
    save.TEMP = (1.0 / save.PL);
    save.TEMP1 = (save.CK2 * save.TEMP);
    save.TEMP2 = (save.TEMP1 * save.TEMP);

    //
    // Update for short periodics
    //
    save.RK = ((save.R * (1.0 - (((1.5 * save.TEMP2) * save.BETAL) * save.X3THM1)))
        + (((0.5 * save.TEMP1) * save.X1MTH2) * save.COS2U));
    save.UK = (save.U - (((0.25 * save.TEMP2) * save.X7THM1) * save.SIN2U));

    save.XNODEK = (save.XNODE + (((1.5 * save.TEMP2) * save.COSIO) * save.SIN2U));
    save.XINCK = (save.XINCL + ((((1.5 * save.TEMP2) * save.COSIO) * save.COS2U) * save.SINIO));
    save.RDOTK = (save.RDOT - (((save.XN * save.TEMP1) * save.X1MTH2) * save.SIN2U));
    save.RFDOTK = (save.RFDOT
        + ((save.XN * save.TEMP1) * ((save.X1MTH2 * save.COS2U) + (1.5 * save.X3THM1))));
    //
    // Orientation vectors
    //
    save.SINUK = f64::sin(save.UK);
    save.COSUK = f64::cos(save.UK);

    save.SINIK = f64::sin(save.XINCK);
    save.COSIK = f64::cos(save.XINCK);

    save.SINNOK = f64::sin(save.XNODEK);
    save.COSNOK = f64::cos(save.XNODEK);

    save.XMX = -(save.SINNOK * save.COSIK);
    save.XMY = (save.COSNOK * save.COSIK);

    save.UX = ((save.XMX * save.SINUK) + (save.COSNOK * save.COSUK));
    save.UY = ((save.XMY * save.SINUK) + (save.SINNOK * save.COSUK));
    save.UZ = (save.SINIK * save.SINUK);
    save.VX = ((save.XMX * save.COSUK) - (save.COSNOK * save.SINUK));
    save.VY = ((save.XMY * save.COSUK) - (save.SINNOK * save.SINUK));
    save.VZ = (save.SINIK * save.COSUK);
    //
    // Position and velocity
    //
    STATE[1] = ((save.TOKM * save.RK) * save.UX);
    STATE[2] = ((save.TOKM * save.RK) * save.UY);
    STATE[3] = ((save.TOKM * save.RK) * save.UZ);
    STATE[4] = (save.TOKMPS * ((save.RDOTK * save.UX) + (save.RFDOTK * save.VX)));
    STATE[5] = (save.TOKMPS * ((save.RDOTK * save.UY) + (save.RFDOTK * save.VY)));
    STATE[6] = (save.TOKMPS * ((save.RDOTK * save.UZ) + (save.RFDOTK * save.VZ)));

    CHKOUT(b"EV2LIN", ctx)?;

    Ok(())
}
