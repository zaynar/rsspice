//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const TOL: f64 = 0.000001;
const TOTHRD: f64 = (2.0 / 3.0);
const ZERO: f64 = 0.0;
const ONE: f64 = 1.0;
const NUMGEO: i32 = 8;
const NUMELM: i32 = 10;

struct SaveVars {
    A3OVK2: f64,
    AE: f64,
    CK2: f64,
    CK4: f64,
    QOMS2T: f64,
    S: f64,
    XJ2: f64,
    XJ3: f64,
    XJ4: f64,
    XKE: f64,
    XKMPER: f64,
    LSTPHS: StackArray<f64, 8>,
    BSTAR: f64,
    EO: f64,
    EPOCH: f64,
    OMEGAO: f64,
    QO: f64,
    SO: f64,
    XINCL: f64,
    XMO: f64,
    XNO: f64,
    XNODEO: f64,
    LSTELM: StackArray<f64, 10>,
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
    C1: f64,
    C2: f64,
    C4: f64,
    CAPU: f64,
    COEF: f64,
    COEF1: f64,
    COS2U: f64,
    COSEPW: f64,
    COSIO: f64,
    COSU: f64,
    COSUK: f64,
    DEL1: f64,
    DELO: f64,
    E: f64,
    ECOSE: f64,
    EETA: f64,
    ELSQ: f64,
    EM: f64,
    EPW: f64,
    ESINE: f64,
    ETA: f64,
    ETASQ: f64,
    OMGADF: f64,
    OMGDOT: f64,
    PERIGE: f64,
    PINVSQ: f64,
    PIO2: f64,
    PIX2: f64,
    PL: f64,
    PSISQ: f64,
    QOMS24: f64,
    RDOT: f64,
    RDOTK: f64,
    RFDOT: f64,
    RFDOTK: f64,
    RK: f64,
    S4: f64,
    SCALE: f64,
    SIN2U: f64,
    SINEPW: f64,
    SINIO: f64,
    SINU: f64,
    SINUK: f64,
    T2COF: f64,
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
    THETA2: f64,
    TSI: f64,
    TSINCE: f64,
    TSQ: f64,
    UANG: f64,
    UK: f64,
    X1M5TH: f64,
    X1MTH2: f64,
    X3THM1: f64,
    X7THM1: f64,
    XHDOT1: f64,
    XINC: f64,
    XINCK: f64,
    XL: f64,
    XLCOF: f64,
    XLL: f64,
    XLT: f64,
    XMAM: f64,
    XMDF: f64,
    XMDOT: f64,
    XN: f64,
    XNODCF: f64,
    XNODDF: f64,
    XNODE: f64,
    XNODEK: f64,
    XNODOT: f64,
    XNODP: f64,
    M: StackArray<f64, 3>,
    N: StackArray<f64, 3>,
    U: StackArray<f64, 3>,
    V: StackArray<f64, 3>,
    CONT: bool,
    DOINIT: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut A3OVK2: f64 = 0.0;
        let mut AE: f64 = 0.0;
        let mut CK2: f64 = 0.0;
        let mut CK4: f64 = 0.0;
        let mut QOMS2T: f64 = 0.0;
        let mut S: f64 = 0.0;
        let mut XJ2: f64 = 0.0;
        let mut XJ3: f64 = 0.0;
        let mut XJ4: f64 = 0.0;
        let mut XKE: f64 = 0.0;
        let mut XKMPER: f64 = 0.0;
        let mut LSTPHS = StackArray::<f64, 8>::new(1..=NUMGEO);
        let mut BSTAR: f64 = 0.0;
        let mut EO: f64 = 0.0;
        let mut EPOCH: f64 = 0.0;
        let mut OMEGAO: f64 = 0.0;
        let mut QO: f64 = 0.0;
        let mut SO: f64 = 0.0;
        let mut XINCL: f64 = 0.0;
        let mut XMO: f64 = 0.0;
        let mut XNO: f64 = 0.0;
        let mut XNODEO: f64 = 0.0;
        let mut LSTELM = StackArray::<f64, 10>::new(1..=NUMELM);
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
        let mut C1: f64 = 0.0;
        let mut C2: f64 = 0.0;
        let mut C4: f64 = 0.0;
        let mut CAPU: f64 = 0.0;
        let mut COEF: f64 = 0.0;
        let mut COEF1: f64 = 0.0;
        let mut COS2U: f64 = 0.0;
        let mut COSEPW: f64 = 0.0;
        let mut COSIO: f64 = 0.0;
        let mut COSU: f64 = 0.0;
        let mut COSUK: f64 = 0.0;
        let mut DEL1: f64 = 0.0;
        let mut DELO: f64 = 0.0;
        let mut E: f64 = 0.0;
        let mut ECOSE: f64 = 0.0;
        let mut EETA: f64 = 0.0;
        let mut ELSQ: f64 = 0.0;
        let mut EM: f64 = 0.0;
        let mut EPW: f64 = 0.0;
        let mut ESINE: f64 = 0.0;
        let mut ETA: f64 = 0.0;
        let mut ETASQ: f64 = 0.0;
        let mut OMGADF: f64 = 0.0;
        let mut OMGDOT: f64 = 0.0;
        let mut PERIGE: f64 = 0.0;
        let mut PINVSQ: f64 = 0.0;
        let mut PIO2: f64 = 0.0;
        let mut PIX2: f64 = 0.0;
        let mut PL: f64 = 0.0;
        let mut PSISQ: f64 = 0.0;
        let mut QOMS24: f64 = 0.0;
        let mut RDOT: f64 = 0.0;
        let mut RDOTK: f64 = 0.0;
        let mut RFDOT: f64 = 0.0;
        let mut RFDOTK: f64 = 0.0;
        let mut RK: f64 = 0.0;
        let mut S4: f64 = 0.0;
        let mut SCALE: f64 = 0.0;
        let mut SIN2U: f64 = 0.0;
        let mut SINEPW: f64 = 0.0;
        let mut SINIO: f64 = 0.0;
        let mut SINU: f64 = 0.0;
        let mut SINUK: f64 = 0.0;
        let mut T2COF: f64 = 0.0;
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
        let mut THETA2: f64 = 0.0;
        let mut TSI: f64 = 0.0;
        let mut TSINCE: f64 = 0.0;
        let mut TSQ: f64 = 0.0;
        let mut UANG: f64 = 0.0;
        let mut UK: f64 = 0.0;
        let mut X1M5TH: f64 = 0.0;
        let mut X1MTH2: f64 = 0.0;
        let mut X3THM1: f64 = 0.0;
        let mut X7THM1: f64 = 0.0;
        let mut XHDOT1: f64 = 0.0;
        let mut XINC: f64 = 0.0;
        let mut XINCK: f64 = 0.0;
        let mut XL: f64 = 0.0;
        let mut XLCOF: f64 = 0.0;
        let mut XLL: f64 = 0.0;
        let mut XLT: f64 = 0.0;
        let mut XMAM: f64 = 0.0;
        let mut XMDF: f64 = 0.0;
        let mut XMDOT: f64 = 0.0;
        let mut XN: f64 = 0.0;
        let mut XNODCF: f64 = 0.0;
        let mut XNODDF: f64 = 0.0;
        let mut XNODE: f64 = 0.0;
        let mut XNODEK: f64 = 0.0;
        let mut XNODOT: f64 = 0.0;
        let mut XNODP: f64 = 0.0;
        let mut M = StackArray::<f64, 3>::new(1..=3);
        let mut N = StackArray::<f64, 3>::new(1..=3);
        let mut U = StackArray::<f64, 3>::new(1..=3);
        let mut V = StackArray::<f64, 3>::new(1..=3);
        let mut CONT: bool = false;
        let mut DOINIT: bool = false;
        let mut FIRST: bool = false;

