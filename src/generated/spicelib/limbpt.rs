//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXSRF: i32 = 100;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const CTRSIZ: i32 = 2;
const DSKSHP: i32 = 2;
const ELLSHP: i32 = 1;
const MTHLEN: i32 = 500;
const SUBLEN: i32 = 20;
const CVTLEN: i32 = 20;
const TANGNT: i32 = 1;
const GUIDED: i32 = 2;
const TMTLEN: i32 = 20;
const LMBCRV: i32 = 0;
const UMBRAL: i32 = 1;
const PNMBRL: i32 = 2;
const ACLLEN: i32 = 25;
const CTRCOR: i32 = 1;
const ELLCOR: i32 = 2;
const RNAME: &[u8] = b"LIMBPT";
const IREF: &[u8] = b"J2000";
const CNVLIM: f64 = 0.000000000000000001;
const MAXITR: i32 = 5;
const SSB: i32 = 0;
const MAXL: i32 = 36;
const FRNMLN: i32 = 32;
const LBCELL: i32 = -5;
const MAXIVL: i32 = 1000;
const MAXWIN: i32 = (2 * MAXIVL);
const UBEL: i32 = 9;

struct SaveVars {
    PRVCOR: Vec<u8>,
    PRVLOC: Vec<u8>,
    PRVMTH: Vec<u8>,
    SUBTYP: Vec<u8>,
    PNTBUF: ActualArray2D<f64>,
    LMBTYP: i32,
    LOCCDE: i32,
    NSURF: i32,
    PRVTRG: i32,
    SHAPE: i32,
    SRFLST: StackArray<i32, 100>,
    SVNRAD: i32,
    FIRST: bool,
    PRI: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
    XMIT: bool,
    SVCTR1: StackArray<i32, 2>,
    SVTARG: Vec<u8>,
    SVTCDE: i32,
    SVFND1: bool,
    SVCTR2: StackArray<i32, 2>,
    SVOBSR: Vec<u8>,
    SVOBSC: i32,
    SVFND2: bool,
    SVCTR3: StackArray<i32, 2>,
    SVFREF: Vec<u8>,
    SVFXFC: i32,
    SVCTR4: StackArray<i32, 2>,
    SVCTR5: StackArray<i32, 2>,
    SVRADI: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PRVLOC = vec![b' '; ACLLEN as usize];
        let mut PRVMTH = vec![b' '; MTHLEN as usize];
        let mut SUBTYP = vec![b' '; SUBLEN as usize];
        let mut PNTBUF = ActualArray2D::<f64>::new(1..=3, 1..=MAXWIN);
        let mut LMBTYP: i32 = 0;
        let mut LOCCDE: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut PRVTRG: i32 = 0;
        let mut SHAPE: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut SVNRAD: i32 = 0;
        let mut FIRST: bool = false;
        let mut PRI: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;
        let mut XMIT: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; MAXL as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; MAXL as usize];
        let mut SVOBSC: i32 = 0;
        let mut SVFND2: bool = false;
        let mut SVCTR3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVFREF = vec![b' '; FRNMLN as usize];
        let mut SVFXFC: i32 = 0;
        let mut SVCTR4 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVCTR5 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVRADI = StackArray::<f64, 3>::new(1..=3);

        FIRST = true;
        fstr::assign(&mut PRVCOR, b" ");
        fstr::assign(&mut PRVLOC, b" ");
        fstr::assign(&mut PRVMTH, b" ");
        PRVTRG = 0;
        SVNRAD = 0;
        USECN = false;
        USELT = false;
        USESTL = false;
        XMIT = false;

        Self {
            PRVCOR,
            PRVLOC,
            PRVMTH,
            SUBTYP,
            PNTBUF,
            LMBTYP,
            LOCCDE,
            NSURF,
            PRVTRG,
            SHAPE,
            SRFLST,
            SVNRAD,
            FIRST,
            PRI,
            USECN,
            USELT,
            USESTL,
            XMIT,
            SVCTR1,
            SVTARG,
            SVTCDE,
            SVFND1,
            SVCTR2,
            SVOBSR,
            SVOBSC,
            SVFND2,
            SVCTR3,
            SVFREF,
            SVFXFC,
            SVCTR4,
            SVCTR5,
            SVRADI,
        }
    }
}

