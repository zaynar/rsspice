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

//$Procedure  ZZEKWEED ( Private: EK, weed out redundant row vectors )
pub fn ZZEKWEED(
    NJRS: &mut i32,
    BASES: &mut [i32],
    NROWS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BASES = DummyArrayMut::new(BASES, 1..);
    let mut BASE: i32 = 0;
    let mut CANDSV = StackArray::<i32, 10>::new(1..=MAXTAB);
    let mut CRWBAS: i32 = 0;
    let mut CRWVEC = StackArray::<i32, 11>::new(1..=(MAXTAB + 1));
    let mut CSGBAS: i32 = 0;
    let mut J: i32 = 0;
    let mut LOC: i32 = 0;
    let mut NCNDRV: i32 = 0;
    let mut NCNDSV: i32 = 0;
    let mut NDEL: i32 = 0;
    let mut NPRDRV: i32 = 0;
    let mut NPRDSV: i32 = 0;
    let mut NR: i32 = 0;
    let mut NRLOC: i32 = 0;
    let mut NSVLOC: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut PREDSV = StackArray::<i32, 10>::new(1..=MAXTAB);
    let mut PRV: i32 = 0;
    let mut PRWBAS: i32 = 0;
    let mut PRWVEC = StackArray::<i32, 11>::new(1..=(MAXTAB + 1));
    let mut PSGBAS: i32 = 0;
    let mut RVSIZE: i32 = 0;
    let mut SVSIZE: i32 = 0;
    let mut HIT: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if ((*NJRS < 1) || (*NJRS > MXJRS)) {
        CHKIN(b"ZZEKWEED", ctx)?;
        SETMSG(b"The number of join row sets in the union is #", ctx);
        ERRINT(b"#", *NJRS, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKWEED", ctx)?;
        return Ok(());
    }

    //
    // Make sure that the addressing routines are properly initialized.
    //
    ZZEKVSET(*NJRS, BASES.as_slice(), ctx)?;

    //
    // Get the segment vector and row vector sizes.  The sizes that
    // apply to the first join row set will suffice throughout.
    //
    LOC = (BASES[1] + JTCIDX);

    ZZEKSRD(LOC, LOC, std::slice::from_mut(&mut NTAB), ctx)?;

    SVSIZE = NTAB;
    RVSIZE = (NTAB + 1);

    //
    // Mark redundant rows vectors for deletion.  One saving grace is
    // that redundant rows can never occur in the same join row set, as
    // long as that join row set represents a set of rows satisfying
    // a conjunction of constraints.
    //
    for CAND in 2..=*NJRS {
        //
        // We'll compare row vectors in the CAND join row set to row
        // vectors in the preceding join row sets.  Only row vectors
        // corresponding to matching segment vectors need be compared.
        // Therefore, we'll loop over the segment vectors in the CAND
        // join row set, and for each such segment vector, loop over the
        // segment vectors in the preceding join row sets.  If a match
        // occurs, we'll compare row vectors corresponding to those
        // segment vectors.
        //
        // NCNDSV will contain the number of segment vectors in the
        // `candidate' join row set.
        //
        NSVLOC = (BASES[CAND] + JSCIDX);

        ZZEKSRD(NSVLOC, NSVLOC, std::slice::from_mut(&mut NCNDSV), ctx)?;

        for CSV in 1..=NCNDSV {
            //
            // Look up the candidate segment vector.
            //
            CSGBAS = ((BASES[CAND] + JSVBAS) + ((CSV - 1) * SVSIZE));

            ZZEKSRD((CSGBAS + 1), (CSGBAS + SVSIZE), CANDSV.as_slice_mut(), ctx)?;

            //
            // Get the row vector count and base address of the set of
            // row vectors for the candidate segment vector, in case
            // we need them.  (Referring to the diagram of the join
            // row set structure in the join row set parameter include
            // file may be helpful here.)
            //
            BASE = (((BASES[CAND] + JSVBAS) + (NCNDSV * SVSIZE)) + ((CSV - 1) * 2));

            ZZEKSRD(
                (BASE + 1),
                (BASE + 1),
                std::slice::from_mut(&mut CRWBAS),
                ctx,
            )?;
            CRWBAS = (CRWBAS + BASES[CAND]);

            ZZEKSRD(
                (BASE + 2),
                (BASE + 2),
                std::slice::from_mut(&mut NCNDRV),
                ctx,
            )?;

            //
            // For the current predecessor join row set, look up the
            // segment vectors in that join row set and compare them to the
            // candidate.
            //
            for PRED in 1..=(CAND - 1) {
                //
                // Get the count of segment vectors in the current
                // predecessor join row set.
                //
                NSVLOC = (BASES[PRED] + JSCIDX);

                ZZEKSRD(NSVLOC, NSVLOC, std::slice::from_mut(&mut NPRDSV), ctx)?;

                for PSV in 1..=NPRDSV {
                    //
                    // Look up the predecessor segment vector.
                    //
                    PSGBAS = ((BASES[PRED] + JSVBAS) + ((PSV - 1) * SVSIZE));

                    ZZEKSRD((CSGBAS + 1), (CSGBAS + SVSIZE), PREDSV.as_slice_mut(), ctx)?;

                    //
                    // Compare the segment vectors and hope for the best.
                    //
                    if SAMEAI(CANDSV.as_slice(), PREDSV.as_slice(), SVSIZE) {
                        //
                        // Unfortunately, the two segment vectors match, so
                        // there's something to do.  We'll have to compare
                        // every row vector corresponding to the candidate
                        // segment vector with every row vector corresponding
                        // to the predecessor.
                        //
                        // Get the row vector count and base address of the
                        // set of row vectors for the current predecessor
                        // segment vector.  We already have on hand the
                        // corresponding quantities for the candidate
                        // segment vector.
                        //

                        BASE = (((BASES[PRED] + JSVBAS) + (NPRDSV * SVSIZE)) + ((PSV - 1) * 2));

                        ZZEKSRD(
                            (BASE + 1),
                            (BASE + 1),
                            std::slice::from_mut(&mut PRWBAS),
                            ctx,
                        )?;
                        PRWBAS = (PRWBAS + BASES[PRED]);

                        ZZEKSRD(
                            (BASE + 2),
                            (BASE + 2),
                            std::slice::from_mut(&mut NPRDRV),
                            ctx,
                        )?;

                        //
                        // Compare all row vectors.
                        //
                        for CRV in 1..=NCNDRV {
                            BASE = (CRWBAS + ((CRV - 1) * RVSIZE));
                            ZZEKSRD((BASE + 1), (BASE + RVSIZE), CRWVEC.as_slice_mut(), ctx)?;

                            PRV = 1;
                            HIT = false;

                            while ((PRV <= NPRDRV) && !HIT) {
                                BASE = (PRWBAS + ((PRV - 1) * RVSIZE));
                                ZZEKSRD((BASE + 1), (BASE + RVSIZE), PRWVEC.as_slice_mut(), ctx)?;

                                if SAMEAI(CRWVEC.as_slice(), PRWVEC.as_slice(), RVSIZE) {
                                    //
                                    // The row vectors, together with their
                                    // qualifying segment vectors, match.  The
                                    // higher-indexed vector is considered
                                    // redundant.  To mark this vector for
                                    // deletion, we simply zero out the first
                                    // element of the row vector.  This makes the
                                    // row vector invalid, so it will not match
                                    // any valid row vector we see later.
                                    //
                                    BASE = (CRWBAS + ((CRV - 1) * RVSIZE));
                                    ZZEKSUPD((BASE + 1), (BASE + 1), &[0], ctx)?;
                                    HIT = true;
                                } else {
                                    PRV = (PRV + 1);
                                }
                            }
                        }
                    }
                    //
                    // We've finished comparing row vectors for a pair of
                    // segment vectors, if it was necessary to do so.
                    //
                }
                //
                // We've compared all segment vectors in the current
                // predecessor join row set with the candidate segment
                // vector.
                //
            }
            //
            // We've compared all segment vectors in all predecessor join
            // row sets to the current segment vector.
            //
        }
        //
        // We've compared the candidate join row set to its predecessors.
        //
    }
    //
    // We've compared all of the join row sets.
    //
    //
    // Now, clean up the join row set union by compressing out deleted
    // rows, segment vectors, and join row sets.
    //
    J = 1;
    NDEL = 0;

    for I in 1..=*NJRS {
        //
        // Compress the current join row set.  If it ends up empty,
        // expel it from the union.
        //
        ZZEKJSQZ(BASES[I], ctx)?;

        NRLOC = (BASES[I] + JRCIDX);
        ZZEKSRD(NRLOC, NRLOC, std::slice::from_mut(&mut NR), ctx)?;

        if (NR == 0) {
            //
            // This entire join row set can be deleted from the union.
            // Consider the next join row set.
            //
            NDEL = (NDEL + 1);
        } else {
            BASES[J] = BASES[I];
            J = (J + 1);
        }
    }

    *NJRS = (*NJRS - NDEL);

    //
    // Count the rows remaining after our clean-up operation.
    //
    *NROWS = 0;

    for I in 1..=*NJRS {
        NRLOC = (BASES[I] + JRCIDX);

        ZZEKSRD(NRLOC, NRLOC, std::slice::from_mut(&mut NR), ctx)?;

        *NROWS = (*NROWS + NR);
    }

    Ok(())
}
