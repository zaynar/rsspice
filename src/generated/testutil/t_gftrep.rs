//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const WINMAX: i32 = 20000;
const MSGMAX: i32 = 32;
const MXPASS: i32 = 20;
const MAXLOG: i32 = 100000;

struct SaveVars {
    SVMSLG: ActualCharArray2D,
    SVCNFN: ActualArray2D<f64>,
    SVULOG: ActualArray2D<f64>,
    SVFSEQ: StackArray<i32, 20>,
    SVISEQ: StackArray<i32, 20>,
    SVNUPD: i32,
    SVNFIN: i32,
    SVNINI: i32,
    SVSQIX: i32,
    SVUSEQ: ActualArray<i32>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVMSLG = ActualCharArray2D::new(MSGMAX, 1..=2, 1..=MXPASS);
        let mut SVCNFN = ActualArray2D::<f64>::new(LBCELL..=WINMAX, 1..=MXPASS);
        let mut SVULOG = ActualArray2D::<f64>::new(1..=3, 1..=MAXLOG);
        let mut SVFSEQ = StackArray::<i32, 20>::new(1..=MXPASS);
        let mut SVISEQ = StackArray::<i32, 20>::new(1..=MXPASS);
        let mut SVNUPD: i32 = 0;
        let mut SVNFIN: i32 = 0;
        let mut SVNINI: i32 = 0;
        let mut SVSQIX: i32 = 0;
        let mut SVUSEQ = ActualArray::<i32>::new(1..=MAXLOG);

        Self {
            SVMSLG,
            SVCNFN,
            SVULOG,
            SVFSEQ,
            SVISEQ,
            SVNUPD,
            SVNFIN,
            SVNINI,
            SVSQIX,
            SVUSEQ,
        }
    }
}

//$Procedure      T_GFTREP ( GF, progress report test utilities )
pub fn T_GFTREP(
    CNFINE: &[f64],
    MSGPRE: &[u8],
    MSGSUF: &[u8],
    IVBEG: f64,
    IVEND: f64,
    ET: f64,
    NMAX: i32,
    MW: i32,
    NCALLS: i32,
    CNFLOG: &[f64],
    MSGLOG: CharArray,
    REPLOG: &[f64],
    SEQLOG: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //
    //
    // Save everything.
    //

    spicelib::CHKIN(b"T_GFTREP", ctx)?;
    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    spicelib::CHKOUT(b"T_GFTREP", ctx)?;

    Ok(())
}

//$Procedure  T_GFUINI ( GF, progress report test utility init )
pub fn T_GFUINI(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::CHKIN(b"T_GFUINI", ctx)?;

    save.SVNINI = 0;
    save.SVNUPD = 0;
    save.SVNFIN = 0;

    spicelib::CLEARD((3 * MAXLOG), save.SVULOG.as_slice_mut());

    for I in 1..=MXPASS {
        spicelib::SSIZED(WINMAX, save.SVCNFN.subarray_mut([LBCELL, I]), ctx)?;

        fstr::assign(save.SVMSLG.get_mut([1, I]), b" ");
        fstr::assign(save.SVMSLG.get_mut([2, I]), b" ");
    }

    save.SVSQIX = 0;

    spicelib::CLEARI(MXPASS, save.SVISEQ.as_slice_mut());
    spicelib::CLEARI(MAXLOG, save.SVUSEQ.as_slice_mut());
    spicelib::CLEARI(MXPASS, save.SVFSEQ.as_slice_mut());

    spicelib::CHKOUT(b"T_GFUINI", ctx)?;
    Ok(())
}

