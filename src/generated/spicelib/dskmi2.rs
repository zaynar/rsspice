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

/// DSK, make spatial index for type 2 segment
///
/// Make spatial index for a DSK type 2 segment. The index is
/// returned as a pair of arrays, one of type INTEGER and one of type
/// DOUBLE PRECISION. These arrays are suitable for use with the DSK
/// type 2 writer DSKW02.
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
///  IXDFIX     P   Size of fixed-size portion of d.p. index component.
///  IXIFIX     P   Size of fixed-size portion of integer index
///                 component.
///  NV         I   Number of vertices.
///  VRTCES     I   Vertices.
///  NP         I   Number of plates.
///  PLATES     I   Plates.
///  FINSCL     I   Fine voxel scale.
///  CORSCL     I   Coarse voxel scale.
///  WORKSZ     I   Workspace size.
///  VOXPSZ     I   Voxel-plate pointer array size.
///  VOXLSZ     I   Voxel-plate list array size.
///  MAKVTL     I   Vertex-plate list flag.
///  SPXISZ     I   Spatial index integer component size.
///  WORK      I-O  Workspace.
///  SPAIXD     O   Double precision component of spatial index.
///  SPAIXI     O   Integer component of spatial index.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NV       is the number of vertices belonging to the input
///           set of plates.
///
///  VRTCES   is an array of coordinates of the vertices. The Ith
///           vertex occupies elements (1:3,I) of this array.
///
///  NP       is the number of plates in the input plate set.
///
///  PLATES   is an array representing the triangular plates of a
///           shape model. The elements of PLATES are vertex
///           indices; vertex indices are 1-based. The vertex
///           indices of the Ith plate occupy elements (1:3,I) of
///           this array.
///
///  FINSCL   is the fine voxel scale. This scale determines the
///           edge length of the cubical voxels comprising the fine
///           voxel grid: the edge length VOXSIZ is approximately
///
///               FINSCL * {average plate extent}
///
///           where the extents of a plate are the respective
///           differences between the maximum and minimum
///           coordinate values of the plate's vertices.
///
///           The relationship between VOXSIZ and the average plate
///           extent is approximate because the VOXSIZ is adjusted
///           so that each dimension of the fine voxel grid is an
///           integer multiple of the coarse voxel scale.
///
///           See the $Particulars section below for further
///           information on voxel scales.
///
///  CORSCL   is the coarse voxel scale. This integer scale is the
///           ratio of the edge length of coarse voxels to
///           that of fine voxels. The coarse scale must be
///           large enough so that the total number of coarse
///           voxels does not exceed MAXCGR (see the $Parameters
///           section below).
///
///  WORKSZ   is the second dimension of the workspace array WORK.
///           WORKSZ must be at least as large as the greater of
///
///              - the number of fine voxel-plate associations
///
///                This number is equal to
///
///                   NP * {average number of fine voxels
///                         intersected by each plate}
///
///              - the number of vertex-plate associations, if
///                the vertex-plate mapping is constructed.
///
///                This number is equal to
///
///                   NV + ( 3 * NP )
///
///  VOXPSZ   is the size of the fine voxel-plate pointer array.
///           This array maps fine voxels to lists of plates that
///           intersect those voxels. VOXPSZ must be at least as
///           large as
///
///                    3
///              CORSCL  * {number of non-empty coarse voxels}
///
///  VOXLSZ   is the size of the fine voxel-plate list array. This
///           array contains, for each non-empty fine voxel, the
///           count of plates that intersect that voxel and the
///           IDs of those plates. VOXLSZ must be at least as large
///           as
///
///                   NP * {average number of fine voxels
///                         intersected by each plate}
///
///               +   {number of non-empty fine voxels}
///
///  MAKVTL   is a logical flag that, when set to .TRUE., indicates
///           that a  vertex-plate association list is to be
///           constructed.
///
///           The amount of workspace that is needed may depend on
///           whether a vertex-plate association list is
///           constructed. When this list is constructed, the size
///           of the integer component of the spatial index is
///           increased by the size of the list and the size of a
///           vertex-plate pointer array; the total of these sizes
///           is
///
///              ( 2 * NV ) + ( 3 * NP )
///
///  SPXISZ   is the declared size of the output array SPAIXI. This
///           size must be at least as large as the sum of
///
///              - the fixed-size part of the integer component of
///                the index, which includes the coarse voxel grid;
///                this value is
///
///                   IXIFIX
///
///              - the size VOXPSZ of the voxel-plate pointer array
///
///              - the size VOXLSZ of the voxel-plate association
///                list
///
///           plus, if the vertex-plate association list is
///           constructed,
///
///              - the size NV of the vertex-plate pointer array
///
///              - the size of the vertex-plate association list;
///                this size is
///
///                   NV + ( 3 * NP )
///
///  WORK     is the workspace array. The array should be declared
///           with dimensions
///
///              (2, WORKSZ)
///
///           See the description of WORKSZ above.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WORK     is the workspace array, modified by the operations
///           performed by this routine.
///
///  SPAIXD,
///  SPAIXI   are, respectively, the double precision and integer
///           components of the spatial index of the segment.
///
///           SPAIXD must be declared with size at least IXDFIX.
///           SPAIXI must be declared with size at least SPXISZ.
/// ```
///
/// # Parameters
///
/// ```text
///  IXDFIX   is the size of the double precision component of
///           the spatial index.
///
///  IXIFIX   is the size of the fixed-size portion of the integer
///           component of the spatial index.
///
///  See the include file
///
///     dsk02.inc
///
///  for declarations of DSK data type 2 (plate model) parameters.
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
///  1)  If the fine voxel scale is non-positive, the error
///      SPICE(BADFINEVOXELSCALE) is signaled.
///
///  2)  If the coarse voxel scale is less than 1, the error
///      SPICE(BADCOARSEVOXSCALE) is signaled.
///
///  3)  If NV is less than 3 or greater than MAXVRT, the error
///      SPICE(BADVERTEXCOUNT) is signaled.
///
///  4)  If NP is less than 1 or greater than MAXPLT, the error
///      SPICE(BADPLATECOUNT) is signaled.
///
///  5)  If the workspace size WORKSZ is less than NP+1, the error
///      SPICE(WORKSPACETOOSMALL) is signaled. This is merely a
///      sanity check; normally the workspace will need to be
///      substantially larger than this reference value. See the
///      description of WORKSZ in the header section $Detailed_Input
///      above.
///
///  6)  If the voxel-plate pointer array size VOXPSZ is less than 1,
///      the error SPICE(PTRARRAYTOOSMALL) is signaled. This is merely
///      a sanity check; normally this pointer array will need to be
///      substantially larger than this reference value. See the
///      description of VOXPSZ in the header section $Detailed_Input
///      above.
///
///  7)  If the voxel-plate list array size VOXLSZ is less than NP+1,
///      the error SPICE(PLATELISTTOOSMALL) is signaled. This is
///      merely a sanity check; normally this array will need to be
///      substantially larger than this reference value. See the
///      description of VOXLSZ in the header section $Detailed_Input
///      above.
///
///  8)  If the size SPXISZ of the integer array SPAIXI is too small
///      to contain its constituent structures, where the sizes
///      of these structures are derived from the inputs
///
///          NV, NP, VOXPSZ, VOXLSZ
///
///      the error SPICE(INTINDEXTOOSMALL) is signaled.
///
///  9)  If there is insufficient room to create any of the data
///      structures contained in the spatial index, an error is
///      signaled by a routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  Users planning to create DSK files should consider whether the
///  SPICE DSK creation utility MKDSK may be suitable for their needs.
///
///  This routine supports use of the DSK type 2 segment writer DSKW02
///  by creating the "spatial index" arrays required as inputs to that
///  routine.
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
///           PROGRAM DSKMI2_EX1
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
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR) (BVS)
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of arguments WORK, SPAIXD and SPAIXI in $Brief_I/O table.
///
///         Added solution to code example.
///
/// -    SPICELIB Version 1.0.0, 13-DEC-2016 (NJB)
///
///         Updated check on NV.
///
///         16-MAR-2016 (NJB)
///
///            Now zeros out the size of the vertex-plate list
///            when the list is not created.
///
///         23-JAN-2016 (NJB)
///
///            Original version.
/// ```
pub fn dskmi2(
    ctx: &mut SpiceContext,
    nv: i32,
    vrtces: &[[f64; 3]],
    np: i32,
    plates: &[[i32; 3]],
    finscl: f64,
    corscl: i32,
    worksz: i32,
    voxpsz: i32,
    voxlsz: i32,
    makvtl: bool,
    spxisz: i32,
    work: &mut [[i32; 2]],
    spaixd: &mut [f64; 10],
    spaixi: &mut [i32],
) -> crate::Result<()> {
    DSKMI2(
        nv,
        vrtces.as_flattened(),
        np,
        plates.as_flattened(),
        finscl,
        corscl,
        worksz,
        voxpsz,
        voxlsz,
        makvtl,
        spxisz,
        work.as_flattened_mut(),
        spaixd,
        spaixi,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKMI2 ( DSK, make spatial index for type 2 segment )
pub fn DSKMI2(
    NV: i32,
    VRTCES: &[f64],
    NP: i32,
    PLATES: &[i32],
    FINSCL: f64,
    CORSCL: i32,
    WORKSZ: i32,
    VOXPSZ: i32,
    VOXLSZ: i32,
    MAKVTL: bool,
    SPXISZ: i32,
    WORK: &mut [i32],
    SPAIXD: &mut [f64],
    SPAIXI: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VRTCES = DummyArray2D::new(VRTCES, 1..=3, 1..);
    let PLATES = DummyArray2D::new(PLATES, 1..=3, 1..);
    let mut WORK = DummyArrayMut2D::new(WORK, 1..=2, 1..=WORKSZ);
    let mut SPAIXD = DummyArrayMut::new(SPAIXD, 1..=IXDFIX);
    let mut SPAIXI = DummyArrayMut::new(SPAIXI, 1..=SPXISZ);
    let mut J: i32 = 0;
    let mut NSHIFT: i32 = 0;
    let mut NVXTOT: i32 = 0;
    let mut REQSIZ: i32 = 0;
    let mut VTLIDX: i32 = 0;
    let mut VTPIDX: i32 = 0;
    let mut VTXLSZ: i32 = 0;
    let mut VXLIDX: i32 = 0;
    let mut VXPIDX: i32 = 0;

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

    CHKIN(b"DSKMI2", ctx)?;

    //
    // Perform error checks on inputs.
    //
    if (FINSCL <= 0.0) {
        SETMSG(
            b"Fine voxel scale = #; scale must be positive. Usually scale should be > 1.0.",
            ctx,
        );
        ERRDP(b"#", FINSCL, ctx);
        SIGERR(b"SPICE(BADFINEVOXELSCALE)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    if (CORSCL < 1) {
        SETMSG(b"Coarse voxel scale = #; scale must be >= 1.", ctx);
        ERRINT(b"#", CORSCL, ctx);
        SIGERR(b"SPICE(BADCOARSEVOXSCALE)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    if ((NV < 3) || (NV > MAXVRT)) {
        SETMSG(b"Vertex count NV = #; count must be in the range 3:#.", ctx);
        ERRINT(b"#", NV, ctx);
        ERRINT(b"#", MAXVRT, ctx);
        SIGERR(b"SPICE(BADVERTEXCOUNT)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    if ((NP < 1) || (NP > MAXPLT)) {
        SETMSG(b"Plate count NP = #; count must be in the range 1:#.", ctx);
        ERRINT(b"#", NP, ctx);
        ERRINT(b"#", MAXPLT, ctx);
        SIGERR(b"SPICE(BADPLATECOUNT)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    if (WORKSZ < (NP + 1)) {
        SETMSG(b"Workspace size = #; size is too small to hold all voxel-plate associations. Size should be at least # * (average number of voxels intersected by each plate).", ctx);
        ERRINT(b"#", WORKSZ, ctx);
        ERRINT(b"#", NP, ctx);
        SIGERR(b"SPICE(WORKSPACETOOSMALL)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    if (VOXPSZ < 1) {
        SETMSG(b"Voxel-pointer array size = #; size is too small to hold all voxel-plate list pointers. Size should be at least # * (number of non-empty coarse voxels).", ctx);
        ERRINT(b"#", VOXPSZ, ctx);
        ERRINT(b"#", intrinsics::pow(CORSCL, 3), ctx);
        SIGERR(b"SPICE(PTRARRAYTOOSMALL)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    if (VOXLSZ < (NP + 1)) {
        SETMSG(b"Voxel-plate list array size = #; size is too small to hold all voxel-plate associations. Size should be at least # * (average number of voxels intersected by each plate).", ctx);
        ERRINT(b"#", VOXLSZ, ctx);
        ERRINT(b"#", NP, ctx);
        SIGERR(b"SPICE(PLATELISTTOOSMALL)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    //
    // Check the size of the integer spatial index array. The
    // declared size must be large enough to hold:
    //
    //    - the fixed-size part of the index, which includes
    //      the coarse voxel grid
    //
    //    - the voxel-plate pointer array
    //
    //    - the voxel-plate association list
    //
    // plus, if the vertex-plate association list is constructed,
    //
    //    - the vertex-plate pointer array
    //
    //    - the vertex-plate association list
    //
    //
    REQSIZ = ((IXIFIX + VOXPSZ) + VOXLSZ);

    if MAKVTL {
        //
        // Add on the sizes of the vertex-plate pointer array (NV)
        // and the vertex-plate list array (NV + 3*NP).
        //
        VTXLSZ = (NV + (3 * NP));

        REQSIZ = ((REQSIZ + NV) + VTXLSZ);
    } else {
        VTXLSZ = 0;
    }

    if (SPXISZ < REQSIZ) {
        SETMSG(
            b"Integer spatial index size = #; size must be at least #.",
            ctx,
        );
        ERRINT(b"#", SPXISZ, ctx);
        ERRINT(b"#", REQSIZ, ctx);
        SIGERR(b"SPICE(INTINDEXTOOSMALL)", ctx)?;
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    //
    // Set known values in spatial index arrays.
    //
    SPAIXI[SICGSC] = CORSCL;

    //
    // Prepare indices in the spatial index arrays.
    //
    //    VXPIDX is the start index of the voxel pointer array.
    //
    VXPIDX = (SICGRD + MAXCGR);
    //
    // VXLIDX is the start index of the voxel-plate list. This
    // list is offset from the start of the pointer array by
    // the input size given for that array. The size is the
    // total room available, not the room actually used.
    //
    VXLIDX = (VXPIDX + VOXPSZ);

    //
    // Create spatial index for plates.
    //
    let [arg9, arg13, arg14, arg15, arg16, arg18] = SPAIXI
        .get_disjoint_slices_mut([SIVGRX, SIVXNP, VXPIDX, SIVXNL, VXLIDX, SICGRD])
        .unwrap();
    let [arg10, arg11, arg17] = SPAIXD
        .get_disjoint_slices_mut([SIVXSZ, SIVXOR, SIVTBD])
        .unwrap();
    ZZMKSPIN(
        NP,
        PLATES.as_slice(),
        VRTCES.as_slice(),
        FINSCL,
        CORSCL,
        VOXPSZ,
        WORKSZ,
        VOXLSZ,
        WORK.as_slice_mut(),
        arg9,
        arg10.first_mut().unwrap(),
        arg11,
        &mut NVXTOT,
        arg13.first_mut().unwrap(),
        arg14,
        arg15.first_mut().unwrap(),
        arg16,
        arg17,
        arg18,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DSKMI2", ctx)?;
        return Ok(());
    }

    //
    // At this point the voxel plate list is offset from the
    // start of the voxel pointer array by the allocated size
    // of the array. We need to shift the plate list so that
    // it starts right after the end of the pointer array.
    //
    NSHIFT = (VOXPSZ - SPAIXI[SIVXNP]);

    for I in 1..=SPAIXI[SIVXNL] {
        J = ((VXLIDX - 1) + I);

        SPAIXI[(J - NSHIFT)] = SPAIXI[J];
    }

    //
    // Update the voxel list start index to reflect the shift.
    //
    VXLIDX = (VXLIDX - NSHIFT);

    //
    // Create vertex-plate mapping, if requested, as indicated
    // by the vertex-plate list size.
    //
    if MAKVTL {
        //
        // VTPIDX is the start index of the vertex pointer array.
        //
        VTPIDX = (VXLIDX + SPAIXI[SIVXNL]);
        //
        // VXLIDX is the start index of the vertex-plate list. The
        // list start is offset from the vertex pointer array by
        // the size of the array, which is always NV.
        //
        VTLIDX = (VTPIDX + NV);

        let [arg6, arg7, arg8] = SPAIXI
            .get_disjoint_slices_mut([VTPIDX, SIVTNL, VTLIDX])
            .unwrap();
        ZZVRTPLT(
            NV,
            NP,
            PLATES.as_slice(),
            WORKSZ,
            VTXLSZ,
            WORK.as_slice_mut(),
            arg6,
            arg7.first_mut().unwrap(),
            arg8,
            ctx,
        )?;
    } else {
        //
        // Zero out the size of the vertex-plate list.
        //
        SPAIXI[SIVTNL] = 0;
    }

    CHKOUT(b"DSKMI2", ctx)?;
    Ok(())
}
