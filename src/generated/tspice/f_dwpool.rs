//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const KVNMLN: i32 = 32;
const MAXVAR: i32 = 26003;
const MAXVAL: i32 = 400000;
const MAXAGT: i32 = 1000;
const MXNOTE: i32 = (MAXVAR * 5);
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const LBPOOL: i32 = -5;

struct SaveVars {
    AGENT: Vec<u8>,
    AGENTS: ActualCharArray,
    AGTSET: ActualCharArray,
    OKVSET: ActualCharArray,
    QNAME: Vec<u8>,
    VARS: ActualCharArray,
    UWAGNT: ActualCharArray,
    UWVARS: ActualCharArray,
    XAGSET: ActualCharArray,
    DPVALS: ActualArray<f64>,
    J: i32,
    N: i32,
    NAGENT: i32,
    NAVAIL: i32,
    NSET: i32,
    NVARS: i32,
    NUSED: i32,
    ONAGNT: i32,
    ONVARS: i32,
    ORDVEC: ActualArray<i32>,
    UWPOOL: ActualArray2D<i32>,
    UWPTRS: ActualArray<i32>,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut AGENT = vec![b' '; KVNMLN as usize];
        let mut AGENTS = ActualCharArray::new(KVNMLN, 1..=MXNOTE);
        let mut AGTSET = ActualCharArray::new(KVNMLN, LBCELL..=MXNOTE);
        let mut OKVSET = ActualCharArray::new(KVNMLN, LBCELL..=MXNOTE);
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut VARS = ActualCharArray::new(KVNMLN, 1..=MAXVAR);
        let mut UWAGNT = ActualCharArray::new(KVNMLN, 1..=MXNOTE);
        let mut UWVARS = ActualCharArray::new(KVNMLN, LBCELL..=MAXVAR);
        let mut XAGSET = ActualCharArray::new(KVNMLN, LBCELL..=MXNOTE);
        let mut DPVALS = ActualArray::<f64>::new(1..=MAXVAL);
        let mut J: i32 = 0;
        let mut N: i32 = 0;
        let mut NAGENT: i32 = 0;
        let mut NAVAIL: i32 = 0;
        let mut NSET: i32 = 0;
        let mut NVARS: i32 = 0;
        let mut NUSED: i32 = 0;
        let mut ONAGNT: i32 = 0;
        let mut ONVARS: i32 = 0;
        let mut ORDVEC = ActualArray::<i32>::new(1..=MXNOTE);
        let mut UWPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=MXNOTE);
        let mut UWPTRS = ActualArray::<i32>::new(1..=MAXVAR);
        let mut UPDATE: bool = false;

        Self {
            AGENT,
            AGENTS,
            AGTSET,
            OKVSET,
            QNAME,
            VARS,
            UWAGNT,
            UWVARS,
            XAGSET,
            DPVALS,
            J,
            N,
            NAGENT,
            NAVAIL,
            NSET,
            NVARS,
            NUSED,
            ONAGNT,
            ONVARS,
            ORDVEC,
            UWPOOL,
            UWPTRS,
            UPDATE,
        }
    }
}