//$Procedure  T_GFREPI ( GF, GFREPI stand-in )
pub fn T_GFREPI(
    CNFINE: &[f64],
    MSGPRE: &[u8],
    MSGSUF: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CNFINE = DummyArray::new(CNFINE, LBCELL..);

    spicelib::CHKIN(b"T_GFREPI", ctx)?;

    //
    // Increment the progress report initialization count.
    //
    save.SVNINI = (save.SVNINI + 1);

    if (save.SVNINI > MXPASS) {
        spicelib::SETMSG(b"Too many initialization calls: number of progress reporting initialization calls (to T_GRREPI) since setup is #; room in static T_GFTREP buffers is #.", ctx);
        spicelib::ERRINT(b"#", save.SVNINI, ctx);
        spicelib::ERRINT(b"#", MXPASS, ctx);
        spicelib::SIGERR(b"SPICE(OVERFLOW)", ctx)?;
        spicelib::CHKOUT(b"T_GFREPI", ctx)?;
        return Ok(());
    }

    //
    // Increment the call sequence index, and save this
    // index in the initialization sequence log.
    //
    save.SVSQIX = (save.SVSQIX + 1);
    save.SVISEQ[save.SVNINI] = save.SVSQIX;

    //
    // Store the inputs in the initialization log.
    //
    spicelib::COPYD(
        CNFINE.as_slice(),
        save.SVCNFN.subarray_mut([LBCELL, save.SVNINI]),
        ctx,
    )?;

    fstr::assign(save.SVMSLG.get_mut([1, save.SVNINI]), MSGPRE);
    fstr::assign(save.SVMSLG.get_mut([2, save.SVNINI]), MSGSUF);

    spicelib::CHKOUT(b"T_GFREPI", ctx)?;
    Ok(())
}