        DOINIT = true;
        FIRST = true;

        Self {
            A3OVK2,
            AE,
            CK2,
            CK4,
            QOMS2T,
            S,
            XJ2,
            XJ3,
            XJ4,
            XKE,
            XKMPER,
            LSTPHS,
            BSTAR,
            EO,
            EPOCH,
            OMEGAO,
            QO,
            SO,
            XINCL,
            XMO,
            XNO,
            XNODEO,
            LSTELM,
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
            C1,
            C2,
            C4,
            CAPU,
            COEF,
            COEF1,
            COS2U,
            COSEPW,
            COSIO,
            COSU,
            COSUK,
            DEL1,
            DELO,
            E,
            ECOSE,
            EETA,
            ELSQ,
            EM,
            EPW,
            ESINE,
            ETA,
            ETASQ,
            OMGADF,
            OMGDOT,
            PERIGE,
            PINVSQ,
            PIO2,
            PIX2,
            PL,
            PSISQ,
            QOMS24,
            RDOT,
            RDOTK,
            RFDOT,
            RFDOTK,
            RK,
            S4,
            SCALE,
            SIN2U,
            SINEPW,
            SINIO,
            SINU,
            SINUK,
            T2COF,
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
            THETA2,
            TSI,
            TSINCE,
            TSQ,
            UANG,
            UK,
            X1M5TH,
            X1MTH2,
            X3THM1,
            X7THM1,
            XHDOT1,
            XINC,
            XINCK,
            XL,
            XLCOF,
            XLL,
            XLT,
            XMAM,
            XMDF,
            XMDOT,
            XN,
            XNODCF,
            XNODDF,
            XNODE,
            XNODEK,
            XNODOT,
            XNODP,
            M,
            N,
            U,
            V,
            CONT,
            DOINIT,
            FIRST,
        }
    }
}

