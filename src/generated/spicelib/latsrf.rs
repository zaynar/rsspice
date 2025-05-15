//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXSRF: i32 = 100;
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
const RNAME: &[u8] = b"LATSRF";
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVMTH: Vec<u8>,
    NSURF: i32,
    SHAPE: i32,
    SRFCTR: StackArray<i32, 2>,
    SRFLST: StackArray<i32, 100>,
    FIRST: bool,
    PRI: bool,
    SVCTR1: StackArray<i32, 2>,
    SVTARG: Vec<u8>,
    SVTCDE: i32,
    SVFND1: bool,
    SVCTR2: StackArray<i32, 2>,
    SVFREF: Vec<u8>,
    SVFXFC: i32,
    SVCTR3: StackArray<i32, 2>,
    SVPRVT: i32,
    SVRADI: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVMTH = vec![b' '; MTHLEN as usize];
        let mut NSURF: i32 = 0;
        let mut SHAPE: i32 = 0;
        let mut SRFCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SRFLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut FIRST: bool = false;
        let mut PRI: bool = false;
        let mut SVCTR1 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVTARG = vec![b' '; BDNMLN as usize];
        let mut SVTCDE: i32 = 0;
        let mut SVFND1: bool = false;
        let mut SVCTR2 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVFREF = vec![b' '; FRNMLN as usize];
        let mut SVFXFC: i32 = 0;
        let mut SVCTR3 = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut SVPRVT: i32 = 0;
        let mut SVRADI = StackArray::<f64, 3>::new(1..=3);

        FIRST = true;
        fstr::assign(&mut PRVMTH, b" ");
        SVPRVT = 0;

        Self {
            PRVMTH,
            NSURF,
            SHAPE,
            SRFCTR,
            SRFLST,
            FIRST,
            PRI,
            SVCTR1,
            SVTARG,
            SVTCDE,
            SVFND1,
            SVCTR2,
            SVFREF,
            SVFXFC,
            SVCTR3,
            SVPRVT,
            SVRADI,
        }
    }
}