/// Limb points on an extended object
///
/// Find limb points on a target body. The limb is the set of points
/// of tangency on the target of rays emanating from the observer.
/// The caller specifies half-planes bounded by the observer-target
/// center vector in which to search for limb points.
///
/// The surface of the target body may be represented either by a
/// triaxial ellipsoid or by topographic data.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DSK](crate::required_reading::dsk)
/// * [FRAMES](crate::required_reading::frames)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [PCK](crate::required_reading::pck)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  METHOD     I   Computation method.
///  TARGET     I   Name of target body.
///  ET         I   Epoch in ephemeris seconds past J2000 TDB.
///  FIXREF     I   Body-fixed, body-centered target body frame.
///  ABCORR     I   Aberration correction.
///  CORLOC     I   Aberration correction locus.
///  OBSRVR     I   Name of observing body.
///  REFVEC     I   Reference vector for cutting half-planes.
///  ROLSTP     I   Roll angular step for cutting half-planes.
///  NCUTS      I   Number of cutting half-planes.
///  SCHSTP     I   Angular step size for searching.
///  SOLTOL     I   Solution convergence tolerance.
///  MAXN       I   Maximum number of entries in output arrays.
///  NPTS       O   Counts of limb points corresponding to cuts.
///  POINTS     O   Limb points.
///  EPOCHS     O   Times associated with limb points.
///  TANGTS     O   Tangent vectors emanating from the observer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  METHOD   is a short string providing parameters defining
///           the computation method to be used. In the syntax
///           descriptions below, items delimited by brackets
///           are optional.
///
///           METHOD may be assigned the following values:
///
///             'TANGENT/DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                 The limb point computation uses topographic data
///                 provided by DSK files (abbreviated as "DSK data"
///                 below) to model the surface of the target body. A
///                 limb point is defined as the point of tangency, on
///                 the surface represented by the DSK data, of a ray
///                 emanating from the observer.
///
///                 Limb points are generated within a specified set
///                 of "cutting" half-planes that have as an edge the
///                 line containing the observer-target vector.
///                 Multiple limb points may be found within a given
///                 half-plane, if the target body shape allows for
///                 this.
///
///                 The surface list specification is optional. The
///                 syntax of the list is
///
///                    <surface 1> [, <surface 2>...]
///
///                 If present, it indicates that data only for the
///                 listed surfaces are to be used; however, data need
///                 not be available for all surfaces in the list. If
///                 the list is absent, loaded DSK data for any
///                 surface associated with the target body are used.
///
///                 The surface list may contain surface names or
///                 surface ID codes. Names containing blanks must
///                 be delimited by double quotes, for example
///
///                    SURFACES = "Mars MEGDR 128 PIXEL/DEG"
///
///                 If multiple surfaces are specified, their names
///                 or IDs must be separated by commas.
///
///                 See the $Particulars section below for details
///                 concerning use of DSK data.
///
///                 This is the highest-accuracy method supported by
///                 this subroutine. It generally executes much more
///                 slowly than the 'GUIDED' method described below.
///
///
///             'GUIDED/DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                 This method uses DSK data as described above, but
///                 limb points generated by this method are "guided"
///                 so as to lie in the limb plane of the target
///                 body's reference ellipsoid, on the target body's
///                 surface. This method produces a unique limb point
///                 for each cutting half-plane. If multiple limb
///                 point candidates lie in a given cutting
///                 half-plane, the outermost one is chosen.
///
///                 This method may be used only with the 'CENTER'
///                 aberration correction locus (see the description
///                 of CORLOC below).
///
///                 Limb points generated by this method are
///                 approximations; they are generally not true
///                 ray-surface tangent points. However, these
///                 approximations can be generated much more quickly
///                 than tangent points.
///
///
///             'TANGENT/ELLIPSOID'
///             'GUIDED/ELLIPSOID'
///
///                 Both of these methods generate limb points on the
///                 target body's reference ellipsoid. The 'TANGENT'
///                 option may be used with any aberration correction
///                 locus, while the 'GUIDED' option may be used only
///                 with the 'CENTER' locus (see the description of
///                 CORLOC below).
///
///                 When the locus is set to 'CENTER', these methods
///                 produce the same results.
///
///
///              Neither case nor white space are significant in
///              METHOD, except within double-quoted strings. For
///              example, the string ' eLLipsoid/tAnGenT ' is valid.
///
///              Within double-quoted strings, blank characters are
///              significant, but multiple consecutive blanks are
///              considered equivalent to a single blank. Case is
///              not significant. So
///
///                 "Mars MEGDR 128 PIXEL/DEG"
///
///              is equivalent to
///
///                 " mars megdr  128  pixel/deg "
///
///              but not to
///
///                 "MARS MEGDR128PIXEL/DEG"
///
///
///  TARGET   is the name of the target body. The target body is
///           an extended ephemeris object.
///
///           The string TARGET is case-insensitive, and leading
///           and trailing blanks in TARGET are not significant.
///           Optionally, you may supply a string containing the
///           integer ID code for the object. For example both
///           'MOON' and '301' are legitimate strings that indicate
///           the Moon is the target body.
///
///           When the target body's surface is represented by a
///           tri-axial ellipsoid, this routine assumes that a
///           kernel variable representing the ellipsoid's radii is
///           present in the kernel pool. Normally the kernel
///           variable would be defined by loading a PCK file.
///
///
///  ET       is the epoch of participation of the observer,
///           expressed as TDB seconds past J2000 TDB: ET is
///           the epoch at which the observer's state is computed.
///
///           When aberration corrections are not used, ET is also
///           the epoch at which the position and orientation of
///           the target body are computed.
///
///           When aberration corrections are used, the position
///           and orientation of the target body are computed at
///           ET-LT, where LT is the one-way light time between the
///           aberration correction locus and the observer. The
///           locus is specified by the input argument CORLOC.
///           See the descriptions of ABCORR and CORLOC below for
///           details.
///
///
///  FIXREF   is the name of a body-fixed reference frame centered
///           on the target body. FIXREF may be any such frame
///           supported by the SPICE system, including built-in
///           frames (documented in the Frames Required Reading)
///           and frames defined by a loaded frame kernel (FK). The
///           string FIXREF is case-insensitive, and leading and
///           trailing blanks in FIXREF are not significant.
///
///           The output limb points in the array POINTS and the
///           output observer-target tangent vectors in the array
///           TANGTS are expressed relative to this reference frame.
///
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the target's position and orientation.
///           Corrections are applied at the location specified by
///           the aberration correction locus argument CORLOC,
///           which is described below.
///
///           For remote sensing applications, where apparent limb
///           points seen by the observer are desired, normally
///           either of the corrections
///
///              'LT+S'
///              'CN+S'
///
///           should be used. The correction 'NONE' may be suitable
///           for cases in which the target is very small and the
///           observer is close to, and has small velocity relative
///           to, the target (e.g. comet Churyumov-Gerasimenko and
///           the Rosetta Orbiter).
///
///           These and the other supported options are described
///           below. ABCORR may be any of the following:
///
///              'NONE'     Apply no correction. Return the
///                         geometric limb points on the target
///                         body.
///
///           Let LT represent the one-way light time between the
///           observer and the aberration correction locus. The
///           following values of ABCORR apply to the "reception"
///           case in which photons depart from the locus at the
///           light-time corrected epoch ET-LT and *arrive* at the
///           observer's location at ET:
///
///
///              'LT'       Correct for one-way light time (also
///                         called "planetary aberration") using a
///                         Newtonian formulation. This correction
///                         yields the locus at the moment it
///                         emitted photons arriving at the
///                         observer at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'LT' option uses two iterations.
///
///                         Both the target position as seen by the
///                         observer, and rotation of the target
///                         body, are corrected for light time.
///
///              'LT+S'     Correct for one-way light time and
///                         stellar aberration using a Newtonian
///                         formulation. This option modifies the
///                         locus obtained with the 'LT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter. These corrections yield
///                         points on the apparent limb.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until the solution converges. Both the
///                         position and rotation of the target
///                         body are corrected for light time.
///
///              'CN+S'     Converged Newtonian light time and
///                         stellar aberration corrections. This
///                         option produces a solution that is at
///                         least as accurate at that obtainable
///                         with the 'LT+S' option. Whether the
///                         'CN+S' solution is substantially more
///                         accurate depends on the geometry of the
///                         participating objects and on the
///                         accuracy of the input data. In all
///                         cases this routine will execute more
///                         slowly when a converged solution is
///                         computed.
///
///           The following values of ABCORR apply to the
///           "transmission" case in which photons depart from the
///           observer's location at ET and arrive at the aberration
///           correction locus at the light-time corrected epoch
///           ET+LT:
///
///              'XLT'      Correct for one-way light time (also
///                         called "planetary aberration") using a
///                         Newtonian formulation. This correction
///                         yields the locus at the moment it
///                         receives photons departing from the
///                         observer at ET.
///
///                         The light time correction uses an
///                         iterative solution of the light time
///                         equation. The solution invoked by the
///                         'LT' option uses two iterations.
///
///                         Both the target position as seen by the
///                         observer, and rotation of the target
///                         body, are corrected for light time.
///
///              'XLT+S'    Correct for one-way transmission light
///                         time and stellar aberration using a
///                         Newtonian formulation. This option
///                         modifies the locus obtained with the 'XLT'
///                         option to account for the observer's
///                         velocity relative to the solar system
///                         barycenter. These corrections yield points
///                         on the apparent limb.
///
///              'XCN'      Converged transmission Newtonian light
///                         time correction. In solving the light time
///                         equation, the 'XCN' correction iterates
///                         until the solution converges. Both the
///                         position and rotation of the target body
///                         are corrected for light time.
///
///              'XCN+S'    Converged transmission Newtonian light
///                         time and stellar aberration corrections.
///                         This option produces a solution that is at
///                         least as accurate at that obtainable with
///                         the `XLT+S' option. Whether the 'XCN+S'
///                         solution is substantially more accurate
///                         depends on the geometry of the
///                         participating objects and on the accuracy
///                         of the input data. In all cases this
///                         routine will execute more slowly when a
///                         converged solution is computed.
///
///
///  CORLOC   is a string specifying the aberration correction
///           locus: the point or set of points for which
///           aberration corrections are performed. CORLOC may be
///           assigned the values:
///
///              'CENTER'
///
///                  Light time and stellar aberration corrections
///                  are applied to the vector from the observer to
///                  the center of the target body. The one way
///                  light time from the target center to the
///                  observer is used to determine the epoch at
///                  which the target body orientation is computed.
///
///                  This choice is appropriate for small target
///                  objects for which the light time from the
///                  surface to the observer varies little across
///                  the entire target. It may also be appropriate
///                  for large, nearly ellipsoidal targets when the
///                  observer is very far from the target.
///
///                  Computation speed for this option is faster
///                  than for the 'ELLIPSOID LIMB' option.
///
///              'ELLIPSOID LIMB'
///
///                  Light time and stellar aberration corrections
///                  are applied to individual limb points on the
///                  reference ellipsoid. For a limb point on the
///                  surface described by topographic data, lying
///                  in a specified cutting half-plane, the unique
///                  reference ellipsoid limb point in the same
///                  half-plane is used as the locus of the
///                  aberration corrections.
///
///                  This choice is appropriate for large target
///                  objects for which the light time from the limb
///                  to the observer is significantly different
///                  from the light time from the target center to
///                  the observer.
///
///                  Because aberration corrections are repeated for
///                  individual limb points, computational speed for
///                  this option is relatively slow.
///
///
///  OBSRVR   is the name of the observing body. The observing body
///           is an ephemeris object: it typically is a spacecraft,
///           the earth, or a surface point on the earth. OBSRVR is
///           case-insensitive, and leading and trailing blanks in
///           OBSRVR are not significant. Optionally, you may
///           supply a string containing the integer ID code for
///           the object. For example both 'MOON' and '301' are
///           legitimate strings that indicate the Moon is the
///           observer.
///
///
///  REFVEC,
///  ROLSTP,
///  NCUTS    are, respectively, a reference vector, a roll step
///           angle, and a count of cutting half-planes.
///
///           REFVEC defines the first of a sequence of cutting
///           half-planes in which limb points are to be found.
///           Each cutting half-plane has as its edge the line
///           containing the observer-target vector; the first
///           half-plane contains REFVEC.
///
///           REFVEC is expressed in the body-fixed reference frame
///           designated by FIXREF.
///
///           ROLSTP is an angular step by which to roll the
///           cutting half-planes about the observer-target vector.
///           The first half-plane is aligned with REFVEC; the Ith
///           half-plane is rotated from REFVEC about the
///           observer-target vector in the counter-clockwise
///           direction by (I-1)*ROLSTP. Units are radians.
///           ROLSTP should be set to
///
///              2*pi/NCUTS
///
///           to generate an approximately uniform distribution of
///           limb points along the limb.
///
///           NCUTS is the number of cutting half-planes used to
///           find limb points; the angular positions of
///           consecutive half-planes increase in the positive
///           sense (counterclockwise) about the target-observer
///           vector and are distributed roughly equally about that
///           vector: each half-plane has angular separation of
///           approximately
///
///              ROLSTP radians
///
///           from each of its neighbors. When the aberration
///           correction locus is set to 'CENTER', the angular
///           separation is the value above, up to round-off. When
///           the locus is 'ELLIPSOID LIMB', the separations are
///           less uniform due to differences in the aberration
///           corrections used for the respective limb points.
///
///
///  SCHSTP,
///  SOLTOL   are used only for DSK-based surfaces. These inputs
///           are, respectively, the search angular step size and
///           solution convergence tolerance used to find tangent
///           rays and associated limb points within each cutting
///           half plane. These values are used when the METHOD
///           argument includes the 'TANGENT' option. In this case,
///           limb points are found by a two-step search process:
///
///              1) Bracketing: starting with the direction
///                 opposite the observer-target vector, rays
///                 emanating from the observer are generated
///                 within the half-plane at successively greater
///                 angular separations from the initial direction,
///                 where the increment of angular separation is
///                 SCHSTP. The rays are tested for intersection
///                 with the target surface. When a transition
///                 between non-intersection to intersection is
///                 found, the angular separation of a tangent ray
///                 has been bracketed.
///
///              2) Root finding: each time a tangent ray is
///                 bracketed, a search is done to find the angular
///                 separation from the starting direction at which
///                 a tangent ray exists. The search terminates
///                 when successive rays are separated by no more
///                 than SOLTOL. When the search converges, the
///                 last ray-surface intersection point found in
///                 the convergence process is considered to be a
///                 limb point.
///
///
///            SCHSTP and SOLTOL have units of radians.
///
///            Target bodies with simple surfaces---for example,
///            convex shapes---will have a single limb point within
///            each cutting half-plane. For such surfaces, SCHSTP
///            can be set large enough so that only one bracketing
///            step is taken. A value greater than pi, for example
///            4.D0, is recommended.
///
///            Target bodies with complex surfaces can have
///            multiple limb points within a given cutting
///            half-plane. To find all limb points, SCHSTP must be
///            set to a value smaller than the angular separation
///            of any two limb points in any cutting half-plane,
///            where the vertex of the angle is the observer.
///            SCHSTP must not be too small, or the search will be
///            excessively slow.
///
///            For both kinds of surfaces, SOLTOL must be chosen so
///            that the results will have the desired precision.
///            Note that the choice of SOLTOL required to meet a
///            specified bound on limb point height errors depends
///            on the observer-target distance.
///
///
///  MAXN     is the maximum number of limb points that can be
///           stored in the output array POINTS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NPTS     is an array of counts of limb points within the
///           specified set of cutting half-planes. The Ith
///           element of NPTS is the limb point count in the Ith
///           half-plane. NPTS should be declared with length
///           at least NCUTS.
///
///           For most target bodies, there will be one limb point
///           per half-plane. For complex target shapes, the limb
///           point count in a given half-plane can be greater
///           than one (see example 3 below), and it can be zero.
///
///
///  POINTS   is an array containing the limb points found by this
///           routine. Sets of limb points associated with
///           half-planes are ordered by the indices of the
///           half-planes in which they're found. The limb points
///           in a given half-plane are ordered by decreasing
///           angular separation from the observer-target
///           direction; the outermost limb point in a given
///           half-plane is the first of that set.
///
///           The limb points for the half-plane containing REFVEC
///           occupy array elements
///
///              POINTS(1,1) through POINTS(3,NPTS(1))
///
///           Limb points for the second half plane occupy
///           elements
///
///              POINTS(1, NPTS(1)+1       ) through
///              POINTS(3, NPTS(1)+NPTS(2) )
///
///           and so on.
///
///           POINTS should be declared with dimensions
///
///              ( 3, MAXN )
///
///           Limb points are expressed in the reference frame
///           designated by FIXREF. For each limb point, the
///           orientation of the frame is evaluated at the epoch
///           corresponding to the limb point; the epoch is
///           provided in the output array EPOCHS (described
///           below).
///
///           Units of the limb points are km.
///
///
///  EPOCHS   is an array of epochs associated with the limb
///           points, accounting for light time if aberration
///           corrections are used. EPOCHS contains one element
///           for each limb point. EPOCHS should be declared
///           with length
///
///              MAXN
///
///           The element
///
///              EPOCHS(I)
///
///           is associated with the limb point
///
///              POINTS(J,I), J = 1 to 3
///
///           If CORLOC is set to 'CENTER', all values of EPOCHS
///           will be the epoch associated with the target body
///           center. That is, if aberration corrections are used,
///           and if LT is the one-way light time from the target
///           center to the observer, the elements of EPOCHS will
///           all be set to
///
///              ET - LT
///
///           If CORLOC is set to 'ELLIPSOID LIMB', all values of
///           EPOCHS for the limb points in a given half plane
///           will be those for the reference ellipsoid limb point
///           in that half plane. That is, if aberration
///           corrections are used, and if LT(I) is the one-way
///           light time to the observer from the reference
///           ellipsoid limb point in the Ith half plane, the
///           elements of EPOCHS for that half plane will all be
///           set to
///
///              ET - LT(I)
///
///           When the target shape is given by DSK data, there
///           normally will be a small difference in the light
///           time between an actual limb point and that implied
///           by the corresponding element of EPOCHS. See the
///           description of TANGTS below.
///
///
///  TANGTS   is an array of tangent vectors connecting the
///           observer to the limb points. The tangent vectors are
///           expressed in the frame designated by FIXREF. For the
///           Ith vector, the orientation of the frame is
///           evaluated at the Ith epoch provided in the output
///           array EPOCHS (described above).
///
///           TANGTS should be declared with dimensions
///
///              ( 3, MAXN )
///
///           The elements
///
///              TANGTS(J,I), J = 1 to 3
///
///           are associated with the limb point
///
///              POINTS(J,I), J = 1 to 3
///
///           Units of the tangent vectors are km.
///
///           When the target shape is given by DSK data, there
///           normally will be a small difference in the light
///           time between an actual limb point and that implied
///           by the corresponding element of EPOCHS. This
///           difference will affect the orientation of the target
///           body-fixed frame and the output tangent vectors
///           returned in the array TANGTS. All other factors
///           being equal, the error in the tangent vector due to
///           the light time error is proportional to the
///           observer-target distance.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified aberration correction is unrecognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If either the target or observer input strings cannot be
///      converted to an integer ID code, the error
///      SPICE(IDCODENOTFOUND) is signaled.
///
///  3)  If OBSRVR and TARGET map to the same NAIF integer ID code,
///      the error SPICE(BODIESNOTDISTINCT) is signaled.
///
///  4)  If the input target body-fixed frame FIXREF is not
///      recognized, the error SPICE(NOFRAME) is signaled. A frame
///      name may fail to be recognized because a required frame
///      specification kernel has not been loaded; another cause is a
///      misspelling of the frame name.
///
///  5)  If the input frame FIXREF is not centered at the target body,
///      the error SPICE(INVALIDFRAME) is signaled.
///
///  6)  If the input argument METHOD is not recognized, the error
///      SPICE(INVALIDMETHOD) is signaled by either this routine or a
///      routine in the call tree of this routine.
///
///  7)  If METHOD contains an invalid limb type, the error
///      SPICE(INVALIDLIMBTYPE) is signaled.
///
///  8)  If the target and observer have distinct identities but are
///      at the same location, the error SPICE(NOSEPARATION) is
///      signaled.
///
///  9)  If insufficient ephemeris data have been loaded prior to
///      calling LIMBPT, an error is signaled by a routine in
///      the call tree of this routine. When light time correction is
///      used, sufficient ephemeris data must be available to
///      propagate the states of both observer and target to the solar
///      system barycenter.
///
///  10) If the computation method requires an ellipsoidal target shape
///      and triaxial radii of the target body have not been loaded
///      into the kernel pool prior to calling LIMBPT, an error is
///      signaled by a routine in the call tree of this routine.
///
///      When the target shape is modeled by topographic data, radii
///      of the reference triaxial ellipsoid are still required if
///      the aberration correction locus is ELLIPSOID LIMB or if
///      the limb point generation method is GUIDED.
///
///  11) If the radii are available in the kernel pool but the count
///      of radii values is not three, the error SPICE(BADRADIUSCOUNT)
///      is signaled.
///
///  12) If the target body's shape is modeled as an ellipsoid, and if
///      any of the radii of the target body are non-positive, an error
///      is signaled by a routine in the call tree of this routine. The
///      target must be an extended body.
///
///  13) If PCK data specifying the target body-fixed frame orientation
///      have not been loaded prior to calling LIMBPT, an error is
///      signaled by a routine in the call tree of this routine.
///
///  14) If METHOD specifies that the target surface is represented by
///      DSK data, and no DSK files are loaded for the specified
///      target, an error is signaled by a routine in the call tree
///      of this routine.
///
///  15) If the array bound MAXN is less than 1, the error
///      SPICE(INVALIDSIZE) is signaled.
///
///  16) If the number of cutting half-planes specified by NCUTS
///      is negative or greater than MAXN, the error
///      SPICE(INVALIDCOUNT) is signaled.
///
///  17) If the aberration correction locus is not recognized, the
///      error SPICE(INVALIDLOCUS) is signaled.
///
///  18) If the aberration correction locus is 'ELLIPSOID LIMB'
///      but limb type is not 'TANGENT', the error
///      SPICE(BADLIMBLOCUSMIX) is signaled.
///
///  19) If the reference vector REFVEC is the zero vector, the
///      error SPICE(ZEROVECTOR) is signaled.
///
///  20) If the reference vector REFVEC and the observer target
///      vector are linearly dependent, the error
///      SPICE(DEGENERATECASE) is signaled.
///
///  21) If the limb computation uses the target ellipsoid limb
///      plane, and the limb plane normal and reference vector
///      REFVEC are linearly dependent, the error
///      SPICE(DEGENERATECASE) is signaled.
///
///  22) If the limb points cannot all be stored in the output POINTS
///      array, the error SPICE(OUTOFROOM) is signaled.
///
///  23) If the surface is represented by DSK data, and if the search
///      step is non-positive, the error SPICE(INVALIDSEARCHSTEP) is
///      signaled.
///
///  24) If the surface is represented by DSK data, and if the search
///      tolerance is non-positive, the error SPICE(INVALIDTOLERANCE)
///      is signaled.
///
///  25) If the roll step is non-positive and NCUTS is greater
///      than 1, the error SPICE(INVALIDROLLSTEP) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  Appropriate kernels must be loaded by the calling program before
///  this routine is called.
///
///  The following data are required:
///
///  -  SPK data: ephemeris data for target and observer must be
///     loaded. If aberration corrections are used, the states of
///     target and observer relative to the solar system barycenter
///     must be calculable from the available ephemeris data.
///     Typically ephemeris data are made available by loading one
///     or more SPK files via FURNSH.
///
///  -  Target body orientation data: these may be provided in a text
///     or binary PCK file. In some cases, target body orientation
///     may be provided by one more more CK files. In either case,
///     data are made available by loading the files via FURNSH.
///
///  -  Shape data for the target body:
///
///        PCK data:
///
///           If the target body shape is modeled as an ellipsoid,
///           triaxial radii for the target body must be loaded into
///           the kernel pool. Typically this is done by loading a
///           text PCK file via FURNSH.
///
///           Triaxial radii are also needed if the target shape is
///           modeled by DSK data but one or both of the GUIDED limb
///           definition method or the ELLIPSOID LIMB aberration
///           correction locus are selected.
///
///        DSK data:
///
///           If the target shape is modeled by DSK data, DSK files
///           containing topographic data for the target body must be
///           loaded. If a surface list is specified, data for at
///           least one of the listed surfaces must be loaded.
///
///  The following data may be required:
///
///  -  Frame data: if a frame definition is required to convert the
///     observer and target states to the body-fixed frame of the
///     target, that definition must be available in the kernel
///     pool. Typically the definition is supplied by loading a
///     frame kernel via FURNSH.
///
///  -  Surface name-ID associations: if surface names are specified
///     in `method', the association of these names with their
///     corresponding surface ID codes must be established by
///     assignments of the kernel variables
///
///        NAIF_SURFACE_NAME
///        NAIF_SURFACE_CODE
///        NAIF_SURFACE_BODY
///
///     Normally these associations are made by loading a text
///     kernel containing the necessary assignments. An example
///     of such a set of assignments is
///
///        NAIF_SURFACE_NAME += 'Mars MEGDR 128 PIXEL/DEG'
///        NAIF_SURFACE_CODE += 1
///        NAIF_SURFACE_BODY += 499
///
///  -  SCLK data: if the target body's orientation is provided by
///     CK files, an associated SCLK kernel must be loaded.
///
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  Using DSK data
///  ==============
///
///     DSK loading and unloading
///     -------------------------
///
///     DSK files providing data used by this routine are loaded by
///     calling FURNSH and can be unloaded by calling UNLOAD or
///     KCLEAR. See the documentation of FURNSH for limits on numbers
///     of loaded DSK files.
///
///     For run-time efficiency, it's desirable to avoid frequent
///     loading and unloading of DSK files. When there is a reason to
///     use multiple versions of data for a given target body---for
///     example, if topographic data at varying resolutions are to be
///     used---the surface list can be used to select DSK data to be
///     used for a given computation. It is not necessary to unload
///     the data that are not to be used. This recommendation presumes
///     that DSKs containing different versions of surface data for a
///     given body have different surface ID codes.
///
///
///     DSK data priority
///     -----------------
///
///     A DSK coverage overlap occurs when two segments in loaded DSK
///     files cover part or all of the same domain---for example, a
///     given longitude-latitude rectangle---and when the time
///     intervals of the segments overlap as well.
///
///     When DSK data selection is prioritized, in case of a coverage
///     overlap, if the two competing segments are in different DSK
///     files, the segment in the DSK file loaded last takes
///     precedence. If the two segments are in the same file, the
///     segment located closer to the end of the file takes
///     precedence.
///
///     When DSK data selection is unprioritized, data from competing
///     segments are combined. For example, if two competing segments
///     both represent a surface as sets of triangular plates, the
///     union of those sets of plates is considered to represent the
///     surface.
///
///     Currently only unprioritized data selection is supported.
///     Because prioritized data selection may be the default behavior
///     in a later version of the routine, the UNPRIORITIZED keyword is
///     required in the METHOD argument.
///
///
///     Syntax of the METHOD input argument
///     -----------------------------------
///
///     The keywords and surface list in the METHOD argument
///     are called "clauses." The clauses may appear in any
///     order, for example
///
///        TANGENT/DSK/UNPRIORITIZED/<surface list>
///        DSK/TANGENT/<surface list>/UNPRIORITIZED
///        UNPRIORITIZED/<surface list>/DSK/TANGENT
///
///     The simplest form of the METHOD argument specifying use of
///     DSK data is one that lacks a surface list, for example:
///
///        'TANGENT/DSK/UNPRIORITIZED'
///        'GUIDED/DSK/UNPRIORITIZED'
///
///     For applications in which all loaded DSK data for the target
///     body are for a single surface, and there are no competing
///     segments, the above strings suffice. This is expected to be
///     the usual case.
///
///     When, for the specified target body, there are loaded DSK
///     files providing data for multiple surfaces for that body, the
///     surfaces to be used by this routine for a given call must be
///     specified in a surface list, unless data from all of the
///     surfaces are to be used together.
///
///     The surface list consists of the string
///
///        SURFACES =
///
///     followed by a comma-separated list of one or more surface
///     identifiers. The identifiers may be names or integer codes in
///     string format. For example, suppose we have the surface
///     names and corresponding ID codes shown below:
///
///        Surface Name                              ID code
///        ------------                              -------
///        'Mars MEGDR 128 PIXEL/DEG'                1
///        'Mars MEGDR 64 PIXEL/DEG'                 2
///        'Mars_MRO_HIRISE'                         3
///
///     If data for all of the above surfaces are loaded, then
///     data for surface 1 can be specified by either
///
///        'SURFACES = 1'
///
///     or
///
///        'SURFACES = "Mars MEGDR 128 PIXEL/DEG"'
///
///     Double quotes are used to delimit the surface name because
///     it contains blank characters.
///
///     To use data for surfaces 2 and 3 together, any
///     of the following surface lists could be used:
///
///        'SURFACES = 2, 3'
///
///        'SURFACES = "Mars MEGDR  64 PIXEL/DEG", 3'
///
///        'SURFACES = 2, Mars_MRO_HIRISE'
///
///        'SURFACES = "Mars MEGDR 64 PIXEL/DEG", Mars_MRO_HIRISE'
///
///     An example of a METHOD argument that could be constructed
///     using one of the surface lists above is
///
///  'TANGENT/DSK/UNPRIORITIZED/SURFACES= "Mars MEGDR 64 PIXEL/DEG",3'
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
///
///  1) Find apparent limb points on Phobos as seen from Mars.
///
///     Due to Phobos' irregular shape, the TANGENT limb point
///     definition will used. It suffices to compute light time and
///     stellar aberration corrections for the center of Phobos, so
///     the CENTER aberration correction locus will be used. Use
///     converged Newtonian light time and stellar aberration
///     corrections in order to model the apparent position and
///     orientation of Phobos.
///
///     For comparison, compute limb points using both ellipsoid
///     and topographic shape models.
///
///     Use the target body-fixed +Z axis as the reference direction
///     for generating cutting half-planes. This choice enables the
///     user to see whether the first limb point is near the target's
///     north pole.
///
///     For each option, use just three cutting half-planes, in order
///     to keep the volume of output manageable. In most applications,
///     the number of cuts and the number of resulting limb points
///     would be much greater.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: limbpt_ex1.tm
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
///           File name                        Contents
///           ---------                        --------
///           de430.bsp                        Planetary ephemeris
///           mar097.bsp                       Mars satellite ephemeris
///           pck00010.tpc                     Planet orientation and
///                                            radii
///           naif0011.tls                     Leapseconds
///           phobos512.bds                    DSK based on
///                                            Gaskell ICQ Q=512
///                                            Phobos plate model
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'pck00010.tpc',
///                               'naif0011.tls',
///                               'phobos512.bds' )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///     C
///     C     LIMBPT example 1
///     C
///     C        Find limb points on Phobos as seen from Mars.
///     C
///     C        Compute limb points using the tangent definition.
///     C        Perform aberration corrections for the target center.
///     C        Use both ellipsoid and DSK shape models.
///     C
///           PROGRAM LIMBPT_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      PI
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'limbpt_ex1.tm' )
///
///           CHARACTER*(*)         FM1
///           PARAMETER           ( FM1     =  '(A,F20.9)' )
///
///           CHARACTER*(*)         FM2
///           PARAMETER           ( FM2     =  '(1X,3F20.9)' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 20 )
///
///           INTEGER               MTHLEN
///           PARAMETER           ( MTHLEN = 50 )
///
///           INTEGER               NMETH
///           PARAMETER           ( NMETH  = 2 )
///
///           INTEGER               MAXN
///           PARAMETER           ( MAXN = 10000 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(CORLEN)    CORLOC
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(MTHLEN)    METHOD ( NMETH )
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(BDNMLN)    TARGET
///
///           DOUBLE PRECISION      DELROL
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      POINTS ( 3, MAXN )
///           DOUBLE PRECISION      ROLL
///           DOUBLE PRECISION      SCHSTP
///           DOUBLE PRECISION      SOLTOL
///           DOUBLE PRECISION      TANGTS ( 3, MAXN )
///           DOUBLE PRECISION      TRGEPS ( MAXN )
///           DOUBLE PRECISION      Z      ( 3 )
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               K
///           INTEGER               M
///           INTEGER               NCUTS
///           INTEGER               NPTS   ( MAXN )
///           INTEGER               START
///
///     C
///     C     Initial values
///     C
///           DATA                  METHOD /
///          .                        'TANGENT/ELLIPSOID',
///          .                        'TANGENT/DSK/UNPRIORITIZED'
///          .                             /
///           DATA                  Z      / 0.D0, 0.D0, 1.D0 /
///     C
///     C     Load kernel files via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///     C
///     C     Set target, observer, and target body-fixed,
///     C     body-centered reference frame.
///     C
///           OBSRVR = 'MARS'
///           TARGET = 'PHOBOS'
///           FIXREF = 'IAU_PHOBOS'
///     C
///     C     Set aberration correction and correction locus.
///     C
///           ABCORR = 'CN+S'
///           CORLOC = 'CENTER'
///     C
///     C     Convert the UTC request time string seconds past
///     C     J2000, TDB.
///     C
///           CALL STR2ET ( '2008 AUG 11 00:00:00', ET )
///     C
///     C     Compute a set of limb points using light time and
///     C     stellar aberration corrections. Use both ellipsoid
///     C     and DSK shape models. Use a step size of 100
///     C     microradians to ensure we don't miss the limb.
///     C     Set the convergence tolerance to 100 nanoradians,
///     C     which will limit the height error to about 1 meter.
///     C     Compute 3 limb points for each computation method.
///     C
///           SCHSTP = 1.D-4
///           SOLTOL = 1.D-7
///           NCUTS  = 3
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Observer:       '//OBSRVR
///           WRITE (*,*) 'Target:         '//TARGET
///           WRITE (*,*) 'Frame:          '//FIXREF
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Number of cuts: ', NCUTS
///           WRITE (*,*) ' '
///
///           DELROL = 2*PI() / NCUTS
///
///           DO I = 1, NMETH
///
///              CALL LIMBPT ( METHOD(I), TARGET, ET,     FIXREF,
///          .                 ABCORR,    CORLOC, OBSRVR, Z,
///          .                 DELROL,    NCUTS,  SCHSTP, SOLTOL,
///          .                 MAXN,      NPTS,   POINTS, TRGEPS,
///          .                 TANGTS                            )
///     C
///     C        Write the results.
///     C
///              WRITE(*,*) ' '
///              WRITE(*,*) 'Computation method = ', METHOD(I)
///              WRITE(*,*) 'Locus              = ', CORLOC
///              WRITE(*,*) ' '
///
///
///              START  = 0
///
///              DO J = 1, NCUTS
///
///                 ROLL = (J-1) * DELROL
///
///                 WRITE(*,*)   ' '
///                 WRITE(*,FM1) '  Roll angle (deg) = ', ROLL * DPR()
///                 WRITE(*,FM1) '     Target epoch  = ', TRGEPS(J)
///                 WRITE(*,*)   '    Number of limb points at this '
///          .      //           'roll angle: ',
///          .                   NPTS(J)
///
///                 WRITE (*,*) '      Limb points'
///
///                 DO K = 1, NPTS(J)
///                    WRITE (*,FM2) ( POINTS(M,K+START), M = 1, 3 )
///                 END DO
///
///                 START = START + NPTS(J)
///
///              END DO
///
///              WRITE (*,*) ' '
///
///           END DO
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Observer:       MARS
///      Target:         PHOBOS
///      Frame:          IAU_PHOBOS
///
///      Number of cuts:            3
///
///
///      Computation method = TANGENT/ELLIPSOID
///      Locus              = CENTER
///
///
///       Roll angle (deg) =          0.000000000
///          Target epoch  =  271684865.152078211
///          Number of limb points at this roll angle:            1
///            Limb points
///               0.016445326        -0.000306114         9.099992715
///
///       Roll angle (deg) =        120.000000000
///          Target epoch  =  271684865.152078211
///          Number of limb points at this roll angle:            1
///            Limb points
///              -0.204288375        -9.235230829        -5.333237706
///
///       Roll angle (deg) =        240.000000000
///          Target epoch  =  271684865.152078211
///          Number of limb points at this roll angle:            1
///            Limb points
///               0.242785221         9.234520095        -5.333231253
///
///
///      Computation method = TANGENT/DSK/UNPRIORITIZED
///      Locus              = CENTER
///
///
///       Roll angle (deg) =          0.000000000
///          Target epoch  =  271684865.152078211
///          Number of limb points at this roll angle:            1
///            Limb points
///              -0.398901673         0.007425178         9.973720555
///
///       Roll angle (deg) =        120.000000000
///          Target epoch  =  271684865.152078211
///          Number of limb points at this roll angle:            1
///            Limb points
///              -0.959300281        -8.537573427        -4.938700447
///
///       Roll angle (deg) =        240.000000000
///          Target epoch  =  271684865.152078211
///          Number of limb points at this roll angle:            1
///            Limb points
///              -1.380536729         9.714334047        -5.592916790
///
///
///  2) Find apparent limb points on Mars as seen from the earth.
///     Compare results using different computation options.
///
///     Use both the TANGENT and GUIDED limb point definitions. For
///     the tangent limb points, use the ELLIPSOID LIMB aberration
///     correction locus; for the guided limb points, use the CENTER
///     locus. For the GUIDED limb points, also compute the distance
///     of each point from the corresponding point computed using the
///     TANGENT definition.
///
///     For comparison, compute limb points using both ellipsoid and
///     topographic shape models.
///
///     Check the limb points by computing the apparent emission
///     angles at each limb point.
///
///     For the ellipsoid shape model, we expect emission angles very
///     close to 90 degrees, since each illumination angle calculation
///     is done using aberration corrections for the limb point at
///     which the angles are measured.
///
///     Use the target body-fixed +Z axis as the reference direction
///     for generating cutting half-planes. This choice enables the
///     user to see whether the first limb point is near the target's
///     north pole.
///
///     For each option, use just three cutting half-planes, in order
///     to keep the volume of output manageable. In most applications,
///     the number of cuts and the number of resulting limb points
///     would be much greater.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: limbpt_ex2.tm
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
///           File name                        Contents
///           ---------                        --------
///           de430.bsp                        Planetary ephemeris
///           mar097.bsp                       Mars satellite ephemeris
///           pck00010.tpc                     Planet orientation and
///                                            radii
///           naif0011.tls                     Leapseconds
///           megr90n000cb_plate.bds           DSK plate model based on
///                                            MGS MOLAR MEGDR DEM,
///                                            resolution 4
///                                            pixels/degree.
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'pck00010.tpc',
///                               'naif0011.tls',
///                               'megr90n000cb_plate.bds' )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///     C
///     C     LIMBPT example 2
///     C
///     C        Find limb points on Mars as seen from the earth.
///     C
///     C        Compute limb points using both the tangent and
///     C        "guided" definitions.
///     C
///     C        For the tangent limb points, perform aberration
///     C        corrections for the reference ellipsoid limb.
///     C
///     C        Check limb points by computing emission angles at
///     C        each point.
///     C
///     C        Use both ellipsoid and DSK shape models.
///     C
///           PROGRAM LIMBPT_EX2
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      PI
///           DOUBLE PRECISION      VDIST
///           DOUBLE PRECISION      VNORM
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META    = 'limbpt_ex2.tm' )
///
///           CHARACTER*(*)         FM1
///           PARAMETER           ( FM1     =  '(A,F20.9)' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 20 )
///
///           INTEGER               MTHLEN
///           PARAMETER           ( MTHLEN = 50 )
///
///           INTEGER               NMETH
///           PARAMETER           ( NMETH  = 3 )
///
///           INTEGER               MAXN
///           PARAMETER           ( MAXN   = 100 )
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(CORLEN)    CORLOC ( NMETH )
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(MTHLEN)    ILUMTH ( NMETH )
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(BDNMLN)    TARGET
///           CHARACTER*(MTHLEN)    METHOD ( NMETH )
///
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      DELROL
///           DOUBLE PRECISION      DIST
///           DOUBLE PRECISION      EMISSN
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      PHASE
///           DOUBLE PRECISION      POINTS ( 3, MAXN )
///           DOUBLE PRECISION      SVPNTS ( 3, MAXN )
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      RADII  ( 3 )
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      ROLL
///           DOUBLE PRECISION      RP
///           DOUBLE PRECISION      SCHSTP
///           DOUBLE PRECISION      SOLAR
///           DOUBLE PRECISION      SOLTOL
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      TANGTS ( 3, MAXN )
///           DOUBLE PRECISION      TRGEPC
///           DOUBLE PRECISION      TRGEPS ( MAXN )
///           DOUBLE PRECISION      Z      ( 3 )
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               K
///           INTEGER               M
///           INTEGER               N
///           INTEGER               NCUTS
///           INTEGER               NPTS   ( MAXN )
///           INTEGER               START
///
///     C
///     C     Initial values
///     C
///           DATA                  CORLOC /
///          .                        'ELLIPSOID LIMB',
///          .                        'ELLIPSOID LIMB',
///          .                        'CENTER'
///          .                             /
///
///           DATA                  ILUMTH /
///          .                        'ELLIPSOID',
///          .                        'DSK/UNPRIORITIZED',
///          .                        'DSK/UNPRIORITIZED'
///          .                             /
///
///           DATA                  METHOD /
///          .                        'TANGENT/ELLIPSOID',
///          .                        'TANGENT/DSK/UNPRIORITIZED',
///          .                        'GUIDED/DSK/UNPRIORITIZED'
///          .                             /
///
///           DATA                  Z      / 0.D0, 0.D0, 1.D0 /
///     C
///     C     Load kernel files via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///     C
///     C     Set target, observer, and target body-fixed,
///     C     body-centered reference frame.
///     C
///           OBSRVR = 'EARTH'
///           TARGET = 'MARS'
///           FIXREF = 'IAU_MARS'
///     C
///     C     Set the aberration correction. We'll set the
///     C     correction locus below.
///     C
///           ABCORR = 'CN+S'
///     C
///     C     Convert the UTC request time string seconds past
///     C     J2000, TDB.
///     C
///           CALL STR2ET ( '2008 AUG 11 00:00:00', ET )
///     C
///     C     Look up the target body's radii. We'll use these to
///     C     convert Cartesian to planetographic coordinates. Use
///     C     the radii to compute the flattening coefficient of
///     C     the reference ellipsoid.
///     C
///           CALL BODVRD ( TARGET, 'RADII', 3, N, RADII )
///     C
///     C     Compute the flattening coefficient for planetodetic
///     C     coordinates
///     C
///           RE = RADII(1)
///           RP = RADII(3)
///           F  = ( RE - RP ) / RE
///     C
///     C     Compute a set of limb points using light time and
///     C     stellar aberration corrections. Use both ellipsoid
///     C     and DSK shape models.
///     C
///     C     Obtain the observer-target distance at ET.
///     C
///           CALL SPKPOS ( TARGET, ET,  'J2000', ABCORR,
///          .              OBSRVR, POS, LT              )
///           DIST = VNORM( POS )
///     C
///     C     Set the angular step size so that a single step will
///     C     be taken in the root bracketing process; that's all
///     C     that is needed since we don't expect to have multiple
///     C     limb points in any cutting half-plane.
///     C
///           SCHSTP = 4.D0
///     C
///     C     Set the convergence tolerance to minimize the height
///     C     error. We can't achieve the 1 millimeter precision
///     C     suggested by the formula because the earth-Mars
///     C     distance is about 3.5e8 km. Compute 3 limb points
///     C     for each computation method.
///     C
///           SOLTOL = 1.D-6/DIST
///     C
///     C     Set the number of cutting half-planes and roll step.
///     C
///           NCUTS  = 3
///           DELROL = 2*PI() / NCUTS
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Observer:       '//OBSRVR
///           WRITE (*,*) 'Target:         '//TARGET
///           WRITE (*,*) 'Frame:          '//FIXREF
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Number of cuts: ', NCUTS
///
///
///           DO I = 1, NMETH
///
///              CALL LIMBPT ( METHOD(I), TARGET,    ET,     FIXREF,
///          .                 ABCORR,    CORLOC(I), OBSRVR, Z,
///          .                 DELROL,    NCUTS,     SCHSTP, SOLTOL,
///          .                 MAXN,      NPTS,      POINTS, TRGEPS,
///          .                 TANGTS                                )
///     C
///     C        Write the results.
///     C
///              WRITE(*,*) ' '
///              WRITE(*,*) 'Computation method = ', METHOD(I)
///              WRITE(*,*) 'Locus              = ', CORLOC(I)
///
///
///              START  = 0
///
///              DO J = 1, NCUTS
///
///                 ROLL = (J-1) * DELROL
///
///                 WRITE(*,*)   ' '
///                 WRITE(*,FM1) '   Roll angle (deg) = ', ROLL * DPR()
///                 WRITE(*,FM1) '     Target epoch   = ', TRGEPS(J)
///                 WRITE(*,*)   '    Number of limb points at this '
///          .      //           'roll angle: ',
///          .                   NPTS(J)
///
///                 DO K = 1, NPTS(J)
///
///                    WRITE (*,*) '    Limb point planetodetic '
///          .         //          'coordinates:'
///
///                    CALL RECGEO ( POINTS(1,K+START), RE,  F,
///          .                       LON,               LAT, ALT )
///
///                    WRITE (*,FM1) '      Longitude      (deg): ',
///          .                       LON*DPR()
///                    WRITE (*,FM1) '      Latitude       (deg): ',
///          .                       LAT*DPR()
///                    WRITE (*,FM1) '      Altitude        (km): ',
///          .                       ALT
///
///     C
///     C              Get illumination angles for this limb point.
///     C
///                    M = K+START
///
///                    CALL ILUMIN ( ILUMTH,      TARGET, ET,
///          .                       FIXREF,      ABCORR, OBSRVR,
///          .                       POINTS(1,M), TRGEPC, SRFVEC,
///          .                       PHASE,       SOLAR,  EMISSN  )
///
///                    WRITE (*,FM1) '      Emission angle (deg): ',
///          .                     EMISSN * DPR()
///
///                    IF ( I .EQ. 2 ) THEN
///
///                       CALL VEQU ( POINTS(1,M), SVPNTS(1,M) )
///
///                    ELSE IF ( I .EQ. 3  ) THEN
///
///                       DIST = VDIST( POINTS(1,M), SVPNTS(1,M) )
///
///                       WRITE (*,FM1)
///          .            '      Distance error  (km): ', DIST
///                    END IF
///
///
///                 END DO
///
///                 START = START + NPTS(J)
///
///              END DO
///
///              WRITE (*,*) ' '
///
///           END DO
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Observer:       EARTH
///      Target:         MARS
///      Frame:          IAU_MARS
///
///      Number of cuts:            3
///
///      Computation method = TANGENT/ELLIPSOID
///      Locus              = ELLIPSOID LIMB
///
///        Roll angle (deg) =          0.000000000
///          Target epoch   =  271683700.368869901
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):        -19.302258950
///           Latitude       (deg):         64.005620446
///           Altitude        (km):         -0.000000000
///           Emission angle (deg):         90.000000000
///
///        Roll angle (deg) =        120.000000000
///          Target epoch   =  271683700.368948162
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):         85.029135674
///           Latitude       (deg):        -26.912378799
///           Altitude        (km):          0.000000000
///           Emission angle (deg):         90.000000000
///
///        Roll angle (deg) =        240.000000000
///          Target epoch   =  271683700.368949771
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):       -123.633654215
///           Latitude       (deg):        -26.912378799
///           Altitude        (km):         -0.000000000
///           Emission angle (deg):         90.000000000
///
///
///      Computation method = TANGENT/DSK/UNPRIORITIZED
///      Locus              = ELLIPSOID LIMB
///
///        Roll angle (deg) =          0.000000000
///          Target epoch   =  271683700.368869901
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):        -19.302258950
///           Latitude       (deg):         63.893637269
///           Altitude        (km):         -3.667553936
///           Emission angle (deg):         90.112271887
///
///        Roll angle (deg) =        120.000000000
///          Target epoch   =  271683700.368948162
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):         85.434644188
///           Latitude       (deg):        -26.705411228
///           Altitude        (km):         -0.044832392
///           Emission angle (deg):         89.583080105
///
///        Roll angle (deg) =        240.000000000
///          Target epoch   =  271683700.368949771
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):       -123.375003954
///           Latitude       (deg):        -27.043096556
///           Altitude        (km):          3.695628339
///           Emission angle (deg):         90.265135303
///
///
///      Computation method = GUIDED/DSK/UNPRIORITIZED
///      Locus              = CENTER
///
///        Roll angle (deg) =          0.000000000
///          Target epoch   =  271683700.368922532
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):        -19.302259163
///           Latitude       (deg):         64.005910146
///           Altitude        (km):         -3.676424552
///           Emission angle (deg):         89.999998824
///           Distance error  (km):          6.664218206
///
///        Roll angle (deg) =        120.000000000
///          Target epoch   =  271683700.368922532
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):         85.029135793
///           Latitude       (deg):        -26.912405352
///           Altitude        (km):         -0.328988915
///           Emission angle (deg):         89.999999843
///           Distance error  (km):         24.686473322
///
///        Roll angle (deg) =        240.000000000
///          Target epoch   =  271683700.368922532
///          Number of limb points at this roll angle:            1
///          Limb point planetodetic coordinates:
///           Longitude      (deg):       -123.633653487
///           Latitude       (deg):        -26.912086524
///           Altitude        (km):          3.626058850
///           Emission angle (deg):         90.000001307
///           Distance error  (km):         15.716034625
///
///
///  3) Find apparent limb points on comet Churyumov-Gerasimenko
///     as seen from the Rosetta orbiter.
///
///     This computation is an example of a case for which some
///     of the cutting half-planes contain multiple limb points.
///
///     Use the TANGENT limb definition, since the target shape
///     is not well approximated by its reference ellipsoid.
///     Use the CENTER aberration correction locus since the
///     light time difference across the object is small.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: limbpt_ex3.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        paths of the kernels referenced here must be adjusted to
///        be compatible with the user's host computer directory
///        structure.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                         Contents
///           ---------                         --------
///           DE405.BSP                         Planetary ephemeris
///           NAIF0011.TLS                      Leapseconds
///           ROS_CG_M004_NSPCESA_N_V1.BDS      DSK plate model based
///                                             on Rosetta NAVCAM data
///           RORB_DV_145_01_______00216.BSP    Rosetta orbiter
///                                             ephemeris
///           CORB_DV_145_01_______00216.BSP    Comet Churyumov-
///                                             Gerasimenko ephemeris
///           ROS_CG_RAD_V10.TPC                Comet Churyumov-
///                                             Gerasimenko radii
///           ROS_V25.TF                        Comet C-G frame kernel
///                                             (includes SCLK
///                                             parameters)
///           CATT_DV_145_01_______00216.BC     Comet C-G C-kernel
///
///
///             \begindata
///
///          KERNELS_TO_LOAD = ( 'DE405.BSP'
///                              'NAIF0011.TLS',
///                              'RORB_DV_145_01_______00216.BSP',
///                              'CORB_DV_145_01_______00216.BSP',
///                              'ROS_CG_RAD_V10.TPC',
///                              'ROS_V25.TF',
///                              'CATT_DV_145_01_______00216.BC',
///                              'ROS_CG_M004_NSPCESA_N_V1.BDS'   )
///             \begintext
///
///             End of meta-kernel
///
///
///     Example code begins here.
///
///
///     C
///     C     LIMBPT example 3
///     C
///     C        Find limb points on comet Churyumov-Gerasimenko
///     C        as seen from the Rosetta orbiter.
///     C
///     C        Compute limb points using the tangent definition.
///     C        Perform aberration corrections for the target center.
///     C        Use both ellipsoid and DSK shape models.
///     C
///     C        Display only limb points lying in half-planes that
///     C        contain multiple limb points.
///     C
///           PROGRAM LIMBPT_EX3
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      PI
///           DOUBLE PRECISION      RPD
///           DOUBLE PRECISION      VNORM
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'limbpt_ex3.tm' )
///
///           CHARACTER*(*)         FM1
///           PARAMETER           ( FM1     =  '(A,F20.9)' )
///
///           CHARACTER*(*)         FM2
///           PARAMETER           ( FM2     =  '(1X,3F20.9)' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               CORLEN
///           PARAMETER           ( CORLEN = 20 )
///
///           INTEGER               MTHLEN
///           PARAMETER           ( MTHLEN = 50 )
///
///           INTEGER               MAXN
///           PARAMETER           ( MAXN = 1000 )
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(CORLEN)    CORLOC
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(MTHLEN)    METHOD
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(BDNMLN)    TARGET
///
///           DOUBLE PRECISION      ANGLE
///           DOUBLE PRECISION      AXIS   ( 3 )
///           DOUBLE PRECISION      DELROL
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POINTS ( 3, MAXN )
///           DOUBLE PRECISION      REFVEC ( 3 )
///           DOUBLE PRECISION      ROLL
///           DOUBLE PRECISION      SCHSTP
///           DOUBLE PRECISION      SOLTOL
///           DOUBLE PRECISION      TANGTS ( 3, MAXN )
///           DOUBLE PRECISION      TRGEPS ( MAXN )
///           DOUBLE PRECISION      TRGPOS ( 3 )
///           DOUBLE PRECISION      XVEC   ( 3 )
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               K
///           INTEGER               NCUTS
///           INTEGER               NPTS   ( MAXN )
///           INTEGER               START
///     C
///     C     Initial values
///     C
///           DATA                  METHOD /
///          .                        'TANGENT/DSK/UNPRIORITIZED'
///          .                             /
///           DATA                  XVEC   / 1.D0, 0.D0, 0.D0 /
///     C
///     C     Load kernel files via the meta-kernel.
///     C
///           CALL FURNSH ( META )
///     C
///     C     Set target, observer, and target body-fixed,
///     C     body-centered reference frame.
///     C
///           OBSRVR = 'ROSETTA'
///           TARGET = 'CHURYUMOV-GERASIMENKO'
///           FIXREF = '67P/C-G_CK'
///     C
///     C     Set aberration correction and correction locus.
///     C
///           ABCORR = 'CN+S'
///           CORLOC = 'CENTER'
///     C
///     C     Convert the UTC request time string seconds past
///     C     J2000, TDB.
///     C
///           CALL STR2ET ( '2015 MAY 10 00:00:00', ET )
///     C
///     C     Compute a set of limb points using light time and
///     C     stellar aberration corrections. Use a step size
///     C     corresponding to a 10 meter height error to ensure
///     C     we don't miss the limb. Set the convergence tolerance
///     C     to 1/100 of this amount, which will limit the height
///     C     convergence error to about 10 cm.
///     C
///           CALL SPKPOS ( TARGET, ET,     FIXREF, ABCORR,
///          .              OBSRVR, TRGPOS, LT             )
///
///
///           SCHSTP = 1.D-2  / VNORM(TRGPOS)
///           SOLTOL = SCHSTP / 100.D0
///
///     C
///     C     Set the reference vector to the start of a
///     C     region of the roll domain on which we know
///     C     (from an external computation) that we'll
///     C     find multiple limb points in some half planes.
///     C     Compute 6 limb points, starting with the
///     C     half-plane containing the reference vector.
///     C
///           CALL VMINUS ( TRGPOS, AXIS )
///
///           ANGLE = 310.0D0 * RPD()
///
///           CALL VROTV  ( XVEC, AXIS, ANGLE, REFVEC )
///
///           NCUTS  = 6
///           DELROL = 2*PI() / 1000
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Observer:       '//OBSRVR
///           WRITE (*,*) 'Target:         '//TARGET
///           WRITE (*,*) 'Frame:          '//FIXREF
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Number of cuts: ', NCUTS
///           WRITE (*,*) ' '
///
///           CALL LIMBPT ( METHOD, TARGET, ET,     FIXREF,
///          .              ABCORR, CORLOC, OBSRVR, REFVEC,
///          .              DELROL, NCUTS,  SCHSTP, SOLTOL,
///          .              MAXN,   NPTS,   POINTS, TRGEPS,
///          .              TANGTS                          )
///     C
///     C     Write the results.
///     C
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Computation method = ', METHOD
///           WRITE(*,*) 'Locus              = ', CORLOC
///           WRITE(*,*) ' '
///
///           START  = 0
///
///           DO I = 1, NCUTS
///
///              ROLL = (I-1) * DELROL
///
///              IF ( NPTS(I) .GT. 1 ) THEN
///
///                 WRITE(*,*)   ' '
///                 WRITE(*,FM1) '  Roll angle (deg) = ', ROLL * DPR()
///                 WRITE(*,FM1) '     Target epoch  = ', TRGEPS(I)
///                 WRITE(*,*)   '    Number of limb points at this '
///          .      //           'roll angle: ',
///          .                   NPTS(I)
///
///                 WRITE (*,*) '      Limb points'
///
///                 DO J = 1, NPTS(I)
///                    WRITE (*,FM2) ( POINTS(K,J+START), K = 1, 3 )
///                 END DO
///
///              END IF
///
///              START = START + NPTS(I)
///
///           END DO
///           WRITE (*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Observer:       ROSETTA
///      Target:         CHURYUMOV-GERASIMENKO
///      Frame:          67P/C-G_CK
///
///      Number of cuts:            6
///
///
///      Computation method = TANGENT/DSK/UNPRIORITIZED
///      Locus              = CENTER
///
///
///       Roll angle (deg) =          0.000000000
///          Target epoch  =  484488067.184933782
///          Number of limb points at this roll angle:            3
///            Limb points
///               1.320416231        -0.347379011         1.445260615
///               0.970350318         0.201685071         0.961996205
///               0.436720618         0.048224590         0.442280714
///
///       Roll angle (deg) =          0.360000000
///          Target epoch  =  484488067.184933782
///          Number of limb points at this roll angle:            3
///            Limb points
///               1.330290293        -0.352340416         1.438802587
///               0.965481808         0.202131806         0.946190003
///               0.453917030         0.082062880         0.447624224
///
///       Roll angle (deg) =          0.720000000
///          Target epoch  =  484488067.184933782
///          Number of limb points at this roll angle:            3
///            Limb points
///               1.339037339        -0.357848188         1.431256926
///               0.962159098         0.192370269         0.934342086
///               0.459160821         0.082273840         0.447880429
///
///       Roll angle (deg) =          1.080000000
///          Target epoch  =  484488067.184933782
///          Number of limb points at this roll angle:            3
///            Limb points
///               1.346729151        -0.365488231         1.423051540
///               0.960760394         0.183652804         0.924323093
///               0.464582286         0.084076587         0.447930141
///
///       Roll angle (deg) =          1.440000000
///          Target epoch  =  484488067.184933782
///          Number of limb points at this roll angle:            3
///            Limb points
///               1.351235771        -0.380664224         1.413164272
///               0.960268777         0.176953543         0.914876859
///               0.466284590         0.079312729         0.445564308
///
///       Roll angle (deg) =          1.800000000
///          Target epoch  =  484488067.184933782
///          Number of limb points at this roll angle:            3
///            Limb points
///               1.358042184        -0.390349186         1.404421386
///               0.959495690         0.170340551         0.905212642
///               0.370611049        -0.167047205         0.395076979
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The light time approximations made by this routine may be
///      unsuitable for some observation geometries. For example, when
///      computing the limb of Mars as seen from the Earth, the
///      tangent vectors returned by this routine may be in error by
///      several km due to the light time error.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.0, 01-NOV-2021 (NJB) (JDR)
///
///         Added support for transmission aberration corrections.
///
///         Bug fix: deleted a computation of TMPVEC that had no effect.
///
///         Bug fix: PRVCOR is no longer set to blank before
///         ABCORR is parsed.
///
///         Bug fix: corrected long error message for an unsupported
///         limb type used with the ELLIPSOID LIMB locus.
///
///         Corrected description of iteration count for non-converged
///         corrections.
///
///         Edited the header to comply with NAIF standard. Reduced
///         the number of cuts to present in the output in Example #3.
///         Modified output format in all examples to comply with the
///         maximum line length of header comments.
///
/// -    SPICELIB Version 1.0.0, 08-MAR-2017 (NJB)
///
///         Based on original version 14-NOV-2015 (NJB)
/// ```
pub fn limbpt(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    abcorr: &str,
    corloc: &str,
    obsrvr: &str,
    refvec: &[f64; 3],
    rolstp: f64,
    ncuts: i32,
    schstp: f64,
    soltol: f64,
    maxn: i32,
    npts: &mut [i32],
    points: &mut [[f64; 3]],
    epochs: &mut [f64],
    tangts: &mut [[f64; 3]],
) -> crate::Result<()> {
    LIMBPT(
        method.as_bytes(),
        target.as_bytes(),
        et,
        fixref.as_bytes(),
        abcorr.as_bytes(),
        corloc.as_bytes(),
        obsrvr.as_bytes(),
        refvec,
        rolstp,
        ncuts,
        schstp,
        soltol,
        maxn,
        npts,
        points.as_flattened_mut(),
        epochs,
        tangts.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LIMBPT ( Limb points on an extended object )
pub fn LIMBPT(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    FIXREF: &[u8],
    ABCORR: &[u8],
    CORLOC: &[u8],
    OBSRVR: &[u8],
    REFVEC: &[f64],
    ROLSTP: f64,
    NCUTS: i32,
    SCHSTP: f64,
    SOLTOL: f64,
    MAXN: i32,
    NPTS: &mut [i32],
    POINTS: &mut [f64],
    EPOCHS: &mut [f64],
    TANGTS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let REFVEC = DummyArray::new(REFVEC, 1..=3);
    let mut NPTS = DummyArrayMut::new(NPTS, 1..);
    let mut POINTS = DummyArrayMut2D::new(POINTS, 1..=3, 1..);
    let mut EPOCHS = DummyArrayMut::new(EPOCHS, 1..);
    let mut TANGTS = DummyArrayMut2D::new(TANGTS, 1..=3, 1..);
    let mut LMBSTR = [b' '; CVTLEN as usize];
    let mut NRMLOC = [b' '; ACLLEN as usize];
    let mut SHPSTR = [b' '; SHPLEN as usize];
    let mut TRMSTR = [b' '; TMTLEN as usize];
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut CORTRG = StackArray::<f64, 3>::new(1..=3);
    let mut CP = StackArray::<f64, 3>::new(1..=3);
    let mut CUTNML = StackArray::<f64, 3>::new(1..=3);
    let mut EDIR = StackArray::<f64, 3>::new(1..=3);
    let mut ENORML = StackArray::<f64, 3>::new(1..=3);
    let mut EPOCH: f64 = 0.0;
    let mut EPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut IPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut ISRFVC = StackArray::<f64, 3>::new(1..=3);
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut LT: f64 = 0.0;
    let mut LTERR: f64 = 0.0;
    let mut MAXRAD: f64 = 0.0;
    let mut PLNVEC = StackArray::<f64, 3>::new(1..=3);
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut PRVLT: f64 = 0.0;
    let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
    let mut RAYVTX = StackArray::<f64, 3>::new(1..=3);
    let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
    let mut ROLL: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut PTARG = StackArray::<f64, 3>::new(1..=3);
    let mut STLOFF = StackArray::<f64, 3>::new(1..=3);
    let mut STLPOS = StackArray::<f64, 3>::new(1..=3);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut SSBLT: f64 = 0.0;
    let mut SSBTRG = StackArray::<f64, 3>::new(1..=3);
    let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
    let mut TRGEPC: f64 = 0.0;
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut FXCENT: i32 = 0;
    let mut FXCLSS: i32 = 0;
    let mut FXFCDE: i32 = 0;
    let mut FXTYID: i32 = 0;
    let mut J: i32 = 0;
    let mut NUMITR: i32 = 0;
    let mut OBSCDE: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut TO: i32 = 0;
    let mut TOTAL: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut ATTBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut SURFUP: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Convergence limit:
    //
    //
    // This limit was chosen to achieve convergence on
    // platforms providing extended precision arithmetic.
    //

    //
    // Maximum number of light time iterations for any
    // aberration correction:
    //

    //
    // Saved body name length.
    //

    //
    // Saved frame name length.
    //

    //
    // SPICELIB ellipse upper bound:
    //

    //
    // Local variables
    //

    //
    // Saved name/ID item declarations.
    //

    //
    // Saved frame name/ID item declarations.
    //

    //
    // Saved surface name/ID item declarations.
    //

    //
    // Saved target radii declarations.
    //

    //
    // Saved variables
    //

    //
    // Saved name/ID items.
    //

    //
    // Saved frame name/ID items.
    //

    //
    // Saved surface name/ID items.
    //

    //
    // Saved reference ellipsoid items.
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(RNAME, ctx)?;

    //
    // Counter initialization is done separately.
    //
    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR3.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR4.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR5.as_slice_mut(), ctx);
    }

    if (save.FIRST || fstr::ne(ABCORR, &save.PRVCOR)) {
        //
        // The aberration correction flag differs from the value it
        // had on the previous call, if any. Analyze the new flag.
        //
        ZZVALCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Set logical flags indicating the attributes of the requested
        // correction:
        //
        //    USELT is .TRUE. when any type of light time correction
        //    (normal or converged Newtonian) is specified.
        //
        //    USECN indicates converged Newtonian light time correction.
        //
        //    USESTL indicates stellar aberration corrections.
        //
        //
        // The above definitions are consistent with those used by
        // ZZVALCOR.
        //
        save.USELT = ATTBLK[LTIDX];
        save.USECN = ATTBLK[CNVIDX];
        save.USESTL = ATTBLK[STLIDX];
        save.XMIT = ATTBLK[XMTIDX];
        //
        // The aberration correction flag is valid; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);
    }

    //
    // Set the sign S prefixing LT in expression for light time-
    // corrected epochs associated with limb points.
    //
    if save.USELT {
        if save.XMIT {
            S = 1.0;
        } else {
            S = -1.0;
        }
    } else {
        S = 0.0;
    }

    //
    // Obtain integer codes for the target and observer.
    //
    ZZBODS2C(
        save.SVCTR1.as_slice_mut(),
        &mut save.SVTARG,
        &mut save.SVTCDE,
        &mut save.SVFND1,
        TARGET,
        &mut TRGCDE,
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    ZZBODS2C(
        save.SVCTR2.as_slice_mut(),
        &mut save.SVOBSR,
        &mut save.SVOBSC,
        &mut save.SVFND2,
        OBSRVR,
        &mut OBSCDE,
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The observer, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check the input body codes. If they are equal, signal
    // an error.
    //
    if (OBSCDE == TRGCDE) {
        SETMSG(b"In computing the surface intercept point, the observing body and target body are the same. Both are #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Determine the attributes of the frame designated by FIXREF.
    //
    ZZNAMFRM(
        save.SVCTR3.as_slice_mut(),
        &mut save.SVFREF,
        &mut save.SVFXFC,
        FIXREF,
        &mut FXFCDE,
        ctx,
    )?;

    FRINFO(FXFCDE, &mut FXCENT, &mut FXCLSS, &mut FXTYID, &mut FND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if !FND {
        SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        SIGERR(b"SPICE(NOFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Make sure that FIXREF is centered at the target body's center.
    //
    if (FXCENT != TRGCDE) {
        SETMSG(b"Reference frame # is not centered at the target body #. The ID code of the frame center is #.", ctx);
        ERRCH(b"#", FIXREF, ctx);
        ERRCH(b"#", TARGET, ctx);
        ERRINT(b"#", FXCENT, ctx);
        SIGERR(b"SPICE(INVALIDFRAME)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check whether the surface name/ID mapping has been updated.
    //
    ZZSRFTRK(save.SVCTR4.as_slice_mut(), &mut SURFUP, ctx)?;

    //
    // Initialize the SINCPT utility package for the next computation.
    // The choice of initialization routine depends on the target
    // surface type.
    //
    if ((save.FIRST || SURFUP) || fstr::ne(METHOD, &save.PRVMTH)) {
        //
        // Set the previous method string to an invalid value, so it
        // cannot match any future, valid input. This will force this
        // routine to parse the input method on the next call if any
        // failure occurs in this branch. Once success is assured, we can
        // record the current method in the previous method string.
        //
        fstr::assign(&mut save.PRVMTH, b" ");

        //
        // Parse the method string. If the string is valid, the
        // outputs SHAPE and SUBTYP will always be be set. However,
        // SUBTYP is not used in this routine.
        //
        // For DSK shapes, the surface list array and count will be set
        // if the method string contains a surface list.
        //
        ZZPRSMET(
            TRGCDE,
            METHOD,
            MAXSRF,
            &mut SHPSTR,
            &mut save.SUBTYP,
            &mut save.PRI,
            &mut save.NSURF,
            save.SRFLST.as_slice_mut(),
            &mut LMBSTR,
            &mut TRMSTR,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if EQSTR(&SHPSTR, b"ELLIPSOID") {
            save.SHAPE = ELLSHP;
        } else if EQSTR(&SHPSTR, b"DSK") {
            save.SHAPE = DSKSHP;
        } else {
            //
            // This is a backstop check.
            //
            SETMSG(b"[1] Returned shape value from method string was <#>.", ctx);
            ERRCH(b"#", &SHPSTR, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if EQSTR(&LMBSTR, b"TANGENT") {
            save.LMBTYP = TANGNT;
        } else if EQSTR(&LMBSTR, b"GUIDED") {
            save.LMBTYP = GUIDED;
        } else {
            SETMSG(
                b"Returned limb type from method string was <#>. Value must be TANGENT or GUIDED.",
                ctx,
            );
            ERRCH(b"#", &LMBSTR, ctx);
            SIGERR(b"SPICE(INVALIDLIMBTYPE)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // There should be no subtype specification in the method
        // string.
        //
        if fstr::ne(&save.SUBTYP, b" ") {
            SETMSG(b"Spurious sub-observer point type <#> was present in the method string #. The sub-observer type is valid in the method strings for SUBPNT and SUBSLR, but is not applicable for LIMBPT.", ctx);
            ERRCH(b"#", &save.SUBTYP, ctx);
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // There should be no terminator specification in the method
        // string.
        //
        if fstr::ne(&TRMSTR, b" ") {
            SETMSG(b"Spurious terminator shadow type <#> was present in the method string #. The terminator shadow type is valid in the method string for TERMPT, but is not applicable for LIMBPT.", ctx);
            ERRCH(b"#", &TRMSTR, ctx);
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        fstr::assign(&mut save.PRVMTH, METHOD);
    }

    //
    // Identify the aberration correction locus.
    //
    if (save.FIRST || fstr::ne(CORLOC, &save.PRVLOC)) {
        LJUCRS(1, CORLOC, &mut NRMLOC, ctx);

        if fstr::eq(&NRMLOC, b"CENTER") {
            save.LOCCDE = CTRCOR;
        } else if fstr::eq(&NRMLOC, b"ELLIPSOID LIMB") {
            save.LOCCDE = ELLCOR;
        } else {
            SETMSG(b"Aberration correction locus <#> was not recognized.", ctx);
            ERRCH(b"#", CORLOC, ctx);
            SIGERR(b"SPICE(INVALIDLOCUS)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
        //
        // At this point we have a valid locus. LOCCDE is set.
        // Save the input locus string so we can check for
        // a change on the next call.
        //
        fstr::assign(&mut save.PRVLOC, CORLOC);
    }

    //
    // Check the reference vector.
    //
    if VZERO(REFVEC.as_slice()) {
        SETMSG(b"The reference vector was the zero vector.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // At this point, the first pass actions were successful.
    //
    save.FIRST = false;

    //
    // Check MAXN.
    //
    if (MAXN < 1) {
        SETMSG(b"MAXN = #; MAXN is required to be at least 1.", ctx);
        ERRINT(b"#", MAXN, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check NCUTS; there must be room for at least one limb point
    // for each cut. NCUTS may not be negative.
    //
    if ((NCUTS < 1) || (NCUTS > MAXN)) {
        SETMSG(
            b"NCUTS = #; MAXN = #; NCUTS is required to be non-negative and no larger than MAXN.",
            ctx,
        );
        ERRINT(b"#", NCUTS, ctx);
        ERRINT(b"#", MAXN, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check the angular search step size and convergence
    // tolerance. These checks apply only to DSK shapes.
    //
    if (save.SHAPE == DSKSHP) {
        if (SCHSTP <= 0.0) {
            SETMSG(
                b"The angular search step SCHSTP = #; SCHSTP is required to be positive.",
                ctx,
            );
            ERRDP(b"#", SCHSTP, ctx);
            SIGERR(b"SPICE(INVALIDSEARCHSTEP)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if (SOLTOL <= 0.0) {
            SETMSG(
                b"The angular search tolerance SOLTOL = #; SOLTOL is required to be positive.",
                ctx,
            );
            ERRDP(b"#", SCHSTP, ctx);
            SIGERR(b"SPICE(INVALIDTOLERANCE)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    //
    // Check the roll step. This value applies only if
    // there are multiple cutting half-planes.
    //
    if ((NCUTS > 1) && (ROLSTP == 0.0)) {
        SETMSG(b"The angular roll step is 0.D0. NCUTS = #. ROLSTP is required to be non-zero when NCUTS is greater than 1.", ctx);
        ERRINT(b"#", NCUTS, ctx);
        SIGERR(b"SPICE(INVALIDROLLSTEP)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if (save.SHAPE == DSKSHP) {
        //
        // This is the DSK case.
        //
        // Initialize the intercept algorithm to use a DSK
        // model for the surface of the target body.
        //
        ZZSUDSKI(TRGCDE, save.NSURF, save.SRFLST.as_slice(), FXFCDE, ctx)?;
    } else if (save.SHAPE != ELLSHP) {
        SETMSG(b"Computation method argument was <#>; this string must specify a supported shape model and computation type. See the description of METHOD in the header of SUBPNT for details.", ctx);
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check MAXN.
    //
    if (MAXN < 1) {
        SETMSG(b"MAXN = #; MAXN is required to be at least 1.", ctx);
        ERRINT(b"#", MAXN, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check NCUTS; there must be room for at least one limb point
    // for each cut. NCUTS may not be negative.
    //
    if ((NCUTS < 1) || (NCUTS > MAXN)) {
        SETMSG(
            b"NCUTS = #; MAXN = #; NCUTS is required to be non-negative and no larger than MAXN.",
            ctx,
        );
        ERRINT(b"#", NCUTS, ctx);
        ERRINT(b"#", MAXN, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Get target body radii if necessary.
    //
    if (((save.SHAPE == ELLSHP) || (save.LOCCDE == ELLCOR)) || (save.LMBTYP == GUIDED)) {
        if (TRGCDE != save.PRVTRG) {
            //
            // Reset counter to force lookup.
            //
            ZZCTRUIN(save.SVCTR5.as_slice_mut(), ctx);
        }
        //
        // Look up target radii using counter.
        //
        ZZBODVCD(
            TRGCDE,
            b"RADII",
            3,
            save.SVCTR5.as_slice_mut(),
            &mut save.SVNRAD,
            save.SVRADI.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if (save.SVNRAD != 3) {
            SETMSG(b"Number of target radii must be 3 but was #.", ctx);
            ERRINT(b"#", save.SVNRAD, ctx);
            SIGERR(b"SPICE(BADRADIUSCOUNT)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        save.PRVTRG = TRGCDE;
    }

    //
    // Set up activities are complete at this point.
    //
    // Find limb points on the target.
    //
    CLEARI(NCUTS, NPTS.as_slice_mut());
    SSIZED(MAXWIN, RESULT.as_slice_mut(), ctx)?;

    //
    // Get initial observer-target vector, expressed in the target
    // body-fixed frame, evaluated at the target epoch. This vector
    // will be used for all option combinations.
    //
    SPKPOS(
        TARGET,
        ET,
        FIXREF,
        ABCORR,
        OBSRVR,
        POS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if VZERO(POS.as_slice()) {
        SETMSG(
            b"The distance between the observer and target at ET # is zero.",
            ctx,
        );
        ERRDP(b"#", ET, ctx);
        SIGERR(b"SPICE(NOSEPARATION)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // The limb-finding technique depends on the aberration correction
    // locus. Start with the 'CENTER' version, since this is the
    // simpler case.
    //

    if (save.LOCCDE == CTRCOR) {
        //
        // Aberration corrections are those applicable at the target
        // center.
        //
        // Compute the epoch associated with the target center.
        //
        ZZCOREPC(ABCORR, ET, LT, &mut TRGEPC, ctx)?;

        //
        // Compute the central axis, which is also the common ray vertex.
        // The axis points from the target to the observer.
        //
        VMINUS(POS.as_slice(), AXIS.as_slice_mut());

        //
        // Make sure the reference vector and axis are linearly
        // independent.
        //
        VCRSS(AXIS.as_slice(), REFVEC.as_slice(), CP.as_slice_mut());

        if VZERO(CP.as_slice()) {
            SETMSG(
                b"Input reference vector and observer-target vector are linearly dependent.",
                ctx,
            );
            SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If we're using an ellipsoidal shape model, or if
        // we're using the "guided" limb option, find the
        // limb parameters of the reference ellipsoid.
        //
        if ((save.SHAPE == ELLSHP) || (save.LMBTYP == GUIDED)) {
            EDLIMB(
                save.SVRADI[1],
                save.SVRADI[2],
                save.SVRADI[3],
                AXIS.as_slice(),
                LIMB.as_slice_mut(),
                ctx,
            )?;

            EL2CGV(
                LIMB.as_slice(),
                CENTER.as_slice_mut(),
                SMAJOR.as_slice_mut(),
                SMINOR.as_slice_mut(),
            );

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            UCRSS(SMAJOR.as_slice(), SMINOR.as_slice(), ENORML.as_slice_mut());

            //
            // Make sure ENORML points into the same half-space as
            // AXIS.
            //
            if (VDOT(ENORML.as_slice(), AXIS.as_slice()) < 0.0) {
                VSCLIP(-1.0, ENORML.as_slice_mut());
            }

            if (save.SHAPE == DSKSHP) {
                //
                // Caution: this requires that ZZSUDSKI has been
                // called first.
                //
                ZZMAXRAD(&mut MAXRAD, ctx);
            }
        }

        TO = 1;
        ROOM = MAXN;
        TOTAL = 0;

        //
        // Loop over the half planes, collecting limb points for
        // each one.
        //
        for I in 1..=NCUTS {
            ROLL = (((I - 1) as f64) * ROLSTP);
            //
            // Rotation of the half-planes is in the positive
            // sense about AXIS.
            //
            VROTV(
                REFVEC.as_slice(),
                AXIS.as_slice(),
                ROLL,
                PLNVEC.as_slice_mut(),
            );

            //
            // Let CUTNML be a vector normal to the current cutting
            // half-plane. We'll use this vector later.
            //
            UCRSS(AXIS.as_slice(), PLNVEC.as_slice(), CUTNML.as_slice_mut());

            if (save.SHAPE == DSKSHP) {
                //
                // This is the DSK case.
                //
                if (save.LMBTYP == TANGNT) {
                    //
                    // This type of solution finds actual tangent rays on
                    // the target.
                    //
                    // Find the limb points that lie in the current
                    // half-plane.
                    //
                    // Note that RESULT is a cell, not a window.
                    //
                    SCARDD(0, RESULT.as_slice_mut(), ctx)?;
                    //
                    // Note that the evaluation epoch for the surface is
                    // optionally corrected for light time.
                    //
                    ZZTANGNT(
                        LMBCRV,
                        0.0,
                        save.SHAPE,
                        TRGCDE,
                        save.NSURF,
                        save.SRFLST.as_slice(),
                        FXFCDE,
                        TRGEPC,
                        PLNVEC.as_slice(),
                        AXIS.as_slice(),
                        SCHSTP,
                        SOLTOL,
                        RESULT.as_slice_mut(),
                        save.PNTBUF.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    NPTS[I] = CARDD(RESULT.as_slice(), ctx)?;
                } else if (save.LMBTYP == GUIDED) {
                    //
                    // This option uses the target's reference ellipsoid for
                    // guidance. For DSK shapes, the limb points are
                    // generated by finding surface intercepts of rays
                    // emanating from the center of the limb on the
                    // reference ellipsoid.
                    //
                    // The limb point we seek must lie in both the limb
                    // plane and the cutting half-plane. Let EDIR the
                    // unit direction vector satisfying these constraints.
                    //
                    UCRSS(CUTNML.as_slice(), ENORML.as_slice(), EDIR.as_slice_mut());

                    if VZERO(EDIR.as_slice()) {
                        SETMSG(b"Vector defining cutting half plane and ellipsoid limb normal vector are linearly dependent.", ctx);
                        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // Find the intercept on the target surface of the ray
                    // emanating from CENTER in the direction EDIR. We must
                    // use a ray pointed in the opposite direction to
                    // perform this computation, since the surface may be
                    // invisible from the interior of the target.
                    //
                    VLCOM(
                        1.0,
                        CENTER.as_slice(),
                        (3.0 * MAXRAD),
                        EDIR.as_slice(),
                        RAYVTX.as_slice_mut(),
                    );
                    VMINUS(EDIR.as_slice(), RAYDIR.as_slice_mut());

                    ZZRAYSFX(
                        RAYVTX.as_slice(),
                        RAYDIR.as_slice(),
                        TRGEPC,
                        save.PNTBUF.subarray_mut([1, 1]),
                        &mut FND,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    if FND {
                        NPTS[I] = 1;
                    } else {
                        NPTS[I] = 0;
                    }
                } else {
                    //
                    // This is a backstop case; it should never be reached.
                    //
                    SETMSG(b"Invalid limb type code: #", ctx);
                    ERRINT(b"#", save.LMBTYP, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            } else if (save.SHAPE == ELLSHP) {
                //
                // This is the ellipsoid case.
                //
                // The limb point we seek must lie in both the limb plane
                // and the cutting half-plane. Let EDIR be the unit
                // direction vector satisfying these constraints.
                //
                UCRSS(CUTNML.as_slice(), ENORML.as_slice(), EDIR.as_slice_mut());

                if VZERO(EDIR.as_slice()) {
                    SETMSG(b"Vector defining cutting half plane and ellipsoid limb normal vector are linearly dependent.", ctx);
                    SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Find the intercept on the target surface of the
                // the ray emanating from CENTER in the direction EDIR.
                //
                SURFPT(
                    CENTER.as_slice(),
                    EDIR.as_slice(),
                    save.SVRADI[1],
                    save.SVRADI[2],
                    save.SVRADI[3],
                    save.PNTBUF.subarray_mut([1, 1]),
                    &mut FND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                if !FND {
                    SETMSG(b"Limb point not found on reference ellipsoid for cutting half plane at index #. The point should always be found.", ctx);
                    ERRINT(b"#", I, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                NPTS[I] = 1;
            } else {
                //
                // This is a backstop case; it should never be reached.
                //
                SETMSG(b"Invalid shape code: #", ctx);
                ERRINT(b"#", save.SHAPE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            TOTAL = (TOTAL + NPTS[I]);

            if (NPTS[I] > ROOM) {
                SETMSG(b"Out of room in output arrays. Index of cutting half-plane is # out of #. Number of limb points collected so far is #. Available room is #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", NCUTS, ctx);
                ERRINT(b"#", TOTAL, ctx);
                ERRINT(b"#", ROOM, ctx);
                SIGERR(b"SPICE(OUTOFROOM)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Transfer the limb points we found to the output limb point
            // array. Set the elements of the surface vector array as we
            // go. Store in each element of the output array the epoch
            // associated with the target center.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = NPTS[I];
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    VEQU(save.PNTBUF.subarray([1, J]), POINTS.subarray_mut([1, TO]));
                    VSUB(
                        save.PNTBUF.subarray([1, J]),
                        AXIS.as_slice(),
                        TANGTS.subarray_mut([1, TO]),
                    );

                    EPOCHS[TO] = TRGEPC;

                    TO = (TO + 1);

                    J += m3__;
                }
            }
        }
    } else if (save.LOCCDE == ELLCOR) {
        //
        // Aberration corrections are done for each cutting half plane.
        // Corrections are performed for the intersections of the
        // half plane with the reference ellipsoid's limb.
        //
        // This locus is supported only for the "tangent" limb point
        // method.
        //
        if (save.LMBTYP != TANGNT) {
            SETMSG(b"Limb type <#> is not supported for the # aberration correction locus. Only the TANGENT limb type is supported for this locus.", ctx);

            if (save.LMBTYP == GUIDED) {
                ERRCH(b"#", b"GUIDED", ctx);
            } else {
                ERRINT(b"#", save.LMBTYP, ctx);
            }

            ERRCH(b"#", CORLOC, ctx);
            SIGERR(b"SPICE(BADLIMBLOCUSMIX)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // We need the state of the observer relative to the solar
        // system barycenter. This state is expressed relative to
        // an inertial reference frame. This state is computed once.
        //
        SPKSSB(OBSCDE, ET, IREF, STOBS.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        TO = 1;
        ROOM = MAXN;
        TOTAL = 0;
        //
        // Loop over the half planes, collecting limb points for
        // each one.
        //
        for I in 1..=NCUTS {
            ROLL = (((I - 1) as f64) * ROLSTP);

            if save.USELT {
                //
                // We'll do an independent light time and stellar
                // aberration correction for each half plane.
                //
                // Let NUMITR be the number of iterations we'll perform to
                // compute the light time.
                //
                if save.USECN {
                    NUMITR = MAXITR;
                } else {
                    NUMITR = 2;
                }

                J = 0;
                LTERR = 1.0;

                while ((J < NUMITR) && (LTERR > CNVLIM)) {
                    //
                    // LT was set either prior to this loop or
                    // during the previous loop iteration.
                    //
                    EPOCH = TOUCHD((ET + (S * LT)));

                    SPKGPS(
                        TRGCDE,
                        EPOCH,
                        IREF,
                        SSB,
                        SSBTRG.as_slice_mut(),
                        &mut SSBLT,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }
                    //
                    // Compute the position of the target center relative to
                    // the observer in the inertial frame.
                    //
                    VSUB(SSBTRG.as_slice(), STOBS.as_slice(), PTARG.as_slice_mut());

                    if save.USESTL {
                        //
                        // Apply a stellar aberration correction to the
                        // observer-target center vector.
                        //
                        if (J == 0) {
                            //
                            // On the first pass, we approximate the
                            // correction by using the correction applicable
                            // to the target center.
                            //
                            if save.XMIT {
                                STLABX(
                                    PTARG.as_slice(),
                                    STOBS.subarray(4),
                                    STLPOS.as_slice_mut(),
                                    ctx,
                                )?;
                            } else {
                                STELAB(
                                    PTARG.as_slice(),
                                    STOBS.subarray(4),
                                    STLPOS.as_slice_mut(),
                                    ctx,
                                )?;
                            }
                        } else {
                            //
                            // We apply the correction found for the previous
                            // limb point estimate.
                            //
                            VADD(PTARG.as_slice(), STLOFF.as_slice(), STLPOS.as_slice_mut());
                        }
                        //
                        // Set CORTRG with the vector corrected for
                        // stellar aberration.
                        //
                        VEQU(STLPOS.as_slice(), CORTRG.as_slice_mut());
                    } else {
                        VEQU(PTARG.as_slice(), CORTRG.as_slice_mut());
                    }
                    //
                    // CORTRG is inertially referenced and includes the
                    // stellar aberration correction, if there is one. PTARG
                    // is inertially referenced and does not include the
                    // stellar aberration correction.
                    //
                    // Transform the aberration-corrected position vector to
                    // the target body-fixed frame; negate the result. This
                    // gives us the axis for the limb computation.
                    //
                    PXFORM(IREF, FIXREF, EPOCH, XFORM.as_slice_mut(), ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    MXV(XFORM.as_slice(), CORTRG.as_slice(), TMPVEC.as_slice_mut());
                    VMINUS(TMPVEC.as_slice(), AXIS.as_slice_mut());
                    //
                    // Rotate the reference vector about the axis by
                    // the current angle to obtain the plane vector.
                    //
                    VROTV(
                        REFVEC.as_slice(),
                        AXIS.as_slice(),
                        ROLL,
                        PLNVEC.as_slice_mut(),
                    );
                    //
                    // Find the limb, the limb center and semi-axes, and
                    // limb plane's normal vector for the current viewing
                    // geometry.
                    //
                    EDLIMB(
                        save.SVRADI[1],
                        save.SVRADI[2],
                        save.SVRADI[3],
                        AXIS.as_slice(),
                        LIMB.as_slice_mut(),
                        ctx,
                    )?;

                    EL2CGV(
                        LIMB.as_slice(),
                        CENTER.as_slice_mut(),
                        SMAJOR.as_slice_mut(),
                        SMINOR.as_slice_mut(),
                    );

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    UCRSS(SMAJOR.as_slice(), SMINOR.as_slice(), ENORML.as_slice_mut());

                    //
                    // Make sure ENORML points into the same half-space as
                    // AXIS.
                    //
                    if (VDOT(ENORML.as_slice(), AXIS.as_slice()) < 0.0) {
                        VSCLIP(-1.0, ENORML.as_slice_mut());
                    }

                    //
                    // Let CUTNML be a vector normal to the current cutting
                    // half-plane.
                    //
                    UCRSS(AXIS.as_slice(), PLNVEC.as_slice(), CUTNML.as_slice_mut());

                    // The limb point we seek must lie in both the limb
                    // plane and the cutting half-plane. Let EDIR be the
                    // unit direction vector satisfying these constraints.
                    //
                    UCRSS(CUTNML.as_slice(), ENORML.as_slice(), EDIR.as_slice_mut());

                    if VZERO(EDIR.as_slice()) {
                        SETMSG(b"Vector defining cutting half plane and ellipsoid limb normal vector are linearly dependent. This error occurred while computing the limb point on the reference ellipsoid in half plane #.", ctx);
                        ERRINT(b"#", I, ctx);
                        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // Compute the ellipsoid limb point.
                    //
                    SURFPT(
                        CENTER.as_slice(),
                        EDIR.as_slice(),
                        save.SVRADI[1],
                        save.SVRADI[2],
                        save.SVRADI[3],
                        EPOINT.as_slice_mut(),
                        &mut FND,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    if !FND {
                        SETMSG(b"Limb point not found on reference ellipsoid for cutting half plane at index #. The point should always be found.", ctx);
                        ERRINT(b"#", I, ctx);
                        SIGERR(b"SPICE(BUG)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // In order to compute the next light time and stellar
                    // aberration correction, we need the inertially
                    // referenced vector from the observer to the light-time
                    // corrected limb point.
                    //
                    MTXV(XFORM.as_slice(), EPOINT.as_slice(), IPOINT.as_slice_mut());
                    VADD(IPOINT.as_slice(), PTARG.as_slice(), ISRFVC.as_slice_mut());

                    if save.USESTL {
                        //
                        // We're correcting for stellar aberration. Another
                        // loop iteration may occur. Prepare the stellar
                        // aberration offset for the next loop iteration.
                        //
                        // Convert the observer-limb vector to the inertial
                        // frame and compute the stellar aberration
                        // correction that applies to this vector.
                        //
                        if save.XMIT {
                            STLABX(
                                ISRFVC.as_slice(),
                                STOBS.subarray(4),
                                STLPOS.as_slice_mut(),
                                ctx,
                            )?;
                        } else {
                            STELAB(
                                ISRFVC.as_slice(),
                                STOBS.subarray(4),
                                STLPOS.as_slice_mut(),
                                ctx,
                            )?;
                        }

                        VSUB(STLPOS.as_slice(), ISRFVC.as_slice(), STLOFF.as_slice_mut());
                    }
                    //
                    // Compute the light time to the limb point.
                    //
                    PRVLT = LT;
                    LT = TOUCHD((VNORM(ISRFVC.as_slice()) / CLIGHT()));

                    //
                    // LTERR is the magnitude of the change between the
                    // current estimate of light time and the previous
                    // estimate, relative to the previous light time
                    // corrected epoch.
                    //
                    LTERR = TOUCHD(
                        (f64::abs((LT - PRVLT)) / intrinsics::DMAX1(&[1.0, f64::abs(EPOCH)])),
                    );

                    J = (J + 1);
                }
                //
                // We now have the light time and the stellar aberration
                // offset applicable to the limb point on the ellipsoid for
                // the current half plane. Compute the axis for the DSK
                // limb point computation.
                //
                // Compute the axis in the body-fixed frame.
                //
                MXV(XFORM.as_slice(), CORTRG.as_slice(), TMPVEC.as_slice_mut());
                VMINUS(TMPVEC.as_slice(), AXIS.as_slice_mut());

                EPOCH = (ET + (S * LT));
            } else {
                //
                // This is the geometric case.
                //
                // We'll use the observer target position vector
                // computed above the IF block that branches based
                // on CORLOC.
                //
                // Compute the central axis, which is the common ray
                // vertex.
                //
                VMINUS(POS.as_slice(), AXIS.as_slice_mut());

                //
                // The target epoch matches the observer epoch.
                //
                EPOCH = ET;

                //
                // EPOCH and AXIS are set. Reset the plane definition
                // vector PLNVEC based on the new value of AXIS.
                //
                VROTV(
                    REFVEC.as_slice(),
                    AXIS.as_slice(),
                    ROLL,
                    PLNVEC.as_slice_mut(),
                );

                //
                // We're ready to compute the limb point in the current
                // half-plane.
                //
                //
                // Find the limb, the limb center and semi-axes, and
                // limb plane's normal vector for the current viewing
                // geometry.
                //
                EDLIMB(
                    save.SVRADI[1],
                    save.SVRADI[2],
                    save.SVRADI[3],
                    AXIS.as_slice(),
                    LIMB.as_slice_mut(),
                    ctx,
                )?;

                EL2CGV(
                    LIMB.as_slice(),
                    CENTER.as_slice_mut(),
                    SMAJOR.as_slice_mut(),
                    SMINOR.as_slice_mut(),
                );

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                UCRSS(SMAJOR.as_slice(), SMINOR.as_slice(), ENORML.as_slice_mut());
                //
                // Make sure ENORML points into the same half-space as
                // AXIS.
                //
                if (VDOT(ENORML.as_slice(), AXIS.as_slice()) < 0.0) {
                    VSCLIP(-1.0, ENORML.as_slice_mut());
                }

                //
                // Let CUTNML be a vector normal to the current cutting
                // half-plane.
                //
                UCRSS(AXIS.as_slice(), PLNVEC.as_slice(), CUTNML.as_slice_mut());
                //
                // The limb point we seek must lie in both the limb
                // plane and the cutting half-plane. Let EDIR be the
                // unit direction vector satisfying these constraints.
                //
                UCRSS(CUTNML.as_slice(), ENORML.as_slice(), EDIR.as_slice_mut());

                if VZERO(EDIR.as_slice()) {
                    SETMSG(b"Vector defining cutting half plane and ellipsoid limb normal vector are linearly dependent. This occurred while computing the limb point on the reference ellipsoid in half plane #.", ctx);
                    ERRINT(b"#", I, ctx);
                    SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
                //
                // Compute the ellipsoid limb point.
                //
                SURFPT(
                    CENTER.as_slice(),
                    EDIR.as_slice(),
                    save.SVRADI[1],
                    save.SVRADI[2],
                    save.SVRADI[3],
                    EPOINT.as_slice_mut(),
                    &mut FND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                if !FND {
                    SETMSG(b"Limb point not found on reference ellipsoid for cutting half plane at index #. The point should always be found.", ctx);
                    ERRINT(b"#", I, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            }
            //
            // Set the output point (there's exactly 1 in all cases) and
            // the point count here. These values apply to the ellipsoid
            // case. In the DSK case, we'll update the values when we
            // know them.
            //
            VEQU(EPOINT.as_slice(), save.PNTBUF.subarray_mut([1, 1]));

            NPTS[I] = 1;

            if (save.SHAPE == DSKSHP) {
                //
                // Find the limb points on the target surface as modeled
                // by DSK data. We'll use the axis and epoch we've
                // determined from the ellipsoid approximation.
                //
                SCARDD(0, RESULT.as_slice_mut(), ctx)?;
                //
                // Note that the evaluation epoch for the surface is
                // corrected for light time.
                //
                ZZTANGNT(
                    LMBCRV,
                    0.0,
                    save.SHAPE,
                    TRGCDE,
                    save.NSURF,
                    save.SRFLST.as_slice(),
                    FXFCDE,
                    EPOCH,
                    PLNVEC.as_slice(),
                    AXIS.as_slice(),
                    SCHSTP,
                    SOLTOL,
                    RESULT.as_slice_mut(),
                    save.PNTBUF.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
                //
                // Update the limb point count for this cutting
                // half-plane.
                //
                NPTS[I] = CARDD(RESULT.as_slice(), ctx)?;
            } else if (save.SHAPE != ELLSHP) {
                SETMSG(b"Backstop error: SHAPE = #.", ctx);
                ERRINT(b"#", save.SHAPE, ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            TOTAL = (TOTAL + NPTS[I]);

            if (NPTS[I] > ROOM) {
                SETMSG(b"Out of room in output arrays. Index of cutting half-plane is # out of #. Number of limb points collected so far is #. Available room is #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", NCUTS, ctx);
                ERRINT(b"#", TOTAL, ctx);
                ERRINT(b"#", ROOM, ctx);
                SIGERR(b"SPICE(OUTOFROOM)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Transfer the limb points we found to the output limb
            // point array. Set the elements of the surface vector
            // array as we go. In this case, we set the elements of
            // the output target epoch array as well.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = NPTS[I];
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    VEQU(save.PNTBUF.subarray([1, J]), POINTS.subarray_mut([1, TO]));
                    VSUB(
                        save.PNTBUF.subarray([1, J]),
                        AXIS.as_slice(),
                        TANGTS.subarray_mut([1, TO]),
                    );

                    EPOCHS[TO] = EPOCH;

                    TO = (TO + 1);

                    J += m3__;
                }
            }
            //
            // We've found the limb points and tangent vectors
            // for the Ith half-plane.
            //
        }
    } else {
        SETMSG(b"Aberration correction locus # is not recognized.", ctx);
        ERRCH(b"#", CORLOC, ctx);
        SIGERR(b"SPICE(INVALIDLOCUS)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
