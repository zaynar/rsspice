//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const RBSIZE: i32 = 100;
const RDSIZE: i32 = (128 * RBSIZE);

struct SaveVars {
    RBHAN: StackArray<i32, 100>,
    RBREC: StackArray<i32, 100>,
    RBREQ: StackArray<i32, 100>,
    RBDAT: ActualArray2D<f64>,
    RBNBR: i32,
    NREAD: i32,
    NREQ: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RBHAN = StackArray::<i32, 100>::new(1..=RBSIZE);
        let mut RBREC = StackArray::<i32, 100>::new(1..=RBSIZE);
        let mut RBREQ = StackArray::<i32, 100>::new(1..=RBSIZE);
        let mut RBDAT = ActualArray2D::<f64>::new(1..=128, 1..=RBSIZE);
        let mut RBNBR: i32 = 0;
        let mut NREAD: i32 = 0;
        let mut NREQ: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), RBSIZE as usize))
                .chain([]);

            RBHAN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), RBSIZE as usize))
                .chain([]);

            RBREC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), RBSIZE as usize))
                .chain([]);

            RBREQ
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), RDSIZE as usize))
                .chain([]);

            RBDAT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        RBNBR = 1;
        NREAD = 0;
        NREQ = 0;

        Self {
            RBHAN,
            RBREC,
            RBREQ,
            RBDAT,
            RBNBR,
            NREAD,
            NREQ,
        }
    }
}

