//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IMPLE: i32 = 0;
const IMPCLS: i32 = 1;
const EXPLT: i32 = 2;
const EXPLE: i32 = 3;
const EXPCLS: i32 = 4;
const MNIDXT: i32 = 0;
const MXIDXT: i32 = 4;
const CONBAS: i32 = 1;
const NCON: i32 = (CONBAS + 1);
const RDRBAS: i32 = (NCON + 1);
const NRDR: i32 = (RDRBAS + 1);
const RDRTYP: i32 = (NRDR + 1);
const REFBAS: i32 = (RDRTYP + 1);
const NREF: i32 = (REFBAS + 1);
const PDRBAS: i32 = (NREF + 1);
const NPDR: i32 = (PDRBAS + 1);
const PDRTYP: i32 = (NPDR + 1);
const PKTBAS: i32 = (PDRTYP + 1);
const NPKT: i32 = (PKTBAS + 1);
const RSVBAS: i32 = (NPKT + 1);
const NRSV: i32 = (RSVBAS + 1);
const PKTSZ: i32 = (NRSV + 1);
const PKTOFF: i32 = (PKTSZ + 1);
const NMETA: i32 = (PKTOFF + 1);
const MXMETA: i32 = NMETA;
const MNMETA: i32 = 15;
const NCONST: i32 = 8;
const NELEMS: i32 = 10;
const NANGS: i32 = 4;
const BEGEL1: i32 = (NCONST + 1);
const BEGEL2: i32 = ((BEGEL1 + NELEMS) + NANGS);
const ENSET1: i32 = ((BEGEL1 + NELEMS) - 1);
const ENSET2: i32 = ((BEGEL2 + NELEMS) - 1);

struct SaveVars {
    VALUE: f64,
    ENDS: StackArray<i32, 2>,
    FROM: i32,
    INDX: i32,
    NEPOCH: i32,
    TO: i32,
    PUTELM: i32,
    GETELM: i32,
    SET1: i32,
    SET2: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VALUE: f64 = 0.0;
        let mut ENDS = StackArray::<i32, 2>::new(1..=2);
        let mut FROM: i32 = 0;
        let mut INDX: i32 = 0;
        let mut NEPOCH: i32 = 0;
        let mut TO: i32 = 0;
        let mut PUTELM: i32 = 0;
        let mut GETELM: i32 = 0;
        let mut SET1: i32 = 0;
        let mut SET2: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            VALUE,
            ENDS,
            FROM,
            INDX,
            NEPOCH,
            TO,
            PUTELM,
            GETELM,
            SET1,
            SET2,
            FOUND,
        }
    }
}

/// SPK, read record from SPK type 10 segment
///
/// Read a single SPK data record from a segment of type 10
/// (NORAD two line element sets).
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
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
///           a SPK segment of type 10.
///
///  ET       is a target epoch, for which a data record from
///           a specific segment is required.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is the record from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative
///           to some center, in some inertial reference frame.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  It is assumed that the descriptor and handle supplied are
///      for a properly constructed type 10 segment. No checks are
///      performed to ensure this.
///
///  2)  If an error occurs while looking up SPK data, the error is
///      signaled by a routine in the call tree of this routine.
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
///  See the SPK Required Reading file for a description of the
///  structure of a data type 10 segment.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the SPKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the SPKRxx
///  routines might be used to "dump" and check segment data for a
///  particular epoch.
///
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL SPKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///  C
///  C     Look at parts of the descriptor.
///  C
///        CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///        CENTER = ICD( 2 )
///        REF    = ICD( 3 )
///        TYPE   = ICD( 4 )
///
///        IF ( TYPE .EQ. 1 ) THEN
///           CALL SPKR10 ( HANDLE, DESCR, ET, RECORD )
///               .
///               .  Look at the RECORD data.
///               .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 17-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Updated entry
///         #2 in $Exceptions section. Moved SPK required reading from
///         $Literature_References to $Required_Reading section.
///
/// -    SPICELIB Version 1.1.0, 09-MAR-2009 (EDW)
///
///         Removed declaration of unused variable DOINT.
///
/// -    SPICELIB Version 1.0.0, 05-JAN-1994 (WLT)
/// ```
pub fn spkr10(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR10(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR10 ( SPK, read record from SPK type 10 segment )
pub fn SPKR10(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // We have 2 nutation/obliquity terms and their rates giving us
    // four angle components for each packet.
    //
    //
    // BEGEL1 is the location in the record where the first
    // two-line element set will begin.
    //
    //
    // BEGEL2 is the location in the record where the second
    // two-line element set will begin.
    //
    //
    // ENSET1 and ENSET2 are the locations in the record where the
    // last element of set 1 and set 2 will be located.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKR10", ctx)?;

    //
    // Fetch the constants and store them in the first part of
    // the output RECORD.
    //
    SGFCON(
        HANDLE,
        DESCR.as_slice(),
        1,
        NCONST,
        RECORD.subarray_mut(1),
        ctx,
    )?;

    //
    // Locate the time in the file closest to the input ET.
    //
    SGFRVI(
        HANDLE,
        DESCR.as_slice(),
        ET,
        &mut save.VALUE,
        &mut save.INDX,
        &mut save.FOUND,
        ctx,
    )?;

    //
    // Determine which pair of element sets to choose so that
    // they will bracket ET.
    //
    if (ET <= save.VALUE) {
        save.FROM = intrinsics::MAX0(&[(save.INDX - 1), 1]);
        save.TO = save.INDX;
    } else {
        SGMETA(HANDLE, DESCR.as_slice(), NREF, &mut save.NEPOCH, ctx)?;
        save.FROM = save.INDX;
        save.TO = intrinsics::MIN0(&[(save.INDX + 1), save.NEPOCH]);
    }

    //
    // Fetch the element sets
    //
    SGFPKT(
        HANDLE,
        DESCR.as_slice(),
        save.FROM,
        save.TO,
        RECORD.subarray_mut(BEGEL1),
        save.ENDS.as_slice_mut(),
        ctx,
    )?;
    //
    // If the size of the packets is not 14, this is an old style
    // two-line element set without nutation information.  We simply
    // set all of the angles to zero.
    //
    if (save.ENDS[1] == NELEMS) {
        //
        // First shift the elements to their proper locations in RECORD
        // so there will be room to fill in the zeros.
        //
        save.PUTELM = ENSET2;
        save.GETELM = (ENSET1 + NELEMS);

        while (save.GETELM > ENSET1) {
            RECORD[save.PUTELM] = RECORD[save.GETELM];
            save.PUTELM = (save.PUTELM - 1);
            save.GETELM = (save.GETELM - 1);
        }

        save.SET1 = (ENSET1 + 1);
        save.SET2 = (ENSET2 + 1);

        for I in 1..=NANGS {
            RECORD[save.SET1] = 0.0;
            RECORD[save.SET2] = 0.0;
            save.SET1 = (save.SET1 + 1);
            save.SET2 = (save.SET2 + 1);
        }
    }

    //
    // If we only got one element set, ET  was either before the
    // first one in the segment or after the last one in the
    // segment.  We simply copy the one fetched a second time so
    // that the record is properly constructed.
    //
    if (save.FROM == save.TO) {
        MOVED(
            &RECORD.subarray(BEGEL1).to_vec(),
            (NELEMS + NANGS),
            RECORD.subarray_mut(BEGEL2),
        );
    }

    CHKOUT(b"SPKR10", ctx)?;
    Ok(())
}
