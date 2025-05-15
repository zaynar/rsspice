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
const DBFSIZ: i32 = 10;
const IBFSIZ: i32 = 10;
const VTBSIZ: i32 = 6;

/// DSK, fetch type 2 bookkeeping data
///
/// Return bookkeeping data from a DSK type 2 segment.
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
///  NV         O   Number of vertices in model.
///  NP         O   Number of plates in model.
///  NVXTOT     O   Number of voxels in fine grid.
///  VTXBDS     O   Vertex bounds.
///  VOXSIZ     O   Fine voxel edge length.
///  VOXORI     O   Fine voxel grid origin.
///  VGREXT     O   Fine voxel grid extent.
///  CGSCAL     O   Coarse voxel grid scale.
///  VTXNPL     O   Size of vertex-plate correspondence list.
///  VOXNPT     O   Size of voxel-plate pointer list.
///  VOXNPL     O   Size of voxel-plate correspondence list.
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
/// ```
///
/// # Detailed Output
///
/// ```text
///  NV       is the number of vertices in model.
///
///  NP       is the number of plates in model.
///
///  NVXTOT   is the total number of voxels in fine grid.
///
///  VTXBDS   are the vertex bounds. This is an array of six values
///           giving the minimum and maximum values of each component
///           of the vertex set. VTXBDS has dimensions ( 2, 3 ).
///           Units are km.
///
///  VOXSIZ   is the fine grid voxel size. DSK voxels are cubes; the
///           edge length of each cube is given by the voxel size.
///           This size applies to the fine voxel grid. Units are km.
///
///  VOXORI   is the voxel grid origin. This is the location of the
///           voxel grid origin in the body-fixed frame associated
///           with the target body. Units are km.
///
///  VGREXT   is the voxel grid extent. This extent is an array of
///           three integers indicating the number of voxels in the
///           X, Y, and Z directions in the fine voxel grid.
///
///  CGSCAL   is the coarse voxel grid scale. The extent of the fine
///           voxel grid is related to the extent of the coarse voxel
///           grid by this scale factor.
///
///  VTXNPL   is the vertex-plate correspondence list size.
///
///  VOXNPT   is the size of the voxel-to-plate pointer list.
///
///  VOXNPL   is the voxel-plate correspondence list size.
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
/// ```
///
/// # Files
///
/// ```text
///  See input argument HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine supports computations involving bookkeeping
///  information stored in DSK type 2 segments. User applications
///  typically will not need to call this routine.
///
///  DSK files are built using the DLA low-level format and
///  the DAS architecture; DLA files are a specialized type of DAS
///  file in which data are organized as a doubly linked list of
///  segments. Each segment's data belong to contiguous components of
///  character, double precision, and integer type.
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
///  1) Dump several parameters from the first DLA segment of
///     a DSK file. The segment is assumed to be of type 2.
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKB02_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///
///     C
///     C     Local parameters
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    DSK
///           CHARACTER*(LNSIZE)    OUTLIN
///
///           DOUBLE PRECISION      VOXORI ( 3 )
///           DOUBLE PRECISION      VOXSIZ
///           DOUBLE PRECISION      VTXBDS ( 2, 3 )
///
///           INTEGER               CGSCAL
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               NP
///           INTEGER               NV
///           INTEGER               NVXTOT
///           INTEGER               VGREXT ( 3 )
///           INTEGER               VOXNPL
///           INTEGER               VOXNPT
///           INTEGER               VTXNPL
///
///           LOGICAL               FOUND
///
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
///
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
///     C     Read and display type 2 bookkeeping data.
///     C
///           CALL DSKB02 ( HANDLE, DLADSC, NV,     NP,     NVXTOT,
///          .              VTXBDS, VOXSIZ, VOXORI, VGREXT, CGSCAL,
///          .              VTXNPL, VOXNPT, VOXNPL                 )
///
///     C
///     C     Show vertex and plate counts.
///     C
///           OUTLIN = 'Number of vertices:                 #'
///           CALL REPMI  ( OUTLIN, '#', NV, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Number of plates:                   #'
///           CALL REPMI  ( OUTLIN, '#', NP, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Voxel edge length (km):             #'
///           CALL REPMF  ( OUTLIN, '#', VOXSIZ, 6, 'E', OUTLIN )
///           CALL TOSTDO ( OUTLIN )
///
///           OUTLIN = 'Number of voxels:                   #'
///           CALL REPMI  ( OUTLIN, '#', NVXTOT, OUTLIN )
///           CALL TOSTDO ( OUTLIN )
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
///     Number of vertices:                 1579014
///     Number of plates:                   3145728
///     Voxel edge length (km):             1.04248E-01
///     Number of voxels:                   11914500
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The caller must verify that the segment associated with
///      the input DLA descriptor is a DSK type 2 segment.
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
/// -    SPICELIB Version 1.0.1, 02-JUL-2021 (JDR) (BVS)
///
///         Edited the header to comply with NAIF standard. Added
///         solution for code example.
///
/// -    SPICELIB Version 1.0.0, 08-FEB-2017 (NJB)
///
///         Updated version info.
///
///         23-JAN-2016 (NJB)
///
///            Removed references to unneeded variables.
///            Updated header comments.
///
///         DSKLIB Version 2.0.0, 05-MAY-2010 (NJB)
///
///            Renamed routine from DSKP02 to DSKB02.
///
///         DSKLIB Version 1.0.1, 08-OCT-2009 (NJB)
///
///            Updated header.
///
///         Beta Version 1.0.0, 30-OCT-2006 (NJB)
/// ```
pub fn dskb02(
    ctx: &mut SpiceContext,
    handle: i32,
    dladsc: &[i32],
    nv: &mut i32,
    np: &mut i32,
    nvxtot: &mut i32,
    vtxbds: &mut [[f64; 2]; 3],
    voxsiz: &mut f64,
    voxori: &mut [f64; 3],
    vgrext: &mut [i32; 3],
    cgscal: &mut i32,
    vtxnpl: &mut i32,
    voxnpt: &mut i32,
    voxnpl: &mut i32,
) -> crate::Result<()> {
    DSKB02(
        handle,
        dladsc,
        nv,
        np,
        nvxtot,
        vtxbds.as_flattened_mut(),
        voxsiz,
        voxori,
        vgrext,
        cgscal,
        vtxnpl,
        voxnpt,
        voxnpl,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKB02 ( DSK, fetch type 2 bookkeeping data )
pub fn DSKB02(
    HANDLE: i32,
    DLADSC: &[i32],
    NV: &mut i32,
    NP: &mut i32,
    NVXTOT: &mut i32,
    VTXBDS: &mut [f64],
    VOXSIZ: &mut f64,
    VOXORI: &mut [f64],
    VGREXT: &mut [i32],
    CGSCAL: &mut i32,
    VTXNPL: &mut i32,
    VOXNPT: &mut i32,
    VOXNPL: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DLADSC = DummyArray::new(DLADSC, 1..);
    let mut VTXBDS = DummyArrayMut2D::new(VTXBDS, 1..=2, 1..=3);
    let mut VOXORI = DummyArrayMut::new(VOXORI, 1..=3);
    let mut VGREXT = DummyArrayMut::new(VGREXT, 1..=3);
    let mut DBUFF = StackArray::<f64, 10>::new(1..=DBFSIZ);
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut DPBASE: i32 = 0;
    let mut IBASE: i32 = 0;
    let mut IBUFF = StackArray::<i32, 10>::new(1..=IBFSIZ);

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

    CHKIN(b"DSKB02", ctx)?;

    DPBASE = DLADSC[DBSIDX];
    IBASE = DLADSC[IBSIDX];

    //
    // Read the integer parameters first.  These are located at the
    // beginning of the integer component of the segment.
    //
    DASRDI(
        HANDLE,
        (IBASE + 1),
        (IBASE + IBFSIZ),
        IBUFF.as_slice_mut(),
        ctx,
    )?;

    *NV = IBUFF[IXNV];
    *NP = IBUFF[IXNP];
    *NVXTOT = IBUFF[IXNVXT];
    *CGSCAL = IBUFF[IXCGSC];
    *VOXNPT = IBUFF[IXVXPS];
    *VOXNPL = IBUFF[IXVXLS];
    *VTXNPL = IBUFF[IXVTLS];

    MOVEI(IBUFF.subarray(IXVGRX), 3, VGREXT.as_slice_mut());

    //
    // Read the d.p. parameters.
    //
    B = (DPBASE + IXVTBD);
    E = (((DPBASE + IXVTBD) + DBFSIZ) - 1);

    DASRDD(HANDLE, B, E, DBUFF.as_slice_mut(), ctx)?;

    MOVED(DBUFF.as_slice(), VTBSIZ, VTXBDS.as_slice_mut());
    VEQU(DBUFF.subarray((VTBSIZ + 1)), VOXORI.as_slice_mut());

    *VOXSIZ = DBUFF[(IXVXSZ - DSKDSZ)];

    CHKOUT(b"DSKB02", ctx)?;
    Ok(())
}
