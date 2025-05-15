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

/// DSK, return DSK segment descriptor
///
/// Return the DSK descriptor from a DSK segment identified
/// by a DAS handle and DLA descriptor.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
/// * [DSK](crate::required_reading::dsk)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of a DSK file.
///  DLADSC     I   DLA segment descriptor.
///  DSKDSC     O   DSK segment descriptor.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DSK file that is open for
///           read access.
///
///  DLADSC   is the DLA segment descriptor corresponding to
///           a DSK segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DSKDSC   is the DSK segment descriptor of the segment
///           designated by the input handle and DLA descriptor.
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
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the size of the double precision component of the
///      segment is smaller than that of a DSK descriptor, the
///      error SPICE(INVALIDFORMAT) is signaled.
///
///  2)  If the input handle is invalid, an error is signaled by a
///      routine in the call tree of this routine.
///
///  3)  If the input DLA descriptor is invalid, the effect of this
///      routine is undefined. The error *may* be diagnosed by
///      routines in the call tree of this routine, but there are no
///      guarantees.
///
///  4)  If any DAS read error is detected, the error is signaled by a
///      routine in the call tree of this routine.
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
///  This is a convenience routine intended for use by low-level
///  routines that read DSK segments. This routine may also be called
///  by user applications that must access DSK files at the segment
///  level.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) Dump the DSK descriptors of a DSK file.
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKGD_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           CHARACTER*(FILSIZ)    DSK
///
///           DOUBLE PRECISION      DSKDSC ( DSKDSZ )
///
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               NXTDSC ( DLADSZ )
///
///           LOGICAL               FOUND
///
///
///           CALL PROMPT ( 'Enter DSK name > ', DSK )
///     C
///     C     Open the DSK file and begin a forward search
///     C     for segments.
///     C
///           CALL DASOPR ( DSK, HANDLE )
///
///           CALL DLABFS ( HANDLE, NXTDSC, FOUND )
///
///           DO WHILE ( FOUND )
///     C
///     C        Make the DLA descriptor we just fetched
///     C        the current one.
///     C
///              CALL MOVEI ( NXTDSC, DLADSZ, DLADSC )
///
///              CALL DSKGD ( HANDLE, DLADSC, DSKDSC )
///
///              WRITE (*,*) 'DSK descriptor contents: '
///
///              DO I = 1, DSKDSZ
///                 WRITE (*,*) DSKDSC(I)
///              END DO
///     C
///     C        Find the next segment, if it exists.
///     C
///              CALL DLAFNS ( HANDLE, DLADSC, NXTDSC, FOUND )
///
///           END DO
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
///      DSK descriptor contents:
///        401.00000000000000
///        401.00000000000000
///        1.0000000000000000
///        2.0000000000000000
///        10021.000000000000
///        1.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///        0.0000000000000000
///       -3.1415926535897931
///        3.1415926535897931
///       -1.5707963267948966
///        1.5707963267948966
///        8.0496322487215526
///        13.940939832123945
///       -1577879958.8160586
///        1577880066.1839132
///
///
///  2) Again, dump the DSK descriptors of a DSK file, this time
///     interpreting the descriptor information and displaying
///     it in a user-friendly form. This display is a simplified
///     version of that created by the utility DSKBRIEF.
///
///     This program requests the name of an optional meta-kernel.
///     The meta-kernel can be used to define surface name-ID
///     associations. If no meta-kernel is needed, the user can
///     enter a carriage return at the prompt for this file.
///
///
///     Example code begins here.
///
///
///           PROGRAM DSKGD_EX2
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///           INCLUDE 'srftrn.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FMT1
///           PARAMETER           ( FMT1 = '(A,2(F19.12))' )
///
///           CHARACTER*(*)         FMT2
///           PARAMETER           ( FMT2 = '(A,I3)' )
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
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 30 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 40 )
///
///           INTEGER               NSYS
///           PARAMETER           ( NSYS   = 4 )
///
///           INTEGER               NCLASS
///           PARAMETER           ( NCLASS = 2 )
///
///           INTEGER               CLNMLN
///           PARAMETER           ( CLNMLN = 25 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(BDNMLN)    BODNAM
///           CHARACTER*(TIMLEN)    BTIME
///           CHARACTER*(CLNMLN)    CLSNMS ( NCLASS )
///           CHARACTER*(FILSIZ)    DSK
///           CHARACTER*(TIMLEN)    ETIME
///           CHARACTER*(FRNMLN)    FRAME
///           CHARACTER*(FILSIZ)    META
///           CHARACTER*(SFNMLN)    SRFNAM
///           CHARACTER*(NAMLEN)    SYSNAM
///           CHARACTER*(NAMLEN)    SYSNMS ( NSYS )
///
///           DOUBLE PRECISION      DSKDSC ( DSKDSZ )
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      RP
///
///           INTEGER               BODYID
///           INTEGER               CORSYS
///           INTEGER               DCLASS
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               DTYPE
///           INTEGER               FRAMID
///           INTEGER               HANDLE
///           INTEGER               NXTDSC ( DLADSZ )
///           INTEGER               SEGNO
///           INTEGER               SURFID
///
///           LOGICAL               FOUND
///           LOGICAL               ISNAME
///
///     C
///     C     Initial values
///     C
///           DATA                  CLSNMS / 'Single-valued surface',
///          .                               'General surface'       /
///
///           DATA                  SYSNMS / 'Latitudinal',
///          .                               'Cylindrical',
///          .                               'Rectangular',
///          .                               'Planetodetic' /
///
///
///           CALL PROMPT ( 'Enter DSK name         > ', DSK  )
///           CALL PROMPT ( 'Enter meta-kernel name > ', META )
///
///           IF ( META .NE. ' ' ) THEN
///              CALL FURNSH ( META )
///           END IF
///
///     C
///     C     Open the DLA file and begin a forward search
///     C     for segments.
///     C
///           CALL DASOPR ( DSK, HANDLE )
///
///           SEGNO = 0
///
///           CALL DLABFS ( HANDLE, NXTDSC, FOUND )
///
///           DO WHILE ( FOUND )
///
///              SEGNO = SEGNO + 1
///     C
///     C        Make the DLA descriptor we just fetched
///     C        the current one.
///     C
///              CALL MOVEI ( NXTDSC, DLADSZ, DLADSC )
///
///              CALL DSKGD ( HANDLE, DLADSC, DSKDSC )
///
///              BODYID = NINT( DSKDSC(CTRIDX) )
///              SURFID = NINT( DSKDSC(SRFIDX) )
///              FRAMID = NINT( DSKDSC(FRMIDX) )
///              DTYPE  = NINT( DSKDSC(TYPIDX) )
///              DCLASS = NINT( DSKDSC(CLSIDX) )
///
///              CALL BODC2S ( BODYID, BODNAM )
///              CALL SRFC2S ( SURFID, BODYID, SRFNAM, ISNAME )
///              CALL FRMNAM ( FRAMID, FRAME  )
///
///              IF ( FRAME .EQ. ' ' ) THEN
///                 CALL INTSTR ( FRAMID, FRAME )
///              END IF
///
///              CALL ETCAL ( DSKDSC(BTMIDX), BTIME )
///              CALL ETCAL ( DSKDSC(ETMIDX), ETIME )
///
///              CORSYS = NINT( DSKDSC(SYSIDX) )
///
///              SYSNAM = SYSNMS( CORSYS )
///
///              WRITE (*,*)    '===================================='
///              WRITE (*,FMT2) ' DSK descriptor for segment ',
///          .                  SEGNO
///              WRITE (*,*)    '  Body:              ', BODNAM
///              WRITE (*,*)    '  Surface:           ', SRFNAM
///              WRITE (*,*)    '  Frame:             ', FRAME
///              WRITE (*,*)    '  Start time (TDB):  ', BTIME
///              WRITE (*,*)    '  Stop time  (TDB):  ', ETIME
///              WRITE (*,*)    '  Data type:         ', DTYPE
///              WRITE (*,*)    '  Data class:        ', DCLASS, ' ',
///          .                                        CLSNMS(DCLASS)
///              WRITE (*,*)    '  Coordinate system: ', SYSNAM
///
///              IF ( CORSYS .EQ. PDTSYS ) THEN
///
///                 RE = DSKDSC(PARIDX  )
///                 F  = DSKDSC(PARIDX+1)
///                 RP = RE * ( 1.D0 - F )
///
///                 WRITE (*,*) '     Equatorial radius (km): ', RE
///                 WRITE (*,*) '     Polar radius      (km): ', RP
///
///              END IF
///
///              WRITE (*,*) '  Segment boundaries:'
///
///              IF ( CORSYS .EQ. LATSYS ) THEN
///
///                 WRITE (*,FMT1) '    Longitude (deg):   ',
///          .                  DPR() * DSKDSC(MN1IDX),
///          .                  DPR() * DSKDSC(MX1IDX)
///                 WRITE (*,FMT1) '    Latitude  (deg):   ',
///          .                  DPR() * DSKDSC(MN2IDX),
///          .                  DPR() * DSKDSC(MX2IDX)
///                 WRITE (*,FMT1) '    Radius     (km):   ',
///          .                          DSKDSC(MN3IDX),
///          .                          DSKDSC(MX3IDX)
///
///              ELSE IF ( CORSYS .EQ. CYLSYS ) THEN
///
///                 CALL SETMSG ( 'Coordinate system was '
///          .      //            'Cylindrical'           )
///                 CALL SIGERR ( 'SPICE(NOTSUPPORTED)'   )
///
///
///              ELSE IF ( CORSYS .EQ. RECSYS ) THEN
///
///                 WRITE (*,FMT1) '    X-coordinate (km): ',
///          .                          DSKDSC(MN1IDX),
///          .                          DSKDSC(MX1IDX)
///                 WRITE (*,FMT1) '    Y-coordinate (km): ',
///          .                          DSKDSC(MN2IDX),
///          .                          DSKDSC(MX2IDX)
///                 WRITE (*,FMT1) '    Z-coordinate (km): ',
///          .                          DSKDSC(MN3IDX),
///          .                          DSKDSC(MX3IDX)
///
///              ELSE IF ( CORSYS .EQ. PDTSYS ) THEN
///
///                 WRITE (*,FMT1) '    Longitude (deg):   ',
///          .                  DPR() * DSKDSC(MN1IDX),
///          .                  DPR() * DSKDSC(MX1IDX)
///                 WRITE (*,FMT1) '    Latitude  (deg):   ',
///          .                  DPR() * DSKDSC(MN2IDX),
///          .                  DPR() * DSKDSC(MX2IDX)
///                 WRITE (*,FMT1) '    Altitude   (km):   ',
///          .                          DSKDSC(MN3IDX),
///          .                          DSKDSC(MX3IDX)
///              END IF
///     C
///     C        Find the next segment, if it exists.
///     C
///              CALL DLAFNS ( HANDLE, DLADSC, NXTDSC, FOUND )
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos512.bds and an empty
///     string instead of the meta-kernel name, the output was:
///
///
///     Enter DSK name         > phobos512.bds
///     Enter meta-kernel name >
///      ====================================
///      DSK descriptor for segment   1
///        Body:              PHOBOS
///        Surface:           401
///        Frame:             IAU_PHOBOS
///        Start time (TDB):  1950 JAN 01 00:00:41.183
///        Stop time  (TDB):  2050 JAN 01 00:01:06.183
///        Data type:                    2
///        Data class:                   1  Single-valued surface
///        Coordinate system: Latitudinal
///        Segment boundaries:
///         Longitude (deg):     -180.000000000000   180.000000000000
///         Latitude  (deg):      -90.000000000000    90.000000000000
///         Radius     (km):        8.049632248722    13.940939832124
///
///
///  3) Again, dump the DSK descriptors of a DSK file, using the
///     program from example 2, but this time reading the DSK file
///
///        phobos_3_3_3seg.bds
///
///     which can be created by running an example program from
///     DSKW02. Use the meta-kernel shown below to demonstrate surface
///     name-ID mapping.
///
///
///        KPL/MK
///
///        File: dskgd_ex3.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The file contents shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///
///        \begindata
///
///        NAIF_SURFACE_NAME += ( 'Phobos example surface 1',
///                               'Phobos example surface 2',
///                               'Phobos example surface 3' )
///        NAIF_SURFACE_CODE += (   1,   2,   3 )
///        NAIF_SURFACE_BODY += ( 401, 401, 401 )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     When Example #2 was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos_3_3_3seg.bds and the
///     meta-kernel dskgd_ex3.tm, the output was:
///
///
///     Enter DSK name         > phobos_3_3_3seg.bds
///     Enter meta-kernel name > dskgd_ex3.tm
///      ====================================
///      DSK descriptor for segment   1
///        Body:              PHOBOS
///        Surface:           Phobos example surface 1
///        Frame:             IAU_PHOBOS
///        Start time (TDB):  1950 JAN 01 00:00:00.000
///        Stop time  (TDB):  2050 JAN 01 00:00:00.000
///        Data type:                    2
///        Data class:                   2  General surface
///        Coordinate system: Latitudinal
///        Segment boundaries:
///         Longitude (deg):     -180.000000000000   180.000000000000
///         Latitude  (deg):      -90.000000000000    90.000000000000
///         Radius     (km):        8.225298075974    14.011768145626
///      ====================================
///      DSK descriptor for segment   2
///        Body:              PHOBOS
///        Surface:           Phobos example surface 2
///        Frame:             IAU_PHOBOS
///        Start time (TDB):  1950 JAN 01 00:00:00.000
///        Stop time  (TDB):  2050 JAN 01 00:00:00.000
///        Data type:                    2
///        Data class:                   2  General surface
///        Coordinate system: Rectangular
///        Segment boundaries:
///         X-coordinate (km):     -1.300000000000     1.310000000000
///         Y-coordinate (km):     -1.210000000000     1.200000000000
///         Z-coordinate (km):     -9.452932357788     9.638179779053
///      ====================================
///      DSK descriptor for segment   3
///        Body:              PHOBOS
///        Surface:           Phobos example surface 3
///        Frame:             IAU_PHOBOS
///        Start time (TDB):  1950 JAN 01 00:00:00.000
///        Stop time  (TDB):  2050 JAN 01 00:00:00.000
///        Data type:                    2
///        Data class:                   2  General surface
///        Coordinate system: Planetodetic
///           Equatorial radius (km):    13.000000000000000
///           Polar radius      (km):    9.0999999999999996
///        Segment boundaries:
///         Longitude (deg):     -180.000000000000   180.000000000000
///         Latitude  (deg):      -90.000000000000    90.000000000000
///         Altitude   (km):       -3.728668683604     1.372015791081
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  See Exception #3.
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
///         Edited the header to comply with NAIF standard. Extended the
///         $Exceptions section and updated the $Restrictions.
///
/// -    SPICELIB Version 1.0.0, 08-FEB-2017 (NJB)
///
///         Updated version info.
///
///         22-JAN-2016 (NJB)
///
///            Added new header example programs and updated existing
///            example program. Made minor changes to code to enhance
///            readability. Corrected header typo.
///
///         09-OCT-2009 (NJB)
/// ```
pub fn dskgd(
    ctx: &mut SpiceContext,
    handle: i32,
    dladsc: &[i32],
    dskdsc: &mut [f64],
) -> crate::Result<()> {
    DSKGD(handle, dladsc, dskdsc, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKGD ( DSK, return DSK segment descriptor  )
pub fn DSKGD(
    HANDLE: i32,
    DLADSC: &[i32],
    DSKDSC: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DLADSC = DummyArray::new(DLADSC, 1..);
    let mut DSKDSC = DummyArrayMut::new(DSKDSC, 1..);
    let mut DPBASE: i32 = 0;
    let mut DPSIZE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKGD", ctx)?;

    //
    // Fetch the base address and size of the DP component of the
    // indicated segment.
    //
    DPBASE = DLADSC[DBSIDX];
    DPSIZE = DLADSC[DSZIDX];

    //
    // If we don't have enough d.p. elements to hold a descriptor,
    // something's wrong.
    //
    if (DPSIZE < DSKDSZ) {
        SETMSG(b"Size of d.p. component of segment is #; cannot extract descriptor.  This is a file format error which may be indicative of a corrupted file.", ctx);
        ERRINT(b"#", DPSIZE, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"DSKGD", ctx)?;
        return Ok(());
    }

    //
    // Extract the descriptor.
    //
    DASRDD(
        HANDLE,
        (DPBASE + 1),
        (DPBASE + DSKDSZ),
        DSKDSC.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"DSKGD", ctx)?;
    Ok(())
}
