//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MXJOIN: i32 = 10;
pub const MXJRS: i32 = 200;
const JSZIDX: i32 = 1;
const JRCIDX: i32 = 2;
const JTCIDX: i32 = 3;
const JSCIDX: i32 = 4;
const JSVBAS: i32 = 4;

struct SaveVars {
    ADDRSS: i32,
    BEGIDX: StackArray<i32, 200>,
    J: i32,
    JRSIDX: i32,
    MAXRWV: i32,
    NSV: i32,
    NTABS: i32,
    RELOFF: i32,
    RBAS: StackArray<i32, 200>,
    SVBAS: StackArray<i32, 200>,
    SVNJRS: i32,
    TOP: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ADDRSS: i32 = 0;
        let mut BEGIDX = StackArray::<i32, 200>::new(1..=MXJRS);
        let mut J: i32 = 0;
        let mut JRSIDX: i32 = 0;
        let mut MAXRWV: i32 = 0;
        let mut NSV: i32 = 0;
        let mut NTABS: i32 = 0;
        let mut RELOFF: i32 = 0;
        let mut RBAS = StackArray::<i32, 200>::new(1..=MXJRS);
        let mut SVBAS = StackArray::<i32, 200>::new(1..=MXJRS);
        let mut SVNJRS: i32 = 0;
        let mut TOP: i32 = 0;

        Self {
            ADDRSS,
            BEGIDX,
            J,
            JRSIDX,
            MAXRWV,
            NSV,
            NTABS,
            RELOFF,
            RBAS,
            SVBAS,
            SVNJRS,
            TOP,
        }
    }
}

//$Procedure  ZZEKVADR  ( Compute row vector address )
pub fn ZZEKVADR(
    NJRS: i32,
    BASES: &[i32],
    RWVIDX: i32,
    RWVBAS: i32,
    SGVBAS: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Include Section:  EK Join Row Set Parameters
    //
    //    JRS$INC Version 1    17-SEP-1994 (NJB)
    //
    // Base-relative index of join row set size
    //

    //
    // Index of row vector count
    //

    //
    // Index of table count
    //

    //
    // Index of segment vector count
    //

    //
    // Base address of first segment vector
    //
    //
    //
    // End Include Section:  EK Join Row Set Parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKVADR", ctx)?;
    }

    //
    // Never come here.
    //
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"ZZEKVADR", ctx)?;
    Ok(())
}