//$Procedure  T_GFRINI ( GF, return values passed to T_GFREPI )
pub fn T_GFRINI(
    NMAX: i32,
    MW: i32,
    NCALLS: &mut i32,
    SEQLOG: &mut [i32],
    CNFLOG: &mut [f64],
    MSGLOG: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CNFLOG = DummyArrayMut2D::new(CNFLOG, LBCELL..=MW, 1..);
    let mut MSGLOG = DummyCharArrayMut2D::new(MSGLOG, None, 1..=2, 1..);
    let mut SEQLOG = DummyArrayMut::new(SEQLOG, 1..);

    spicelib::CHKIN(b"T_GFRINI", ctx)?;

    //
    // Make sure we have room for the output.
    //
    if (save.SVNINI > NMAX) {
        spicelib::SETMSG(b"Number of progress reporting initialization calls (to T_GRREPI) since setup is #; room in output buffer is #.", ctx);
        spicelib::ERRINT(b"#", save.SVNINI, ctx);
        spicelib::ERRINT(b"#", NMAX, ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_GFRINI", ctx)?;
        return Ok(());
    }

    //
    // Set the outputs:
    //
    //    - Number of calls to T_GFREPI
    //    - Sequence numbers of the calls to T_GFREPI
    //    - Input confinement windows
    //    - Input messages
    //

    *NCALLS = save.SVNINI;

    spicelib::MOVEI(save.SVISEQ.as_slice(), save.SVNINI, SEQLOG.as_slice_mut());

    for I in 1..=save.SVNINI {
        spicelib::COPYD(
            save.SVCNFN.subarray([LBCELL, I]),
            CNFLOG.subarray_mut([LBCELL, I]),
            ctx,
        )?;

        fstr::assign(MSGLOG.get_mut([1, I]), save.SVMSLG.get([1, I]));
        fstr::assign(MSGLOG.get_mut([2, I]), save.SVMSLG.get([2, I]));
    }

    spicelib::CHKOUT(b"T_GFRINI", ctx)?;
    Ok(())
}

//$Procedure  T_GFREPU ( GF, GFREPU stand-in )
pub fn T_GFREPU(IVBEG: f64, IVEND: f64, ET: f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::CHKIN(b"T_GFREPU", ctx)?;

    //
    // Increment the progress report update count.
    //
    save.SVNUPD = (save.SVNUPD + 1);

    if (save.SVNUPD > MAXLOG) {
        spicelib::SETMSG(b"Too many update calls: number of progress reporting update calls (to T_GRREPU) since setup is #; room in static T_GFTREP buffers is #.", ctx);
        spicelib::ERRINT(b"#", save.SVNUPD, ctx);
        spicelib::ERRINT(b"#", MAXLOG, ctx);
        spicelib::SIGERR(b"SPICE(OVERFLOW)", ctx)?;
        spicelib::CHKOUT(b"T_GFREPU", ctx)?;
        return Ok(());
    }

    //
    // Increment the call sequence index, and save this
    // index in the update sequence log.
    //
    save.SVSQIX = (save.SVSQIX + 1);

    save.SVUSEQ[save.SVNUPD] = save.SVSQIX;

    //
    // Store the inputs in the update log.
    //
    save.SVULOG[[1, save.SVNUPD]] = IVBEG;
    save.SVULOG[[2, save.SVNUPD]] = IVEND;
    save.SVULOG[[3, save.SVNUPD]] = ET;

    spicelib::CHKOUT(b"T_GFREPU", ctx)?;
    Ok(())
}

//$Procedure  T_GFRPLO ( GF, return values passed to T_GFREPU )
pub fn T_GFRPLO(
    NMAX: i32,
    NCALLS: &mut i32,
    SEQLOG: &mut [i32],
    REPLOG: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut REPLOG = DummyArrayMut2D::new(REPLOG, 1..=3, 1..);
    let mut SEQLOG = DummyArrayMut::new(SEQLOG, 1..);

    spicelib::CHKIN(b"T_GFRPLO", ctx)?;

    //
    // Make sure we have room for the output.
    //
    if (save.SVNUPD > NMAX) {
        spicelib::SETMSG(b"Number of progress reporting update calls (to T_GRREPU) since setup is #; room in buffer is #.", ctx);
        spicelib::ERRINT(b"#", save.SVNUPD, ctx);
        spicelib::ERRINT(b"#", NMAX, ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_GFRPLO", ctx)?;
        return Ok(());
    }

    //
    // Set the outputs:
    //
    //    - Number of calls to T_GFREPU
    //
    //    - Sequence numbers of the calls to T_GFREPU
    //
    //    - Update log, which contains
    //
    //        > interval begin times
    //        > interval end times
    //        > ET values
    //

    *NCALLS = save.SVNUPD;

    spicelib::MOVEI(save.SVUSEQ.as_slice(), save.SVNUPD, SEQLOG.as_slice_mut());

    for I in 1..=save.SVNUPD {
        REPLOG[[1, I]] = save.SVULOG[[1, I]];
        REPLOG[[2, I]] = save.SVULOG[[2, I]];
        REPLOG[[3, I]] = save.SVULOG[[3, I]];
    }

    spicelib::CHKOUT(b"T_GFRPLO", ctx)?;
    Ok(())
}

//$Procedure  T_GFREPF ( GF, GFREPF stand-in )
pub fn T_GFREPF(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::CHKIN(b"T_GFREPF", ctx)?;

    //
    // Increment the progress report termination count.
    //
    save.SVNFIN = (save.SVNFIN + 1);

    if (save.SVNFIN > MAXLOG) {
        spicelib::SETMSG(b"Too many termination calls: number of progress reporting termination calls (to T_GRREPF) since setup is #; room in static T_GFTREP buffers is #.", ctx);
        spicelib::ERRINT(b"#", save.SVNFIN, ctx);
        spicelib::ERRINT(b"#", MAXLOG, ctx);
        spicelib::SIGERR(b"SPICE(OVERFLOW)", ctx)?;
        spicelib::CHKOUT(b"T_GFREPF", ctx)?;
        return Ok(());
    }

    //
    // Increment the call sequence index, and save this
    // index in the termination sequence log.
    //
    save.SVSQIX = (save.SVSQIX + 1);
    save.SVFSEQ[save.SVNFIN] = save.SVSQIX;

    spicelib::CHKOUT(b"T_GFREPF", ctx)?;
    Ok(())
}

//$Procedure  T_GFRTRM ( GF, return values passed to T_GFREPU )
pub fn T_GFRTRM(
    NMAX: i32,
    NCALLS: &mut i32,
    SEQLOG: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SEQLOG = DummyArrayMut::new(SEQLOG, 1..);

    spicelib::CHKIN(b"T_GFRTRM", ctx)?;

    //
    // Make sure we have room for the output.
    //
    if (save.SVNFIN > NMAX) {
        spicelib::SETMSG(b"Number of progress reporting termination calls (to T_GRREPF) since setup is #; room in buffer is #.", ctx);
        spicelib::ERRINT(b"#", save.SVNFIN, ctx);
        spicelib::ERRINT(b"#", NMAX, ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_GFRTRM", ctx)?;
        return Ok(());
    }

    //
    // Set the outputs:
    //
    //    - Number of calls to T_GFRFIN
    //
    //    - Sequence numbers of the calls to T_GFRFIN
    //

    *NCALLS = save.SVNFIN;

    spicelib::MOVEI(save.SVFSEQ.as_slice(), save.SVNFIN, SEQLOG.as_slice_mut());

    spicelib::CHKOUT(b"T_GFRTRM", ctx)?;
    Ok(())
}