/// Evaluate "two-line" element data, deep-space
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine EVSGP4. *ALL* TLE evaluations should use that routine.
/// NAIF supports DPSPCE only for backward compatibility.
///
/// Evaluate NORAD two-line element data for deep-space, earth
/// orbiting spacecraft (that is spacecraft with orbital periods
/// greater-than 225 minutes). This evaluator uses algorithms as
/// described in Hoots 1980 \[1].
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ET         I   Time for state evaluation in seconds past ephemeris
///                 epoch J2000.
///  GEOPHS     I   The array of geophysical constants.
///  ELEMS      I   Array of orbit elements.
///  STATE      O   State vector at ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the epoch in TDB seconds past ephemeris epoch J2000
///           to produced a state from the input elements.
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
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine is an extensive rewrite of the SDP4
///  routine as described in the Spacetrack 3 report. All common
///  blocks were removed and all variables are explicitly defined.
///
///  The removal of common blocks causes the set of routines to
///  execute slower than the original version of SDP4. However the
///  stability improves especially as concerns memory and
///  expanded internal documentation.
///
///  Trivial or redundant variables have been eliminated.
///
///     R         removed, occurrence replaced with RK
///     E6A       renamed TOL
///     THETA4    removed, relevant equation recast in Horner's form
///               i.e. something like x^4 + x^2 -> x^2 ( x^2 + 1 )
///     U         renamed UANG, U is now a euclidean 3 vector.
///     Ux,Uy,Uz  removed, replaced with 3-vector U
///     Vx,Vy,Vz  removed, replaced with 3-vector V
///     OMEGAQ    removed, usage replaced with OMEGAO
///     OMGDT     removed, same variable as OMGDOT, so all occurrences
///               replaced with OMGDOT
///     SSL,SSG   replaced with the 5-vector SSX
///     SSH,SSE
///     SSI
///
///  Three functions present in the original Spacetrack report, ACTAN,
///  FMOD2P and THETAG, have been either replaced with an intrinsic
///  FORTRAN function (ACTAN -> DATAN2, FMOD2P -> DMOD) or recoded
///  using SPICELIB calls (THETAG).
///
///  The code at the end of this subroutine which calculates
///  orientation vectors, was replaced with a set of calls to
///  SPICELIB vector routines.
///
///  A direct comparison of output from the original Spacetrack 3 code
///  and these NAIF routines for the same elements and time parameters
///  will produce unacceptably different results.
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
///  1) Suppose that you have collected the two-line element data
///     for the TDRS 4 geosynchronous satellite. The following example
///     code demonstrates how you obtain its state at an epoch of
///     interest.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: dpspce_ex1.tm
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
///           PROGRAM DPSPCE_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               PNAMLN
///           PARAMETER           ( PNAMLN = 2  )
///
///           INTEGER               TIMSLN
///           PARAMETER           ( TIMSLN = 25 )
///
///           INTEGER               TLELLN
///           PARAMETER           ( TLELLN = 69 )
///
///     C
///     C     The TDRS-4 satellite is an Earth orbiting object; set
///     C     the center ID to the Earth ID.
///     C
///           INTEGER               CENTER
///           PARAMETER           ( CENTER  = 399     )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(PNAMLN)    NOADPN ( 8  )
///           CHARACTER*(TIMSLN)    TIMSTR
///           CHARACTER*(TLELLN)    TLE    ( 2  )
///
///           DOUBLE PRECISION      DELT
///           DOUBLE PRECISION      ELEMS  ( 10 )
///           DOUBLE PRECISION      EPOCH
///           DOUBLE PRECISION      GEOPHS ( 8  )
///           DOUBLE PRECISION      STATE  ( 6  )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      TF
///
///           INTEGER               I
///           INTEGER               N
///
///     C
///     C     These are the variables that will hold the constants
///     C     required by DPSPCE. These constants are available from
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
///     C     Define the Two-Line Element set for TDRS-4.
///     C
///           TLE(1)  = '1 19883U 89021B   97133.05943164 -.00000277'
///          .      //                   '  00000-0  10000-3 0  3315'
///           TLE(2)  = '2 19883   0.5548  86.7278 0001786 312.2904 '
///          .      //                   '172.2391  1.00269108202415'
///
///     C
///     C     Load the PCK file that provides the geophysical
///     C     constants required for the evaluation of the two-line
///     C     elements sets. Load also an LSK, as it is required by
///     C     GETELM to perform time conversions. Use a meta-kernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'dpspce_ex1.tm' )
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
///     C     Set the lower bound for the years to be the earliest
///     C     first year for the elements.
///     C
///           CALL GETELM ( 1988, TLE, EPOCH, ELEMS )
///
///     C
///     C     Define the final time past epoch, 1400 mins (in seconds),
///     C     the step size for elements output, 360 mins (in seconds),
///     C     and the start time keyed off epoch.
///     C
///           TF   = 1440.D0 * 60.D0
///           DELT = 360.D0  * 60.D0
///           ET   = EPOCH - 4.D0 * DELT
///
///     C
///     C     Display the reference epoch for the elements.
///     C
///           CALL ET2UTC ( EPOCH, 'C', 3, TIMSTR )
///           WRITE(*,'(2A)') 'Reference epoch: ', TIMSTR
///
///     C
///     C     Obtain the state at different epochs around the
///     C     reference epoch.
///     C
///           DO WHILE ( DABS(TIME - EPOCH) .LE. DABS(TF) )
///
///              CALL DPSPCE ( ET, GEOPHS, ELEMS, STATE  )
///              CALL ET2UTC ( ET, 'C',    3,     TIMSTR )
///
///              WRITE(*,*)
///              WRITE(*,'(2A)')       'Time    : ', TIMSTR
///              WRITE(*,'(A,3F16.8)') 'Position: ', (STATE(I), I=1,3)
///              WRITE(*,'(A,3F16.8)') 'Velocity: ', (STATE(I), I=4,6)
///
///              ET = ET + DELT
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Reference epoch: 1997 MAY 13 01:25:34.894
///
///     Time    : 1997 MAY 12 01:25:34.894
///     Position:  -36415.06027922 -21268.27678598    324.63292275
///     Velocity:       1.55019424     -2.65462811     -0.01618361
///
///     Time    : 1997 MAY 12 07:25:34.894
///     Position:   21403.92800065 -36329.04571718   -223.49047558
///     Velocity:       2.64861127      1.56125103     -0.02363616
///
///     Time    : 1997 MAY 12 13:25:34.894
///     Position:   36222.44396185  21565.83478418   -323.53517502
///     Velocity:      -1.57313376      2.64235356      0.01642601
///
///     Time    : 1997 MAY 12 19:25:34.894
///     Position:  -21736.44940088  36128.44906628    226.92953877
///     Velocity:      -2.63481828     -1.58473777      0.02355544
///
///     Time    : 1997 MAY 13 01:25:34.894
///     Position:  -36048.19799634 -21884.57697027    322.54133293
///     Velocity:       1.59511867     -2.62787056     -0.01665374
///
///     Time    : 1997 MAY 13 07:25:34.894
///     Position:   22018.52968309 -35959.79405656   -229.86590206
///     Velocity:       2.62168498      1.60606852     -0.02347795
///
///     Time    : 1997 MAY 13 13:25:34.894
///     Position:   35850.58313589  22178.69474099   -321.27968318
///     Velocity:      -1.61782447      2.61522408      0.01688604
///
///     Time    : 1997 MAY 13 19:25:34.894
///     Position:  -22347.26148339  35754.07411618    233.17456026
///     Velocity:      -2.60750172     -1.62927167      0.02338424
///
///     Time    : 1997 MAY 14 01:25:34.894
///     Position:  -35671.56713967 -22493.50518477    320.10323846
///     Velocity:       1.63950543     -2.60040084     -0.01710490
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
/// -    SPICELIB Version 2.1.0, 01-NOV-2021 (JDR) (EDW)
///
///         Declared routine as deprecated.
///
///         Changed input argument name "TIME" to "ET" for consistency
///         with other routines.
///
///         Corrected the description of QO and SO constants in the
///         detailed description of the input argument GEOPHS and the input
///         element names in ELEMS.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragments.
///
/// -    SPICELIB Version 2.0.0, 23-JAN-2013 (EDW)
///
///         Corrected initialization block error. The ZZDPINIT call
///         causes a side-effect required for each DPSPCE call.
///         The ZZDPINIT call now occurs outside the initialization
///         block. Note from designer, side-effects are bad.
///
///         Added proper citation for Hoots paper.
///
/// -    SPICELIB Version 1.2.2, 22-AUG-2006 (EDW)
///
///         Replaced references to LDPOOL with references
///         to FURNSH.
///
/// -    SPICELIB Version 1.2.1, 27-DEC-2000 (EDW)
///
///         Corrected error in header documentation. Horner's Rule
///         not Butcher's.
///
/// -    SPICELIB Version 1.2.0, 24-MAR-1999 (EDW)
///
///         Documentation expanded to include modifications made
///         to private routines. Some English errors corrected.
///
///         Alphabetized variable declaration lists.
///
///         Temporary variable TEMP removed. OMGDOT argument added to
///         ZZDPSEC call.
///
/// -    SPICELIB Version 1.1.0, 05-OCT-1998 (WLT)
///
///         Forced initialization section until we can figure out
///         why it doesn't work on SUNs.
///
/// -    SPICELIB Version 1.0.1, 11-MAR-1998 (EDW)
///
///         Corrected error in header describing GEOPHS array.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1998 (EDW)
/// ```
pub fn dpspce(
    ctx: &mut SpiceContext,
    et: f64,
    geophs: &[f64; 8],
    elems: &[f64; 10],
    state: &mut [f64; 6],
) -> crate::Result<()> {
    DPSPCE(et, geophs, elems, state, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DPSPCE ( Evaluate "two-line" element data, deep-space )
pub fn DPSPCE(
    ET: f64,
    GEOPHS: &[f64],
    ELEMS: &[f64],
    STATE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let GEOPHS = DummyArray::new(GEOPHS, 1..=8);
    let ELEMS = DummyArray::new(ELEMS, 1..=10);
    let mut STATE = DummyArrayMut::new(STATE, 1..=6);

    //
    // Local variables
    //

    //
    // Define parameters for convergence tolerance and the value for 2/3,
    // 0 and 1.
    //

    //
    // The geophysical Quantities
    //

    //
    // Elements
    //

    //
    // Other quantities
    //

    //
    // SPICELIB routines
    //

    //
    // Save everything.
    //

    //
    // Set initialization flags
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DPSPCE", ctx)?;
    }

    //
    // If this is the very first time into this routine, set these
    // values.
    //
    if save.FIRST {
        save.PIX2 = TWOPI(ctx);
        save.PIO2 = HALFPI(ctx);
        save.FIRST = false;
    }

    //
    // If initialization flag is .FALSE., then this is not the first
    // call to this routine.  Check the stuff.
    //
    if !save.DOINIT {
        //
        // Check whether the current and last constants and elements
        // match.  If not, we need to reinitialize everything
        // since the propagation is dependent on the value of these
        // arrays.
        //
        for I in 1..=NUMGEO {
            if (save.LSTPHS[I] != GEOPHS[I]) {
                save.DOINIT = true;
            }
        }

        for I in 1..=NUMELM {
            if (save.LSTELM[I] != ELEMS[I]) {
                save.DOINIT = true;
            }
        }
    }

    //
    // Initialization block.  Always called on the initial entry and
    // anytime the geophysical or elements array changes.
    //
    if save.DOINIT {
        save.DOINIT = false;

        //
        // Retrieve the geophysical constants from the GEOPHS array
        //
        save.XJ2 = GEOPHS[1];
        save.XJ3 = GEOPHS[2];
        save.XJ4 = GEOPHS[3];
        save.XKE = GEOPHS[4];
        save.QO = GEOPHS[5];
        save.SO = GEOPHS[6];
        save.XKMPER = GEOPHS[7];
        save.AE = GEOPHS[8];

        //
        // Save the geophysical constants for later comparison
        //
        for I in 1..=NUMGEO {
            save.LSTPHS[I] = GEOPHS[I];
        }

        //
        // Unpack the elements array.
        //
        save.BSTAR = ELEMS[3];
        save.XINCL = ELEMS[4];
        save.XNODEO = ELEMS[5];
        save.EO = ELEMS[6];
        save.OMEGAO = ELEMS[7];
        save.XMO = ELEMS[8];
        save.XNO = ELEMS[9];
        save.EPOCH = ELEMS[10];

        //
        // Save the elements for later comparison
        //
        for I in 1..=NUMELM {
            save.LSTELM[I] = ELEMS[I];
        }

        //
        // Set common variables, the init flag and calculate the
        // WGS-72 physical and geopotential constants
        //
        // CK2 =  0.5   * J2 * AE^2
        // CK4 = -0.375 * J4 * AE^4
        //
        // These are values calculated only once and then saved for
        // future access.
        //
        save.CK2 = ((0.5 * save.XJ2) * f64::powi(save.AE, 2));
        save.CK4 = -((0.375 * save.XJ4) * f64::powi(save.AE, 4));
        save.QOMS2T = f64::powi((((save.QO - save.SO) * save.AE) / save.XKMPER), 4);
        save.S = (save.AE * (ONE + (save.SO / save.XKMPER)));

        //
        // Recover original mean motion (XNODP) and semimajor axis (AODP)
        // from input elements
        //
        save.A1 = f64::powf((save.XKE / save.XNO), TOTHRD);
        save.COSIO = f64::cos(save.XINCL);
        save.THETA2 = f64::powi(save.COSIO, 2);
        save.X3THM1 = ((3.0 * save.THETA2) - ONE);
        save.BETAO2 = (ONE - f64::powi(save.EO, 2));
        save.BETAO = f64::sqrt(save.BETAO2);

        save.DEL1 = (((1.5 * save.CK2) * save.X3THM1)
            / ((f64::powi(save.A1, 2) * save.BETAO) * save.BETAO2));
        save.AO = (save.A1
            * (ONE
                - (save.DEL1
                    * ((0.5 * TOTHRD) + (save.DEL1 * (ONE + ((134.0 / 81.0) * save.DEL1)))))));

        save.DELO = (((1.5 * save.CK2) * save.X3THM1)
            / ((f64::powi(save.AO, 2) * save.BETAO) * save.BETAO2));
        save.XNODP = (save.XNO / (ONE + save.DELO));
        save.AODP = (save.AO / (ONE - save.DELO));

        //
        // For perigee below 156 km, the values of S and QOMS2T are
        // altered
        //
        save.S4 = save.S;
        save.QOMS24 = save.QOMS2T;
        save.PERIGE = (((save.AODP * (ONE - save.EO)) - save.AE) * save.XKMPER);

        if (save.PERIGE < 156.0) {
            save.S4 = (save.PERIGE - 78.0);

            if (save.PERIGE > 98.0) {
                save.QOMS24 = f64::powi((((120.0 - save.S4) * save.AE) / save.XKMPER), 4);
                save.S4 = ((save.S4 / save.XKMPER) + save.AE);
            } else {
                save.S4 = 20.0;
            }
        }

        save.PINVSQ = (ONE / (f64::powi(save.AODP, 2) * f64::powi(save.BETAO2, 2)));
        save.TSI = (ONE / (save.AODP - save.S4));
        save.ETA = ((save.AODP * save.EO) * save.TSI);
        save.ETASQ = f64::powi(save.ETA, 2);
        save.EETA = (save.EO * save.ETA);
        save.PSISQ = f64::abs((ONE - save.ETASQ));
        save.COEF = (save.QOMS24 * f64::powi(save.TSI, 4));
        save.COEF1 = (save.COEF / f64::powf(save.PSISQ, 3.5));

        save.C2 = ((save.COEF1 * save.XNODP)
            * ((save.AODP * ((ONE + (1.5 * save.ETASQ)) + (save.EETA * (4.0 + save.ETASQ))))
                + (((((0.75 * save.CK2) * save.TSI) / save.PSISQ) * save.X3THM1)
                    * (8.0 + ((3.0 * save.ETASQ) * (8.0 + save.ETASQ))))));

        save.C1 = (save.BSTAR * save.C2);
        save.SINIO = f64::sin(save.XINCL);
        save.A3OVK2 = -((save.XJ3 / save.CK2) * f64::powi(save.AE, 3));
        save.X1MTH2 = (ONE - save.THETA2);

        save.C4 = (((((2.0 * save.XNODP) * save.COEF1) * save.AODP) * save.BETAO2)
            * (((save.ETA * (2.0 + (0.5 * save.ETASQ))) + (save.EO * (0.5 + (2.0 * save.ETASQ))))
                - ((((2.0 * save.CK2) * save.TSI) / (save.AODP * save.PSISQ))
                    * (-((3.0 * save.X3THM1)
                        * ((ONE - (2.0 * save.EETA))
                            + (save.ETASQ * (1.5 - (0.5 * save.EETA)))))
                        + (((0.75 * save.X1MTH2)
                            * ((2.0 * save.ETASQ) - (save.EETA * (ONE + save.ETASQ))))
                            * f64::cos((2.0 * save.OMEGAO)))))));

        save.TEMP1 = (((3.0 * save.CK2) * save.PINVSQ) * save.XNODP);
        save.TEMP2 = ((save.TEMP1 * save.CK2) * save.PINVSQ);
        save.TEMP3 = ((((1.25 * save.CK4) * save.PINVSQ) * save.PINVSQ) * save.XNODP);

        save.XMDOT = ((save.XNODP + (((0.5 * save.TEMP1) * save.BETAO) * save.X3THM1))
            + (((0.0625 * save.TEMP2) * save.BETAO)
                * (13.0 + (save.THETA2 * ((137.0 * save.THETA2) - 78.0)))));

        save.X1M5TH = (ONE - (5.0 * save.THETA2));

        save.OMGDOT = ((-((0.5 * save.TEMP1) * save.X1M5TH)
            + ((0.0625 * save.TEMP2) * (7.0 + (save.THETA2 * ((395.0 * save.THETA2) - 114.0)))))
            + (save.TEMP3 * (3.0 + (save.THETA2 * ((49.0 * save.THETA2) - 36.0)))));

        save.XHDOT1 = -(save.TEMP1 * save.COSIO);

        save.XNODOT = (save.XHDOT1
            + ((((0.5 * save.TEMP2) * (4.0 - (19.0 * save.THETA2)))
                + ((2.0 * save.TEMP3) * (3.0 - (7.0 * save.THETA2))))
                * save.COSIO));

        save.XNODCF = (((3.5 * save.BETAO2) * save.XHDOT1) * save.C1);
        save.T2COF = (1.5 * save.C1);
        save.XLCOF = ((((0.125 * save.A3OVK2) * save.SINIO) * (3.0 + (5.0 * save.COSIO)))
            / (ONE + save.COSIO));
        save.AYCOF = ((0.25 * save.A3OVK2) * save.SINIO);
        save.X7THM1 = ((7.0 * save.THETA2) - ONE);
    }

    ZZDPINIT(
        save.AODP,
        save.XMDOT,
        save.OMGDOT,
        save.XNODOT,
        save.XNODP,
        ELEMS.as_slice(),
        ctx,
    );

    //
    // Get the time since the EPOCH in minutes.
    //
    save.TSINCE = ((ET - save.EPOCH) / 60.0);

    //
    // Update for secular gravity and atmospheric drag
    //
    save.XMDF = (save.XMO + (save.XMDOT * save.TSINCE));
    save.OMGADF = (save.OMEGAO + (save.OMGDOT * save.TSINCE));
    save.XNODDF = (save.XNODEO + (save.XNODOT * save.TSINCE));
    save.TSQ = (save.TSINCE * save.TSINCE);
    save.XNODE = (save.XNODDF + (save.XNODCF * save.TSQ));
    save.TEMPA = (ONE - (save.C1 * save.TSINCE));
    save.TEMPE = ((save.BSTAR * save.C4) * save.TSINCE);
    save.TEMPL = (save.T2COF * save.TSQ);
    save.XN = save.XNODP;

    //
    // Calculate the secular terms.
    //
    ZZDPSEC(
        &mut save.XMDF,
        &mut save.OMGADF,
        &mut save.XNODE,
        &mut save.EM,
        &mut save.XINC,
        &mut save.XN,
        save.TSINCE,
        ELEMS.as_slice(),
        save.OMGDOT,
        ctx,
    );

    save.A = (f64::powf((save.XKE / save.XN), TOTHRD) * f64::powi(save.TEMPA, 2));
    save.E = (save.EM - save.TEMPE);
    save.XMAM = (save.XMDF + (save.XNODP * save.TEMPL));

    //
    // Calculate the periodic terms.
    //
    ZZDPPER(
        save.TSINCE,
        &mut save.E,
        &mut save.XINC,
        &mut save.OMGADF,
        &mut save.XNODE,
        &mut save.XMAM,
        ctx,
    );

    save.XL = ((save.XMAM + save.OMGADF) + save.XNODE);
    save.XN = (save.XKE / f64::powf(save.A, 1.5));

    //
    // Long period periodics
    //
    save.AXN = (save.E * f64::cos(save.OMGADF));
    save.TEMP = (ONE / (save.A * (ONE - f64::powi(save.E, 2))));
    save.XLL = ((save.TEMP * save.XLCOF) * save.AXN);
    save.AYNL = (save.TEMP * save.AYCOF);
    save.XLT = (save.XL + save.XLL);
    save.AYN = ((save.E * f64::sin(save.OMGADF)) + save.AYNL);

    //
    // Solve Kepler's equation
    //
    //       U = EPW - AXN * SIN(EPW)  +  AYN * COS(EPW)
    //
    // Where
    //
    //    AYN  = E * SIN(OMEGA)  +   AYNL
    //    AXN  = E * COS(OMEGA)
    //
    // And
    //
    //    AYNL =  -0.50D0 * SINIO * AE * J3 / (J2 * A * (1.0D0  -  E^2))
    //

    //
    // Get the mod division of CAPU with 2 Pi
    //
    save.CAPU = intrinsics::DMOD((save.XLT - save.XNODE), save.PIX2);

    if (save.CAPU < ZERO) {
        save.CAPU = (save.CAPU + save.PIX2);
    }

    //
    // Set initial states for the Kepler solution
    //

    save.EPW = save.CAPU;
    save.CONT = true;

    while save.CONT {
        save.TEMP2 = save.EPW;
        save.SINEPW = f64::sin(save.TEMP2);
        save.COSEPW = f64::cos(save.TEMP2);
        save.TEMP3 = (save.AXN * save.SINEPW);
        save.TEMP4 = (save.AYN * save.COSEPW);
        save.TEMP5 = (save.AXN * save.COSEPW);
        save.TEMP6 = (save.AYN * save.SINEPW);
        save.EPW = (((((save.CAPU - save.TEMP4) + save.TEMP3) - save.TEMP2)
            / ((ONE - save.TEMP5) - save.TEMP6))
            + save.TEMP2);

        //
        // Test for convergence against the defined tolerance
        //

        if (f64::abs((save.EPW - save.TEMP2)) <= TOL) {
            save.CONT = false;
        }
    }

    //
    // Short period preliminary quantities
    //

    save.ECOSE = (save.TEMP5 + save.TEMP6);
    save.ESINE = (save.TEMP3 - save.TEMP4);
    save.ELSQ = ((save.AXN * save.AXN) + (save.AYN * save.AYN));
    save.TEMP = (ONE - save.ELSQ);
    save.PL = (save.A * save.TEMP);
    save.RK = (save.A * (ONE - save.ECOSE));
    save.TEMP1 = (ONE / save.RK);
    save.RDOT = (((save.XKE * f64::sqrt(save.A)) * save.ESINE) * save.TEMP1);
    save.RFDOT = ((save.XKE * f64::sqrt(save.PL)) * save.TEMP1);
    save.TEMP2 = (save.A * save.TEMP1);
    save.BETAL = f64::sqrt(save.TEMP);
    save.TEMP3 = (ONE / (ONE + save.BETAL));

    save.COSU = (save.TEMP2 * ((save.COSEPW - save.AXN) + ((save.AYN * save.ESINE) * save.TEMP3)));
    save.SINU = (save.TEMP2 * ((save.SINEPW - save.AYN) - ((save.AXN * save.ESINE) * save.TEMP3)));

    //
    // Compute the angle from the x-axis of the point ( COSU, SINU )
    //
    if ((save.SINU != ZERO) || (save.COSU != ZERO)) {
        save.UANG = f64::atan2(save.SINU, save.COSU);

        if (save.UANG < ZERO) {
            save.UANG = (save.UANG + save.PIX2);
        }
    } else {
        save.UANG = ZERO;
    }

    save.SIN2U = ((2.0 * save.SINU) * save.COSU);
    save.COS2U = (((2.0 * save.COSU) * save.COSU) - ONE);
    save.TEMP1 = (save.CK2 * (ONE / save.PL));
    save.TEMP2 = (save.TEMP1 * (ONE / save.PL));

    //
    // Update for short periodics
    //
    save.RK = ((save.RK * (ONE - (((1.5 * save.TEMP2) * save.BETAL) * save.X3THM1)))
        + (((0.5 * save.TEMP1) * save.X1MTH2) * save.COS2U));
    save.UK = (save.UANG - (((0.25 * save.TEMP2) * save.X7THM1) * save.SIN2U));
    save.XNODEK = (save.XNODE + (((1.5 * save.TEMP2) * save.COSIO) * save.SIN2U));
    save.XINCK = (save.XINC + ((((1.5 * save.TEMP2) * save.COSIO) * save.SINIO) * save.COS2U));
    save.RDOTK = (save.RDOT - (((save.XN * save.TEMP1) * save.X1MTH2) * save.SIN2U));
    save.RFDOTK = (save.RFDOT
        + ((save.XN * save.TEMP1) * ((save.X1MTH2 * save.COS2U) + (1.5 * save.X3THM1))));

    //
    // Orientation vectors are calculated by
    //
    // U = M sin(uk) + N cos(uk)
    // V = M cos(uk) - N sin(uk)
    //
    // Where M and N are euclidean 3 vectors
    //
    // M = (-sin(xnodek)cos(xinck), cos(xnodek)cos(xinck), sin(xinck) )
    // N = (           cos(xnodek), sin(xnodek)          , 0          )
    //

    save.SINUK = f64::sin(save.UK);
    save.COSUK = f64::cos(save.UK);

    //
    // Use LATREC to generate M and N.  M is a latitude to rectangle
    // conversion of a unit vector where PI/2 + XNODEK is the longitude
    //

    LATREC(
        ONE,
        (save.PIO2 + save.XNODEK),
        save.XINCK,
        save.M.as_slice_mut(),
    );
    LATREC(ONE, save.XNODEK, ZERO, save.N.as_slice_mut());

    //
    // Sum the components to obtain U and V
    //
    VLCOM(
        save.SINUK,
        save.M.as_slice(),
        save.COSUK,
        save.N.as_slice(),
        save.U.as_slice_mut(),
    );
    VLCOM(
        save.COSUK,
        save.M.as_slice(),
        -save.SINUK,
        save.N.as_slice(),
        save.V.as_slice_mut(),
    );

    //
    // Determine the position and velocity then pack the STATE vector
    // with value scaled to KM and KPS.
    //
    // R = RK    U +        0 V
    // V = RKDOT U + RK RFDOT V
    //

    save.SCALE = (save.XKMPER / save.AE);

    VLCOM(
        (save.RK * save.SCALE),
        save.U.as_slice(),
        ZERO,
        save.V.as_slice(),
        STATE.subarray_mut(1),
    );

    //
    // Now scale to KPS for the velocity component
    //

    save.SCALE = (save.SCALE / 60.0);

    VLCOM(
        (save.RDOTK * save.SCALE),
        save.U.as_slice(),
        (save.RFDOTK * save.SCALE),
        save.V.as_slice(),
        STATE.subarray_mut(4),
    );

    //
    // All done now....
    //

    CHKOUT(b"DPSPCE", ctx)?;

    Ok(())
}
