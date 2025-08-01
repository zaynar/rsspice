//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);
const CTRSIZ: i32 = 2;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;

struct SaveVars {
    PRVFRM: Vec<u8>,
    SVTNAM: Vec<u8>,
    FIXFID: i32,
    FRMCTR: StackArray<i32, 2>,
    PRVTCD: i32,
    SVTCDE: i32,
    TRGCDE: i32,
    TRGCTR: StackArray<i32, 2>,
    FIRST: bool,
    SVTFND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PRVFRM = vec![b' '; FRNMLN as usize];
        let mut SVTNAM = vec![b' '; BDNMLN as usize];
        let mut FIXFID: i32 = 0;
        let mut FRMCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut PRVTCD: i32 = 0;
        let mut SVTCDE: i32 = 0;
        let mut TRGCDE: i32 = 0;
        let mut TRGCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut FIRST: bool = false;
        let mut SVTFND: bool = false;

        FIRST = true;
        fstr::assign(&mut PRVFRM, b" ");
        PRVTCD = 0;

        Self {
            PRVFRM,
            SVTNAM,
            FIXFID,
            FRMCTR,
            PRVTCD,
            SVTCDE,
            TRGCDE,
            TRGCTR,
            FIRST,
            SVTFND,
        }
    }
}

/// DSK, ray-surface intercept, vectorized
///
/// Compute ray-surface intercepts for a set of rays, using data
/// provided by multiple loaded DSK segments.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
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
///  PRI        I   Data prioritization flag.
///  TARGET     I   Target body name.
///  NSURF      I   Number of surface IDs in list.
///  SRFLST     I   Surface ID list.
///  ET         I   Epoch, expressed as seconds past J2000 TDB.
///  FIXREF     I   Name of target body-fixed reference frame.
///  NRAYS      I   Number of rays.
///  VTXARR     I   Array of vertices of rays.
///  DIRARR     I   Array of direction vectors of rays.
///  XPTARR     O   Intercept point array.
///  FNDARR     O   Found flag array.
/// ```
///
/// # Detailed Input
///
/// ```text
///  PRI      is a logical flag indicating whether to perform
///           a prioritized or unprioritized DSK segment search.
///           In an unprioritized search, no segment masks another:
///           data from all specified segments are used to
///           define the surface of interest.
///
///           The search is unprioritized if and only if PRI
///           is set to .FALSE. In the N0066 SPICE Toolkit, this
///           is the only allowed value.
///
///  TARGET   is the name of the target body on which a surface
///           intercept is sought.
///
///  NSURF,
///  SRFLST   are, respectively, a count of surface ID codes in a
///           list and the containing list. Only DSK segments for
///           the body designated by TARGET and having surface
///           IDs in this list will considered in the intercept
///           computation. If the list is empty, all DSK segments
///           for TARGET will be considered.
///
///  ET       is the epoch of the intersection computation,
///           expressed as seconds past J2000 TDB. This epoch is
///           used only for DSK segment selection. Segments used
///           the intercept computation must include ET in their
///           time coverage intervals.
///
///  FIXREF   is the name of a body-fixed, body-centered reference
///           frame associated with the target. The input ray vectors
///           are specified in this frame, as is the output intercept
///           point.
///
///           The frame designated by FIXREF must have a fixed
///           orientation relative to the frame of any DSK segment
///           used in the computation.
///
///  NRAYS,
///  VTXARR,
///  DIRARR   are, respectively, a count of rays, an array containing
///           the vertices of rays, and an array containing the
///           direction vectors of the rays.
///
///           The ray's vertices are considered to represent offsets
///           from the center of the target body.
///
///           The rays' vertices and direction vectors are
///           represented in the reference frame designated by
///           FIXREF.
/// ```
///
/// # Detailed Output
///
/// ```text
///  XPTARR   is an array containing the intercepts of the input
///           rays on the surface specified by the inputs
///
///              PRI
///              TARGET
///              NSURF
///              SRFLST
///              ET
///
///           The Ith element of XPTARR is the intercept
///           corresponding to the Ith ray, if such an intercept
///           exists. If a ray intersects the surface at multiple
///           points, the intercept closest to the ray's vertex is
///           selected.
///
///           The Ith element of XPTARR is defined if and only if the
///           Ith element of FNDARR is .TRUE.
///
///           Units are km.
///
///  FNDARR   is an array of logical flags indicating whether the
///           input rays intersect the surface. The Ith element of
///           FNDARR is set to .TRUE. if and only if an intercept
///           was found for the Ith ray.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file
///
///     dsktol.inc
///
///  for the values of tolerance parameters used by default by the
///  ray-surface intercept algorithm.
///
///  These parameters are discussed in the $Particulars section
///  below.
///
///  See the include file
///
///     dla.inc
///
///  for declarations of DLA descriptor sizes and documentation of the
///  contents of DLA descriptors.
///
///  See the include file
///
///     dskdsc.inc
///
///  for declarations of DSK descriptor sizes and documentation of the
///  contents of DSK descriptors.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input prioritization flag PRI is set to .TRUE.,
///      the error SPICE(BADPRIORITYSPEC) is signaled.
///
///  2)  If NRAYS is less than 1, the error SPICE(INVALIDCOUNT)
///      is signaled.
///
///  3)  If NSURF is less than 0, the error SPICE(INVALIDCOUNT)
///      is signaled.
///
///  4)  If the input body name TARGET cannot be mapped to an
///      ID code, the error SPICE(IDCODENOTFOUND) is signaled.
///
///  5)  If the input frame name FIXREF cannot be mapped to an
///      ID code, the error SPICE(IDCODENOTFOUND) is signaled.
///
///  6)  If the frame center associated with FIXREF cannot be
///      retrieved, the error SPICE(NOFRAMEINFO) is signaled.
///
///  7)  If the frame center associated with FIXREF is not
///      the target body, the error SPICE(INVALIDFRAME) is signaled.
///
///  8)  If an error occurs during the intercept computation, the error
///      is signaled by a routine in the call tree of this routine.
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
///  -  SPK data: ephemeris data for the positions of the centers
///     of DSK reference frames relative to the target body are
///     required if those frames are not centered at the target
///     body center.
///
///     Typically ephemeris data are made available by loading one
///     or more SPK files via FURNSH.
///
///  -  DSK data: DSK files containing topographic data for the
///     target body must be loaded. If a surface list is specified,
///     data for at least one of the listed surfaces must be loaded.
///
///  -  Frame data: if a frame definition is required to convert
///     DSK segment data to the body-fixed frame designated by
///     FIXREF, the target, that definition must be available in the
///     kernel pool. Typically the definitions of frames not already
///     built-in to SPICE are supplied by loading a frame kernel.
///
///  -  CK data: if the frame to which FIXREF refers is a CK frame,
///     and if any DSK segments used in the computation have a
///     different frame, at least one CK file will be needed to
///     permit transformation of vectors between that frame and both
///     the J2000 and the target body-fixed frames.
///
///  -  SCLK data: if a CK file is needed, an associated SCLK
///     kernel is required to enable conversion between encoded SCLK
///     (used to time-tag CK data) and barycentric dynamical time
///     (TDB).
///
///  In all cases, kernel data are normally loaded once per program
///  run, NOT every time this routine is called.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is suitable for efficient ray-surface intercept
///  computations in which the relative observer-target geometry is
///  constant but the rays vary.
///
///  For cases in which it is necessary to know the source of the
///  data defining the surface on which an intercept was found,
///  use the SPICELIB routine DSKXSI.
///
///  For cases in which a ray's vertex is not explicitly known but is
///  defined by relative observer-target geometry, the SPICELIB
///  ray-surface intercept routine SINCPT should be used.
///
///  This routine works with multiple DSK files. It places no
///  restrictions on the data types or coordinate systems of the DSK
///  segments used in the computation. DSK segments using different
///  reference frames may be used in a single computation. The only
///  restriction is that any pair of reference frames used directly or
///  indirectly are related by a constant rotation.
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
///     in a later version of the routine, the presence of the PRI
///     argument is required.
///
///
///     Round-off errors and mitigating algorithms
///     ------------------------------------------
///
///     When topographic data are used to represent the surface of a
///     target body, round-off errors can produce some results that
///     may seem surprising.
///
///     Note that, since the surface in question might have mountains,
///     valleys, and cliffs, the points of intersection found for
///     nearly identical sets of inputs may be quite far apart from
///     each other: for example, a ray that hits a mountain side in a
///     nearly tangent fashion may, on a different host computer, be
///     found to miss the mountain and hit a valley floor much farther
///     from the observer, or even miss the target altogether.
///
///     Round-off errors can affect segment selection: for example, a
///     ray that is expected to intersect the target body's surface
///     near the boundary between two segments might hit either
///     segment, or neither of them; the result may be
///     platform-dependent.
///
///     A similar situation exists when a surface is modeled by a set
///     of triangular plates, and the ray is expected to intersect the
///     surface near a plate boundary.
///
///     To avoid having the routine fail to find an intersection when
///     one clearly should exist, this routine uses two "greedy"
///     algorithms:
///
///        1) If the ray passes sufficiently close to any of the
///           boundary surfaces of a segment (for example, surfaces of
///           maximum and minimum longitude or latitude), that segment
///           is tested for an intersection of the ray with the
///           surface represented by the segment's data.
///
///           This choice prevents all of the segments from being
///           missed when at least one should be hit, but it could, on
///           rare occasions, cause an intersection to be found in a
///           segment other than the one that would be found if higher
///           precision arithmetic were used.
///
///        2) For type 2 segments, which represent surfaces as
///           sets of triangular plates, each plate is expanded very
///           slightly before a ray-plate intersection test is
///           performed. The default plate expansion factor is
///
///              1 + XFRACT
///
///           where XFRACT is declared in
///
///              dsktol.inc
///
///           For example, given a value for XFRACT of 1.e-10, the
///           sides of the plate are lengthened by 1/10 of a micron
///           per km. The expansion keeps the centroid of the plate
///           fixed.
///
///           Plate expansion prevents all plates from being missed
///           in cases where clearly at least one should be hit.
///
///           As with the greedy segment selection algorithm, plate
///           expansion can occasionally cause an intercept to be
///           found on a different plate than would be found if higher
///           precision arithmetic were used. It also can occasionally
///           cause an intersection to be found when the ray misses
///           the target by a very small distance.
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
///  1) Compute surface intercepts of rays emanating from a set of
///     vertices distributed on a longitude-latitude grid. All
///     vertices are outside the target body, and all rays point
///     toward the target's center.
///
///     Check intercepts against expected values. Indicate the
///     number of errors, the number of computations, and the
///     number of intercepts found.
///
///
///     Use the meta-kernel shown below to load example SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: dskxv_ex1.tm
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
///           phobos512.bds                    DSK based on
///                                            Gaskell ICQ Q=512
///                                            plate model
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'phobos512.bds' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKXV_EX1
///           IMPLICIT NONE
///
///     C
///     C     Multi-segment, vectorized spear program.
///     C
///     C     This program expects all loaded DSKs
///     C     to represent the same body and surface.
///     C
///           INCLUDE 'dla.inc'
///           INCLUDE 'dsk.inc'
///           INCLUDE 'dskdsc.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      RPD
///           DOUBLE PRECISION      VDIST
///
///     C
///     C     Local parameters
///     C
///           DOUBLE PRECISION      DTOL
///           PARAMETER           ( DTOL   = 1.D-14 )
///
///           INTEGER               BDNMLN
///           PARAMETER           ( BDNMLN = 36 )
///
///           INTEGER               CMDLEN
///           PARAMETER           ( CMDLEN = 1000 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               MAXN
///           PARAMETER           ( MAXN   = 100000 )
///
///           INTEGER               TYPLEN
///           PARAMETER           ( TYPLEN = 4 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CMDLEN)    CMD
///           CHARACTER*(FILSIZ)    DSK1
///           CHARACTER*(TYPLEN)    FILTYP
///           CHARACTER*(FRNMLN)    FIXREF
///           CHARACTER*(FILSIZ)    FNAME
///           CHARACTER*(LNSIZE)    IDCH
///           CHARACTER*(FILSIZ)    SOURCE
///           CHARACTER*(BDNMLN)    TARGET
///
///           DOUBLE PRECISION      D
///           DOUBLE PRECISION      DSKDSC ( DSKDSZ )
///           DOUBLE PRECISION      DIRARR ( 3, MAXN )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LATCRD ( 3 )
///           DOUBLE PRECISION      LATSTP
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      LONSTP
///           DOUBLE PRECISION      POLMRG
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      RADIUS
///           DOUBLE PRECISION      VLAT
///           DOUBLE PRECISION      VLON
///           DOUBLE PRECISION      VRAD
///           DOUBLE PRECISION      VTXARR ( 3, MAXN )
///           DOUBLE PRECISION      XPTARR ( 3, MAXN )
///           DOUBLE PRECISION      XYZHIT ( 3 )
///
///           INTEGER               BODYID
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               FRAMID
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               NDERR
///           INTEGER               NHITS
///           INTEGER               NLSTEP
///           INTEGER               NRAYS
///           INTEGER               NSURF
///           INTEGER               SRFLST ( MAXSRF )
///           INTEGER               SURFID
///
///           LOGICAL               FNDARR ( MAXN )
///           LOGICAL               FOUND
///
///     C
///     C     Saved variables
///     C
///     C     Save large arrays to avoid stack problems.
///     C
///           SAVE                  DIRARR
///           SAVE                  FNDARR
///           SAVE                  XPTARR
///
///
///           CALL CHKIN ( 'SPEAR' )
///
///     C
///     C     Prompt for the name of the meta-kernel.
///     C
///           CALL PROMPT ( 'Enter meta-kernel name >  ', FNAME )
///
///     C
///     C     Load the meta-kernel.
///     C
///           CALL FURNSH ( FNAME )
///
///     C
///     C     Get a handle for one of the loaded DSKs,
///     C     then find the first segment and extract
///     C     the body and surface IDs.
///     C
///           CALL KDATA  ( 1,      'DSK',  DSK1, FILTYP,
///          .              SOURCE, HANDLE, FOUND )
///
///           CALL DLABFS ( HANDLE, DLADSC, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///              CALL SIGERR ( 'SPICE(NOSEGMENT)' )
///           END IF
///
///           CALL DSKGD ( HANDLE, DLADSC, DSKDSC )
///
///           BODYID = NINT( DSKDSC(CTRIDX) )
///           SURFID = NINT( DSKDSC(SRFIDX) )
///           FRAMID = NINT( DSKDSC(FRMIDX) )
///
///           CALL BODC2N ( BODYID, TARGET, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///              CALL SETMSG ( 'Cannot map body ID # to a name.' )
///              CALL ERRINT ( '#',  BODYID                      )
///              CALL SIGERR ( 'SPICE(BODYNAMENOTFOUND)'         )
///           END IF
///
///           CALL FRMNAM ( FRAMID, FIXREF )
///
///           IF ( FIXREF .EQ. ' ' ) THEN
///              CALL SETMSG ( 'Cannot map frame ID # to a name.' )
///              CALL ERRINT ( '#',  FRAMID                       )
///              CALL SIGERR ( 'SPICE(FRAMENAMENOTFOUND)'         )
///           END IF
///
///     C
///     C     Set the magnitude of the ray vertices. Use a large
///     C     number to to ensure the vertices are outside of
///     C     any realistic target.
///     C
///           R = 1.D10
///
///     C
///     C     Spear the target with rays pointing toward
///     C     the origin.  Use a grid of ray vertices
///     C     located on a sphere enclosing the target.
///     C
///     C     The variable POLMRG ("pole margin") can
///     C     be set to a small positive value to reduce
///     C     the number of intercepts done at the poles.
///     C     This may speed up the computation for
///     C     the multi-segment case, since rays parallel
///     C     to the Z axis will cause all segments converging
///     C     at the pole of interest to be tested for an
///     C     intersection.
///     C
///           POLMRG = 5.D-1
///           LATSTP = 1.D0
///           LONSTP = 2.D0
///
///           NHITS  = 0
///           NDERR  = 0
///
///           LON    = -180.D0
///           LAT    = 90.D0
///           NLSTEP = 0
///           NRAYS  = 0
///
///     C
///     C     Set the epoch for interval selection.
///     C
///           ET     = 0.D0
///
///     C
///     C     Generate rays.
///     C
///           DO WHILE ( LON .LT. 180.D0 )
///
///              DO WHILE ( NLSTEP .LE. 180  )
///
///                 IF ( LON .EQ. -180.D0 ) THEN
///
///                    LAT = 90.D0 - NLSTEP*LATSTP
///
///                 ELSE
///
///                    IF ( NLSTEP .EQ. 0 ) THEN
///
///                       LAT =  90.D0 - POLMRG
///
///                    ELSE IF ( NLSTEP .EQ. 180 ) THEN
///
///                       LAT = -90.D0 + POLMRG
///
///                    ELSE
///
///                       LAT =  90.D0 - NLSTEP*LATSTP
///
///                    END IF
///
///                 END IF
///
///                 NRAYS  = NRAYS  + 1
///
///                 CALL LATREC ( R,               LON*RPD(),
///          .                    LAT*RPD(),       VTXARR(1,NRAYS) )
///                 CALL VMINUS ( VTXARR(1,NRAYS), DIRARR(1,NRAYS) )
///
///                 NLSTEP = NLSTEP + 1
///
///              END DO
///
///              LON    = LON + LONSTP
///              LAT    = 90.D0
///              NLSTEP = 0
///
///           END DO
///
///     C
///     C     Assign surface ID list.
///     C
///     C     Note that, if we knew that all files had the desired
///     C     surface ID, we could set `nsurf' to 0 and omit the
///     C     initialization of the surface ID list.
///     C
///           NSURF     = 1
///           SRFLST(1) = SURFID
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Computing intercepts...'
///
///           CALL DSKXV ( .FALSE., TARGET, NSURF, SRFLST,
///          .             ET,      FIXREF, NRAYS, VTXARR,
///          .             DIRARR,  XPTARR, FNDARR        )
///
///           WRITE (*,*) 'Done.'
///           WRITE (*,*) ' '
///
///     C
///     C     Check results.
///     C
///           DO I = 1, NRAYS
///
///              IF ( FNDARR(I) ) THEN
///     C
///     C           Record that a new intercept was found.
///     C
///                 NHITS = NHITS + 1
///     C
///     C           Compute the latitude and longitude of
///     C           the intercept. Make sure these agree
///     C           well with those of the vertex.
///     C
///                 CALL RECLAT ( XPTARR(1,I), LATCRD(1),
///          .                    LATCRD(2),   LATCRD(3) )
///
///                 RADIUS = LATCRD(1)
///
///     C
///     C           Recover the vertex longitude and latitude.
///     C
///                 CALL RECLAT ( VTXARR(1,I), VRAD, VLON, VLAT )
///
///                 CALL LATREC ( RADIUS,  VLON,
///          .                    VLAT,    XYZHIT )
///
///                 D = VDIST ( XPTARR(1,I), XYZHIT )
///
///                 IF ( D/R .GT. DTOL ) THEN
///     C
///     C              Get the intercept segment's plate ID if
///     C              applicable.
///     C
///                    WRITE (*,*) '======================'
///                    WRITE (*,*) 'LON, LAT       = ', LON, LAT
///                    WRITE (*,*) 'Bad intercept'
///                    WRITE (*,*) 'Distance error = ', D
///                    WRITE (*,*) 'XPT            = ',
///          .                     ( XPTARR(J,I), J = 1, 3 )
///                    WRITE (*,*) 'XYZHIT         = ', XYZHIT
///
///                    NDERR = NDERR + 1
///
///                 END IF
///
///              ELSE
///     C
///     C           Missing the target entirely is a fatal error.
///     C
///     C           This is true only for this program, not in
///     C           general. For example, if the target shape is
///     C           a torus, many rays would miss the target.
///     C
///                 WRITE (*,*) '======================'
///                 WRITE (*,*) 'LON, LAT = ', LON, LAT
///                 WRITE (*,*) 'No intercept'
///                 WRITE (*,*) 'I        = ', I
///                 STOP
///
///              END IF
///
///           END DO
///
///           WRITE (*,*) 'NRAYS  = ', NRAYS
///           WRITE (*,*) 'NHITS  = ', NHITS
///           WRITE (*,*) 'NDERR  = ', NDERR
///           WRITE (*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using as input the meta-kernel dskxv_ex1.tm, the
///     output was:
///
///
///     Enter meta-kernel name >  dskxv_ex1.tm
///
///      Computing intercepts...
///      Done.
///
///      NRAYS  =        32580
///      NHITS  =        32580
///      NDERR  =            0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The frame designated by FIXREF must have a fixed
///      orientation relative to the frame of any DSK segment
///      used in the computation. This routine has no
///      practical way of ensuring that this condition is met;
///      so this responsibility is delegated to the calling
///      application.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 06-AUG-2021 (JDR) (BVS)
///
///         Edited the header to comply with NAIF standard.
///
///         Updated code example to prompt for input meta-kernel name and
///         set input time to zero.
///
/// -    SPICELIB Version 1.0.0, 21-FEB-2017 (NJB)
///
///         Original 25-FEB-2016 (NJB)
/// ```
pub fn dskxv(
    ctx: &mut SpiceContext,
    pri: bool,
    target: &str,
    nsurf: i32,
    srflst: &[i32],
    et: f64,
    fixref: &str,
    nrays: i32,
    vtxarr: &[[f64; 3]],
    dirarr: &[[f64; 3]],
    xptarr: &mut [[f64; 3]],
    fndarr: &mut [bool],
) -> crate::Result<()> {
    DSKXV(
        pri,
        target.as_bytes(),
        nsurf,
        srflst,
        et,
        fixref.as_bytes(),
        nrays,
        vtxarr.as_flattened(),
        dirarr.as_flattened(),
        xptarr.as_flattened_mut(),
        fndarr,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKXV ( DSK, ray-surface intercept, vectorized )
pub fn DSKXV(
    PRI: bool,
    TARGET: &[u8],
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    FIXREF: &[u8],
    NRAYS: i32,
    VTXARR: &[f64],
    DIRARR: &[f64],
    XPTARR: &mut [f64],
    FNDARR: &mut [bool],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);
    let VTXARR = DummyArray2D::new(VTXARR, 1..=3, 1..);
    let DIRARR = DummyArray2D::new(DIRARR, 1..=3, 1..);
    let mut XPTARR = DummyArrayMut2D::new(XPTARR, 1..=3, 1..);
    let mut FNDARR = DummyArrayMut::new(FNDARR, 1..);
    let mut FXCENT: i32 = 0;
    let mut FXCLSS: i32 = 0;
    let mut FXTPID: i32 = 0;
    let mut FRMFND: bool = false;
    let mut NEWFRM: bool = false;
    let mut NEWTRG: bool = false;
    let mut TRGFND: bool = false;
    let mut UPDATE: bool = false;

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
    // Saved variables
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKXV", ctx)?;

    if save.FIRST {
        //
        // Initialize counters.
        //
        ZZCTRUIN(save.TRGCTR.as_slice_mut(), ctx);
        ZZCTRUIN(save.FRMCTR.as_slice_mut(), ctx);

        if FAILED(ctx) {
            CHKOUT(b"DSKXV", ctx)?;
            return Ok(());
        }
    }

    //
    // Reject PRI if not set properly.
    //
    if PRI {
        SETMSG(b"In the N0066 SPICE Toolkit, PRI must be set to .FALSE., indicating that an unprioritized search is to be performed.", ctx);
        SIGERR(b"SPICE(BADPRIORITYSPEC)", ctx)?;
        CHKOUT(b"DSKXV", ctx)?;
        return Ok(());
    }

    //
    // Reject NRAYS if not set properly.
    //
    if (NRAYS < 1) {
        SETMSG(b"The ray count NRAYS must be at least 1 but was #.", ctx);
        ERRINT(b"#", NRAYS, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"DSKXV", ctx)?;
        return Ok(());
    }

    //
    // Reject NSURF if not set properly. Zero is a valid value.
    //
    if (NSURF < 0) {
        SETMSG(
            b"The surface count NSURF must be non-negative but was #.",
            ctx,
        );
        ERRINT(b"#", NSURF, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"DSKXV", ctx)?;
        return Ok(());
    }

    //
    // Obtain integer codes for the target and reference frame.
    //
    ZZBODS2C(
        save.TRGCTR.as_slice_mut(),
        &mut save.SVTNAM,
        &mut save.SVTCDE,
        &mut save.SVTFND,
        TARGET,
        &mut save.TRGCDE,
        &mut TRGFND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DSKXV", ctx)?;
        return Ok(());
    }

    if !TRGFND {
        SETMSG(b"The target, \'#\', is not a recognized name for an ephemeris object. The cause of this problem may be that you need an updated version of the SPICE Toolkit, or that you failed to load a kernel containing a name-ID mapping for this body.", ctx);
        ERRCH(b"#", TARGET, ctx);
        SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
        CHKOUT(b"DSKXV", ctx)?;
        return Ok(());
    }

    NEWFRM = (fstr::ne(FIXREF, &save.PRVFRM) || save.FIRST);
    NEWTRG = ((save.TRGCDE != save.PRVTCD) || save.FIRST);

    //
    // Get the frame ID if the pool state has changed. The
    // first call to ZZPCKTRCK will indicate an update.
    //
    ZZPCTRCK(save.FRMCTR.as_slice_mut(), &mut UPDATE, ctx);

    if ((UPDATE || NEWFRM) || NEWTRG) {
        NAMFRM(FIXREF, &mut save.FIXFID, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DSKXV", ctx)?;
            return Ok(());
        }

        if (save.FIXFID == 0) {
            SETMSG(b"Reference frame # is not recognized by the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
            ERRCH(b"#", FIXREF, ctx);
            SIGERR(b"SPICE(IDCODENOTFOUND)", ctx)?;
            CHKOUT(b"DSKXV", ctx)?;
            return Ok(());
        }

        //
        // Determine the attributes of the frame designated by FIXREF.
        //
        FRINFO(
            save.FIXFID,
            &mut FXCENT,
            &mut FXCLSS,
            &mut FXTPID,
            &mut FRMFND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"DSKXV", ctx)?;
            return Ok(());
        }

        if !FRMFND {
            SETMSG(b"Attributes for reference frame # could not be obtained from the SPICE frame subsystem. Possibly a required frame definition kernel has not been loaded.", ctx);
            ERRCH(b"#", FIXREF, ctx);
            SIGERR(b"SPICE(NOFRAMEINFO)", ctx)?;
            CHKOUT(b"DSKXV", ctx)?;
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
            CHKOUT(b"DSKXV", ctx)?;
            return Ok(());
        }

        //
        // We have a valid frame at this point. Save the name.
        //
        save.FIRST = false;
        fstr::assign(&mut save.PRVFRM, FIXREF);

        //
        // Update the previous target ID code as well.
        //
        save.PRVTCD = save.TRGCDE;
    }

    //
    // TRGCDE and FIXFID are set.
    //
    //
    // Perform the intercept computations.
    //
    for I in 1..=NRAYS {
        ZZSBFXR(
            save.TRGCDE,
            NSURF,
            SRFLST.as_slice(),
            ET,
            save.FIXFID,
            VTXARR.subarray([1, I]),
            DIRARR.subarray([1, I]),
            XPTARR.subarray_mut([1, I]),
            &mut FNDARR[I],
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"DSKXV", ctx)?;
            return Ok(());
        }
    }

    CHKOUT(b"DSKXV", ctx)?;
    Ok(())
}
