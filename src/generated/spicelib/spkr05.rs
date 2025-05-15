//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const SIZE: i32 = 100;
const DIRSIZ: i32 = SIZE;
const BUFSIZ: i32 = SIZE;
const STASIZ: i32 = 6;
const ND: i32 = 2;
const NI: i32 = 6;

/// Read SPK record from segment, type 5
///
/// Read a single SPK data record from a segment of type 5
/// ( two body propagation between discrete state vectors ).
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
///           the type 05 SPK segment to be read.
///
///  ET       is a target epoch, specified as ephemeris seconds past
///           J2000, for which a data record from the segment is
///           required.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is a logical record from the specified segment which,
///           when evaluated at epoch ET, will give the state
///           (position and velocity) of some body, relative
///           to some center, in some inertial reference frame.
///
///           The structure of RECORD is:
///
///               RECORD(1)
///                  .            state of the body at epoch 1.
///                  .
///                  .
///               RECORD(6)
///
///               RECORD(7)
///                  .
///                  .            state of the body at epoch 2.
///                  .
///               RECORD(12)
///               RECORD(13)      epoch 1 in seconds past 2000.
///               RECORD(14)      epoch 2 in seconds past 2000.
///               RECORD(15)      GM for the center of motion.
///
///
///           Epoch 1 and epoch 2 are the times in the segment that
///           bracket ET. If ET is less than the first time in the
///           segment then both epochs 1 and 2 are equal to the
///           first time. And if ET is greater than the last time
///           then, epochs 1 and 2 are set equal to this last time.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the segment specified by DESCR is not of data type 05,
///      the error SPICE(WRONGSPKTYPE) is signaled.
///
///  2)  No error is signaled if ET is outside the time bounds of
///      the segment. The output RECORD will contain epochs and the
///      associated states which satisfy the rules stated above.
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
///  This routine reads the segment specified by DESCR from the SPK
///  file attached to HANDLE to locate the two epochs in the segment
///  that bracket the input ET. It then returns a logical record which
///  contains these times and their associated states, and also the
///  mass of the center of motion. The routine makes explicit use of
///  the structure of the type 05 data segment to locate this data.
///
///  See the section of the SPK Required Reading on data type 05 for
///  a description of the structure of a type 05 segment.
/// ```
///
/// # Examples
///
/// ```text
///  The data returned by the SPKRnn routine is in its rawest form,
///  taken directly from the segment. As such, it will be meaningless
///  to a user unless he/she understands the structure of the data type
///  completely. Given that understanding, however, the SPKRnn
///  routines might be used to "dump" and check segment data for a
///  particular epoch.
///
///
///  C
///  C     Get a segment applicable to a specified body and epoch.
///  C
///        CALL SPKSFS ( BODY, ET, HANDLE, DESCR, IDENT, FOUND )
///
///        IF ( FOUND ) THEN
///
///  C
///  C        Look at parts of the descriptor.
///  C
///           CALL DAFUS ( DESCR, 2, 6, DCD, ICD )
///           CENTER = ICD( 2 )
///           REF    = ICD( 3 )
///           TYPE   = ICD( 4 )
///
///           IF ( TYPE .EQ. 05 ) THEN
///
///              CALL SPKR05 ( HANDLE, DESCR, ET, RECORD )
///                  .
///                  .  Look at the RECORD data.
///                  .
///           END IF
///
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.1, 12-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///         Added IMPLICIT NONE.
///
/// -    SPICELIB Version 1.0.0, 01-APR-1992 (JML) (WLT) (IMU)
/// ```
pub fn spkr05(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64; 5],
    et: f64,
    record: &mut [f64],
) -> crate::Result<()> {
    SPKR05(handle, descr, et, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKR05 ( Read SPK record from segment, type 5 )
pub fn SPKR05(
    HANDLE: i32,
    DESCR: &[f64],
    ET: f64,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..=5);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DATA = StackArray::<f64, 100>::new(1..=SIZE);
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut BEGIN: i32 = 0;
    let mut END: i32 = 0;
    let mut TYPE: i32 = 0;
    let mut NREC: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut GROUP: i32 = 0;
    let mut DIRLOC: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut SKIP: i32 = 0;
    let mut GRPADD: i32 = 0;
    let mut ADDRSS: i32 = 0;
    let mut I: i32 = 0;
    let mut N: i32 = 0;
    let mut FND: bool = false;

    //
    // Local parameters
    //

    //
    // SPICELIB functions
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
        CHKIN(b"SPKR05", ctx)?;
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

    TYPE = IC[4];
    BEGIN = IC[5];
    END = IC[6];

    //
    // Make sure that this really is a type 5 data segment.
    //
    if (TYPE != 5) {
        SETMSG(
            b"You are attempting to locate type 5 data in a type # data segment.",
            ctx,
        );
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(WRONGSPKTYPE)", ctx)?;
        CHKOUT(b"SPKR05", ctx)?;
        return Ok(());
    }

    //
    // Get the number of records in the segment. While we're at it,
    // get the GM of the central body (it's adjacent to NREC)
    // since we'll need it anyway. Put it where it belongs, at the
    // end of the output record.
    //
    DAFGDA(HANDLE, (END - 1), END, DATA.as_slice_mut(), ctx)?;

    NREC = intrinsics::IDNINT(DATA[2]);
    RECORD[15] = DATA[1];

    //
    // From the number of records, we can compute the number of
    // directory epochs.
    //
    NDIR = (NREC / DIRSIZ);

    //
    // The directory epochs narrow down the search to a group of DIRSIZ
    // or fewer records. Because the Ith directory epoch is the I*100th
    // epoch, the Ith group will contain epochs ((I-1)*100 + 1) through
    // (I*100).  For example:
    //                        group   first epoch #   last epoch #
    //                        -----   -------------   ------------
    //                          1               1          100
    //                          2             101          200
    //                          .               .            .
    //                          .               .            .
    //                         10             901         1000
    //                          .               .            .
    //                          .               .            .
    //                          N     (N-1)*100+1        N*100

    if (NDIR == 0) {
        //
        // There is only one group if there are no directory epochs.
        //
        GROUP = 1;
    } else {
        //
        // Compute the location of the first directory epoch.  From the
        // beginning of the segment, we need to go through all of the
        // NREC states and epochs.
        //
        DIRLOC = (BEGIN + ((STASIZ + 1) * NREC));

        //
        // Determine which group of DIRSIZ times to search, by finding
        // the last directory epoch that is less than ET.
        //
        FND = false;
        REMAIN = NDIR;
        GROUP = 0;

        while !FND {
            //
            // Read in as many as BUFSIZ directory epochs at a time
            // for comparison.
            //
            N = intrinsics::MIN0(&[REMAIN, BUFSIZ]);

            DAFGDA(HANDLE, DIRLOC, ((DIRLOC + N) - 1), DATA.as_slice_mut(), ctx)?;

            REMAIN = (REMAIN - N);

            //
            // Determine the last directory element in DATA that's less
            // than ET.
            //
            // If we reach the end of the directories, and still haven't
            // found one bigger than the epoch, the group is the last group
            // in the segment.
            //
            // Otherwise keep looking.
            //
            //
            I = LSTLTD(ET, N, DATA.as_slice());

            if (I < N) {
                GROUP = ((GROUP + I) + 1);
                FND = true;
            } else if (REMAIN == 0) {
                GROUP = (NDIR + 1);
                FND = true;
            } else {
                DIRLOC = (DIRLOC + N);
                GROUP = (GROUP + N);
            }
        }
    }

    //
    // Now we know which group of DIRSIZ (or less) epochs to look at.
    // Out of the NREC epochs, the number that we should skip over
    // to get to the proper group is DIRSIZ * ( GROUP - 1 ).
    //
    SKIP = (DIRSIZ * (GROUP - 1));

    //
    // From this we can compute the index into the segment of the group
    // of times we want.  From the beginning, we need to pass through
    // STASIZ * NREC state numbers to get to the first epoch. Then we
    // skip over the number just computed above.
    //
    GRPADD = ((BEGIN + (NREC * STASIZ)) + SKIP);

    //
    // The number of epochs that we have to look at may be less than
    // DIRSIZ.  However many there are, go ahead and read them into the
    // buffer.
    //
    // If there are no times in the last group then the time that we
    // are looking for is the same as the last directory epoch.
    // We should not try to read in this instance.
    //
    N = intrinsics::MIN0(&[DIRSIZ, (NREC - SKIP)]);

    if (N != 0) {
        DAFGDA(HANDLE, GRPADD, ((GRPADD + N) - 1), DATA.as_slice_mut(), ctx)?;
        //
        // Find the index of the largest time in the group that is less
        // than the input time.
        //
        I = LSTLTD(ET, N, DATA.as_slice());
    } else {
        //
        // If we are here it means that ET is greater then the last
        // time in the segment and there are no elements in the last
        // group.  This can occur when the number of epochs is a multiple
        // DIRSIZ.
        //
        // By setting N equal to I we can handle this case in the
        // same branch as when there are elements in the last group.
        // This is because the DATA array still contains the directory
        // epochs and I is pointing at the last element which is also the
        // last time in the segment.
        //
        N = I;
    }

    //
    // At this point N is the number of epochs in this GROUP which is
    // also the size of the array DATA which contains the epochs. I is
    // the index of the largest time in DATA which is less than ET.
    //
    // We need to take different actions depending on whether ET is less
    // than the first time or greater than the last one in the GROUP.
    //

    if (I == 0) {
        if (GROUP == 1) {
            //
            // ET is less than or equal to the first time in the segment.
            // Return the state at the first time twice.
            //
            RECORD[13] = DATA[1];
            RECORD[14] = DATA[1];

            DAFGDA(HANDLE, BEGIN, (BEGIN + 5), DATA.as_slice_mut(), ctx)?;

            MOVED(DATA.as_slice(), 6, RECORD.subarray_mut(1));
            MOVED(DATA.as_slice(), 6, RECORD.subarray_mut(7));

            CHKOUT(b"SPKR05", ctx)?;
            return Ok(());
        } else {
            //
            // ET is less than or equal to the first time in this group
            // but not the first time in the segment. Get the last time
            // from the preceding group. The states for this case will
            // be read outside of the IF block.
            //
            DAFGDA(HANDLE, (GRPADD - 1), GRPADD, DATA.as_slice_mut(), ctx)?;

            RECORD[13] = DATA[1];
            RECORD[14] = DATA[2];
        }
    } else if (I == N) {
        if (GROUP == (NDIR + 1)) {
            //
            // ET is greater than all of the times in the segment. Return
            // the state for the last time twice.
            //
            RECORD[13] = DATA[N];
            RECORD[14] = DATA[N];

            ADDRSS = (BEGIN + ((NREC - 1) * STASIZ));

            DAFGDA(HANDLE, ADDRSS, (ADDRSS + 5), DATA.as_slice_mut(), ctx)?;

            MOVED(DATA.as_slice(), 6, RECORD.subarray_mut(1));
            MOVED(DATA.as_slice(), 6, RECORD.subarray_mut(7));

            CHKOUT(b"SPKR05", ctx)?;
            return Ok(());
        } else {
            //
            // ET is greater than the last time in this group but this is
            // not the last time in the segment.  Need the first time from
            // the following group. The states for this case will be read
            // outside of the IF block.
            //
            DAFGDA(
                HANDLE,
                ((GRPADD + N) - 1),
                (GRPADD + N),
                DATA.as_slice_mut(),
                ctx,
            )?;

            RECORD[13] = DATA[1];
            RECORD[14] = DATA[2];
        }
    } else {
        //
        // There are two times in the group that bracket ET. The states
        // for this case will be read outside of the IF block.
        //
        RECORD[13] = DATA[I];
        RECORD[14] = DATA[(I + 1)];
    }

    //
    // Read the consecutive states for the two epochs found above.
    // ET is greater than the (SKIP + I)th time but less than or
    // equal to the time (SKIP + I + 1).
    //

    ADDRSS = (BEGIN + (((SKIP + I) - 1) * STASIZ));

    DAFGDA(HANDLE, ADDRSS, (ADDRSS + 11), DATA.as_slice_mut(), ctx)?;

    MOVED(DATA.as_slice(), 12, RECORD.subarray_mut(1));

    CHKOUT(b"SPKR05", ctx)?;
    Ok(())
}
