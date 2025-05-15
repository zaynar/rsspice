//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
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
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const LBPOOL: i32 = -5;

//$Procedure  ZZEKJOIN  ( Perform join on two join row sets )
pub fn ZZEKJOIN(
    JBASE1: i32,
    JBASE2: i32,
    NJCNST: i32,
    ACTIVE: &[bool],
    CPIDX1: &[i32],
    CLIDX1: &[i32],
    ELTS1: &[i32],
    OPS: &[i32],
    CPIDX2: &[i32],
    CLIDX2: &[i32],
    ELTS2: &[i32],
    STHAN: &[i32],
    STSDSC: &[i32],
    STDTPT: &[i32],
    DTPOOL: &[i32],
    DTDSCS: &[i32],
    JBASE3: &mut i32,
    NROWS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ACTIVE = DummyArray::new(ACTIVE, 1..);
    let CPIDX1 = DummyArray::new(CPIDX1, 1..);
    let CLIDX1 = DummyArray::new(CLIDX1, 1..);
    let ELTS1 = DummyArray::new(ELTS1, 1..);
    let OPS = DummyArray::new(OPS, 1..);
    let CPIDX2 = DummyArray::new(CPIDX2, 1..);
    let CLIDX2 = DummyArray::new(CLIDX2, 1..);
    let ELTS2 = DummyArray::new(ELTS2, 1..);
    let STHAN = DummyArray::new(STHAN, 1..);
    let STSDSC = DummyArray2D::new(STSDSC, 1..=SDSCSZ, 1..);
    let STDTPT = DummyArray::new(STDTPT, 1..);
    let DTPOOL = DummyArray2D::new(DTPOOL, 1..=2, LBPOOL..);
    let DTDSCS = DummyArray2D::new(DTDSCS, 1..=CDSCSZ, 1..);
    let mut OFFSET: i32 = 0;
    let mut NR1: i32 = 0;
    let mut NR2: i32 = 0;
    let mut NR3: i32 = 0;
    let mut NRESV: i32 = 0;
    let mut NSV1: i32 = 0;
    let mut NSV2: i32 = 0;
    let mut NSV3: i32 = 0;
    let mut NT1: i32 = 0;
    let mut NT2: i32 = 0;
    let mut NT3: i32 = 0;
    let mut RB1: i32 = 0;
    let mut RB2: i32 = 0;
    let mut RB3: i32 = 0;
    let mut ROWVEC = StackArray::<i32, 11>::new(1..=(MXJOIN + 1));
    let mut S3: i32 = 0;
    let mut SEGVEC = StackArray::<i32, 10>::new(1..=MXJOIN);
    let mut SGVBAS: i32 = 0;
    let mut TOP: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local variables
    //

    //
    // For speed, we use discovery check-in.  We don't check
    // RETURN at all.
    //
    //
    // Validate constraint count.
    //
    if ((NJCNST < 0) || (NJCNST > MXJCON)) {
        CHKIN(b"ZZEKJOIN", ctx)?;
        SETMSG(b"Number of join constraints was #; valid range is 0:#", ctx);
        ERRINT(b"#", NJCNST, ctx);
        ERRINT(b"#", MXJCON, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKJOIN", ctx)?;
        return Ok(());
    }

    //
    // Get the table count and segment vector count for each input join
    // row set.
    //
    ZZEKSRD(
        (JBASE1 + JTCIDX),
        (JBASE1 + JTCIDX),
        std::slice::from_mut(&mut NT1),
        ctx,
    )?;
    ZZEKSRD(
        (JBASE1 + JSCIDX),
        (JBASE1 + JSCIDX),
        std::slice::from_mut(&mut NSV1),
        ctx,
    )?;
    ZZEKSRD(
        (JBASE2 + JTCIDX),
        (JBASE2 + JTCIDX),
        std::slice::from_mut(&mut NT2),
        ctx,
    )?;
    ZZEKSRD(
        (JBASE2 + JSCIDX),
        (JBASE2 + JSCIDX),
        std::slice::from_mut(&mut NSV2),
        ctx,
    )?;

    //
    // Set the table count and segment vector count for the output join
    // row set.
    //
    NT3 = (NT1 + NT2);
    NSV3 = (NSV1 * NSV2);

    if ((NT1 < 1) || (NT2 > (MXJOIN - 1))) {
        CHKIN(b"ZZEKJOIN", ctx)?;
        SETMSG(
            b"Number tables in first join row set was #; valid range is 1:#",
            ctx,
        );
        ERRINT(b"#", NT1, ctx);
        ERRINT(b"#", (MXJOIN - 1), ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKJOIN", ctx)?;
        return Ok(());
    } else if ((NT2 < 1) || (NT2 > (MXJOIN - 1))) {
        CHKIN(b"ZZEKJOIN", ctx)?;
        SETMSG(
            b"Number tables in second join row set was #; valid range is 1:#",
            ctx,
        );
        ERRINT(b"#", NT2, ctx);
        ERRINT(b"#", (MXJOIN - 1), ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKJOIN", ctx)?;
        return Ok(());
    } else if (NT3 > MXJOIN) {
        CHKIN(b"ZZEKJOIN", ctx)?;
        SETMSG(b"Number of crossed tables was #; valid range is 0:#", ctx);
        ERRINT(b"#", NT3, ctx);
        ERRINT(b"#", MXJOIN, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKJOIN", ctx)?;
        return Ok(());
    }

    //
    // Validate cross product indices.  The column indices don't lend
    // themselves to such a convenient check; we'll check those as we
    // use them.
    //
    for I in 1..=NJCNST {
        if ACTIVE[I] {
            if ((CPIDX1[I] < 1) || (CPIDX1[I] > NT3)) {
                CHKIN(b"ZZEKJOIN", ctx)?;
                SETMSG(b"Cross product table index for left hand side of constraint # was #; valid range is 1:#", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", CPIDX1[I], ctx);
                ERRINT(b"#", NT3, ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKJOIN", ctx)?;
                return Ok(());
            } else if ((CPIDX2[I] < 1) || (CPIDX2[I] > NT3)) {
                CHKIN(b"ZZEKJOIN", ctx)?;
                SETMSG(b"Cross product table index for right hand side of constraint # was #; valid range is 1:#", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", CPIDX2[I], ctx);
                ERRINT(b"#", NT3, ctx);
                SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
                CHKOUT(b"ZZEKJOIN", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Form the joint row set control area for output join row set.
    //
    // The current stack top is the base address of the output join row
    // set.
    //
    ZZEKSTOP(JBASE3, ctx);
    //
    // Save room for the size and row vector count
    //
    for I in 1..=2 {
        ZZEKSPSH(1, &[0], ctx)?;
    }

    //
    // The table count and segment vector count come next.
    //
    ZZEKSPSH(1, &[NT3], ctx)?;
    ZZEKSPSH(1, &[NSV3], ctx)?;

    //
    // Just reserve room for the segment vectors and the segment vector
    // row set base addresses and counts.
    //
    NRESV = (NSV3 * (NT3 + 2));

    for I in 1..=NRESV {
        ZZEKSPSH(1, &[0], ctx)?;
    }

    //
    // Initialize the output segment vector count and the total row
    // count.
    //
    S3 = 0;
    *NROWS = 0;

    //
    // For every segment vector in the first join row set,
    //
    for S1 in 1..=NSV1 {
        //
        // Fill in the first NT1 elements of our composite segment vector
        // with the current segment vector from the first join row set.
        //
        OFFSET = (JSVBAS + ((S1 - 1) * NT1));

        ZZEKSRD(
            ((JBASE1 + OFFSET) + 1),
            ((JBASE1 + OFFSET) + NT1),
            SEGVEC.as_slice_mut(),
            ctx,
        )?;

        //
        // Get the row set base address and count for this segment vector.
        //
        OFFSET = (((JSVBAS + (NSV1 * NT1)) + (2 * (S1 - 1))) + 1);

        ZZEKSRD(
            (JBASE1 + OFFSET),
            (JBASE1 + OFFSET),
            std::slice::from_mut(&mut RB1),
            ctx,
        )?;
        ZZEKSRD(
            ((JBASE1 + OFFSET) + 1),
            ((JBASE1 + OFFSET) + 1),
            std::slice::from_mut(&mut NR1),
            ctx,
        )?;

        //
        // For every segment vector in the second join row set,
        //
        for S2 in 1..=NSV2 {
            //
            // Fill in the last NT2 elements of our composite segment
            // vector with the current segment vector from the second join
            // row set.
            //
            OFFSET = (JSVBAS + ((S2 - 1) * NT2));

            ZZEKSRD(
                ((JBASE2 + OFFSET) + 1),
                ((JBASE2 + OFFSET) + NT2),
                SEGVEC.subarray_mut((NT1 + 1)),
                ctx,
            )?;

            //
            // Write this segment vector to the output join row set.
            //
            S3 = (S3 + 1);
            SGVBAS = (JSVBAS + ((S3 - 1) * NT3));

            ZZEKSUPD(
                ((*JBASE3 + SGVBAS) + 1),
                ((*JBASE3 + SGVBAS) + NT3),
                SEGVEC.as_slice(),
                ctx,
            )?;

            //
            // Get the row set base address and count for this segment
            // vector.
            //
            OFFSET = (((JSVBAS + (NSV2 * NT2)) + (2 * (S2 - 1))) + 1);

            ZZEKSRD(
                (JBASE2 + OFFSET),
                (JBASE2 + OFFSET),
                std::slice::from_mut(&mut RB2),
                ctx,
            )?;
            ZZEKSRD(
                ((JBASE2 + OFFSET) + 1),
                ((JBASE2 + OFFSET) + 1),
                std::slice::from_mut(&mut NR2),
                ctx,
            )?;

            //
            // It's time to decide which row vectors corresponding to
            // our two segment vectors satisfy the join constraints.
            // We pass off the job of determining which row vectors to
            // consider to the subroutine pair ZZEKJPRP (join preparation)
            // and ZZEKJNXT (get next joined row vector).
            //
            // We defer establishing the base address of the output
            // row vector set until the join reduction is done, since
            // the join operation will use the scratch area.
            //

            ZZEKJPRP(
                SEGVEC.as_slice(),
                JBASE1,
                NT1,
                RB1,
                NR1,
                JBASE2,
                NT2,
                RB2,
                NR2,
                NJCNST,
                ACTIVE.as_slice(),
                CPIDX1.as_slice(),
                CLIDX1.as_slice(),
                ELTS1.as_slice(),
                OPS.as_slice(),
                CPIDX2.as_slice(),
                CLIDX2.as_slice(),
                ELTS2.as_slice(),
                STHAN.as_slice(),
                STSDSC.as_slice(),
                STDTPT.as_slice(),
                DTPOOL.as_slice(),
                DTDSCS.as_slice(),
                ctx,
            )?;

            //
            // Initialize the row count for the current output segment
            // vector.  Also set the segment vector row set base address.
            //
            NR3 = 0;

            ZZEKSTOP(&mut TOP, ctx);

            RB3 = (TOP - *JBASE3);
            OFFSET = (((JSVBAS + (NSV3 * NT3)) + ((S3 - 1) * 2)) + 1);

            ZZEKSUPD((*JBASE3 + OFFSET), (*JBASE3 + OFFSET), &[RB3], ctx)?;

            //
            // Fetch the row vectors that satisfy the join constraints.
            //
            NR3 = 0;
            ZZEKJNXT(&mut FOUND, ROWVEC.as_slice_mut(), ctx)?;

            while FOUND {
                //
                // Append the base offset of the parent segment vector
                // to the row vector.  The base offset is one less than
                // the base-relative address of the segment vector.
                //
                NR3 = (NR3 + 1);
                ROWVEC[(NT3 + 1)] = SGVBAS;

                //
                // Add this vector to the output join row set.  Get the
                // next row vector.
                //
                ZZEKSPSH((NT3 + 1), ROWVEC.as_slice(), ctx)?;
                ZZEKJNXT(&mut FOUND, ROWVEC.as_slice_mut(), ctx)?;
            }

            //
            // At this point, we've tested every row corresponding to the
            // current segment vector.  Update the row count for this
            // segment vector.
            //
            OFFSET = (((JSVBAS + (NSV3 * NT3)) + ((S3 - 1) * 2)) + 2);

            ZZEKSUPD((*JBASE3 + OFFSET), (*JBASE3 + OFFSET), &[NR3], ctx)?;

            //
            // Keep the overall row total up to date.
            //
            *NROWS = (*NROWS + NR3);
        }
    }

    //
    // Fill in the row count and size values in the output join row
    // set.
    //
    ZZEKSTOP(&mut TOP, ctx);
    ZZEKSUPD(
        (*JBASE3 + JSZIDX),
        (*JBASE3 + JSZIDX),
        &[(TOP - *JBASE3)],
        ctx,
    )?;
    ZZEKSUPD((*JBASE3 + JRCIDX), (*JBASE3 + JRCIDX), &[*NROWS], ctx)?;

    //
    // We've constructed the output join row set resulting from
    // joining the input row sets.
    //

    Ok(())
}
