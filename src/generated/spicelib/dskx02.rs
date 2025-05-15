//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
const IXNV: i32 = 1;
const IXNP: i32 = (IXNV + 1);
const IXNVXT: i32 = (IXNP + 1);
const IXVGRX: i32 = (IXNVXT + 1);
const IXCGSC: i32 = (IXVGRX + 3);
const IXVXPS: i32 = (IXCGSC + 1);
const IXVXLS: i32 = (IXVXPS + 1);
const IXVTLS: i32 = (IXVXLS + 1);
const IXPLAT: i32 = (IXVTLS + 1);
const IXDSCR: i32 = 1;
const DSCSZ2: i32 = 24;
const IXVTBD: i32 = (IXDSCR + DSCSZ2);
const IXVXOR: i32 = (IXVTBD + 6);
const IXVXSZ: i32 = (IXVXOR + 3);
const IXVERT: i32 = (IXVXSZ + 1);
const KWNV: i32 = 1;
const KWNP: i32 = (KWNV + 1);
const KWNVXT: i32 = (KWNP + 1);
const KWVGRX: i32 = (KWNVXT + 1);
const KWCGSC: i32 = (KWVGRX + 1);
const KWVXPS: i32 = (KWCGSC + 1);
const KWVXLS: i32 = (KWVXPS + 1);
const KWVTLS: i32 = (KWVXLS + 1);
const KWPLAT: i32 = (KWVTLS + 1);
const KWVXPT: i32 = (KWPLAT + 1);
const KWVXPL: i32 = (KWVXPT + 1);
const KWVTPT: i32 = (KWVXPL + 1);
const KWVTPL: i32 = (KWVTPT + 1);
const KWCGPT: i32 = (KWVTPL + 1);
const KWDSC: i32 = (KWCGPT + 1);
const KWVTBD: i32 = (KWDSC + 1);
const KWVXOR: i32 = (KWVTBD + 1);
const KWVXSZ: i32 = (KWVXOR + 1);
const KWVERT: i32 = (KWVXSZ + 1);
const MAXVRT: i32 = 16000002;
const MAXPLT: i32 = (2 * (MAXVRT - 2));
const MAXNPV: i32 = (((3 * MAXPLT) / 2) + 1);
const MAXVOX: i32 = 100000000;
const MAXCGR: i32 = 100000;
const MAXEDG: i32 = 120;
const SIVGRX: i32 = 1;
const SICGSC: i32 = (SIVGRX + 3);
const SIVXNP: i32 = (SICGSC + 1);
const SIVXNL: i32 = (SIVXNP + 1);
const SIVTNL: i32 = (SIVXNL + 1);
const SICGRD: i32 = (SIVTNL + 1);
const IXIFIX: i32 = (MAXCGR + 7);
const SIVTBD: i32 = 1;
const SIVXOR: i32 = (SIVTBD + 6);
const SIVXSZ: i32 = (SIVXOR + 3);
const IXDFIX: i32 = 10;
const MAXVXP: i32 = (MAXPLT / 2);
const MAXCEL: i32 = 60000000;
const MXNVLS: i32 = (MAXCEL + (MAXVXP / 2));
const SPAISZ: i32 = ((((IXIFIX + MAXVXP) + MXNVLS) + MAXVRT) + MAXNPV);
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
const VTXTOL: f64 = 0.000000000001;
const NVXLST: i32 = 50000;
const MAXPL: i32 = (MAXPLT / 125);
const NONE: i32 = 0;
const BUFSIZ: i32 = 200;

struct SaveVars {
    DSKDSC: StackArray<f64, 24>,
    GRDEXT: StackArray<f64, 3>,
    GRDTOL: f64,
    VBUFF: ActualArray2D<f64>,
    VOXORI: StackArray<f64, 3>,
    VOXSIZ: f64,
    VTXBDS: StackArray2D<f64, 6>,
    XTOL: f64,
    CGREXT: StackArray<i32, 3>,
    CGRPTR: ActualArray<i32>,
    CGSCAL: i32,
    CGSCL2: i32,
    CORSYS: i32,
    NCGR: i32,
    NP: i32,
    NV: i32,
    NVXTOT: i32,
    ORDVEC: ActualArray<i32>,
    PLATID: ActualArray<i32>,
    PRVDSC: StackArray<i32, 8>,
    PRVHAN: i32,
    SOURCE: ActualArray<i32>,
    VGREXT: StackArray<i32, 3>,
    VIDXS: StackArray<i32, 200>,
    VOXLST: ActualArray2D<i32>,
    VOXNPL: i32,
    VOXNPT: i32,
    VTXNPL: i32,
    VXLCG: ActualArray2D<i32>,
    VXLOUT: ActualArray<i32>,
    VXLSTR: ActualArray<i32>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
        let mut GRDEXT = StackArray::<f64, 3>::new(1..=3);
        let mut GRDTOL: f64 = 0.0;
        let mut VBUFF = ActualArray2D::<f64>::new(1..=3, 1..=BUFSIZ);
        let mut VOXORI = StackArray::<f64, 3>::new(1..=3);
        let mut VOXSIZ: f64 = 0.0;
        let mut VTXBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
        let mut XTOL: f64 = 0.0;
        let mut CGREXT = StackArray::<i32, 3>::new(1..=3);
        let mut CGRPTR = ActualArray::<i32>::new(1..=MAXCGR);
        let mut CGSCAL: i32 = 0;
        let mut CGSCL2: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut NCGR: i32 = 0;
        let mut NP: i32 = 0;
        let mut NV: i32 = 0;
        let mut NVXTOT: i32 = 0;
        let mut ORDVEC = ActualArray::<i32>::new(1..=MAXPL);
        let mut PLATID = ActualArray::<i32>::new(1..=MAXPL);
        let mut PRVDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut PRVHAN: i32 = 0;
        let mut SOURCE = ActualArray::<i32>::new(1..=MAXPL);
        let mut VGREXT = StackArray::<i32, 3>::new(1..=3);
        let mut VIDXS = StackArray::<i32, 200>::new(1..=BUFSIZ);
        let mut VOXLST = ActualArray2D::<i32>::new(1..=3, 1..=NVXLST);
        let mut VOXNPL: i32 = 0;
        let mut VOXNPT: i32 = 0;
        let mut VTXNPL: i32 = 0;
        let mut VXLCG = ActualArray2D::<i32>::new(1..=3, 1..=NVXLST);
        let mut VXLOUT = ActualArray::<i32>::new(1..=NVXLST);
        let mut VXLSTR = ActualArray::<i32>::new(1..=NVXLST);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(-1.0), 3 as usize))
                .chain([]);

            GRDEXT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PRVHAN = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), DLADSZ as usize))
                .chain([]);

            PRVDSC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            DSKDSC,
            GRDEXT,
            GRDTOL,
            VBUFF,
            VOXORI,
            VOXSIZ,
            VTXBDS,
            XTOL,
            CGREXT,
            CGRPTR,
            CGSCAL,
            CGSCL2,
            CORSYS,
            NCGR,
            NP,
            NV,
            NVXTOT,
            ORDVEC,
            PLATID,
            PRVDSC,
            PRVHAN,
            SOURCE,
            VGREXT,
            VIDXS,
            VOXLST,
            VOXNPL,
            VOXNPT,
            VTXNPL,
            VXLCG,
            VXLOUT,
            VXLSTR,
        }
    }
}