//$Procedure      F_DWPOOL ( DWPOOL tests )
pub fn F_DWPOOL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_DWPOOL", ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Initializations", ctx)?;

    spicelib::SSIZEC(MAXVAR, save.UWVARS.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARI(MAXVAR, save.UWPTRS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LNKINI(MXNOTE, save.UWPOOL.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CLEARC(MAXVAR, save.UWAGNT.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEC(MXNOTE, save.AGTSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEC(MXNOTE, save.OKVSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEC(MXNOTE, save.XAGSET.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Find out what's already in the watcher system.
    // Save the original set of watched variables in
    // OKVSET.
    //
    spicelib::ZZVUPOOL(
        save.OKVSET.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ONVARS = spicelib::CARDC(save.OKVSET.as_arg(), ctx)?;

    //
    // It's not helpful to save the original agent pool, but
    // we will save the number of entries.
    //
    save.ONAGNT = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Basic SWPOOL/CVPOOL checks: 1 agent, 1 variable, variable exists in pool.",
        ctx,
    )?;

    fstr::assign(&mut save.AGENT, b"AGENT1");
    save.NVARS = 1;
    fstr::assign(save.VARS.get_mut(1), b"VAR1");
    save.DPVALS[1] = 999.0;

    spicelib::PDPOOL(&save.VARS[1], 1, save.DPVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SWPOOL(&save.AGENT, save.NVARS, save.VARS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect the agent to receive an update
    // notice as soon as the watch is set.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 0", save.UPDATE, true, OK, ctx)?;

    //
    // Now the next check should show no update.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 1", save.UPDATE, false, OK, ctx)?;

    //
    // Clear the kernel pool; an update should be
    // indicated.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 2", save.UPDATE, true, OK, ctx)?;

    //
    // Now the next check should show no update.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 1", save.UPDATE, false, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Basic SWPOOL/CVPOOL checks: 1 agent, 1 variable, variable does not exist in pool.",
        ctx,
    )?;

    fstr::assign(&mut save.AGENT, b"AGENT2");
    save.NVARS = 1;
    fstr::assign(save.VARS.get_mut(1), b"VAR2");

    spicelib::SWPOOL(&save.AGENT, save.NVARS, save.VARS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect the agent to receive an update
    // notice as soon as the watch is set.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 0", save.UPDATE, true, OK, ctx)?;

    //
    // Now the next check should show no update.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 1", save.UPDATE, false, OK, ctx)?;

    //
    // Clear the kernel pool; an update should be
    // indicated.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 3", save.UPDATE, true, OK, ctx)?;

    //
    // Now the next check should show no update.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 4", save.UPDATE, false, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Set one watch and delete it; variable exists.", ctx)?;

    fstr::assign(&mut save.AGENT, b"AGENT3");
    save.NVARS = 1;
    fstr::assign(save.VARS.get_mut(1), b"VAR3");
    save.DPVALS[1] = -999.0;

    spicelib::PDPOOL(&save.VARS[1], 1, save.DPVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SWPOOL(&save.AGENT, save.NVARS, save.VARS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect the agent to receive an update
    // notice as soon as the watch is set. Clear
    // the update notice.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the watch.
    //
    spicelib::DWPOOL(&save.AGENT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Clear the kernel pool; an update should be
    // indicated if the watch is still set. We
    // expect that the watch isn't set.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now the next check should show no update.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"Update 0", save.UPDATE, false, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Delete the watches from the previous tests.", ctx)?;

    spicelib::CVPOOL(b"AGENT1", &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DWPOOL(b"AGENT1", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CVPOOL(b"AGENT2", &mut save.UPDATE, ctx)?;
    spicelib::DWPOOL(b"AGENT2", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CVPOOL(b"AGENT3", &mut save.UPDATE, ctx)?;
    spicelib::DWPOOL(b"AGENT3", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Set multiple watches on one variable; variable does not exist in the kernel pool.",
        ctx,
    )?;

    fstr::assign(save.VARS.get_mut(1), b"VAR1");

    save.NAGENT = 100;

    for I in 1..=save.NAGENT {
        fstr::assign(save.AGENTS.get_mut(I), b"AGENT#");
        spicelib::REPMI(&save.AGENTS[I].to_vec(), b"#", I, &mut save.AGENTS[I], ctx);

        spicelib::SWPOOL(&save.AGENTS[I], 1, save.VARS.subarray(1), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Since these watches are new, each agent should be notified
    // of an update. Verify this.
    //
    for I in 1..=save.NAGENT {
        fstr::assign(&mut save.QNAME, b"(0) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(1) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Examine the watcher system; see whether we have the
    // expected contents.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the agents for VAR(1).
    //
    spicelib::SSIZEC(MXNOTE, save.AGTSET.as_arg_mut(), ctx)?;

    spicelib::ZZGAPOOL(
        &save.VARS[1],
        save.UWVARS.as_arg(),
        save.UWPTRS.as_slice(),
        save.UWPOOL.as_slice(),
        save.UWAGNT.as_arg(),
        save.AGTSET.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We must sort the expected agent array before the comparison.
    //
    spicelib::MOVEC(
        save.AGENTS.as_arg(),
        save.NAGENT,
        save.XAGSET.subarray_mut(1),
    );

    spicelib::VALIDC(MXNOTE, save.NAGENT, save.XAGSET.as_arg_mut(), ctx)?;

    testutil::CHCKAC(
        b"AGTSET",
        save.AGTSET.subarray(1),
        b"=",
        save.XAGSET.subarray(1),
        save.NAGENT,
        OK,
        ctx,
    )?;

    //
    // The order of deletion can be important, so we're going to
    // delete the watches in:
    //
    //     - order of insertion
    //     - reverse order of insertion
    //     - alphabetically sorted order
    //
    // We'll re-set the watches in the original order each time we
    // re-set them.
    //
    // Delete watches in order of insertion.
    //
    for I in 1..=save.NAGENT {
        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (0)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set watches in the original order.
    //
    for I in 1..=save.NAGENT {
        spicelib::SWPOOL(&save.AGENTS[I], 1, save.VARS.subarray(1), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.QNAME, b"(2) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(3) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Delete watches in the reverse of the order of insertion.
    //
    for I in intrinsics::range(save.NAGENT, 1, -1) {
        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set watches in the original order.
    //
    for I in 1..=save.NAGENT {
        spicelib::SWPOOL(&save.AGENTS[I], 1, save.VARS.subarray(1), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.QNAME, b"(4) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(5) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Delete watches in the reverse of the order of insertion.
    //
    for I in intrinsics::range(save.NAGENT, 1, -1) {
        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (2)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set watches in the original order.
    //
    for I in 1..=save.NAGENT {
        spicelib::SWPOOL(&save.AGENTS[I], 1, save.VARS.subarray(1), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.QNAME, b"(6) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(7) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Delete watches in sorted order.
    //
    spicelib::ORDERC(
        save.AGENTS.as_arg(),
        save.NAGENT,
        save.ORDVEC.as_slice_mut(),
    );

    for I in 1..=save.NAGENT {
        save.J = save.ORDVEC[I];

        spicelib::DWPOOL(&save.AGENTS[save.J], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (3)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Set multiple watches on multiple variables; variables do not exist in the kernel pool. Each agent watches the same variables.", ctx)?;

    save.NVARS = 19;
    save.NAGENT = 11;

    for I in 1..=save.NVARS {
        fstr::assign(save.VARS.get_mut(I), b"VARS#");
        spicelib::REPMI(&save.VARS[I].to_vec(), b"#", I, &mut save.VARS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.NAGENT {
        fstr::assign(save.AGENTS.get_mut(I), b"AGENT#");
        spicelib::REPMI(&save.AGENTS[I].to_vec(), b"#", I, &mut save.AGENTS[I], ctx);

        spicelib::SWPOOL(&save.AGENTS[I], save.NVARS, save.VARS.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Since these watches are new, each agent should be notified
    // of an update. Verify this.
    //
    for I in 1..=save.NAGENT {
        fstr::assign(&mut save.QNAME, b"(0) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(1) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Examine the watcher system; see whether we have the
    // expected contents.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We must sort the expected agent array before the comparison.
    //
    spicelib::MOVEC(
        save.AGENTS.as_arg(),
        save.NAGENT,
        save.XAGSET.subarray_mut(1),
    );

    spicelib::VALIDC(MXNOTE, save.NAGENT, save.XAGSET.as_arg_mut(), ctx)?;

    //
    // Get the agents for each kernel variable; compare these to
    // our sorted agent set.
    //
    for I in 1..=save.NVARS {
        spicelib::ZZGAPOOL(
            &save.VARS[I],
            save.UWVARS.as_arg(),
            save.UWPTRS.as_slice(),
            save.UWPOOL.as_slice(),
            save.UWAGNT.as_arg(),
            save.AGTSET.as_arg_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAC(
            b"AGTSET",
            save.AGTSET.subarray(1),
            b"=",
            save.XAGSET.subarray(1),
            save.NAGENT,
            OK,
            ctx,
        )?;
    }

    //
    // Delete watches in order of insertion.
    //
    for I in 1..=save.NAGENT {
        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (0)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set the watches.
    //
    for I in 1..=save.NAGENT {
        spicelib::SWPOOL(&save.AGENTS[I], save.NVARS, save.VARS.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.QNAME, b"(2) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(3) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Delete watches in the reverse of the order of insertion.
    //
    for I in intrinsics::range(save.NAGENT, 1, -1) {
        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set the watches.
    //
    for I in 1..=save.NAGENT {
        spicelib::SWPOOL(&save.AGENTS[I], save.NVARS, save.VARS.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.QNAME, b"(4) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(5) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Delete watches in sorted order.
    //
    spicelib::ORDERC(
        save.AGENTS.as_arg(),
        save.NAGENT,
        save.ORDVEC.as_slice_mut(),
    );

    for I in 1..=save.NAGENT {
        save.J = save.ORDVEC[I];

        spicelib::DWPOOL(&save.AGENTS[save.J], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (3)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Set multiple watches on multiple variables; variables do not exist in the kernel pool. Each agent watches a different set of variables.", ctx)?;

    //
    // The sets of watched variables are disjoint in this test case.
    // Each variable is watched by only one agent.
    //
    save.NVARS = 31;
    save.NAGENT = 21;

    for I in 1..=save.NAGENT {
        fstr::assign(save.AGENTS.get_mut(I), b"AGENT#");
        spicelib::REPMI(&save.AGENTS[I].to_vec(), b"#", I, &mut save.AGENTS[I], ctx);

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        spicelib::SWPOOL(&save.AGENTS[I], save.NVARS, save.VARS.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Since these watches are new, each agent should be notified
    // of an update. Verify this.
    //
    for I in 1..=save.NAGENT {
        fstr::assign(&mut save.QNAME, b"(0) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(1) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Examine the watcher system; see whether we have the
    // expected contents.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the agent (note: singular) for each kernel variable.
    //
    for I in 1..=save.NAGENT {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Re-create the name VARS(J).
                //
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Get the current watch state.
                //
                spicelib::ZZGAPOOL(
                    &save.VARS[save.J],
                    save.UWVARS.as_arg(),
                    save.UWPTRS.as_slice(),
                    save.UWPOOL.as_slice(),
                    save.UWAGNT.as_arg(),
                    save.AGTSET.as_arg_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"CARDC(AGTSET) (#,#)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.QNAME,
                    spicelib::CARDC(save.AGTSET.as_arg(), ctx)?,
                    b"=",
                    1,
                    0,
                    OK,
                    ctx,
                )?;

                testutil::CHCKSC(
                    b"AGTSET (0)",
                    &save.AGTSET[1],
                    b"=",
                    &save.AGENTS[I],
                    OK,
                    ctx,
                )?;

                save.J += m3__;
            }
        }
    }

    //
    // Delete watches in order of insertion.
    //
    for I in 1..=save.NAGENT {
        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (0)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set the watches.
    //
    for I in 1..=save.NAGENT {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        spicelib::SWPOOL(&save.AGENTS[I], save.NVARS, save.VARS.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.QNAME, b"(2) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(3) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Delete watches in the reverse of the order of insertion.
    //
    for I in intrinsics::range(save.NAGENT, 1, -1) {
        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set the watches.
    //
    for I in 1..=save.NAGENT {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        spicelib::SWPOOL(&save.AGENTS[I], save.NVARS, save.VARS.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.QNAME, b"(4) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

        //
        // Make sure a second test indicates "no update."
        //
        fstr::assign(&mut save.QNAME, b"(5) Update #");
        spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);

        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;
    }

    //
    // Delete watches in sorted order.
    //
    spicelib::ORDERC(
        save.AGENTS.as_arg(),
        save.NAGENT,
        save.ORDVEC.as_slice_mut(),
    );

    for I in 1..=save.NAGENT {
        save.J = save.ORDVEC[I];

        spicelib::DWPOOL(&save.AGENTS[save.J], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (2)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Set multiple watches on multiple variables; variables do not exist in the kernel pool. Variables are watched by multiple agents.", ctx)?;

    //
    // The sets of watched variables are disjoint in this test case.
    // Each variable is watched by multiple agents.
    //
    save.NSET = 5;
    save.NVARS = 21;
    save.NAGENT = 11;

    for I in 1..=save.NSET {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each variable name includes I and J.
                //
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each agent name includes I and J.
                //
                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Set a watch for the Jth agent on the Ith
                // set of variables.
                //
                spicelib::SWPOOL(&save.AGENTS[save.J], save.NVARS, save.VARS.as_arg(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }
    }

    //
    // Since these watches are new, each agent should be notified
    // of an update. Verify this.
    //
    for I in 1..=save.NSET {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut save.QNAME, b"(0) Update # #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CVPOOL(&save.AGENTS[save.J], &mut save.UPDATE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

                //
                // Make sure a second test indicates "no update."
                //
                fstr::assign(&mut save.QNAME, b"(1) Update # #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CVPOOL(&save.AGENTS[save.J], &mut save.UPDATE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;

                save.J += m3__;
            }
        }
    }

    //
    // Examine the watcher system; see whether we have the
    // expected contents.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the agent (note: singular) for each kernel variable.
    //
    for I in 1..=save.NSET {
        //
        // Re-create the agent and variable names.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each variable name includes I and J.
                //
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Create an agent set while we're creating agent names.
        //
        spicelib::SCARDC(0, save.XAGSET.as_arg_mut(), ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each agent name includes I and J.
                //
                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::INSRTC(&save.AGENTS[save.J], save.XAGSET.as_arg_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Get the current watch state.
                //
                spicelib::ZZGAPOOL(
                    &save.VARS[save.J],
                    save.UWVARS.as_arg(),
                    save.UWPTRS.as_slice(),
                    save.UWPOOL.as_slice(),
                    save.UWAGNT.as_arg(),
                    save.AGTSET.as_arg_mut(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(&mut save.QNAME, b"CARDC(AGTSET) (#,#)");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.QNAME,
                    spicelib::CARDC(save.AGTSET.as_arg(), ctx)?,
                    b"=",
                    save.NAGENT,
                    0,
                    OK,
                    ctx,
                )?;

                testutil::CHCKAC(
                    b"AGTSET (0)",
                    save.AGTSET.subarray(1),
                    b"=",
                    save.XAGSET.subarray(1),
                    save.NAGENT,
                    OK,
                    ctx,
                )?;

                save.J += m3__;
            }
        }
    }

    //
    // Delete watches in order of insertion.
    //
    for I in 1..=save.NSET {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::DWPOOL(&save.AGENTS[save.J], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }
    }

    //
    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (0)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set the watches.
    //
    for I in 1..=save.NSET {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each variable name includes I and J.
                //
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each agent name includes I and J.
                //
                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Set a watch for the Jth agent on the Ith
                // set of variables.
                //
                spicelib::SWPOOL(&save.AGENTS[save.J], save.NVARS, save.VARS.as_arg(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Check update status.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut save.QNAME, b"(2) Update # #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CVPOOL(&save.AGENTS[save.J], &mut save.UPDATE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

                //
                // Make sure a second test indicates "no update."
                //
                fstr::assign(&mut save.QNAME, b"(3) Update # #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CVPOOL(&save.AGENTS[save.J], &mut save.UPDATE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;

                save.J += m3__;
            }
        }
    }

    //
    // Delete watches in the reverse of the order of insertion.
    //
    for I in intrinsics::range(save.NSET, 1, -1) {
        {
            let m1__: i32 = save.NAGENT;
            let m2__: i32 = 1;
            let m3__: i32 = -1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::DWPOOL(&save.AGENTS[save.J], ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }
    }

    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    // Re-set the watches.
    //
    // This time, create a set of all agents.
    //
    spicelib::SCARDC(0, save.AGTSET.as_arg_mut(), ctx)?;

    for I in 1..=save.NSET {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NVARS;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each variable name includes I and J.
                //
                fstr::assign(save.VARS.get_mut(save.J), b"VARS#_#");
                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.VARS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.VARS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Each agent name includes I and J.
                //
                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                //
                // Set a watch for the Jth agent on the Ith
                // set of variables.
                //
                spicelib::SWPOOL(&save.AGENTS[save.J], save.NVARS, save.VARS.as_arg(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Add this agent to the agent set.
                //
                spicelib::INSRTC(&save.AGENTS[save.J], save.AGTSET.as_arg_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.J += m3__;
            }
        }

        //
        // Check update status.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NAGENT;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut save.QNAME, b"(4) Update # #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                fstr::assign(save.AGENTS.get_mut(save.J), b"AGENTS#_#");
                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    I,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::REPMI(
                    &save.AGENTS[save.J].to_vec(),
                    b"#",
                    save.J,
                    &mut save.AGENTS[save.J],
                    ctx,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CVPOOL(&save.AGENTS[save.J], &mut save.UPDATE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.QNAME, save.UPDATE, true, OK, ctx)?;

                //
                // Make sure a second test indicates "no update."
                //
                fstr::assign(&mut save.QNAME, b"(5) Update # #");
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::REPMI(&save.QNAME.to_vec(), b"#", save.J, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::CVPOOL(&save.AGENTS[save.J], &mut save.UPDATE, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.QNAME, save.UPDATE, false, OK, ctx)?;

                save.J += m3__;
            }
        }
    }

    //
    // Delete watches in sorted order.
    //
    for I in 1..=spicelib::CARDC(save.AGTSET.as_arg(), ctx)? {
        spicelib::DWPOOL(&save.AGTSET[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    // Check the watch system status.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Try to delete watch with update pending", ctx)?;

    fstr::assign(&mut save.AGENT, b"AGENT1");
    save.NVARS = 1;
    fstr::assign(save.VARS.get_mut(1), b"VAR1");
    save.DPVALS[1] = -999.0;

    spicelib::PDPOOL(&save.VARS[1], 1, save.DPVALS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SWPOOL(&save.AGENT, save.NVARS, save.VARS.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Delete the watch.
    //
    spicelib::DWPOOL(&save.AGENT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(UPDATEPENDING)", OK, ctx)?;

    //
    // Clean up the watch.
    //
    spicelib::CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DWPOOL(&save.AGENT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Determine the number of watches set outside of this test family.",
        ctx,
    )?;

    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NUSED = spicelib::CARDC(save.UWVARS.as_arg(), ctx)?;
    save.NAVAIL = (MAXVAR - save.NUSED);

    //
    // Make sure we have enough watches available so the rest
    // of our tests make sense. The exact number of available
    // slots in the watched variable set depends not only on
    // the Toolkit version but the position of this test family
    // in the sequence of TSPICE tests.
    //

    testutil::CHCKSI(b"NAVAIL", save.NAVAIL, b">", 4000, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Kernel variable set overflow: large variable set", ctx)?;

    for I in 1..=(save.NAVAIL + 1) {
        fstr::assign(save.VARS.get_mut(I), b"VAR(#)");
        spicelib::REPMI(&save.VARS[I].to_vec(), b"#", I, &mut save.VARS[I], ctx);
    }

    spicelib::SWPOOL(b"AGENT1", (save.NAVAIL + 1), save.VARS.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERVARSETOVERFLOW)", OK, ctx)?;

    //
    // We expect that no new watch was set. Check the watch
    // system.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Kernel variable set overflow: single variable", ctx)?;

    for I in 1..=save.NAVAIL {
        fstr::assign(save.VARS.get_mut(I), b"VAR(#)");
        spicelib::REPMI(&save.VARS[I].to_vec(), b"#", I, &mut save.VARS[I], ctx);

        spicelib::SWPOOL(b"AGENT1", 1, save.VARS.subarray(I), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // The last straw.
    //
    spicelib::SWPOOL(b"AGENT1", 1, CharArray::from_ref(b"last straw"), ctx)?;
    testutil::CHCKXC(true, b"SPICE(KERVARSETOVERFLOW)", OK, ctx)?;

    //
    // Clean up.
    //
    spicelib::CVPOOL(b"AGENT1", &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DWPOOL(b"AGENT1", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the watch system.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Too many agents for one variable", ctx)?;

    for I in 1..=MAXAGT {
        fstr::assign(save.AGENTS.get_mut(I), b"AGENTS(#)");
        spicelib::REPMI(&save.AGENTS[I].to_vec(), b"#", I, &mut save.AGENTS[I], ctx);

        spicelib::SWPOOL(&save.AGENTS[I], 1, CharArray::from_ref(b"VAR1"), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // The last straw.
    //
    spicelib::SWPOOL(b"last straw", 1, CharArray::from_ref(b"VAR1"), ctx)?;
    testutil::CHCKXC(true, b"SPICE(TOOMANYWATCHES)", OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=MAXAGT {
        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Agent pool overflow", ctx)?;

    //
    // Start out by finding out how much room is in
    // the agent list.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.N = (spicelib::LNKNFN(save.UWPOOL.as_slice()) / MAXAGT);

    for I in 1..=MAXAGT {
        fstr::assign(save.VARS.get_mut(I), b"VARS(#)");
        spicelib::REPMI(&save.VARS[I].to_vec(), b"#", I, &mut save.VARS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    for I in 1..=save.N {
        fstr::assign(save.AGENTS.get_mut(I), b"AGENTS(#)");
        spicelib::REPMI(&save.AGENTS[I].to_vec(), b"#", I, &mut save.AGENTS[I], ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SWPOOL(&save.AGENTS[I], MAXAGT, save.VARS.as_arg(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // The last straw.
    //
    spicelib::SWPOOL(b"last straw", MAXAGT, save.VARS.as_arg(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(AGENTLISTOVERFLOW)", OK, ctx)?;

    //
    // Clean up.
    //
    for I in 1..=save.N {
        spicelib::CVPOOL(&save.AGENTS[I], &mut save.UPDATE, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DWPOOL(&save.AGENTS[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Check the watch system.
    //
    spicelib::ZZVUPOOL(
        save.UWVARS.as_arg_mut(),
        save.UWPTRS.as_slice_mut(),
        save.UWPOOL.as_slice_mut(),
        save.UWAGNT.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAC(
        b"UWVARS (1)",
        save.UWVARS.subarray(1),
        b"=",
        save.OKVSET.subarray(1),
        save.ONVARS,
        OK,
        ctx,
    )?;

    save.N = (MXNOTE - spicelib::LNKNFN(save.UWPOOL.as_slice()));

    testutil::CHCKSI(b"N", save.N, b"=", save.ONAGNT, 0, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
