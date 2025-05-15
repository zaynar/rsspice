//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const CTRSIZ: i32 = 2;
const MAXANG: i32 = 200;
const MXPHAS: i32 = 4;
const MXPOLY: i32 = 3;
const NAMLEN: i32 = 32;
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 35;
const LBPOOL: i32 = -5;
const MAXBOD: i32 = 157;

struct SaveVars {
    DTYPE: Vec<u8>,
    ERRMSG: Vec<u8>,
    FIXFRM: Vec<u8>,
    ITEM: Vec<u8>,
    ITEM2: Vec<u8>,
    TIMSTR: Vec<u8>,
    AC: StackArray<f64, 200>,
    COSTH: StackArray<f64, 200>,
    D: f64,
    DC: StackArray<f64, 200>,
    DCOEF: StackArray<f64, 3>,
    DCOSTH: StackArray<f64, 200>,
    DDEC: f64,
    DDELTA: f64,
    DEC: f64,
    DELTA: f64,
    DPHI: f64,
    DPVAL: f64,
    DRA: f64,
    DSINTH: StackArray<f64, 200>,
    DTIPM: StackArray2D<f64, 9>,
    DTHETA: f64,
    DW: f64,
    EULSTA: StackArray<f64, 6>,
    EPOCH: f64,
    PCKEPC: f64,
    PHI: f64,
    RA: f64,
    RCOEF: StackArray<f64, 3>,
    REQ2PC: StackArray2D<f64, 9>,
    SINTH: StackArray<f64, 200>,
    T: f64,
    TC: f64,
    TD: f64,
    TCOEF: ActualArray<f64>,
    TIPM: StackArray2D<f64, 9>,
    THETA: f64,
    TMPEPC: f64,
    W: f64,
    WC: StackArray<f64, 200>,
    WCOEF: StackArray<f64, 3>,
    XDTIPM: StackArray2D<f64, 9>,
    XTIPM: StackArray2D<f64, 9>,
    CENT: i32,
    DEG: i32,
    DIM: i32,
    FRCODE: i32,
    J2CODE: i32,
    K: i32,
    L: i32,
    M: i32,
    NA: i32,
    ND: i32,
    NPHASE: i32,
    NPHSCO: i32,
    NTHETA: i32,
    NW: i32,
    PCREF: i32,
    REFID: i32,
    REQREF: i32,
    FIRST: bool,
    FOUND: bool,
    FOUND2: bool,
    PULCTR: StackArray<i32, 2>,
    UPDATE: bool,
    BIDLST: StackArray<i32, 157>,
    BIDPOL: StackArray<i32, 163>,
    BIDIDS: StackArray<i32, 157>,
    BAC: ActualArray2D<f64>,
    BDC: ActualArray2D<f64>,
    BDCOEF: ActualArray2D<f64>,
    BPCKEP: StackArray<f64, 157>,
    BRCOEF: ActualArray2D<f64>,
    BTCOEF: ActualArray2D<f64>,
    BWC: ActualArray2D<f64>,
    BWCOEF: ActualArray2D<f64>,
    BNA: StackArray<i32, 157>,
    BND: StackArray<i32, 157>,
    BNPHAS: StackArray<i32, 157>,
    BNPHCO: StackArray<i32, 157>,
    BNW: StackArray<i32, 157>,
    BPCREF: StackArray<i32, 157>,
    SINTMP: f64,
    COSTMP: f64,
    AT: i32,
    AVAIL: i32,
    NEW: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DTYPE = vec![b' '; 1 as usize];
        let mut ERRMSG = vec![b' '; LMSGLN as usize];
        let mut FIXFRM = vec![b' '; FRNMLN as usize];
        let mut ITEM = vec![b' '; NAMLEN as usize];
        let mut ITEM2 = vec![b' '; NAMLEN as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut AC = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut COSTH = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut D: f64 = 0.0;
        let mut DC = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut DCOEF = StackArray::<f64, 3>::new(0..=(MXPOLY - 1));
        let mut DCOSTH = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut DDEC: f64 = 0.0;
        let mut DDELTA: f64 = 0.0;
        let mut DEC: f64 = 0.0;
        let mut DELTA: f64 = 0.0;
        let mut DPHI: f64 = 0.0;
        let mut DPVAL: f64 = 0.0;
        let mut DRA: f64 = 0.0;
        let mut DSINTH = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut DTIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut DTHETA: f64 = 0.0;
        let mut DW: f64 = 0.0;
        let mut EULSTA = StackArray::<f64, 6>::new(1..=6);
        let mut EPOCH: f64 = 0.0;
        let mut PCKEPC: f64 = 0.0;
        let mut PHI: f64 = 0.0;
        let mut RA: f64 = 0.0;
        let mut RCOEF = StackArray::<f64, 3>::new(0..=(MXPOLY - 1));
        let mut REQ2PC = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut SINTH = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut T: f64 = 0.0;
        let mut TC: f64 = 0.0;
        let mut TD: f64 = 0.0;
        let mut TCOEF = ActualArray::<f64>::new(1..=(MXPHAS * MAXANG));
        let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut THETA: f64 = 0.0;
        let mut TMPEPC: f64 = 0.0;
        let mut W: f64 = 0.0;
        let mut WC = StackArray::<f64, 200>::new(1..=MAXANG);
        let mut WCOEF = StackArray::<f64, 3>::new(0..=(MXPOLY - 1));
        let mut XDTIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut XTIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut CENT: i32 = 0;
        let mut DEG: i32 = 0;
        let mut DIM: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut J2CODE: i32 = 0;
        let mut K: i32 = 0;
        let mut L: i32 = 0;
        let mut M: i32 = 0;
        let mut NA: i32 = 0;
        let mut ND: i32 = 0;
        let mut NPHASE: i32 = 0;
        let mut NPHSCO: i32 = 0;
        let mut NTHETA: i32 = 0;
        let mut NW: i32 = 0;
        let mut PCREF: i32 = 0;
        let mut REFID: i32 = 0;
        let mut REQREF: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut FOUND2: bool = false;
        let mut PULCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut UPDATE: bool = false;
        let mut BIDLST = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut BIDPOL = StackArray::<i32, 163>::new(LBPOOL..=MAXBOD);
        let mut BIDIDS = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut BAC = ActualArray2D::<f64>::new(1..=MAXANG, 1..=MAXBOD);
        let mut BDC = ActualArray2D::<f64>::new(1..=MAXANG, 1..=MAXBOD);
        let mut BDCOEF = ActualArray2D::<f64>::new(0..=(MXPOLY - 1), 1..=MAXBOD);
        let mut BPCKEP = StackArray::<f64, 157>::new(1..=MAXBOD);
        let mut BRCOEF = ActualArray2D::<f64>::new(0..=(MXPOLY - 1), 1..=MAXBOD);
        let mut BTCOEF = ActualArray2D::<f64>::new(1..=(MXPHAS * MAXANG), 1..=MAXBOD);
        let mut BWC = ActualArray2D::<f64>::new(1..=MAXANG, 1..=MAXBOD);
        let mut BWCOEF = ActualArray2D::<f64>::new(0..=(MXPOLY - 1), 1..=MAXBOD);
        let mut BNA = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut BND = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut BNPHAS = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut BNPHCO = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut BNW = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut BPCREF = StackArray::<i32, 157>::new(1..=MAXBOD);
        let mut SINTMP: f64 = 0.0;
        let mut COSTMP: f64 = 0.0;
        let mut AT: i32 = 0;
        let mut AVAIL: i32 = 0;
        let mut NEW: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXBOD as usize))
                .chain([]);

            BPCREF
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        FOUND = false;

        Self {
            DTYPE,
            ERRMSG,
            FIXFRM,
            ITEM,
            ITEM2,
            TIMSTR,
            AC,
            COSTH,
            D,
            DC,
            DCOEF,
            DCOSTH,
            DDEC,
            DDELTA,
            DEC,
            DELTA,
            DPHI,
            DPVAL,
            DRA,
            DSINTH,
            DTIPM,
            DTHETA,
            DW,
            EULSTA,
            EPOCH,
            PCKEPC,
            PHI,
            RA,
            RCOEF,
            REQ2PC,
            SINTH,
            T,
            TC,
            TD,
            TCOEF,
            TIPM,
            THETA,
            TMPEPC,
            W,
            WC,
            WCOEF,
            XDTIPM,
            XTIPM,
            CENT,
            DEG,
            DIM,
            FRCODE,
            J2CODE,
            K,
            L,
            M,
            NA,
            ND,
            NPHASE,
            NPHSCO,
            NTHETA,
            NW,
            PCREF,
            REFID,
            REQREF,
            FIRST,
            FOUND,
            FOUND2,
            PULCTR,
            UPDATE,
            BIDLST,
            BIDPOL,
            BIDIDS,
            BAC,
            BDC,
            BDCOEF,
            BPCKEP,
            BRCOEF,
            BTCOEF,
            BWC,
            BWCOEF,
            BNA,
            BND,
            BNPHAS,
            BNPHCO,
            BNW,
            BPCREF,
            SINTMP,
            COSTMP,
            AT,
            AVAIL,
            NEW,
        }
    }
}