fn VOX2ID(I1: i32, I2: i32, I3: i32, DIM1: i32, DIM2: i32) -> i32 {
    (I1 + (DIM1 * ((I2 + (I3 * DIM2)) - (1 + DIM2))))
}

/// DSK, ray-surface intercept, type 2
///
/// Determine the plate ID and body-fixed coordinates of the
/// intersection of a specified ray with the surface defined by a
/// type 2 DSK plate model.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DSK kernel containing plate model.
///  DLADSC     I   DLA descriptor of plate model segment.
///  VERTEX     I   Ray's vertex in the  body fixed frame.
///  RAYDIR     I   Ray direction in the body fixed frame.
///  PLID       O   ID code of the plate intersected by the ray.
///  XPT        O   Intercept.
///  FOUND      O   Flag indicating whether intercept exists.
///  XFRACT     P   Plate expansion fraction.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a DSK file containing a shape
///           model for a target body. The shape model is stored
///           in a type 2 DSK segment.
///
///  DLADSC   is the DLA descriptor of a type 2 DSK segment
///           containing plate model data representing the surface of
///           the target body. The caller should declare DLADSC
///           with size DLADSZ; this size parameter is defined in
///           the INCLUDE file dla.inc. Normally this descriptor
///           will be obtained by a search through a DSK file
///           using the DLA search routines; see the $Examples
///           header section below for a working code example
///           illustrating a simple search.
///
///  VERTEX   is the vertex of a ray. VERTEX is expressed relative
///           to the body fixed reference frame associated with the
///           target body. This reference frame is the same frame
///           relative to which the vertices of the plate model
///           are expressed. Units are km.
///
///           The vertex is required to be outside the target
///           body.
///
///  RAYDIR   is the ray's direction vector. RAYDIR is expressed
///           relative to the body fixed reference frame associated
///           with the target body.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PLID     is the ID of the plate closest to the input ray's
///           vertex at which a ray-surface intercept exists.
///           If no intercept exists, PLID is undefined.
///
///  XPT      is the ray-target intercept closest to the ray's
///           vertex, if an intercept exists. XPT is expressed
///           relative to the body-fixed reference frame associated
///           with the target body. Units are km.
///
///           If no intercept exists, XPT is undefined.
///
///  FOUND    is a logical flag that indicates whether or not the
///           ray does indeed intersect the target. If the ray
///           intersects a plate FOUND is .TRUE. Otherwise FOUND is
///           .FALSE.
/// ```
///
/// # Parameters
///
/// ```text
///  XFRACT   is the default plate expansion fraction. This
///           parameter can be overridden.
///
///  See the include file
///
///     dsktol.inc
///
///  for the values of tolerance parameters used by default by the
///  ray-surface intercept algorithm.
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
///
///  See the include file
///
///     dsk02.inc
///
///  for declarations of DSK data type 2 (plate model) parameters.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input handle is invalid, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If a file read error occurs, the error is signaled by a
///      routine in the call tree of this routine.
///
///  3)  If the input DLA descriptor is invalid, the effect of this
///      routine is undefined. The error *may* be diagnosed by
///      routines in the call tree of this routine, but there are no
///      guarantees.
///
///  4)  If an error occurs while trying to look up any component
///      of the shape model, the error is signaled by a routine in the
///      call tree of this routine.
///
///  5)  If the input ray direction is the zero vector, the error
///      SPICE(ZEROVECTOR) is signaled.
///
///  6)  If the coarse voxel grid scale of the shape model is less
///      than 1, the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  7)  If the coarse voxel grid of the shape model contains more
///      than MAXCGR (see dsk02.inc) voxels, the error
///      SPICE(GRIDTOOLARGE) is signaled.
///
///  8)  If the plate list for any intersected voxel is too large
///      for this routine to buffer, the error SPICE(ARRAYTOOSMALL)
///      is signaled.
///
///  9)  Due to round-off errors, results from this routine may
///      differ across platforms. Results also may differ from
///      those expected---and not necessarily by a small amount.
///      For example, a ray may miss a plate it was expected to
///      hit and instead hit another plate considerably farther
///      from the ray's vertex, or miss the target entirely.
///
///  10) In the event that an intercept point lies on multiple
///      plates (that is, the point is on an edge or vertex),
///      a plate will be selected. Due to round-off error, the
///      selection may vary across platforms.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the input argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine solves the ray-surface intercept problem for
///  a specified ray and a surface represented by triangular plate
///  model. The surface representation is provided by data in a
///  type 2 segment of a DSK file.
///
///  This routine does not assume that the segment from which the
///  surface model data are read represents the entire surface of
///  the target body. A program could call this routine repeatedly
///  to find the surface intercept of a ray and a shape model
///  partitioned into multiple segments.
///
///  In general, this routine should be expected to run faster
///  when used with smaller shape models.
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
///  1) Find the surface intercept points corresponding to a latitude/
///     longitude grid of a specified resolution, for a specified
///     target body.
///
///     This simple program assumes the shape model for the target
///     body is stored in a single type 2 DSK segment, and that this
///     segment is the first one in the DSK file to which it belongs.
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKX02_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///     C
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DSKSGR
///           DOUBLE PRECISION      RPD
///     C
///     C
///     C     Local parameters
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               NLAT
///           PARAMETER           ( NLAT   = 9 )
///
///           INTEGER               NLON
///           PARAMETER           ( NLON   = 9 )
///
///     C
///     C     Local parameters
///     C
///           DOUBLE PRECISION      TOL
///           PARAMETER           ( TOL   =  1.D-12 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    DSK
///
///           DOUBLE PRECISION      DSKDSC ( DSKDSZ )
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      MAXR
///           DOUBLE PRECISION      R
///           DOUBLE PRECISION      RAYDIR ( 3 )
///           DOUBLE PRECISION      VERTEX ( 3 )
///           DOUBLE PRECISION      XLAT
///           DOUBLE PRECISION      XLON
///           DOUBLE PRECISION      XPT    ( 3 )
///           DOUBLE PRECISION      XR
///
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               PLID
///
///           LOGICAL               FOUND
///
///     C
///     C     Prompt for the name of the DSK to read.
///     C
///           CALL PROMPT ( 'Enter DSK name > ', DSK )
///     C
///     C     Open the DSK file for read access.
///     C     We use the DAS-level interface for
///     C     this function.
///     C
///           CALL DASOPR ( DSK, HANDLE )
///     C
///     C     Begin a forward search through the
///     C     kernel, treating the file as a DLA.
///     C     In this example, it's a very short
///     C     search.
///     C
///           CALL DLABFS ( HANDLE, DLADSC, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///     C
///     C        We arrive here only if the kernel
///     C        contains no segments.  This is
///     C        unexpected, but we're prepared for it.
///     C
///              CALL SETMSG ( 'No segments found '
///          .   //            'in DSK file #.'    )
///              CALL ERRCH  ( '#',  DSK           )
///              CALL SIGERR ( 'SPICE(NODATA)'     )
///
///           END IF
///
///     C
///     C     If we made it this far, DLADSC is the
///     C     DLA descriptor of the first segment.
///     C
///     C     We're going to generate the intercept points
///     C     using a set of rays which point toward the
///     C     origin and whose vertices are on a
///     C     specified lat/lon grid.  To start out we
///     C     must pick a reasonable range from the origin
///     C     for the vertices:  the range must be large
///     C     enough so that the vertices are guaranteed
///     C     to be outside the target body but small
///     C     enough that we don't lose too much precision
///     C     in the surface intercept computation.
///     C
///     C     We'll look up the upper bound for the target
///     C     radius, then use 2 times this value as the
///     C     vertex magnitude.
///     C
///           CALL DSKGD ( HANDLE, DLADSC, DSKDSC )
///
///           MAXR = DSKDSC(MX3IDX)
///           R    = 2.D0 * MAXR
///
///     C
///     C     Now generate the intercept points.  We generate
///     C     intercepts along latitude bounds, working from
///     C     north to south. Latitude ranges from +80 to
///     C     -80 degrees. Longitude ranges from 0 to 320
///     C     degrees. The increment is 20 degrees for
///     C     latitude and 40 degrees for longitude.
///     C
///           DO I = 1, NLAT
///
///              LAT = RPD() * ( 100.D0 - 20.D0*I )
///
///              DO J = 1, NLON
///
///                 LON = RPD() * 40.D0 * (J-1)
///     C
///     C           Produce a ray vertex for the current
///     C           lat/lon value.  Negate the vertex to
///     C           produce the ray's direction vector.
///     C
///                 CALL LATREC ( R, LON, LAT, VERTEX )
///                 CALL VMINUS ( VERTEX,      RAYDIR )
///     C
///     C           Find the surface intercept for this
///     C           ray.
///     C
///                 CALL DSKX02 ( HANDLE, DLADSC, VERTEX,
///          .                    RAYDIR, PLID,   XPT,    FOUND  )
///     C
///     C           Since the ray passes through the origin on
///     C           the body-fixed frame associated with the
///     C           target body, we'd rarely expect to find that
///     C           the ray failed to intersect the surface.
///     C           For safety, we check the FOUND flag.  (A
///     C           "not found" condition could be a sign of
///     C           a bug.)
///     C
///                 IF ( .NOT. FOUND ) THEN
///
///                    WRITE(*,*) ' '
///                    WRITE(*,*) 'Intercept not found!'
///                    WRITE(*,*) '   Ray vertex:'
///                    WRITE(*,*) '   Longitude (deg): ', LON/RPD()
///                    WRITE(*,*) '   Latitude  (deg): ', LAT/RPD()
///                    WRITE(*,*) '   Range      (km): ', R
///                    WRITE(*,*) ' '
///
///                 ELSE
///     C
///     C              This is the normal case.  Display the
///     C              intercept plate ID and the intercept
///     C              point in both Cartesian and latitudinal
///     C              coordinates.  Show the corresponding ray
///     C              vertex to facilitate validation of results.
///     C
///     C              Use RECRAD rather than RECLAT to produce
///     C              non-negative longitudes.
///     C
///                    CALL RECRAD ( XPT, XR, XLON, XLAT )
///
///                    WRITE(*,*) ' '
///                    WRITE(*,*) 'Intercept found:'
///                    WRITE(*,*) '   Plate ID:             ', PLID
///                    WRITE(*, '(1X,A,3F12.8)' )
///          .         '   Cartesian coordinates:', XPT
///                    WRITE(*,*) '   Latitudinal coordinates:'
///                    WRITE(*,*) '   Longitude (deg): ', XLON/RPD()
///                    WRITE(*,*) '   Latitude  (deg): ', XLAT/RPD()
///                    WRITE(*,*) '   Range      (km): ', XR
///                    WRITE(*,*)
///                    WRITE(*,*) '   Ray vertex:'
///                    WRITE(*,*) '   Longitude (deg): ', LON/RPD()
///                    WRITE(*,*) '   Latitude  (deg): ', LAT/RPD()
///                    WRITE(*,*) '   Range      (km): ', R
///                    WRITE(*,*) ' '
///
///                 END IF
///
///              END DO
///
///           END DO
///     C
///     C     Close the kernel.  This isn't necessary in a stand-
///     C     alone program, but it's good practice in subroutines
///     C     because it frees program and system resources.
///     C
///           CALL DASCLS ( HANDLE )
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos_3_3.bds, the output
///     was:
///
///
///     Enter DSK name > phobos_3_3.bds
///
///      Intercept found:
///         Plate ID:                   306238
///         Cartesian coordinates:  1.52087789  0.00000000  8.62532711
///         Latitudinal coordinates:
///         Longitude (deg):    0.0000000000000000
///         Latitude  (deg):    80.000000000000014
///         Range      (km):    8.7583866856211490
///
///         Ray vertex:
///         Longitude (deg):    0.0000000000000000
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    28.023536291251524
///
///
///      Intercept found:
///         Plate ID:                   317112
///         Cartesian coordinates:  1.18970365  0.99827989  8.80777185
///         Latitudinal coordinates:
///         Longitude (deg):    40.000000000000000
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    8.9436459265318629
///
///         Ray vertex:
///         Longitude (deg):    40.000000000000000
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    28.023536291251524
///
///
///      Intercept found:
///         Plate ID:                   324141
///         Cartesian coordinates:  0.27777518  1.57534131  9.07202903
///         Latitudinal coordinates:
///         Longitude (deg):    80.000000000000028
///         Latitude  (deg):    80.000000000000014
///         Range      (km):    9.2119797003191017
///
///         Ray vertex:
///         Longitude (deg):    80.000000000000000
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    28.023536291251524
///
///
///      Intercept found:
///         Plate ID:                   327994
///         Cartesian coordinates: -0.81082405  1.40438846  9.19682344
///         Latitudinal coordinates:
///         Longitude (deg):    120.00000000000001
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    9.3386992651610452
///
///         Ray vertex:
///         Longitude (deg):    119.99999999999999
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    28.023536291251524
///
///
///      Intercept found:
///         Plate ID:                   329431
///         Cartesian coordinates: -1.47820193  0.53802150  8.92132122
///         Latitudinal coordinates:
///         Longitude (deg):    160.00000000000006
///         Latitude  (deg):    80.000000000000014
///         Range      (km):    9.0589469760393797
///
///         Ray vertex:
///         Longitude (deg):    160.00000000000000
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    28.023536291251524
///
///
///      Intercept found:
///         Plate ID:                   196042
///         Cartesian coordinates: -1.49854761 -0.54542673  9.04411256
///         Latitudinal coordinates:
///         Longitude (deg):    200.00000000000000
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    9.1836325764960041
///
///         Ray vertex:
///         Longitude (deg):    200.00000000000000
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    28.023536291251524
///
///
///      Intercept found:
///         Plate ID:                   235899
///         Cartesian coordinates: -0.78240454 -1.35516441  8.87447325
///         Latitudinal coordinates:
///         Longitude (deg):    239.99999999999991
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    9.0113763066160804
///
///         Ray vertex:
///         Longitude (deg):    239.99999999999997
///         Latitude  (deg):    80.000000000000000
///         Range      (km):    28.023536291251524
///
///
///
///     [...]
///
///
///     Warning: incomplete output. Only 100 out of 1135 lines have
///     been provided.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  A. Woo, "Fast Ray-Box Intersection", Graphic Gems I,
///       395-396, Aug. 1990
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J.A. Bytof         (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 13-JAN-2021 (NJB) (JDR) (BVS)
///
///         Bug fix: in some cases the previous version of this routine
///         could still return an intercept outside of the segment
///         boundaries by more than the allowed margin. In those cases,
///         the returned plate ID was invalid. Both problems have been
///         corrected.
///
///         See $Revisions for details.
///
///         Edited the header to comply with NAIF standard. Updated the
///         example code to retrieve directly from the DSK descriptor the
///         upper bound for the target radius. Added record in
///         $Index_Entries.
///
/// -    SPICELIB Version 1.0.0, 04-APR-2017 (NJB) (EDW) (JAB)
///
///         Added test for containment of intersection point
///         within segment boundaries. Updated logic for saving
///         segment attributes so that errors won't cause saved
///         values to be improperly re-used on subsequent calls.
///
///         24-FEB-2016 (NJB)
///
///            Replaced call to TOGRID with call to ZZTOGRID.
///            Replaced call to PLTREC with call to PLTNRM.
///            Now obtains plate expansion fraction from DSKGTL.
///
///         25-FEB-2015 (NJB)
///
///            Bug fix: now ray-voxel grid intercept is displaced toward
///            the input ray's vertex only when the vertex is outside
///            the target body's voxel grid.
///
///         10-SEP-2014 (NJB)
///
///            Bug fix: during an intercept search over the voxel list
///            returned by XDDA, if an intercept outside the current
///            voxel---by more than a small tolerance---is found, the
///            search rejects that intercept and continues until a
///            valid intercept is found and all plates in the voxel
///            containing that intercept have been checked for an
///            intersection. The rejected intercept may later be
///            determined to be a valid solution during a check of
///            plates associated with a voxel that contains that
///            intercept; in fact it is the correct solution if no
///            other plates contain a solution closer to the ray's
///            vertex. (Usually there is a unique voxel containing the
///            intercept, but this is not so if the intercept lies on
///            a voxel boundary not on an edge of the voxel grid.)
///
///            Note that there's no need to look for intersections in
///            voxels further out in the voxel list than the first one
///            that contains an intercept.
///
///            The previous version of the routine terminated the
///            search after checking all plates in the current voxel,
///            after an intercept was found on any plate associated
///            with that voxel. The intercept was not required to be
///            contained in the voxel.
///
///            See the $Revisions section for details.
///
///         30-JUN-2014 (NJB)
///
///            Bug fix: renamed "found" flag returned by ZZRAYBOX.
///
///            Added code to test for empty voxel list after
///            voxel list compression.
///
///         15-JUN-2014 (NJB)
///
///            Made some minor edits to in-line comments, and removed
///            comments that had become inapplicable due to code changes.
///
///         06-JUN-2014 (NJB)
///
///            Now expands plates slightly before performing ray-plate
///            intersection computations.
///
///            Bug fix: now calls ZZRAYBOX to find the ray-box intercept.
///            This reduces round-off error in the variable COORD.
///
///         02-MAY-2014 (NJB)
///
///            Bug fix: added FAILED checks after each DSKI02 and DSKD02
///            call.
///
///            Some precautionary measures were added: a backstop
///            check for an empty voxel list was added after the XDDA
///            call. A backstop initialization of PNTR was added
///            before the plate collection loop. This initialization
///            is needed only if the voxel list returned by XDDA is
///            empty. The list should never be empty.
///
///         25-MAR-2014 (NJB)
///
///            Bug fix: duplicate plates are now marked so that the
///            unmarked instance is the one in the closest voxel to
///            the ray's origin.
///
///            Bug fix: corrected buffer overflow error detection for
///            insertion of plate IDs into plate ID array.
///
///         20-JUL-2011 (NJB)
///
///            Bug fix: this routine now tests FAILED after its
///            call to XDDA.
///
///            Header correction: the detailed input section
///            now says that the ray's vertex *is* required to
///            be outside the target body.
///
///         09-JUN-2011 (NJB)
///
///            All large local arrays are now saved in order to support
///            calling a C translation of this routine from Java.
///
///            The buffer VIDXS is now initialized prior to its
///            first use in an argument list. This was done to
///            to suppress compiler warnings. The original code was
///            correct, since along with the buffer, an array size
///            of zero was passed to the called function.
///
///            The example program was updated for compatibility with
///            the final DSK descriptor layout. The output format was
///            adjusted. Sample output from the program is now shown
///            in the header.
///
///         13-MAY-2010 (NJB)
///
///            No longer uses plate records to weed out
///            plates prior to ray-plate intercept tests.
///            Now uses local vertex buffer. Logic for choosing
///            plate when intercept is on edge or vertex has
///            been simplified.
///
///         06-MAY-2010 (NJB)
///
///            Now calls DSKB02 rather than DSKP02.
///
///         20-APR-2010 (NJB)
///
///            Updated header section order.
///
///         26-SEP-2008 (NJB)
///
///            Moved OBSMAT computation out of loop.
///
///         27-DEC-2006 (NJB) (EDW)
///
///            Header example was updated to show maximum radius
///            being obtained from DSK descriptor rather than via
///            all to DSKD02.
///
///         31-OCT-2006 (NJB) (EDW)
///
///            Modified to work with DLA-based kernels. Many
///            changes were made to the algorithm to improve
///            execution speed.
///
///         19-AUG-2004 (EDW)
///
///            Implemented "Fast Ray-Box Intersection" algorithm.
///            Renamed routine DSKX02 from PLBORE_3.
///
///         25-FEB-1999 (JAB)
///
///            Based on PLBORE and PLBORE_2.
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 13-JAN-2021 (NJB)
///
///         Bug fix: in some cases the previous version of this
///         routine could return an intercept outside of the segment
///         boundaries (the "outside intercept") by more than the
///         allowed margin. In those cases, the returned plate ID was
///         invalid.
///
///         Both problems have been corrected.
///
///         Description of the bug
///         ----------------------
///
///         In the case where all of the follow conditions hold:
///
///            - the input ray's intercepts exist both within and
///              outside the segment's boundaries
///
///            - the outside intercept is considered the nearest
///              solution to the vertex at the time the intercept
///              is found
///
///            - the intercept that should have been selected was
///              found before the outside intercept
///
///            - both intercepts lie on plates belonging to the same voxel
///
///         the outside intercept will overwrite the correct intercept.
///
///         In the situation described above, the plate ID returned
///         will not be that of the outside plate.
///
///         Solution
///         --------
///
///         Each intercept that passes the test for being closest, of
///         all intercepts seen so far, to the ray's vertex is stored
///         in a temporary variable. The output XPT is updated only
///         when the intercept is found to lie within the segment's
///         coordinate bounds, or outside the bounds by no more than
///         the allowed margin.
///
/// -    SPICELIB Version 1.0.0, 04-APR-2017 (NJB)
///
///         10-SEP-2014 (NJB)
///
///            Bug fix: during an intercept search over the voxel
///            list returned by XDDA, if an intercept outside the
///            current voxel---by more than a small tolerance---is
///            found, the search rejects that intercept and continues
///            until a valid intercept is found and all plates in the
///            voxel containing that intercept have been checked for
///            an intersection. The rejected intercept may later be
///            determined to be a valid solution during a check of
///            plates associated with a voxel that contains that
///            intercept; in fact it is the correct solution if no
///            other plates contain a solution closer to the ray's
///            vertex. (Usually there is a unique voxel containing
///            the intercept, but this is not so if the intercept
///            lies on a voxel boundary not on an edge of the voxel
///            grid.)
///
///            Note that there's no need to look for intersections in
///            voxels further out in the voxel list than the first
///            one that contains an intercept.
///
///            The previous version of the routine terminated the
///            search after checking all plates in the current voxel,
///            after an intercept was found on any plate associated
///            with that voxel. The intercept was not required to be
///            contained in the voxel.
///
///            In the previous version of the routine, an intercept
///            found outside of the current voxel could effectively
///            mask an intercept closer to the ray's vertex, as shown
///            in the diagram below.
///
///            In this diagram, "V" represents the vertex of the ray.
///            The letter sequences "Q*" and "P*" represent plates.
///            Here the ray hits voxel 1 and finds an intercept on
///            plate P* at the point marked by "@." No other
///            intercepts on plates in voxel 1 exist, so the search
///            terminates. The intercept marked by "$" is closer to
///            the vertex but is not seen.
///
///                                    V
///                                   /
///                                  /
///                                 /
///               +--------------+-/------------+
///               |    voxel 2   |/    voxel 1  |
///               |              /              |
///               |        QQQQQ$|              |
///               |            / |              |
///               |           /  |              |
///               |          /   |              |
///               |       PP@PPPPPPPPPPPPPPP    |
///               +--------------+--------------+
///
///
///            The updated algorithm, when presented with the
///            situation shown above, will check all plates in voxel
///            2 before terminating.
///
///            Note that the problem could occur in cases where
///            voxels 1 and 2 are not adjacent.
/// ```
pub fn dskx02(
    ctx: &mut SpiceContext,
    handle: i32,
    dladsc: &[i32],
    vertex: &[f64; 3],
    raydir: &[f64; 3],
    plid: &mut i32,
    xpt: &mut [f64; 3],
    found: &mut bool,
) -> crate::Result<()> {
    DSKX02(
        handle,
        dladsc,
        vertex,
        raydir,
        plid,
        xpt,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKX02 ( DSK, ray-surface intercept, type 2 )
pub fn DSKX02(
    HANDLE: i32,
    DLADSC: &[i32],
    VERTEX: &[f64],
    RAYDIR: &[f64],
    PLID: &mut i32,
    XPT: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DLADSC = DummyArray::new(DLADSC, 1..);
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut COORD = StackArray::<f64, 3>::new(1..=3);
    let mut EDGES = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut GREEDM: f64 = 0.0;
    let mut HITCOR = StackArray::<f64, 3>::new(1..=3);
    let mut NEAR: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut OBSMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut POINTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SCALE: f64 = 0.0;
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut VTX2 = StackArray::<f64, 3>::new(1..=3);
    let mut VTXOFF = StackArray::<f64, 3>::new(1..=3);
    let mut XPDFRC: f64 = 0.0;
    let mut XPNTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut CGXYZ = StackArray::<i32, 3>::new(1..=3);
    let mut CVID: i32 = 0;
    let mut DIM: i32 = 0;
    let mut FINAL: i32 = 0;
    let mut FX: i32 = 0;
    let mut FY: i32 = 0;
    let mut FZ: i32 = 0;
    let mut GROUP: i32 = 0;
    let mut GRPBEG: i32 = 0;
    let mut GRPEND: i32 = 0;
    let mut GRPSIZ: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut MINIDX: i32 = 0;
    let mut NGROUP: i32 = 0;
    let mut NPLATE: i32 = 0;
    let mut NVBUF: i32 = 0;
    let mut NVXOUT: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut PLROOM: i32 = 0;
    let mut PNTR: i32 = 0;
    let mut START: i32 = 0;
    let mut TO: i32 = 0;
    let mut TOTPLT: i32 = 0;
    let mut VIDS = StackArray::<i32, 3>::new(1..=3);
    let mut VLOC: i32 = 0;
    let mut VOXPTR: i32 = 0;
    let mut VXC1: i32 = 0;
    let mut VXC2: i32 = 0;
    let mut VXC3: i32 = 0;
    let mut W: i32 = 0;
    let mut BOXHIT: bool = false;
    let mut EXTRA: bool = false;
    let mut HAVE: bool = false;
    let mut HITS: bool = false;
    let mut INSEG: bool = false;
    let mut INVOX: bool = false;
    let mut NEWSEG: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Statement function type declarations
    //

    //
    // Local parameters
    //

    //
    // Tolerance used for vertex-voxel grid distance test:
    //

    //
    // Maximum number of voxels we can accept for
    // one XDDA call.
    //

    //
    // Maximum number of plates we work with
    // at any time.
    //

    //
    // Parameter indicating no coordinates are to be excluded
    // in the test for a point being within segment boundaries.
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Statement functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKX02", ctx)?;

    //
    // Until we have better knowledge we assume there is no intersection.
    //
    *PLID = 0;
    *FOUND = false;
    HAVE = false;
    NEAR = DPMAX();

    //
    // Initialize the vertex buffer.
    //
    CLEARI(BUFSIZ, save.VIDXS.as_slice_mut());

    //
    // Check whether the ray direction vector is the zero vector.
    //
    if VZERO(RAYDIR.as_slice()) {
        SETMSG(b"Ray direction is the zero vector.", ctx);
        SIGERR(b"SPICE(RAYISZEROVECTOR)", ctx)?;
        CHKOUT(b"DSKX02", ctx)?;
        return Ok(());
    }

    //
    // Obtain the unit vector of the ray from the observer.
    //
    VHAT(RAYDIR.as_slice(), UDIR.as_slice_mut());

    //
    // Decide whether we're looking at a new segment.
    //
    NEWSEG = true;

    if (HANDLE == save.PRVHAN) {
        //
        // The input handle matches the previous handle.  Note that the
        // initial value of PRVHAN is 0, which is never a valid handle,
        // so on the first pass, this test will fail.
        //
        if (((DLADSC[IBSIDX] == save.PRVDSC[IBSIDX]) && (DLADSC[DBSIDX] == save.PRVDSC[DBSIDX]))
            && (DLADSC[CBSIDX] == save.PRVDSC[CBSIDX]))
        {
            //
            // All of the DLA segment base addresses match.
            //
            NEWSEG = false;
        }
    }

    if NEWSEG {
        //
        // Make sure we can't have a match with an earlier
        // segment on a subsequent call, if we exit this
        // block due to an error.
        //
        save.PRVHAN = 0;

        //
        // Retrieve the voxel grid origin in model
        // units and calculate the farthest extent of the
        // voxel grid in voxel space.
        //
        DSKB02(
            HANDLE,
            DLADSC.as_slice(),
            &mut save.NV,
            &mut save.NP,
            &mut save.NVXTOT,
            save.VTXBDS.as_slice_mut(),
            &mut save.VOXSIZ,
            save.VOXORI.as_slice_mut(),
            save.VGREXT.as_slice_mut(),
            &mut save.CGSCAL,
            &mut save.VTXNPL,
            &mut save.VOXNPT,
            &mut save.VOXNPL,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"DSKX02", ctx)?;
            return Ok(());
        }

        //
        // Compute the grid dimensions in units of km. First check
        // the voxel size.
        //
        if (save.VOXSIZ == 0 as f64) {
            SETMSG(
                b"Voxel size is zero. This is an error in the DSK file attached to handle #.",
                ctx,
            );
            ERRINT(b"#", HANDLE, ctx);
            SIGERR(b"SPICE(INVALIDVALUE)", ctx)?;
            CHKOUT(b"DSKX02", ctx)?;
            return Ok(());
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.GRDEXT[I] = ((save.VGREXT[I] as f64) * save.VOXSIZ);
                I += m3__;
            }
        }

        //
        // Set the margin used for checking whether the ray's vertex
        // is close to the voxel grid.
        //
        save.GRDTOL =
            (VTXTOL * intrinsics::DMAX1(&[save.GRDEXT[1], save.GRDEXT[2], save.GRDEXT[3]]));

        //
        // Check the coarse grid voxel scale.
        //
        if (save.CGSCAL < 1) {
            SETMSG(b"Coarse grid scale = #; should be >= 1.", ctx);
            ERRINT(b"#", save.CGSCAL, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"DSKX02", ctx)?;
            return Ok(());
        }

        //
        // Get the coarse voxel grid dimensions and the coarse voxel
        // occupancy flags.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = 3;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.CGREXT[I] = (save.VGREXT[I] / save.CGSCAL);
                I += m3__;
            }
        }

        save.CGSCL2 = (save.CGSCAL * save.CGSCAL);

        save.NCGR = (save.NVXTOT / intrinsics::pow(save.CGSCAL, 3));

        if (save.NCGR > MAXCGR) {
            SETMSG(b"Coarse grid size NCGR = #. Buffer size = #", ctx);
            ERRINT(b"#", save.NCGR, ctx);
            ERRINT(b"#", MAXCGR, ctx);
            SIGERR(b"SPICE(GRIDTOOLARGE)", ctx)?;
            CHKOUT(b"DSKX02", ctx)?;
            return Ok(());
        }

        DSKI02(
            HANDLE,
            DLADSC.as_slice(),
            KWCGPT,
            1,
            MAXCGR,
            &mut DIM,
            save.CGRPTR.as_slice_mut(),
            ctx,
        )?;

        DSKGD(HANDLE, DLADSC.as_slice(), save.DSKDSC.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DSKX02", ctx)?;
            return Ok(());
        }

        save.CORSYS = intrinsics::IDNINT(save.DSKDSC[SYSIDX]);

        save.PRVHAN = HANDLE;

        MOVEI(DLADSC.as_slice(), DLADSZ, save.PRVDSC.as_slice_mut());
    }

    //
    // Compute tolerance used for determining whether an intercept
    // is inside a voxel. The expansion fraction must be fetched
    // on every call to DSKX02.
    //
    DSKGTL(KEYXFR, &mut XPDFRC, ctx)?;

    save.XTOL = (XPDFRC
        * intrinsics::DMAX1(&[
            f64::abs(save.GRDEXT[1]),
            f64::abs(save.GRDEXT[2]),
            f64::abs(save.GRDEXT[3]),
        ]));

    //
    // Find the ray intercept on the surface of the fine voxel grid,
    // if the intercept exists.
    //
    ZZRAYBOX(
        VERTEX.as_slice(),
        RAYDIR.as_slice(),
        save.VOXORI.as_slice(),
        save.GRDEXT.as_slice(),
        VTX2.as_slice_mut(),
        &mut BOXHIT,
        ctx,
    )?;

    if !BOXHIT {
        CHKOUT(b"DSKX02", ctx)?;
        return Ok(());
    }

    //
    // Convert the grid intercept to voxel space coordinates.
    // The result COORD will be used as the ray's vertex in XDDA.
    //
    ZZTOGRID(
        VTX2.as_slice(),
        save.VOXORI.as_slice(),
        save.VOXSIZ,
        COORD.as_slice_mut(),
        ctx,
    )?;

    //
    // Determine the voxels hit by the ray.
    //
    XDDA(
        COORD.as_slice(),
        UDIR.as_slice(),
        save.VGREXT.as_slice(),
        NVXLST,
        &mut NVXOUT,
        save.VOXLST.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DSKX02", ctx)?;
        return Ok(());
    }

    //
    // We don't expect the voxel list to be empty, but leave now
    // if it is.
    //
    if (NVXOUT == 0) {
        CHKOUT(b"DSKX02", ctx)?;
        return Ok(());
    }

    //
    // Rather than using the original observer's location, we use a
    // location derived from COORD, which is the intercept of the ray
    // and the surface of the voxel grid.  We start with COORD, convert
    // this location to the model coordinate system, and back outward a
    // bit to make sure we obtain a location outside the grid (we don't
    // want to miss any plates that might be located right on the grid's
    // surface).
    //
    // This vertex change is not performed if the vertex is already
    // inside, or within a small margin away from, the voxel grid.
    //
    VSUB(
        VERTEX.as_slice(),
        save.VOXORI.as_slice(),
        VTXOFF.as_slice_mut(),
    );

    if ((((((VTXOFF[1] < -save.GRDTOL) || (VTXOFF[1] > (save.GRDTOL + save.GRDEXT[1])))
        || (VTXOFF[2] < -save.GRDTOL))
        || (VTXOFF[2] > (save.GRDTOL + save.GRDEXT[2])))
        || (VTXOFF[3] < -save.GRDTOL))
        || (VTXOFF[3] > (save.GRDTOL + save.GRDEXT[3])))
    {
        //
        // The vertex is outside of the voxel grid by more than the
        // margin. Move the ray-grid intercept away from the grid to
        // improve numeric performance.
        //
        VLCOM3(
            1.0,
            save.VOXORI.as_slice(),
            save.VOXSIZ,
            COORD.as_slice(),
            -1.0,
            UDIR.as_slice(),
            VTX2.as_slice_mut(),
        );
    }

    //
    // We are going to need to subtract the location of the observer
    // from vertices of a plate. To speed things up a tiny bit, we'll
    // make 3 copies of the observer's location so that we make a single
    // subroutine call to handle the 3 subtractions.
    //
    VEQU(VTX2.as_slice(), OBSMAT.subarray_mut([1, 1]));
    VEQU(VTX2.as_slice(), OBSMAT.subarray_mut([1, 2]));
    VEQU(VTX2.as_slice(), OBSMAT.subarray_mut([1, 3]));

    //
    // Use the coarse voxel grid to compress the voxel list. We
    // remove all voxels belonging to empty coarse voxels.
    //
    TO = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NVXOUT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Find the coordinates, then the ID, of the coarse voxel
            // containing this voxel.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    CGXYZ[J] = (((save.VOXLST[[J, I]] - 1) / save.CGSCAL) + 1);
                    J += m3__;
                }
            }

            CVID = VOX2ID(CGXYZ[1], CGXYZ[2], CGXYZ[3], save.CGREXT[1], save.CGREXT[2]);

            if (save.CGRPTR[CVID] > 0) {
                //
                // This coarse voxel is non-empty; add the index of the
                // current voxel to the output list.  Save the coordinates of
                // the parent coarse voxel as well.
                //
                TO = (TO + 1);
                save.VXLOUT[TO] = I;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 3;
                    let m3__: i32 = 1;
                    J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        save.VXLCG[[J, TO]] = CGXYZ[J];
                        J += m3__;
                    }
                }
                //
                // Save the coarse voxel start pointer as well.
                //
                save.VXLSTR[TO] = save.CGRPTR[CVID];
            }

            I += m3__;
        }
    }

    //
    // Update NVXOUT to be the number of voxels in the compressed list.
    //
    NVXOUT = TO;
    //
    // If the voxel list became empty after compression, we're
    // done.
    //
    if (NVXOUT == 0) {
        CHKOUT(b"DSKX02", ctx)?;
        return Ok(());
    }

    //
    // Initialize PNTR in case the voxel list is empty.
    // (This is a backstop precaution: the voxel list
    // should never be empty at this point.) PNTR will
    // be referenced after the end of the outer loop below.
    //
    PNTR = 1;

    //
    // The vertex buffer is empty so far.
    //
    NVBUF = 0;

    //
    // Break up the list of voxels into groups; process each
    // group in turn until we find an intersection or run out
    // of voxels.
    //
    GRPSIZ = intrinsics::MAX0(&[1, ((NVXOUT + 1) / 2)]);

    NGROUP = (((NVXOUT - 1) / GRPSIZ) + 1);

    GROUP = 1;

    while ((GROUP <= NGROUP) && !HAVE) {
        PNTR = 1;

        GRPBEG = (((GROUP - 1) * GRPSIZ) + 1);

        GRPEND = intrinsics::MIN0(&[((GRPBEG + GRPSIZ) - 1), NVXOUT]);

        PLROOM = MAXPL;

        for VI in GRPBEG..=GRPEND {
            //
            // Look up the plate list pointer for this voxel.
            //
            //
            // We begin by finding the offset of the voxel from
            // the base of its parent coarse voxel.
            //
            J = save.VXLOUT[VI];

            FX = (save.VOXLST[[1, J]] - (save.CGSCAL * (save.VXLCG[[1, VI]] - 1)));
            FY = (save.VOXLST[[2, J]] - (save.CGSCAL * (save.VXLCG[[2, VI]] - 1)));
            FZ = (save.VOXLST[[3, J]] - (save.CGSCAL * (save.VXLCG[[3, VI]] - 1)));

            OFFSET = ((FX + (save.CGSCAL * (FY - 1))) + (save.CGSCL2 * (FZ - 1)));

            //
            // Now compute the index of voxel-plate list pointer in
            // the pointer array, and look up the pointer.
            //
            J = ((save.VXLSTR[VI] + OFFSET) - 1);

            DSKI02(
                HANDLE,
                DLADSC.as_slice(),
                KWVXPT,
                J,
                1,
                &mut DIM,
                std::slice::from_mut(&mut VOXPTR),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DSKX02", ctx)?;
                return Ok(());
            }

            if (VOXPTR == -1) {
                NPLATE = 0;
            } else {
                //
                // Get the plate count for this voxel.
                //
                DSKI02(
                    HANDLE,
                    DLADSC.as_slice(),
                    KWVXPL,
                    VOXPTR,
                    1,
                    &mut DIM,
                    std::slice::from_mut(&mut NPLATE),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"DSKX02", ctx)?;
                    return Ok(());
                }
            }

            if (NPLATE > 0) {
                if (NPLATE <= PLROOM) {
                    //
                    // Get the plate list for this voxel.
                    //
                    DSKI02(
                        HANDLE,
                        DLADSC.as_slice(),
                        KWVXPL,
                        (VOXPTR + 1),
                        NPLATE,
                        &mut DIM,
                        save.PLATID.subarray_mut(PNTR),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"DSKX02", ctx)?;
                        return Ok(());
                    }

                    PLROOM = (PLROOM - NPLATE);
                } else {
                    SETMSG(
                        b"NPLATE = #. Available room in PLATID array = #. Array size = #.",
                        ctx,
                    );
                    ERRINT(b"#", NPLATE, ctx);
                    ERRINT(b"#", PLROOM, ctx);
                    ERRINT(b"#", MAXPL, ctx);
                    SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
                    CHKOUT(b"DSKX02", ctx)?;
                    return Ok(());
                }

                //
                // Fill in the corresponding elements of the parallel
                // "source" array with the current voxel loop index.
                // XDDA lists these voxels in the order the ray hits
                // them, so the lowest indexed voxels are hit first.
                //
                FILLI(VI, NPLATE, save.SOURCE.subarray_mut(PNTR));
            }

            //
            // NPLATE returns zero or greater.
            //
            PNTR = (PNTR + NPLATE);
        }
        //
        // We've collected all plate info for the current voxel group.
        //
        TOTPLT = (PNTR - 1);

        //
        // We want to sort the plate ID array and remove duplicates.
        // However, we want to keep the plates ordered according to the
        // sequence in which their containing voxels were hit by the ray.
        // So we find the order vector for the plate ID array, then use
        // this vector to mark duplicates.
        //
        ORDERI(save.PLATID.as_slice(), TOTPLT, save.ORDVEC.as_slice_mut());

        //
        // Negate the plate ID of every duplicate we find, leaving
        // the instance in the voxel closest to the ray's origin
        // unmarked. For every pair of plates with the same ID,
        // we'll mark the one with the greater index in the plate
        // ID array.
        //
        // We use MINDIX to identify the index, in the plate ID array,
        // of the current unmarked plate ID. MINIDX is re-used for
        // each set of distinct plate IDs.
        //
        // The following loop considers plate IDs in increasing order.
        //
        MINIDX = save.ORDVEC[1];

        {
            let m1__: i32 = 2;
            let m2__: i32 = TOTPLT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // The condition below uses absolute value because the plate
                // ID at index I-1 may have been "marked" via negation.
                //
                if (save.PLATID[save.ORDVEC[I]] == i32::abs(save.PLATID[save.ORDVEC[(I - 1)]])) {
                    //
                    // The plates having indices ORDVEC(I-1) and ORDVEC(I) are
                    // duplicates.
                    //
                    // At this point MINIDX is the lowest index in the plate ID
                    // array of any plate seen so far having an ID equal to
                    // PLATID(ORDVEC(I)).
                    //
                    // If ORDVEC(I) is the new "minimum," negate the plate ID
                    // at the old "minimum"; otherwise negate the plate ID at
                    // index ORDVEC(I).
                    //
                    if (save.ORDVEC[I] < MINIDX) {
                        //
                        // The plate that was previously at the minimum index is
                        // now considered a duplicate. The new minimum index for
                        // the current plate ID value is ORDVEC(I).
                        //
                        save.PLATID[MINIDX] = -save.PLATID[MINIDX];
                        MINIDX = save.ORDVEC[I];
                    } else {
                        //
                        // The current plate is a duplicate; mark it.
                        //
                        save.PLATID[save.ORDVEC[I]] = -save.PLATID[save.ORDVEC[I]];
                    }
                } else {
                    //
                    // We're looking at a new plate ID. For the moment, this
                    // ID has no duplicates.
                    //
                    MINIDX = save.ORDVEC[I];
                }

                I += m3__;
            }
        }

        //
        // If something went wrong up above there is no point in
        // going on from here.
        //
        if FAILED(ctx) {
            CHKOUT(b"DSKX02", ctx)?;
            return Ok(());
        }

        //
        // Now examine each plate in the PLATID list for this voxel group.
        // PNTR has the value of the index available for data in
        // PLATID, so the final location of data is at index PNTR - 1.
        //
        EXTRA = false;
        FINAL = 0;
        NEAR = 0.0;

        I = 1;

        while ((I <= TOTPLT) && !EXTRA) {
            //
            // Retrieve the current plate ID.
            //
            J = save.PLATID[I];

            if (J > 0) {
                //
                // This is not a duplicate plate; consider it.
                //
                if HAVE {
                    //
                    // We already have a hit. See whether this plate resides
                    // in the voxel in which the last hit was found, or in a
                    // later voxel.
                    //
                    if (save.SOURCE[I] > FINAL) {
                        //
                        // This is a "late plate": it occurs in a voxel later
                        // than that in which the first valid hit was found.
                        //
                        EXTRA = true;
                    }
                }

                if !EXTRA {
                    //
                    // Fetch the vertex IDs of this plate.
                    //
                    START = ((3 * (J - 1)) + 1);

                    DSKI02(
                        HANDLE,
                        DLADSC.as_slice(),
                        KWPLAT,
                        START,
                        3,
                        &mut DIM,
                        VIDS.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"DSKX02", ctx)?;
                        return Ok(());
                    }

                    //
                    // Fetch the vertices themselves.
                    //
                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = 3;
                        let m3__: i32 = 1;
                        K = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            //
                            // Any vertex may be buffered already. Look in
                            // the vertex buffer before reading the vertex.
                            //
                            VLOC = ISRCHI(VIDS[K], NVBUF, save.VIDXS.as_slice());

                            if (VLOC > 0) {
                                //
                                // The vertex was buffered; just copy it.
                                //
                                VEQU(save.VBUFF.subarray([1, VLOC]), POINTS.subarray_mut([1, K]));
                            } else {
                                //
                                // Read in the vertex.
                                //
                                START = ((3 * (VIDS[K] - 1)) + 1);

                                DSKD02(
                                    HANDLE,
                                    DLADSC.as_slice(),
                                    KWVERT,
                                    START,
                                    3,
                                    &mut DIM,
                                    POINTS.subarray_mut([1, K]),
                                    ctx,
                                )?;

                                if FAILED(ctx) {
                                    CHKOUT(b"DSKX02", ctx)?;
                                    return Ok(());
                                }

                                //
                                // If there's room, buffer this vertex.
                                //
                                if (NVBUF < BUFSIZ) {
                                    NVBUF = (NVBUF + 1);

                                    VEQU(
                                        POINTS.subarray([1, K]),
                                        save.VBUFF.subarray_mut([1, NVBUF]),
                                    );

                                    save.VIDXS[NVBUF] = VIDS[K];
                                }
                            }

                            K += m3__;
                        }
                    }
                }

                if !EXTRA {
                    //
                    // The current plate qualifies for testing using INSANG.
                    //
                    // Retrieve the model coordinates of the J'th plate's
                    // three vertices. Expand the plate slightly to prevent
                    // round-off error from causing the ray to miss the
                    // plate. Compute the edges of the tetrahedral angle
                    // with the observer as the apex and the vertices as
                    // members of the edge rays. Finally see if the ray
                    // lies inside the tetrahedron.
                    //
                    VSUBG(
                        POINTS.as_slice(),
                        OBSMAT.as_slice(),
                        9,
                        EDGES.as_slice_mut(),
                    );

                    PLTEXP(EDGES.as_slice(), XPDFRC, XPNTS.as_slice_mut());

                    INSANG(
                        UDIR.as_slice(),
                        XPNTS.subarray([1, 1]),
                        XPNTS.subarray([1, 2]),
                        XPNTS.subarray([1, 3]),
                        &mut HITS,
                        &mut SCALE,
                    );

                    if HITS {
                        //
                        // Reject intersections with plates that face away
                        // from the ray. Accept intersections with plates
                        // that face toward the ray.
                        //
                        PLTNRM(
                            POINTS.subarray([1, 1]),
                            POINTS.subarray([1, 2]),
                            POINTS.subarray([1, 3]),
                            NORMAL.as_slice_mut(),
                        );

                        HITS = (VDOT(UDIR.as_slice(), NORMAL.as_slice()) <= 0.0);
                    }

                    if HITS {
                        //
                        // The ray intersects this plate.
                        //
                        if (!HAVE || (SCALE < NEAR)) {
                            //
                            // Either this is the first intersection we've
                            // found, or this is the closest intersection to
                            // the vertex we've found. Compute the intercept
                            // coordinates and see whether the intercept is
                            // within the current voxel. Use a small tolerance
                            // for the comparison.

                            // If this intersection point is closer to the
                            // ray's vertex than the last one, pick this point
                            // and the plate it's on.
                            //
                            // Note that we don't yet know that this solution
                            // is valid.
                            //
                            //    ___    ____   __________
                            //    XPT2 = VTX2 + SCALE*UDIR
                            //
                            VLCOM(
                                1.0,
                                VTX2.as_slice(),
                                SCALE,
                                UDIR.as_slice(),
                                XPT2.as_slice_mut(),
                            );

                            //
                            // Compute the voxel grid coordinates of the
                            // intercept. HITCOR is a double precision vector
                            // having units of voxels (voxel edge length, to
                            // be precise). Note that the components of HITCOR
                            // are zero-based.
                            //
                            ZZTOGRID(
                                XPT2.as_slice(),
                                save.VOXORI.as_slice(),
                                save.VOXSIZ,
                                HITCOR.as_slice_mut(),
                                ctx,
                            )?;
                            //
                            // Look up the voxel grid coordinates (integer,
                            // 1-based) of the current voxel.
                            //
                            K = save.VXLOUT[save.SOURCE[I]];

                            VXC1 = save.VOXLST[[1, K]];
                            VXC2 = save.VOXLST[[2, K]];
                            VXC3 = save.VOXLST[[3, K]];

                            INVOX = ((((((HITCOR[1]
                                > (((VXC1 as f64) - save.XTOL) - 1 as f64))
                                && (HITCOR[1] < ((VXC1 as f64) + save.XTOL)))
                                && (HITCOR[2] > (((VXC2 as f64) - save.XTOL) - 1 as f64)))
                                && (HITCOR[2] < ((VXC2 as f64) + save.XTOL)))
                                && (HITCOR[3] > (((VXC3 as f64) - save.XTOL) - 1 as f64)))
                                && (HITCOR[3] < ((VXC3 as f64) + save.XTOL)));

                            if INVOX {
                                //
                                // Reject solutions that are outside of the
                                // segment's boundaries, where the boundaries
                                // are extended using the "greedy" margin.
                                //
                                DSKGTL(KEYSGR, &mut GREEDM, ctx)?;

                                ZZINVELT(
                                    XPT2.as_slice(),
                                    save.CORSYS,
                                    save.DSKDSC.subarray(PARIDX),
                                    save.DSKDSC.subarray(MN1IDX),
                                    GREEDM,
                                    NONE,
                                    &mut INSEG,
                                    ctx,
                                )?;

                                if INSEG {
                                    //
                                    // We have a viable intercept. Record the
                                    // scale, plate ID, and source voxel index
                                    // in the compressed voxel list pointer
                                    // array VXLOUT. We won't look for
                                    // intercepts beyond the voxel designated by
                                    // FINAL.
                                    //
                                    VEQU(XPT2.as_slice(), XPT.as_slice_mut());

                                    HAVE = true;
                                    NEAR = SCALE;
                                    *PLID = J;
                                    FINAL = save.SOURCE[I];
                                    //
                                    // Indicate that a solution was found. We'll
                                    // keep looking for a better one if PLID is
                                    // not the last plate of the current voxel.
                                    //
                                    *FOUND = true;
                                }
                            } else {
                                //
                                // We must re-consider this plate if we
                                // encounter it in a voxel later in the voxel
                                // list. Remove all duplication markings for
                                // this plate.
                                //
                                W = i32::abs(J);

                                {
                                    let m1__: i32 = 1;
                                    let m2__: i32 = TOTPLT;
                                    let m3__: i32 = 1;
                                    K = m1__;
                                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                                        if (i32::abs(save.PLATID[K]) == W) {
                                            save.PLATID[K] = W;
                                        }

                                        K += m3__;
                                    }
                                }
                            }
                        }
                        //
                        // End of case of checking possible solution
                        // intercept.
                        //
                    }
                    //
                    // We're done with processing the current hit.
                    //
                }
                //
                // We're done with processing the current qualifying plate.
                //
            }
            //
            // We're done with the current plate.
            //
            // Fetch the next plate for this voxel group.
            //
            I = (I + 1);
        }
        //
        // We're done with the current voxel group.
        //
        GROUP = (GROUP + 1);
    }
    //
    // We've either found an intercept or have run out of voxel groups
    // to check.
    //
    // That's all folks.
    //
    CHKOUT(b"DSKX02", ctx)?;
    Ok(())
}
