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
const INT: i32 = 3;
const DIR: i32 = 4;
const BWDLOC: i32 = 1;
const FWDLOC: i32 = 2;
const RNGBAS: i32 = 2;
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

/// DAS, segregate data records
///
/// Segregate the data records in a DAS file into clusters, using
/// one cluster per data type present in the file.
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
///  HANDLE     I   DAS file handle.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is a file handle of a DAS file opened for writing.
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
///  1)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If a Fortran read attempted by this routine fails, an error is
///      signaled by a routine in the call tree of this routine. The
///      state of the DAS file undergoing re-ordering will be
///      indeterminate.
///
///  3)  If a Fortran write attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      state of the DAS file undergoing re-ordering will be
///      indeterminate.
///
///  4)  If any other I/O error occurs during the re-arrangement of the
///      records in the indicated DAS file, the error is signaled by a
///      routine in the call tree of this routine.
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
///  Normally, there should be no need for routines outside of
///  SPICELIB to call this routine.
///
///  The effect of this routine is to re-arrange the data records
///  in a DAS file so that the file contains a single cluster for
///  each data type present in the file: in the general case, there
///  will be a single cluster of each of the integer, double
///  precision, and character data types.
///
///  The relative order of data records of a given type is not
///  affected by this re-ordering. After the re-ordering, the DAS
///  file contains a single directory record that has one descriptor
///  for each cluster. After that point, the order in the file of the
///  sets of data records of the various data types will be:
///
///     +-------+
///     |  CHAR |
///     +-------+
///     |  DP   |
///     +-------+
///     |  INT  |
///     +-------+
///
///  Files that contain multiple directory records will have all but
///  the first directory record moved to the end of the file when the
///  re-ordering is complete. These records are not visible to the
///  DAS system and will be overwritten if data is subsequently added
///  to the DAS file.
///
///  The purpose of segregating a DAS file's data records into three
///  clusters is to make read access more efficient: when a DAS file
///  contains a single directory with at most three cluster type
///  descriptors, mapping logical to physical addresses can be done
///  in constant time.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Segregate data records in a DAS file designated by
///      HANDLE:
///
///         CALL DASSDR ( HANDLE )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.1, 19-DEC-1995 (NJB)
///
///         Corrected title of permuted index entry section.
///
/// -    SPICELIB Version 2.0.0, 17-NOV-1993 (KRG)
///
///         Added test of FAILED after each DAS call, or sequence of calls,
///         which returns immediately if FAILED is .TRUE. This fixes a bug
///         where DASOPS signals an error and then DASSDR has a
///         segmentation fault.
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.2.0, 07-OCT-1993 (NJB) (HAN) (MJS)
///
///         Bug fix: call to CLEARD replaced with call to
///         CLEARI.
///
/// -    SPICELIB Version 1.1.0, 08-JUL-1993 (NJB) (MJS)
///
///         Bug fix: extraneous commas removed from argument lists
///         in calls to DASADI.
///
/// -    SPICELIB Version 1.0.0, 11-NOV-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 17-NOV-1993 (KRG)
///
///         Added test of failed after each DAS call, or sequence of calls,
///         which returns immediately if FAILED is .TRUE. This fixes a bug
///         where DASOPS signals an error and then DASSDR has a
///         segmentation fault.
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.2.0, 07-OCT-1993 (NJB) (HAN) (MJS)
///
///         Bug fix: call to CLEARD replaced with call to
///         CLEARI.
///
/// -    SPICELIB Version 1.1.0, 08-JUL-1993 (NJB)
///
///         Bug fix: extraneous commas removed from argument lists
///         in calls to DASADI. This bug had no visible effect on
///         VAX and Sun systems, but generated a compile error under
///         Lahey Fortran.
/// ```
pub fn dassdr(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DASSDR(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASSDR ( DAS, segregate data records )
pub fn DASSDR(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CREC = [b' '; NWC as usize];
    let mut SAVEC = [b' '; NWC as usize];
    let mut DREC = StackArray::<f64, 128>::new(1..=NWD);
    let mut SAVED = StackArray::<f64, 128>::new(1..=NWD);
    let mut BASE: i32 = 0;
    let mut COUNT = StackArray::<i32, 4>::new(1..=4);
    let mut DEST: i32 = 0;
    let mut DRBASE: i32 = 0;
    let mut FREE: i32 = 0;
    let mut I: i32 = 0;
    let mut IREC = StackArray::<i32, 256>::new(1..=NWI);
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut LOC: i32 = 0;
    let mut LREC: i32 = 0;
    let mut LTYPE: i32 = 0;
    let mut LWORD: i32 = 0;
    let mut MAXADR: i32 = 0;
    let mut MINADR: i32 = 0;
    let mut N: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut POS: i32 = 0;
    let mut PRVTYP: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut SAVEI = StackArray::<i32, 256>::new(1..=NWI);
    let mut SAVTYP: i32 = 0;
    let mut SCRHAN: i32 = 0;
    let mut START: i32 = 0;
    let mut TOTAL: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut MORE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Data type parameters
    //

    //
    // Directory pointer locations (backward and forward):
    //

    //
    // Directory address range location base
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
    } else {
        CHKIN(b"DASSDR", ctx)?;
    }

    //
    // Before starting, make sure that this DAS file is open for
    // writing.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    //
    // Get the logical unit for this file.
    //
    DASHLU(HANDLE, &mut UNIT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASSDR", ctx)?;
        return Ok(());
    }

    //
    // Write out any buffered records that belong to the file.
    //
    DASWBR(HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASSDR", ctx)?;
        return Ok(());
    }

    //
    // We're going to re-order the physical records in the DAS file,
    // starting with the first record after the first directory.
    // The other directory records are moved to the end of the file
    // as a result of the re-ordering.
    //
    // The re-ordering algorithm is based on that used in the REORDx
    // routines.  To use this algorithm, we'll build an order vector
    // for the records to be ordered; we'll construct this order vector
    // in a scratch DAS file.  First, we'll traverse the directories
    // to build up a sort of inverse order vector that tells us the
    // final destination and data type of each data record;  from this
    // inverse vector we can easily build a true order vector.  The
    // cycles of the true order vector can be traversed without
    // repetitive searching, and with a minimum of assignment of the
    // contents of data records to temporary variables.
    //

    //
    // Allocate a scratch DAS file to keep our vectors in.
    //
    DASOPS(&mut SCRHAN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASSDR", ctx)?;
        return Ok(());
    }

    //
    // Now build up our `inverse order vector'.   This array is an
    // inverse order vector only in loose sense:  it actually consists
    // of an integer array that contains a sequence of pairs of integers,
    // the first of which indicates a data type, and the second of which
    // is an ordinal number.  There is one pair for each data record in
    // the file.  The ordinal number gives the ordinal position of the
    // record described by the number pair, relative to the other records
    // of the same type.  Directory records are considered to have type
    // `directory', which is represented by the code DIR.
    //
    // We also must maintain a count of records of each type.
    //
    CLEARI(4, COUNT.as_slice_mut());

    //
    // Get the file summary for the DAS file to be segregated.
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

    if FAILED(ctx) {
        CHKOUT(b"DASSDR", ctx)?;
        return Ok(());
    }
    //
    // Find the record and word positions LREC and LWORD of the last
    // descriptor in the file, and also find the type of the descriptor
    // LTYPE.
    //
    MAXAI(LASTRC.as_slice(), 3, &mut LREC, &mut LOC);
    LWORD = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if ((LASTRC[I] == LREC) && (LASTWD[I] > LWORD)) {
                LWORD = LASTWD[I];
                LTYPE = I;
            }

            I += m3__;
        }
    }

    //
    // The first directory starts after the last comment record.
    //
    RECNO = ((NRESVR + NCOMR) + 2);

    while ((RECNO <= LREC) && (RECNO > 0)) {
        //
        // Read the directory record.
        //
        DASRRI(HANDLE, RECNO, 1, NWI, IREC.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASSDR", ctx)?;
            return Ok(());
        }
        //
        // Increment the directory count.
        //
        COUNT[DIR] = (COUNT[DIR] + 1);

        //
        // Add the data type (`directory') and count (1) of the current
        // record to the inverse order vector.
        //
        DASADI(SCRHAN, 1, &[DIR], ctx)?;
        DASADI(SCRHAN, 1, COUNT.subarray(DIR), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASSDR", ctx)?;
            return Ok(());
        }
        //
        // Set up our `finite state machine' that tells us the data
        // types of the records described by the last read directory.
        //
        TYPE = IREC[BEGDSC];
        PRVTYP = save.PREV[TYPE];

        //
        // Now traverse the directory and update the inverse order
        // vector based on the descriptors we find.
        //
        MORE = true;
        I = (BEGDSC + 1);

        while MORE {
            //
            // Obtain the count for the current descriptor.
            //
            N = i32::abs(IREC[I]);

            //
            // Update our inverse order vector to describe the positions
            // of the N records described by the current descriptor.
            //
            for J in 1..=N {
                DASADI(SCRHAN, 1, &[TYPE], ctx)?;
                DASADI(SCRHAN, 1, &[(COUNT[TYPE] + J)], ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"DASSDR", ctx)?;
                    return Ok(());
                }
            }

            //
            // Adjust the count of records of data type TYPE.
            //
            COUNT[TYPE] = (COUNT[TYPE] + N);

            //
            // Find the next type.
            //
            I = (I + 1);

            if ((I > NWI) || ((RECNO == LREC) && (I > LWORD))) {
                MORE = false;
            } else {
                if (IREC[I] > 0) {
                    TYPE = save.NEXT[TYPE];
                } else if (IREC[I] < 0) {
                    TYPE = save.PREV[TYPE];
                } else {
                    MORE = false;
                }
            }
        }

        //
        // The forward pointer in this directory tells us where the
        // next directory record is.  When there are no more directory
        // records, this pointer will be zero.
        //
        RECNO = IREC[FWDLOC];
    }

    //
    // At this point, the inverse order vector is set up.  The array
    // COUNT contains counts of the number of records of each type we've
    // seen.  Set TOTAL to the total number of records that we've going
    // to permute.
    //
    TOTAL = SUMAI(COUNT.as_slice(), 4);

    //
    // The next step is to build a true order vector.  Let BASE be
    // the base address for the order vector; this address is the
    // last logical address of the inverse order vector.
    //
    BASE = (2 * TOTAL);

    //
    // We'll store the actual order vector in locations BASE + 1
    // through BASE + TOTAL.  In addition, we'll build a parallel array
    // that contains, for each element of the order vector, the type of
    // data corresponding to that element.  This type vector will
    // reside in locations BASE + TOTAL + 1 through BASE + 2*TOTAL.
    //
    // Before setting the values of the order vector and its parallel
    // type vector, we'll allocate space in the scratch DAS file by
    // zeroing out the locations we plan to use.  After this, locations
    // BASE+1 through BASE + 2*TOTAL can be written to in random access
    // fashion using DASUDI.
    //
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = (2 * TOTAL);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DASADI(SCRHAN, 1, &[0], ctx)?;

            I += m3__;
        }
    }

    if FAILED(ctx) {
        CHKOUT(b"DASSDR", ctx)?;
        return Ok(());
    }
    //
    // We note that the way to construct the inverse of a permutation
    // SIGMA in a single loop is suggested by the relation
    //
    //         -1
    //    SIGMA   (  SIGMA(I)  )   =   I
    //
    // We'll use this method.  In our case, our order vector plays
    // the role of
    //
    //         -1
    //    SIGMA
    //
    // and the `inverse order vector' plays the role of SIGMA.  We'll
    // exclude the first directory from the order vector, since it's
    // an exception:  we wish to reserve this record.  Since the first
    // element of the order vector (logically) contains the index 1, we
    // can ignore it.
    //
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = TOTAL;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            DASRDI(
                SCRHAN,
                ((2 * I) - 1),
                ((2 * I) - 1),
                std::slice::from_mut(&mut TYPE),
                ctx,
            )?;
            DASRDI(
                SCRHAN,
                (2 * I),
                (2 * I),
                std::slice::from_mut(&mut DEST),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASSDR", ctx)?;
                return Ok(());
            }
            //
            // Set DEST to the destination location, measured as an offset
            // from the last comment record, of the Ith record by adding
            // on the count of the predecessors of the block of records of
            // TYPE.
            //
            for J in 1..=3 {
                if (TYPE > J) {
                    DEST = (DEST + COUNT[J]);
                }
            }

            //
            // The destination offset of each record should be incremented to
            // allow room for the first directory record.  However, we don't
            // need to do this for directory records; they'll already have
            // this offset accounted for.
            //
            if (TYPE != DIR) {
                DEST = (DEST + 1);
            }

            //
            // The value of element DEST of the order vector is I.
            // Write this value to location BASE + DEST.
            //
            DASUDI(SCRHAN, (BASE + DEST), (BASE + DEST), &[I], ctx)?;
            //
            // We want the ith element of the order vector to give us the
            // number of the record to move to position i (offset from the
            // last comment record),  but we want the corresponding element
            // of the type array to give us the type of the record currently
            // occupying position i.
            //
            DASUDI(
                SCRHAN,
                ((BASE + I) + TOTAL),
                ((BASE + I) + TOTAL),
                &[TYPE],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASSDR", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Ok, here's what we've got in the scratch file that's still of
    // interest:
    //
    //    -- In integer logical addresses BASE + 1 : BASE + TOTAL,
    //       we have an order vector.  The Ith element of this
    //       vector indicates the record that should be moved to
    //       location DRBASE + I in the DAS file we're re-ordering,
    //       where DRBASE is the base address of the data records
    //       (the first directory record follows the record having this
    //       index).
    //
    //
    //    -- In integer logical addresses BASE + TOTAL + 1  :  BASE +
    //       2*TOTAL, we have data type indicators for the records to
    //       be re-ordered.  The type for the Ith record in the file,
    //       counted from the last comment record, is located in logical
    //       address BASE + TOTAL + I.
    //
    //
    DRBASE = ((NRESVR + NCOMR) + 1);

    //
    // As we traverse the order vector, we flip the sign of elements
    // we've accessed, so that we can tell when we encounter an element
    // of a cycle that we've already traversed.
    //
    // Traverse the order vector.  The variable START indicates the
    // first element to look at.  Ignore the first element; it's a
    // singleton cycle.
    //
    //
    START = 2;

    while (START < TOTAL) {
        //
        // Traverse the current cycle of the order vector.
        //
        // We `make a hole' in the file by saving the record in position
        // START, then we traverse the cycle in reverse order, filling in
        // the hole at the ith position with the record whose number is
        // the ith element of the order vector.  At the end, we deposit
        // the saved record into the `hole' left behind by the last
        // record we moved.
        //
        // We're going to read and write records to and from the DAS file
        // directly, rather than going through the buffering system.
        // This will allow us to avoid any untoward interactions between
        // the buffers for different data types.
        //
        DASRDI(
            SCRHAN,
            ((BASE + TOTAL) + START),
            ((BASE + TOTAL) + START),
            std::slice::from_mut(&mut SAVTYP),
            ctx,
        )?;

        DASRDI(
            SCRHAN,
            (BASE + START),
            (BASE + START),
            std::slice::from_mut(&mut OFFSET),
            ctx,
        )?;

        //
        // Save the record at the location DRBASE + START.
        //
        if (SAVTYP == CHAR) {
            DASIOC(b"READ", UNIT, (DRBASE + START), &mut SAVEC, ctx)?;
        } else if (SAVTYP == DP) {
            DASIOD(b"READ", UNIT, (DRBASE + START), SAVED.as_slice_mut(), ctx)?;
        } else {
            DASIOI(b"READ", UNIT, (DRBASE + START), SAVEI.as_slice_mut(), ctx)?;
        }

        if FAILED(ctx) {
            CHKOUT(b"DASSDR", ctx)?;
            return Ok(());
        }
        //
        // Let I be the index of the record that we are going to move
        // data into next.  I is an offset from the last comment record.
        //
        I = START;

        while (OFFSET != START) {
            //
            // Mark the order vector element by writing its negative
            // back to the location it came from.
            //
            DASUDI(SCRHAN, (BASE + I), (BASE + I), &[-OFFSET], ctx)?;
            //
            // Move the record at location
            //
            //    DRBASE + OFFSET
            //
            // to location
            //
            //    DRBASE + I
            //
            // There is no need to do anything about the corresponding
            // elements of the type vector; we won't need them again.
            //
            // The read and write operations, as well as the temporary
            // record required to perform the move, are dependent on the
            // data type of the record to be moved.
            //
            DASRDI(
                SCRHAN,
                ((BASE + TOTAL) + OFFSET),
                ((BASE + TOTAL) + OFFSET),
                std::slice::from_mut(&mut TYPE),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASSDR", ctx)?;
                return Ok(());
            }
            //
            // Only pick records up if we're going to put them down in
            // a location other than their original one.
            //
            if (I != OFFSET) {
                if (TYPE == CHAR) {
                    DASIOC(b"READ", UNIT, (DRBASE + OFFSET), &mut CREC, ctx)?;
                    DASIOC(b"WRITE", UNIT, (DRBASE + I), &mut CREC, ctx)?;
                } else if (TYPE == DP) {
                    DASIOD(b"READ", UNIT, (DRBASE + OFFSET), DREC.as_slice_mut(), ctx)?;
                    DASIOD(b"WRITE", UNIT, (DRBASE + I), DREC.as_slice_mut(), ctx)?;
                } else {
                    DASIOI(b"READ", UNIT, (DRBASE + OFFSET), IREC.as_slice_mut(), ctx)?;
                    DASIOI(b"WRITE", UNIT, (DRBASE + I), IREC.as_slice_mut(), ctx)?;
                }

                if FAILED(ctx) {
                    CHKOUT(b"DASSDR", ctx)?;
                    return Ok(());
                }
            }

            //
            // OFFSET is the index of the next order vector element to
            // look at.
            //
            I = OFFSET;

            DASRDI(
                SCRHAN,
                (BASE + I),
                (BASE + I),
                std::slice::from_mut(&mut OFFSET),
                ctx,
            )?;
            DASRDI(
                SCRHAN,
                ((BASE + I) + TOTAL),
                ((BASE + I) + TOTAL),
                std::slice::from_mut(&mut TYPE),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASSDR", ctx)?;
                return Ok(());
            }
        }

        //
        // The last value of I is the location in the cycle that element
        // START followed.  Therefore, the saved record corresponding
        // to index START should be written to this location.
        //
        if (SAVTYP == CHAR) {
            DASIOC(b"WRITE", UNIT, (DRBASE + I), &mut SAVEC, ctx)?;
        } else if (SAVTYP == DP) {
            DASIOD(b"WRITE", UNIT, (DRBASE + I), SAVED.as_slice_mut(), ctx)?;
        } else {
            DASIOI(b"WRITE", UNIT, (DRBASE + I), SAVEI.as_slice_mut(), ctx)?;
        }
        //
        // Mark the order vector element by writing its negative
        // back to the location it came from.
        //
        DASUDI(SCRHAN, (BASE + I), (BASE + I), &[-START], ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DASSDR", ctx)?;
            return Ok(());
        }
        //
        // Update START so that it points to the first element of a cycle
        // of the order vector that has not yet been traversed.  This will
        // be the first positive element of the order vector in a location
        // indexed higher than the current value of START.  Note that
        // this way of updating START guarantees that we don't have to
        // backtrack to find an element in the next cycle.
        //

        OFFSET = -1;

        while ((OFFSET < 0) && (START < TOTAL)) {
            START = (START + 1);

            DASRDI(
                SCRHAN,
                (BASE + START),
                (BASE + START),
                std::slice::from_mut(&mut OFFSET),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASSDR", ctx)?;
                return Ok(());
            }
        }

        //
        // At this point, START is the index of an element in the order
        // vector that belongs to a cycle where no routine has gone
        // before, or else START is the last index in the order vector,
        // in which case we're done.
        //
    }

    //
    // At this point, the records in the DAS are organized as follows:
    //
    //    +----------------------------------+
    //    |           File record            |  ( 1 )
    //    +----------------------------------+
    //    |         Reserved records         |  ( 0 or more )
    //    |                                  |
    //    +----------------------------------+
    //    |          Comment records         |  ( 0 or more )
    //    |                                  |
    //    |                                  |
    //    +----------------------------------+
    //    |      First directory  record     |  ( 1 )
    //    +----------------------------------+
    //    |      Character data records      |  ( 0 or more )
    //    |                                  |
    //    +----------------------------------+
    //    |   Double precision data records  |  ( 0 or more )
    //    |                                  |
    //    +----------------------------------+
    //    |       Integer data records       |  ( 0 or more )
    //    |                                  |
    //    +----------------------------------+
    //    |   Additional directory records   |  ( 0 or more )
    //    |                                  |
    //    +----------------------------------+
    //
    //
    // Not all of the indicated components must be present; only the
    // file record and first directory record will exist in all cases.
    // The `additional directory records' at the end of the file serve
    // no purpose; if more data is appended to the file, they will be
    // overwritten.
    //
    // The last step in preparing the file is to fill in the first
    // directory record with the correct information, and to update
    // the file summary.
    //
    //
    RECNO = (DRBASE + 1);

    CLEARI(NWI, IREC.as_slice_mut());

    //
    // Set the logical address ranges in the directory record, for each
    // data type.
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        TYPE = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            MAXADR = LASTLA[TYPE];

            if (MAXADR > 0) {
                MINADR = 1;
            } else {
                MINADR = 0;
            }

            IREC[((RNGBAS + (2 * TYPE)) - 1)] = MINADR;
            IREC[(RNGBAS + (2 * TYPE))] = MAXADR;

            TYPE += m3__;
        }
    }

    //
    // Set the descriptors in the directory.  Determine which type
    // comes first:  the order of priority is character, double
    // precision, integer.
    //
    POS = BEGDSC;

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        TYPE = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (LASTLA[TYPE] > 0) {
                if (POS == BEGDSC) {
                    //
                    // This is the first type for which any data is present.
                    // We must enter a type code at position BEGDSC in the
                    // directory, and we must enter a count at position
                    // BEGDSC+1.
                    //
                    IREC[BEGDSC] = TYPE;
                    IREC[(BEGDSC + 1)] = COUNT[TYPE];
                    LASTRC[TYPE] = RECNO;
                    LASTWD[TYPE] = (BEGDSC + 1);
                    POS = (POS + 2);
                    PRVTYP = TYPE;
                } else {
                    //
                    // Place an appropriately signed count at location POS in
                    // the directory.
                    //
                    if (TYPE == save.NEXT[PRVTYP]) {
                        IREC[POS] = COUNT[TYPE];
                    } else {
                        IREC[POS] = -COUNT[TYPE];
                    }

                    LASTRC[TYPE] = RECNO;
                    LASTWD[TYPE] = POS;
                    POS = (POS + 1);
                    PRVTYP = TYPE;
                }
            }

            TYPE += m3__;
        }
    }

    //
    // Since we've done away with all but the first directory, the first
    // free record is decremented by 1 less than the directory count.
    //
    FREE = ((FREE - COUNT[DIR]) + 1);

    //
    // Write out the new directory record.  Don't use the DAS buffered
    // write mechanism; this could trash the file by dumping buffered
    // records in the wrong places.
    //
    DASIOI(b"WRITE", UNIT, RECNO, IREC.as_slice_mut(), ctx)?;
    //
    // Write out the updated file summary.
    //
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
    //
    // Clean up the DAS data buffers:  we don't want buffered scratch
    // file records hanging around there.  Then get rid of the scratch
    // file.
    //
    DASWBR(SCRHAN, ctx)?;
    DASLLC(SCRHAN, ctx)?;

    CHKOUT(b"DASSDR", ctx)?;
    Ok(())
}
