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

/// DSK, type 2, compute normal vector for plate
///
/// Compute the unit normal vector for a specified plate from a type
/// 2 DSK segment.
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
///  HANDLE     I   DSK file handle.
///  DLADSC     I   DLA descriptor.
///  PLID       I   Plate ID.
///  NORMAL     O   Plate's unit normal vector.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DSK file containing a type 2
///           segment from which data are to be fetched.
///
///  DLADSC   is the DLA descriptor associated with the segment
///           from which data are to be fetched.
///
///  PLID     is the plate ID. Plate IDs range from 1 to NP
///           (the number of plates).
/// ```
///
/// # Detailed Output
///
/// ```text
///  NORMAL   is the normal vector associated with the plate
///           designated by PLID. The direction of NORMAL is
///           determined by the order of the plate's vertices;
///           the vertices are presumed to be ordered in the
///           right-handed (counterclockwise) sense about the
///           normal direction. If the plate's vertices are
///           V1, V2, V3, then NORMAL points in the direction
///
///              (V2 - V1) x ( V3 - V2 )
///
///           where "x" represents the cross product operator.
///
///           The vector NORMAL is expressed in the body-fixed
///           reference frame of the segment designated by DLADSC.
///           The center of this frame is the origin of the cartesian
///           coordinate system in which the vertices are expressed.
///           Note that the frame center need not coincide with the
///           central body of the segment. Units are km.
///
///           The vector has magnitude 1.
///
///           If an error occurs on the call, NORMAL is undefined.
/// ```
///
/// # Parameters
///
/// ```text
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
///  4)  If PLID is less than 1 or greater than the number of plates
///      in the segment, the error SPICE(INDEXOUTOFRANGE) is signaled.
///
///  5)  This routine does not check for linear independence of the
///      plate's edges. The plate model is assumed to be geometrically
///      valid.
/// ```
///
/// # Files
///
/// ```text
///  See input argument HANDLE.
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
///  1) Look up all the vertices associated with each plate
///     of the model contained in a specified type 2 segment. For each
///     of the first 5 plates, display the plate's vertices and normal
///     vector.
///
///     For this example, we'll show the context of this look-up:
///     opening the DSK file for read access, traversing a trivial,
///     one-segment list to obtain the segment of interest.
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKN02_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dsk02.inc'
///
///
///           CHARACTER*(*)         FMT
///           PARAMETER           ( FMT    = '(1X,A,3(1XE15.8))' )
///
///
///           INTEGER               BUFSIZ
///           PARAMETER           ( BUFSIZ = 10000 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///
///           CHARACTER*(FILSIZ)    DSK
///
///           DOUBLE PRECISION      NORMAL ( 3 )
///           DOUBLE PRECISION      VERTS  ( 3, BUFSIZ )
///
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               J
///           INTEGER               N
///           INTEGER               NNORM
///           INTEGER               NP
///           INTEGER               NREAD
///           INTEGER               NV
///           INTEGER               NVTX
///           INTEGER               PLATES  ( 3, BUFSIZ )
///           INTEGER               PLIX
///           INTEGER               REMAIN
///           INTEGER               START
///
///           LOGICAL               FOUND
///
///     C
///     C     Prompt for name of DSK and open file for reading.
///     C
///           CALL PROMPT ( 'Enter DSK name > ', DSK )
///
///           CALL DASOPR ( DSK, HANDLE )
///
///           CALL DLABFS ( HANDLE, DLADSC, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///
///              CALL SETMSG ( 'No segment found in file #.' )
///              CALL ERRCH  ( '#',  DSK                     )
///              CALL SIGERR ( 'SPICE(NOSEGMENT)'            )
///
///           END IF
///
///     C
///     C     Get segment vertex and plate counts.
///     C
///           CALL DSKZ02 ( HANDLE, DLADSC, NV, NP )
///
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Number of vertices: ', NV
///           WRITE (*,*) 'Number of plates:   ', NP
///     C
///     C     Display the vertices of the first 5 plates.
///     C
///           REMAIN = MIN(5, NP)
///           START  = 1
///
///           DO WHILE ( REMAIN .GT. 0 )
///     C
///     C        NREAD is the number of plates we'll read on this
///     C        loop pass.
///     C
///              NREAD  = MIN ( BUFSIZ, REMAIN )
///
///              CALL DSKP02 ( HANDLE, DLADSC, START, NREAD, N,
///          .                 PLATES                          )
///
///              DO I = 1, N
///
///                 PLIX = START + I - 1
///     C
///     C           Read the vertices of the current plate.
///     C
///                 DO J = 1, 3
///                    CALL DSKV02 ( HANDLE, DLADSC, PLATES(J,I),
///          .                       1,      NVTX,   VERTS (1,J)  )
///                 END DO
///     C
///     C           Display the vertices of the current plate:
///     C
///                 WRITE (*,*  ) ' '
///                 WRITE (*,*  ) 'Plate number: ', PLIX
///                 WRITE (*,FMT) '   Vertex 1: ', (VERTS(J,1), J=1,3)
///                 WRITE (*,FMT) '   Vertex 2: ', (VERTS(J,2), J=1,3)
///                 WRITE (*,FMT) '   Vertex 3: ', (VERTS(J,3), J=1,3)
///
///     C
///     C           Display the normal vector of the current plate:
///     C
///                 CALL DSKN02 ( HANDLE, DLADSC, PLIX, NORMAL )
///
///                 WRITE (*,FMT) '   Normal:   ', (NORMAL(J), J=1,3)
///
///              END DO
///
///              START  = START  + NREAD
///              REMAIN = REMAIN - NREAD
///
///           END DO
///
///     C
///     C     Close the kernel.  This isn't necessary in a stand-
///     C     alone program, but it's good practice in subroutines
///     C     because it frees program and system resources.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos512.bds, the output
///     was:
///
///
///     Enter DSK name > phobos512.bds
///
///      Number of vertices:      1579014
///      Number of plates:        3145728
///
///      Plate number:            1
///         Vertex 1:  -0.67744400E+01  0.62681500E+01  0.60114900E+01
///         Vertex 2:  -0.67623800E+01  0.62572800E+01  0.60255600E+01
///         Vertex 3:  -0.67571000E+01  0.62775400E+01  0.60209600E+01
///         Normal:    -0.58197377E+00  0.32128561E+00  0.74704892E+00
///
///      Plate number:            2
///         Vertex 1:  -0.67744400E+01  0.62681500E+01  0.60114900E+01
///         Vertex 2:  -0.67797300E+01  0.62479000E+01  0.60161000E+01
///         Vertex 3:  -0.67623800E+01  0.62572800E+01  0.60255600E+01
///         Normal:    -0.58145695E+00  0.32198831E+00  0.74714881E+00
///
///      Plate number:            3
///         Vertex 1:  -0.67797300E+01  0.62479000E+01  0.60161000E+01
///         Vertex 2:  -0.67676800E+01  0.62370100E+01  0.60301900E+01
///         Vertex 3:  -0.67623800E+01  0.62572800E+01  0.60255600E+01
///         Normal:    -0.58159707E+00  0.32264196E+00  0.74675767E+00
///
///      Plate number:            4
///         Vertex 1:  -0.67797300E+01  0.62479000E+01  0.60161000E+01
///         Vertex 2:  -0.67849900E+01  0.62276200E+01  0.60207000E+01
///         Vertex 3:  -0.67676800E+01  0.62370100E+01  0.60301900E+01
///         Normal:    -0.58312901E+00  0.32056070E+00  0.74645924E+00
///
///      Plate number:            5
///         Vertex 1:  -0.67849900E+01  0.62276200E+01  0.60207000E+01
///         Vertex 2:  -0.67729900E+01  0.62167400E+01  0.60348200E+01
///         Vertex 3:  -0.67676800E+01  0.62370100E+01  0.60301900E+01
///         Normal:    -0.58366405E+00  0.32306020E+00  0.74496200E+00
///
///
///     Note that only the vertex information for first 5 plates is
///     provided.
/// ```
///
/// # Restrictions
///
/// ```text
///  See $Exceptions section.
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
/// -    SPICELIB Version 1.0.1, 09-JUL-2020 (JDR) (BVS)
///
///         Edited the header to comply with NAIF standard. Modified code
///         example to reduce the output.
///
///         Extended detailed description of NORMAL output argument and
///         added an additional record to $Index_Entries.
///
/// -    SPICELIB Version 1.0.0, 17-MAR-2016 (NJB)
///
///         Now calls ZZDDHHLU.
///
///         Deleted references to unused parameter. Updated
///         $Examples section.
///
///         DSKLIB Version 1.0.0, 02-JUN-2010 (NJB)
/// ```
pub fn dskn02(
    ctx: &mut SpiceContext,
    handle: i32,
    dladsc: &[i32],
    plid: i32,
    normal: &mut [f64; 3],
) -> crate::Result<()> {
    DSKN02(handle, dladsc, plid, normal, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKN02 ( DSK, type 2, compute normal vector for plate )
pub fn DSKN02(
    HANDLE: i32,
    DLADSC: &[i32],
    PLID: i32,
    NORMAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DLADSC = DummyArray::new(DLADSC, 1..);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);
    let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut EDGE1 = StackArray::<f64, 3>::new(1..=3);
    let mut EDGE2 = StackArray::<f64, 3>::new(1..=3);
    let mut VERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut N: i32 = 0;
    let mut NP: i32 = 0;
    let mut NV: i32 = 0;
    let mut PLATE = StackArray::<i32, 3>::new(1..=3);
    let mut START: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKN02", ctx)?;

    //
    // Look up the DSK descriptor for this segment.
    //
    DSKGD(HANDLE, DLADSC.as_slice(), DSKDSC.as_slice_mut(), ctx)?;

    //
    // Get the plate model size parameters for this segment.
    // Note that we get a segment data type check for free from
    // DSKZ02.
    //
    DSKZ02(HANDLE, DLADSC.as_slice(), &mut NV, &mut NP, ctx)?;

    //
    // Check START.
    //
    if ((PLID < 1) || (PLID > NP)) {
        ZZDDHHLU(HANDLE, b"DAS", false, &mut UNIT, ctx)?;

        SETMSG(b"Segment in DSK file # with DAS base addresses INT = #, DP = #, CHR = # contains # plates, so PLID must be in the range 1:#; actual value was #.", ctx);
        ERRFNM(b"#", UNIT, ctx)?;
        ERRINT(b"#", DLADSC[IBSIDX], ctx);
        ERRINT(b"#", DLADSC[DBSIDX], ctx);
        ERRINT(b"#", DLADSC[CBSIDX], ctx);
        ERRINT(b"#", NP, ctx);
        ERRINT(b"#", NP, ctx);
        ERRINT(b"#", PLID, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DSKN02", ctx)?;
        return Ok(());
    }

    //
    // Look up the plate and its vertices.
    //
    START = (((PLID - 1) * 3) + 1);

    DSKI02(
        HANDLE,
        DLADSC.as_slice(),
        KWPLAT,
        START,
        3,
        &mut N,
        PLATE.as_slice_mut(),
        ctx,
    )?;

    for I in 1..=3 {
        START = (((PLATE[I] - 1) * 3) + 1);

        DSKD02(
            HANDLE,
            DLADSC.as_slice(),
            KWVERT,
            START,
            3,
            &mut N,
            VERTS.subarray_mut([1, I]),
            ctx,
        )?;
    }

    if FAILED(ctx) {
        CHKOUT(b"DSKN02", ctx)?;
        return Ok(());
    }

    //
    // Use the right-handed order of the vertices to determine the
    // correct choice of normal direction.
    //
    VSUB(
        VERTS.subarray([1, 2]),
        VERTS.subarray([1, 1]),
        EDGE1.as_slice_mut(),
    );
    VSUB(
        VERTS.subarray([1, 3]),
        VERTS.subarray([1, 1]),
        EDGE2.as_slice_mut(),
    );

    UCRSS(EDGE1.as_slice(), EDGE2.as_slice(), NORMAL.as_slice_mut());

    CHKOUT(b"DSKN02", ctx)?;
    Ok(())
}