/// Transformation, inertial state to bodyfixed
///
/// Return a 6x6 matrix that transforms states in inertial
/// coordinates to states in body-equator-and-prime-meridian
/// coordinates.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
/// * [PCK](crate::required_reading::pck)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [ROTATION](crate::required_reading::rotation)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  REF        I   ID of inertial reference frame to transform from
///  BODY       I   ID code of body
///  ET         I   Epoch of transformation
///  TSIPM      O   Transformation (state), inertial to prime meridian
/// ```
///
/// # Detailed Input
///
/// ```text
///  REF      is the NAIF name for an inertial reference frame.
///           Acceptable names include:
///
///              Name       Description
///              --------   --------------------------------
///              'J2000'    Earth mean equator, dynamical
///                         equinox of J2000
///
///              'B1950'    Earth mean equator, dynamical
///                         equinox of B1950
///
///              'FK4'      Fundamental Catalog (4)
///
///              'DE-118'   JPL Developmental Ephemeris (118)
///
///              'DE-96'    JPL Developmental Ephemeris ( 96)
///
///              'DE-102'   JPL Developmental Ephemeris (102)
///
///              'DE-108'   JPL Developmental Ephemeris (108)
///
///              'DE-111'   JPL Developmental Ephemeris (111)
///
///              'DE-114'   JPL Developmental Ephemeris (114)
///
///              'DE-122'   JPL Developmental Ephemeris (122)
///
///              'DE-125'   JPL Developmental Ephemeris (125)
///
///              'DE-130'   JPL Developmental Ephemeris (130)
///
///              'GALACTIC' Galactic System II
///
///              'DE-200'   JPL Developmental Ephemeris (200)
///
///              'DE-202'   JPL Developmental Ephemeris (202)
///
///           See the Frames Required Reading frames.req for a full
///           list of inertial reference frame names built into
///           SPICE.
///
///           The output TSIPM will give the transformation
///           from this frame to the bodyfixed frame specified by
///           BODY at the epoch specified by ET.
///
///  BODY     is the integer ID code of the body for which the
///           state transformation matrix is requested. Bodies
///           are numbered according to the standard NAIF numbering
///           scheme. The numbering scheme is explained in the NAIF
///           IDs Required Reading naif_ids.req.
///
///  ET       is the epoch at which the state transformation
///           matrix is requested. (This is typically the
///           epoch of observation minus the one-way light time
///           from the observer to the body at the epoch of
///           observation.)
/// ```
///
/// # Detailed Output
///
/// ```text
///  TSIPM    is a 6x6 transformation matrix. It is used to
///           transform states from inertial coordinates to body
///           fixed (also called equator and prime meridian ---
///           PM) coordinates.
///
///           Given a state S in the inertial reference frame
///           specified by REF, the corresponding bodyfixed state
///           is given by the matrix vector product:
///
///              TSIPM * S
///
///           The X axis of the PM system is directed  to the
///           intersection of the equator and prime meridian.
///           The Z axis points along  the spin axis and points
///           towards the same side of the invariable plane of
///           the solar system as does earth's north pole.
///
///           NOTE: The inverse of TSIPM is NOT its transpose.
///           The matrix, TSIPM, has a structure as shown below:
///
///              .-            -.
///              |       :      |
///              |   R   :  0   |
///              | ......:......|
///              |       :      |
///              | dR/dt :  R   |
///              |       :      |
///              `-            -'
///
///           where R is a time varying rotation matrix and dR/dt is
///           its derivative. The inverse of this matrix is:
///
///              .-              -.
///              |     T  :       |
///              |    R   :  0    |
///              | .......:.......|
///              |        :       |
///              |      T :   T   |
///              | dR/dt  :  R    |
///              |        :       |
///              `-              -'
///
///           The SPICELIB routine INVSTM is available for producing
///           this inverse.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If data required to define the body-fixed frame associated
///      with BODY are not found in the binary PCK system or the kernel
///      pool, the error SPICE(FRAMEDATANOTFOUND) is signaled. In
///      the case of IAU style body-fixed frames, the absence of
///      prime meridian polynomial data (which are required) is used
///      as an indicator of missing data.
///
///  2)  If the test for exception (1) passes, but in fact requested
///      data are not available in the kernel pool, an error is
///      signaled by a routine in the call tree of this routine.
///
///  3)  If the kernel pool does not contain all of the data required
///      to define the number of nutation precession angles
///      corresponding to the available nutation precession
///      coefficients, the error SPICE(INSUFFICIENTANGLES) is
///      signaled.
///
///  4)  If the reference frame REF is not recognized, an error is
///      signaled by a routine in the call tree of this routine.
///
///  5)  If the specified body code BODY is not recognized, an error is
///      signaled by a routine in the call tree of this routine.
///
///  6)  If, for a given body, both forms of the kernel variable names
///
///         BODY<body ID>_CONSTANTS_JED_EPOCH
///         BODY<body ID>_CONSTS_JED_EPOCH
///
///      are found in the kernel pool, the error
///      SPICE(COMPETINGEPOCHSPEC) is signaled. This is done
///      regardless of whether the values assigned to the kernel
///      variable names match.
///
///  7)  If, for a given body, both forms of the kernel variable names
///
///         BODY<body ID>_CONSTANTS_REF_FRAME
///         BODY<body ID>_CONSTS_REF_FRAME
///
///      are found in the kernel pool, the error
///      SPICE(COMPETINGFRAMESPEC) is signaled. This is done
///      regardless of whether the values assigned to the kernel
///      variable names match.
///
///  8)  If the central body associated with the input BODY, whether
///      a system barycenter or BODY itself, has associated phase
///      angles (aka nutation precession angles), and the kernel
///      variable BODY<body ID>_MAX_PHASE_DEGREE for the central
///      body is present but has a value outside the range 1:3,
///      the error SPICE(DEGREEOUTOFRANGE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Note: NAIF recommends the use of SPKEZR with the appropriate
///  frames kernels when possible over TISBOD.
///
///  The matrix for transforming inertial states to bodyfixed
///  states is the 6x6 matrix shown below as a block structured
///  matrix.
///
///     .-            -.
///     |       :      |
///     | TIPM  :  0   |
///     | ......:......|
///     |       :      |
///     | DTIPM : TIPM |
///     |       :      |
///     `-            -'
///
///  This can also be expressed in terms of Euler angles
///  PHI, DELTA and W. The transformation from inertial to
///  bodyfixed coordinates is represented in the SPICE kernel
///  pool as:
///
///     TIPM   = [W]  [DELTA]  [PHI]
///                 3        1      3
///  Thus
///
///     DTIPM  = d[W] /dt [DELTA]  [PHI]
///                  3           1      3
///
///            + [W]  d[DELTA] /dt  [PHI]
///                 3             1      3
///
///            + [W]  [DELTA]  d[PHI] /dt
///                 3        1           3
///
///
///  If a binary PCK file record can be used for the time and
///  body requested, it will be used. The most recently loaded
///  binary PCK file has first priority, followed by previously
///  loaded binary PCK files in backward time order. If no
///  binary PCK file has been loaded, the text P_constants
///  kernel file is used.
///
///  If there is only text PCK kernel information, it is
///  expressed in terms of RA, DEC and W, where
///
///     RA  = PHI - HALFPI
///     DEC = HALFPI - DELTA
///     W   = W
///
///  The angles RA, DEC, and W are defined as follows in the
///  text PCK file:
///
///                                   2    .-----
///                   RA1*t      RA2*t      \
///     RA  = RA0  + -------  + -------   +  )  a(i) * sin( theta(i) )
///                     T          2        /
///                               T        '-----
///                                           i
///
///                                    2   .-----
///                   DEC1*t     DEC2*t     \
///     DEC = DEC0 + -------- + --------  +  )  d(i) * cos( theta(i) )
///                     T           2       /
///                                T       '-----
///                                           i
///
///                                  2     .-----
///                    W1*t      W2*t       \
///     W   = W0   +  ------  + -------   +  )  w(i) * sin( theta(i) )
///                     d          2        /
///                               d        '-----
///                                           i
///
///
///  where `d' is in seconds/day; T in seconds/Julian century;
///  a(i), d(i), and w(i) arrays apply to satellites only; and
///  theta(i), defined as
///
///                             THETA1(i)*t
///     theta(i) = THETA0(i) + -------------
///                                  T
///
///  are specific to each planet.
///
///  These angles ---typically nodal rates--- vary in number and
///  definition from one planetary system to the next.
///
///  Thus
///
///                                .-----
///              RA1     2*RA2*t    \   a(i)*THETA1(i)*cos(theta(i))
///  dRA/dt   = ----- + --------- +  ) ------------------------------
///               T          2      /                 T
///                         T      '-----
///                                   i
///
///                                  .-----
///              DEC1     2*DEC2*t    \   d(i)*THETA1(i)*sin(theta(i))
///   dDEC/dt = ------ + ---------- -  ) ------------------------------
///                T          2       /                 T
///                          T       '-----
///                                     i
///
///                              .-----
///              W1     2*W2*t    \   w(i)*THETA1(i)*cos(theta(i))
///   dW/dt   = ---- + -------- +  ) ------------------------------
///              d         2      /                 T
///                       d      '-----
///                                 i
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Calculate the matrix to transform a state vector from the
///     J2000 frame to the Saturn fixed frame at a specified
///     time, and use it to compute the geometric position and
///     velocity of Titan in Saturn's body-fixed frame.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: tisbod_ex1.tm
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
///           File name                     Contents
///           ---------                     --------
///           sat375.bsp                    Saturn satellite ephemeris
///           pck00010.tpc                  Planet orientation and
///                                         radii
///           naif0012.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'sat375.bsp',
///                               'pck00010.tpc',
///                               'naif0012.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM TISBOD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      STATE  ( 6    )
///           DOUBLE PRECISION      SATVEC ( 6    )
///           DOUBLE PRECISION      TSIPM  ( 6, 6 )
///
///           INTEGER               I
///           INTEGER               SATID
///
///     C
///     C     Load the kernels.
///     C
///           CALL FURNSH ( 'tisbod_ex1.tm' )
///
///     C
///     C     The body ID for Saturn.
///     C
///           SATID = 699
///
///     C
///     C     Retrieve the transformation matrix at some time.
///     C
///           CALL STR2ET ( 'Jan 1 2005',   ET        )
///           CALL TISBOD ( 'J2000', SATID, ET, TSIPM )
///
///     C
///     C     Retrieve the state of Titan as seen from Saturn
///     C     in the J2000 frame at ET.
///     C
///           CALL SPKEZR ( 'TITAN',  ET,    'J2000', 'NONE',
///          .              'SATURN', STATE, LT              )
///
///           WRITE(*,'(A)') 'Titan as seen from Saturn '
///          .            // '(J2000 frame):'
///           WRITE(*,'(A,3F13.3)') '   position   (km):',
///          .               ( STATE(I), I=1,3 )
///           WRITE(*,'(A,3F13.3)') '   velocity (km/s):',
///          .               ( STATE(I), I=4,6 )
///
///     C
///     C     Rotate the 6-vector STATE into the
///     C     Saturn body-fixed reference frame.
///     C
///           CALL MXVG ( TSIPM, STATE, 6, 6, SATVEC )
///
///           WRITE(*,'(A)') 'Titan as seen from Saturn '
///          .            // '(IAU_SATURN frame):'
///           WRITE(*,'(A,3F13.3)') '   position   (km):',
///          .               ( SATVEC(I), I=1,3 )
///           WRITE(*,'(A,3F13.3)') '   velocity (km/s):',
///          .               ( SATVEC(I), I=4,6 )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Titan as seen from Saturn (J2000 frame):
///        position   (km):  1071928.661  -505781.970   -60383.976
///        velocity (km/s):        2.404        5.176       -0.560
///     Titan as seen from Saturn (IAU_SATURN frame):
///        position   (km):   401063.338 -1116965.364    -5408.806
///        velocity (km/s):     -177.547      -63.745        0.028
///
///
///     Note that the complete example could be replaced by a single
///     SPKEZR call:
///
///        CALL SPKEZR ( 'TITAN',  ET,    'IAU_SATURN', 'NONE',
///       .              'SATURN', STATE, LT                   )
///
///
///  2) Use TISBOD is used to compute the angular velocity vector (with
///     respect to the J2000 inertial frame) of the specified body at
///     given time.
///
///     Use the meta-kernel from Example 1 above.
///
///
///     Example code begins here.
///
///
///           PROGRAM TISBOD_EX2
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      AV     ( 3    )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      DTIPM  ( 3, 3 )
///           DOUBLE PRECISION      OMEGA  ( 3, 3 )
///           DOUBLE PRECISION      ROT    ( 3, 3 )
///           DOUBLE PRECISION      TIPM   ( 3, 3 )
///           DOUBLE PRECISION      TSIPM  ( 6, 6 )
///           DOUBLE PRECISION      V      ( 3    )
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               SATID
///
///     C
///     C     Load the kernels.
///     C
///           CALL FURNSH ( 'tisbod_ex1.tm' )
///
///     C
///     C     The body ID for Saturn.
///     C
///           SATID = 699
///
///     C
///     C     First get the state transformation matrix.
///     C
///           CALL STR2ET ( 'Jan 1 2005',   ET        )
///           CALL TISBOD ( 'J2000', SATID, ET, TSIPM )
///
///     C
///     C     This matrix has the form:
///     C
///     C          .-            -.
///     C          |       :      |
///     C          | TIPM  :  0   |
///     C          | ......:......|
///     C          |       :      |
///     C          | DTIPM : TIPM |
///     C          |       :      |
///     C          `-            -'
///     C
///     C     We extract TIPM and DTIPM
///     C
///           DO  I = 1,3
///              DO  J = 1,3
///
///                 TIPM  ( I, J ) = TSIPM ( I,   J )
///                 DTIPM ( I, J ) = TSIPM ( I+3, J )
///
///              END DO
///           END DO
///
///     C
///     C     The transpose of TIPM and DTIPM, (TPMI and DTPMI), gives
///     C     the transformation from bodyfixed coordinates to inertial
///     C     coordinates.
///     C
///     C     Here is a fact about the relationship between angular
///     C     velocity associated with a time varying rotation matrix
///     C     that gives the orientation of a body with respect to
///     C     an inertial frame.
///     C
///     C        The angular velocity vector can be read from the off
///     C        diagonal components of the matrix product:
///     C
///     C                                t
///     C        OMEGA =     DTPMI * TPMI
///     C
///     C                         t
///     C              =     DTIPM * TIPM
///     C
///     C        the components of the angular velocity V will appear
///     C        in this matrix as:
///     C
///     C            .-                   -.
///     C            |                     |
///     C            |   0    -V(3)  V(2)  |
///     C            |                     |
///     C            |  V(3)    0   -V(1)  |
///     C            |                     |
///     C            | -V(2)   V(1)   0    |
///     C            |                     |
///     C            `-                   -'
///     C
///     C
///           CALL MTXM ( DTIPM, TIPM, OMEGA )
///
///           V(1) = OMEGA (3,2)
///           V(2) = OMEGA (1,3)
///           V(3) = OMEGA (2,1)
///
///     C
///     C     Display the results.
///     C
///           WRITE(*,'(A)') 'Angular velocity (km/s):'
///           WRITE(*,'(3F16.9)') V
///
///     C
///     C     It is possible to compute the angular velocity using
///     C     a single call to XF2RAV.
///     C
///           CALL XF2RAV ( TSIPM, ROT, AV )
///
///           WRITE(*,'(A)') 'Angular velocity using XF2RAV (km/s):'
///           WRITE(*,'(3F16.9)') AV
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Angular velocity (km/s):
///          0.000014001     0.000011995     0.000162744
///     Angular velocity using XF2RAV (km/s):
///          0.000014001     0.000011995     0.000162744
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The kernel pool must be loaded with the appropriate
///      coefficients (from a text or binary PCK file) prior to calling
///      this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.5.1, 16-DEC-2021 (NJB) (JDR)
///
///         The routine was updated to support user-defined maximum phase
///         angle degrees. The additional text kernel kernel variable name
///         BODYnnn_MAX_PHASE_DEGREE must be used when the phase angle
///         polynomials have degree higher than 1. The maximum allowed
///         degree is 3.
///
///         The kernel variable names
///
///            BODY#_CONSTS_REF_FRAME
///            BODY#_CONSTS_JED_EPOCH
///
///         are now recognized.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Added note to $Particulars section.
///
/// -    SPICELIB Version 4.5.0, 26-JUL-2016 (BVS)
///
///         The routine was updated to be more efficient by using a hash
///         and buffers so save text PCK data instead of doing kernel POOL
///         look-ups over an over again. The routine now checks the POOL
///         state counter and dumps all buffered data if it changes.
///
///         BUG FIX: changed available room in the BODVCD call
///         fetching 'NUT_PREC_ANGLES' from MAXANG to MAXANG*2.
///
/// -    SPICELIB Version 4.4.0, 01-FEB-2008 (NJB)
///
///         The routine was updated to improve the error messages created
///         when required PCK data are not found. Now in most cases the
///         messages are created locally rather than by the kernel pool
///         access routines. In particular missing binary PCK data will
///         be indicated with a reasonable error message.
///
/// -    SPICELIB Version 4.3.0, 13-DEC-2005 (NJB)
///
///         Bug fix: previous update introduced bug in state
///         transformation when REF was unequal to PCK native frame.
///
/// -    SPICELIB Version 4.2.0, 23-OCT-2005 (NJB)
///
///         Re-wrote portions of algorithm to simplify source code.
///         Updated to remove non-standard use of duplicate arguments
///         in MXM and VADDG calls.
///
///         Replaced calls to ZZBODVCD with calls to BODVCD.
///
/// -    SPICELIB Version 4.1.0, 05-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added.
///
/// -    SPICELIB Version 4.0.0, 12-FEB-2004 (NJB)
///
///         Code has been updated to support satellite ID codes in the
///         range 10000 to 99999 and to allow nutation precession angles
///         to be associated with any object.
///
///         Implementation changes were made to improve robustness
///         of the code.
///
/// -    SPICELIB Version 3.3.0, 29-MAR-1995 (WLT)
///
///         Properly initialized the variable NPAIRS.
///
/// -    SPICELIB Version 3.2.0, 22-MAR-1995 (KSZ)
///
///         Changed to call PCKMAT rather than PCKEUL.
///
/// -    SPICELIB Version 3.1.0, 18-OCT-1994 (KSZ)
///
///         Fixed bug which incorrectly modded DW by two pi.
///
/// -    SPICELIB Version 3.0.0, 10-MAR-1994 (KSZ)
///
///         Changed to look for binary PCK file, and used this
///         to find Euler angles, if such data has been loaded.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 04-SEP-1991 (NJB)
///
///         Updated to handle P_constants referenced to different epochs
///         and inertial reference frames.
///
///         $Required_Reading and $Literature_References sections were
///         updated.
///
/// -    SPICELIB Version 1.0.0, 05-NOV-1990 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.5.0, 26-JUL-2016 (BVS)
///
///         The routine was updated to be more efficient by using a hash
///         and buffers so save text PCK data instead of doing kernel POOL
///         look-ups over an over again. The routine now checks the POOL
///         state counter and dumps all buffered data if it changes.
///
/// -    SPICELIB Version 4.2.0, 23-OCT-2005 (NJB)
///
///         Re-wrote portions of algorithm to simplify source code.
///         The routine now takes advantage of EUL2XF, which wasn't
///         available when the first version of this routine was written.
///
///         Updated to remove non-standard use of duplicate arguments
///         in MXM and VADDG calls.
///
///         Replaced calls to ZZBODVCD with calls to BODVCD.
///
/// -    SPICELIB Version 4.1.0, 05-JAN-2005 (NJB)
///
///         Tests of routine FAILED() were added. The new checks
///         are intended to prevent arithmetic operations from
///         being performed with uninitialized or invalid data.
///
/// -     SPICELIB Version 4.0.0, 27-JAN-2004 (NJB)
///
///          Code has been updated to support satellite ID codes in the
///          range 10000 to 99999 and to allow nutation precession angles
///          to be associated with any object.
///
///          Calls to deprecated kernel pool access routine RTPOOL
///          were replaced by calls to GDPOOL.
///
///          Calls to BODVAR have been replaced with calls to
///          ZZBODVCD.
///
/// -     SPICELIB Version 3.3.0, 29-MAR-1995 (WLT)
///
///         The variable NPAIRS is now initialized
///         at the same point as NA, NTHETA, ND, and NW to be
///         zero. This prevents the routine from performing
///         needless calculations for planets and avoids possible
///         floating point exceptions.
///
/// -     SPICELIB Version 3.2.0, 22-MAR-1995 (KSZ)
///
///         TISBOD now gets the TSIPM matrix from PCKMAT.
///         Reference frame calculation moved to end.
///
/// -     SPICELIB Version 3.0.1, 07-OCT-1994 (KSZ)
///
///         TISBOD bug which mistakenly moded DW by 2PI
///         was removed.
///
/// -     SPICELIB Version 3.0.0, 10-MAR-1994 (KSZ)
///
///         TISBOD now uses new software to check for the
///         existence of binary PCK files, search the for
///         data corresponding to the requested body and time,
///         and return the appropriate Euler angles. Otherwise
///         the code calculates the Euler angles from the
///         P_constants kernel file.
///
/// -     SPICELIB Version 2.0.0, 04-SEP-1991 (NJB)
///
///          Updated to handle P_constants referenced to different epochs
///          and inertial reference frames.
///
///          TISBOD now checks the kernel pool for presence of the
///          variables
///
///             BODY#_CONSTANTS_REF_FRAME
///
///          and
///
///             BODY#_CONSTANTS_JED_EPOCH
///
///          where # is the NAIF integer code of the barycenter of a
///          planetary system or of a body other than a planet or
///          satellite. If either or both of these variables are
///          present, the P_constants for BODY are presumed to be
///          referenced to the specified inertial frame or epoch.
///          If the epoch of the constants is not J2000, the input
///          time ET is converted to seconds past the reference epoch.
///          If the frame of the constants is not the frame specified
///          by REF, the rotation from the P_constants' frame to
///          body-fixed coordinates is transformed to the rotation from
///          the requested frame to body-fixed coordinates. The same
///          transformation is applied to the derivative of this
///          rotation.
///
///          Due to the prescience of the original author, the code
///          was already prepared to handle the possibility of
///          specification of a P_constants inertial reference frame via
///          kernel pool variables.
///
///
///          Also, the $Required_Reading and $Literature_References
///          sections were updated. The SPK required reading has been
///          deleted from the $Literature_References section, and the
///          NAIF_IDS, KERNEL, and TIME Required Reading files have
///          been added in the $Required_Reading section.
/// ```
pub fn tisbod(
    ctx: &mut SpiceContext,
    ref_: &str,
    body: i32,
    et: f64,
    tsipm: &mut [[f64; 6]; 6],
) -> crate::Result<()> {
    TISBOD(
        ref_.as_bytes(),
        body,
        et,
        tsipm.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TISBOD ( Transformation, inertial state to bodyfixed )
pub fn TISBOD(
    REF: &[u8],
    BODY: i32,
    ET: f64,
    TSIPM: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TSIPM = DummyArrayMut2D::new(TSIPM, 1..=6, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Maximum number of coefficients per phase angle polynomial.
    //

    //
    // Local variables
    //

    //
    // POOL state counter.
    //

    //
    // ID-based hash for text PCK data. KIDLST, KIDPOL, and KIDIDS
    // provide the index in the body PCK data arrays at which the data
    // of the body with a given ID are stored.
    //

    //
    // Saved variables
    //
    // Because we need to save almost everything, we save everything
    // rather than taking a chance and accidentally leaving something
    // off the list.
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"TISBOD", ctx)?;

    //
    // Perform any needed first pass initializations.
    //
    if save.FIRST {
        //
        // Initialize POOL state counter to the user value.
        //
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Initialize kernel POOL frame hashes.
        //
        ZZHSIINI(
            MAXBOD,
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            ctx,
        )?;

        //
        // Get the code for the J2000 frame.
        //
        IRFNUM(b"J2000", &mut save.J2CODE, ctx);

        //
        // Set seconds per day and per century.
        //
        save.D = SPD();
        save.T = (save.D * 36525.0);

        save.FIRST = false;
    }

    IRFNUM(REF, &mut save.REQREF, ctx);

    //
    // Get state transformation matrix from high precision PCK file, if
    // available.
    //
    PCKMAT(
        BODY,
        ET,
        &mut save.PCREF,
        TSIPM.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    if !save.FOUND {
        //
        // The data for the frame of interest are not available in a
        // loaded binary PCK file. This is not an error: the data may be
        // present in the kernel pool.
        //
        // Check the POOL counter. If it changed, dump all buffered
        // constants data.
        //
        ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.UPDATE, ctx);

        if save.UPDATE {
            ZZHSIINI(
                MAXBOD,
                save.BIDLST.as_slice_mut(),
                save.BIDPOL.as_slice_mut(),
                ctx,
            )?;
        }

        //
        // Check if we have data for this body in our buffers.
        //
        ZZHSICHK(
            save.BIDLST.as_slice(),
            save.BIDPOL.as_slice(),
            save.BIDIDS.as_slice(),
            BODY,
            &mut save.AT,
            ctx,
        )?;

        if (save.AT != 0) {
            //
            // Set PCREF as it is used after the text PCK "IF".
            //
            save.PCREF = save.BPCREF[save.AT];
        } else {
            //
            // Conduct a non-error-signaling check for the presence of a
            // kernel variable that is required to implement an IAU style
            // body-fixed reference frame. If the data aren't available,
            // we don't want BODVCD to signal a SPICE(KERNELVARNOTFOUND)
            // error; we want to issue the error signal locally, with a
            // better error message.
            //
            fstr::assign(&mut save.ITEM, b"BODY#_PM");
            REPMI(&save.ITEM.to_vec(), b"#", BODY, &mut save.ITEM, ctx);
            DTPOOL(
                &save.ITEM,
                &mut save.FOUND,
                &mut save.NW,
                &mut save.DTYPE,
                ctx,
            )?;

            if !save.FOUND {
                //
                // Now we do have an error.
                //
                // We don't have the data we'll need to produced the
                // requested state transformation matrix. In order to
                // create an error message understandable to the user,
                // find, if possible, the name of the reference frame
                // associated with the input body. Note that the body is
                // really identified by a PCK frame class ID code, though
                // most of the documentation just calls it a body ID code.
                //
                CCIFRM(
                    PCK,
                    BODY,
                    &mut save.FRCODE,
                    &mut save.FIXFRM,
                    &mut save.CENT,
                    &mut save.FOUND,
                    ctx,
                )?;
                ETCAL(ET, &mut save.TIMSTR, ctx);

                fstr::assign(&mut save.ERRMSG, b"PCK data required to compute the orientation of the # # for epoch # TDB were not found. If these data were to be provided by a binary PCK file, then it is possible that the PCK file does not have coverage for the specified body-fixed frame at the time of interest. If the data were to be provided by a text PCK file, then possibly the file does not contain data for the specified body-fixed frame. In either case it is possible that a required PCK file was not loaded at all.");

                //
                // Fill in the variable data in the error message.
                //
                if save.FOUND {
                    //
                    // The frame system knows the name of the body-fixed
                    // frame.
                    //
                    SETMSG(&save.ERRMSG, ctx);
                    ERRCH(b"#", b"body-fixed frame", ctx);
                    ERRCH(b"#", &save.FIXFRM, ctx);
                    ERRCH(b"#", &save.TIMSTR, ctx);
                } else {
                    //
                    // The frame system doesn't know the name of the
                    // body-fixed frame, most likely due to a missing
                    // frame kernel.
                    //
                    SUFFIX(b"#", 1, &mut save.ERRMSG);
                    SETMSG(&save.ERRMSG, ctx);
                    ERRCH(b"#", b"body-fixed frame associated with the ID code", ctx);
                    ERRINT(b"#", BODY, ctx);
                    ERRCH(b"#", &save.TIMSTR, ctx);
                    ERRCH(b"#", b"Also, a frame kernel defining the body-fixed frame associated with body # may need to be loaded.", ctx);
                    ERRINT(b"#", BODY, ctx);
                }

                SIGERR(b"SPICE(FRAMEDATANOTFOUND)", ctx)?;
                CHKOUT(b"TISBOD", ctx)?;
                return Ok(());
            }

            //
            // Find the body code used to label the reference frame and
            // epoch specifiers for the orientation constants for BODY.
            //
            // For planetary systems, the reference frame and epoch for
            // the orientation constants is associated with the system
            // barycenter, not with individual bodies in the system. For
            // any other bodies, (the Sun or asteroids, for example) the
            // body's own code is used as the label.
            //
            save.REFID = ZZBODBRY(BODY);

            //
            // Look up the epoch of the constants. The epoch is specified
            // as a Julian ephemeris date. The epoch defaults to J2000.
            //
            // Look for both forms of the JED epoch kernel variable. At
            // most one is allowed to be present.
            //
            fstr::assign(&mut save.ITEM, b"BODY#_CONSTANTS_JED_EPOCH");
            REPMI(&save.ITEM.to_vec(), b"#", save.REFID, &mut save.ITEM, ctx);

            fstr::assign(&mut save.ITEM2, b"BODY#_CONSTS_JED_EPOCH");
            REPMI(&save.ITEM2.to_vec(), b"#", save.REFID, &mut save.ITEM2, ctx);

            GDPOOL(
                &save.ITEM,
                1,
                1,
                &mut save.DIM,
                std::slice::from_mut(&mut save.PCKEPC),
                &mut save.FOUND,
                ctx,
            )?;

            if !save.FOUND {
                GDPOOL(
                    &save.ITEM2,
                    1,
                    1,
                    &mut save.DIM,
                    std::slice::from_mut(&mut save.PCKEPC),
                    &mut save.FOUND2,
                    ctx,
                )?;

                if !save.FOUND2 {
                    save.PCKEPC = J2000();
                }
            } else {
                //
                // Check for presence of both forms of the variable names.
                //
                DTPOOL(
                    &save.ITEM2,
                    &mut save.FOUND2,
                    &mut save.DIM,
                    &mut save.DTYPE,
                    ctx,
                )?;

                if save.FOUND2 {
                    SETMSG(b"Both kernel variables # and # are present in the kernel pool. At most one form of the kernel variable name may be present.", ctx);
                    ERRCH(b"#", &save.ITEM, ctx);
                    ERRCH(b"#", &save.ITEM2, ctx);
                    SIGERR(b"SPICE(COMPETINGEPOCHSPEC)", ctx)?;
                    CHKOUT(b"TISBOD", ctx)?;
                    return Ok(());
                }
            }

            //
            // Look up the reference frame of the constants. The reference
            // frame is specified by a code recognized by CHGIRF. The
            // default frame is J2000, symbolized by the code J2CODE.
            //
            fstr::assign(&mut save.ITEM, b"BODY#_CONSTANTS_REF_FRAME");
            REPMI(&save.ITEM.to_vec(), b"#", save.REFID, &mut save.ITEM, ctx);

            fstr::assign(&mut save.ITEM2, b"BODY#_CONSTS_REF_FRAME");
            REPMI(&save.ITEM2.to_vec(), b"#", save.REFID, &mut save.ITEM2, ctx);

            REPMI(&save.ITEM.to_vec(), b"#", save.REFID, &mut save.ITEM, ctx);
            GIPOOL(
                &save.ITEM,
                1,
                1,
                &mut save.DIM,
                std::slice::from_mut(&mut save.PCREF),
                &mut save.FOUND,
                ctx,
            )?;

            if !save.FOUND {
                GIPOOL(
                    &save.ITEM2,
                    1,
                    1,
                    &mut save.DIM,
                    std::slice::from_mut(&mut save.PCREF),
                    &mut save.FOUND,
                    ctx,
                )?;

                if !save.FOUND {
                    save.PCREF = save.J2CODE;
                }
            } else {
                //
                // Check for presence of both forms of the variable names.
                //
                DTPOOL(
                    &save.ITEM2,
                    &mut save.FOUND,
                    &mut save.DIM,
                    &mut save.DTYPE,
                    ctx,
                )?;

                if save.FOUND {
                    SETMSG(b"Both kernel variables # and # are present in the kernel pool. At most one form of the kernel variable name may be present.", ctx);
                    ERRCH(b"#", &save.ITEM, ctx);
                    ERRCH(b"#", &save.ITEM2, ctx);
                    SIGERR(b"SPICE(COMPETINGFRAMESPEC)", ctx)?;
                    CHKOUT(b"TISBOD", ctx)?;
                    return Ok(());
                }
            }

            //
            // Whatever the body, it has quadratic time polynomials for
            // the RA and Dec of the pole, and for the rotation of the
            // Prime Meridian.
            //
            fstr::assign(&mut save.ITEM, b"POLE_RA");
            CLEARD(MXPOLY, save.RCOEF.as_slice_mut());
            BODVCD(
                BODY,
                &save.ITEM,
                MXPOLY,
                &mut save.NA,
                save.RCOEF.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"TISBOD", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.ITEM, b"POLE_DEC");
            CLEARD(MXPOLY, save.DCOEF.as_slice_mut());
            BODVCD(
                BODY,
                &save.ITEM,
                MXPOLY,
                &mut save.ND,
                save.DCOEF.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"TISBOD", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.ITEM, b"PM");
            CLEARD(MXPOLY, save.WCOEF.as_slice_mut());
            BODVCD(
                BODY,
                &save.ITEM,
                MXPOLY,
                &mut save.NW,
                save.WCOEF.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"TISBOD", ctx)?;
                return Ok(());
            }

            //
            // Whether or not the body is a satellite, there may be
            // additional nutation and libration (THETA) terms.
            //
            save.NTHETA = 0;
            save.NPHASE = 0;
            save.NPHSCO = 0;
            save.NA = 0;
            save.ND = 0;
            save.NW = 0;

            fstr::assign(&mut save.ITEM, b"NUT_PREC_ANGLES");

            if BODFND(save.REFID, &save.ITEM, ctx)? {
                // Find out whether the maximum phase angle degree
                // has been explicitly set.
                //
                fstr::assign(&mut save.ITEM2, b"MAX_PHASE_DEGREE");

                if BODFND(save.REFID, &save.ITEM2, ctx)? {
                    BODVCD(
                        save.REFID,
                        &save.ITEM2,
                        1,
                        &mut save.DIM,
                        std::slice::from_mut(&mut save.DPVAL),
                        ctx,
                    )?;

                    save.DEG = intrinsics::IDNINT(save.DPVAL);

                    if ((save.DEG < 1) || (save.DEG > (MXPHAS - 1))) {
                        SETMSG(b"Maximum phase angle degree for body # must be in the range 1:# but was #.", ctx);
                        ERRINT(b"#", save.REFID, ctx);
                        ERRINT(b"#", (MXPHAS - 1), ctx);
                        ERRINT(b"#", save.DEG, ctx);
                        SIGERR(b"SPICE(DEGREEOUTOFRANGE)", ctx)?;
                        CHKOUT(b"TISBOD", ctx)?;
                        return Ok(());
                    }

                    save.NPHSCO = (save.DEG + 1);
                } else {
                    //
                    // The default degree is 1, yielding two coefficients.
                    //
                    save.NPHSCO = 2;
                }
                //
                // There is something a bit obscure going on below. BODVCD
                // loads the array TCOEF in the following order
                //
                //    TCOEF(1,1), TCOEF(2,1), ... TCOEF(NPHSCO),
                //    TCOEF(1,2), TCOEF(2,2), ...
                //
                // The NTHETA that comes back is the total number of items
                // loaded, but we will need the actual limit on the second
                // dimension. That is --- NTHETA / NPHSCO.
                //
                BODVCD(
                    save.REFID,
                    &save.ITEM,
                    (MAXANG * MXPHAS),
                    &mut save.NTHETA,
                    save.TCOEF.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"TISBOD", ctx)?;
                    return Ok(());
                }

                //
                // NPHSCO is at least 1; this division is safe.
                //
                save.NPHASE = (save.NTHETA / save.NPHSCO);
            }

            //
            // Look up the right ascension nutations in the precession of
            // the pole. NA is the number of Ascension coefficients. AC
            // are the Ascension coefficients.
            //
            fstr::assign(&mut save.ITEM, b"NUT_PREC_RA");

            if BODFND(BODY, &save.ITEM, ctx)? {
                BODVCD(
                    BODY,
                    &save.ITEM,
                    MAXANG,
                    &mut save.NA,
                    save.AC.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"TISBOD", ctx)?;
                    return Ok(());
                }
            }

            //
            // Look up the declination nutations in the precession of the
            // pole. ND is the number of Declination coefficients. DC are
            // the Declination coefficients.
            //
            fstr::assign(&mut save.ITEM, b"NUT_PREC_DEC");

            if BODFND(BODY, &save.ITEM, ctx)? {
                BODVCD(
                    BODY,
                    &save.ITEM,
                    MAXANG,
                    &mut save.ND,
                    save.DC.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"TISBOD", ctx)?;
                    return Ok(());
                }
            }

            //
            // Finally look up the prime meridian nutations. NW is the
            // number of coefficients. WC is the array of coefficients.
            //
            fstr::assign(&mut save.ITEM, b"NUT_PREC_PM");

            if BODFND(BODY, &save.ITEM, ctx)? {
                BODVCD(
                    BODY,
                    &save.ITEM,
                    MAXANG,
                    &mut save.NW,
                    save.WC.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"TISBOD", ctx)?;
                    return Ok(());
                }
            }

            //
            // The number of coefficients returned had better not be
            // bigger than the number of angles we are going to compute.
            // If it is we simply signal an error and bag it, for sure.
            //
            if (intrinsics::MAX0(&[save.NA, save.ND, save.NW]) > save.NPHASE) {
                SETMSG(b"Insufficient number of nutation/precession angles for body * at time #. Number of angles is #; number required is #.", ctx);
                ERRINT(b"*", BODY, ctx);
                ERRDP(b"#", ET, ctx);
                ERRINT(b"#", save.NPHASE, ctx);
                ERRINT(b"#", intrinsics::MAX0(&[save.NA, save.ND, save.NW]), ctx);
                SIGERR(b"SPICE(INSUFFICIENTANGLES)", ctx)?;
                CHKOUT(b"TISBOD", ctx)?;
                return Ok(());
            }

            //
            // We succeeded to fetch all data for this body. Save it in
            // the buffers. First check if there is room. If not, dump the
            // buffers to make room.
            //
            ZZHSIAVL(save.BIDPOL.as_slice(), &mut save.AVAIL);
            if (save.AVAIL <= 0) {
                ZZHSIINI(
                    MAXBOD,
                    save.BIDLST.as_slice_mut(),
                    save.BIDPOL.as_slice_mut(),
                    ctx,
                )?;
            }

            //
            // Add this body to the hash and save its data in the buffers.
            //
            ZZHSIADD(
                save.BIDLST.as_slice_mut(),
                save.BIDPOL.as_slice_mut(),
                save.BIDIDS.as_slice_mut(),
                BODY,
                &mut save.AT,
                &mut save.NEW,
                ctx,
            )?;

            save.BPCKEP[save.AT] = save.PCKEPC;
            save.BPCREF[save.AT] = save.PCREF;
            save.BNPHAS[save.AT] = save.NPHASE;
            save.BNPHCO[save.AT] = save.NPHSCO;
            save.BNA[save.AT] = save.NA;
            save.BND[save.AT] = save.ND;
            save.BNW[save.AT] = save.NW;

            MOVED(
                save.RCOEF.subarray(0),
                MXPOLY,
                save.BRCOEF.subarray_mut([0, save.AT]),
            );
            MOVED(
                save.DCOEF.subarray(0),
                MXPOLY,
                save.BDCOEF.subarray_mut([0, save.AT]),
            );
            MOVED(
                save.WCOEF.subarray(0),
                MXPOLY,
                save.BWCOEF.subarray_mut([0, save.AT]),
            );
            MOVED(
                save.TCOEF.subarray(1),
                (MAXANG * save.NPHSCO),
                save.BTCOEF.subarray_mut([1, save.AT]),
            );
            MOVED(
                save.AC.subarray(1),
                MAXANG,
                save.BAC.subarray_mut([1, save.AT]),
            );
            MOVED(
                save.DC.subarray(1),
                MAXANG,
                save.BDC.subarray_mut([1, save.AT]),
            );
            MOVED(
                save.WC.subarray(1),
                MAXANG,
                save.BWC.subarray_mut([1, save.AT]),
            );
        }

        //
        // The reference epoch in the PCK is given as JED. Convert to
        // ephemeris seconds past J2000. Then convert the input ET to
        // seconds past the reference epoch.
        //
        save.TMPEPC = (SPD() * (save.BPCKEP[save.AT] - J2000()));
        save.EPOCH = (ET - save.TMPEPC);
        save.TD = (save.EPOCH / save.D);
        save.TC = (save.EPOCH / save.T);

        //
        // Evaluate the time polynomials and their derivatives w.r.t.
        // EPOCH at EPOCH.
        //
        // Evaluate the time polynomials at EPOCH.
        //
        save.RA = (save.BRCOEF[[0, save.AT]]
            + (save.TC * (save.BRCOEF[[1, save.AT]] + (save.TC * save.BRCOEF[[2, save.AT]]))));
        save.DEC = (save.BDCOEF[[0, save.AT]]
            + (save.TC * (save.BDCOEF[[1, save.AT]] + (save.TC * save.BDCOEF[[2, save.AT]]))));
        save.W = (save.BWCOEF[[0, save.AT]]
            + (save.TD * (save.BWCOEF[[1, save.AT]] + (save.TD * save.BWCOEF[[2, save.AT]]))));

        save.DRA =
            ((save.BRCOEF[[1, save.AT]] + ((2.0 * save.TC) * save.BRCOEF[[2, save.AT]])) / save.T);
        save.DDEC =
            ((save.BDCOEF[[1, save.AT]] + ((2.0 * save.TC) * save.BDCOEF[[2, save.AT]])) / save.T);
        save.DW =
            ((save.BWCOEF[[1, save.AT]] + ((2.0 * save.TD) * save.BWCOEF[[2, save.AT]])) / save.D);

        //
        // Compute the nutations and librations (and their derivatives)
        // as appropriate.
        //
        for I in 1..=save.BNPHAS[save.AT] {
            if (save.BNPHCO[save.AT] == 2) {
                //
                // This case is so common that we'll deal with it
                // separately. We'll avoid unnecessary arithmetic
                // operations.
                //
                save.K = (1 + (2 * (I - 1)));
                save.M = (1 + save.K);

                save.THETA = ((save.BTCOEF[[save.K, save.AT]]
                    + (save.TC * save.BTCOEF[[save.M, save.AT]]))
                    * RPD(ctx));
                save.DTHETA = ((save.BTCOEF[[save.M, save.AT]] / save.T) * RPD(ctx));
            } else {
                //
                // THETA and DTHETA have higher-order terms; there are
                // BNPHCO(AT) coefficients for each angle.
                //
                save.THETA = 0.0;

                for J in 1..=save.BNPHCO[save.AT] {
                    //
                    // K is the start index for the coefficients of the
                    // Ith angle.
                    //
                    save.K = (J + (save.BNPHCO[save.AT] * (I - 1)));

                    save.THETA = (save.THETA
                        + (f64::powi(save.TC, (J - 1)) * save.BTCOEF[[save.K, save.AT]]));
                }

                save.THETA = (save.THETA * RPD(ctx));

                save.K = (1 + (save.BNPHCO[save.AT] * (I - 1)));
                save.M = (1 + save.K);

                save.DTHETA = (save.BTCOEF[[save.M, save.AT]] / save.T);

                for J in 3..=save.BNPHCO[save.AT] {
                    save.K = (J + (save.BNPHCO[save.AT] * (I - 1)));
                    save.L = (J - 1);
                    //
                    // Differentiate with respect to EPOCH the Jth
                    // term of the expression for the Ith angle; add
                    // the result to the current sum.
                    //
                    save.DTHETA = (save.DTHETA
                        + ((((save.L as f64) * f64::powi(save.TC, (save.L - 1)))
                            * save.BTCOEF[[save.K, save.AT]])
                            / f64::powi(save.T, (save.L - 1))));
                }

                save.DTHETA = (save.DTHETA * RPD(ctx));
            }

            save.SINTMP = f64::sin(save.THETA);
            save.COSTMP = f64::cos(save.THETA);

            save.SINTH[I] = save.SINTMP;
            save.COSTH[I] = save.COSTMP;
            save.DSINTH[I] = (save.COSTMP * save.DTHETA);
            save.DCOSTH[I] = -(save.SINTMP * save.DTHETA);
        }

        //
        // Adjust RA, DEC, W and their derivatives by the librations
        // and nutations.
        //
        save.RA = (save.RA
            + VDOTG(
                save.BAC.subarray([1, save.AT]),
                save.SINTH.as_slice(),
                save.BNA[save.AT],
            ));
        save.DEC = (save.DEC
            + VDOTG(
                save.BDC.subarray([1, save.AT]),
                save.COSTH.as_slice(),
                save.BND[save.AT],
            ));
        save.W = (save.W
            + VDOTG(
                save.BWC.subarray([1, save.AT]),
                save.SINTH.as_slice(),
                save.BNW[save.AT],
            ));

        save.DRA = (save.DRA
            + VDOTG(
                save.BAC.subarray([1, save.AT]),
                save.DSINTH.as_slice(),
                save.BNA[save.AT],
            ));
        save.DDEC = (save.DDEC
            + VDOTG(
                save.BDC.subarray([1, save.AT]),
                save.DCOSTH.as_slice(),
                save.BND[save.AT],
            ));
        save.DW = (save.DW
            + VDOTG(
                save.BWC.subarray([1, save.AT]),
                save.DSINTH.as_slice(),
                save.BNW[save.AT],
            ));

        //
        // Convert from degrees to radians
        //
        save.RA = (save.RA * RPD(ctx));
        save.DEC = (save.DEC * RPD(ctx));
        save.W = (save.W * RPD(ctx));

        save.DRA = (save.DRA * RPD(ctx));
        save.DDEC = (save.DDEC * RPD(ctx));
        save.DW = (save.DW * RPD(ctx));
        //
        // Convert to Euler angles.
        //
        save.W = intrinsics::DMOD(save.W, TWOPI(ctx));
        save.PHI = (save.RA + HALFPI(ctx));
        save.DELTA = (HALFPI(ctx) - save.DEC);

        save.DPHI = save.DRA;
        save.DDELTA = -save.DDEC;

        if FAILED(ctx) {
            CHKOUT(b"TISBOD", ctx)?;
            return Ok(());
        }

        //
        // Pack the Euler angles and their derivatives into
        // a state vector.
        //
        VPACK(save.W, save.DELTA, save.PHI, save.EULSTA.as_slice_mut());
        VPACK(save.DW, save.DDELTA, save.DPHI, save.EULSTA.subarray_mut(4));

        //
        // Find the state transformation defined by the Euler angle
        // state vector. The transformation matrix TSIPM has the
        // following structure:
        //
        //     -            -
        //    |       :      |
        //    | TIPM  :  0   |
        //    | ......:......|
        //    |       :      |
        //    | DTIPM : TIPM |
        //    |       :      |
        //     -            -
        //
        EUL2XF(save.EULSTA.as_slice(), 3, 1, 3, TSIPM.as_slice_mut(), ctx)?;
    }

    //
    // At this point the base frame PCREF has been determined.
    //
    // If the requested base frame is not base frame associated with the
    // PCK data, adjust the transformation matrix TSIPM to map from the
    // requested frame to the body-fixed frame.
    //
    if (save.REQREF != save.PCREF) {
        //
        // Next get the position transformation from the user specified
        // inertial frame to the native PCK inertial frame.
        //
        IRFROT(save.REQREF, save.PCREF, save.REQ2PC.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"TISBOD", ctx)?;
            return Ok(());
        }

        //
        // Since we're applying an inertial transformation to TSIPM,
        // we can rotate the non-zero blocks of TSIPM. This saves
        // a bunch of double precision multiplications.
        //
        // Extract the upper and lower left blocks of TSIPM.
        //
        for I in 1..=3 {
            for J in 1..=3 {
                save.TIPM[[I, J]] = TSIPM[[I, J]];
                save.DTIPM[[I, J]] = TSIPM[[(I + 3), J]];
            }
        }

        //
        // Rotate the blocks. Note this is a right multiplication.
        //
        MXM(
            save.TIPM.as_slice(),
            save.REQ2PC.as_slice(),
            save.XTIPM.as_slice_mut(),
        );
        MXM(
            save.DTIPM.as_slice(),
            save.REQ2PC.as_slice(),
            save.XDTIPM.as_slice_mut(),
        );

        //
        // Replace the non-zero blocks of TSIPM. This gives us the
        // transformation from the requested frame to the
        // bodyfixed frame.
        //
        for I in 1..=3 {
            for J in 1..=3 {
                TSIPM[[I, J]] = save.XTIPM[[I, J]];
                TSIPM[[(I + 3), (J + 3)]] = save.XTIPM[[I, J]];
                TSIPM[[(I + 3), J]] = save.XDTIPM[[I, J]];
            }
        }
    }
    //
    // That's all folks. Check out and get out.
    //
    CHKOUT(b"TISBOD", ctx)?;
    Ok(())
}
