//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXJRS: i32 = 200;
const JSZIDX: i32 = 1;
const JRCIDX: i32 = 2;
const JTCIDX: i32 = 3;
const JSCIDX: i32 = 4;
const JSVBAS: i32 = 4;
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;

//$Procedure  ZZEKJSQZ ( Private: EK, join row set squeeze )
pub fn ZZEKJSQZ(JRSBAS: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CNTLOC: i32 = 0;
    let mut DELTA: i32 = 0;
    let mut NSVDEL: i32 = 0;
    let mut NRVDEL: i32 = 0;
    let mut NEWNSV: i32 = 0;
    let mut NR: i32 = 0;
    let mut NRLOC: i32 = 0;
    let mut NSV: i32 = 0;
    let mut NSVLOC: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut NTLOC: i32 = 0;
    let mut PCPAIR = StackArray::<i32, 2>::new(1..=2);
    let mut PTARG: i32 = 0;
    let mut PTBASE: i32 = 0;
    let mut PTRLOC: i32 = 0;
    let mut RC: i32 = 0;
    let mut RBASE: i32 = 0;
    let mut ROWVEC = StackArray::<i32, 11>::new(1..=(MAXTAB + 1));
    let mut RTARG: i32 = 0;
    let mut RVSIZE: i32 = 0;
    let mut SEGVEC = StackArray::<i32, 10>::new(1..=MAXTAB);
    let mut SETBAS: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut SIZLOC: i32 = 0;
    let mut SVBASE: i32 = 0;
    let mut SVSIZE: i32 = 0;
    let mut VTARG: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    //
    // Look up the counts that are of interest:
    //
    //   -- The table count
    //   -- The segment vector count
    //   -- The join row set size
    //
    // Save the address of each count.
    //
    SIZLOC = (JRSBAS + JSZIDX);
    NSVLOC = (JRSBAS + JSCIDX);
    NTLOC = (JRSBAS + JTCIDX);

    ZZEKSRD(SIZLOC, SIZLOC, std::slice::from_mut(&mut SIZE), ctx)?;
    ZZEKSRD(NTLOC, NTLOC, std::slice::from_mut(&mut NTAB), ctx)?;
    ZZEKSRD(NSVLOC, NSVLOC, std::slice::from_mut(&mut NSV), ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Set the sizes of segment and row vectors.
    //
    SVSIZE = NTAB;
    RVSIZE = (NTAB + 1);

    //
    // For each segment vector, obtain the row count.  Clean up after
    // null segment vectors:  compress out the space allocated for their
    // row vector pointers.  Keep track of the number of deletions.
    //
    NSVDEL = 0;
    NRVDEL = 0;
    VTARG = (JRSBAS + JSVBAS);

    for I in 1..=NSV {
        //
        // The location of the row count is CNTLOC.  The row vector base
        // pointer precedes the row count.
        //
        CNTLOC = ((((JRSBAS + JSVBAS) + (NSV * SVSIZE)) + (2 * (I - 1))) + 2);
        PTRLOC = (CNTLOC - 1);

        ZZEKSRD(CNTLOC, CNTLOC, std::slice::from_mut(&mut RC), ctx)?;

        if (RC > 0) {
            //
            // The row vector set for this segment vector is non-empty.
            // scan the rows, looking for those marked for deletion, and
            // update the row count to reflect the number of rows that
            // we're going to keep.
            //
            ZZEKSRD(PTRLOC, PTRLOC, std::slice::from_mut(&mut SETBAS), ctx)?;

            NRVDEL = 0;

            for J in 1..=RC {
                RBASE = ((JRSBAS + SETBAS) + ((J - 1) * RVSIZE));

                ZZEKSRD((RBASE + 1), (RBASE + 1), ROWVEC.subarray_mut(1), ctx)?;

                if (ROWVEC[1] == 0) {
                    NRVDEL = (NRVDEL + 1);
                }
            }
        }

        //
        // Compute the base address of the current segment vector.
        //
        SVBASE = ((JRSBAS + JSVBAS) + ((I - 1) * SVSIZE));

        if ((RC == 0) || (NRVDEL == RC)) {
            //
            // We're going to delete the current segment vector.  We'll
            // just skip over it without advancing our target pointers.
            //
            NSVDEL = (NSVDEL + 1);
        } else if (NSVDEL > 0) {
            //
            // We need to shift the current segment vector to its
            // destination.
            //
            ZZEKSRD((SVBASE + 1), (SVBASE + SVSIZE), SEGVEC.as_slice_mut(), ctx)?;
            ZZEKSUPD((VTARG + 1), (VTARG + SVSIZE), SEGVEC.as_slice(), ctx)?;

            VTARG = (VTARG + SVSIZE);
        } else {
            //
            // No segment vectors have been deleted yet.  We still must
            // update the target in case we shift vectors later on in this
            // loop.
            //
            VTARG = (VTARG + SVSIZE);
        }
    }

    //
    // At this point, we've compressed out the null segment vectors.
    // The next step is to compress out the row vector counts and row
    // vector pointers that corresponded to those segment vectors.  We
    // also want to remove the gap between the segment vectors and the
    // row vector pointer/count pairs.
    //
    // We need to do this only if we deleted some segment vectors.
    //
    if (NSVDEL > 0) {
        NEWNSV = (NSV - NSVDEL);
        PTARG = ((JRSBAS + JSVBAS) + (NEWNSV * SVSIZE));

        for I in 1..=NSV {
            //
            // The row count is RC.
            //
            SVSIZE = NTAB;
            CNTLOC = ((((JRSBAS + JSVBAS) + (NSV * SVSIZE)) + (2 * (I - 1))) + 2);

            ZZEKSRD(CNTLOC, CNTLOC, std::slice::from_mut(&mut RC), ctx)?;

            PTBASE = (CNTLOC - 2);

            if (RC > 0) {
                //
                // Shift the current row vector pointer and row vector
                // count.
                //
                ZZEKSRD((PTBASE + 1), (PTBASE + 2), PCPAIR.as_slice_mut(), ctx)?;
                ZZEKSUPD((PTARG + 1), (PTARG + 2), PCPAIR.as_slice(), ctx)?;

                PTARG = (PTARG + 2);
            }
        }
    } else {
        NEWNSV = NSV;
    }

    //
    // Update the segment vector count.
    //
    ZZEKSUPD(NSVLOC, NSVLOC, &[NEWNSV], ctx)?;

    //
    // Remove any gaps that may exist between any of the row vectors,
    // or between the end of the segment vector's row vector counts
    // and base addresses and the first row vector.
    //
    // The initial target location is the first element following the
    // last segment vector's row vector count.  RTARG is used as a base
    // address; it precedes this location by 1.
    //
    // If we deleted any segment vectors, the segment vector pointers
    // embedded in the row vectors must change.  Make these updates
    // if necessary.
    //
    //
    RTARG = ((JRSBAS + JSVBAS) + (NEWNSV * (SVSIZE + 2)));

    for I in 1..=NEWNSV {
        //
        // Find the row count and row pointer for the current segment
        // vector.
        //
        CNTLOC = ((((JRSBAS + JSVBAS) + (NEWNSV * SVSIZE)) + (2 * (I - 1))) + 2);

        ZZEKSRD(CNTLOC, CNTLOC, std::slice::from_mut(&mut RC), ctx)?;

        PTRLOC = (CNTLOC - 1);

        //
        // Get the row vector set base pointer.  After capturing the
        // current value, we'll update this pointer to account for
        // the shifting of row vectors.
        //
        ZZEKSRD(PTRLOC, PTRLOC, std::slice::from_mut(&mut SETBAS), ctx)?;

        RBASE = (JRSBAS + SETBAS);
        DELTA = (RTARG - RBASE);

        ZZEKSUPD(PTRLOC, PTRLOC, &[(SETBAS + DELTA)], ctx)?;

        //
        // Shift the row vectors for the current segment vector,
        // leaving behind the row vectors marked for deletion.
        //
        NRVDEL = 0;

        for J in 1..=RC {
            ZZEKSRD((RBASE + 1), (RBASE + RVSIZE), ROWVEC.as_slice_mut(), ctx)?;

            if (ROWVEC[1] == 0) {
                //
                // This row vector is to be deleted; don't copy it.
                //
                RBASE = (RBASE + RVSIZE);
                NRVDEL = (NRVDEL + 1);
            } else {
                //
                // The segment vector pointer is base-relative.
                //
                ROWVEC[RVSIZE] = (JSVBAS + ((I - 1) * SVSIZE));

                ZZEKSUPD((RTARG + 1), (RTARG + RVSIZE), ROWVEC.as_slice(), ctx)?;

                RBASE = (RBASE + RVSIZE);
                RTARG = (RTARG + RVSIZE);
            }
        }

        //
        // Update the row count for the current segment vector, if
        // necessary.  Note that no segment vector will become empty
        // as a result of the row vector deletions we've done; we
        // already eliminated any segment vectors for which that
        // could happen, before we entered this loop.
        //
        if (NRVDEL > 0) {
            ZZEKSUPD(CNTLOC, CNTLOC, &[(RC - NRVDEL)], ctx)?;
        }
    }

    //
    // Update the total row count and size of the join row set.
    //
    NR = 0;

    for I in 1..=NEWNSV {
        CNTLOC = ((((JRSBAS + JSVBAS) + (NEWNSV * SVSIZE)) + ((I - 1) * 2)) + 2);

        ZZEKSRD(CNTLOC, CNTLOC, std::slice::from_mut(&mut RC), ctx)?;

        NR = (NR + RC);
    }

    NRLOC = (JRSBAS + JRCIDX);
    SIZE = ((JSVBAS + (NEWNSV * (SVSIZE + 2))) + (NR * RVSIZE));

    ZZEKSUPD(NRLOC, NRLOC, &[NR], ctx)?;
    ZZEKSUPD(SIZLOC, SIZLOC, &[SIZE], ctx)?;

    Ok(())
}