/// DAF, read, write double precision
///
/// Read, write, and rewrite double precision records to and
/// from DAFs.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  HANDLE     I   DAFGDR. DAFGSR, DAFRDR (Obsolete), DAFWDR
///  RECNO      I   DAFGDR. DAFGSR, DAFRDR (Obsolete), DAFWDR
///  BEGIN      I   DAFGDR. DAFGSR, DAFRDR (Obsolete)
///  END        I   DAFGDR. DAFGSR, DAFRDR (Obsolete)
///  DREC       I   DAFWDR
///  DATA       O   DAFGDR. DAFGSR, DAFRDR (Obsolete)
///  FOUND      O   DAFGDR. DAFGSR, DAFRDR (Obsolete)
///  READS      O   DAFNRR
///  REQS       O   DAFNRR
///  RBSIZE     P   DAFGDR. DAFGSR, DAFRDR (Obsolete), DAFWDR, DAFNRR
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF.
///
///  RECNO    is the record number of a double precision record
///           within a DAF to be read or written.
///
///  BEGIN    is the first in word in a double precision record
///           to be read.
///
///  END      is the last in word in a double precision record
///           to be read.
///
///  DREC     contains a single double precision record, to be
///           written to the specified DAF.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     contains a portion of a single double precision
///           record, read from the specified DAF.
///
///  FOUND    is .TRUE. when the specified record is found, and is
///           .FALSE. otherwise.
///
///  READS,
///  REQS     are the number of physical reads and the number
///           of requests processed by DAFRDR during the current
///           execution of the calling program.
/// ```
///
/// # Parameters
///
/// ```text
///  RBSIZE   is the size of the record buffer maintained by
///           DAFRWD. In effect, RBSIZE is the maximum number
///           of records that can be stored (buffered) at any
///           one time. Higher values of RBSIZE reduce the
///           amount of time spent reading from disk at the
///           cost of increasing the amount of space required
///           by the calling program. The optimal value of
///           RBSIZE may differ from environment to environment,
///           and may even vary from application to application.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If DAFRWD is called directly, the error SPICE(BOGUSENTRY)
///      is signaled.
///
///  2)  See entry points DAFGDR, DAFGSR, DAFRDR, DAFWDR, and DAFNRR
///      for exceptions specific to those entry points.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFRWD serves as an umbrella, allowing data to be shared by its
///  entry points:
///
///     DAFGDR         Read double precision record.
///
///     DAFGSR         Read summary/descriptor record.
///
///     DAFRDR         Read double precision record. (Obsolete, use
///                    DAFGDR)
///
///     DAFWDR         Write double precision record.
///
///     DAFNRR         Number of reads, requests.
///
///  DAFGDR, DAFGSR, and DAFWDR are the only approved means for
///  reading and writing double precision records to and from DAFs.
///  DAFRDR continues to function, but only on files of the native
///  binary format. They keep track of which records have been read
///  most recently, and of which records have been requested most
///  often, in order to minimize the amount of time spent actually
///  reading from external storage.
///
///  DAFNRR may be used at any time during the execution of a
///  program to determine the number of requests that have been
///  processed, and the number of actual read operations needed
///  to fulfill those requests. Ideally, the ratio of reads to
///  requests should approach zero. In the worst case, the ratio
///  approaches one. The ratio is related to the size of the
///  record buffer, which controlled by parameter RBSIZE. The
///  results returned by DAFNRR may be used to determine the
///  optimal value of RBSIZE empirically.
///
///  All data records in a DAF can be treated as an undifferentiated
///  collection of double precision numbers. Summary records must
///  be read using the DAFGSR interface, but their contents are
///  properly buffered in a single buffer with the data records.
///  No special buffers are required for each new data type, or to
///  keep summary records separate from data records.
/// ```
///
/// # Examples
///
/// ```text
///  See entry points DAFGDR, DAFGSR, DAFRDR, DAFWDR, and DAFNRR
///  for examples specific to those entry points.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  An integer overflow may occur if the number of requests
///      by a single program exceeds the maximum number that can
///      be stored in an integer variable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 12-AUG-2021 (NJB) (JDR)
///
///         Bug fixes: now NREAD is not incremented once it reaches
///         INTMAX(), if it does.
///
///         Re-ordered header sections in all entry points. Corrected
///         typos in comments.
///
///         Edited the header of the DAFRWD umbrella routine and all its
///         entry entry points to comply with NAIF standard.
///
///         Removed DAF required reading from $Literature_References.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Added DAFGDR and DAFGSR entry points to allow read access
///         to DAFs utilizing non-native, but supported, binary file
///         formats.
///
///         DAFRDR was phased into obsolescence.
///
///         The umbrella no longer suffers from integer overflow if
///         a sufficient number of successful read requests are made.
///
///         DAFWDR no longer uses DAFHLU to retrieve a logical unit
///         for HANDLE. This call has been replaced with the handle
///         manager interface, which does not lock handles to their
///         logical units.
///
/// -    SPICELIB Version 1.3.0, 24-MAR-2000 (WLT)
///
///         The loop in DAFRDR that moved buffered d.p.s into the output
///         array DATA was modified to use the routine MOVED.
///
/// -    SPICELIB Version 1.2.0, 01-AUG-1997 (NJB)
///
///         Unnecessary CHKIN and CHKOUT calls were removed from entry
///         point DAFRDR.
///
/// -    SPICELIB Version 1.1.0, 25-NOV-1992 (JML)
///
///         1) In DAFRDR, the found flag is now set to false if the
///            call to DAFHLU fails.
///
///         2) In the example code fragment in DAFRDR and DAFWDR, the
///            calling sequence to MOVED was corrected.
///
///         3) In DAFRDR a variable name was changed.
///
///         4) In DAFNRR a cut and paste error in the header was fixed.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Updated this umbrella and its entry points in preparation
///         for DAF's utilization of the handle manager. DAFRDR is
///         obsolete, and will now signal errors when used to read
///         records from DAFs using non-native, binary file formats.
///
///         Two new entry points were added: DAFGDR and DAFGDR. These
///         are the translation-aware 'get data record' and 'get
///         summary record' routines that all new software developed
///         should utilize.
///
/// -    SPICELIB Version 1.3.0, 24-MAR-2000 (WLT)
///
///         The loop in DAFRDR that moved buffered d.p.s into the output
///         array DATA was modified to use the routine MOVED.
///
/// -    SPICELIB Version 1.2.0, 01-AUG-1997 (NJB)
///
///         Unnecessary CHKIN and CHKOUT calls were removed from entry
///         point DAFRDR.
/// ```
pub fn dafrwd(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    begin: i32,
    end: i32,
    drec: &[f64; 128],
    data: &[f64],
    found: bool,
    reads: i32,
    reqs: i32,
) -> crate::Result<()> {
    DAFRWD(
        handle,
        recno,
        begin,
        end,
        drec,
        data,
        found,
        reads,
        reqs,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRWD ( DAF, read, write double precision )
pub fn DAFRWD(
    HANDLE: i32,
    RECNO: i32,
    BEGIN: i32,
    END: i32,
    DREC: &[f64],
    DATA: &[f64],
    FOUND: bool,
    READS: i32,
    REQS: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
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
    // As double precision records are processed, they are stored in a
    // record buffer. (File and character records are not buffered.)
    // The user controls the number of records that may be stored at
    // any one time by setting the value of the parameter RBSIZE before
    // compiling the routine.
    //
    // The record buffer contains one entry for each record that has
    // been read.
    //
    //    +----------+----------+----------+----------+
    //    | File       Record     Request    Contents |
    //    | Handle     Number     Number              |
    //    +----------+----------+----------+----------+
    //    | INT        INT        INT        DP(128)  |
    //    +----------+----------+----------+----------+
    //
    // The request number is a counter that is incremented every time
    // a record is requested. When all the slots in the record buffer are
    // full, the least recently requested record (the one with the lowest
    // request number) is replaced by the new record.
    //
    // In addition, a separate counter is used to keep track of the
    // number of actual file reads performed. It is possible to tune
    // the entire package by checking the read/request ratio for
    // any specific buffer configuration.
    //
    // Note also that whenever a write operation fails, the affected
    // buffers (if any) should NOT be updated.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFRWD", ctx)?;
        SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
        CHKOUT(b"DAFRWD", ctx)?;
    }

    Ok(())
}

/// DAF, get double precision record
///
/// Read a portion of the contents of a double precision record in a
/// DAF file.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAF.
///  RECNO      I   Record number.
///  BEGIN      I   First word to read from record.
///  END        I   Last word to read from record.
///  DATA       O   Contents of record.
///  FOUND      O   .TRUE. if record is found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF.
///
///  RECNO    is the record number of a particular double precision
///           record within the DAF, whose contents are to be read.
///
///  BEGIN    is the first word in the specified record to be
///           returned.
///
///  END      is the final word in the specified record to be
///           returned.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     contains the specified portion (from BEGIN to END,
///           inclusive) of the specified record from the specified
///           file, specifically.
///
///  FOUND    is .TRUE. when the specified record is found, and is
///           .FALSE. otherwise.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFGDR checks the record buffer to see if the requested
///  record can be returned without actually reading it from
///  external storage. If not, it reads the record and stores
///  it in the buffer, typically removing another record from
///  the buffer as a result.
///
///  Once in the buffer, the specified portion of the record is
///  returned, using the following control loop.
///
///     J = 1
///     DO I = MAX( 1, BEGIN ), MIN( 128, END )
///        DATA( J ) = Buffered record ( I )
///        J = J + 1
///     END DO
///
///  Therefore bad values for BEGIN and END (BEGIN < 1, END < BEGIN,
///  etc.) are not signaled as errors, but result in the actions
///  implied by the above.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates one way that DAFGDR
///  and DAFWDR can be used to update part of a double precision
///  record. If the record does not yet exist, we can assume that
///  it is filled with zeros.
///
///     CALL DAFGDR ( HANDLE, RECNO, 1, 128, DREC, FOUND )
///
///     IF ( .NOT. FOUND ) THEN
///        CALL MOVED ( 0.D0, 128, DREC )
///     END IF
///
///     DO I = FIRST, LAST
///        DREC(I) = NEW_VALUE(I)
///     END DO
///
///     CALL DAFWDR ( HANDLE, RECNO, DREC )
///
///  Note that since only entire records may be written using DAFWDR,
///  the entire record needs to be read also.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Bad values for BEGIN and END ( BEGIN < 1, END > 128,
///      END < BEGIN ) are not signaled as errors. The effects of
///      such assignments on the returned data are defined by the
///      following control structure:
///
///         J = 1
///         DO I = MAX( 1, BEGIN ), MIN( 128, END )
///            DATA( J ) = Buffered record ( I )
///            J = J + 1
///         END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 12-AUG-2021 (NJB) (JDR)
///
///         Bug fix: now NREAD is not incremented once it reaches
///         INTMAX(), if it does.
///
///         Re-ordered header sections. Corrected typos in comments.
///         Edited the header to comply with NAIF standard.
///
///         Removed DAF required reading from $Literature_References.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
/// ```
pub fn dafgdr(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    begin: i32,
    end: i32,
    data: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    DAFGDR(handle, recno, begin, end, data, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFGDR ( DAF, get double precision record )
pub fn DAFGDR(
    HANDLE: i32,
    RECNO: i32,
    BEGIN: i32,
    END: i32,
    DATA: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut BUFLOC: i32 = 0;
    let mut DONE: bool = false;
    let mut STORED: bool = false;
    let mut B: i32 = 0;
    let mut COUNT: i32 = 0;
    let mut E: i32 = 0;
    let mut MINVAL: i32 = 0;
    let mut LOCFND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Assume that the record will be found until proven otherwise.
    //
    *FOUND = true;

    //
    // First, find the record.
    //
    // If the specified handle and record number match those of
    // a buffered record, determine the location of that record
    // within the buffer.
    //
    BUFLOC = 0;
    DONE = false;
    STORED = false;

    while !DONE {
        BUFLOC = (BUFLOC + 1);

        STORED = ((HANDLE == save.RBHAN[BUFLOC]) && (RECNO == save.RBREC[BUFLOC]));

        DONE = (STORED || (BUFLOC == save.RBNBR));
    }

    //
    // If not, determine the location of the least recently requested
    // record (the one with the smallest request number). Get the unit
    // number for the file, and read the record into this location.
    //
    // If an error occurs while reading the record, clear the entire
    // buffer entry in case the entry was corrupted by a partial read.
    // Otherwise, increment the number of reads performed so far.
    //
    if !STORED {
        MINAI(save.RBREQ.as_slice(), save.RBNBR, &mut MINVAL, &mut BUFLOC);

        ZZDAFGDR(
            HANDLE,
            RECNO,
            save.RBDAT.subarray_mut([1, BUFLOC]),
            &mut LOCFND,
            ctx,
        )?;

        //
        // If the call to ZZDAFGDR failed, or the record was not found,
        // then clean up.
        //
        if (FAILED(ctx) || !LOCFND) {
            *FOUND = false;
            save.RBHAN[BUFLOC] = 0;
            save.RBREC[BUFLOC] = 0;
            save.RBREQ[BUFLOC] = 0;
        } else {
            if (save.NREAD < INTMAX()) {
                save.NREAD = (save.NREAD + 1);
            }

            save.RBHAN[BUFLOC] = HANDLE;
            save.RBREC[BUFLOC] = RECNO;

            if (save.RBNBR < RBSIZE) {
                save.RBNBR = (save.RBNBR + 1);
            }
        }
    }

    //
    // Whether previously stored or just read, the record is now in
    // the buffer. Return the specified portion directly, and increment
    // the corresponding request number.
    //
    if *FOUND {
        B = intrinsics::MAX0(&[1, BEGIN]);
        E = intrinsics::MIN0(&[128, END]);
        COUNT = ((E - B) + 1);

        MOVED(save.RBDAT.subarray([B, BUFLOC]), COUNT, DATA.as_slice_mut());

        //
        // Increment the request counter in such a way that integer
        // overflow will not occur.  This private module from the
        // handle manager halves RBREQ if adding 1 to NREQ would
        // cause its value to exceed INTMAX.
        //
        ZZDDHRCM(save.RBNBR, save.RBREQ.as_slice_mut(), &mut save.NREQ);
        save.RBREQ[BUFLOC] = save.NREQ;
    }

    Ok(())
}

/// DAF, get summary/descriptor record
///
/// Read a portion of the contents of a summary record in a DAF file.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAF.
///  RECNO      I   Record number.
///  BEGIN      I   First word to read from record.
///  END        I   Last word to read from record.
///  DATA       O   Contents of record.
///  FOUND      O   .TRUE. if record is found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF.
///
///  RECNO    is the record number of a particular double precision
///           record within the DAF, whose contents are to be read. DAF
///           record numbers start at 1.
///
///  BEGIN    is the first word in the specified record to be returned.
///           Word numbers range from 1 to 128.
///
///  END      is the final word in the specified record to be returned.
///           Word numbers range from 1 to 128.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     contains the specified portion (from BEGIN to END,
///           inclusive) of the specified record from the specified
///           file.
///
///  FOUND    is .TRUE. when the specified record is found, and is
///           .FALSE. otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If HANDLE does not belong to any file that is currently open,
///      an error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If an error occurs while reading the record, the error is
///      signaled by a routine in the call tree of this routine.
///
///  3)  Bad values for BEGIN and END ( BEGIN < 1, END > 128,
///      END < BEGIN ) are not diagnosed. See $Particulars and
///      $Restrictions for the effect of this routine in this case.
/// ```
///
/// # Files
///
/// ```text
///  The input HANDLE must refer to a DAF file that is open for read
///  or write access.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFGSR checks the DAF record buffer to see if the requested
///  record can be returned without actually reading it from
///  external storage. If not, it reads the record and stores
///  it in the buffer, typically removing another record from
///  the buffer as a result.
///
///  Once in the buffer, the specified portion of the record is
///  returned, using the following control loop.
///
///     J = 1
///     DO I = MAX( 1, BEGIN ), MIN( 128, END )
///        DATA( J ) = Buffered record ( I )
///        J = J + 1
///     END DO
///
///  Therefore bad values for BEGIN and END (BEGIN < 1, END < BEGIN,
///  etc.) are not signaled as errors, but result in the actions
///  implied by the above.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates one way that DAFGSR
///  and DAFWDR can be used to update part of a summary record.
///  If the record does not yet exist, we can assume that it is
///  filled with zeros.
///
///     CALL DAFGSR ( HANDLE, RECNO, 1, 128, DREC, FOUND )
///
///     IF ( .NOT. FOUND ) THEN
///        CALL MOVED ( 0.D0, 128, DREC )
///     END IF
///
///     DO I = FIRST, LAST
///        DREC(I) = NEW_VALUE(I)
///     END DO
///
///     CALL DAFWDR ( HANDLE, RECNO, DREC )
///
///  Note that since only entire records may be written using DAFWDR,
///  the entire record needs to be read also.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Bad values for BEGIN and END ( BEGIN < 1, END > 128,
///      END < BEGIN ) are not signaled as errors. The effects of
///      such assignments on the returned data are defined by the
///      following control structure:
///
///         J = 1
///         DO I = MAX( 1, BEGIN ), MIN( 128, END )
///            DATA( J ) = Buffered record ( I )
///            J = J + 1
///         END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 09-AUG-2021 (NJB) (JDR)
///
///         Bug fix: now NREAD is not incremented once it reaches
///         INTMAX(), if it does.
///
///         Corrected typos in comments. Edited the header to comply with
///         NAIF standard. Added $Exceptions and $Files sections and
///         extended $Detailed_Input.
///
///         Removed DAF required reading from $Literature_References.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
/// ```
pub fn dafgsr(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    begin: i32,
    end: i32,
    data: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    DAFGSR(handle, recno, begin, end, data, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFGSR ( DAF, get summary/descriptor record )
pub fn DAFGSR(
    HANDLE: i32,
    RECNO: i32,
    BEGIN: i32,
    END: i32,
    DATA: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut BUFLOC: i32 = 0;
    let mut DONE: bool = false;
    let mut STORED: bool = false;
    let mut B: i32 = 0;
    let mut COUNT: i32 = 0;
    let mut E: i32 = 0;
    let mut MINVAL: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut LOCFND: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Assume that the record will be found until proven otherwise.
    //
    *FOUND = true;

    //
    // First, find the record.
    //
    // If the specified handle and record number match those of
    // a buffered record, determine the location of that record
    // within the buffer.
    //
    BUFLOC = 0;
    DONE = false;
    STORED = false;

    while !DONE {
        BUFLOC = (BUFLOC + 1);

        STORED = ((HANDLE == save.RBHAN[BUFLOC]) && (RECNO == save.RBREC[BUFLOC]));

        DONE = (STORED || (BUFLOC == save.RBNBR));
    }

    //
    // If not, determine the location of the least recently requested
    // record (the one with the smallest request number). Get the unit
    // number for the file, and read the record into this location.
    //
    // If an error occurs while reading the record, clear the entire
    // buffer entry in case the entry was corrupted by a partial read.
    // Otherwise, increment the number of reads performed so far.
    //
    if !STORED {
        MINAI(save.RBREQ.as_slice(), save.RBNBR, &mut MINVAL, &mut BUFLOC);

        DAFHSF(HANDLE, &mut ND, &mut NI, ctx)?;

        ZZDAFGSR(
            HANDLE,
            RECNO,
            ND,
            NI,
            save.RBDAT.subarray_mut([1, BUFLOC]),
            &mut LOCFND,
            ctx,
        )?;

        //
        // If the call to ZZDAFGSR or DAFHSF failed, or the record
        // was not found, then clean up.
        //
        if (FAILED(ctx) || !LOCFND) {
            *FOUND = false;
            save.RBHAN[BUFLOC] = 0;
            save.RBREC[BUFLOC] = 0;
            save.RBREQ[BUFLOC] = 0;
        } else {
            if (save.NREAD < INTMAX()) {
                save.NREAD = (save.NREAD + 1);
            }

            save.RBHAN[BUFLOC] = HANDLE;
            save.RBREC[BUFLOC] = RECNO;

            if (save.RBNBR < RBSIZE) {
                save.RBNBR = (save.RBNBR + 1);
            }
        }
    }

    //
    // Whether previously stored or just read, the record is now in
    // the buffer. Return the specified portion directly, and increment
    // the corresponding request number.
    //
    if *FOUND {
        B = intrinsics::MAX0(&[1, BEGIN]);
        E = intrinsics::MIN0(&[128, END]);
        COUNT = ((E - B) + 1);

        MOVED(save.RBDAT.subarray([B, BUFLOC]), COUNT, DATA.as_slice_mut());

        //
        // Increment the request counter in such a way that integer
        // overflow will not occur.  This private module from the
        // handle manager halves RBREQ if adding 1 to NREQ would
        // cause its value to exceed INTMAX.
        //
        ZZDDHRCM(save.RBNBR, save.RBREQ.as_slice_mut(), &mut save.NREQ);
        save.RBREQ[BUFLOC] = save.NREQ;
    }

    Ok(())
}

/// DAF, read double precision record
///
/// Read a portion of the contents of a double precision record in a
/// DAF file.
/// Obsolete: This routine has been superseded by DAFGDR, and it is
/// supported for purposes of backwards compatibility only.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAF.
///  RECNO      I   Record number.
///  BEGIN      I   First word to read from record.
///  END        I   Last word to read from record.
///  DATA       O   Contents of record.
///  FOUND      O   .TRUE. if record is found.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF.
///
///  RECNO    is the record number of a particular double precision
///           record within the DAF, whose contents are to be read.
///
///  BEGIN    is the first word in the specified record to be
///           returned.
///
///  END      is the final word in the specified record to be
///           returned.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATA     contains the specified portion (from BEGIN to END,
///           inclusive) of the specified record from the specified
///           file, specifically.
///
///  FOUND    is .TRUE. when the specified record is found, and is
///           .FALSE. otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the file associated with HANDLE is not of the native
///      binary file format, the error SPICE(UNSUPPORTEDBFF) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  DAFRDR checks the record buffer to see if the requested
///  record can be returned without actually reading it from
///  external storage. If not, it reads the record and stores
///  it in the buffer, typically removing another record from
///  the buffer as a result.
///
///  Once in the buffer, the specified portion of the record is
///  returned, using the following control loop.
///
///     J = 1
///     DO I = MAX( 1, BEGIN ), MIN( 128, END )
///        DATA( J ) = Buffered record ( I )
///        J = J + 1
///     END DO
///
///  Therefore bad values for BEGIN and END (BEGIN < 1, END < BEGIN,
///  etc.) are not signaled as errors, but result in the actions
///  implied by the above.
///
///  This routine has been made obsolete by the routine DAFGDR,
///  and it is supported for reasons of backwards compatibility
///  only. New software development should utilize DAFGDA.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates one way that DAFRDR
///  and DAFWDR can be used to update part of a double precision
///  record. If the record does not yet exist, we can assume that
///  it is filled with zeros.
///
///     CALL DAFRDR ( HANDLE, RECNO, 1, 128, DREC, FOUND )
///
///     IF ( .NOT. FOUND ) THEN
///        CALL MOVED ( 0.D0, 128, DREC )
///     END IF
///
///     DO I = FIRST, LAST
///        DREC(I) = NEW_VALUE(I)
///     END DO
///
///     CALL DAFWDR ( HANDLE, RECNO, DREC )
///
///  Note that since only entire records may be written using DAFWDR,
///  the entire record needs to be read also.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  An integer overflow may occur if the number of requests
///      by a single program exceeds the maximum number that can
///      be stored in an integer variable.
///
///  2)  Bad values for BEGIN and END ( BEGIN < 1, END > 128,
///      END < BEGIN ) are not signaled as errors. The effects of
///      such assignments on the returned data are defined by the
///      following control structure:
///
///         J = 1
///         DO I = MAX( 1, BEGIN ), MIN( 128, END )
///            DATA( J ) = Buffered record ( I )
///            J = J + 1
///         END DO
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 12-AUG-2021 (NJB) (JDR)
///
///         Bug fix: now NREAD is not incremented once it reaches
///         INTMAX(), if it does.
///
///         Tweaked one line of code to suppress ftnchek warning.
///
///         Re-ordered header sections. Corrected typos in comments.
///         Edited the header to comply with NAIF standard.
///
///         Removed DAF required reading from $Literature_References.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Added SPICE(UNSUPPORTEDBFF) exception to the routine.
///
/// -    SPICELIB Version 1.3.0, 24-MAR-2000 (WLT)
///
///         The loop in DAFRDR that moved buffered d.p.s into the output
///         array DATA was modified to use the routine MOVED.
///
/// -    SPICELIB Version 1.2.0, 01-AUG-1997 (NJB)
///
///         Unnecessary CHKIN and CHKOUT calls were removed from entry
///         point DAFRDR.
///
/// -    SPICELIB Version 1.1.0, 25-NOV-1992 (JML)
///
///         1) In DAFRDR, the found flag is now set to false if the
///            call to DAFHLU fails.
///
///         2) In the example code fragment in DAFRDR and DAFWDR, the
///            calling sequence to MOVED was corrected.
///
///         3) In the call to MINAI the argument for the minimum value
///            was changed from I to MINVAL.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         The exception SPICE(UNSUPPORTEDBFF) was added to guarantee
///         this routine's functionality remains unchanged as a result
///         of the updates to the underlying DAF software's utilization of
///         the handle manager. In versions of the toolkit prior to this,
///         all DAFs loaded were of the native binary file format.
///         Previously, this routine was used to read the contents of
///         summary records in addition to the usual data records.
///         The non-native to native translation process for these two
///         different types of records in general are not the same.
///         Rather than attempt to interpret the caller's intent, this
///         routine is obsolete and restricted to functioning only on
///         DAFs of the native binary file format.
///
/// -    SPICELIB Version 1.3.0, 24-MAR-2000 (WLT)
///
///         The loop in DAFRDR that moved buffered d.p.s into the output
///         array DATA was modified to use the routine MOVED.
///
/// -    SPICELIB Version 1.2.0, 01-AUG-1997 (NJB)
///
///         Unnecessary CHKIN and CHKOUT calls were removed from entry
///         point DAFRDR. These calls were placed together prior to
///         a RETURN statement. It's unclear why they were there in the
///         first place.
///
/// -    SPICELIB Version 1.1.0, 25-NOV-1992 (JML)
///
///         1) In DAFRDR, the found flag is now set to false if the
///            call to DAFHLU fails.
///
///         2) In the example code fragment in DAFRDR and DAFWDR, the
///            calling sequence to MOVED was corrected.
///
///         3) In the call to MINAI the argument for the minimum value
///            was changed from I to MINVAL.
///
/// -     Beta Version 2.0.0, 1-NOV-1989 (RET)
///
///         The function of DAFRDR was changed so that it returns only
///         a specified portion of the record. The calling sequence there-
///         fore changed from
///
///            DAFRDR ( HANDLE, RECNO, DREC, FOUND ) to
///            DAFRDR ( HANDLE, RECNO, BEGIN, END, DATA, FOUND )
///
///         The change was made to cut down on the shuffling of unneeded
///         data.
///
///         Also, DAFRDR now only checks in and checks out if DAFHLU has
///         failed (the only routine called by DAFRDR that could possibly
///         signal an error). The purpose of this change was to help
///         speed up a routine that gets called constantly by higher level
///         DAF routines.
/// ```
pub fn dafrdr(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    begin: i32,
    end: i32,
    data: &mut [f64],
    found: &mut bool,
) -> crate::Result<()> {
    DAFRDR(handle, recno, begin, end, data, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFRDR ( DAF, read double precision record )
pub fn DAFRDR(
    HANDLE: i32,
    RECNO: i32,
    BEGIN: i32,
    END: i32,
    DATA: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DATA = DummyArrayMut::new(DATA, 1..);
    let mut BUFLOC: i32 = 0;
    let mut DONE: bool = false;
    let mut STORED: bool = false;
    let mut B: i32 = 0;
    let mut COUNT: i32 = 0;
    let mut E: i32 = 0;
    let mut MINVAL: i32 = 0;
    let mut LOCFND: bool = false;
    let mut NATIVE: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Assume that the record will be found until proven otherwise.
    //
    *FOUND = true;

    //
    // First check to see if HANDLE is associated with a DAF of the
    // native binary file format.
    //
    ZZDDHISN(HANDLE, &mut NATIVE, &mut LOCFND, ctx)?;

    if (LOCFND && !NATIVE) {
        *FOUND = false;

        CHKIN(b"DAFRDR", ctx)?;
        SETMSG(b"The binary file format for file \'#\' is not native. This routine operates only on files of the native format.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(UNSUPPORTEDBFF)", ctx)?;
        CHKOUT(b"DAFRDR", ctx)?;
        return Ok(());
    }

    //
    // Now, find the record.
    //
    // If the specified handle and record number match those of
    // a buffered record, determine the location of that record
    // within the buffer.
    //
    BUFLOC = 0;
    DONE = false;
    STORED = false;

    while !DONE {
        BUFLOC = (BUFLOC + 1);

        STORED = ((HANDLE == save.RBHAN[BUFLOC]) && (RECNO == save.RBREC[BUFLOC]));

        DONE = (STORED || (BUFLOC == save.RBNBR));
    }

    //
    // If not, determine the location of the least recently requested
    // record (the one with the smallest request number). Get the unit
    // number for the file, and read the record into this location.
    //
    // If an error occurs while reading the record, clear the entire
    // buffer entry in case the entry was corrupted by a partial read.
    // Otherwise, increment the number of reads performed so far.
    //
    if !STORED {
        MINAI(save.RBREQ.as_slice(), save.RBNBR, &mut MINVAL, &mut BUFLOC);

        ZZDAFGDR(
            HANDLE,
            RECNO,
            save.RBDAT.subarray_mut([1, BUFLOC]),
            &mut LOCFND,
            ctx,
        )?;

        //
        // If the call to ZZDAFGDR failed, or the record was not found,
        // then clean up.
        //
        if (FAILED(ctx) || !LOCFND) {
            *FOUND = false;
            save.RBHAN[BUFLOC] = 0;
            save.RBREC[BUFLOC] = 0;
            save.RBREQ[BUFLOC] = 0;
        } else {
            if (save.NREAD < INTMAX()) {
                save.NREAD = (save.NREAD + 1);
            }

            save.RBHAN[BUFLOC] = HANDLE;
            save.RBREC[BUFLOC] = RECNO;

            if (save.RBNBR < RBSIZE) {
                save.RBNBR = (save.RBNBR + 1);
            }
        }
    }

    //
    // Whether previously stored or just read, the record is now in
    // the buffer. Return the specified portion directly, and increment
    // the corresponding request number.
    //
    if *FOUND {
        B = intrinsics::MAX0(&[1, BEGIN]);
        E = intrinsics::MIN0(&[128, END]);
        COUNT = ((E - B) + 1);

        MOVED(save.RBDAT.subarray([B, BUFLOC]), COUNT, DATA.as_slice_mut());

        //
        // Increment the request counter in such a way that integer
        // overflow will not occur.  This private module from the
        // handle manager halves RBREQ if adding 1 to NREQ would
        // cause its value to exceed INTMAX.
        //
        ZZDDHRCM(save.RBNBR, save.RBREQ.as_slice_mut(), &mut save.NREQ);
        save.RBREQ[BUFLOC] = save.NREQ;
    }

    Ok(())
}

/// DAF, write double precision record
///
/// Write or rewrite the contents of a double precision record in
/// a DAF.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAF.
///  RECNO      I   Record number.
///  DREC       I   Contents of record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle associated with a DAF.
///
///  RECNO    is the record number of a particular double
///           precision record within the file, whose
///           contents are to be written (if the record does
///           not yet exist) or overwritten (if it does).
///
///  DREC     contains the new contents of the record.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the file is not open for write access, the error
///      SPICE(DAFILLEGWRITE) is signaled.
///
///  2)  If (for some reason) the record cannot be written, the
///      error SPICE(DAFDPWRITEFAIL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Like DAFRDR, DAFWDR checks the record buffer to see if the
///  requested record is in the buffer. If so, the buffer is
///  updated along with the file. This prevents the buffer from
///  becoming outdated.
/// ```
///
/// # Examples
///
/// ```text
///  The following code fragment illustrates one way that DAFRDR
///  and DAFWDR can be used to update part of a double precision
///  record. If the record does not yet exist, we can assume that
///  it is filled with zeros.
///
///     CALL DAFRDR ( HANDLE, RECNO, DREC, FOUND )
///
///     IF ( .NOT. FOUND ) THEN
///        CALL MOVED ( 0.D0, 128, DREC )
///     END IF
///
///     DO I = FIRST, LAST
///        DREC(I) = NEW_VALUE(I)
///     END DO
///
///     CALL DAFWDR ( HANDLE, RECNO, DREC )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 09-AUG-2021 (NJB) (JDR)
///
///         Re-ordered header sections. Corrected typos in comments.
///         Edited the header to comply with NAIF standard.
///
///         Removed DAF required reading from $Literature_References.
///
/// -    SPICELIB Version 2.0.0, 16-NOV-2001 (FST)
///
///         Replaced the call to DAFHLU to ZZDDHHLU. This prevents
///         DAFWDR from tying up resources in the handle manager.
///
/// -    SPICELIB Version 1.3.0, 24-MAR-2000 (WLT)
///
///         The loop in DAFRDR that moved buffered d.p.s into the output
///         array DATA was modified to use the routine MOVED.
///
/// -    SPICELIB Version 1.1.0, 25-NOV-1992 (JML)
///
///         In the example code fragment in DAFRDR and DAFWDR, the
///         calling sequence to MOVED was corrected.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafwdr(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    drec: &[f64; 128],
) -> crate::Result<()> {
    DAFWDR(handle, recno, drec, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFWDR ( DAF, write double precision record )
pub fn DAFWDR(HANDLE: i32, RECNO: i32, DREC: &[f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DREC = DummyArray::new(DREC, 1..=128);
    let mut UNIT: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut BUFLOC: i32 = 0;
    let mut DONE: bool = false;
    let mut STORED: bool = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFWDR", ctx)?;
    }

    //
    // No fair writing to a read-only file!
    //
    if (HANDLE >= 0) {
        SETMSG(b"Attempt was made to write to a read-only file.", ctx);
        SIGERR(b"SPICE(DAFILLEGWRITE)", ctx)?;

        CHKOUT(b"DAFWDR", ctx)?;
        return Ok(());
    }

    //
    // If the specified handle and record number match those of
    // a buffered record, determine the location of that record
    // within the buffer.
    //
    BUFLOC = 0;
    DONE = false;
    STORED = false;

    while !DONE {
        BUFLOC = (BUFLOC + 1);

        STORED = ((HANDLE == save.RBHAN[BUFLOC]) && (RECNO == save.RBREC[BUFLOC]));

        DONE = (STORED || (BUFLOC == RBSIZE));
    }

    //
    // Get the unit number for the file, and write the record.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut UNIT, ctx)?;

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            for n in DREC.iter() {
                writer.write_f64(*n)?;
            }
            writer.finish()?;
            Ok(())
        })?;
    }

    //
    // If the record was buffered, replace it---with the input
    // record if the write was successful, or with zeros if it
    // was not.
    //
    if STORED {
        if (IOSTAT == 0) {
            MOVED(DREC.as_slice(), 128, save.RBDAT.subarray_mut([1, BUFLOC]));
        } else {
            save.RBHAN[BUFLOC] = 0;
            save.RBREC[BUFLOC] = 0;
            save.RBREQ[BUFLOC] = 0;
        }
    }

    //
    // Declare an error if the write failed.
    //
    if (IOSTAT != 0) {
        SETMSG(b"Double precision write failed. Value of IOSTAT was #", ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFDPWRITEFAIL)", ctx)?;
    }

    CHKOUT(b"DAFWDR", ctx)?;
    Ok(())
}

/// DAF number of reads, requests
///
/// Return the number of reads and requests fielded by DAFRDR.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  READS,
///  REQS       O   Reads, requests in this execution.
/// ```
///
/// # Detailed Output
///
/// ```text
///  READS,
///  REQS     are the number of physical reads and the number
///           of requests processed by DAFRDR during the current
///           execution of the calling program.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  The ratio of reads to requests tells you something about
///  the effectiveness with which the record buffer is preventing
///  unwanted disk access. In the ideal case, most of the records
///  needed by the calling program can be returned directly from
///  the buffer, and the ratio of reads to requests approaches zero.
///  More realistically, it should be be somewhere between 1/10
///  and 1/2.
///
///  If the ratio is greater than 1/2, you should consider increasing
///  the size of the record buffer (which is controlled by parameter
///  RBSIZE) in order to improve the performance of the DAF package,
///  unless your application is strapped for space.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, the ratio of reads to requests
///  is determined following a series of calls to the reader DAFEZ.
///
///     DO I = 1, N
///        CALL DAFEZ ( ..., STATES(1,I), ... )
///     END DO
///
///     CALL DAFNRR ( READS, REQS )
///
///     WRITE (*,*) 'Reads/requests = ', FLOAT( READS ) / FLOAT( REQS )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J.M. Lynch         (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.1, 09-AUG-2021 (NJB)
///
///         Re-ordered header sections. Corrected typos in comments.
///         Edited the header to comply with NAIF standard.
///
///         Removed DAF required reading from $Literature_References.
///
/// -    SPICELIB Version 1.3.0, 24-MAR-2000 (WLT)
///
///         The loop in DAFRDR that moved buffered d.p.s into the output
///         array DATA was modified to use the routine MOVED.
///
/// -    SPICELIB Version 1.1.0, 25-NOV-1992 (JML)
///
///         A cut and paste error in the literature references
///         section of the header was fixed.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafnrr(ctx: &mut SpiceContext, reads: &mut i32, reqs: &mut i32) {
    DAFNRR(reads, reqs, ctx.raw_context());
}

//$Procedure DAFNRR ( DAF number of reads, requests )
pub fn DAFNRR(READS: &mut i32, REQS: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *READS = save.NREAD;
    *REQS = save.NREQ;
}
