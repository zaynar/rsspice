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
const FWDLOC: i32 = 1;
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

/// DAS, remove comment records
///
/// Decrease the size of the comment area in a DAS file to reclaim
/// space freed by the removal of a specified number of comment
/// records.
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
///  N          I   Number of comment records to remove.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an existing DAS file opened for
///           comment area modification by DASOPC.
///
///  N        is the number of records to remove from the end of
///           the comment area. of the specified file. If NCOMR
///           is the number of comment records present in the
///           file on input, then on output the number of comment
///           records will be MAX ( 0,  NCOMR - N ).
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
///  2)  If an I/O error occurs during the removal process, the error
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
///  This routine is used to reclaim freed space in the comment area
///  of a DAS file subsequent to removal of comments from the file.
///  Any existing directory records and data records will be shifted
///  up by N records.
///
///  This routine updates the file record of the specified DAS file
///  to reflect the addition of records to the file's comment area.
///  Also, the file summary obtainable from DASHFS will be updated to
///  reflect the addition of comment records.
///
///  The disk space occupied by the specified DAS file will not
///  decrease as a result of calling this routine, but the number of
///  records occupied by meaningful data will decrease. The useful
///  records in the file can be copied by DAS routines to create a
///  new, smaller file which contains only the meaningful data.
///
///  This routine may be used only on existing DAS files opened by
///  DASOPC.
///
///  The association of DAS logical addresses and data within the
///  specified file will remain unaffected by use of this routine.
///
///  Normally, SPICELIB applications will not call this routine
///  directly, but will remove comments by calling DASRC.
///
///  This routine has an inverse DASACR, which appends a specified
///  number of records to the end of the comment area.
/// ```
///
/// # Examples
///
/// ```text
///  C
///  C     Open an existing DAS file for modification of
///  C     the comment area. We'll presume that the file
///  C     contains 20 comment records.
///  C
///        CALL DASOPC ( DAS, HANDLE )
///
///  C
///  C     Remove the last 10 comment records from the file.
///  C
///        CALL DASRCR ( HANDLE, 10  )
///
///  C
///  C     Close the file.
///  C
///        CALL DASCLS ( HANDLE )
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
/// -    SPICELIB Version 1.3.0, 02-JUN-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 15-NOV-1992 (NJB) (WLT)
/// ```
pub fn dasrcr(ctx: &mut SpiceContext, handle: i32, n: i32) -> crate::Result<()> {
    DASRCR(handle, n, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASRCR ( DAS, remove comment records )
pub fn DASRCR(HANDLE: i32, N: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    let mut NSHIFT: i32 = 0;
    let mut POS: i32 = 0;
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
    // Directory pointer location (forward):
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

    CHKIN(b"DASRCR", ctx)?;

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
        CHKOUT(b"DASRCR", ctx)?;
        return Ok(());
    }

    //
    // It's a mistake to use a negative value of N.
    //
    if (N < 0) {
        SETMSG(b"Number of comment records to remove must be non-negative.  Actual number requested was #.", ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(DASINVALIDCOUNT)", ctx)?;
        CHKOUT(b"DASRCR", ctx)?;
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
    // reserved records and the number of the first free record.
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
    // Determine the size of the record shift we'll actually perform.
    //
    NSHIFT = intrinsics::MIN0(&[N, NCOMR]);

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
    // LREC and LWORD are now the record and word index of the last
    // descriptor in the file. If LREC is zero, there are no directories
    // in the file yet. However, even DAS files that don't contain any
    // data have their first directory records zeroed out, and this
    // should remain true after the removal of the comment records.
    //
    if (LREC == 0) {
        //
        // Just write the zero-filled record to record number
        //
        //    NRESVR + NCOMR + 2 - NSHIFT
        //
        CLEARI(NWI, DIRREC.as_slice_mut());
        DASIOI(
            b"WRITE",
            UNIT,
            (((NRESVR + NCOMR) + 2) - NSHIFT),
            DIRREC.as_slice_mut(),
            ctx,
        )?;
    } else {
        //
        // There really is stuff to move.  For each directory record,
        // move the record and then all of the records described by that
        // record.  We start at the beginning of the data area and move
        // downwards in the file as we go.
        //
        NREC = ((NRESVR + NCOMR) + 2);

        while ((NREC <= LREC) && (NREC != 0)) {
            //
            // Read the current directory record and move it.
            //
            DASIOI(b"READ", UNIT, NREC, DIRREC.as_slice_mut(), ctx)?;
            DASIOI(b"WRITE", UNIT, (NREC - NSHIFT), DIRREC.as_slice_mut(), ctx)?;

            //
            // For each descriptor in the current directory, move the
            // cluster of data records it refers to.
            //
            // Find the data type, size, and base record number of the
            // first cluster described by the current directory.  Also
            // find the index within the directory of the directory's
            // last descriptor.
            //
            TYPE = DIRREC[BEGDSC];
            BASE = (NREC + 1);

            if (NREC == LREC) {
                LINDEX = LWORD;
            } else {
                LINDEX = NWI;
            }

            //
            // We'll now traverse the directory in forward order, keeping
            // track of cluster sizes and types as we go.
            //
            // POS will be the index of the descriptor of the current
            // cluster.
            //
            POS = (BEGDSC + 1);

            while (POS <= LINDEX) {
                if (POS > (BEGDSC + 1)) {
                    //
                    // We'll need to determine the type of the current
                    // cluster.  If the descriptor contains a positive
                    // value, the data type of the cluster it refers to is
                    // the successor of the previous type, according to our
                    // ordering of types.
                    //
                    if (DIRREC[POS] > 0) {
                        TYPE = save.NEXT[TYPE];
                    } else {
                        TYPE = save.PREV[TYPE];
                    }

                    //
                    // Update the cluster base record number.
                    //
                    BASE = (BASE + i32::abs(DIRREC[(POS - 1)]));
                }

                //
                // BASE and TYPE now are correctly set for the current
                // cluster.  Move the cluster.
                //
                for I in BASE..=((BASE + i32::abs(DIRREC[POS])) - 1) {
                    if (TYPE == CHAR) {
                        DASIOC(b"READ", UNIT, I, &mut RECC, ctx)?;
                        DASIOC(b"WRITE", UNIT, (I - NSHIFT), &mut RECC, ctx)?;
                    } else if (TYPE == DP) {
                        DASIOD(b"READ", UNIT, I, RECD.as_slice_mut(), ctx)?;
                        DASIOD(b"WRITE", UNIT, (I - NSHIFT), RECD.as_slice_mut(), ctx)?;
                    } else {
                        DASIOI(b"READ", UNIT, I, RECI.as_slice_mut(), ctx)?;
                        DASIOI(b"WRITE", UNIT, (I - NSHIFT), RECI.as_slice_mut(), ctx)?;
                    }
                }

                //
                // The next descriptor to look at is the next one in the
                // current directory.
                //
                POS = (POS + 1);
            }

            //
            // Find the next directory record.
            //
            NREC = DIRREC[FWDLOC];
        }
    }

    //
    // Update the file summary.  The number of comment records and the
    // number of the first free record have been decremented by NSHIFT.
    // The numbers of the records containing the last descriptor of each
    // type have been decremented by NSHIFT only if they were non-zero.
    //
    //
    // The call to DASUFS will update the file record as well as the
    // file summary.
    //
    NCOMR = (NCOMR - NSHIFT);
    FREE = (FREE - NSHIFT);

    for I in 1..=3 {
        if (LASTRC[I] != 0) {
            LASTRC[I] = (LASTRC[I] - NSHIFT);
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

    CHKOUT(b"DASRCR", ctx)?;
    Ok(())
}
