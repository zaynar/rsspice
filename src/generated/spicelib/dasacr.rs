//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NWC: i32 = 1024;
const NWD: i32 = 128;
const NWI: i32 = 256;
const CHAR: i32 = 1;
const DP: i32 = 2;
const BWDLOC: i32 = 1;
const FWDLOC: i32 = 2;
const BEGDSC: i32 = 9;

struct SaveVars {
    NEXT: StackArray<i32, 3>,
    PREV: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);
        let mut PREV = StackArray::<i32, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(3), Val::I(1), Val::I(2)].into_iter();
            PREV.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { NEXT, PREV }
    }
}

/// DAS, add comment records
///
/// Increase the size of the comment area in a DAS file to accommodate
/// a specified number of additional comment records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   A DAS file handle.
///  N          I   Number of comment records to append to the comment
///                 area of the specified file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an existing DAS file opened for
///           comment area modification by DASOPC.
///
///  N        is the number of records to append to the comment
///           area. If NCOMR is the number of comment records
///           present in the file on input, then on output the
///           number of comment records will be NCOMR + N.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the effect of this
///  routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input handle is invalid, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If an I/O error occurs during the addition process, the error
///      is signaled by a routine in the call tree of this routine. The
///      DAS file will probably be corrupted in this case.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is used to create space in the comment area of a DAS
///  file to allow addition of comments to the file. If there are
///  comment records present in the file at the time this routine is
///  called, the number of comment records specified by the input
///  argument N will be appended to the existing comment records.
///  In any case, any existing directory records and data records will
///  be shifted down by N records.
///
///  This routine updates the file record of the specified DAS file
///  to reflect the addition of records to the file's comment area.
///  Also, the file summary obtainable from DASHFS will be updated to
///  reflect the addition of comment records.
///
///  This routine may be used only on existing DAS files opened by
///  DASOPW.
///
///  The association of DAS logical addresses and data within the
///  specified file will remain unaffected by use of this routine.
///
///  Normally, SPICELIB applications will not call this routine
///  directly, but will add comments by calling DASAC.
///
///  This routine has an inverse DASRCR, which removes a specified
///  number of records from the end of the comment area.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Make room for 10 comment records in the comment area of a
///      new DAS file.
///
///         C
///         C     Create a new DAS file.
///         C
///               CALL DASOPW ( DAS, HANDLE )
///
///         C
///         C     Now add 10 comment records to the comment area.
///         C
///               CALL DASACR ( HANDLE, 10 )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The DAS file must have a binary file format native to the host
///      system.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 05-FEB-2015 (NJB)
///
///         Updated to support integration with the handle
///         manager subsystem.
///
///         Cleaned up use of unnecessary variables and unneeded
///         declarations.
///
/// -    SPICELIB Version 1.1.0, 11-OCT-1996 (NJB)
///
///         Bug fix: backward and forward directory record pointers
///         are now updated when directory records are moved.
///
/// -    SPICELIB Version 1.0.0, 01-FEB-1993 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 11-OCT-1996 (NJB)
///
///         Bug fix: backward and forward directory record pointers
///         are now updated when directory records are moved.
///
///         Because these pointers are not used by the DAS software
///         once a DAS file is segregated, this bug had no effect on
///         DAS files that were created and closed via DASCLS, then
///         commented via the commnt utility.
/// ```
pub fn dasacr(ctx: &mut SpiceContext, handle: i32, n: i32) -> crate::Result<()> {
    DASACR(handle, n, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASACR ( DAS, add comment records )
pub fn DASACR(HANDLE: i32, N: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut RECC = [b' '; NWC as usize];
    let mut RECD = StackArray::<f64, 128>::new(1..=NWD);
    let mut BASE: i32 = 0;
    let mut DIRREC = StackArray::<i32, 256>::new(1..=NWI);
    let mut FREE: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut LOC: i32 = 0;
    let mut LREC: i32 = 0;
    let mut LINDEX: i32 = 0;
    let mut LWORD: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NREC: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NXTTYP: i32 = 0;
    let mut POS: i32 = 0;
    let mut PREC: i32 = 0;
    let mut RECI = StackArray::<i32, 256>::new(1..=NWI);
    let mut TYPE: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Words per data record, for each data type:
    //

    //
    // Data type parameters
    //

    //
    // Directory pointer locations (backward and forward):
    //

    //
    // Location of first type descriptor
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // NEXT and PREV map the DAS data type codes to their
    // successors and predecessors, respectively.
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

    CHKIN(b"DASACR", ctx)?;

    //
    // Programmer's note: the calls to
    //
    //    DASIOC
    //    DASIOD
    //    DASIOI
    //
    // for read access are valid only for native format DAS files.
    // If this routine is updated to support writing to non-native
    // DAS files, at least the calls to the numeric I/O routines
    // will need to be replaced. (Consider using ZZDASGRD, ZZDASGRI.)
    //
    // Make sure this DAS file is open for writing.  Signal an error if
    // not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    //
    // Get the logical unit for this DAS file.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut UNIT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASACR", ctx)?;
        return Ok(());
    }

    //
    // It's a mistake to use a negative value of N.
    //
    if (N < 0) {
        SETMSG(b"Number of comment records to add must be non-negative.  Actual number requested was #.", ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(DASINVALIDCOUNT)", ctx)?;
        CHKOUT(b"DASACR", ctx)?;
        return Ok(());
    }

    //
    // Before doing anything to the file, make sure that the DASRWR
    // data buffers do not contain any updated records for this file.
    // All of the record numbers that pertain to this file and remain
    // in the DASRWR buffers will be invalidated after this routine
    // returns.
    //
    // DASWBR flushes buffered records to the file.
    //
    DASWBR(HANDLE, ctx)?;

    //
    // Grab the file summary for this DAS file.  Find the number of
    // comment records and the number of the first free record.
    //
    DASHFS(
        HANDLE,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        &mut FREE,
        LASTLA.as_slice_mut(),
        LASTRC.as_slice_mut(),
        LASTWD.as_slice_mut(),
        ctx,
    )?;

    //
    // Find the record and word positions LREC and LWORD of the last
    // descriptor in the file.
    //
    MAXAI(LASTRC.as_slice(), 3, &mut LREC, &mut LOC);
    LWORD = 0;

    for I in 1..=3 {
        if ((LASTRC[I] == LREC) && (LASTWD[I] > LWORD)) {
            LWORD = LASTWD[I];
        }
    }

    //
    // LREC and LWORD are now the record and word index of the last word
    // of the last descriptor in the file. If LREC is zero, there are no
    // directories in the file yet. However, even DAS files that don't
    // contain any data have their first directory records zeroed out,
    // and this should remain true after the addition of the comment
    // records.
    //
    if (LREC == 0) {
        //
        // Just write the zero-filled record to record number
        //
        //    NRESVR + NCOMR + N + 2
        //
        CLEARI(NWI, DIRREC.as_slice_mut());
        DASIOI(
            b"WRITE",
            UNIT,
            (((NRESVR + NCOMR) + N) + 2),
            DIRREC.as_slice_mut(),
            ctx,
        )?;
    } else {
        //
        // There really is stuff to move.  For each directory record,
        // move all of the records described by that directory.  We start
        // with the last directory and work our way toward the beginning
        // of the file.
        //
        NREC = LREC;

        while (NREC > 0) {
            //
            // For each descriptor in the current directory, move the
            // cluster of data records it refers to.
            //
            // Read the current directory record.
            //
            DASIOI(b"READ", UNIT, NREC, DIRREC.as_slice_mut(), ctx)?;

            //
            // Find the data type, size, and base record number of the
            // last cluster in the current directory.  To do this,
            // traverse the directory record, keeping track of the record
            // count and data types of descriptors as we go.
            //
            TYPE = DIRREC[BEGDSC];
            BASE = (NREC + 1);

            if (NREC == LREC) {
                LINDEX = LWORD;
            } else {
                LINDEX = NWI;
            }

            for I in (BEGDSC + 2)..=LINDEX {
                if (DIRREC[I] < 0) {
                    TYPE = save.PREV[TYPE];
                } else {
                    TYPE = save.NEXT[TYPE];
                }

                BASE = (BASE + i32::abs(DIRREC[(I - 1)]));
            }

            //
            // TYPE and BASE are now the data type and base record number
            // of the last cluster described by the current directory.
            //
            // We'll now traverse the directory in reverse order, keeping
            // track of cluster sizes and types as we go.
            //
            // POS will be the index of the descriptor of the current
            // cluster.
            //
            POS = LINDEX;

            while (POS > BEGDSC) {
                if (POS < LINDEX) {
                    //
                    // We'll need to determine the type of the current
                    // cluster.  If the next descriptor contains a positive
                    // value, the data type of the cluster it refers to is
                    // the successor of the current type, according to our
                    // ordering of types.
                    //
                    if (DIRREC[(POS + 1)] > 0) {
                        //
                        // This assignment and the one below in the ELSE
                        // block are performed from the second loop iteration
                        // onward. NXTTYP is initialized on the first loop
                        // iteration.
                        //
                        TYPE = save.PREV[NXTTYP];
                    } else {
                        TYPE = save.NEXT[NXTTYP];
                    }

                    //
                    // Update the cluster base record number.
                    //
                    BASE = (BASE - i32::abs(DIRREC[POS]));
                }

                //
                // Move the current cluster.
                //
                for I in intrinsics::range(((BASE + i32::abs(DIRREC[POS])) - 1), BASE, -1) {
                    if (TYPE == CHAR) {
                        DASIOC(b"READ", UNIT, I, &mut RECC, ctx)?;
                        DASIOC(b"WRITE", UNIT, (I + N), &mut RECC, ctx)?;
                    } else if (TYPE == DP) {
                        DASIOD(b"READ", UNIT, I, RECD.as_slice_mut(), ctx)?;
                        DASIOD(b"WRITE", UNIT, (I + N), RECD.as_slice_mut(), ctx)?;
                    } else {
                        DASIOI(b"READ", UNIT, I, RECI.as_slice_mut(), ctx)?;
                        DASIOI(b"WRITE", UNIT, (I + N), RECI.as_slice_mut(), ctx)?;
                    }
                }

                //
                // The next descriptor to look at is the preceding one in
                // the directory.
                //
                POS = (POS - 1);
                NXTTYP = TYPE;
            }

            //
            // Find the preceding directory record.
            //
            PREC = DIRREC[BWDLOC];

            //
            // Update the backward and forward pointers in the current
            // directory record.  However, don't modify null pointers.
            //
            if (DIRREC[FWDLOC] > 0) {
                DIRREC[FWDLOC] = (DIRREC[FWDLOC] + N);
            }

            if (DIRREC[BWDLOC] > 0) {
                DIRREC[BWDLOC] = (DIRREC[BWDLOC] + N);
            }

            //
            // Move the current directory record.
            //
            DASIOI(b"WRITE", UNIT, (NREC + N), DIRREC.as_slice_mut(), ctx)?;

            //
            // Consider the previous directory.
            //
            NREC = PREC;
        }
    }

    //
    // Update the file summary.  The number of comment records and the
    // number of the first free record have been incremented by N.
    // The numbers of the records containing the last descriptor of each
    // type have been incremented by N only if they were non-zero.
    //
    // The call to DASUFS will update the file record as well as the
    // file summary.
    //
    NCOMR = (NCOMR + N);
    FREE = (FREE + N);

    for I in 1..=3 {
        if (LASTRC[I] != 0) {
            LASTRC[I] = (LASTRC[I] + N);
        }
    }

    DASUFS(
        HANDLE,
        NRESVR,
        NRESVC,
        NCOMR,
        NCOMC,
        FREE,
        LASTLA.as_slice(),
        LASTRC.as_slice(),
        LASTWD.as_slice(),
        ctx,
    )?;

    CHKOUT(b"DASACR", ctx)?;
    Ok(())
}