/// Latitudinal grid to surface points
///
/// Map array of planetocentric longitude/latitude coordinate pairs
/// to surface points on a specified target body.
///
/// The surface of the target body may be represented by a triaxial
/// ellipsoid or by topographic data provided by DSK files.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
/// * [FRAMES](crate::required_reading::frames)
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
///  ET         I   Epoch in TDB seconds past J2000 TDB.
///  FIXREF     I   Body-fixed, body-centered target body frame.
///  NPTS       I   Number of coordinate pairs in input array.
///  LONLAT     I   Array of longitude/latitude coordinate pairs.
///  SRFPTS     O   Array of surface points.
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
///              'ELLIPSOID'
///
///                 The surface point computation uses a triaxial
///                 ellipsoid to model the surface of the target
///                 body. The ellipsoid's radii must be available
///                 in the kernel pool.
///
///              'DSK/UNPRIORITIZED[/SURFACES = <surface list>]'
///
///                 The surface point computation uses topographic
///                 data to model the surface of the target body.
///                 These data must be provided by loaded DSK
///                 files.
///
///                 The surface list specification is optional. The
///                 syntax of the list is
///
///                    <surface 1> [, <surface 2>...]
///
///                 If present, it indicates that data only for the
///                 listed surfaces are to be used; however, data
///                 need not be available for all surfaces in the
///                 list. If absent, loaded DSK data for any surface
///                 associated with the target body are used.
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
///
///           Neither case nor white space are significant in
///           METHOD, except within double-quoted strings. For
///           example, the string ' eLLipsoid ' is valid.
///
///           Within double-quoted strings, blank characters are
///           significant, but multiple consecutive blanks are
///           considered equivalent to a single blank. Case is
///           not significant. So
///
///              "Mars MEGDR 128 PIXEL/DEG"
///
///           is equivalent to
///
///              " mars megdr  128  pixel/deg "
///
///           but not to
///
///              "MARS MEGDR128PIXEL/DEG"
///
///
///  TARGET   is the name of the target body. TARGET is
///           case-insensitive, and leading and trailing blanks in
///           TARGET are not significant. Optionally, you may
///           supply a string containing the integer ID code for
///           the object. For example both 'MOON' and '301' are
///           legitimate strings that indicate the Moon is the
///           target body.
///
///           When the target body's surface is represented by a
///           tri-axial ellipsoid, this routine assumes that a
///           kernel variable representing the ellipsoid's radii is
///           present in the kernel pool. Normally the kernel
///           variable would be defined by loading a PCK file.
///
///
///  ET       is the epoch for which target surface data will be
///           selected, if the surface is modeled using DSK data.
///           In this case, only segments having time coverage that
///           includes the epoch ET will be used.
///
///           ET is ignored if the target is modeled as an
///           ellipsoid.
///
///           ET is expressed as TDB seconds past J2000 TDB.
///
///
///  FIXREF   is the name of a body-fixed reference frame centered
///           on the target body.
///
///           If the target shape is given by DSK data, FIXREF may
///           designate any such frame supported by the SPICE
///           system, including built-in frames (documented in the
///           Frames Required Reading) and frames defined by a
///           loaded frame kernel (FK).
///
///           When the target surface is modeled as an ellipsoid,
///           the reference frame designated by FIXREF (described
///           below) must have its coordinate axes aligned with the
///           respective principal axes of the reference ellipsoid.
///
///           The string FIXREF is case-insensitive, and leading
///           and trailing blanks in FIXREF are not significant.
///
///           The output surface points in the array SRFPTS will be
///           expressed relative to this reference frame.
///
///
///  NPTS     is the number of coordinate pairs in the array LONLAT.
///
///
///  LONLAT   is an array of pairs of planetocentric longitudes and
///           latitudes of surface points. Elements
///
///              LONLAT(1,I)
///              LONLAT(2,I)
///
///           are, respectively, the planetocentric longitude and
///           latitude of the Ith surface point.
///
///           The units of longitude and latitude are radians.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SRFPTS   is an array of target body surface points
///           corresponding to the pairs of coordinates in the
///           input LONLAT array. Elements
///
///              SRFPTS(1,I)
///              SRFPTS(2,I)
///              SRFPTS(3,I)
///
///           are the Cartesian coordinates, expressed in the
///           reference frame designated by FIXREF, of the surface
///           point corresponding to the Ith pair of input
///           coordinates.
///
///           If there are multiple solutions for a given input
///           coordinate pair, this routine will return the point
///           at those coordinates having the greatest distance
///           from the origin of the coordinate system.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the target body name input string cannot be converted to an
///      integer ID code, the error SPICE(IDCODENOTFOUND) is signaled.
///
///  2)  If the input target body-fixed frame FIXREF is not
///      recognized, the error SPICE(NOFRAME) is signaled. A frame
///      name may fail to be recognized because a required frame
///      specification kernel has not been loaded; another cause is a
///      misspelling of the frame name.
///
///  3)  If the input frame FIXREF is not centered at the target body,
///      the error SPICE(INVALIDFRAME) is signaled.
///
///  4)  If data are not available to convert between the frame
///      FIXREF and the frame of a DSK segment of interest, an error
///      is signaled by a routine in the call tree of this
///      routine.
///
///  5)  If the input argument METHOD cannot be parsed, an error
///      is signaled by either this routine or a routine in
///      the call tree of this routine.
///
///  6)  If the computation method specifies an ellipsoidal target
///      model, and if triaxial radii of the target body have not been
///      loaded into the kernel pool prior to calling LATSRF, an error
///      is signaled by a routine in the call tree of this routine.
///
///  7)  If the computation method specifies an ellipsoidal target
///      model, and if any of the radii of the target body are
///      non-positive, an error is signaled by a routine in the call
///      tree of this routine. The target must be an extended body.
///
///  8)  If METHOD specifies that the target surface is represented by
///      DSK data, and no DSK files are loaded for the specified
///      target, an error is signaled by a routine in the call tree
///      of this routine.
///
///  9)  If METHOD specifies that the target surface is represented
///      by DSK data, and data representing the portion of the surface
///      corresponding to the coordinates provided in LONLAT are not
///      available, an error is signaled by a routine in the call
///      tree of this routine.
///
///  10) If a surface point cannot be computed because the ray
///      corresponding to a longitude/latitude pair fails to intersect
///      the target surface as defined by the plate model, the error
///      SPICE(NOINTERCEPT) is signaled.
///
///  11) If the surface point corresponding to a longitude/latitude
///      pair in LONLAT does not have matching longitude and latitude
///      (because it is on the opposite side of the origin), the error
///      SPICE(SHAPENOTSUPPORTED) is signaled.
///
///  12) If the radii are not available in the kernel pool, an error is
///      signaled by a routine in the call tree of this routine.
///
///  13) If the target shape is "ellipsoid" and not all radii of the
///      ellipsoid are strictly positive, the error
///      SPICE(BADAXISLENGTH) is signaled.
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
///  -  Shape data for the target body:
///
///        PCK data:
///
///           If the target shape is modeled as an ellipsoid,
///           triaxial radii for the target body must be loaded into
///           the kernel pool. Typically this is done by loading a
///           text PCK file via FURNSH.
///
///        DSK data:
///
///           If the target shape is modeled by DSK data, DSK files
///           containing topographic data for the target body must be
///           loaded. If a surface list is specified, data for at
///           least one of the listed surfaces must be loaded.
///
///  -  Target body orientation data: these may be provided in a
///     text or binary PCK file. In some cases, target body
///     orientation may be provided by one more more CK files. In
///     either case, data are made available by loading the files
///     via FURNSH.
///
///  The following data may be required:
///
///  -  Frame data: if a frame definition is required to convert
///     between the body-fixed frame of the target and the frame of
///     a DSK segment providing topographic data, that definition
///     must be available in the kernel pool. Typically the
///     definition is supplied by loading a frame kernel via FURNSH.
///
///  -  Surface name-ID associations: if surface names are specified
///     in METHOD, the association of these names with their
///     corresponding surface ID codes must be established by
///     assignments of the kernel variables
///
///        NAIF_SURFACE_NAME
///        NAIF_SURFACE_CODE
///        NAIF_SURFACE_BODY
///
///     Normally these associations are made by loading a text
///     kernel containing the necessary assignments. An example of
///     such a set of assignments is
///
///        NAIF_SURFACE_NAME += 'Mars MEGDR 128 PIXEL/DEG'
///        NAIF_SURFACE_CODE += 1
///        NAIF_SURFACE_BODY += 499
///
///  -  SCLK data: if the target body's orientation is provided by
///     CK files, an associated SCLK kernel must be loaded.
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is intended to be used for target body surfaces that
///  have a unique radius for each pair of planetocentric longitude
///  and latitude coordinates.
///
///  If the target surface is represented by topographic data, it is
///  possible for there to be multiple surface points at a given
///  planetocentric longitude and latitude. For example, this can
///  occur if the surface has features such as cliffs, caves, or
///  arches.
///
///  For more complex surfaces, the routine
///
///     DSKSXV {DSK, ray-surface intercept, vectorized}
///
///  may be more suitable. That routine works with rays having vertices
///  anywhere outside of the target body.
///
///
///  Planetocentric coordinates
///  ==========================
///
///  Planetocentric longitude and latitude are defined as follows:
///
///     Longitude of a point P is the angle between the prime meridian
///     and the meridian containing P. The direction of increasing
///     longitude is from the +X axis towards the +Y axis.
///
///     Latitude of a point P is the angle from the XY plane of the
///     ray from the origin through the point.
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
///        DSK/<surface list>/UNPRIORITIZED
///        DSK/UNPRIORITIZED/<surface list>
///        UNPRIORITIZED/<surface list>/DSK
///
///     The simplest form of the METHOD argument specifying use of
///     DSK data is one that lacks a surface list, for example:
///
///        'DSK/UNPRIORITIZED'
///
///     For applications in which all loaded DSK data for the target
///     body are for a single surface, and there are no competing
///     segments, the above string suffices. This is expected to be
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
///        'DSK/UNPRIORITIZED/SURFACES = "Mars MEGDR 64 PIXEL/DEG", 3'
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///
///  1) In the following example program, a DSK file containing a
///     type 2 segment is used to provide a plate model representation
///     of the surface of Phobos.
///
///     Find the surface points on a target body corresponding to a
///     given planetocentric longitude/latitude grid.
///
///
///     Example code begins here.
///
///
///           PROGRAM LATSRF_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///           DOUBLE PRECISION      RPD
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1   = '(1X,A,F11.6)' )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 79 )
///
///           INTEGER               MAXN
///           PARAMETER           ( MAXN   = 100 )
///
///           INTEGER               MTHLEN
///           PARAMETER           ( MTHLEN = 80 )
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    DSK
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(MTHLEN)    METHOD
///           CHARACTER*(LNSIZE)    OUTLIN
///           CHARACTER*(BDNMLN)    TARGET
///
///           DOUBLE PRECISION      DLAT
///           DOUBLE PRECISION      DLON
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      GRID   ( 2, MAXN )
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LAT0
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      LON0
///           DOUBLE PRECISION      SRFPTS ( 3, MAXN )
///           DOUBLE PRECISION      XLAT
///           DOUBLE PRECISION      XLON
///           DOUBLE PRECISION      XR
///
///           INTEGER               I
///           INTEGER               J
///           INTEGER               N
///           INTEGER               NLAT
///           INTEGER               NLON
///
///     C
///     C     Set target, reference frame, and epoch.
///     C
///           TARGET = 'PHOBOS'
///           FIXREF = 'IAU_PHOBOS'
///           ET     = 0.D0
///     C
///     C     Use DSK data to represent the surface.
///     C
///           METHOD = 'DSK/UNPRIORITIZED'
///     C
///     C     Set the grid dimensions.
///     C
///           NLON   = 3
///           NLAT   = 2
///     C
///     C     Derive evenly spaced grid separations and starting
///     C     values in the longitude and latitude dimensions.
///     C     Units are degrees.
///     C
///           LAT0 = 90.D0
///           LON0 =  0.D0
///
///           DLAT = 180.D0 / (NLAT + 1)
///           DLON = 360.D0 /  NLON
///     C
///     C     Prompt for the name of the DSK to read.
///     C
///           CALL PROMPT ( 'Enter DSK name    > ', DSK )
///     C
///     C     Load the DSK file.
///     C
///           CALL FURNSH ( DSK )
///     C
///     C     Now generate the grid points.  We generate
///     C     points along latitude bands, working from
///     C     north to south.  The latitude range is selected
///     C     to range from +30 to -30 degrees.  Longitude
///     C     ranges from 0 to 240 degrees.  The increment
///     C     is 90 degrees for latitude and 120 degrees for
///     C     longitude.
///     C
///           N = 0
///
///           DO I = 1, NLAT
///
///              LAT = RPD() * ( LAT0 - I*DLAT )
///
///              DO J = 1, NLON
///
///                 N   = N + 1
///                 LON = RPD() * ( LON0 + (J-1)*DLON )
///
///                 GRID(1,N) = LON
///                 GRID(2,N) = LAT
///
///              END DO
///
///           END DO
///     C
///     C     Find the surface points corresponding to the grid points.
///     C
///           CALL LATSRF ( METHOD, TARGET, ET,
///          .              FIXREF, N,      GRID, SRFPTS )
///     C
///     C     Print out the surface points in latitudinal
///     C     coordinates and compare the derived lon/lat values
///     C     to those of the input grid.
///     C
///           DO I = 1, N
///     C
///     C        Use RECRAD rather than RECLAT to produce
///     C        non-negative longitudes.
///     C
///              CALL RECRAD ( SRFPTS(1,I), XR, XLON, XLAT )
///
///              WRITE (*,*) ' '
///
///              OUTLIN = 'Intercept for grid point #:'
///              CALL REPMI ( OUTLIN, '#', I, OUTLIN )
///
///              WRITE(*,*)  OUTLIN
///              OUTLIN = '  Cartesian coordinates: (#, #, #)'
///
///              DO J = 1, 3
///                 CALL REPMF( OUTLIN, '#', SRFPTS(J,I),
///          .                  8,      'F', OUTLIN      )
///              END DO
///
///              WRITE (*,*) OUTLIN
///
///              WRITE (*,*)    '  Latitudinal Coordinates:'
///              WRITE (*,FMT1) '   Longitude (deg): ', XLON*DPR()
///              WRITE (*,FMT1) '   Latitude  (deg): ', XLAT*DPR()
///              WRITE (*,FMT1) '   Radius     (km): ', XR
///              WRITE (*,*)    ' '
///              WRITE (*,*)    '  Original Grid Coordinates:'
///              WRITE (*,FMT1) '   Longitude (deg): ', GRID(1,I)*DPR()
///              WRITE (*,FMT1) '   Latitude  (deg): ', GRID(2,I)*DPR()
///
///           END DO
///           WRITE (*,*) ' '
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos512.bds, the output
///     was:
///
///
///     Enter DSK name    > phobos512.bds
///
///      Intercept for grid point 1:
///        Cartesian coordinates: (9.5706817, 0.00000000, 5.5256356)
///        Latitudinal Coordinates:
///         Longitude (deg):    0.000000
///         Latitude  (deg):   30.000000
///         Radius     (km):   11.051271
///
///        Original Grid Coordinates:
///         Longitude (deg):    0.000000
///         Latitude  (deg):   30.000000
///
///      Intercept for grid point 2:
///        Cartesian coordinates: (-4.7586430, 8.2422114, 5.4948076)
///        Latitudinal Coordinates:
///         Longitude (deg):  120.000000
///         Latitude  (deg):   30.000000
///         Radius     (km):   10.989615
///
///        Original Grid Coordinates:
///         Longitude (deg):  120.000000
///         Latitude  (deg):   30.000000
///
///      Intercept for grid point 3:
///        Cartesian coordinates: (-4.5704268, -7.9162115, 5.2774743)
///        Latitudinal Coordinates:
///         Longitude (deg):  240.000000
///         Latitude  (deg):   30.000000
///         Radius     (km):   10.554949
///
///        Original Grid Coordinates:
///         Longitude (deg):  240.000000
///         Latitude  (deg):   30.000000
///
///      Intercept for grid point 4:
///        Cartesian coordinates: (10.959385, 0.00000000, -6.3274040)
///        Latitudinal Coordinates:
///         Longitude (deg):    0.000000
///         Latitude  (deg):  -30.000000
///         Radius     (km):   12.654808
///
///        Original Grid Coordinates:
///         Longitude (deg):    0.000000
///         Latitude  (deg):  -30.000000
///
///      Intercept for grid point 5:
///        Cartesian coordinates: (-4.8830077, 8.4576174, -5.6384116)
///        Latitudinal Coordinates:
///         Longitude (deg):  120.000000
///         Latitude  (deg):  -30.000000
///         Radius     (km):   11.276823
///
///        Original Grid Coordinates:
///         Longitude (deg):  120.000000
///         Latitude  (deg):  -30.000000
///
///      Intercept for grid point 6:
///        Cartesian coordinates: (-4.5322568, -7.8500991, -5.2333994)
///        Latitudinal Coordinates:
///         Longitude (deg):  240.000000
///         Latitude  (deg):  -30.000000
///         Radius     (km):   10.466799
///
///        Original Grid Coordinates:
///         Longitude (deg):  240.000000
///         Latitude  (deg):  -30.000000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that the origin of the body-fixed
///      reference frame associated with the target body is located in
///      the interior of that body.
///
///  2)  The results returned by this routine may not be meaningful
///      if the target surface has multiple surface points associated
///      with some (longitude, latitude) coordinates.
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (NJB) (JDR)
///
///         Bug fix: removed spurious blank from long error
///         message.
///
///         Edited the header to comply with NAIF standard. Modified
///         the grid dimensions and output format in the code example to
///         reduce the number of lines and their length in the solution.
///
/// -    SPICELIB Version 1.0.0, 21-FEB-2017 (NJB)
///
///         Original version 01-JUL-2016 (NJB)
/// ```
pub fn latsrf(
    ctx: &mut SpiceContext,
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    npts: i32,
    lonlat: &[[f64; 2]],
    srfpts: &mut [[f64; 3]],
) -> crate::Result<()> {
    LATSRF(
        method.as_bytes(),
        target.as_bytes(),
        et,
        fixref.as_bytes(),
        npts,
        lonlat.as_flattened(),
        srfpts.as_flattened_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LATSRF ( Latitudinal grid to surface points )
pub fn LATSRF(
    METHOD: &[u8],
    TARGET: &[u8],
    ET: f64,
    FIXREF: &[u8],
    NPTS: i32,
    LONLAT: &[f64],
    SRFPTS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LONLAT = DummyArray2D::new(LONLAT, 1..=2, 1..);
    let mut SRFPTS = DummyArrayMut2D::new(SRFPTS, 1..=3, 1..);
    let mut LMBTYP = [b' '; CVTLEN as usize];
    let mut SHPSTR = [b' '; SHPLEN as usize];
    let mut SUBTYP = [b' '; SUBLEN as usize];
    let mut TRMTYP = [b' '; TMTLEN as usize];
    let mut R: f64 = 0.0;
    let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut FIXFID: i32 = 0;
    let mut FXCENT: i32 = 0;
    let mut FXCLSS: i32 = 0;
    let mut FXCLID: i32 = 0;
    let mut N: i32 = 0;
    let mut TRGCDE: i32 = 0;
    let mut FND: bool = false;
    let mut SURFUP: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    // Saved target radius values.
    //

    //
    // Saved name/ID items.
    //
    //
    // Saved frame name/ID items.
    //

    //
    // Saved target radius values.
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(RNAME, ctx)?;

    if save.FIRST {
        //
        // Initialize local surface counter.
        //
        ZZCTRUIN(save.SRFCTR.as_slice_mut(), ctx);
        //
        // Initialize target, frame, and radius counters.
        //
        ZZCTRUIN(save.SVCTR1.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR2.as_slice_mut(), ctx);
        ZZCTRUIN(save.SVCTR3.as_slice_mut(), ctx);
    }

    //
    // Obtain integer code for the target.
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

    //
    // Determine the attributes of the frame designated by FIXREF.
    //
    ZZNAMFRM(
        save.SVCTR2.as_slice_mut(),
        &mut save.SVFREF,
        &mut save.SVFXFC,
        FIXREF,
        &mut FIXFID,
        ctx,
    )?;

    FRINFO(FIXFID, &mut FXCENT, &mut FXCLSS, &mut FXCLID, &mut FND, ctx)?;

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
    ZZSRFTRK(save.SRFCTR.as_slice_mut(), &mut SURFUP, ctx)?;

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
            &mut SUBTYP,
            &mut save.PRI,
            &mut save.NSURF,
            save.SRFLST.as_slice_mut(),
            &mut LMBTYP,
            &mut TRMTYP,
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

        //
        // There should be no subtype specification in the method
        // string.
        //
        if fstr::ne(&SUBTYP, b" ") {
            SETMSG(b"Spurious sub-observer point type <#> was present in the method string #. The sub-observer type is valid in the method strings for SUBPNT and SUBSLR, but is not applicable for LATSRF.", ctx);
            ERRCH(b"#", &SUBTYP, ctx);
            ERRCH(b"#", METHOD, ctx);
            SIGERR(b"SPICE(INVALIDMETHOD)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        fstr::assign(&mut save.PRVMTH, METHOD);
    }

    //
    // At this point, the first pass actions were successful.
    //
    save.FIRST = false;

    //
    // Check the target body shape.
    //
    if (save.SHAPE == ELLSHP) {
        if (TRGCDE != save.SVPRVT) {
            //
            // Reset counter to force lookup.
            //
            ZZCTRUIN(save.SVCTR3.as_slice_mut(), ctx);
        }
        //
        // Look up target radii using counter.
        //
        ZZBODVCD(
            TRGCDE,
            b"RADII",
            3,
            save.SVCTR3.as_slice_mut(),
            &mut N,
            save.SVRADI.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if (intrinsics::DMIN1(&[save.SVRADI[1], save.SVRADI[2], save.SVRADI[3]]) <= 0.0) {
            SETMSG(b"Body # radii should be positive but were # # #.", ctx);
            ERRCH(b"#", TARGET, ctx);
            ERRDP(b"#", save.SVRADI[1], ctx);
            ERRDP(b"#", save.SVRADI[2], ctx);
            ERRDP(b"#", save.SVRADI[3], ctx);
            SIGERR(b"SPICE(BADAXISLENGTH)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // The radii are valid. Update the previous target ID.
        //
        save.SVPRVT = TRGCDE;

        //
        // Generate surface points.
        //
        for I in 1..=NPTS {
            //
            // Let X be a point having norm 1 and located at the Ith input
            // longitude and latitude.
            //
            LATREC(1.0, LONLAT[[1, I]], LONLAT[[2, I]], X.as_slice_mut());
            //
            // Scale X to place it on the ellipsoid surface.
            //
            EDPNT(
                X.as_slice(),
                save.SVRADI[1],
                save.SVRADI[2],
                save.SVRADI[3],
                SRFPTS.subarray_mut([1, I]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }
    } else if (save.SHAPE == DSKSHP) {
        //
        // Initialize the DSK ray-surface intercept algorithm to use a
        // DSK model for the surface of the target body.
        //
        ZZSUDSKI(TRGCDE, save.NSURF, save.SRFLST.as_slice(), FIXFID, ctx)?;

        //
        // Get the radius of an outer bounding sphere for the body. Scale
        // up to avoid getting too close to the target surface.
        //
        ZZMAXRAD(&mut R, ctx);
        R = ((2 as f64) * R);

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Generate surface points.
        //
        for I in 1..=NPTS {
            LATREC(R, LONLAT[[1, I]], LONLAT[[2, I]], X.as_slice_mut());

            VMINUS(X.as_slice(), RAYDIR.as_slice_mut());
            //
            // Find the ray-surface intercept for the ray emanating
            // from X and pointing in the -X direction, where the
            // surface is represented by DSK data for the specified
            // body and surface list (the surface list was supplied
            // to ZZSUDSKI).
            //
            ZZRAYSFX(
                X.as_slice(),
                RAYDIR.as_slice(),
                ET,
                SRFPTS.subarray_mut([1, I]),
                &mut FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if !FND {
                SETMSG(b"No surface point was found on body # at planetocentric longitude # (# deg), latitude # (# deg). This problem may be due to insufficient DSK data having been loaded for the body. It also could be due to the body having a shape not suitable for this computation, for example, a torus.", ctx);
                ERRCH(b"#", TARGET, ctx);
                ERRDP(b"#", LONLAT[[1, I]], ctx);
                ERRDP(b"#", (LONLAT[[1, I]] * DPR(ctx)), ctx);
                ERRDP(b"#", LONLAT[[2, I]], ctx);
                ERRDP(b"#", (LONLAT[[2, I]] * DPR(ctx)), ctx);
                SIGERR(b"SPICE(POINTNOTFOUND)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Make sure the intercept is on the correct side of the
            // object.
            //
            if (VDOT(X.as_slice(), SRFPTS.subarray([1, I])) < 0.0) {
                SETMSG(b"A surface point was found on body # for the input planetocentric longitude # (# deg), latitude # (# deg), but this point is on the opposite side of the body. This likely indicates the the body does not contain the origin of the coordinate system. LATSRF does not work with such surfaces. Consider using DSKSXV for this computation.", ctx);
                ERRCH(b"#", TARGET, ctx);
                ERRDP(b"#", LONLAT[[1, I]], ctx);
                ERRDP(b"#", (LONLAT[[1, I]] * DPR(ctx)), ctx);
                ERRDP(b"#", LONLAT[[2, I]], ctx);
                ERRDP(b"#", (LONLAT[[2, I]] * DPR(ctx)), ctx);
                SIGERR(b"SPICE(SHAPENOTSUPPORTED)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }
        }
    } else {
        SETMSG(
            b"Input method <#> does not specify the target shape as either ELLIPSOID or DSK.",
            ctx,
        );
        ERRCH(b"#", METHOD, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
