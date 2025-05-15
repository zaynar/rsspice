//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const ND: i32 = 2;
const NI: i32 = 6;
const DTYPE: i32 = 6;
const CTRLSZ: i32 = 2;
const MCTLSZ: i32 = 4;
const BUFSIZ: i32 = MCTLSZ;
const DIRSIZ: i32 = 100;

struct SaveVars {
    PKTSZS: StackArray<i32, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PKTSZS = StackArray::<i32, 4>::new(0..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(C06PS0),
                Val::I(C06PS1),
                Val::I(C06PS2),
                Val::I(C06PS3),
            ]
            .into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { PKTSZS }
    }
}

/// C-kernel, get record, type 06
///
/// Return a specified pointing record from a type 6 CK segment, given
/// the CK file's handle and the segment's descriptor.
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
///  DESCR      I   The segment descriptor.
///  MSNO       I   Index of the mini-segment containing the record.
///  RECNO      I   Index of the pointing record to be returned.
///  RECORD     O   The pointing record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of the binary CK file containing the
///           segment. Normally the CK file should be open for read
///           access. See the $Files section below for details.
///
///  DESCR    is the DAF descriptor of the type 6 segment.
///
///  RECNO    is the number of the discrete pointing instance to be
///           returned from the specified type 6 segment.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the pointing record indexed by RECNO in the
///           segment. The contents are as follows:
///
///              RECORD( 1 ) = CLKOUT
///
///           CLKOUT is the encoded spacecraft clock time associated
///           with the returned pointing values.
///
///              RECORD( 2 ) = SUBTYP
///
///           SUBTYP is the CK type 6 subtype code. This code
///           identifies the structure and meaning of the rest
///           of the record. However, all subtypes have a
///           quaternion stored in elements 4-7.
///
///              RECORD( 3 ) = RATE
///
///           RATE is the nominal SCLK rate expressed in units of
///           seconds per tick. This rate is required to convert
///           quaternion or angular velocity derivatives from units
///           of radians/tick to radians/s.
///
///              RECORD( 4 ) = q0
///              RECORD( 5 ) = q1
///              RECORD( 6 ) = q2
///              RECORD( 7 ) = q3
///
///           Subtype 1 ends here; there are no angular velocity
///           data. Angular velocity is derived by differentiating
///           Lagrange interpolating polynomials.
///
///              RECORD(  8 ) =  ]
///              RECORD(  9 ) =  ] --- For subtypes 0 and 2, these
///              RECORD( 10 ) =  ]     elements contain a quaternion
///              RECORD( 11 ) =  ]     derivative. For subtype 3,
///                                    elements 8-10 contain an
///                                    angular velocity vector;
///                                    element 11 is unassigned.
///
///                                    All subtypes except subtype
///                                    2 stop here.
///
///              RECORD( 12 ) =  ]
///              RECORD( 13 ) =  ] --- For subtype 2, these
///              RECORD( 14 ) =  ]     elements contain an angular
///                                    velocity vector.
///
///
///              RECORD( 15 ) =  ]
///              RECORD( 16 ) =  ] --- For subtype 2, these
///              RECORD( 17 ) =  ]     elements contain the
///                                    derivative of an angular
///                                    velocity vector.
///
///           The quantities q0 - q3 are the components of the
///           quaternion that represents the C-matrix that transforms
///           vectors from the inertial reference frame of the
///           segment to the instrument frame at time CLKOUT.
///
///           Quaternion derivatives, angular velocity, or the
///           derivative of angular velocity are valid only if
///           these are supported by the segment subtype and
///           if the segment descriptor indicates that angular
///           velocity is present.
///
///           The components of the angular velocity vector are
///           specified relative to the inertial reference frame of
///           the segment.
///
///           Units of angular velocity and of quaternion
///           derivatives are radians/second and 1/second
///           respectively.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment is not of data type 6, the error
///      SPICE(CKWRONGDATATYPE) is signaled.
///
///  2)  If MSNO is less than one or greater than the number of
///      mini-segments in the specified segment, the error
///      SPICE(INDEXOUTOFRANGE) is signaled.
///
///  3)  If RECNO is less than one or greater than the number of
///      records in the specified segment, the error
///      SPICE(CKNONEXISTREC) is signaled.
///
///  4)  If the specified handle does not belong to any DAF file that
///      is currently known to be open, an error is signaled by a
///      routine in the call tree of this routine.
///
///  5)  If DESCR is not a valid descriptor of a valid segment in the
///      CK file specified by HANDLE, the results of this routine are
///      unpredictable.
///
///  6)  If the segment subtype is not recognized, the error
///      SPICE(NOTSUPPORTED) is signaled.
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
///  Note that the mini-segment interpolation window size is not
///  returned in the pointing record; this parameter is not required
///  in order to interpret the record. Call CKMP06 to obtain the
///  window size.
///
///  For a complete description of the internal structure of a type 6
///  segment, see the CK Required Reading.
///
///  This routine is normally used in conjunction with CKNM06 and
///  CKGM06 to obtain time tags and packet data from a specified type
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
///     Example code begins here.
///
///
///           PROGRAM CKGR06_EX1
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
pub fn ckgr06(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    msno: i32,
    recno: i32,
    record: &mut [f64],
) -> crate::Result<()> {
    CKGR06(handle, descr, msno, recno, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKGR06 ( C-kernel, get record, type 06 )
pub fn CKGR06(
    HANDLE: i32,
    DESCR: &[f64],
    MSNO: i32,
    RECNO: i32,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut BUFFER = StackArray::<f64, 4>::new(1..=BUFSIZ);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DPDATA = StackArray::<f64, 1>::new(1..=1);
    let mut RATE: f64 = 0.0;
    let mut BADDR: i32 = 0;
    let mut BUFBAS: i32 = 0;
    let mut EADDR: i32 = 0;
    let mut EPADDR: i32 = 0;
    let mut EPCBAS: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut MINIB: i32 = 0;
    let mut MINIE: i32 = 0;
    let mut NEPDIR: i32 = 0;
    let mut NINTVL: i32 = 0;
    let mut NREC: i32 = 0;
    let mut PKTSIZ: i32 = 0;
    let mut PTRBAS: i32 = 0;
    let mut SUBTYP: i32 = 0;

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKGR06", ctx)?;

    //
    // The number of discrete pointing instances contained in a data
    // type 6 segment is stored in the last double precision word of the
    // segment. Since the address of the last word is stored in the
    // sixth integer component of the segment descriptor, it is a
    // trivial matter to extract the count.
    //
    // The unpacked descriptor contains the following information
    // about the segment:
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
        CHKOUT(b"CKGR06", ctx)?;
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
        CHKOUT(b"CKGR06", ctx)?;
        return Ok(());
    }

    NINTVL = intrinsics::IDNINT(DPDATA[1]);

    if ((MSNO < 1) || (MSNO > NINTVL)) {
        SETMSG(b"Mini-segment index must be in range 1:# but was #.", ctx);
        ERRINT(b"#", NINTVL, ctx);
        ERRINT(b"#", MSNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"CKGR06", ctx)?;
        return Ok(());
    }

    //
    // Set the base address of the mini-segment pointers. There
    // are NINTVL+1 pointers, and these precede the control area.
    //
    PTRBAS = ((EADDR - CTRLSZ) - (NINTVL + 1));

    //
    // Compute the mini-segment pointers as absolute
    // DAF addresses. The stored value is a relative address.
    //
    BUFBAS = ((PTRBAS + MSNO) - 1);

    DAFGDA(
        HANDLE,
        (BUFBAS + 1),
        (BUFBAS + 2),
        BUFFER.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"CKGR06", ctx)?;
        return Ok(());
    }

    MINIB = ((BADDR - 1) + intrinsics::IDNINT(BUFFER[1]));
    MINIE = (((BADDR - 1) + intrinsics::IDNINT(BUFFER[2])) - 1);

    //
    // Fetch the control area of the mini-segment.
    //
    BUFBAS = (MINIE - MCTLSZ);

    DAFGDA(
        HANDLE,
        (BUFBAS + 1),
        (BUFBAS + MCTLSZ),
        BUFFER.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"CKGR06", ctx)?;
        return Ok(());
    }

    //
    // Fetch the SCLK rate (seconds per tick), mini-segment
    // subtype, and record count.
    //
    RATE = BUFFER[1];
    SUBTYP = intrinsics::IDNINT(BUFFER[2]);
    NREC = intrinsics::IDNINT(BUFFER[4]);

    //
    // Compute the packet size for this mini-segment. This will
    // be used a bit later. We'll also check the subtype.
    //
    if ((SUBTYP < C06TP0) || (SUBTYP > C06TP3)) {
        SETMSG(
            b"Unexpected CK type 6 subtype # found in mini-segment #.",
            ctx,
        );
        ERRINT(b"#", SUBTYP, ctx);
        ERRINT(b"#", MSNO, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"CKGR06", ctx)?;
        return Ok(());
    }

    PKTSIZ = save.PKTSZS[SUBTYP];

    //
    // Check the record index.
    //
    if ((RECNO < 1) || (RECNO > NREC)) {
        SETMSG(b"Record index must be in range 1:# but was #.", ctx);
        ERRINT(b"#", NREC, ctx);
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(CKNONEXISTREC)", ctx)?;
        CHKOUT(b"CKGR06", ctx)?;
        return Ok(());
    }

    //
    // The epochs of the mini-segment precede the
    // mini-segment's control area and the epoch directories.
    //
    NEPDIR = ((NREC - 1) / DIRSIZ);

    EPCBAS = (((MINIE - MCTLSZ) - NEPDIR) - NREC);

    //
    // Fetch the mini-segment's epoch at index RECNO into
    // element 1 of the output record.
    //
    EPADDR = (EPCBAS + RECNO);

    DAFGDA(HANDLE, EPADDR, EPADDR, RECORD.as_slice_mut(), ctx)?;

    //
    // Transfer the subtype and rate to the output record.
    //
    RECORD[2] = (SUBTYP as f64);
    RECORD[3] = RATE;

    //
    // Locate the data packet at index RECNO.
    //
    BUFBAS = ((MINIB - 1) + ((RECNO - 1) * PKTSIZ));

    DAFGDA(
        HANDLE,
        (BUFBAS + 1),
        (BUFBAS + PKTSIZ),
        RECORD.subarray_mut(4),
        ctx,
    )?;

    //
    // The record is complete if DAFGDA did its job. We don't
    // check FAILED here since we're about to return.
    //
    CHKOUT(b"CKGR06", ctx)?;
    Ok(())
}
