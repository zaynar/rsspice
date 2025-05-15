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
const RNAME: &[u8] = b"TERMPT";
const IREF: &[u8] = b"J2000";
const CNVLIM: f64 = 0.00000000000000001;
const MAXITR: i32 = 5;
const SSB: i32 = 0;
const MAXL: i32 = 36;
const FRNMLN: i32 = 32;
const LBCELL: i32 = -5;
const MAXIVL: i32 = 1000;
const MAXWIN: i32 = (2 * MAXIVL);

struct SaveVars {
    PRVCOR: Vec<u8>,
    PRVLOC: Vec<u8>,
    PRVMTH: Vec<u8>,
    SUBTYP: Vec<u8>,
    MAXRAD: f64,
    PNTBUF: ActualArray2D<f64>,
    ILUCDE: i32,
    LOCCDE: i32,
    NRAD: i32,
    NSURF: i32,
    PRVILU: i32,
    PRVTRG: i32,
    SHADOW: i32,
    SHAPE: i32,
    SRFLST: StackArray<i32, 100>,
    SVLCOD: i32,
    TRGCDE: i32,
    TRMTYP: i32,
    FIRST: bool,
    PRI: bool,
    UFLAG: bool,
    USECN: bool,
    USELT: bool,
    USESTL: bool,
    SVCTR1: StackArray<i32, 2>,
    SVTARG: Vec<u8>,
    SVTCDE: i32,
    SVFND1: bool,
    SVCTR2: StackArray<i32, 2>,
    SVOBSR: Vec<u8>,
    SVOBSC: i32,
    SVFND2: bool,
    SVCTR3: StackArray<i32, 2>,
    SVILUM: Vec<u8>,
    SVICDE: i32,
    SVFND3: bool,
    SVCTR4: StackArray<i32, 2>,
    SVFREF: Vec<u8>,
    SVFXFC: i32,
    SVCTR5: StackArray<i32, 2>,
    SVCTR6: StackArray<i32, 2>,
    SVTRAD: StackArray<f64, 3>,
    SVCTR7: StackArray<i32, 2>,
    SVSRAD: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVCOR = vec![b' '; CORLEN as usize];
        let mut PRVLOC = vec![b' '; ACLLEN as usize];
        let mut PRVMTH = vec![b' '; MTHLEN as usize];
        let mut SUBTYP = vec![b' '; SUBLEN as usize];
        let mut MAXRAD: f64 = 0.0;
        let mut PNTBUF = ActualArray2D::<f64>::new(1..=3, 1..=MAXWIN);
        let mut ILUCDE: i32 = 0;
        let mut LOCCDE: i32 = 0;
        let mut NRAD: i32 = 0;
        let mut NSURF: i32 = 0;
        let mut PRVILU: i32 = 0;
        let mut PRVTRG: i32 = 0;
        let mut SHADOW: i32 = 0;
        let mut SHAPE: i32 = 0;
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut SVLCOD: i32 = 0;
        let mut TRGCDE: i32 = 0;
        let mut TRMTYP: i32 = 0;
        let mut FIRST: bool = false;
        let mut PRI: bool = false;
        let mut UFLAG: bool = false;
        let mut USECN: bool = false;
        let mut USELT: bool = false;
        let mut USESTL: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; MAXL as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVOBSR = vec![b' '; MAXL as usize];
        let mut SVOBSC: i32 = 0;
        let mut SVFND2: bool = false;
        let mut SVCTR3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVILUM = vec![b' '; MAXL as usize];
        let mut SVICDE: i32 = 0;
        let mut SVFND3: bool = false;
        let mut SVCTR4 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVFREF = vec![b' '; FRNMLN as usize];
        let mut SVFXFC: i32 = 0;
        let mut SVCTR5 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVCTR6 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTRAD = StackArray::<f64, 3>::new(1..=3);
        let mut SVCTR7 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVSRAD = StackArray::<f64, 3>::new(1..=3);

        FIRST = true;
        NRAD = 0;
        fstr::assign(&mut PRVCOR, b" ");
        PRVILU = 0;
        fstr::assign(&mut PRVLOC, b" ");
        fstr::assign(&mut PRVMTH, b" ");
        PRVTRG = 0;
        USECN = false;
        USELT = false;
        USESTL = false;

        Self {
            PRVCOR,
            PRVLOC,
            PRVMTH,
            SUBTYP,
            MAXRAD,
            PNTBUF,
            ILUCDE,
            LOCCDE,
            NRAD,
            NSURF,
            PRVILU,
            PRVTRG,
            SHADOW,
            SHAPE,
            SRFLST,
            SVLCOD,
            TRGCDE,
            TRMTYP,
            FIRST,
            PRI,
            UFLAG,
            USECN,
            USELT,
            USESTL,
            SVCTR1,
            SVTARG,
            SVTCDE,
            SVFND1,
            SVCTR2,
            SVOBSR,
            SVOBSC,
            SVFND2,
            SVCTR3,
            SVILUM,
            SVICDE,
            SVFND3,
            SVCTR4,
            SVFREF,
            SVFXFC,
            SVCTR5,
            SVCTR6,
            SVTRAD,
            SVCTR7,
            SVSRAD,
        }
    }
}

