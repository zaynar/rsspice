//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MEMSIZ: i32 = 2500000;

struct SaveVars {
    B: i32,
    BASE: i32,
    E: i32,
    LASTC: i32,
    LASTD: i32,
    LASTI: i32,
    NUMADD: i32,
    NUMRD: i32,
    RB: i32,
    REMAIN: i32,
    RT: i32,
    SCRHAN: i32,
    SCRTCH: ActualArray<i32>,
    START: i32,
    T: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut B: i32 = 0;
        let mut BASE: i32 = 0;
        let mut E: i32 = 0;
        let mut LASTC: i32 = 0;
        let mut LASTD: i32 = 0;
        let mut LASTI: i32 = 0;
        let mut NUMADD: i32 = 0;
        let mut NUMRD: i32 = 0;
        let mut RB: i32 = 0;
        let mut REMAIN: i32 = 0;
        let mut RT: i32 = 0;
        let mut SCRHAN: i32 = 0;
        let mut SCRTCH = ActualArray::<i32>::new(1..=MEMSIZ);
        let mut START: i32 = 0;
        let mut T: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        T = 0;

        Self {
            B,
            BASE,
            E,
            LASTC,
            LASTD,
            LASTI,
            NUMADD,
            NUMRD,
            RB,
            REMAIN,
            RT,
            SCRHAN,
            SCRTCH,
            START,
            T,
            FIRST,
        }
    }
}

//$Procedure      ZZEKSCA ( EK, scratch area )
pub fn ZZEKSCA(
    N: i32,
    BEG: i32,
    END: i32,
    IDATA: &[i32],
    TOP: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // The parameter MEMSIZ is the size of an integer array used as
    // part of the scratch area.  The first MEMSIZ scratch area addresses
    // refer to elements of this array.  Additional storage is supplied
    // by the integer logical array of a scratch DAS file; the first
    // word of the scratch DAS file corresponds to scratch area address
    // MEMSIZ + 1.
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKSCA", ctx)?;
    }

    //
    // This routine should never be called directly.
    //
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"ZZEKSCA", ctx)?;
    Ok(())
}

//$Procedure    ZZEKSTOP  ( EK scratch area, stack top )
pub fn ZZEKSTOP(TOP: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *TOP = save.T;
}

//$Procedure    ZZEKSPSH  ( EK scratch area, push data )
pub fn ZZEKSPSH(N: i32, IDATA: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let IDATA = DummyArray::new(IDATA, 1..);

    //
    // No checking in here.
    //

    //
    // First time through, open a scratch DAS file.
    //
    if save.FIRST {
        save.FIRST = false;

        DASOPS(&mut save.SCRHAN, ctx)?;

        if FAILED(ctx) {
            return Ok(());
        }
    }

    //
    // Go back if there's no data to write.
    //
    if (N < 1) {
        return Ok(());
    }

    //
    // Add as much data as possible to our big array.
    //
    if (save.T < MEMSIZ) {
        save.NUMADD = intrinsics::MIN0(&[N, (MEMSIZ - save.T)]);

        for I in 1..=save.NUMADD {
            save.SCRTCH[(save.T + I)] = IDATA[I];
        }

        save.T = (save.T + save.NUMADD);

        if (save.NUMADD == N) {
            return Ok(());
        }

        save.REMAIN = (N - save.NUMADD);
        save.START = (1 + save.NUMADD);

        if (save.REMAIN == 0) {
            return Ok(());
        }
    } else {
        save.REMAIN = N;
        save.START = 1;
    }
    //
    // At this point, REMAIN and START are set, and T reflects the
    // amount of data we've pushed so far.  If we got this far,
    // we'll need to put the rest of the data in the scratch DAS.
    //
    // The DAS system requires separate operations for updating
    // an existing range of addresses and for appending data.
    // We need to know the last integer address in use in the DAS
    // file in order to determine which part of the data will
    // be written to addresses previously written to, and which
    // part will be appended.
    //
    DASLLA(
        save.SCRHAN,
        &mut save.LASTC,
        &mut save.LASTD,
        &mut save.LASTI,
        ctx,
    )?;

    //
    // To simplify our arithmetic, we'll work with a variable RT
    // that represents the stack top measured relative to the base
    // of the DAS integer array.  At this point, RT is greater than
    // or equal to zero.
    //
    save.RT = (save.T - MEMSIZ);

    if (save.RT < save.LASTI) {
        //
        // Some data can be added by updating DAS addresses.  The
        // available range for updating is B:E, where B and E are
        // calculated below.  This case can occur only when LASTI > 0.
        //
        save.B = (save.RT + 1);
        save.E = intrinsics::MIN0(&[save.LASTI, (save.RT + save.REMAIN)]);

        DASUDI(save.SCRHAN, save.B, save.E, IDATA.subarray(save.START), ctx)?;

        save.NUMADD = ((save.E - save.B) + 1);
        save.START = (save.START + save.NUMADD);
        save.REMAIN = (save.REMAIN - save.NUMADD);
        save.T = (save.T + save.NUMADD);

        if (save.REMAIN == 0) {
            return Ok(());
        }
    }

    //
    // At this point, START and REMAIN are set, and T reflects the
    // amount of data we've pushed so far..  The remaining data
    // must be appended to the scratch DAS file.
    //
    DASADI(save.SCRHAN, save.REMAIN, IDATA.subarray(save.START), ctx)?;

    save.T = (save.T + save.REMAIN);

    Ok(())
}

