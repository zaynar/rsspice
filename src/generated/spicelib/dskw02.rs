//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

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

/// DSK, write type 2 segment
///
/// Write a type 2 segment to a DSK file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle assigned to the opened DSK file.
///  CENTER     I   Central body ID code.
///  SURFID     I   Surface ID code.
///  DCLASS     I   Data class.
///  FRAME      I   Reference frame.
///  CORSYS     I   Coordinate system code.
///  CORPAR     I   Coordinate system parameters.
///  MNCOR1     I   Minimum value of first coordinate.
///  MXCOR1     I   Maximum value of first coordinate.
///  MNCOR2     I   Minimum value of second coordinate.
///  MXCOR2     I   Maximum value of second coordinate.
///  MNCOR3     I   Minimum value of third coordinate.
///  MXCOR3     I   Maximum value of third coordinate.
///  FIRST      I   Coverage start time.
///  LAST       I   Coverage stop time.
///  NV         I   Number of vertices.
///  VRTCES     I   Vertices.
///  NP         I   Number of plates.
///  PLATES     I   Plates.
///  SPAIXD     I   Double precision component of spatial index.
///  SPAIXI     I   Integer component of spatial index.
///  ANGMRG     P   Angular round-off margin.
///  GENCLS     P   General surface DSK class.
///  SVFCLS     P   Single-valued function DSK class.
///  NSYPAR     P   Maximum number of coordinate system parameters in
///                 a DSK descriptor.
///  MAXCGR     P   Maximum DSK type 2 coarse voxel count.
///  MAXPLT     P   Maximum DSK type 2 plate count.
///  MAXVOX     P   Maximum DSK type 2 voxel count.
///  MAXVRT     P   Maximum DSK type 2 vertex count.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the DAS file handle associated with a DSK file.
///           The file must be open for write access.
///
///  CENTER   is the ID code of the body whose surface is described
///           by the input plate model. CENTER refers to an
///           ephemeris object.
///
///  SURFID   is the ID code of the surface described by the input
///           plate model. Multiple surfaces (for example, surfaces
///           having different resolutions) may be associated with
///           a given body.
///
///  DCLASS   is the data class of the input data set. See the
///           INCLUDE file dskdsc.inc for values and meanings.
///
///  FRAME    is the name of the reference frame with respect to
///           which the input data are expressed.
///
///  CORSYS   is the coordinate system in which the spatial coverage
///           of the input data is expressed. CORSYS is an integer
///           code. The supported values of CORPAR are
///
///              Parameter name      Coordinate system
///              --------------      -----------------
///              LATSYS              Planetocentric latitudinal
///              RECSYS              Rectangular (Cartesian)
///              PDTSYS              Planetodetic
///
///           See the INCLUDE file dskdsc.inc for parameter
///           declarations.
///
///
///  CORPAR   is an array of parameters associated with the input
///           coordinate system.
///
///           For latitudinal and rectangular coordinates, CORPAR
///           is ignored.
///
///           For planetodetic coordinates, the contents of CORPAR
///           are:
///
///              Element         Contents
///              ---------       -----------------------------------
///              CORPAR(1)       Equatorial radius of reference
///                              spheroid.
///
///              CORPAR(2)       Flattening coefficient. The polar
///                              radius of the reference spheroid
///                              is given by
///
///                                 CORPAR(1) * ( 1 - CORPAR(2) )
///
///              CORPAR(3)...
///              CORPAR(NSYPAR)  Unused.
///
///
///  MNCOR1,
///  MXCOR1,
///  MNCOR2,
///  MXCOR2,
///  MNCOR3,
///  MXCOR3   are, respectively, lower and upper bounds of
///           each of the coordinates of the input data, where the
///           coordinate system is defined by CORSYS and CORPAR.
///           These bounds define the region for which the segment
///           provides data.
///
///           Distance units are km. Angular units are radians.
///
///           The interpretation of these bounds depends on the data
///           class; see DCLASS above.
///
///              Single-valued surfaces
///              ----------------------
///
///              If the segment has data class SVFCLS (see
///              dskdsc.inc), the segment defines a surface as a
///              single-valued function of its domain coordinates:
///              for example, it may define the radius of the
///              surface as a function of planetocentric longitude
///              and latitude, or Z as a function of X and Y.
///
///              In this case, the input data must cover a
///              rectangle in dimensions 1 and 2 of the input
///              coordinate system: the set of points
///
///                 R = { (x,y): MNCOR1 <= x <= MXCOR1;
///                              MNCOR2 <= y <= MXCOR2  }
///
///              must be completely covered by the input data. In
///              other words, for each point (x,y) of R, there must
///              be some plate containing a point whose first two
///              coordinates are (x,y).
///
///              The plate set may extend beyond the coordinate
///              range defined by the bounds on the domain.
///
///              Normally for single-valued surfaces, MNCOR3 and
///              MXCOR3 are the minimum and maximum values of the
///              function attained on the domain.
///
///
///              General surfaces
///              ----------------
///
///              If the segment has data class GENCLS (see
///              dskdsc.inc), the segment simply contains a
///              collection of plates: no guarantees are made about
///              the topology of the surface. The coordinate bounds
///              simply indicate the spatial region for which the
///              segment provides data.
///
///              Note that shapes of small bodies such as asteroids
///              and comet nuclei may fall into the "general
///              surface" category. Surface features such as cliffs,
///              caves, and arches can prevent a surface from being
///              represented as a single-valued function of latitude
///              and longitude, for example.
///
///
///           Longitude interpretation and restrictions
///           -----------------------------------------
///
///           The following rules apply to longitudes provided in
///           the arguments
///
///              MNCOR1
///              MXCOR1
///
///           All angles have units of radians. The tolerance
///           ANGMRG is used for the comparisons shown below.
///
///              1) Longitudes must be in the range
///
///                    -2*pi  :  2*pi
///
///                 Values that are out of range by no more than
///                 ANGMRG are bracketed to be in range.
///
///
///              2) It is acceptable for the longitude bounds to be
///                 out of order. If
///
///                    MXCOR1 < MNCOR1
///
///                 then either MXCOR1 is treated by the DSK
///                 subsystem as though it were MXCOR1 + 2*pi, or
///                 MNCOR1 is treated as MNCOR1 - 2*pi: whichever
///                 shift puts the bounds in the allowed range is
///                 made.
///
///                 The input longitude bounds must not be equal.
///                 If the lower bound is greater than the upper
///                 bound, the difference between the bounds must
///                 not be an integer multiple of 2*pi.
///
///              3) MXCOR1 must not exceed MNCOR1 by more than 2*pi.
///                 Values that are out of range by no more than
///                 ANGMRG are bracketed to be in range.
///
///
///  FIRST,
///  LAST     are the endpoints of the time interval over which
///           this data set is applicable. These times are
///           expressed as seconds past J2000 TDB.
///
///  NV       is the number of vertices belonging to the plate
///           model.
///
///  VRTCES   is an array of coordinates of the vertices.
///           The Ith vertex occupies elements (1:3,I) of
///           this array.
///
///  NP       is the number of plates in the plate model.
///
///  PLATES   is an array representing the plates of the model.
///           The elements of PLATES are vertex indices. The vertex
///           indices of the Ith plate occupy elements (1:3,I) of
///           this array.
///
///  SPAIXD,
///  SPAIXI   are, respectively, the double precision and integer
///           components of the spatial index of the segment.
///
///           It is strongly recommended that the helper routine
///           DSKMI2 be used to create these arrays. See the
///           $Examples section below.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. This routine operates by side effects.
/// ```
///
/// # Parameters
///
/// ```text
///  See the SPICELIB include files
///
///     dsk02.inc
///     dskdsc.inc
///     dsktol.inc
///
///  for declarations and detailed descriptions of the parameters
///  referenced in this header.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the reference frame name FRAME could not be mapped to
///      an ID code, the error SPICE(FRAMEIDNOTFOUND) is signaled.
///
///  2)  If the segment stop time precedes the start time, the
///      error SPICE(TIMESOUTOFORDER) is signaled.
///
///  3)  If an input longitude value is outside the range
///
///         [ -2*pi - ANGMRG,   2*pi + ANGMRG ]
///
///      the error SPICE(VALUEOUTOFRANGE) is signaled. Longitudes
///      outside of the range by a smaller amount than ANGMRG will be
///      truncated to lie in the interval [-2*pi, 2*pi].
///
///  4)  If the absolute value of the difference between the input
///      maximum longitude and the minimum longitude is more than 2*pi
///      + ANGMRG, the error SPICE(INVALIDLONEXTENT) is signaled.
///      If either longitude bound exceeds the other by an amount
///      between 2*pi and 2*pi+ANGMRG, the larger value will be
///      truncated to the smaller value plus 2*pi.
///
///  5)  If an input latitude value is outside the range
///
///         [ -pi/2 - ANGMRG,   pi/2 + ANGMRG ]
///
///      the error SPICE(VALUEOUTOFRANGE) is signaled. Latitudes
///      outside of the range by a smaller amount than ANGMRG will be
///      truncated to lie in the interval [-pi/2, pi/2].
///
///  6)  If the coordinate system is latitudinal and the lower radius
///      bound is negative, or if the upper radius bound is
///      non-positive, the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  7)  If the coordinate system is latitudinal or planetodetic and
///      the bounds of the latitude, radius or altitude coordinate are
///      out of order, the error SPICE(BOUNDSOUTOFORDER) is signaled.
///
///  8)  If the coordinate system is latitudinal or planetodetic and
///      the lower and upper bounds of the longitude, latitude, radius
///      or altitude coordinate, respectively, are equal, the error
///      SPICE(ZEROBOUNDSEXTENT) is signaled. If the lower
///      longitude bound is greater than the upper bound, and if the
///      difference between the bounds is an integer multiple of 2*pi,
///      the same error is signaled.
///
///  9)  If the coordinate system is planetodetic and the input
///      equatorial radius is non-positive, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///  10) If the coordinate system is planetodetic and the input
///      flattening coefficient is greater than or equal to 1, the
///      error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  11) If the coordinate system is planetodetic, and if the minimum
///      altitude is less than the maximum of
///
///                 2           2
///           {  -(B / A),   -(A / B)  }
///
///      where A and B are the semi-major and semi-minor axis lengths
///      of the reference ellipsoid, the error
///      SPICE(DEGENERATESURFACE) is signaled.
///
///  12) If the coordinate system is rectangular and any coordinate
///      lower bound is greater than or equal to the corresponding
///      upper bound, the error SPICE(BOUNDSOUTOFORDER) is signaled.
///
///  13) If the coordinate system code is not recognized, the error
///      SPICE(NOTSUPPORTED) is signaled.
///
///  14) If any vertex index belonging to an input plate is outside of
///      the range 1:NV, the error SPICE(BADVERTEXINDEX) is signaled.
///
///  15) If NV is less than 1 or greater than MAXVRT, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///  16) If NP is less than 1 or greater than MAXPLT, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///  17) If any voxel grid extent is less than 1 or greater than
///      MAXVOX, the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  18) If the voxel count is greater than MAXVOX, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///  19) If the coarse voxel count is less than 1 or greater than
///      MAXCGR, the error SPICE(VALUEOUTOFRANGE) is signaled.
///
///  20) If the coarse voxel scale is less than 1 or more than
///      the cube root of the fine voxel count, the error
///      SPICE(VALUEOUTOFRANGE) is signaled.
///
///  21) If the cube of the coarse voxel scale does not divide the
///      fine voxel count evenly, the error SPICE(INCOMPATIBLESCALE)
///      is signaled.
///
///  22) If the input data class is not recognized, the error
///      SPICE(NOTSUPPORTED) is signaled.
///
///  23) If an error occurs while writing the segment to the output
///      DSK file, the error is signaled by a routine in the call
///      tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  See argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes a type 2 segment to a DSK file that
///  has been opened for write access.
///
///  Users planning to create DSK files should consider whether the
///  SPICE DSK creation utility MKDSK may be suitable for their needs.
///
///  This routine is supported by the routines DSKMI2 and DSKRB2
///  DSKMI2 simplifies use of this routine by creating the "spatial
///  index" arrays required as inputs by this routine. DSKRB2 computes
///  bounds on the third coordinate of the input plate set.
///
///  Spatial Indexes
///  ===============
///
///  A spatial index is a group of data structures that facilitates
///  rapid high-level computations involving sets of plates. The data
///  structures created by this routine are aggregated into arrays
///  of type INTEGER and type DOUBLE PRECISION.
///
///
///  Voxel grids
///  ===========
///
///  A key geometric computation---probably the most important, as it
///  serves as a foundation for other high-level computations---is
///  finding the intersection of a ray with the plate set. DSK type 2
///  segments use data structures called "voxel grids" as part of
///  their indexing mechanism. There is a "coarse grid": a box that
///  completely encloses a DSK type 2 segment's plate set, and which
///  is composed of identically-sized cubes called "coarse voxels."
///  Each coarse voxel in composed of smaller cubes called "fine
///  voxels." When the term "voxel" is used without qualification, it
///  refers to fine voxels.
///
///  Type 2 DSK segments contain data structures that associate plates
///  with the fine voxels intersected by those plates. These
///  structures enable the type 2 DSK software to rapidly find plates
///  in a given region of space.
///
///  Voxel scales
///  ============
///
///  There are two voxel scales:
///
///  -  The coarse voxel scale is the integer ratio of the
///     edge length of a coarse voxel to the edge length of
///     a fine voxel
///
///  -  The fine voxel scale is the double precision ratio
///     of the edge length of a fine voxel to the average
///     extent of the plates in the input plate set. "Extents"
///     of a plate are the absolute values of the differences
///     between the respective maximum and minimum X, Y, and Z
///     coordinates of the plate's vertices.
///
///  Voxel scales determine the resolution of the voxel grid.
///  Voxel scales must be chosen to satisfy size constraints and
///  provide reasonable plate lookup performance.
///
///  The following considerations apply to spatial indexes of
///  type 2 DSK segments:
///
///     1)  The maximum number of coarse voxels is fixed at MAXCGR
///         (declared in dsk02.inc).
///
///     2)  If there are too few fine voxels, the average number of
///         plates per fine voxel will be very large. This largely
///         negates the performance improvement afforded by having an
///         index. Also, the number of plates per voxel may exceed
///         limits imposed by DSK subroutines that use static arrays.
///
///     3)  If there are too many fine voxels, the average number of
///         voxels intersected by a given plate may be too large for
///         all the plate-voxel associations to be stored. In
///         addition, the time needed to examine the plate lists for
///         each voxel (including the empty ones) may become quite
///         large, again negating the value of the index.
///
///  In many cases, voxel scales yielding optimum performance must be
///  determined by experiment. However, the following heuristics can
///  provide reasonable starting values:
///
///     Let NP be the number of plates. Let FS be the fine voxel
///     scale. Then a reasonable value of FS may be
///
///                (0.25D0)
///        FS =  NP       / 8.D0
///
///     In general, FS should not smaller than 1.
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
///  1) Create a three-segment DSK file using plate model data for
///     Phobos. Use latitudinal, rectangular, and planetodetic
///     coordinates in the respective segments. This is not a
///     realistic example, but it serves to demonstrate use of
///     the supported coordinate systems.
///
///     Use the DSK kernel below to provide, for simplicity, the
///     input plate and vertex data. The selected input file has one
///     segment.
///
///        phobos_3_3.bds
///
///
///     Example code begins here.
///
///
///     C
///     C     Example program for DSKW02, DSKMI2, and DSKRB2
///     C
///     C        Create a three-segment DSK file using plate model
///     C        data for Phobos. Use latitudinal, rectangular, and
///     C        planetodetic coordinates in the respective segments.
///     C
///     C        For simplicity, use an existing DSK file to provide
///     C        the input plate and vertex data. The selected input
///     C        file has one segment.
///     C
///     C           Version 1.0.0 22-JAN-2016 (NJB)
///     C
///           PROGRAM DSKW02_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      JYEAR
///           DOUBLE PRECISION      PI
///     C
///     C     Local parameters
///     C
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               NSEG
///           PARAMETER           ( NSEG   = 3 )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 20 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///           INTEGER               NCOR
///           PARAMETER           ( NCOR   = 4 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(NAMLEN)    CORNAM ( NCOR )
///           CHARACTER*(FILSIZ)    DSK
///           CHARACTER*(FRNMLN)    FRAME
///           CHARACTER*(FILSIZ)    INDSK
///           CHARACTER*(LNSIZE)    LINE
///     C
///     C     Note: the values of MAXVRT and MAXPLT declared
///     C     in dsk02.inc, and the integer spatial index
///     C     dimension SPAISZ are very large. Smaller buffers
///     C     can be used for most applications.
///     C
///           DOUBLE PRECISION      CORPAR ( NSYPAR )
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      FINSCL
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///           DOUBLE PRECISION      MNCOR1
///           DOUBLE PRECISION      MNCOR2
///           DOUBLE PRECISION      MNCOR3
///           DOUBLE PRECISION      MXCOR1
///           DOUBLE PRECISION      MXCOR2
///           DOUBLE PRECISION      MXCOR3
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      RP
///           DOUBLE PRECISION      SPAIXD ( IXDFIX )
///           DOUBLE PRECISION      VRTCES ( 3, MAXVRT )
///
///           INTEGER               CENTER
///           INTEGER               CORSCL
///           INTEGER               CORSYS
///           INTEGER               DCLASS
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               INHAN
///           INTEGER               NP
///           INTEGER               NV
///           INTEGER               PLATES ( 3, MAXPLT )
///           INTEGER               SEGNO
///           INTEGER               SPAIXI ( SPAISZ )
///           INTEGER               SURFID
///           INTEGER               VOXPSZ
///           INTEGER               VOXLSZ
///           INTEGER               WORK   ( 2, MAXCEL )
///           INTEGER               WORKSZ
///
///           LOGICAL               FOUND
///     C
///     C     Saved variables
///     C
///     C     Save all large arrays to avoid stack problems.
///     C
///           SAVE
///     C
///     C     Initial values
///     C
///           DATA                  CORNAM / 'radius',
///          .                               'Z-coordinate',
///          .                               'Z-coordinate',
///          .                               'altitude'     /
///
///     C
///     C     Assign names of input and output DSK files.
///     C
///           INDSK = 'phobos_3_3.bds'
///           DSK   = 'phobos_3_3_3seg.bds'
///     C
///     C     Open input DSK for read access; find first segment.
///     C
///           CALL DASOPR ( INDSK, INHAN )
///           CALL DLABFS ( INHAN, DLADSC, FOUND )
///     C
///     C     Fetch vertices and plates from input DSK file.
///     C
///           WRITE (*,*) 'Reading input data...'
///
///           CALL DSKV02 ( INHAN, DLADSC, 1, MAXVRT, NV, VRTCES )
///           CALL DSKP02 ( INHAN, DLADSC, 1, MAXPLT, NP, PLATES )
///
///           WRITE (*,*) 'Done.'
///     C
///     C     Set input array sizes required by DSKMI2.
///     C
///           VOXPSZ = MAXVXP
///           VOXLSZ = MXNVLS
///           WORKSZ = MAXCEL
///     C
///     C     Set fine and coarse voxel scales. (These usually
///     C     need to determined by experimentation.)
///     C
///           FINSCL = 5.D0
///           CORSCL = 4
///     C
///     C     Open a new DSK file.
///     C
///           CALL DSKOPN ( DSK, DSK, 0, HANDLE )
///     C
///     C     Create three segments and add them to the file.
///     C
///           DO SEGNO = 1, NSEG
///     C
///     C        Create spatial index.
///     C
///              WRITE (*,*) 'Creating segment ', SEGNO
///              WRITE (*,*) 'Creating spatial index...'
///
///              CALL DSKMI2 ( NV,     VRTCES, NP,     PLATES, FINSCL,
///          .                 CORSCL, WORKSZ, VOXPSZ, VOXLSZ, .TRUE.,
///          .                 SPAISZ, WORK,   SPAIXD, SPAIXI        )
///
///              WRITE (*,*) 'Done.'
///     C
///     C        Set up inputs describing segment attributes:
///     C
///     C        - Central body: Phobos
///     C        - Surface ID code: user's choice.
///     C          We use the segment number here.
///     C        - Data class: general (arbitrary) shape
///     C        - Body-fixed reference frame
///     C        - Time coverage bounds (TBD)
///     C
///              CENTER = 401
///              SURFID = SEGNO
///              DCLASS = GENCLS
///              FRAME  = 'IAU_PHOBOS'
///
///              FIRST = -50 * JYEAR()
///              LAST  =  50 * JYEAR()
///     C
///     C        Set the coordinate system and coordinate system
///     C        bounds based on the segment index.
///     C
///     C        Zero out the coordinate parameters to start.
///     C
///              CALL CLEARD ( NSYPAR, CORPAR )
///
///              IF ( SEGNO .EQ. 1 ) THEN
///     C
///     C           Use planetocentric latitudinal coordinates. Set
///     C           the longitude and latitude bounds.
///     C
///                 CORSYS = LATSYS
///
///                 MNCOR1 = -PI()
///                 MXCOR1 =  PI()
///                 MNCOR2 = -PI()/2
///                 MXCOR2 =  PI()/2
///
///              ELSE IF ( SEGNO .EQ. 2 ) THEN
///     C
///     C           Use rectangular coordinates. Set the
///     C           X and Y bounds.
///     C
///     C           The bounds shown here were derived from
///     C           the plate data. They lie slightly outside
///     C           of the range spanned by the plates.
///     C
///                 CORSYS = RECSYS
///
///                 MNCOR1 = -1.3D0
///                 MXCOR1 =  1.31D0
///                 MNCOR2 = -1.21D0
///                 MXCOR2 =  1.2D0
///
///              ELSE
///     C
///     C           Set the coordinate system to planetodetic.
///     C
///                 CORSYS    = PDTSYS
///
///                 MNCOR1    = -PI()
///                 MXCOR1    =  PI()
///                 MNCOR2    = -PI()/2
///                 MXCOR2    =  PI()/2
///     C
///     C           We'll use equatorial and polar radii from
///     C           pck00010.tpc. These normally would be fetched
///     C           at run time, but for simplicity, we'll use
///     C           hard-coded values.
///
///                 RE        = 13.0D0
///                 RP        =  9.1D0
///                 F         = ( RE - RP ) / RE
///
///                 CORPAR(1) = RE
///                 CORPAR(2) = F
///
///              END IF
///     C
///     C        Compute plate model radius bounds.
///     C
///              LINE = 'Computing # bounds of plate set...'
///
///              CALL REPMC ( LINE, '#', CORNAM(CORSYS), LINE )
///              WRITE (*,*) LINE
///
///              CALL DSKRB2 ( NV,     VRTCES, NP,     PLATES,
///          .                 CORSYS, CORPAR, MNCOR3, MXCOR3 )
///
///              WRITE (*,*) 'Done.'
///     C
///     C        Write the segment to the file.
///     C
///              WRITE (*,*) 'Writing segment...'
///
///              CALL DSKW02 ( HANDLE,
///          .                 CENTER, SURFID, DCLASS, FRAME,  CORSYS,
///          .                 CORPAR, MNCOR1, MXCOR1, MNCOR2, MXCOR2,
///          .                 MNCOR3, MXCOR3, FIRST,  LAST,   NV,
///          .                 VRTCES, NP,     PLATES, SPAIXD, SPAIXI )
///
///              WRITE (*,*) 'Done.'
///
///           END DO
///     C
///     C     Segregate the data records in the DSK file and
///     C     close the file.
///     C
///           WRITE (*,*) 'Segregating and closing DSK file...'
///
///           CALL DSKCLS ( HANDLE, .TRUE. )
///
///           WRITE (*,*) 'Done.'
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Reading input data...
///      Done.
///      Creating segment            1
///      Creating spatial index...
///      Done.
///      Computing radius bounds of plate set...
///      Done.
///      Writing segment...
///      Done.
///      Creating segment            2
///      Creating spatial index...
///      Done.
///      Computing Z-coordinate bounds of plate set...
///      Done.
///      Writing segment...
///      Done.
///      Creating segment            3
///      Creating spatial index...
///      Done.
///      Computing altitude bounds of plate set...
///      Done.
///      Writing segment...
///      Done.
///      Segregating and closing DSK file...
///      Done.
///
///
///     Note that after run completion, a new DSK exists in the output
///     directory.
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
/// -    SPICELIB Version 1.0.1, 28-AUG-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///         Added solution to code example.
///
///         Corrected description of coverage requirements for class 1
///         segments.
///
///         Deleted paragraph saying that, except for changes made to
///         move longitude values into range, the values are stored in
///         segment "as is."
///
/// -    SPICELIB Version 1.0.0, 04-MAR-2017 (NJB)
///
///         Fixed some comment typos.
///
///      10-OCT-2016 (NJB)
///
///         New error checks on inputs were added.
///
///      07-MAR-2016 (NJB)
///
///         New error checks on inputs were added.
///
///         Argument list change: spatial index is now passed in
///         as two arrays: SPAIXD and SPAIXI.
///
///         Argument CORPAR was added.
///
///         Double precision data are now written to the output
///         segment before integer data.
///
///         22-AUG-2012 (NJB)
///
///            Bug fix: corrected upper bound in test for
///            vertex count.
///
///         13-MAY-2010 (NJB)
///
///            Updated to reflect new type 2 segment design: normal
///            vectors, plate centers, and lengths of longest plate sides
///            are no longer stored in these segments.
///
///         03-APR-2010 (NJB)
///
///            New interface; general coordinates are supported. Time
///            bounds, surface ID, data class, and bounds of third
///            coordinate have been added. Albedo inputs have been
///            deleted.
///
///         09-OCT-2009 (NJB)
///
///            Header was added.
///
///         31-OCT-2006 (NJB)
///
///            Input arguments CGSCAL and VTXBDS were removed.
///
///         27-JUN-2006 (NJB)
///
///            Initial version.
/// ```
pub fn dskw02(
    ctx: &mut SpiceContext,
    handle: i32,
    center: i32,
    surfid: i32,
    dclass: i32,
    frame: &str,
    corsys: i32,
    corpar: &[f64],
    mncor1: f64,
    mxcor1: f64,
    mncor2: f64,
    mxcor2: f64,
    mncor3: f64,
    mxcor3: f64,
    first: f64,
    last: f64,
    nv: i32,
    vrtces: &[[f64; 3]],
    np: i32,
    plates: &[[i32; 3]],
    spaixd: &[f64],
    spaixi: &[i32],
) -> crate::Result<()> {
    DSKW02(
        handle,
        center,
        surfid,
        dclass,
        frame.as_bytes(),
        corsys,
        corpar,
        mncor1,
        mxcor1,
        mncor2,
        mxcor2,
        mncor3,
        mxcor3,
        first,
        last,
        nv,
        vrtces.as_flattened(),
        np,
        plates.as_flattened(),
        spaixd,
        spaixi,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKW02 ( DSK, write type 2 segment )
pub fn DSKW02(
    HANDLE: i32,
    CENTER: i32,
    SURFID: i32,
    DCLASS: i32,
    FRAME: &[u8],
    CORSYS: i32,
    CORPAR: &[f64],
    MNCOR1: f64,
    MXCOR1: f64,
    MNCOR2: f64,
    MXCOR2: f64,
    MNCOR3: f64,
    MXCOR3: f64,
    FIRST: f64,
    LAST: f64,
    NV: i32,
    VRTCES: &[f64],
    NP: i32,
    PLATES: &[i32],
    SPAIXD: &[f64],
    SPAIXI: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let VRTCES = DummyArray2D::new(VRTCES, 1..=3, 1..=NV);
    let PLATES = DummyArray2D::new(PLATES, 1..=3, 1..=NP);
    let SPAIXD = DummyArray::new(SPAIXD, 1..);
    let SPAIXI = DummyArray::new(SPAIXI, 1..);
    let mut A: f64 = 0.0;
    let mut ALTLIM: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut R: f64 = 0.0;
    let mut SEGBDS = StackArray2D::<f64, 4>::new(1..=2, 1..=2);
    let mut VOXORI = StackArray::<f64, 3>::new(1..=3);
    let mut VOXSIZ: f64 = 0.0;
    let mut VTXBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
    let mut CGRSCL: i32 = 0;
    let mut FRMCDE: i32 = 0;
    let mut K: i32 = 0;
    let mut NCGR: i32 = 0;
    let mut NVXTOT: i32 = 0;
    let mut PVOXPL: i32 = 0;
    let mut PVOXPT: i32 = 0;
    let mut PVTXPL: i32 = 0;
    let mut PVTXPT: i32 = 0;
    let mut Q: i32 = 0;
    let mut VGREXT = StackArray::<i32, 3>::new(1..=3);
    let mut VOXNPT: i32 = 0;
    let mut VOXNPL: i32 = 0;
    let mut VTXNPL: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKW02", ctx)?;

    //
    // Map the input reference frame name to an ID code.
    //
    NAMFRM(FRAME, &mut FRMCDE, ctx)?;

    if (FRMCDE == 0) {
        SETMSG(b"Input reference frame # could not be mapped to an ID code. The frame name might be misspelled, or possibly a required frame kernel was not loaded. ", ctx);
        ERRCH(b"#", FRAME, ctx);
        SIGERR(b"SPICE(FRAMEIDNOTFOUND)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // Make sure the time bounds are in order.
    //
    if (LAST <= FIRST) {
        SETMSG(
            b"Segment time bounds must be increasing; bounds were #:#.",
            ctx,
        );
        ERRDP(b"#", FIRST, ctx);
        ERRDP(b"#", LAST, ctx);
        SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // If applicable, check segment boundaries. Check the
    // coordinate system as well.
    //
    if ((CORSYS == LATSYS) || (CORSYS == PDTSYS)) {
        //
        // Reject invalid latitudes and longitudes. Move
        // values that are slightly out of range into range.
        //
        // Longitude bounds must be distinct.
        //
        if (MNCOR1 == MXCOR1) {
            SETMSG(b"Minimum longitude # radians (# degrees) was equal to maximum longitude. Longitude bounds must be distinct.", ctx);
            ERRDP(b"#", MNCOR1, ctx);
            ERRDP(b"#", (MNCOR1 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(ZEROBOUNDSEXTENT)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        //
        // Check minimum longitude.
        //
        if ((MNCOR1 < (-TWOPI(ctx) - ANGMRG)) || (MNCOR1 > (TWOPI(ctx) - ANGMRG))) {
            SETMSG(b"Minimum longitude # radians (# degrees) was outside of valid range [-2*pi, 2*pi - ANGMRG]", ctx);
            ERRDP(b"#", MNCOR1, ctx);
            ERRDP(b"#", (MNCOR1 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        //
        // The minimum longitude is too small by ANGMRG, at worst.
        // Make it greater than or equal to -2*pi.
        //
        SEGBDS[[1, 1]] = intrinsics::DMAX1(&[-TWOPI(ctx), MNCOR1]);

        //
        // Check maximum longitude.
        //
        if ((MXCOR1 < (-TWOPI(ctx) + ANGMRG)) || (MXCOR1 > (TWOPI(ctx) + ANGMRG))) {
            SETMSG(b"Maximum longitude # radians (# degrees) was outside of valid range [-2*pi+ANGMRG, 2*pi]", ctx);
            ERRDP(b"#", MXCOR1, ctx);
            ERRDP(b"#", (MXCOR1 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        //
        // The maximum longitude is too large by ANGMRG, at worst.
        // Make it less than or equal to 2*pi.
        //
        SEGBDS[[2, 1]] = intrinsics::DMIN1(&[TWOPI(ctx), MXCOR1]);

        //
        // The longitude extent cannot exceed 2*pi.
        //
        if (MXCOR1 > ((MNCOR1 + TWOPI(ctx)) + ANGMRG)) {
            SETMSG(
                b"Longitude bounds #:# radians (#:# degrees) are too far apart.",
                ctx,
            );
            ERRDP(b"#", MXCOR1, ctx);
            ERRDP(b"#", MXCOR2, ctx);
            ERRDP(b"#", (MXCOR1 * DPR(ctx)), ctx);
            ERRDP(b"#", (MXCOR2 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(INVALIDLONEXTENT)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        if (MXCOR1 < ((MNCOR1 - TWOPI(ctx)) - ANGMRG)) {
            SETMSG(
                b"Longitude bounds #:# radians (#:# degrees) are too far apart.",
                ctx,
            );
            ERRDP(b"#", MXCOR1, ctx);
            ERRDP(b"#", MXCOR2, ctx);
            ERRDP(b"#", (MXCOR1 * DPR(ctx)), ctx);
            ERRDP(b"#", (MXCOR2 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(INVALIDLONEXTENT)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        if (SEGBDS[[2, 1]] > SEGBDS[[1, 1]]) {
            //
            // The upper bound exceeds the lower by at most 2*pi + ANGMRG.
            // Trim the upper bound to make the difference no more than
            // 2*pi.
            //
            SEGBDS[[2, 1]] = intrinsics::DMIN1(&[SEGBDS[[2, 1]], (SEGBDS[[1, 1]] + TWOPI(ctx))]);
        } else if (SEGBDS[[2, 1]] < SEGBDS[[1, 1]]) {
            //
            // The lower bound exceeds the upper by at most 2*pi + ANGMRG.
            // Advance the upper bound, if necessary, to make the
            // difference no more than 2*pi.
            //
            SEGBDS[[2, 1]] = intrinsics::DMAX1(&[SEGBDS[[2, 1]], (SEGBDS[[1, 1]] - TWOPI(ctx))]);
        }

        //
        // Make sure the adjusted longitude bounds don't describe an
        // interval that could be interpreted as having length zero,
        // if the bounds were placed in order. If the lower bound is
        // greater than the upper bound, then the difference between
        // the bounds must not be an integer multiple of 2*pi.
        //
        if ((SEGBDS[[2, 1]] == SEGBDS[[1, 1]]) || (SEGBDS[[2, 1]] == (SEGBDS[[1, 1]] - TWOPI(ctx))))
        {
            SETMSG(b"After adjustment, minimum longitude # radians (# degrees) was equal to maximum longitude. Longitude bounds must be distinct.", ctx);
            ERRDP(b"#", SEGBDS[[1, 1]], ctx);
            ERRDP(b"#", (MNCOR1 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(ZEROBOUNDSEXTENT)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        //
        // Check minimum latitude.
        //
        if ((MNCOR2 < (-HALFPI(ctx) - ANGMRG)) || (MNCOR2 > (HALFPI(ctx) - ANGMRG))) {
            SETMSG(b"Minimum latitude # radians (# degrees) was outside of valid range [-pi/2, pi/2 - ANGMRG]", ctx);
            ERRDP(b"#", MNCOR2, ctx);
            ERRDP(b"#", (MNCOR2 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        //
        // Trim the lower latitude bound to make it at least -pi/2.
        //
        SEGBDS[[1, 2]] = intrinsics::DMAX1(&[-HALFPI(ctx), MNCOR2]);

        //
        // Check maximum latitude.
        //
        if ((MXCOR2 < (-HALFPI(ctx) + ANGMRG)) || (MXCOR2 > (HALFPI(ctx) + ANGMRG))) {
            SETMSG(b"Maximum latitude # radians (# degrees) was outside of valid range [-pi/2+ANGMRG, pi/2]", ctx);
            ERRDP(b"#", MXCOR2, ctx);
            ERRDP(b"#", (MXCOR2 * DPR(ctx)), ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        //
        // Trim the upper latitude bound to make it no more than -pi/2.
        //
        SEGBDS[[1, 2]] = intrinsics::DMAX1(&[-HALFPI(ctx), MNCOR2]);
        SEGBDS[[2, 2]] = intrinsics::DMIN1(&[HALFPI(ctx), MXCOR2]);

        //
        // The latitude bounds must be in order.
        //
        if (MXCOR2 < MNCOR2) {
            SETMSG(b"Latitude bounds # and # are out of order.", ctx);
            ERRDP(b"#", MNCOR2, ctx);
            ERRDP(b"#", MXCOR2, ctx);
            SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        if (CORSYS == LATSYS) {
            //
            // The coordinate system is latitudinal. Check radius
            // bounds.
            //
            if (MNCOR3 < 0.0) {
                SETMSG(b"Radius lower bound must be non-negative but was #.", ctx);
                ERRDP(b"#", MNCOR3, ctx);
                SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
                CHKOUT(b"DSKW02", ctx)?;
                return Ok(());
            }

            if (MXCOR3 <= 0.0) {
                SETMSG(
                    b"Radius upper bound must be strictly positive but was #.",
                    ctx,
                );
                ERRDP(b"#", MXCOR3, ctx);
                SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
                CHKOUT(b"DSKW02", ctx)?;
                return Ok(());
            }
        }

        if (CORSYS == PDTSYS) {
            //
            // The coordinate system is planetodetic. Check the coordinate
            // parameters as well.
            //
            if (CORPAR[1] <= 0.0) {
                SETMSG(
                    b"Equatorial radius was #; this radius must be strictly positive.",
                    ctx,
                );
                ERRDP(b"#", CORPAR[1], ctx);
                SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
                CHKOUT(b"DSKW02", ctx)?;
                return Ok(());
            }

            if (CORPAR[2] >= 1.0) {
                SETMSG(
                    b"Flattening coefficient was #; this value must be strictly less than 1.",
                    ctx,
                );
                ERRDP(b"#", CORPAR[2], ctx);
                SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
                CHKOUT(b"DSKW02", ctx)?;
                return Ok(());
            }

            //
            // Make sure the surface of minimum altitude is smooth and
            // non-self-intersecting.
            //
            A = CORPAR[1];
            B = (A * ((1 as f64) - CORPAR[2]));

            ALTLIM = intrinsics::DMAX1(&[-(f64::powi(A, 2) / B), -(f64::powi(B, 2) / A)]);

            if (MNCOR3 <= ALTLIM) {
                SETMSG(b"Reference ellipsoid has semi-axis lengths # and #. The minimum altitude was #. The minimum altitude is required to be greater than the maximum of {-(A**2)/B, -(B**2)/A}, which is #.", ctx);
                ERRDP(b"#", A, ctx);
                ERRDP(b"#", B, ctx);
                ERRDP(b"#", MNCOR3, ctx);
                ERRDP(b"#", ALTLIM, ctx);
                SIGERR(b"SPICE(DEGENERATESURFACE)", ctx)?;
                CHKOUT(b"DSKW02", ctx)?;
                return Ok(());
            }
        }

        //
        // The bounds of the third coordinate, whether radius or altitude,
        // must be in order and must have positive extent.
        //
        if (MXCOR3 < MNCOR3) {
            if (CORSYS == LATSYS) {
                SETMSG(b"Radius bounds # and # are out of order", ctx);
            } else {
                SETMSG(b"Altitude bounds # and # are out of order.", ctx);
            }

            ERRDP(b"#", MNCOR3, ctx);
            ERRDP(b"#", MXCOR3, ctx);
            SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        if (MXCOR3 == MNCOR3) {
            if (CORSYS == LATSYS) {
                SETMSG(
                    b"Radius bounds # and # must have positive extent but are equal.",
                    ctx,
                );
            } else {
                SETMSG(
                    b"Altitude bounds # and # must have positive extent but are equal.",
                    ctx,
                );
            }

            ERRDP(b"#", MNCOR3, ctx);
            ERRDP(b"#", MXCOR3, ctx);
            SIGERR(b"SPICE(ZEROBOUNDSEXTENT)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }
    } else if (CORSYS == RECSYS) {
        //
        // All coordinate bounds must be in strictly increasing order.
        //
        if (((MXCOR1 <= MNCOR1) || (MXCOR2 <= MNCOR2)) || (MXCOR3 <= MNCOR3)) {
            SETMSG(b"Rectangular coordinate bounds must be strictly increasing in each dimension. The bounds were:  X = #:#; Y = #:#; Z = #:#.", ctx);
            ERRDP(b"#", MNCOR1, ctx);
            ERRDP(b"#", MXCOR1, ctx);
            ERRDP(b"#", MNCOR2, ctx);
            ERRDP(b"#", MXCOR2, ctx);
            ERRDP(b"#", MNCOR3, ctx);
            ERRDP(b"#", MXCOR3, ctx);
            SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }

        SEGBDS[[1, 1]] = MNCOR1;
        SEGBDS[[2, 1]] = MXCOR1;
        SEGBDS[[1, 2]] = MNCOR2;
        SEGBDS[[2, 2]] = MXCOR2;
    } else {
        SETMSG(b"Coordinate system code # is not recognized.", ctx);
        ERRINT(b"#", CORSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // Check the data class.
    //
    if ((DCLASS < 1) || (DCLASS > 2)) {
        SETMSG(b"Data class # is not recognized.", ctx);
        ERRINT(b"#", DCLASS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // Check NV and NP.
    //
    // Note that we don't apply Euler's law, since the data
    // set need not represent a complete surface.
    //
    if ((NV < 1) || (NV > MAXVRT)) {
        SETMSG(b"Vertex count NV = #; count must be in the range 1:#.", ctx);
        ERRINT(b"#", NV, ctx);
        ERRINT(b"#", MAXVRT, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    if ((NP < 1) || (NP > MAXPLT)) {
        SETMSG(b"Plate count NP = #; count must be in the range 1:#.", ctx);
        ERRINT(b"#", NP, ctx);
        ERRINT(b"#", MAXPLT, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // Check the vertex indices in the plates.
    //
    for I in 1..=NP {
        for J in 1..=3 {
            K = PLATES[[J, I]];

            if ((K < 1) || (K > NV)) {
                SETMSG(b"Vertex index # of plate # was #; vertex indices must be in the range 1:NV. The input NV = #.", ctx);
                ERRINT(b"#", J, ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", K, ctx);
                ERRINT(b"#", NV, ctx);
                SIGERR(b"SPICE(BADVERTEXINDEX)", ctx)?;
                CHKOUT(b"DSKW02", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Locate the spatial index elements. Some of the elements are at
    // fixed addresses; for others the addresses must be calculated.
    //
    // The two components of the spatial index together contain the
    // following items:
    //
    //    VGREXT      is an array containing the extents of the voxel
    //                grid in the X, Y, and Z directions of the
    //                body-fixed frame. The extents are measured as
    //                voxel counts.
    //
    //    ORIGIN      is the position, in the body-fixed, body-centered
    //                reference frame associated with BODY, of the
    //                origin of the both the fine and coarse voxel grids
    //                associated with this model.
    //
    //    VOXSIZ      is the voxel edge length in km.
    //
    //    CGRSCL      is the coarse voxel grid scale factor: the edge
    //                length of each coarse voxel is scaled up from the
    //                length of a fine voxel edge by this factor.
    //
    //    CGRPTR      is an array of pointers associated with this
    //                model's coarse voxel grid; these pointers map
    //                one-dimensional coarse voxel indices to start
    //                indices in the fine voxel pointer list.
    //
    //    VOXNPT      is the cardinality of the fine voxel pointer list.
    //
    //    VOXPTR      is the fine voxel pointer list. For each fine
    //                voxel belonging to a non-empty coarse voxel, there
    //                is a pointer in this list that identifies the
    //                start index in VOXPLT of the list of plate indices
    //                associated with this fine voxel.
    //
    //                The start index in VOXPTR of the set of pointers
    //                associated with a coarse voxel is given by the
    //                element of CGRPTR associated with that coarse
    //                voxel.
    //
    //                Within a given coarse voxel, each fine voxel has
    //                an associated one-dimensional offset from the
    //                corner of the coarse voxel closest to the origin
    //                of the voxel grids. This offset gives the location
    //                in VOXPTR of the plate list pointer for the fine
    //                voxel.
    //
    //    VOXNPL      is the cardinality of the plate list of the fine
    //                voxel-plate mapping.
    //
    //    VOXPLT      is the plate list of the fine voxel-plate mapping.
    //
    //    VTXPTR      is the vertex pointer list.
    //
    //    VTXNPL      is the cardinality of the plate list of the
    //                vertex-plate mapping.
    //
    //
    //
    // Extract double precision elements of the spatial index.
    //
    MOVED(SPAIXD.subarray(SIVTBD), 6, VTXBDS.as_slice_mut());
    VEQU(SPAIXD.subarray(SIVXOR), VOXORI.as_slice_mut());

    VOXSIZ = SPAIXD[SIVXSZ];

    //
    // Extract scalars and small fixed-size arrays from the integer
    // component of the spatial index.
    //
    // Fetch grid extents (in units of whole voxels):
    //
    MOVEI(SPAIXI.subarray(SIVGRX), 3, VGREXT.as_slice_mut());

    //
    // Fetch coarse grid scale, voxel pointer count, and voxel-plate
    // list count.
    //
    CGRSCL = SPAIXI[SICGSC];
    VOXNPT = SPAIXI[SIVXNP];
    VOXNPL = SPAIXI[SIVXNL];

    //
    // Create a pointer to the voxel-plate pointer array.
    //
    PVOXPT = (SICGRD + MAXCGR);

    //
    // Create a pointer to the voxel-plate list array.
    //
    PVOXPL = (PVOXPT + VOXNPT);

    //
    // Create a pointer to the vertex pointer array.
    //
    PVTXPT = (PVOXPL + VOXNPL);

    //
    // Create a pointer to the vertex-plate list array.
    //
    PVTXPL = (PVTXPT + NV);

    //
    // Fetch vertex-plate list size.
    //
    VTXNPL = SPAIXI[SIVTNL];

    //
    // Check the input parameters.
    //
    //
    //
    // Make sure the voxel grid extents are within range.
    //
    for I in 1..=3 {
        if ((VGREXT[I] < 1) || (VGREXT[I] > MAXVOX)) {
            SETMSG(
                b"Voxel grid extents are = (#, #, #); all be in the range 1:#.",
                ctx,
            );
            ERRINT(b"#", VGREXT[1], ctx);
            ERRINT(b"#", VGREXT[2], ctx);
            ERRINT(b"#", VGREXT[3], ctx);
            ERRINT(b"#", MAXVOX, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"DSKW02", ctx)?;
            return Ok(());
        }
    }

    //
    // Make sure the number of voxels NVXTOT is within range.
    //
    NVXTOT = ((VGREXT[1] * VGREXT[2]) * VGREXT[3]);

    if (NVXTOT > MAXVOX) {
        SETMSG(
            b"Fine voxel count NVXTOT = #; count must be in the range 1:#.",
            ctx,
        );
        ERRINT(b"#", NVXTOT, ctx);
        ERRINT(b"#", MAXVOX, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // Check the coarse voxel scale. It must be at least 1, and its
    // cube must not exceed the fine voxel count.
    //
    if ((CGRSCL < 1) || ((CGRSCL as f64) > f64::powf(NVXTOT as f64, (1.0 / 3 as f64)))) {
        SETMSG(b"Coarse voxel scale = #; scale must be in the range 1:NVXTOT**3, where NVXTOT is the total fine voxel count. In this case, NVXTOT = #.", ctx);
        ERRINT(b"#", CGRSCL, ctx);
        ERRINT(b"#", NVXTOT, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // The cube of the coarse scale must divide the total voxel count
    // evenly.
    //
    Q = (NVXTOT / intrinsics::pow(CGRSCL, 3));
    R = (NVXTOT - (Q * intrinsics::pow(CGRSCL, 3))) as f64;

    if (R != 0 as f64) {
        SETMSG(b"Coarse voxel scale = #; the cube of the scale must divide NVXTOT evenly, where NVXTOT is the total  fine voxel count. In this case, NVXTOT = #.", ctx);
        ERRINT(b"#", CGRSCL, ctx);
        ERRINT(b"#", NVXTOT, ctx);
        SIGERR(b"SPICE(INCOMPATIBLESCALE)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // NCGR        is the number of voxels in the coarse voxel grid
    //             associated with this model. Since each coarse voxel
    //             is a cube containing an integer number of fine
    //             voxels, this number is determined by NVXTOT and
    //             CGRSCL.
    //
    NCGR = (NVXTOT / intrinsics::pow(CGRSCL, 3));

    //
    // Make sure NCGR is within range.
    //
    if ((NCGR < 1) || (NCGR > MAXCGR)) {
        SETMSG(
            b"Coarse voxel count = #; count must be in the range 1:#.",
            ctx,
        );
        ERRINT(b"#", NCGR, ctx);
        ERRINT(b"#", MAXCGR, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // Start a new DLA segment.
    //
    DLABNS(HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DSKW02", ctx)?;
        return Ok(());
    }

    //
    // Write the d.p. data to the segment first. In the segregated
    // segment, d.p. data will precede integer data, so less
    // rearrangement will occur this way.
    //
    //
    // First, fill in the DSK segment descriptor.
    //
    CLEARD(DSKDSZ, DESCR.as_slice_mut());

    DESCR[SRFIDX] = SURFID as f64;
    DESCR[CTRIDX] = CENTER as f64;
    DESCR[CLSIDX] = DCLASS as f64;
    DESCR[TYPIDX] = 2 as f64;
    DESCR[FRMIDX] = FRMCDE as f64;
    DESCR[SYSIDX] = CORSYS as f64;

    MOVED(CORPAR.as_slice(), NSYPAR, DESCR.subarray_mut(PARIDX));

    DESCR[MN1IDX] = SEGBDS[[1, 1]];
    DESCR[MX1IDX] = SEGBDS[[2, 1]];
    DESCR[MN2IDX] = SEGBDS[[1, 2]];
    DESCR[MX2IDX] = SEGBDS[[2, 2]];
    DESCR[MN3IDX] = MNCOR3;
    DESCR[MX3IDX] = MXCOR3;
    DESCR[BTMIDX] = FIRST;
    DESCR[ETMIDX] = LAST;

    //
    // Now write the descriptor into the segment.
    //
    DASADD(HANDLE, DSKDSZ, DESCR.as_slice(), ctx)?;

    //
    // Add the voxel grid origin and voxel size.
    // Finish with the vertex data.
    //
    DASADD(HANDLE, 6, VTXBDS.as_slice(), ctx)?;
    DASADD(HANDLE, 3, VOXORI.as_slice(), ctx)?;
    DASADD(HANDLE, 1, &[VOXSIZ], ctx)?;
    DASADD(HANDLE, (3 * NV), VRTCES.as_slice(), ctx)?;

    //
    // Next add the integer data to the segment.
    //
    // NV is the number of vertices.
    // NP is the number of plates.
    // NVXTOT is the number of voxels in the spatial index.
    //
    DASADI(HANDLE, 1, &[NV], ctx)?;
    DASADI(HANDLE, 1, &[NP], ctx)?;
    DASADI(HANDLE, 1, &[NVXTOT], ctx)?;
    DASADI(HANDLE, 3, VGREXT.as_slice(), ctx)?;
    DASADI(HANDLE, 1, &[CGRSCL], ctx)?;
    DASADI(HANDLE, 1, &[VOXNPT], ctx)?;
    DASADI(HANDLE, 1, &[VOXNPL], ctx)?;
    DASADI(HANDLE, 1, &[VTXNPL], ctx)?;
    DASADI(HANDLE, (3 * NP), PLATES.as_slice(), ctx)?;
    DASADI(HANDLE, VOXNPT, SPAIXI.subarray(PVOXPT), ctx)?;
    DASADI(HANDLE, VOXNPL, SPAIXI.subarray(PVOXPL), ctx)?;
    DASADI(HANDLE, NV, SPAIXI.subarray(PVTXPT), ctx)?;
    DASADI(HANDLE, VTXNPL, SPAIXI.subarray(PVTXPL), ctx)?;
    DASADI(HANDLE, NCGR, SPAIXI.subarray(SICGRD), ctx)?;

    //
    // End the segment.
    //
    DLAENS(HANDLE, ctx)?;

    CHKOUT(b"DSKW02", ctx)?;
    Ok(())
}