//$Procedure  ZZEKVSET  ( Row vector address calculation set-up )
pub fn ZZEKVSET(NJRS: i32, BASES: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BASES = DummyArray::new(BASES, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKVSET", ctx)?;
    }

    //
    // Validate join row set count.
    //
    if ((NJRS < 1) || (NJRS > MXJRS)) {
        SETMSG(b"Number of join row sets was #; valid range is 1:#", ctx);
        ERRINT(b"#", NJRS, ctx);
        ERRINT(b"#", MXJRS, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKVSET", ctx)?;
        return Ok(());
    }

    //
    // Validate the join row set bases.
    //
    ZZEKSTOP(&mut save.TOP, ctx);

    for I in 1..=NJRS {
        if ((BASES[I] < 0) || (BASES[I] > save.TOP)) {
            SETMSG(b"Base address # was #; valid range is 1:#", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", BASES[I], ctx);
            ERRINT(b"#", save.TOP, ctx);
            SIGERR(b"SPICE(BADADDRESS)", ctx)?;
            CHKOUT(b"ZZEKVSET", ctx)?;
            return Ok(());
        }

        save.SVBAS[I] = BASES[I];
    }

    //
    // Validate and save the table count.  It's an error for this
    // count not to be identical for all of the join row sets in the
    // union.
    //
    save.ADDRSS = (BASES[1] + JTCIDX);
    ZZEKSRD(
        save.ADDRSS,
        save.ADDRSS,
        std::slice::from_mut(&mut save.NTABS),
        ctx,
    )?;

    if ((save.NTABS < 1) || (save.NTABS > MXJOIN)) {
        SETMSG(
            b"Table count for first join row set was #; valid range is 1:#",
            ctx,
        );
        ERRINT(b"#", save.NTABS, ctx);
        ERRINT(b"#", MXJOIN, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKVSET", ctx)?;
        return Ok(());
    }

    for I in 2..=NJRS {
        save.ADDRSS = (BASES[I] + JTCIDX);
        ZZEKSRD(
            save.ADDRSS,
            save.ADDRSS,
            std::slice::from_mut(&mut save.J),
            ctx,
        )?;

        if (save.J != save.NTABS) {
            SETMSG(b"Join row set # contains # tables; first join row set contains # tables.  These counts are supposed to match.", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", save.J, ctx);
            ERRINT(b"#", save.NTABS, ctx);
            SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
            CHKOUT(b"ZZEKVSET", ctx)?;
            return Ok(());
        }
    }

    //
    // Validate the row vector counts for each join row set.
    // These counts must be in range.  Save the start indices of
    // the row vectors in each join row set.
    //
    CLEARI(MXJRS, save.BEGIDX.as_slice_mut());
    save.BEGIDX[1] = 1;

    for I in 1..=NJRS {
        save.ADDRSS = (BASES[I] + JRCIDX);
        ZZEKSRD(
            save.ADDRSS,
            save.ADDRSS,
            std::slice::from_mut(&mut save.J),
            ctx,
        )?;

        if ((save.J < 0) || (save.J > save.TOP)) {
            SETMSG(b"Join row set # has row count #; valid range is 0:#", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", save.J, ctx);
            ERRINT(b"#", save.TOP, ctx);
            SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
            CHKOUT(b"ZZEKVSET", ctx)?;
            return Ok(());
        }

        if (I < NJRS) {
            save.BEGIDX[(I + 1)] = (save.BEGIDX[I] + save.J);
        }
    }

    //
    // Retain the index of the last row vector.
    //
    save.MAXRWV = (save.BEGIDX[NJRS] + save.J);

    //
    // Save the base addresses of the row vectors in each join row set.
    // Validate the segment vector counts while we're at it.
    //
    for I in 1..=NJRS {
        save.ADDRSS = (BASES[I] + JSCIDX);
        ZZEKSRD(
            save.ADDRSS,
            save.ADDRSS,
            std::slice::from_mut(&mut save.NSV),
            ctx,
        )?;

        if (save.NSV < 0) {
            SETMSG(
                b"Join row set # has segment vector count #; count must be non-negative.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", save.NSV, ctx);
            ERRINT(b"#", save.TOP, ctx);
            SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
            CHKOUT(b"ZZEKVSET", ctx)?;
            return Ok(());
        }

        save.RBAS[I] = (save.ADDRSS + (save.NSV * (save.NTABS + 2)));
    }

    //
    // Retain the count of join row sets in the union.
    //
    save.SVNJRS = NJRS;

    CHKOUT(b"ZZEKVSET", ctx)?;
    Ok(())
}

//$Procedure  ZZEKVCAL  ( Row vector address calculation  )
pub fn ZZEKVCAL(
    RWVIDX: i32,
    RWVBAS: &mut i32,
    SGVBAS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Use discovery check-in for speed; don't check RETURN.
    //
    //
    // If the index is out of range, that's an error.
    //
    if ((RWVIDX < 1) || (RWVIDX > save.MAXRWV)) {
        CHKIN(b"ZZEKVCAL", ctx)?;
        SETMSG(b"Row vector index was #; valid range is 0:#", ctx);
        ERRINT(b"#", RWVIDX, ctx);
        ERRINT(b"#", save.MAXRWV, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKVCAL", ctx)?;
        return Ok(());
    }

    //
    // Identify the join row set containing the indicated row.  Our error
    // check guarantees a non-zero result.
    //
    save.JRSIDX = LSTLEI(RWVIDX, save.SVNJRS, save.BEGIDX.as_slice());

    //
    // Compute the offset of the indicated row vector relative to the
    // first row vector in the parent join row set.  This offset is one
    // less than the relative index of the row vector, multiplied by
    // the augmented row vector size.
    //
    save.RELOFF = ((RWVIDX - save.BEGIDX[save.JRSIDX]) * (save.NTABS + 1));

    //
    // Find the base address of the row vector.
    //
    *RWVBAS = (save.RBAS[save.JRSIDX] + save.RELOFF);

    //
    // Compute the base address of the parent segment vector.  The base-
    // relative address of the segment vector is stored at the end of the
    // row vector.
    //
    ZZEKSRD(
        ((*RWVBAS + save.NTABS) + 1),
        ((*RWVBAS + save.NTABS) + 1),
        std::slice::from_mut(SGVBAS),
        ctx,
    )?;

    *SGVBAS = (save.SVBAS[save.JRSIDX] + *SGVBAS);

    Ok(())
}