//$Procedure    ZZEKSPOP  ( EK scratch area, pop data )
pub fn ZZEKSPOP(N: i32, IDATA: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IDATA = DummyArrayMut::new(IDATA, 1..);

    //
    // No checking in here.
    //

    //
    // First time through, open a scratch DAS file.
    //
    if save.FIRST {
        save.FIRST = false;

        DASOPS(&mut save.SCRHAN, ctx)?;

        if FAILED(ctx) {
            return Ok(());
        }
    }

    //
    // You can't pop a negative number of elements.
    //
    if (N < 0) {
        CHKIN(b"ZZEKSPOP", ctx)?;
        SETMSG(
            b"Pop count must be non-negative; call requests popping # elements.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKSPOP", ctx)?;
        return Ok(());

    //
    // It's an error to try to pop more data than we have on the
    // stack.
    //
    } else if (N > save.T) {
        CHKIN(b"ZZEKSPOP", ctx)?;
        SETMSG(b"EK stack pointer = #; call requests popping # items.", ctx);
        ERRINT(b"#", save.T, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKSPOP", ctx)?;
        return Ok(());
    }

    //
    // Read as much data as possible from our big array.
    //
    save.BASE = (save.T - N);

    if (save.BASE < MEMSIZ) {
        save.NUMRD = intrinsics::MIN0(&[N, (MEMSIZ - save.BASE)]);

        for I in 1..=save.NUMRD {
            IDATA[I] = save.SCRTCH[(save.BASE + I)];
        }

        if (save.NUMRD == N) {
            save.T = (save.T - save.NUMRD);
            return Ok(());
        }

        save.REMAIN = (N - save.NUMRD);
        save.BASE = MEMSIZ;
        save.START = (save.NUMRD + 1);
    } else {
        save.REMAIN = N;
        save.START = 1;
    }

    //
    // At this point, REMAIN, START and BASE are set.  If we got this
    // far, we'll need to read the rest of the data from the scratch DAS.
    // Compute the base address to read from relative to the start of
    // the DAS array.
    //
    save.RB = (save.BASE - MEMSIZ);
    save.B = (save.RB + 1);
    save.E = (save.RB + save.REMAIN);

    DASRDI(
        save.SCRHAN,
        save.B,
        save.E,
        IDATA.subarray_mut(save.START),
        ctx,
    )?;

    save.T = (save.T - N);

    Ok(())
}

//$Procedure    ZZEKSDEC  ( EK scratch area, decrement stack pointer )
pub fn ZZEKSDEC(N: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // No checking in here.
    //

    //
    // First time through, open a scratch DAS file.
    //
    if save.FIRST {
        save.FIRST = false;

        DASOPS(&mut save.SCRHAN, ctx)?;

        if FAILED(ctx) {
            return Ok(());
        }
    }

    //
    // Catch non-positive decrement requests.
    //
    if (N < 0) {
        CHKIN(b"ZZEKSDEC", ctx)?;
        SETMSG(
            b"Decrement value must be non-negative; call requests decrement by #.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKSDEC", ctx)?;
        return Ok(());

    //
    // It's an error to try to decrement the pointer by more than
    // the current stack depth.
    //
    } else if (N > save.T) {
        CHKIN(b"ZZEKSDEC", ctx)?;
        SETMSG(b"EK stack pointer = #; call requests  decrement by #.", ctx);
        ERRINT(b"#", save.T, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKSDEC", ctx)?;
        return Ok(());
    }

    save.T = (save.T - N);

    Ok(())
}

//$Procedure    ZZEKSUPD  ( EK scratch area, update )
pub fn ZZEKSUPD(BEG: i32, END: i32, IDATA: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let IDATA = DummyArray::new(IDATA, 1..);

    //
    // No checking in here.
    //

    //
    // Validate the addresses.
    //
    if ((BEG < 1) || (BEG > save.T)) {
        CHKIN(b"ZZEKSUPD", ctx)?;
        SETMSG(b"Start address BEG was #; valid range is 1:#", ctx);
        ERRINT(b"#", BEG, ctx);
        ERRINT(b"#", save.T, ctx);
        SIGERR(b"SPICE(INVALIDADDRESS)", ctx)?;
        CHKOUT(b"ZZEKSUPD", ctx)?;
        return Ok(());
    } else if ((END < 1) || (END > save.T)) {
        CHKIN(b"ZZEKSUPD", ctx)?;
        SETMSG(b"End address END was #; valid range is 1:#", ctx);
        ERRINT(b"#", END, ctx);
        ERRINT(b"#", save.T, ctx);
        SIGERR(b"SPICE(INVALIDADDRESS)", ctx)?;
        CHKOUT(b"ZZEKSUPD", ctx)?;
        return Ok(());
    } else if (BEG > END) {
        return Ok(());
    }

    if (END <= MEMSIZ) {
        //
        // If the entire range is in memory, fine.  Update the range
        // now.
        //
        for I in BEG..=END {
            save.SCRTCH[I] = IDATA[((I - BEG) + 1)];
        }
    } else if (BEG <= MEMSIZ) {
        //
        // Update the portion of the address range that's in memory.
        //
        for I in BEG..=MEMSIZ {
            save.SCRTCH[I] = IDATA[((I - BEG) + 1)];
        }

        //
        // Now update the rest of the range, which is in the scratch
        // DAS file.
        //
        DASUDI(
            save.SCRHAN,
            1,
            (END - MEMSIZ),
            IDATA.subarray(((MEMSIZ - BEG) + 2)),
            ctx,
        )?;
    } else {
        //
        // The whole range is in the DAS file.
        //
        DASUDI(
            save.SCRHAN,
            (BEG - MEMSIZ),
            (END - MEMSIZ),
            IDATA.as_slice(),
            ctx,
        )?;
    }

    Ok(())
}

//$Procedure    ZZEKSRD  ( EK scratch area, read )
pub fn ZZEKSRD(BEG: i32, END: i32, IDATA: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IDATA = DummyArrayMut::new(IDATA, 1..);

    //
    // No checking in here.
    //

    //
    // Validate the addresses.
    //
    if ((BEG < 1) || (BEG > save.T)) {
        CHKIN(b"ZZEKSRD", ctx)?;
        SETMSG(b"Start address BEG was #; valid range is 1:#", ctx);
        ERRINT(b"#", BEG, ctx);
        ERRINT(b"#", save.T, ctx);
        SIGERR(b"SPICE(INVALIDADDRESS)", ctx)?;
        CHKOUT(b"ZZEKSRD", ctx)?;
        return Ok(());
    } else if ((END < 1) || (END > save.T)) {
        CHKIN(b"ZZEKSRD", ctx)?;
        SETMSG(b"End address END was #; valid range is 1:#", ctx);
        ERRINT(b"#", END, ctx);
        ERRINT(b"#", save.T, ctx);
        SIGERR(b"SPICE(INVALIDADDRESS)", ctx)?;
        CHKOUT(b"ZZEKSRD", ctx)?;
        return Ok(());
    } else if (BEG > END) {
        return Ok(());
    }

    if (END <= MEMSIZ) {
        //
        // If the entire range is in memory, fine.  Read from the range
        // now.
        //
        for I in BEG..=END {
            IDATA[((I - BEG) + 1)] = save.SCRTCH[I];
        }
    } else if (BEG <= MEMSIZ) {
        //
        // Read from the portion of the address range that's in memory.
        //
        for I in BEG..=MEMSIZ {
            IDATA[((I - BEG) + 1)] = save.SCRTCH[I];
        }

        //
        // Now read the rest of the range, which is in the scratch
        // DAS file.
        //
        DASRDI(
            save.SCRHAN,
            1,
            (END - MEMSIZ),
            IDATA.subarray_mut(((MEMSIZ - BEG) + 2)),
            ctx,
        )?;
    } else {
        //
        // The whole range is in the DAS file.
        //
        DASRDI(
            save.SCRHAN,
            (BEG - MEMSIZ),
            (END - MEMSIZ),
            IDATA.as_slice_mut(),
            ctx,
        )?;
    }

    Ok(())
}

//$Procedure    ZZEKSCLN  ( EK scratch area, clean up )
pub fn ZZEKSCLN(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // No checking in here.
    //

    //
    // Clean out the stack buffer.
    //
    CLEARI(MEMSIZ, save.SCRTCH.as_slice_mut());
    save.T = 0;

    //
    // If FIRST has been set to .FALSE., we've an open scratch DAS
    // to dispose of.
    //
    if !save.FIRST {
        //
        // Write out the buffered records belonging to the scratch file;
        // this will cause them to be returned to the free list.
        //
        DASWBR(save.SCRHAN, ctx)?;

        //
        // Dump the scratch DAS.
        //
        DASLLC(save.SCRHAN, ctx)?;
    }

    //
    // Tell the system to re-initialize on the next pass.
    //
    save.FIRST = true;

    Ok(())
}
