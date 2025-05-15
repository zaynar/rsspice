//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ND: i32 = 2;
const NI: i32 = 5;

/// PCK, read record from type 2 segment
///
/// Read a single PCK data record from a segment of type 2
/// (Chebyshev, 3-vector only).
///
/// # Required Reading
///
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   File handle.
///  DESCR      I   Segment descriptor.
///  ET         I   Target epoch.
///  RECORD     O   Data record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE,
///  DESCR    are the file handle and segment descriptor for
///           a PCK segment of type 2.
///
///  ET       is a target epoch, for which a data record from
///           a specific segment is required.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the record from the specified segment which,
///           when evaluated at epoch ET, will give the Euler
///           angles (orientation) of some body.
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
///  See the PCK Required Reading file for a description of the
///  structure of a data type 2 (Chebyshev polynomials, Euler
///  angles only) segment.
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
///  1) Dump the record of a type 2 PCK which, when evaluated at
///     a given epoch, will give the Euler angles (orientation) of
///     the Moon body-fixed frame with class ID 31004 with respect
///     to J2000.
///
///     Note that the data returned is in its rawest form, taken
///     directly from the segment. As such, it will be meaningless to
///     a user unless he/she understands the structure of the data
///     type completely. Given that understanding, however, the PCKR02
///     routine might be used to "dump" and check segment data for a
///     particular epoch.
///
///     Use the PCK kernel below to obtain the record.
///
///        moon_pa_de418_1950-2050.bpc
///
///
///     Example code begins here.
///
///
///           PROGRAM PCKR02_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           INTEGER               BODY
///           PARAMETER           ( BODY   = 31004 )
///
///           INTEGER               DESCSZ
///           PARAMETER           ( DESCSZ = 5     )
///
///           INTEGER               IDSIZE
///           PARAMETER           ( IDSIZE = 40    )
///
///     C
///     C     Set the maximum record size:
///     C
///     C        RSIZE = 2 + 3 * (PDEG +1)
///     C
///     C     Assume a maximum polynomial degree of 25, and
///     C     knowing that PCKR02 returns RSIZE as first element
///     C     of the output record...
///     C
///           INTEGER               RECRSZ
///           PARAMETER           ( RECRSZ = 81    )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(IDSIZE)    SEGID
///
///           DOUBLE PRECISION      BEGET
///           DOUBLE PRECISION      DESCR  ( DESCSZ )
///           DOUBLE PRECISION      ENDET
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      RECORD ( RECRSZ )
///
///           INTEGER               BADDR
///           INTEGER               BODYID
///           INTEGER               EADDR
///           INTEGER               FRAMID
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               INDEX
///           INTEGER               PCKHDL
///           INTEGER               PCKTYP
///           INTEGER               PDEG
///           INTEGER               RSIZE
///
///           LOGICAL               FOUND
///
///     C
///     C     Load the PCK file.
///     C
///           CALL PCKLOF ( 'moon_pa_de418_1950-2050.bpc', PCKHDL )
///
///     C
///     C     Set the epoch. Use ephemeris time of J2000 epoch.
///     C
///           ET = 0.D0
///
///     C
///     C     Get a segment applicable to a specified body and epoch.
///     C
///           CALL PCKSFS ( BODY, ET, HANDLE, DESCR, SEGID, FOUND )
///
///           IF ( FOUND ) THEN
///
///     C
///     C        Unpack the segment.
///     C
///              CALL PCKUDS ( DESCR, BODYID, FRAMID, PCKTYP,
///          .                 BEGET, ENDET,  BADDR,  EADDR  )
///
///
///              IF ( PCKTYP .EQ. 2 ) THEN
///
///                 CALL PCKR02 ( HANDLE, DESCR, ET, RECORD )
///
///                 RSIZE = RECORD(1)
///                 PDEG  = ( RSIZE - 2 ) / 3 - 1
///
///     C
///     C           Output the data.
///     C
///                 WRITE(*,*) 'Record size      : ', RSIZE
///                 WRITE(*,*) 'Polynomial degree: ', PDEG
///
///                 WRITE(*,*) 'Record data      :'
///                 WRITE(*,*) '   Interval midpoint: ', RECORD(2)
///                 WRITE(*,*) '   Interval radius  : ', RECORD(3)
///
///                 INDEX = 4
///                 WRITE(*,*) '   RA coefficients  : '
///                 DO I = 0, PDEG
///                    WRITE(*,*) '      ', RECORD(INDEX+I)
///                 END DO
///
///                 INDEX = 4 + ( PDEG + 1 )
///                 WRITE(*,*) '   DEC coefficients : '
///                 DO I = 0, PDEG
///                    WRITE(*,*) '      ', RECORD(INDEX+I)
///                 END DO
///
///                 INDEX = 4 + 2 * ( PDEG + 1 )
///                 WRITE(*,*) '   W coefficients   : '
///                 DO I = 0, PDEG
///                    WRITE(*,*) '      ', RECORD(INDEX+I)
///                 END DO
///
///              ELSE
///
///                 WRITE(*,*) 'PCK is not type 2'
///
///              END IF
///
///           ELSE
///
///              WRITE(*,*) '   ***** SEGMENT NOT FOUND *****'
///
///           END IF
///
///     C
///     C     Unload the PCK file.
///     C
///           CALL PCKUOF ( PCKHDL )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Record size      :           32
///      Polynomial degree:            9
///      Record data      :
///         Interval midpoint:    302400.00000000000
///         Interval radius  :    345600.00000000000
///         RA coefficients  :
///              -5.4242086033301107E-002
///              -5.2241405162792561E-005
///               8.9751456289930307E-005
///              -1.5288696963234620E-005
///               1.3218870864581395E-006
///               5.9822156790328180E-007
///              -6.5967702052551211E-008
///              -9.9084309118396298E-009
///               4.9276055963541578E-010
///               1.1612267413829385E-010
///         DEC coefficients :
///              0.42498898565916610
///               1.3999219324235620E-004
///              -1.8855140511098865E-005
///              -2.1964684808526649E-006
///               1.4229817868138752E-006
///              -1.6991716166847001E-007
///              -3.4824688140649506E-008
///               2.9208428745895990E-009
///               4.4217757657060300E-010
///              -3.9211207055305402E-012
///         W coefficients   :
///               2565.0633504619473
///              0.92003769451305328
///              -8.0503797901914501E-005
///               1.1960860244433900E-005
///              -1.2237900518372542E-006
///              -5.3651349407824562E-007
///               6.0843372260403005E-008
///               9.0211287487688797E-009
///              -4.6460429330339309E-010
///              -1.0446918704281774E-010
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
///  K.S. Zukor         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.2, 06-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example from existing fragment.
///
/// -    SPICELIB Version 1.1.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.0, 11-MAR-1993 (KSZ)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
/// ```
pub fn pckr02(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    PCKR02(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKR02 ( PCK, read record from type 2 segment )
pub fn PCKR02(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut INIT: f64 = 0.0;
    let mut INTLEN: f64 = 0.0;
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut IC = StackArray::<i32, 5>::new(1..=NI);
    let mut NREC: i32 = 0;
    let mut RECADR: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut RECSIZ: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"PCKR02", ctx)?;
    }

    //
    // Unpack the segment descriptor.
    //
    DAFUS(
        DESCR.as_slice(),
        ND,
        NI,
        DC.as_slice_mut(),
        IC.as_slice_mut(),
    );

    BEGIN = IC[(NI - 1)];
    END = IC[NI];

    //
    // The segment is made up of a number of logical records, each
    // having the same size, and covering the same length of time.
    //
    // We can determine which record to return by comparing the input
    // epoch with the initial time of the segment and the length of the
    // interval covered by each record.  These final two constants are
    // located at the end of the segment, along with the size of each
    // logical record and the total number of records.
    //
    DAFGDA(HANDLE, (END - 3), END, RECORD.as_slice_mut(), ctx)?;

    INIT = RECORD[1];
    INTLEN = RECORD[2];
    RECSIZ = (RECORD[3] as i32);
    NREC = (RECORD[4] as i32);

    RECNO = ((((ET - INIT) / INTLEN) as i32) + 1);
    RECNO = intrinsics::MIN0(&[RECNO, NREC]);

    //
    // Compute the address of the desired record.
    //
    RECADR = (((RECNO - 1) * RECSIZ) + BEGIN);

    //
    // Along with the record, return the size of the record.
    //
    RECORD[1] = RECORD[3];
    DAFGDA(
        HANDLE,
        RECADR,
        ((RECADR + RECSIZ) - 1),
        RECORD.subarray_mut(2),
        ctx,
    )?;

    CHKOUT(b"PCKR02", ctx)?;
    Ok(())
}
