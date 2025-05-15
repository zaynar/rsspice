//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 6;
const DTYPE: i32 = 6;
const CTRLSZ: i32 = 2;
const MCTLSZ: i32 = 4;
const BUFSIZ: i32 = MCTLSZ;
const DIRSIZ: i32 = 100;

/// C-kernel, get mini-segment parameters, type 06
///
/// Return the mini-segment control parameters, mini-segment interval
/// bounds, and last epoch for a specified mini-segment in a type 6
/// CK segment.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   The handle of the file containing the segment.
///  DESCR      I   The descriptor of the type 6 segment.
///  MSNO       I   Mini-segment index.
///  RATE       O   SCLK rate in seconds/tick.
///  SUBTYP     O   Subtype code.
///  WINSIZ     O   Interpolation window size.
///  NREC       O   Number of records in mini-segment.
///  IVLBDS     O   Mini-segment interval bounds of mini-segment.
///  LSTEPC     O   Last epoch of mini-segment.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the binary CK file containing the
///           segment. Normally the CK file should be open for
///           read access. See the $Files section below for details.
///
///  DESCR    is the DAF descriptor of a CK data type 6 segment.
///
///  MSNO     is the index of a mini-segment within the segment
///           identified by HANDLE and DESCR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RATE     is the spacecraft clock rate for the specified
///           mini-segment. RATE has units of seconds/tick.
///
///  SUBTYP   is the subtype code of the specified mini-segment.
///
///  WINSIZ   is the interpolation window size for the specified
///           mini-segment. The window size is
///
///              ( DEGREE + 1 ) / 2     for subtypes 0 and 2
///                DEGREE + 1           for subtypes 1 and 3
///
///           where DEGREE is the interpolation degree of the
///           mini-segment.
///
///  NREC     is the number of data records in the CK mini-segment
///           identified by HANDLE, DESCR, and MSNO
///
///  IVLBDS   is a two-element array containing, in order, the
///           encoded SCLK start and stop times of the coverage
///           interval of the specified mini-segment. The
///           mini-segment provides data for times within this
///           interval.
///
///  LSTEPC   is the last epoch in the sequence of time tags
///           belonging to the specified mini-segment. LSTEPC is
///           an encoded SCLK time.
///
///           If LSTEPC precedes IVLBDS(2), the mini-segment has
///           a coverage gap between those two epochs.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file ck06.inc for a description of CK type 6
///  subtypes.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment indicated by DESCR is not a type 6 segment,
///      the error SPICE(CKWRONGDATATYPE) is signaled.
///
///  2)  If the specified handle does not belong to any DAF file that
///      is currently known to be open, an error is signaled by a
///      routine in the call tree of this routine.
///
///  3)  If DESCR is not a valid descriptor of a valid segment in the
///      CK file specified by HANDLE, the results of this routine are
///      unpredictable.
///
///  4)  If N is less than 1 or greater than the number of
///      mini-segments in the specified segment, the error
///      SPICE(INDEXOUTOFRANGE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  The CK file specified by HANDLE may be open for read or write
///  access. Normally, the file should have been opened for read
///  access. If the file is open for write access, the calling
///  application must ensure integrity of the CK segment being read.
///  If the structure of the segment is invalid---for example, if the
///  segment has been partially written---this routine will either
///  return invalid results, or it will cause a system-level runtime
///  error.
/// ```
///
/// # Particulars
///
/// ```text
///  For a complete description of the internal structure of a type 6
///  segment, see the CK Required Reading.
///
///  This routine is normally used in conjunction with CKNM06 and
///  CKGR06 to obtain time tags and packet data from a specified type
///  6 CK segment.
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
///
///  1) The following program dumps records from a CK file that
///     contains only type 6 segments.
///
///
///     Example code begins here.
///
///
///           PROGRAM CKMP06_EX1
///           IMPLICIT NONE
///
///     C
///     C     Dump all records from a CK that
///     C     contains only segments of type 6.
///     C
///           INCLUDE 'ck06.inc'
///
///     C
///     C     Local parameters
///     C
///           INTEGER               ND
///           PARAMETER           ( ND     = 2 )
///
///           INTEGER               NI
///           PARAMETER           ( NI     = 6 )
///
///           INTEGER               DSCSIZ
///           PARAMETER           ( DSCSIZ = 5 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///     C
///     C     RECSIZ is the size of the largest pointing
///     C     record, which corresponds to subtype 2.
///     C
///           INTEGER               RECSIZ
///           PARAMETER           ( RECSIZ = C06PS2 + 3 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    CK
///
///           DOUBLE PRECISION      DC     ( ND )
///           DOUBLE PRECISION      DESCR  ( DSCSIZ )
///           DOUBLE PRECISION      IVLBDS ( 2 )
///           DOUBLE PRECISION      LSTEPC
///           DOUBLE PRECISION      RATE
///           DOUBLE PRECISION      RECORD ( RECSIZ )
///
///           INTEGER               DTYPE
///           INTEGER               HANDLE
///           INTEGER               IC     ( NI )
///           INTEGER               RECNO
///           INTEGER               MSNO
///           INTEGER               NMINI
///           INTEGER               NREC
///           INTEGER               SEGNO
///           INTEGER               SUBTYP
///           INTEGER               WINSIZ
///
///           LOGICAL               FOUND
///
///
///           CALL PROMPT ( 'Enter name of CK to dump > ', CK )
///
///           CALL DAFOPR ( CK, HANDLE )
///
///     C
///     C     Dump data from each CK segment.
///     C
///           SEGNO = 0
///
///           CALL DAFBFS ( HANDLE )
///           CALL DAFFNA ( FOUND  )
///
///           DO WHILE ( FOUND )
///
///              SEGNO = SEGNO + 1
///
///              WRITE (*,*) ' '
///              WRITE (*,*) ' '
///              WRITE (*,*) 'Segment number: ', SEGNO
///
///     C
///     C        Fetch and unpack the descriptor of the
///     C        current segment; check the data type.
///     C
///              CALL DAFGS ( DESCR )
///              CALL DAFUS ( DESCR, ND, NI, DC, IC )
///
///              DTYPE = IC(3)
///
///              IF ( DTYPE .NE. 6 ) THEN
///
///                 CALL SETMSG ( 'Data type must be 6 but was #.' )
///                 CALL ERRINT ( '#',  DTYPE                      )
///                 CALL SIGERR ( 'SPICE(NOTSUPPORTED)'            )
///
///              END IF
///
///     C
///     C        Get the mini-segment count for this
///     C        segment.
///     C
///              CALL CKNM06 ( HANDLE, DESCR, NMINI )
///
///     C
///     C        Dump data from each mini-segment.
///     C
///              DO MSNO = 1, NMINI
///
///     C
///     C           Get the mini-segment's record count
///     C           and time bounds.
///     C
///                 CALL CKMP06 ( HANDLE, DESCR,  MSNO,
///          .                    RATE,   SUBTYP, WINSIZ,
///          .                    NREC,   IVLBDS, LSTEPC )
///
///                 WRITE (*,*) ' '
///                 WRITE (*,*) '   Mini-segment number: ', MSNO
///                 WRITE (*,*) '      Rate:           ',   RATE
///                 WRITE (*,*) '      Subtype:        ',   SUBTYP
///                 WRITE (*,*) '      Window size:    ',   WINSIZ
///                 WRITE (*,*) '      Interval start: ',   IVLBDS(1)
///                 WRITE (*,*) '      Interval stop:  ',   IVLBDS(2)
///                 WRITE (*,*) '      Last epoch:     ',   LSTEPC
///                 WRITE (*,*) ' '
///
///                 DO RECNO = 1, NREC
///
///                    CALL CKGR06 ( HANDLE, DESCR,
///          .                       MSNO,   RECNO,  RECORD )
///
///                    WRITE (*,*) '      Record number: ', RECNO
///                    WRITE (*,*) '         SCLKDP:     ', RECORD(1)
///                    WRITE (*,*) '         Clock rate: ', RECORD(3)
///
///                    IF ( SUBTYP .EQ. C06TP0 ) THEN
///
///                       WRITE (*,*) '         Q(0): ', RECORD(4)
///                       WRITE (*,*) '         Q(1): ', RECORD(5)
///                       WRITE (*,*) '         Q(2): ', RECORD(6)
///                       WRITE (*,*) '         Q(3): ', RECORD(7)
///                       WRITE (*,*) '    d Q(0)/dt: ', RECORD(8)
///                       WRITE (*,*) '    d Q(1)/dt: ', RECORD(9)
///                       WRITE (*,*) '    d Q(2)/dt: ', RECORD(10)
///                       WRITE (*,*) '    d Q(3)/dt: ', RECORD(11)
///
///                    ELSE IF ( SUBTYP .EQ. C06TP1 ) THEN
///
///                       WRITE (*,*) '         Q(0): ', RECORD(4)
///                       WRITE (*,*) '         Q(1): ', RECORD(5)
///                       WRITE (*,*) '         Q(2): ', RECORD(6)
///                       WRITE (*,*) '         Q(3): ', RECORD(7)
///
///                    ELSE IF ( SUBTYP .EQ. C06TP2 ) THEN
///
///                       WRITE (*,*) '         Q(0): ', RECORD(4)
///                       WRITE (*,*) '         Q(1): ', RECORD(5)
///                       WRITE (*,*) '         Q(2): ', RECORD(6)
///                       WRITE (*,*) '         Q(3): ', RECORD(7)
///                       WRITE (*,*) '    d Q(0)/dt: ', RECORD(8)
///                       WRITE (*,*) '    d Q(1)/dt: ', RECORD(9)
///                       WRITE (*,*) '    d Q(2)/dt: ', RECORD(10)
///                       WRITE (*,*) '    d Q(3)/dt: ', RECORD(11)
///                       WRITE (*,*) '        AV(1): ', RECORD(12)
///                       WRITE (*,*) '        AV(2): ', RECORD(13)
///                       WRITE (*,*) '        AV(3): ', RECORD(14)
///                       WRITE (*,*) '   d AV(1)/dt: ', RECORD(15)
///                       WRITE (*,*) '   d AV(2)/dt: ', RECORD(16)
///                       WRITE (*,*) '   d AV(3)/dt: ', RECORD(17)
///
///                    ELSE IF ( SUBTYP .EQ. C06TP3 ) THEN
///
///                       WRITE (*,*) '         Q(0): ', RECORD(4)
///                       WRITE (*,*) '         Q(1): ', RECORD(5)
///                       WRITE (*,*) '         Q(2): ', RECORD(6)
///                       WRITE (*,*) '         Q(3): ', RECORD(7)
///                       WRITE (*,*) '        AV(1): ', RECORD(8)
///                       WRITE (*,*) '        AV(2): ', RECORD(9)
///                       WRITE (*,*) '        AV(3): ', RECORD(10)
///
///                    ELSE
///                       CALL SETMSG ( 'Subtype # is not '
///          .            //            'recognized.'         )
///                       CALL ERRINT ( '#', SUBTYP           )
///                       CALL SIGERR ( 'SPICE(NOTSUPPORTED)' )
///                    END IF
///
///                    WRITE (*,*) ' '
///
///                END DO
///
///              END DO
///
///              CALL DAFFNA ( FOUND )
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the Rosetta CK file named
///     RATT_DV_257_02_01_T6_00344.BC, the output was:
///
///
///     Enter name of CK to dump > RATT_DV_257_02_01_T6_00344.BC
///
///
///      Segment number:            1
///
///         Mini-segment number:            1
///            Rate:              1.5258789062500000E-005
///            Subtype:                   1
///            Window size:              10
///            Interval start:    24471796593941.691
///            Interval stop:     24472844252095.523
///            Last epoch:        24472844252095.523
///
///            Record number:            1
///               SCLKDP:        24471796593941.691
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.95514652599900884
///               Q(1):   0.16277660709912350
///               Q(2):   0.11688592199582469
///               Q(3):  -0.21802883133317097
///
///            Record number:            2
///               SCLKDP:        24472234538651.801
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.95746293340938016
///               Q(1):   0.14880147654385018
///               Q(2):   0.12021705739210503
///               Q(3):  -0.21603405018065600
///
///            Record number:            3
///               SCLKDP:        24472676416997.039
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.95956954083287593
///               Q(1):   0.13478976855182764
///               Q(2):   0.12355113537344563
///               Q(3):  -0.21399329790313779
///
///            Record number:            4
///               SCLKDP:        24472844252095.523
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.96030932381589129
///               Q(1):   0.12949634043544370
///               Q(2):   0.12480922302154081
///               Q(3):  -0.21321200307405938
///
///
///         Mini-segment number:            2
///            Rate:              1.5258789062500000E-005
///            Subtype:                   1
///            Window size:              10
///            Interval start:    24472844252095.523
///            Interval stop:     24472863912889.105
///            Last epoch:        24472863912889.105
///
///            Record number:            1
///               SCLKDP:        24472844252095.523
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.96030932403888680
///               Q(1):   0.12949633879120778
///               Q(2):   0.12480922338599261
///               Q(3):  -0.21321200285498659
///
///            Record number:            2
///               SCLKDP:        24472851309816.297
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.96035266600496283
///               Q(1):   0.12922730685291675
///               Q(2):   0.12480259688433022
///               Q(3):  -0.21318389214860939
///
///            Record number:            3
///               SCLKDP:        24472859879905.805
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.96041575813224878
///               Q(1):   0.12886248165419970
///               Q(2):   0.12474605317805663
///               Q(3):  -0.21315359384649502
///
///            Record number:            4
///               SCLKDP:        24472863912889.105
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.96043784177251290
///               Q(1):   0.12871819083493355
///               Q(2):   0.12475418449192528
///               Q(3):  -0.21313651233726627
///
///
///         Mini-segment number:            3
///            Rate:              1.5258789062500000E-005
///            Subtype:                   1
///            Window size:              10
///            Interval start:    24472863912889.105
///            Interval stop:     24473139163999.207
///            Last epoch:        24473139163999.207
///
///            Record number:            1
///               SCLKDP:        24472863912889.105
///               Clock rate:    1.5258789062500000E-005
///               Q(0):  -0.96043784177455394
///               Q(1):   0.12871819083614683
///
///     [...]
///
///
///     Warning: incomplete output. Only 100 out of 2378824 lines have
///     been provided.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 06-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 14-MAR-2014 (NJB) (JML) (BVS)
/// ```
pub fn ckmp06(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    msno: i32,
    rate: &mut f64,
    subtyp: &mut i32,
    winsiz: &mut i32,
    nrec: &mut i32,
    ivlbds: &mut [f64; 2],
    lstepc: &mut f64,
) -> crate::Result<()> {
    CKMP06(
        handle,
        descr,
        msno,
        rate,
        subtyp,
        winsiz,
        nrec,
        ivlbds,
        lstepc,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKMP06 ( C-kernel, get mini-segment parameters, type 06 )
pub fn CKMP06(
    HANDLE: i32,
    DESCR: &[f64],
    MSNO: i32,
    RATE: &mut f64,
    SUBTYP: &mut i32,
    WINSIZ: &mut i32,
    NREC: &mut i32,
    IVLBDS: &mut [f64],
    LSTEPC: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut IVLBDS = DummyArrayMut::new(IVLBDS, 1..=2);
    let mut BUFFER = StackArray::<f64, 4>::new(1..=BUFSIZ);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DPDATA = StackArray::<f64, 1>::new(1..=1);
    let mut BADDR: i32 = 0;
    let mut EADDR: i32 = 0;
    let mut EPADDR: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut MINIE: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut NEPDIR: i32 = 0;
    let mut NINTVL: i32 = 0;
    let mut IVLBAS: i32 = 0;
    let mut PTRBAS: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //    ND         is the number of double precision components in an
    //               unpacked C-kernel descriptor.
    //
    //    NI         is the number of integer components in an unpacked
    //               C-kernel descriptor.
    //
    //    DTYPE      is the data type of the segment that this routine
    //               operates on.
    //

    //
    // Mini-segment control area size:
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKMP06", ctx)?;

    //
    // The number of discrete pointing instances contained in a data
    // type 6 segment is stored in the last double precision word of the
    // segment. Since the address of the last word is stored in the
    // sixth integer component of the segment descriptor, it is a
    // trivial matter to extract the count.
    //
    // The unpacked descriptor contains the following information about
    // the segment:
    //
    //    DC(1)  Initial encoded SCLK
    //    DC(2)  Final encoded SCLK
    //
    //    IC(1)  CK frame class ID (aka "instrument")
    //    IC(2)  Inertial reference frame
    //    IC(3)  Data type
    //    IC(4)  Angular velocity flag
    //    IC(5)  Initial address of segment data
    //    IC(6)  Final address of segment data
    //
    //
    DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DC.as_slice_mut(),
        IC.as_slice_mut(),
    );

    //
    // If this segment is not of data type 6, then signal an error.
    //
    if (IC[3] != DTYPE) {
        SETMSG(
            b"Data type of the segment should be 6: Passed descriptor shows type = #.",
            ctx,
        );
        ERRINT(b"#", IC[3], ctx);
        SIGERR(b"SPICE(CKWRONGDATATYPE)", ctx)?;
        CHKOUT(b"CKMP06", ctx)?;
        return Ok(());
    }

    //
    // Check the mini-segment index.
    //
    // The number of mini-segments is the final word in the segment.
    //
    BADDR = IC[5];
    EADDR = IC[6];

    DAFGDA(HANDLE, EADDR, EADDR, DPDATA.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKMP06", ctx)?;
        return Ok(());
    }

    NINTVL = intrinsics::IDNINT(DPDATA[1]);

    if ((MSNO < 1) || (MSNO > NINTVL)) {
        SETMSG(b"Mini-segment index must be in range 1:# but was #.", ctx);
        ERRINT(b"#", NINTVL, ctx);
        ERRINT(b"#", MSNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"CKMP06", ctx)?;
        return Ok(());
    }

    //
    // Set the base address of the mini-segment pointers. There are
    // NINTVL+1 pointers, and these precede the control area.
    //
    PTRBAS = ((EADDR - CTRLSZ) - (NINTVL + 1));

    //
    // Set the base address of the interval bounds. There are N+1
    // bounds, and these precede the interval bound directories and the
    // mini-segment pointers.
    //
    // The directory count is
    //
    //    ( ( NINTVL + 1 ) - 1 ) / DIRSIZ
    //
    //
    NDIR = (NINTVL / DIRSIZ);

    IVLBAS = ((PTRBAS - NDIR) - (NINTVL + 1));

    //
    // Fetch the interval bounds for the mini-segment of interest.
    //
    DAFGDA(
        HANDLE,
        (IVLBAS + MSNO),
        ((IVLBAS + MSNO) + 1),
        IVLBDS.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the mini-segment end pointer as an absolute DAF address.
    // The stored value is a relative address. Begin by looking up the
    // start pointer of the next mini-segment.
    //
    DAFGDA(
        HANDLE,
        ((PTRBAS + MSNO) + 1),
        ((PTRBAS + MSNO) + 1),
        BUFFER.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"CKMP06", ctx)?;
        return Ok(());
    }

    MINIE = (((BADDR - 1) + intrinsics::IDNINT(BUFFER[1])) - 1);

    //
    // Fetch the parameters for the mini-segment.
    //
    DAFGDA(
        HANDLE,
        ((MINIE - MCTLSZ) + 1),
        MINIE,
        BUFFER.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"CKMP06", ctx)?;
        return Ok(());
    }

    *RATE = BUFFER[1];
    *SUBTYP = intrinsics::IDNINT(BUFFER[2]);
    *WINSIZ = intrinsics::IDNINT(BUFFER[3]);
    *NREC = intrinsics::IDNINT(BUFFER[4]);

    //
    // The last epoch of the mini-segment precedes the mini-segment's
    // control area and the epoch directories.
    //
    NEPDIR = ((*NREC - 1) / DIRSIZ);

    EPADDR = ((MINIE - MCTLSZ) - NEPDIR);

    DAFGDA(HANDLE, EPADDR, EPADDR, std::slice::from_mut(LSTEPC), ctx)?;

    CHKOUT(b"CKMP06", ctx)?;
    Ok(())
}