/// Terminator points on an extended object
///
/// Find terminator points on a target body. The caller specifies
/// half-planes, bounded by the illumination source center-target
/// center vector, in which to search for terminator points.
///
/// The terminator can be either umbral or penumbral. The umbral
/// terminator is the boundary of the region on the target surface
/// where no light from the source is visible. The penumbral
/// terminator is the boundary of the region on the target surface
/// where none of the light from the source is blocked by the target
/// itself.
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
///  ILUSRC     I   Illumination source.
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
///  NPTS       O   Counts of terminator points corresponding to cuts.
///  POINTS     O   Terminator points.
///  EPOCHS     O   Times associated with terminator points.
///  TRMVCS     O   Terminator vectors emanating from the observer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  METHOD   is a short string providing parameters defining
///           the computation method to be used. In the syntax
///           descriptions below, items delimited by angle brackets
///           "<>" are to be replaced by actual values. Items
///           delimited by brackets "[]" are optional.
///
///           METHOD may be assigned the following values:
///
///              '<shadow>/<curve type>/<shape specification>'
///
///           An example of such a string is
///
///              'UMBRAL/TANGENT/DSK/UNPRIORITIZED'
///
///           In the METHOD string
///
///              <shadow> may be either of the strings
///
///                 'UMBRAL'    indicates the terminator is the
///                             boundary of the portion of the surface
///                             that receives no light from the
///                             illumination source. The shape of the
///                             source is modeled as a sphere. See the
///                             $Particulars section below for details.
///
///                 'PENUMBRAL' indicates the terminator is the
///                             boundary of the portion of the surface
///                             that receives all possible light from
///                             the illumination source. The shape of
///                             the source is modeled as a sphere.
///
///                             The penumbral terminator bounds the
///                             portion of the surface that is not
///                             subject to self-occultation of light
///                             from the illumination source. Given
///                             that the light source is modeled as a
///                             sphere, from any target surface point
///                             nearer to the source than the
///                             penumbral terminator, the source
///                             appears to be a lit disc. See the
///                             $Particulars section below for details.
///
///
///              <curve type> may be either of the strings
///
///                 'TANGENT'   for topographic (DSK) target models
///                             indicates that a terminator point is
///                             defined as the point of tangency, on
///                             the surface represented by the
///                             specified data, of a line also tangent
///                             to the illumination source.
///
///                             For ellipsoidal target models, a
///                             terminator point is a point of
///                             tangency of a plane that is also
///                             tangent to the illumination source.
///                             See the $Particulars section below for
///                             details.
///
///                             Terminator points are generated within
///                             a specified set of "cutting"
///                             half-planes that have as an edge the
///                             line containing the illumination
///                             source center-target center vector.
///                             Multiple terminator points may be
///                             found within a given half-plane, if
///                             the target body shape allows for this.
///
///                             This is the highest-accuracy method
///                             supported by this subroutine. It
///                             generally executes much more slowly
///                             than the GUIDED method described
///                             below.
///
///                 'GUIDED'    indicates that terminator points are
///                             "guided" so as to lie on rays
///                             emanating from the target body's
///                             center and passing through the
///                             terminator on the target body's
///                             reference ellipsoid. The terminator
///                             points are constrained to lie on the
///                             target body's surface. As with the
///                             'TANGENT' method (see above), cutting
///                             half-planes are used to generate
///                             terminator points.
///
///                             The GUIDED method produces a unique
///                             terminator point for each cutting
///                             half-plane. If multiple terminator
///                             point candidates lie in a given
///                             cutting half-plane, the outermost one
///                             is chosen.
///
///                             This method may be used only with the
///                             CENTER aberration correction locus
///                             (see the description of CORLOC below).
///
///                             Terminator points generated by this
///                             method are approximations; they are
///                             generally not true ray-surface tangent
///                             points. However, these approximations
///                             can be generated much more quickly
///                             than tangent points.
///
///
///              <shape specification> may be either of the strings
///
///                 'DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                    The DSK option indicates that terminator point
///                    computation is to use topographic data provided
///                    by DSK files (abbreviated as "DSK data" below)
///                    to model the surface of the target body.
///
///                    The surface list specification is optional. The
///                    syntax of the list is
///
///                       <surface 1> [, <surface 2>...]
///
///                    If present, it indicates that data only for the
///                    listed surfaces are to be used; however, data
///                    need not be available for all surfaces in the
///                    list. If the list is absent, loaded DSK data
///                    for any surface associated with the target body
///                    are used.
///
///                    The surface list may contain surface names or
///                    surface ID codes. Names containing blanks must
///                    be delimited by double quotes, for example
///
///                       SURFACES = "Mars MEGDR 128 PIXEL/DEG"
///
///                    If multiple surfaces are specified, their names
///                    or IDs must be separated by commas.
///
///                    See the $Particulars section below for details
///                    concerning use of DSK data.
///
///
///                 'ELLIPSOID'
///
///                    The ELLIPSOID shape option generates terminator
///                    points on the target body's reference
///                    ellipsoid. When the ELLIPSOID shape is
///                    selected, The TANGENT curve option may be used
///                    with any aberration correction locus, while the
///                    GUIDED option may be used only with the CENTER
///                    locus (see the description of CORLOC below).
///
///                    When the locus is set to 'CENTER', the
///                    'TANGENT' and 'GUIDED' curve options produce
///                    the same results.
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
///  ILUSRC   is the name of the illumination source. This source
///           may be any ephemeris object. Case, blanks, and
///           numeric values are treated in the same way as for the
///           input TARGET.
///
///           The shape of the illumination source is considered
///           to be spherical. The radius of the sphere is the
///           largest radius of the source's reference ellipsoid.
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
///           The output terminator points in the array POINTS and
///           the output observer-terminator vectors in the array
///           TRMVCS are expressed relative to this reference
///           frame.
///
///
///  ABCORR   indicates the aberration corrections to be applied
///           when computing the target's position and orientation.
///           Corrections are applied at the location specified by
///           the aberration correction locus argument CORLOC,
///           which is described below.
///
///           For remote sensing applications, where apparent
///           terminator points seen by the observer are desired,
///           normally either of the corrections
///
///              'LT+S'
///              'CN+S'
///
///           should be used. These and the other supported options
///           are described below. ABCORR may be any of the
///           following:
///
///              'NONE'     Apply no correction. Return the
///                         geometric terminator points on the
///                         target body.
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
///                         'LT' option uses one iteration.
///
///                         Both the target position as seen by the
///                         observer, and rotation of the target
///                         body, are corrected for light time. The
///                         position of the illumination source as
///                         seen from the target is corrected as
///                         well.
///
///              'LT+S'     Correct for one-way light time and
///                         stellar aberration using a Newtonian
///                         formulation. This option modifies the
///                         locus obtained with the 'LT' option to
///                         account for the observer's velocity
///                         relative to the solar system
///                         barycenter. These corrections yield
///                         points on the apparent terminator.
///
///              'CN'       Converged Newtonian light time
///                         correction. In solving the light time
///                         equation, the 'CN' correction iterates
///                         until the solution converges. Both the
///                         position and rotation of the target
///                         body are corrected for light time. The
///                         position of the illumination source as
///                         seen from the target is corrected as
///                         well.
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
///                  than for the ELLIPSOID TERMINATOR option.
///
///              'ELLIPSOID TERMINATOR'
///
///                  Light time and stellar aberration corrections
///                  are applied to individual terminator points on
///                  the reference ellipsoid. For a terminator
///                  point on the surface described by topographic
///                  data, lying in a specified cutting half-plane,
///                  the unique reference ellipsoid terminator
///                  point in the same half-plane is used as the
///                  locus of the aberration corrections.
///
///                  This choice is appropriate for large target
///                  objects for which the light time from the
///                  terminator to the observer is significantly
///                  different from the light time from the target
///                  center to the observer.
///
///                  Because aberration corrections are repeated
///                  for individual terminator points,
///                  computational speed for this option is
///                  relatively slow.
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
///           half-planes in which terminator points are to be
///           found. Each cutting half-plane has as its edge the
///           line containing the illumination source center-target
///           center vector; the first half-plane contains REFVEC.
///
///           REFVEC is expressed in the body-fixed reference frame
///           designated by FIXREF.
///
///           ROLSTP is an angular step by which to roll the
///           cutting half-planes about the target-illumination
///           source vector, which we'll call the "axis." The Ith
///           half-plane is rotated from REFVEC about the axis in
///           the counter-clockwise direction by (I-1)*ROLSTP.
///           Units are radians. ROLSTP should be set to
///
///              2*pi/NCUTS
///
///           to generate an approximately uniform distribution of
///           points along the terminator.
///
///           NCUTS is the number of cutting half-planes used to
///           find terminator points; the angular positions of
///           consecutive half-planes increase in the positive
///           (counterclockwise) sense about the axis and are
///           distributed roughly equally about that vector: each
///           half-plane has angular separation of approximately
///
///              ROLSTP radians
///
///           from each of its neighbors. When the aberration
///           correction locus is set to 'CENTER', the angular
///           separation is the value above, up to round-off.
///           When the locus is 'TANGENT', the separations are
///           less uniform due to differences in the aberration
///           corrections used for the respective terminator points.
///
///
///  SCHSTP,
///  SOLTOL   are used only for DSK-based surfaces. These inputs
///           are, respectively, the search angular step size and
///           solution convergence tolerance used to find tangent
///           rays and associated terminator points within each
///           cutting half plane. These values are used when the
///           METHOD argument includes the TANGENT option. In this
///           case, terminator points are found by a two-step
///           search process:
///
///              1) Bracketing: starting with a direction having
///                 sufficiently small angular separation from the
///                 axis, rays emanating from the surface of the
///                 illumination source are generated within the
///                 half-plane at successively greater angular
///                 separations from the axis, where the increment
///                 of angular separation is SCHSTP. The rays are
///                 tested for intersection with the target
///                 surface. When a transition from
///                 non-intersection to intersection is found, the
///                 angular separation of a tangent ray has been
///                 bracketed.
///
///              2) Root finding: each time a tangent ray is
///                 bracketed, a search is done to find the angular
///                 separation from the starting direction at which
///                 a tangent ray exists. The search terminates
///                 when successive rays are separated by no more
///                 than SOLTOL. When the search converges, the
///                 last ray-surface intersection point found in
///                 the convergence process is considered to be a
///                 terminator point.
///
///
///           SCHSTP and SOLTOL have units of radians.
///
///           Target bodies with simple surfaces---for example,
///           convex shapes---will have a single terminator point
///           within each cutting half-plane. For such surfaces,
///           SCHSTP can be set large enough so that only one
///           bracketing step is taken. A value greater than pi,
///           for example 4.D0, is recommended.
///
///           Target bodies with complex surfaces can have
///           multiple terminator points within a given cutting
///           half-plane. To find all terminator points, SCHSTP
///           must be set to a value smaller than the angular
///           separation of any two terminator points in any
///           cutting half-plane, where the vertex of the angle is
///           near a point on the surface of the illumination
///           source. SCHSTP must not be too small, or the search
///           will be excessively slow.
///
///           For both kinds of surfaces, SOLTOL must be chosen so
///           that the results will have the desired precision.
///           Note that the choice of SOLTOL required to meet a
///           specified bound on terminator point height errors
///           depends on the illumination source-target distance.
///
///
///  MAXN     is the maximum number of terminator points that can
///           be stored in the output array POINTS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NPTS     is an array of counts of terminator points within
///           the specified set of cutting half-planes. The Ith
///           element of NPTS is the terminator point count in the
///           Ith half-plane. NPTS should be declared with length
///           at least NCUTS.
///
///
///  POINTS   is an array containing the terminator points found
///           by this routine. Terminator points are ordered by
///           the indices of the half-planes in which they're
///           found. The terminator points in a given half-plane
///           are ordered by decreasing angular separation from
///           the illumination source-target direction; the
///           outermost terminator point in a given half-plane is
///           the first of that set.
///
///           The terminator points for the half-plane containing
///           REFVEC occupy array elements
///
///              POINTS(1,1) through POINTS(3,NPTS(1))
///
///           Terminator points for the second half plane occupy
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
///           Terminator points are expressed in the reference
///           frame designated by FIXREF. For each terminator
///           point, the orientation of the frame is evaluated at
///           the epoch corresponding to the terminator point; the
///           epoch is provided in the output array EPOCHS
///           (described below).
///
///           Units of the terminator points are km.
///
///
///  EPOCHS   is an array of epochs associated with the terminator
///           points, accounting for light time if aberration
///           corrections are used. EPOCHS contains one element
///           for each terminator point. EPOCHS should be declared
///           with length
///
///              MAXN
///
///           The element
///
///              EPOCHS(I)
///
///           is associated with the terminator point
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
///           If CORLOC is set to 'ELLIPSOID TERMINATOR', all
///           values of EPOCHS for the terminator points in a
///           given half plane will be those for the reference
///           ellipsoid terminator point in that half plane. That
///           is, if aberration corrections are used, and if LT(I)
///           is the one-way light time to the observer from the
///           reference ellipsoid terminator point in the Ith half
///           plane, the elements of EPOCHS for that half plane
///           will all be set to
///
///              ET - LT(I)
///
///
///  TRMVCS   is an array of vectors connecting the observer to
///           the terminator points. The terminator vectors are
///           expressed in the frame designated by FIXREF. For the
///           Ith vector, the orientation of the frame is
///           evaluated at the Ith epoch provided in the output
///           array EPOCHS (described above).
///
///           TRMVCS should be declared with dimensions
///
///              ( 3, MAXN )
///
///           The elements
///
///              TRMVCS(J,I), J = 1 to 3
///
///           are associated with the terminator point
///
///              POINTS(J,I), J = 1 to 3
///
///           Units of the terminator vectors are km.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the specified aberration correction is unrecognized, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If transmission corrections are commanded, the error
///      SPICE(INVALIDOPTION) is signaled.
///
///  3)  If either the target or observer input strings cannot be
///      converted to an integer ID code, the error
///      SPICE(IDCODENOTFOUND) is signaled.
///
///  4)  If OBSRVR and TARGET map to the same NAIF integer ID code,
///      the error SPICE(BODIESNOTDISTINCT) is signaled.
///
///  5)  If the input target body-fixed frame FIXREF is not
///      recognized, the error SPICE(NOFRAME) is signaled. A frame
///      name may fail to be recognized because a required frame
///      specification kernel has not been loaded; another cause is a
///      misspelling of the frame name.
///
///  6)  If the input frame FIXREF is not centered at the target body,
///      the error SPICE(INVALIDFRAME) is signaled.
///
///  7)  If the input argument METHOD is not recognized, the error
///      SPICE(INVALIDMETHOD) is signaled by either this routine or a
///      routine in the call tree of this routine.
///
///  8)  If METHOD contains an invalid terminator type, the error
///      SPICE(INVALIDTERMTYPE) is signaled.
///
///  9)  If the target and observer have distinct identities but are
///      at the same location, the error SPICE(NOSEPARATION) is
///      signaled.
///
///  10) If insufficient ephemeris data have been loaded prior to
///      calling TERMPT, an error is signaled by a routine in
///      the call tree of this routine. When light time correction is
///      used, sufficient ephemeris data must be available to
///      propagate the states of both observer and target to the solar
///      system barycenter.
///
///  11) If the computation method requires an ellipsoidal target shape
///      and triaxial radii of the target body have not been loaded
///      into the kernel pool prior to calling TERMPT, an error is
///      signaled by a routine in the call tree of this routine.
///
///      When the target shape is modeled by topographic data, radii
///      of the reference triaxial ellipsoid are still required if
///      the aberration correction locus is ELLIPSOID TERMINATOR or if
///      the terminator point generation method is GUIDED.
///
///  12) If the target body's shape is modeled as an ellipsoid, and if
///      any of the radii of the target body are non-positive, an error
///      is signaled by a routine in the call tree of this routine. The
///      target must be an extended body.
///
///  13) If PCK data specifying the target body-fixed frame orientation
///      have not been loaded prior to calling TERMPT, an error is
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
///  18) If the GUIDED terminator type is used with the
///      ELLIPSOID TERMINATOR aberration correction locus, the
///      error SPICE(BADTERMLOCUSMIX) is signaled.
///
///  19) If the reference vector REFVEC is the zero vector, the
///      error SPICE(ZEROVECTOR) is signaled.
///
///  20) If the reference vector REFVEC and the observer target
///      vector are linearly dependent, the error
///      SPICE(DEGENERATECASE) is signaled.
///
///  21) If the terminator points cannot all be stored in the output
///      POINTS array, the error SPICE(OUTOFROOM) is signaled.
///
///  22) If NCUTS is greater than 1, the roll step ROLSTP must be
///      positive. Otherwise, the error SPICE(INVALIDROLLSTEP) is
///      signaled.
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
///  -  SPK data: ephemeris data for the target, observer, and
///     illumination source must be loaded. If aberration
///     corrections are used, the states of target and observer
///     relative to the solar system barycenter must be calculable
///     from the available ephemeris data. Typically ephemeris data
///     are made available by loading one or more SPK files via
///     FURNSH.
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
///           modeled by DSK data but one or both of the GUIDED
///           terminator definition method or the ELLIPSOID
///           TERMINATOR aberration correction locus are selected.
///
///        DSK data:
///
///           If the target shape is modeled by DSK data, DSK files
///           containing topographic data for the target body must be
///           loaded. If a surface list is specified, data for at
///           least one of the listed surfaces must be loaded.
///
///  -  Shape data for the illumination source:
///
///        PCK data:
///
///           Triaxial radii for the illumination source must be
///           loaded into the kernel pool. Typically this is done by
///           loading a text PCK file via FURNSH.
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
///  Terminator definition
///  =====================
///
///  The definitions of terminators used by this routine vary
///  depending on the target surface model.
///
///  In all cases, the surface of the illumination source is
///  modeled as a sphere.
///
///
///  Ellipsoidal target surface model
///  --------------------------------
///
///  The umbral terminator is the boundary of the set of target
///  surface points at which the illumination source is completely
///  below the local tangent plane: the entire illumination source is
///  below the horizon as seen from any surface point on the far side,
///  relative to the source, of the umbral terminator. At an umbral
///  terminator point, the target surface tangent plane containing
///  that point is tangent to the surface of the light source as well,
///  and the outward normal vectors at the two points of tangency are
///  parallel.
///
///  The penumbral terminator is the boundary of the set of target
///  surface points at which the illumination source is completely
///  above the local tangent plane: the entire illumination source is
///  above the horizon as seen from any surface point on the near
///  side, relative to the source, of the penumbral terminator. At a
///  penumbral terminator point, the target surface tangent plane
///  containing that point is tangent to the surface of the light
///  source as well, and the outward normal vectors at the two points
///  of tangency are anti-parallel.
///
///
///  Topographic target surface model (DSK case)
///  -------------------------------------------
///
///  The concept of a plane tangent to both a topographic target
///  surface and an illumination source is problematic. If the target
///  tangent point is required to lie in a given cutting half-plane
///  bounded by the line containing the target-source vector, the
///  desired plane may not exist. In general, planes tangent to both
///  the illumination source and the target will rest upon the high
///  points of the target surface.
///
///  For topographic target surface models, this routine uses a
///  modified terminator definition: terminator points are target
///  surface points at which a line is tangent to both the target and
///  the illumination source. The line is constrained to lie in the
///  plane containing the specified cutting half-plane. The concepts
///  of umbral and penumbral terminators still apply. For umbral
///  terminator points, the common tangent line does not cross the
///  target-source line; for penumbral points, it does.
///
///  Note that for ellipsoids, the terminator definitions based on
///  tangent lines are not equivalent to the definitions based on
///  tangent planes. Typically, a plane tangent to the target
///  ellipsoid at a point found by the method described above will not
///  be tangent to the illumination source: it will be rotated about
///  the common tangent line and "cut into" the sphere representing
///  the light source. This implies that some of the source will be
///  visible at umbral terminator points and some will be blocked at
///  penumbral terminator points: both umbral and penumbral terminator
///  points found by this method will lie in a region bounded by the
///  true terminators.
///
///  The two definitions are equivalent for spherical targets.
///
///
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
///        UMBRAL/TANGENT/DSK/UNPRIORITIZED/<surface list>
///        DSK/UMBRAL/TANGENT/<surface list>/UNPRIORITIZED
///        UNPRIORITIZED/<surface list>/DSK/TANGENT/UMBRAL
///
///     The simplest form of the METHOD argument specifying use of
///     DSK data is one that lacks a surface list, for example:
///
///        'PENUMBRAL/TANGENT/DSK/UNPRIORITIZED'
///        'UMBRAL/GUIDED/DSK/UNPRIORITIZED'
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
///        'UMBRAL/TANGENT/DSK/UNPRIORITIZED/
///         SURFACES= "Mars MEGDR 64 PIXEL/DEG",3'
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
///  1) Find apparent terminator points on Phobos as seen from Mars.
///     Use the "umbral" shadow definition.
///
///     Due to Phobos' irregular shape, the TANGENT terminator point
///     definition will be used. It suffices to compute light time and
///     stellar aberration corrections for the center of Phobos, so
///     the CENTER aberration correction locus will be used. Use
///     converged Newtonian light time and stellar aberration
///     corrections in order to model the apparent position and
///     orientation of Phobos.
///
///     For comparison, compute terminator points using both ellipsoid
///     and topographic shape models.
///
///     Use the target body-fixed +Z axis as the reference direction
///     for generating cutting half-planes. This choice enables the
///     user to see whether the first terminator point is near the
///     target's north pole.
///
///     For each option, use just three cutting half-planes in order
///     to keep the volume of output manageable. In most applications,
///     the number of cuts and the number of resulting terminator
///     points would be much greater.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: termpt_ex1.tm
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
///
///     Example code begins here.
///
///
///     C
///     C     Find terminator points on Phobos as seen from Mars.
///     C
///     C     Compute terminator points using the tangent
///     C     definition, using the "umbral" shadow type.
///     C     The sun is the illumination source. Perform
///     C     aberration corrections for the target center.
///     C     Use both ellipsoid and DSK shape models.
///     C
///           PROGRAM TERMPT_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      PI
///           DOUBLE PRECISION      VNORM
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'termpt_ex1.tm' )
///
///           CHARACTER*(*)         FM1
///           PARAMETER           ( FM1    =  '(A,F21.9)' )
///
///           CHARACTER*(*)         FM2
///           PARAMETER           ( FM2    =  '(1X,3F20.9)' )
///
///           CHARACTER*(*)         FM3
///           PARAMETER           ( FM3    =  '(A,I2)' )
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
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(CORLEN)    CORLOC
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(BDNMLN)    ILUSRC
///           CHARACTER*(MTHLEN)    METHOD ( NMETH )
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(BDNMLN)    TARGET
///
///           DOUBLE PRECISION      DELROL
///           DOUBLE PRECISION      DIST
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      POINTS ( 3, MAXN )
///           DOUBLE PRECISION      POS    ( 3 )
///           DOUBLE PRECISION      ROLL
///           DOUBLE PRECISION      SCHSTP
///           DOUBLE PRECISION      SOLTOL
///           DOUBLE PRECISION      TRMVCS ( 3, MAXN )
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
///     C
///     C     Initial values
///     C
///           DATA                  METHOD /
///          .               'UMBRAL/TANGENT/ELLIPSOID',
///          .               'UMBRAL/TANGENT/DSK/UNPRIORITIZED'
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
///           ILUSRC = 'SUN'
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
///     C     Compute a set of terminator points using light
///     C     time and stellar aberration corrections. Use
///     C     both ellipsoid and DSK shape models. Use an
///     C     angular step size corresponding to a height of
///     C     about 100 meters to ensure we don't miss the
///     C     terminator. Set the convergence tolerance to limit
///     C     the height convergence error to about 1 meter.
///     C     Compute 3 terminator points for each computation
///     C     method.
///     C
///     C     Get the approximate light source-target distance
///     C     at ET. We'll ignore the observer-target light
///     C     time for this approximation.
///     C
///           CALL SPKPOS ( ILUSRC, ET,  'J2000', ABCORR,
///          .              TARGET, POS, LT              )
///
///           DIST   = VNORM(POS)
///
///           SCHSTP = 1.D-1 / DIST
///           SOLTOL = 1.D-3 / DIST
///           NCUTS  = 3
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Light source:   '//ILUSRC
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
///              CALL TERMPT ( METHOD(I), ILUSRC, TARGET, ET,
///          .                 FIXREF,    ABCORR, CORLOC, OBSRVR,
///          .                 Z,         DELROL, NCUTS,  SCHSTP,
///          .                 SOLTOL,    MAXN,   NPTS,   POINTS,
///          .                 TRGEPS,    TRMVCS                 )
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
///                 WRITE(*,FM3) '    Number of terminator points  '
///          .      //           'at this roll angle: ',
///          .                   NPTS(J)
///
///                 WRITE (*,*) '      Terminator points:'
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
///      Light source:   SUN
///      Observer:       MARS
///      Target:         PHOBOS
///      Frame:          IAU_PHOBOS
///
///      Number of cuts:            3
///
///
///      Computation method = UMBRAL/TANGENT/ELLIPSOID
///      Locus              = CENTER
///
///
///       Roll angle (deg) =           0.000000000
///          Target epoch  =   271684865.152078211
///         Number of terminator points  at this roll angle:  1
///            Terminator points:
///               2.040498332         5.012722925         8.047281838
///
///       Roll angle (deg) =         120.000000000
///          Target epoch  =   271684865.152078211
///         Number of terminator points  at this roll angle:  1
///            Terminator points:
///             -11.058054707         0.167672089        -4.782740292
///
///       Roll angle (deg) =         240.000000000
///          Target epoch  =   271684865.152078211
///         Number of terminator points  at this roll angle:  1
///            Terminator points:
///               8.195238564        -6.093889437        -5.122310498
///
///
///      Computation method = UMBRAL/TANGENT/DSK/UNPRIORITIZED
///      Locus              = CENTER
///
///
///       Roll angle (deg) =           0.000000000
///          Target epoch  =   271684865.152078211
///         Number of terminator points  at this roll angle:  1
///            Terminator points:
///               1.626396028         3.995432180         8.853689595
///
///       Roll angle (deg) =         120.000000000
///          Target epoch  =   271684865.152078211
///         Number of terminator points  at this roll angle:  1
///            Terminator points:
///             -11.186660113        -0.142367244        -4.646136750
///
///       Roll angle (deg) =         240.000000000
///          Target epoch  =   271684865.152078211
///         Number of terminator points  at this roll angle:  1
///            Terminator points:
///               9.338447202        -6.091352186        -5.960849442
///
///
///  2) Find apparent terminator points on Mars as seen from the
///     earth.
///
///     Use both the "umbral" and "penumbral" shadow definitions. Use
///     only ellipsoid shape models for easier comparison. Find
///     distances between corresponding terminator points on the
///     umbral and penumbral terminators.
///
///     Use the ELLIPSOID TERMINATOR aberration correction locus
///     in order to perform separate aberration corrections for
///     each terminator point. Because of the large size of Mars,
///     corrections for the target center are less accurate.
///
///     For each option, use just three cutting half-planes, in order
///     to keep the volume of output manageable. In most applications,
///     the number of cuts and the number of resulting terminator
///     points would be much greater.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: termpt_ex2.tm
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
///           megr90n000cb_plate.bds           Plate model based on
///                                            MEGDR DEM, resolution
///                                            4 pixels/degree.
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'pck00010.tpc',
///                               'naif0011.tls',
///                               'megr90n000cb_plate.bds' )
///
///        \begintext
///
///
///     Example code begins here.
///
///
///     C
///     C     Find terminator points on Mars as seen from the
///     C     earth.
///     C
///     C     Use only ellipsoid shape models. Use the
///     C     ELLIPSOID TERMINATOR aberration correction
///     C     locus.
///     C
///     C     Use both UMBRAL and PENUMBRAL shadow definitions.
///     C     Compute the distances between corresponding
///     C     umbral and penumbral terminator points.
///     C
///     C     Check terminator points by computing solar
///     C     incidence angles at each point.
///     C
///     C
///           PROGRAM TERMPT_EX2
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
///           PARAMETER           ( META    = 'termpt_ex2.tm' )
///
///           CHARACTER*(*)         FM1
///           PARAMETER           ( FM1     =  '(A,F21.9)' )
///
///           CHARACTER*(*)         FM2
///           PARAMETER           ( FM2     =  '(A,I2)' )
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
///           PARAMETER           ( MAXN   = 100 )
///     C
///     C     Local variables
///     C
///           CHARACTER*(CORLEN)    ABCORR
///           CHARACTER*(CORLEN)    CORLOC ( NMETH )
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(MTHLEN)    ILUMTH ( NMETH )
///           CHARACTER*(BDNMLN)    ILUSRC
///           CHARACTER*(BDNMLN)    OBSRVR
///           CHARACTER*(BDNMLN)    TARGET
///           CHARACTER*(MTHLEN)    METHOD ( NMETH )
///
///           DOUBLE PRECISION      ADJANG
///           DOUBLE PRECISION      ALT
///           DOUBLE PRECISION      ANGSRC
///           DOUBLE PRECISION      DELROL
///           DOUBLE PRECISION      DIST
///           DOUBLE PRECISION      EMISSN
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      ILUPOS ( 3 )
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      LT
///           DOUBLE PRECISION      PHASE
///           DOUBLE PRECISION      POINTS ( 3, MAXN )
///           DOUBLE PRECISION      SVPNTS ( 3, MAXN )
///           DOUBLE PRECISION      TPTILU ( 3 )
///           DOUBLE PRECISION      RADII  ( 3 )
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      ROLL
///           DOUBLE PRECISION      RP
///           DOUBLE PRECISION      SCHSTP
///           DOUBLE PRECISION      SOLAR
///           DOUBLE PRECISION      SOLTOL
///           DOUBLE PRECISION      SRCRAD ( 3 )
///           DOUBLE PRECISION      SRFVEC ( 3 )
///           DOUBLE PRECISION      TRMVCS ( 3, MAXN )
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
///          .                        'ELLIPSOID TERMINATOR',
///          .                        'ELLIPSOID TERMINATOR'
///          .                             /
///
///           DATA                  ILUMTH /
///          .                        'ELLIPSOID',
///          .                        'ELLIPSOID'
///          .                             /
///
///           DATA                  METHOD /
///          .                      'UMBRAL/ELLIPSOID/TANGENT',
///          .                      'PENUMBRAL/ELLIPSOID/TANGENT'
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
///           ILUSRC = 'SUN'
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
///
///     C
///     C     Get the radii of the illumination source as well.
///     C     We'll use these radii to compute the angular radius
///     C     of the source as seen from the terminator points.
///     C
///           CALL BODVRD ( ILUSRC, 'RADII', 3, N, SRCRAD )
///     C
///     C     Compute a set of terminator points using light time and
///     C     stellar aberration corrections. Use both ellipsoid
///     C     and DSK shape models.
///     C
///     C     Get the approximate light source-target distance
///     C     at ET. We'll ignore the observer-target light
///     C     time for this approximation.
///     C
///           CALL SPKPOS ( ILUSRC, ET,     FIXREF, ABCORR,
///          .              TARGET, ILUPOS, LT             )
///
///           DIST = VNORM( ILUPOS )
///     C
///     C     Set the angular step size so that a single step will
///     C     be taken in the root bracketing process; that's all
///     C     that is needed since we don't expect to have multiple
///     C     terminator points in any cutting half-plane.
///     C
///           SCHSTP = 4.D0
///     C
///     C     Set the convergence tolerance to minimize the
///     C     height error. We can't achieve the precision
///     C     suggested by the formula because the sun-Mars
///     C     distance is about 2.4e8 km. Compute 3 terminator
///     C     points for each computation method.
///     C
///           SOLTOL = 1.D-7/DIST
///     C
///     C     Set the number of cutting half-planes and roll step.
///     C
///           NCUTS  = 3
///           DELROL = 2*PI() / NCUTS
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Light source:          '//ILUSRC
///           WRITE (*,*) 'Observer:              '//OBSRVR
///           WRITE (*,*) 'Target:                '//TARGET
///           WRITE (*,*) 'Frame:                 '//FIXREF
///           WRITE (*,*) 'Aberration Correction: '//ABCORR
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Number of cuts: ', NCUTS
///
///           DO I = 1, NMETH
///
///              CALL TERMPT ( METHOD(I), ILUSRC, TARGET,    ET,
///          .                 FIXREF,    ABCORR, CORLOC(I), OBSRVR,
///          .                 Z,         DELROL, NCUTS,     SCHSTP,
///          .                 SOLTOL,    MAXN,   NPTS,      POINTS,
///          .                 TRGEPS,    TRMVCS                    )
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
///                 WRITE(*,FM1) '    Target epoch    = ', TRGEPS(J)
///                 WRITE(*,FM2) '    Number of terminator points at '
///          .      //           'this roll angle: ',
///          .                   NPTS(J)
///
///                 DO K = 1, NPTS(J)
///
///                    WRITE (*,*) '    Terminator point planetodetic '
///          .         //          'coordinates:'
///
///                    CALL RECGEO ( POINTS(1,K+START), RE,  F,
///          .                       LON,               LAT, ALT )
///
///                    WRITE (*,FM1) '      Longitude       (deg): ',
///          .                       LON*DPR()
///                    WRITE (*,FM1) '      Latitude        (deg): ',
///          .                       LAT*DPR()
///                    WRITE (*,FM1) '      Altitude         (km): ',
///          .                       ALT
///
///     C
///     C              Get illumination angles for this terminator
///     C              point.
///     C
///                    M = K+START
///
///                    CALL ILLUMG ( ILUMTH,      TARGET, ILUSRC, ET,
///          .                       FIXREF,      ABCORR, OBSRVR,
///          .                       POINTS(1,M), TRGEPC, SRFVEC,
///          .                       PHASE,       SOLAR,  EMISSN )
///
///                    WRITE (*,FM1) '      Incidence angle '
///          .         //            '(deg): ', SOLAR * DPR()
///
///
///     C
///     C              Adjust the incidence angle for the angular
///     C              radius of the illumination source. Use the
///     C              epoch associated with the terminator point
///     C              for this lookup.
///     C
///                    CALL SPKPOS ( ILUSRC, TRGEPS(M), FIXREF,
///          .                       ABCORR, TARGET,    TPTILU, LT )
///
///                    DIST   = VNORM( TPTILU )
///
///                    ANGSRC = ASIN (  MAX( SRCRAD(1),
///          .                               SRCRAD(2),
///          .                               SRCRAD(3) )  / DIST  )
///
///                    IF ( I .EQ. 1 ) THEN
///     C
///     C                 For points on the umbral terminator,
///     C                 the ellipsoid outward normal is tilted
///     C                 away from the terminator-source center
///     C                 direction by the angular radius of the
///     C                 source. Subtract this radius from the
///     C                 illumination incidence angle to get the
///     C                 angle between the local normal and the
///     C                 direction to the corresponding tangent
///     C                 point on the source.
///     C
///                       ADJANG = SOLAR - ANGSRC
///
///                    ELSE
///     C
///     C                 For the penumbral case, the outward
///     C                 normal is tilted toward the illumination
///     C                 source by the angular radius of the
///     C                 source. Adjust the illumination
///     C                 incidence angle for this.
///     C
///                       ADJANG = SOLAR + ANGSRC
///
///                    END IF
///
///                    WRITE (*,FM1)  '      Adjusted angle  '
///          .         //             '(deg): ', ADJANG * DPR()
///
///
///                    IF ( I .EQ. 1 ) THEN
///     C
///     C                 Save terminator points for comparison.
///     C
///                       CALL VEQU ( POINTS(1,M), SVPNTS(1,M) )
///
///                    ELSE
///     C
///     C                 Compare terminator points with last
///     C                 saved values.
///     C
///                       DIST = VDIST( POINTS(1,M), SVPNTS(1,M) )
///
///                       WRITE (*,FM1)
///          .            '      Distance offset  (km): ', DIST
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
///      Light source:          SUN
///      Observer:              EARTH
///      Target:                MARS
///      Frame:                 IAU_MARS
///      Aberration Correction: CN+S
///
///      Number of cuts:            3
///
///      Computation method = UMBRAL/ELLIPSOID/TANGENT
///      Locus              = ELLIPSOID TERMINATOR
///
///        Roll angle (deg) =           0.000000000
///         Target epoch    =   271683700.369686902
///         Number of terminator points at this roll angle:  1
///          Terminator point planetodetic coordinates:
///           Longitude       (deg):           4.189318082
///           Latitude        (deg):          66.416132677
///           Altitude         (km):           0.000000000
///           Incidence angle (deg):          90.163842885
///           Adjusted angle  (deg):          89.999999980
///
///        Roll angle (deg) =         120.000000000
///         Target epoch    =   271683700.372003794
///         Number of terminator points at this roll angle:  1
///          Terminator point planetodetic coordinates:
///           Longitude       (deg):         107.074551917
///           Latitude        (deg):         -27.604435701
///           Altitude         (km):           0.000000000
///           Incidence angle (deg):          90.163842793
///           Adjusted angle  (deg):          89.999999888
///
///        Roll angle (deg) =         240.000000000
///         Target epoch    =   271683700.364983618
///         Number of terminator points at this roll angle:  1
///          Terminator point planetodetic coordinates:
///           Longitude       (deg):         -98.695906077
///           Latitude        (deg):         -27.604435700
///           Altitude         (km):          -0.000000000
///           Incidence angle (deg):          90.163843001
///           Adjusted angle  (deg):          90.000000096
///
///
///      Computation method = PENUMBRAL/ELLIPSOID/TANGENT
///      Locus              = ELLIPSOID TERMINATOR
///
///        Roll angle (deg) =           0.000000000
///         Target epoch    =   271683700.369747400
///         Number of terminator points at this roll angle:  1
///          Terminator point planetodetic coordinates:
///           Longitude       (deg):           4.189317837
///           Latitude        (deg):          66.743818467
///           Altitude         (km):           0.000000000
///           Incidence angle (deg):          89.836157094
///           Adjusted angle  (deg):          89.999999999
///           Distance offset  (km):          19.483590936
///
///        Roll angle (deg) =         120.000000000
///         Target epoch    =   271683700.372064054
///         Number of terminator points at this roll angle:  1
///          Terminator point planetodetic coordinates:
///           Longitude       (deg):         107.404259674
///           Latitude        (deg):         -27.456458359
///           Altitude         (km):          -0.000000000
///           Incidence angle (deg):          89.836157182
///           Adjusted angle  (deg):          90.000000087
///           Distance offset  (km):          19.411414247
///
///        Roll angle (deg) =         240.000000000
///         Target epoch    =   271683700.365043879
///         Number of terminator points at this roll angle:  1
///          Terminator point planetodetic coordinates:
///           Longitude       (deg):         -99.025614323
///           Latitude        (deg):         -27.456458357
///           Altitude         (km):           0.000000000
///           Incidence angle (deg):          89.836156972
///           Adjusted angle  (deg):          89.999999877
///           Distance offset  (km):          19.411437239
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
/// -    SPICELIB Version 1.1.0, 12-SEP-2021 (NJB) (JDR)
///
///         Bug fix: PRVCOR is no longer set to blank before
///         ABCORR is parsed.
///
///         ZZVALCOR is now used instead of ZZPRSCOR. This provides
///         better error handling.
///
///         Edited the header to comply with NAIF standard.
///         Corrected the filename of code example #1's meta-kernel.
///         Removed unnecessary SAVE statement from code example #2.
///
/// -    SPICELIB Version 1.0.0, 04-APR-2017 (NJB)
///
///         11-MAR-2016 (NJB)
///
///         Changed ellipsoid algorithm to use ZZEDTMPT. Added ROLSTP
///         argument. Updated calls to ZZTANGNT to accommodate argument
///         list change. Added code examples. Updated $Detailed_Input. Made
///         various header corrections.
///
///         Original version 18-NOV-2015 (NJB)
/// ```
pub fn termpt(
    ctx: &mut SpiceContext,
    method: &str,
    ilusrc: &str,
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
    trmvcs: &mut [[f64; 3]],
) -> crate::Result<()> {
    TERMPT(
        method.as_bytes(),
        ilusrc.as_bytes(),
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
        trmvcs.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TERMPT ( Terminator points on an extended object )
pub fn TERMPT(
    METHOD: &[u8],
    ILUSRC: &[u8],
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
    TRMVCS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let REFVEC = DummyArray::new(REFVEC, 1..=3);
    let mut NPTS = DummyArrayMut::new(NPTS, 1..);
    let mut POINTS = DummyArrayMut2D::new(POINTS, 1..=3, 1..);
    let mut EPOCHS = DummyArrayMut::new(EPOCHS, 1..);
    let mut TRMVCS = DummyArrayMut2D::new(TRMVCS, 1..=3, 1..);
    let mut PNTDEF = [b' '; CVTLEN as usize];
    let mut NRMLOC = [b' '; ACLLEN as usize];
    let mut SHPSTR = [b' '; SHPLEN as usize];
    let mut TRMSTR = [b' '; TMTLEN as usize];
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut CP = StackArray::<f64, 3>::new(1..=3);
    let mut EDIR = StackArray::<f64, 3>::new(1..=3);
    let mut EPOCH: f64 = 0.0;
    let mut ICORVC = StackArray::<f64, 3>::new(1..=3);
    let mut ILUMLT: f64 = 0.0;
    let mut ILURAD: f64 = 0.0;
    let mut ITRMVC = StackArray::<f64, 3>::new(1..=3);
    let mut LT: f64 = 0.0;
    let mut LTERR: f64 = 0.0;
    let mut PLNVEC = StackArray::<f64, 3>::new(1..=3);
    let mut PRVLT: f64 = 0.0;
    let mut PTARG = StackArray::<f64, 3>::new(1..=3);
    let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
    let mut RAYVTX = StackArray::<f64, 3>::new(1..=3);
    let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
    let mut ROLL: f64 = 0.0;
    let mut SRFVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SSBLT: f64 = 0.0;
    let mut SSBTRG = StackArray::<f64, 3>::new(1..=3);
    let mut STLOFF = StackArray::<f64, 3>::new(1..=3);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
    let mut TRGEPC: f64 = 0.0;
    let mut TRGPOS = StackArray::<f64, 3>::new(1..=3);
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
        ZZCTRUIN(save.SVCTR6.as_slice_mut(), ctx);
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
        // In this routine, we don't allow transmission corrections.
        //
        if ATTBLK[XMTIDX] {
            SETMSG(b"Aberration correction # calls for transmission-style corrections. These are not supported for terminator finding.", ctx);
            ERRCH(b"#", ABCORR, ctx);
            SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
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

        //
        // The aberration correction flag is valid; save it.
        //
        fstr::assign(&mut save.PRVCOR, ABCORR);
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
        &mut save.TRGCDE,
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
    // Obtain an integer code for the illumination source.
    //
    ZZBODS2C(
        save.SVCTR3.as_slice_mut(),
        &mut save.SVILUM,
        &mut save.SVICDE,
        &mut save.SVFND3,
        ILUSRC,
        &mut save.ILUCDE,
        &mut FND,
        ctx,
    )?;

    if !FND {
        SETMSG(b"The illumination source, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", ILUSRC, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Check the observer and target body codes. If they are equal,
    // signal an error. The illumination source must be distinct
    // from the target as well.
    //
    if (OBSCDE == save.TRGCDE) {
        SETMSG(b"In computing the terminator, the observing body and target body are the same. Both are #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if (save.ILUCDE == save.TRGCDE) {
        SETMSG(b"In computing the terminator, the observing body and illumination source are the same. Both are #.", ctx);
        ERRCH(b"#", OBSRVR, ctx);
        SIGERR(b"SPICE(BODIESNOTDISTINCT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Determine the attributes of the frame designated by FIXREF.
    //
    ZZNAMFRM(
        save.SVCTR4.as_slice_mut(),
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
    if (FXCENT != save.TRGCDE) {
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
    ZZSRFTRK(save.SVCTR5.as_slice_mut(), &mut SURFUP, ctx)?;

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
            save.TRGCDE,
            METHOD,
            MAXSRF,
            &mut SHPSTR,
            &mut save.SUBTYP,
            &mut save.PRI,
            &mut save.NSURF,
            save.SRFLST.as_slice_mut(),
            &mut PNTDEF,
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

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if EQSTR(&PNTDEF, b"TANGENT") {
            save.TRMTYP = TANGNT;
        } else if EQSTR(&PNTDEF, b"GUIDED") {
            save.TRMTYP = GUIDED;
        } else {
            SETMSG(b"Returned point definition from method string was <#>. Value must be TANGENT or GUIDED.", ctx);
            ERRCH(b"#", &PNTDEF, ctx);
            SIGERR(b"SPICE(INVALIDTERMTYPE)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if EQSTR(&TRMSTR, b"UMBRAL") {
            save.SHADOW = UMBRAL;
            save.UFLAG = true;
        } else if EQSTR(&TRMSTR, b"PENUMBRAL") {
            save.SHADOW = PNMBRL;
            save.UFLAG = false;
        } else {
            SETMSG(b"Returned shadow type from method string was <#>. Value must be UMBRAL or PENUMBRAL.", ctx);
            ERRCH(b"#", &TRMSTR, ctx);
            SIGERR(b"SPICE(INVALIDSHADOW)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // There should be no subtype specification in the method
        // string.
        //
        if fstr::ne(&save.SUBTYP, b" ") {
            SETMSG(b"Spurious sub-observer point type <#> was present in the method string #. The sub-observer type is valid in the method strings for SUBPNT and SUBSLR, but is not applicable for TERMPT.", ctx);
            ERRCH(b"#", &save.SUBTYP, ctx);
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Save the current method as the previous method that we've
        // successfully processed the input method.
        //
        fstr::assign(&mut save.PRVMTH, METHOD);
    }

    //
    // Identify the aberration correction locus.
    //
    if (save.FIRST || fstr::ne(CORLOC, &save.PRVLOC)) {
        LJUCRS(1, CORLOC, &mut NRMLOC, ctx);

        if fstr::eq(&NRMLOC, b"CENTER") {
            save.LOCCDE = CTRCOR;
        } else if fstr::eq(&NRMLOC, b"ELLIPSOID TERMINATOR") {
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
        save.SVLCOD = save.LOCCDE;
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
    // Check NCUTS; there must be room for at least one terminator point
    // for each cut. NCUTS may not be negative.
    //
    if ((NCUTS < 0) || (NCUTS > MAXN)) {
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
    // Check ROLSTP, if this step is needed.
    //
    if (NCUTS > 1) {
        if (ROLSTP == 0.0) {
            SETMSG(b"ROLSTP is zero; NCUTS = #; the roll step is required to be non-zero when more than one cutting half-plane is used.", ctx);
            ERRINT(b"#", NCUTS, ctx);
            SIGERR(b"SPICE(INVALIDROLLSTEP)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    }

    if (save.SHAPE == DSKSHP) {
        //
        // This is the DSK case.
        //
        // Initialize the intercept algorithm to use a DSK
        // model for the surface of the target body.
        //
        ZZSUDSKI(save.TRGCDE, save.NSURF, save.SRFLST.as_slice(), FXFCDE, ctx)?;
        //
        // Save the radius of the outer bounding sphere of
        // the target.
        //
        ZZMAXRAD(&mut save.MAXRAD, ctx);
    } else if (save.SHAPE != ELLSHP) {
        SETMSG(b"Computation method argument was <#>; this string must specify a supported shape model and computation type. See the description of METHOD in the header of TERMPT for details.", ctx);
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
    // Get illumination source radii.
    //
    if (save.ILUCDE != save.PRVILU) {
        //
        // Reset counter to force lookup.
        //
        ZZCTRUIN(save.SVCTR7.as_slice_mut(), ctx);
    }

    //
    // Look up illumination source radii using counter.
    //
    ZZBODVCD(
        save.ILUCDE,
        b"RADII",
        3,
        save.SVCTR7.as_slice_mut(),
        &mut save.NRAD,
        save.SVSRAD.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        //
        // Make sure we don't reuse the outputs from ZZBODVCD.
        //
        save.PRVILU = 0;

        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if (save.NRAD != 3) {
        SETMSG(
            b"Number of illumination source radii must be 3 but was #.",
            ctx,
        );
        ERRINT(b"#", save.NRAD, ctx);
        SIGERR(b"SPICE(BADRADIUSCOUNT)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // Obtain the largest radius of the source.
    //
    ILURAD = intrinsics::DMAX1(&[save.SVSRAD[1], save.SVSRAD[2], save.SVSRAD[3]]);

    save.PRVILU = save.ILUCDE;

    //
    // Get target body radii if necessary.
    //
    if (((save.SHAPE == ELLSHP) || (save.SVLCOD == ELLCOR)) || (save.TRMTYP == GUIDED)) {
        if (save.TRGCDE != save.PRVTRG) {
            //
            // Reset counter to force lookup.
            //
            ZZCTRUIN(save.SVCTR6.as_slice_mut(), ctx);
        }
        //
        // Look up target radii using counter.
        //
        ZZBODVCD(
            save.TRGCDE,
            b"RADII",
            3,
            save.SVCTR6.as_slice_mut(),
            &mut save.NRAD,
            save.SVTRAD.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if (save.NRAD != 3) {
            SETMSG(b"Number of target radii must be 3 but was #.", ctx);
            ERRINT(b"#", save.NRAD, ctx);
            SIGERR(b"SPICE(BADRADIUSCOUNT)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        save.PRVTRG = save.TRGCDE;
    }

    //
    // Set up activities are complete at this point.
    //
    //
    // Find terminator points on the target.
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
        TRGPOS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if VZERO(TRGPOS.as_slice()) {
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
    // The terminator-finding technique depends on the aberration
    // correction locus. Start with the 'CENTER' version, since this is
    // the simpler case.
    //
    if (save.SVLCOD == CTRCOR) {
        //
        // Aberration corrections are those applicable at the target
        // center.
        //
        // Compute the epoch associated with the target center.
        //
        ZZCOREPC(ABCORR, ET, LT, &mut TRGEPC, ctx)?;

        //
        // Get the vector from the target center to the illumination
        // source.
        //
        SPKPOS(
            ILUSRC,
            TRGEPC,
            FIXREF,
            ABCORR,
            TARGET,
            AXIS.as_slice_mut(),
            &mut ILUMLT,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // The target-source vector is the central axis.
        //
        // Make sure the reference vector and axis are linearly
        // independent.
        //
        VCRSS(AXIS.as_slice(), REFVEC.as_slice(), CP.as_slice_mut());

        if VZERO(CP.as_slice()) {
            SETMSG(b"Input reference vector and illumination source-target vector are linearly  dependent.", ctx);
            SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        TO = 1;
        ROOM = MAXN;
        TOTAL = 0;

        //
        // Loop over the half planes, collecting terminator points for
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

            if (save.SHAPE == DSKSHP) {
                //
                // This is the DSK case.
                //
                if (save.TRMTYP == TANGNT) {
                    //
                    // This type of solution finds actual tangent rays on
                    // the target.
                    //
                    // Find the terminator points that lie in the current
                    // half-plane.
                    //
                    // Note that RESULT is a cell, not a window.
                    //
                    SCARDD(0, RESULT.as_slice_mut(), ctx)?;
                    //
                    // Note that the evaluation epoch for the surface is
                    // optionally corrected for light time.
                    //
                    // For this computation, the ray's vertex is computed
                    // on the fly, since it depends on the ray's direction.
                    // The location of the center of the source is passed
                    // to the tangent utilities instead.
                    //
                    ZZTANGNT(
                        save.SHADOW,
                        ILURAD,
                        save.SHAPE,
                        save.TRGCDE,
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
                } else if (save.TRMTYP == GUIDED) {
                    //
                    // This option uses the target's reference ellipsoid for
                    // guidance. For DSK shapes, the DSK terminator points
                    // are generated by finding terminator points on the
                    // target body's reference ellipsoid, then finding
                    // topographic surface intercepts of rays emanating from
                    // the target body's center and passing through the
                    // terminator points on the ellipsoid. If multiple
                    // intercepts are found for a given ray, the outermost
                    // is selected.
                    //
                    // Find the terminator point on the ellipsoid in the
                    // current cutting half-plane.
                    //
                    ZZEDTMPT(
                        save.UFLAG,
                        save.SVTRAD[1],
                        save.SVTRAD[2],
                        save.SVTRAD[3],
                        ILURAD,
                        AXIS.as_slice(),
                        PLNVEC.as_slice(),
                        save.PNTBUF.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    NPTS[I] = 1;

                    VHAT(save.PNTBUF.subarray([1, 1]), EDIR.as_slice_mut());
                    //
                    // Find the intercept on the target surface of the ray
                    // emanating from the target in the direction of EDIR.
                    // We must use a ray pointed in the opposite direction
                    // to perform this computation, since the surface may be
                    // invisible from the interior of the target.
                    //
                    VSCL((3.0 * save.MAXRAD), EDIR.as_slice(), RAYVTX.as_slice_mut());
                    VMINUS(EDIR.as_slice(), RAYDIR.as_slice_mut());

                    ZZSUDSKI(save.TRGCDE, save.NSURF, save.SRFLST.as_slice(), FXFCDE, ctx)?;

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
                    SETMSG(b"Invalid terminator type code: #", ctx);
                    ERRINT(b"#", save.TRMTYP, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            } else if (save.SHAPE == ELLSHP) {
                //
                // This is the ellipsoid case.
                //
                // Find the terminator point in the current cutting
                // half-plane.
                //
                ZZEDTMPT(
                    save.UFLAG,
                    save.SVTRAD[1],
                    save.SVTRAD[2],
                    save.SVTRAD[3],
                    ILURAD,
                    AXIS.as_slice(),
                    PLNVEC.as_slice(),
                    save.PNTBUF.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                SCARDD(0, RESULT.as_slice_mut(), ctx)?;

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
                SETMSG(b"Out of room in output arrays. Index of cutting half-plane is # out of #. Number of terminator points collected so far is #. Available room is #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", NCUTS, ctx);
                ERRINT(b"#", TOTAL, ctx);
                ERRINT(b"#", ROOM, ctx);
                SIGERR(b"SPICE(OUTOFROOM)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Transfer the terminator points we found to the output
            // terminator point array. Set the elements of the tangent
            // vector array as we go. Store in each element of the output
            // array the epoch associated with the target center.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = NPTS[I];
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    VEQU(save.PNTBUF.subarray([1, J]), POINTS.subarray_mut([1, TO]));
                    VADD(
                        save.PNTBUF.subarray([1, J]),
                        TRGPOS.as_slice(),
                        TRMVCS.subarray_mut([1, TO]),
                    );

                    EPOCHS[TO] = TRGEPC;

                    TO = (TO + 1);

                    ROOM = (ROOM - NPTS[I]);

                    J += m3__;
                }
            }
        }
    } else if (save.SVLCOD == ELLCOR) {
        //
        // Aberration corrections are done for each cutting half plane.
        // Corrections are performed for the intersections of the
        // half plane with the reference ellipsoid's terminator.
        //
        // This locus is supported only for the "tangent" terminator
        // point method.
        //
        if (save.TRMTYP != TANGNT) {
            SETMSG(b"Terminator point definition type <#> is not supported for the # aberration correction locus.", ctx);
            ERRCH(b"#", &PNTDEF, ctx);
            ERRCH(b"#", CORLOC, ctx);
            SIGERR(b"SPICE(BADTERMLOCUSMIX)", ctx)?;
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
        // Loop over the half planes, collecting terminator points for
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
                    NUMITR = 1;
                }

                J = 0;
                LTERR = 1.0;

                while ((J < NUMITR) && (LTERR > CNVLIM)) {
                    //
                    // LT was set either prior to this loop or
                    // during the previous loop iteration.
                    //
                    EPOCH = (ET - LT);

                    SPKGPS(
                        save.TRGCDE,
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

                    //
                    // Find the position of the illumination source relative
                    // to the target center at EPOCH.
                    //
                    SPKEZP(
                        save.ILUCDE,
                        EPOCH,
                        FIXREF,
                        ABCORR,
                        save.TRGCDE,
                        AXIS.as_slice_mut(),
                        &mut ILUMLT,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }
                    //
                    // The illumination source position vector gives us
                    // the axis for the terminator computation.
                    //
                    // Let PLNVEC be the secondary vector defining the
                    // cutting half-plane. Rotation of the half-planes is in
                    // the positive sense about AXIS.
                    //
                    VROTV(
                        REFVEC.as_slice(),
                        AXIS.as_slice(),
                        ROLL,
                        PLNVEC.as_slice_mut(),
                    );

                    //
                    // Find the terminator point on the reference
                    // ellipsoid, in the cutting half-plane.
                    //
                    ZZEDTMPT(
                        save.UFLAG,
                        save.SVTRAD[1],
                        save.SVTRAD[2],
                        save.SVTRAD[3],
                        ILURAD,
                        AXIS.as_slice(),
                        PLNVEC.as_slice(),
                        save.PNTBUF.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    NPTS[I] = 1;
                    //
                    //
                    // Compute the vector from the observer to the terminator
                    // point.
                    //
                    PXFORM(IREF, FIXREF, EPOCH, XFORM.as_slice_mut(), ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    MXV(XFORM.as_slice(), PTARG.as_slice(), TRGPOS.as_slice_mut());
                    VADD(
                        save.PNTBUF.subarray([1, 1]),
                        TRGPOS.as_slice(),
                        SRFVEC.as_slice_mut(),
                    );

                    //
                    // Compute the light time to the terminator point.
                    //
                    PRVLT = LT;
                    LT = TOUCHD((VNORM(SRFVEC.as_slice()) / CLIGHT()));

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
                // We now have the light time offset (but not the stellar
                // aberration correction) applicable to the terminator
                // point on the ellipsoid for the current half plane.
                // Compute the vertex and axis for the terminator point
                // computation.
                //
                EPOCH = (ET - LT);
                //
                // Compute the position of the target at EPOCH relative
                // to the observer at ET. This vector is computed in
                // an inertial frame.
                //
                SPKGPS(
                    save.TRGCDE,
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
                //
                // Transform the observer-target position to the body-fixed
                // frame at EPOCH.
                //
                PXFORM(IREF, FIXREF, EPOCH, XFORM.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // If we're using stellar aberration corrections, find the
                // correction applicable to the ellipsoid terminator point.
                //
                if save.USESTL {
                    //
                    // The vector ICORVC below is the inertially-referenced
                    // stellar aberration correction.
                    //
                    MTXV(
                        XFORM.as_slice(),
                        save.PNTBUF.subarray([1, 1]),
                        TMPVEC.as_slice_mut(),
                    );
                    VADD(TMPVEC.as_slice(), PTARG.as_slice(), ITRMVC.as_slice_mut());

                    STELAB(
                        ITRMVC.as_slice(),
                        STOBS.subarray(4),
                        TMPVEC.as_slice_mut(),
                        ctx,
                    )?;
                    VSUB(TMPVEC.as_slice(), ITRMVC.as_slice(), ICORVC.as_slice_mut());

                    MXV(XFORM.as_slice(), ICORVC.as_slice(), STLOFF.as_slice_mut());
                }

                MXV(XFORM.as_slice(), PTARG.as_slice(), TMPVEC.as_slice_mut());
                VEQU(TMPVEC.as_slice(), TRGPOS.as_slice_mut());

                //
                // Find the apparent position of the illumination source
                // relative to the target at EPOCH.
                //
                SPKEZP(
                    save.ILUCDE,
                    EPOCH,
                    FIXREF,
                    ABCORR,
                    save.TRGCDE,
                    AXIS.as_slice_mut(),
                    &mut ILUMLT,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            } else {
                //
                // This is the geometric case.
                //
                // Get the position of the illumination source
                // as seen from the target at ET.
                //
                SPKEZP(
                    save.ILUCDE,
                    ET,
                    FIXREF,
                    ABCORR,
                    save.TRGCDE,
                    AXIS.as_slice_mut(),
                    &mut ILUMLT,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
                //
                // The target epoch matches the observer epoch.
                //
                EPOCH = ET;
                //
                // The position of the target relative to the observer
                // is already present in POS.
                //
            }

            //
            // POS, EPOCH, and AXIS are set.
            //
            // Reset the plane definition vector PLNVEC based on the new
            // value of AXIS.
            //
            VROTV(
                REFVEC.as_slice(),
                AXIS.as_slice(),
                ROLL,
                PLNVEC.as_slice_mut(),
            );
            //
            // We're ready to compute the terminator point in the current
            // half-plane.
            //
            if (save.SHAPE == DSKSHP) {
                //
                // Find the terminator points on the target surface as
                // modeled by DSK data.
                //
                SCARDD(0, RESULT.as_slice_mut(), ctx)?;
                //
                // Note that the evaluation epoch for the surface is
                // corrected for light time.
                //
                ZZTANGNT(
                    save.SHADOW,
                    ILURAD,
                    save.SHAPE,
                    save.TRGCDE,
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

                NPTS[I] = CARDD(RESULT.as_slice(), ctx)?;
            } else {
                //
                // Find the terminator point on the target surface modeled
                // by an ellipsoid.
                //
                // If we performed a light time computation, we already
                // have the answer stored in PNTBUF. If this is a geometric
                // computation, we still need to compute the terminator
                // point.
                //
                if !save.USELT {
                    ZZEDTMPT(
                        save.UFLAG,
                        save.SVTRAD[1],
                        save.SVTRAD[2],
                        save.SVTRAD[3],
                        ILURAD,
                        AXIS.as_slice(),
                        PLNVEC.as_slice(),
                        save.PNTBUF.as_slice_mut(),
                        ctx,
                    )?;
                }

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                NPTS[I] = 1;
            }

            TOTAL = (TOTAL + NPTS[I]);

            if (NPTS[I] > ROOM) {
                SETMSG(b"Out of room in output arrays. Index of cutting half-plane is # out of #. Number of terminator points collected so far is #. Available room is #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", NCUTS, ctx);
                ERRINT(b"#", TOTAL, ctx);
                ERRINT(b"#", ROOM, ctx);
                SIGERR(b"SPICE(OUTOFROOM)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Transfer the terminator points we found to the output
            // terminator point array. Set the elements of the tangent
            // vector array as we go. In this case, we set the elements of
            // the output target epoch array as well.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = NPTS[I];
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    VEQU(save.PNTBUF.subarray([1, J]), POINTS.subarray_mut([1, TO]));
                    VADD(
                        save.PNTBUF.subarray([1, J]),
                        TRGPOS.as_slice(),
                        TRMVCS.subarray_mut([1, TO]),
                    );

                    if save.USESTL {
                        //
                        // Apply the stellar aberration offset for the current
                        // half-plane to each terminator vector in the output
                        // buffer.
                        //
                        VADD(
                            TRMVCS.subarray([1, TO]),
                            STLOFF.as_slice(),
                            TMPVEC.as_slice_mut(),
                        );
                        VEQU(TMPVEC.as_slice(), TRMVCS.subarray_mut([1, TO]));
                    }

                    EPOCHS[TO] = EPOCH;

                    TO = (TO + 1);

                    ROOM = (ROOM - NPTS[I]);

                    J += m3__;
                }
            }
            //
            // We've found the terminator points and observer-terminator
            // vectors for the Ith half-plane.
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
