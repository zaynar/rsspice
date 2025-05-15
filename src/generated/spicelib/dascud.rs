//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const CHAR: i32 = 1;
pub const DP: i32 = 2;
pub const INT: i32 = 3;
const NWC: i32 = 1024;
const NWD: i32 = 128;
const NWI: i32 = 256;
const BWDLOC: i32 = 1;
const FWDLOC: i32 = 2;
const CHRRNG: i32 = 3;
const DPRNG: i32 = (CHRRNG + 2);
const INTRNG: i32 = (DPRNG + 2);
const BEGDSC: i32 = 9;

struct SaveVars {
    NEXT: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NEXT = StackArray::<i32, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(2), Val::I(3), Val::I(1)].into_iter();
            NEXT.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { NEXT }
    }
}

/// DAS, create or update directories
///
/// Create or update directories in a DAS file to reflect addition
/// of a specified number of words of a specified data type.
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
///  TYPE       I   Data type specifier.
///  NWORDS     I   Number of words of data being added.
///  CHAR       P   Parameter indicating character data type.
///  DP         P   Parameter indicating double precision data type.
///  INT        P   Parameter indicating integer data type.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a DAS file open for writing.
///
///  TYPE     is a data type specifier. TYPE may be any of
///           the parameters
///
///              CHAR
///              DP
///              INT
///
///           which indicate `character', `double precision',
///           and `integer' respectively.
///
///  NWORDS   is the number of words of data of the data type
///           indicated by TYPE whose addition to the indicated
///           DAS file is to be accounted for.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the action of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  CHAR,
///  DP,
///  INT      are data type specifiers which indicate
///           `character', `double precision', and `integer'
///           respectively. These parameters are used in
///           all DAS routines that require a data type
///           specifier as input.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input handle is invalid, an error is signaled by a
///      routine in the call tree of this routine.
///
///  2)  If TYPE is not recognized, the error SPICE(DASINVALIDTYPE)
///      is signaled.
///
///  3)  If NWORDS is negative, the error SPICE(VALUEOUTOFRANGE) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine operates by side effects: the directories in the
///  indicated DAS file will be updated to reflect the addition of
///  the indicated number of words of the specified data type.
///  If necessary, a new directory record will be added to the file
///  to hold a new cluster descriptor.
///
///  In addition, the file summary for the indicated DAS file will be
///  updated with the new values of the descriptor location and last
///  logical address of the indicated type, as well as with the new
///  value of the free record pointer.
///
///  This routine is used by the DASADx routines: after each data
///  addition, they call this routine to update the directories of the
///  affected DAS file.
///
///  Normally, there will be no need for routines outside of SPICELIB
///  to call this routine directly. To add data to or update a DAS
///  file, the DASADx and DASUDx routines should be used; these
///  routines take care of directory creation and updates.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Update directories after writing N integer words to a
///      DAS file designated by HANDLE:
///
///          CALL DASCUD ( HANDLE, INT, N )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine is intended for use by the SPICELIB DAS routines.
///      Non-SPICELIB software normally will not need to call this
///      routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.5.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.4.0, 07-AUG-2006 (NJB)
///
///         Bug fix: added initialization of variable LTYPE to support
///                  operation under the Macintosh Intel Fortran
///                  compiler. Note that this bug did not affect
///                  operation of this routine on other platforms.
///
/// -    SPICELIB Version 1.3.0, 16-JAN-2003 (NJB)
///
///         Bug fix: fixed previous bug fix.
///
/// -    SPICELIB Version 1.2.0, 10-DEC-2002 (NJB)
///
///         Bug fix: now a new, empty directory record with valid
///         backward and forward pointers is written immediately
///         when it is created.
///
/// -    SPICELIB Version 1.1.1, 19-DEC-1995 (NJB)
///
///         Corrected title of permuted index entry section.
///
/// -    SPICELIB Version 1.0.1, 26-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
///         Removed an unused variable.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.4.0 07-AUG-2006 (NJB)
///
///         Bug fix: added initialization of variable LTYPE to support
///                  operation under the Macintosh Intel Fortran
///                  compiler. Note that this bug did not affect
///                  operation of this routine on other platforms. The
///                  statement referencing the uninitialized variable
///                  was:
///
///            ELSE IF (       ( TYPE   .EQ. LTYPE )
///           .          .AND. ( DSCREC .GT. 0     )
///           .          .AND. ( LWORD  .LT. NWI   )  ) THEN
///
///
///         In the previous version of the code, LTYPE is uninitialized
///         when the DAS file is empty, which implies DSCREC is 0.
///         Otherwise LTYPE is initialized. So the value of the logical
///         expression is not affected by the uninitialized value of
///         LTYPE.
///
///         However, the Intel Fortran compiler for the Mac flags a runtime
///         error when the above code is exercised. So LTYPE is now
///         initialized to an invalid value prior to execution of this
///         code. If the invalid value is ever used, a runtime error
///         should result.
///
///
/// -    SPICELIB Version 1.3.0 16-JAN-2003 (NJB)
///
///         Bug fix: fixed previous bug fix.
///
///
///         The offending line (#778) in previous version) of code is:
///
///            CALL DASWRI ( HANDLE, RECNO, DIRREC )
///
///         The correct line of code is:
///
///           CALL DASWRI ( HANDLE, FREE, DIRREC )
///
///
/// -    SPICELIB Version 1.2.0 10-DEC-2002 (NJB)
///
///         Bug fix: now a new, empty directory record with valid
///         backward and forward pointers is written immediately
///         when it is created. This prevents an unsegregated file
///         from being left with an invalid forward pointer.
///
/// -    SPICELIB Version 1.0.1, 26-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
///         Removed an unused variable, PREV.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn dascud(ctx: &mut SpiceContext, handle: i32, type_: i32, nwords: i32) -> crate::Result<()> {
    DASCUD(handle, type_, nwords, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASCUD ( DAS, create or update directories )
pub fn DASCUD(HANDLE: i32, TYPE: i32, NWORDS: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DESCR: i32 = 0;
    let mut DIRREC = StackArray::<i32, 256>::new(1..=NWI);
    let mut DSCREC: i32 = 0;
    let mut FREE: i32 = 0;
    let mut LAST: i32 = 0;
    let mut LASTLA = StackArray::<i32, 3>::new(1..=3);
    let mut LASTRC = StackArray::<i32, 3>::new(1..=3);
    let mut LASTWD = StackArray::<i32, 3>::new(1..=3);
    let mut LREC: i32 = 0;
    let mut LOC: i32 = 0;
    let mut LTYPE: i32 = 0;
    let mut LWORD: i32 = 0;
    let mut MAXADR: i32 = 0;
    let mut MINADR: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NEEDED: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NW: i32 = 0;
    let mut RECNO: i32 = 0;
    let mut RNGLOC: i32 = 0;
    let mut ROOM: i32 = 0;

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
    // Directory pointer locations (backward and forward):
    //

    //
    // Directory address range locations
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

    //
    // NEXT maps the DAS data type codes to their successors.
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
        CHKIN(b"DASCUD", ctx)?;
    }

    //
    // Here's a preview of coming attractions:
    //
    //    We're going to update the directories in the indicated
    //    DAS file to reflect the addition of NWORDS new data words.
    //    This data is supposed to have been added to the file BEFORE
    //    this routine is called.  There are several possible states
    //    the file can be in at the point this routine is called.
    //
    //
    //       1)  There is already a descriptor of TYPE in the file, and
    //           the addition of data does not require this descriptor
    //           to be modified.
    //
    //           We can tell that we have this case when the file
    //           summary indicates that, before the addition of data,
    //           there was room for NWORDS of data in the last data
    //           record in the file.  Since no new data records were
    //           required to accommodate the new data, the descriptor
    //           for TYPE does not have to be updated.
    //
    //           However, even though the descriptor need not be
    //           modified, the address range for TYPE covered by the
    //           directory record containing this last descriptor must be
    //           updated, as must be the file summary.
    //
    //
    //       2)  There is already a descriptor of TYPE in the file, and
    //           in order to describe the new data added to the file,
    //           it suffices to update this descriptor and the address
    //           range in the directory containing it.
    //
    //           This happens when case (1) doesn't apply, and the
    //           descriptor of TYPE is the last descriptor in the last
    //           directory, and the descriptor is not in the last
    //           position (index NWI) of the directory.
    //
    //           Note that we never update the last descriptor in a
    //           directory record.  The reason for this is that after
    //           this descriptor is written, we build a new directory
    //           record.  All subsequent additions of data are made to
    //           records that follow this new directory record;
    //           otherwise, the new directory would get overwritten
    //           with data.
    //
    //
    //       3)  A new descriptor of TYPE is needed.
    //
    //           This can happen in several ways:
    //
    //           a)  There are no directories in the file yet, in which
    //               case space has been reserved for the first
    //               directory.
    //
    //               This can happen only when the file had no data at
    //               all in it before the last addition of data.
    //
    //               In this case, we must fill in the first descriptor
    //               and the address range for TYPE.  We must also update
    //               the file summary, because the descriptor location,
    //               last logical address of TYPE, and the free pointer
    //               have changed.
    //
    //           b)  The conditions for cases (1) and (2) are not
    //               satisfied, and the current last directory record
    //               has room for a new descriptor.  In this case, if
    //               the data addition filled in the last data record
    //               described by the current last descriptor of type,
    //               (which will usually be the case), we must update
    //               the appropriate address range in the directory
    //               record containing that descriptor.  We will then
    //               add a new descriptor to the last directory record
    //               and update the address range for TYPE in that
    //               record.  The file summary must be updated as well.
    //
    //               If the new descriptor we've added went into the
    //               last slot in a directory record (index NWI), we
    //               also create a new, empty directory record and
    //               update the forward pointer of the current directory
    //               to point to it.  We also update the file summary
    //               so that the free pointer points to the record
    //               following the empty directory record.
    //
    //
    //           c)  The conditions for cases (1) and (2) are not
    //               satisfied, and the current last directory record
    //               has no room for a new descriptor.
    //
    //               In this case, if the data addition filled in the
    //               last data record described by the current last
    //               descriptor of TYPE, (which will usually be the
    //               case), we must update the appropriate address range
    //               in the directory record containing that descriptor.
    //               We will then add a new descriptor to the empty
    //               directory record and initialize the address range
    //               for TYPE in that record.  The file summary must be
    //               updated as well.
    //
    //
    // To start out, we'll need to find out how the file is currently
    // disposed.  We'll need the location of the last descriptor of
    // TYPE, the last logical address of TYPE, and the location of
    // the last descriptor of any type.
    //
    // Get the file summary.
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
    // Now do all of the data-type-dependent work:
    //
    //    -- Set the last address of the indicated data type LAST.
    //
    //    -- Set the physical record of the last descriptor of TYPE.
    //
    //    -- Set the number of words of data of the specified type per
    //       physical record NW.
    //
    //    -- Set the address range location used to pick address ranges
    //       out of directory records.
    //
    //
    // Note that the address and descriptor location information from
    // the file summary is assumed NOT to take into account the latest
    // data addition.
    //
    //
    LAST = LASTLA[TYPE];
    DSCREC = LASTRC[TYPE];

    if (TYPE == DP) {
        NW = NWD;
        RNGLOC = DPRNG;
    } else if (TYPE == INT) {
        NW = NWI;
        RNGLOC = INTRNG;
    } else if (TYPE == CHAR) {
        NW = NWC;
        RNGLOC = CHRRNG;
    } else {
        SETMSG(b"Invalid data type: #. ", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(DASINVALIDTYPE)", ctx)?;
        CHKOUT(b"DASCUD", ctx)?;
        return Ok(());
    }

    //
    // Make sure that NWORDS is something sensible.
    //
    if (NWORDS < 0) {
        SETMSG(b"NWORDS was #; should be non-negative.", ctx);
        ERRINT(b"#", NWORDS, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASCUD", ctx)?;
        return Ok(());
    }

    //
    // Find the record and word positions LREC and LWORD of the last
    // descriptor in the file, and also find the type of the descriptor
    // LTYPE.
    //
    MAXAI(LASTRC.as_slice(), 3, &mut LREC, &mut LOC);
    LWORD = 0;
    LTYPE = 0;

    for I in 1..=3 {
        if ((LASTRC[I] == LREC) && (LASTWD[I] > LWORD)) {
            LWORD = LASTWD[I];
            LTYPE = I;
        }
    }

    //
    // LREC, LWORD, and LTYPE are now the record, word, and data type
    // of the last descriptor in the file.  If LREC is zero, there are
    // no directories in the file yet.  In this case, LWORD and
    // LTYPE are both zero.
    //

    //
    // Compute the number of words we have room for in the current
    // last data record of the indicated type.
    //
    if (LAST > 0) {
        ROOM = (NW - (LAST - (((LAST - 1) / NW) * NW)));
    } else {
        ROOM = 0;
    }

    //
    // Compute the number of additional data records needed to
    // accommodate (NWORDS - ROOM) additional words of data of type
    // TYPE.
    //
    NEEDED = ((((NWORDS - ROOM) + NW) - 1) / NW);

    //
    // Now, update the descriptor directories.
    //

    if ((ROOM >= NWORDS) && (DSCREC > 0)) {
        //
        // This is case (1).
        //
        // There is already a descriptor of TYPE in the file.  The data
        // fits in the current record, so no descriptors have to change.
        //
        // Update the address range in the directory record containing
        // the last descriptor of TYPE.
        //
        MAXADR = (LAST + NWORDS);

        DASURI(HANDLE, DSCREC, (RNGLOC + 1), (RNGLOC + 1), &[MAXADR], ctx)?;

        //
        // The last logical address of TYPE is now MAXADR.
        //
        LASTLA[TYPE] = MAXADR;

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
    } else if (((TYPE == LTYPE) && (DSCREC > 0)) && (LWORD < NWI)) {
        //
        //
        // This is case (2).
        //
        // The descriptor of TYPE is the last descriptor in the
        // file but is not in the last location (index NWI) of a
        // directory record.  All we have to do is update this last
        // descriptor to reflect the addition of the number of needed
        // data records.
        //
        // Get the old descriptor, since we're going to update it.
        //
        //
        DASRRI(
            HANDLE,
            DSCREC,
            LWORD,
            LWORD,
            std::slice::from_mut(&mut DESCR),
            ctx,
        )?;

        //
        // Update the descriptor and write it back into the file.
        //
        if (DESCR < 0) {
            DESCR = (DESCR - NEEDED);
        } else {
            DESCR = (DESCR + NEEDED);
        }

        DASURI(HANDLE, DSCREC, LWORD, LWORD, &[DESCR], ctx)?;

        //
        // Update the address range for this type.
        //
        MAXADR = (LAST + NWORDS);

        DASURI(HANDLE, DSCREC, (RNGLOC + 1), (RNGLOC + 1), &[MAXADR], ctx)?;

        //
        // The last logical address of TYPE is now MAXADR.  The first
        // free record follows the last data record in use.
        //
        LASTLA[TYPE] = MAXADR;
        FREE = (FREE + NEEDED);

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
    } else {
        //
        // This is case (3).  We need a new descriptor.
        //

        if (LREC == 0) {
            //
            // This is case (3a).  We have a virgin directory record.
            // Set the number of this record.
            //
            RECNO = ((NRESVR + NCOMR) + 2);

            //
            // Start with an empty directory record.
            //
            CLEARI(NWI, DIRREC.as_slice_mut());

            //
            // Add a new descriptor to the directory.  The record
            // count is the number of new records required:  NEEDED.
            //
            DIRREC[BEGDSC] = TYPE;
            DIRREC[(BEGDSC + 1)] = NEEDED;

            //
            // Fill in the address range for TYPE covered by this
            // directory.
            //
            DIRREC[RNGLOC] = 1;
            DIRREC[(RNGLOC + 1)] = NWORDS;

            //
            // Write out this directory.
            //
            DASWRI(HANDLE, RECNO, DIRREC.as_slice(), ctx)?;

            //
            // Update the file summary:  the location of the descriptor
            // and the last logical address for this type must be set.
            // The count portion of the descriptor goes after the initial
            // data type indicator; this data type indicator is not
            // considered to be part of the descriptor.
            //
            // The first free record follows the last data record in use.
            //
            FREE = ((RECNO + NEEDED) + 1);
            LASTLA[TYPE] = NWORDS;
            LASTRC[TYPE] = RECNO;
            LASTWD[TYPE] = (BEGDSC + 1);

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
        } else if (LWORD < NWI) {
            //
            // This is case (3b).  We have room for another descriptor
            // in the current directory record.
            //
            // Before adding the new descriptor, we must update the
            // directory containing the current last descriptor of TYPE,
            // if the range of addresses covered by the cluster it
            // describes was increased by the last data addition.  Of
            // course, this update is required only if there IS such a
            // descriptor, and if it is in a record that precedes LREC.
            //
            if (((DSCREC > 0) && (DSCREC < LREC)) && (ROOM > 0)) {
                //
                // Update the address range for TYPE in record DSCREC.
                // The upper bound is increased by ROOM, since that many
                // words of TYPE were added to the last record in the
                // last cluster of TYPE described by that directory.
                //
                MAXADR = (LAST + ROOM);

                DASURI(HANDLE, DSCREC, (RNGLOC + 1), (RNGLOC + 1), &[MAXADR], ctx)?;
            }

            //
            // Make up the new descriptor and write it to the last
            // directory, following the current last descriptor.  The
            // sign of the new descriptor is a function of the type of
            // the current last descriptor.
            //
            if (TYPE == save.NEXT[LTYPE]) {
                //
                // TYPE is the successor in the type sequence of the type
                // of the previous descriptor; use a positive count.
                //
                DESCR = NEEDED;
            } else {
                DESCR = -NEEDED;
            }

            DASURI(HANDLE, LREC, (LWORD + 1), (LWORD + 1), &[DESCR], ctx)?;

            //
            // Update the address range for this type.  Some care is needed
            // when updating the minimum address:  this value should be
            // assigned only if this is the first descriptor of TYPE in
            // this directory record.
            //
            if (DSCREC < LREC) {
                MINADR = ((LAST + ROOM) + 1);

                DASURI(HANDLE, LREC, RNGLOC, RNGLOC, &[MINADR], ctx)?;
            }

            MAXADR = (LAST + NWORDS);

            DASURI(HANDLE, LREC, (RNGLOC + 1), (RNGLOC + 1), &[MAXADR], ctx)?;

            //
            // Update the file summary:  the location of the descriptor
            // and the last logical address for this type must be set.
            //
            // The first free record follows the last data record in use.
            //
            FREE = (FREE + NEEDED);
            LASTLA[TYPE] = (LAST + NWORDS);
            LASTRC[TYPE] = LREC;
            LASTWD[TYPE] = (LWORD + 1);

            //
            // Before writing out the summary, see whether we'll need
            // a new directory; this will decide whether the first free
            // record changes.
            //
            // If we just filled in the last descriptor in a directory,
            // it's time to add a new directory record to the file.
            // All we have to do at the moment is make room for it, and
            // set the forward pointer of the current directory record
            // to point to the saved record.  Initialize the pointers
            // of the new directory record to make the linked list valid.
            //

            if ((LWORD + 1) == NWI) {
                //
                // Update the previous directory to point forward to the
                // next one.
                //
                DASURI(HANDLE, LREC, FWDLOC, FWDLOC, &[FREE], ctx)?;

                //
                // Prepare the new directory record: clear it, set the
                // backward pointer, and write the record.
                //
                CLEARI(NWI, DIRREC.as_slice_mut());

                DIRREC[BWDLOC] = LREC;

                DASWRI(HANDLE, FREE, DIRREC.as_slice(), ctx)?;

                //
                // Update the free record number.
                //
                FREE = (FREE + 1);
            }

            //
            // Now write out the file summary.
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
        } else {
            //
            // This is case (3c).  We must put the new descriptor in
            // the last directory record, which is currently empty.
            //
            // As in case (3b), we may have to update the directory
            // containing the current last descriptor of TYPE, if the
            // range of addresses covered by the cluster it describes was
            // increased by the last data addition.  Of course, this
            // update is required only if there IS such a descriptor.
            //
            if ((DSCREC > 0) && (ROOM > 0)) {
                //
                // Update the address range for TYPE in record DSCREC.
                // The upper bound is increased by ROOM, since that many
                // words of TYPE were added to the last record in the
                // last cluster of TYPE described by that directory.
                //
                MAXADR = (LAST + ROOM);

                DASURI(HANDLE, DSCREC, (RNGLOC + 1), (RNGLOC + 1), &[MAXADR], ctx)?;
            }

            //
            // Obtain the record number for this directory.
            //
            DASRRI(
                HANDLE,
                LREC,
                FWDLOC,
                FWDLOC,
                std::slice::from_mut(&mut RECNO),
                ctx,
            )?;

            //
            // Now fill in the new directory record.  Start with a clean
            // record.
            //
            CLEARI(NWI, DIRREC.as_slice_mut());

            //
            // Set the backward pointer, the address range for TYPE,
            // initial data type, and record count.
            //
            DIRREC[BWDLOC] = LREC;
            DIRREC[RNGLOC] = ((LAST + ROOM) + 1);
            DIRREC[(RNGLOC + 1)] = (LAST + NWORDS);
            DIRREC[BEGDSC] = TYPE;
            DIRREC[(BEGDSC + 1)] = NEEDED;

            //
            // Write out the record.
            //
            DASWRI(HANDLE, RECNO, DIRREC.as_slice(), ctx)?;

            //
            // Update the file summary to reflect the new record and word
            // offsets of the last descriptor of the indicated type.  The
            // last address of TYPE has increased also.  The first free
            // record lies after the added data records.
            //
            FREE = (FREE + NEEDED);
            LASTLA[TYPE] = (LAST + NWORDS);
            LASTRC[TYPE] = RECNO;
            LASTWD[TYPE] = (BEGDSC + 1);

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
        }
    }

    CHKOUT(b"DASCUD", ctx)?;
    Ok(())
}
