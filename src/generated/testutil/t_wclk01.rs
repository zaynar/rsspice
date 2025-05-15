//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NDELIM: i32 = 5;
const DELIMS: &[u8; NDELIM as usize] = &fstr::extend_const::<{ NDELIM as usize }>(b".:-, ");
const MXPART: i32 = 9999;
const PARTLN: i32 = 5;
const MXCOEF: i32 = 100000;
const MXNFLD: i32 = 10;
const DPLEN: i32 = 30;
const MXTSYS: i32 = 2;
const LBCELL: i32 = -5;
const MSGLEN: i32 = 240;
const LNSIZE: i32 = 80;
const LNGSIZ: i32 = 200;

struct SaveVars {
    DLCHRS: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DLCHRS = vec![b' '; NDELIM as usize];

        fstr::assign(&mut DLCHRS, DELIMS);

        Self { DLCHRS }
    }
}

//$Procedure T_WCLK01 ( Write a type 1 SCLK kernel )
pub fn T_WCLK01(
    FNAME: &[u8],
    COMS: CharArray,
    CLKID: i32,
    KERID: &[u8],
    TIMSYS: &[u8],
    NFIELD: i32,
    MODULI: &[f64],
    OFFSET: &[f64],
    DELIM: &[u8],
    NPARTS: i32,
    PSTART: &[f64],
    PSTOP: &[f64],
    NCOEFF: i32,
    COEFFS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let COMS = DummyCharArray::new(COMS, None, LBCELL..);
    let MODULI = DummyArray::new(MODULI, 1..=NFIELD);
    let OFFSET = DummyArray::new(OFFSET, 1..=NFIELD);
    let PSTART = DummyArray::new(PSTART, 1..=NPARTS);
    let PSTOP = DummyArray::new(PSTOP, 1..=NPARTS);
    let COEFFS = DummyArray2D::new(COEFFS, 1..=3, 1..=NCOEFF);
    let mut ERRMSG = [b' '; MSGLEN as usize];
    let mut LINE = [b' '; LNSIZE as usize];
    let mut LONGLN = [b' '; LNGSIZ as usize];
    let mut NUMSTR = [b' '; LNSIZE as usize];
    let mut SP2000: f64 = 0.0;
    let mut PRODM: f64 = 0.0;
    let mut I: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut J: i32 = 0;
    let mut L: i32 = 0;
    let mut LNB: i32 = 0;
    let mut START: i32 = 0;
    let mut STRID: i32 = 0;
    let mut TIMCDE: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Test utility functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //
    //
    // We use a longer, non-standard length for ABCORR because we
    // want to include embedded blanks for testing.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_WCLK01", ctx)?;

    //
    // The clock ID to use in strings is the negative of
    // the input ID.
    //
    STRID = -CLKID;

    //
    // Don't overwrite an existing file.
    //
    if spicelib::EXISTS(FNAME, ctx)? {
        spicelib::SETMSG(b"File # already exists.", ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::SIGERR(b"SPICE(FILEEXISTS)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    //
    // Open the SCLK kernel as a new text file.
    //
    spicelib::TXTOPN(FNAME, &mut UNIT, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    //
    // Write the SCLK ID word.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;
    spicelib::WRITLN(b"KPL/SCLK", UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Write any initial comments that may have been
    // supplied.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(COMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::WRITLN(&COMS[I], UNIT, ctx)?;
            I += m3__;
        }
    }

    //
    // Write the "begindata" marker.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;
    let mut arg0 = vec![b' '; 80];
    BEGDAT(&mut arg0);
    spicelib::WRITLN(&arg0, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Write the kernel ID. The kernel ID must be a date
    // of the form
    //
    //    @<date>
    //
    // where <date contains no embedded blanks.
    //
    spicelib::LJUST(KERID, &mut LINE);
    L = spicelib::LASTNB(&LINE);

    if (L == 0) {
        spicelib::SETMSG(b"Kernel ID written to file # must be non-blank.", ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDIDSTRING)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    if (intrinsics::INDEX(fstr::substr(&LINE, 1..=L), b" ") != 0) {
        spicelib::SETMSG(
            b"Kernel ID <#> written to file # must contain no embedded blanks",
            ctx,
        );
        spicelib::ERRCH(b"#", KERID, ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDIDSTRING)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    if fstr::ne(fstr::substr(&LINE, 1..=1), b"@") {
        spicelib::SETMSG(b"Kernel ID <#> written to file # must be of the form @<date> where date contains no embedded blanks.", ctx);
        spicelib::ERRCH(b"#", KERID, ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDIDSTRING)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    if (L < 9) {
        spicelib::SETMSG(b"Kernel ID <#> written to file # must be of the form @<date> where date contains no embedded blanks.", ctx);
        spicelib::ERRCH(b"#", KERID, ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDIDSTRING)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    spicelib::TPARSE(fstr::substr(&LINE, 2..=L), &mut SP2000, &mut ERRMSG, ctx)?;

    if fstr::ne(&ERRMSG, b" ") {
        spicelib::SETMSG(b"Kernel ID <#> written to file # must be of the form @<date> where date contains no embedded blanks. TPARSE returned message < # > ", ctx);
        spicelib::ERRCH(b"#", KERID, ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRCH(b"#", &ERRMSG, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDIDSTRING)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut LINE, b"   SCLK_KERNEL_ID                  = ( # )");
    spicelib::REPMC(&LINE.clone(), b"#", KERID, &mut LINE);

    spicelib::WRITLN(&LINE, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Write the SCLK data type: for this routine, the value
    // is always 1.
    //
    fstr::assign(&mut LINE, b"   SCLK_DATA_TYPE_#              = ( 1 )");
    spicelib::REPMI(&LINE.clone(), b"#", STRID, &mut LINE, ctx);

    spicelib::WRITLN(&LINE, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Write the time system code.
    //
    if spicelib::EQSTR(TIMSYS, b"TDB") {
        TIMCDE = 1;
    } else if spicelib::EQSTR(TIMSYS, b"TDT") {
        TIMCDE = 2;
    } else {
        spicelib::SETMSG(
            b"SCLK time system for file # must be TDB or TDT but was #.",
            ctx,
        );
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRCH(b"#", TIMSYS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDTIMESYSTEM)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut LINE, b"   SCLK_DATA_TYPE_#              = ( 1 )");
    fstr::assign(&mut LINE, b"   SCLK01_TIME_SYSTEM_#          = ( # )");
    spicelib::REPMI(&LINE.clone(), b"#", STRID, &mut LINE, ctx);
    spicelib::REPMI(&LINE.clone(), b"#", TIMCDE, &mut LINE, ctx);

    //
    // No blank line follows this output because the clock
    // attribute parameters are written as a contiguous
    // block. Why?  ...tradition.
    //
    spicelib::WRITLN(&LINE, UNIT, ctx)?;

    //
    // Write the number of fields.
    //
    if ((NFIELD < 1) || (NFIELD > MXNFLD)) {
        spicelib::SETMSG(
            b"SCLK field count for file # must be in range 1:# but was #.",
            ctx,
        );
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRINT(b"#", MXNFLD, ctx);
        spicelib::ERRINT(b"#", NFIELD, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDFIELDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut LINE, b"   SCLK01_N_FIELDS_#             = ( # )");
    spicelib::REPMI(&LINE.clone(), b"#", STRID, &mut LINE, ctx);
    spicelib::REPMI(&LINE.clone(), b"#", NFIELD, &mut LINE, ctx);

    spicelib::WRITLN(&LINE, UNIT, ctx)?;

    //
    // Write the moduli. Recall these are d.p. numbers.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NFIELD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // All moduli must be in the range 1 : 1e14.
            //
            if ((MODULI[I] < 1.0) || (MODULI[I] > 100000000000000.0)) {
                spicelib::SETMSG(
                    b"SCLK modulus # for file # must be in range 1 : # but was #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", 100000000000000.0, ctx);
                spicelib::ERRDP(b"#", MODULI[I], ctx);
                spicelib::SIGERR(b"SPICE(INVALIDMODULUS)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Make sure the product of the moduli can fit in the d.p. mantissa
    // without truncation.
    //
    PRODM = MODULI[1];

    {
        let m1__: i32 = 2;
        let m2__: i32 = NFIELD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            PRODM = (PRODM * MODULI[I]);
            I += m3__;
        }
    }

    if (PRODM > 1000000000000000.0) {
        spicelib::SETMSG(b"SCLK modulus product for file # must be in range # : # but was #. (Lower bound is 2**NFIELD.)", ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRINT(b"#", intrinsics::pow(2, NFIELD), ctx);
        spicelib::ERRDP(b"#", 1000000000000000.0, ctx);
        spicelib::ERRDP(b"#", PRODM, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDMODULI)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    //
    // If we have 4 or fewer fields, write the moduli out
    // on one line on the right hand side. Otherwise
    // stack up the values on multiple lines.
    //
    // Build the line in a long string; check the length
    // before writing the line.
    //

    fstr::assign(&mut LONGLN, b"   SCLK01_MODULI_#               = ( #");
    spicelib::REPMI(&LONGLN.clone(), b"#", STRID, &mut LONGLN, ctx);

    if (NFIELD <= 4) {
        {
            let m1__: i32 = 1;
            let m2__: i32 = (NFIELD - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::REPMC(&LONGLN.clone(), b"#", b"# #", &mut LONGLN);

                spicelib::DPFMT(MODULI[I], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
                spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
                spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

                I += m3__;
            }
        }

        spicelib::DPFMT(MODULI[NFIELD], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
        spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

        spicelib::SUFFIX(b")", 1, &mut LONGLN);

        LNB = spicelib::LASTNB(&LONGLN);

        if (LNB <= 80) {
            spicelib::WRITLN(&LONGLN, UNIT, ctx)?;
        }
    }

    //
    // The following case is for moduli assignments that
    // span multiple lines.
    //
    if ((NFIELD > 4) || (LNB > 80)) {
        //
        // Save the start position for the moduli.
        //
        START = spicelib::LASTNB(&LONGLN);
        //
        //
        // Fill in the first line and write it.
        //
        spicelib::DPFMT(MODULI[1], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
        spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

        //
        // Append a comma to the line.
        //
        spicelib::SUFFIX(b",", 0, &mut LONGLN);

        spicelib::WRITLN(&LONGLN, UNIT, ctx)?;

        //
        // Fill in the lines preceding the last line and
        // write these as well.
        //
        {
            let m1__: i32 = 2;
            let m2__: i32 = (NFIELD - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut LONGLN, b" ");
                fstr::assign(fstr::substr_mut(&mut LONGLN, START..), b"#,");

                spicelib::DPFMT(MODULI[I], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
                spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
                spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

                spicelib::WRITLN(&LONGLN, UNIT, ctx)?;

                I += m3__;
            }
        }

        //
        // Fill in and write the last line.
        //
        fstr::assign(&mut LONGLN, b" ");
        fstr::assign(fstr::substr_mut(&mut LONGLN, START..), b"# )");

        spicelib::DPFMT(MODULI[NFIELD], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
        spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

        spicelib::WRITLN(&LONGLN, UNIT, ctx)?;
    }

    //
    // Write the offsets. Recall these are d.p. numbers.
    // The method is the same as for the moduli, except for
    // the range check.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NFIELD;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // All offsets must be in the range 0 : moduli(i)-1.
            //
            if ((OFFSET[I] < 0.0) || (OFFSET[I] > (MODULI[1] - 1 as f64))) {
                spicelib::SETMSG(
                    b"SCLK offset # for file # must be in range 0 : # but was #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", (MODULI[I] - 1 as f64), ctx);
                spicelib::ERRDP(b"#", OFFSET[I], ctx);
                spicelib::SIGERR(b"SPICE(INVALIDOFFSET)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // If we have 4 or fewer fields, write the offsets out
    // on one line on the right hand side. Otherwise
    // stack up the values on multiple lines.
    //
    // Build the line in a long string; check the length
    // before writing the line.
    //
    fstr::assign(&mut LONGLN, b"   SCLK01_OFFSETS_#              = ( #");
    spicelib::REPMI(&LONGLN.clone(), b"#", STRID, &mut LONGLN, ctx);

    if (NFIELD <= 4) {
        {
            let m1__: i32 = 1;
            let m2__: i32 = (NFIELD - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::REPMC(&LONGLN.clone(), b"#", b"# #", &mut LONGLN);

                spicelib::DPFMT(OFFSET[I], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
                spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
                spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

                I += m3__;
            }
        }

        spicelib::DPFMT(OFFSET[NFIELD], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
        spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

        spicelib::SUFFIX(b")", 1, &mut LONGLN);

        LNB = spicelib::LASTNB(&LONGLN);

        if (LNB <= 80) {
            spicelib::WRITLN(&LONGLN, UNIT, ctx)?;
        }
    }

    //
    // The following case is for offset assignments that
    // span multiple lines.
    //
    if ((NFIELD > 4) || (LNB > 80)) {
        //
        // Save the start position for the offsets.
        //
        START = spicelib::LASTNB(&LONGLN);
        //
        //
        // Fill in the first line and write it.
        //
        spicelib::DPFMT(OFFSET[1], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
        spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

        //
        // Append a comma to the line.
        //
        spicelib::SUFFIX(b",", 0, &mut LONGLN);

        spicelib::WRITLN(&LONGLN, UNIT, ctx)?;

        //
        // Fill in the lines preceding the last line and
        // write these as well.
        //
        {
            let m1__: i32 = 2;
            let m2__: i32 = (NFIELD - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut LONGLN, b" ");
                fstr::assign(fstr::substr_mut(&mut LONGLN, START..), b"#,");

                spicelib::DPFMT(OFFSET[I], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
                spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
                spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

                spicelib::WRITLN(&LONGLN, UNIT, ctx)?;

                I += m3__;
            }
        }

        //
        // Fill in and write the last line.
        //
        fstr::assign(&mut LONGLN, b" ");
        fstr::assign(fstr::substr_mut(&mut LONGLN, START..), b"# )");

        spicelib::DPFMT(OFFSET[NFIELD], b"XXXXXXXXXXXXXX", &mut NUMSTR, ctx)?;
        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);
        spicelib::REPMC(&LONGLN.clone(), b"#", &NUMSTR, &mut LONGLN);

        spicelib::WRITLN(&LONGLN, UNIT, ctx)?;
    }

    //
    // Write the output delimiter line.
    //
    J = intrinsics::INDEX(&save.DLCHRS, DELIM);

    if (J == 0) {
        spicelib::SETMSG(
            b"SCLK output delimiter for file # must be in set {#} but was #.",
            ctx,
        );
        spicelib::ERRCH(b"#", FNAME, ctx);

        {
            let m1__: i32 = 1;
            let m2__: i32 = (NDELIM - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::ERRCH(
                    b"#",
                    &fstr::concat(
                        &fstr::concat(b"#", fstr::substr(&save.DLCHRS, I..=I)),
                        b",#",
                    ),
                    ctx,
                );
                I += m3__;
            }
        }

        spicelib::ERRCH(b"#", fstr::substr(&save.DLCHRS, NDELIM..=NDELIM), ctx);
        spicelib::ERRCH(b"#", DELIM, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDDELIMITER)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    fstr::assign(&mut LINE, b"   SCLK01_OUTPUT_DELIM_#         = ( # )");
    spicelib::REPMI(&LINE.clone(), b"#", STRID, &mut LINE, ctx);
    spicelib::REPMI(&LINE.clone(), b"#", J, &mut LINE, ctx);

    spicelib::WRITLN(&LINE, UNIT, ctx)?;

    //
    // A blank line separates the clock attributes from the
    // partition data.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Write the partition start data; one value only per line.
    //
    if ((NPARTS < 1) || (NPARTS > MXPART)) {
        spicelib::SETMSG(
            b"SCLK partition count for file # must be in range 1 : # but was #.",
            ctx,
        );
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRINT(b"#", MXPART, ctx);
        spicelib::ERRINT(b"#", NPARTS, ctx);
        spicelib::SIGERR(b"SPICE(BADPARTITIONCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = NPARTS;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Partition start readouts must be at least zero
            // and must be strictly less than the corresponding
            // stop readouts.
            //
            if ((PSTART[I] < 0.0) || (PSTART[I] >= PSTOP[I])) {
                spicelib::SETMSG(
                    b"SCLK partition start # for file # must be in range 0 : # but was #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", (PSTOP[I] - 1 as f64), ctx);
                spicelib::ERRDP(b"#", PSTART[I], ctx);
                spicelib::SIGERR(b"SPICE(BADPARTITIONSTART)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            //
            // Partition start times must be integral.
            //
            if (PSTART[I] != f64::round(PSTART[I])) {
                spicelib::SETMSG(
                    b"SCLK partition start # for file # must be integral but was #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", PSTART[I], ctx);
                spicelib::SIGERR(b"SPICE(BADPARTITIONSTART)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    fstr::assign(&mut LINE, b"   SCLK_PARTITION_START_#        = ( #");
    spicelib::REPMI(&LINE.clone(), b"#", STRID, &mut LINE, ctx);

    //
    // Record the value start index in the line.
    //
    START = spicelib::LASTNB(&LINE);

    //
    // We must format partition bounds using a Fortran internal write
    // because the SPICE formatting routines can't handle 16
    // significant digits.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let internal_file = io::InternalFile::open(&mut NUMSTR);
        let mut writer = io::FormattedWriter::new(internal_file, None, b"(F20.1)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_f64(PSTART[1])?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(
            b"SCLK partition start # for file # could not be formatted; value = #; IOSTAT = #.",
            ctx,
        );
        spicelib::ERRINT(b"#", I, ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRDP(b"#", PSTART[1], ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(WRITEERROR)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    spicelib::REPMC(&LINE.clone(), b"#", &NUMSTR, &mut LINE);

    if (NPARTS > 1) {
        spicelib::SUFFIX(b",", 0, &mut LINE);
    } else {
        spicelib::SUFFIX(b")", 1, &mut LINE);
    }

    spicelib::WRITLN(&LINE, UNIT, ctx)?;

    if (NPARTS > 1) {
        //
        // Write the rest of the partitions.
        //
        {
            let m1__: i32 = 2;
            let m2__: i32 = (NPARTS - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let internal_file = io::InternalFile::open(&mut NUMSTR);
                    let mut writer = io::FormattedWriter::new(internal_file, None, b"(F20.1)")?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_f64(PSTART[I])?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    spicelib::SETMSG(b"SCLK partition start # for file # could not be formatted; value = #; IOSTAT = #.", ctx);
                    spicelib::ERRINT(b"#", I, ctx);
                    spicelib::ERRCH(b"#", FNAME, ctx);
                    spicelib::ERRDP(b"#", PSTART[I], ctx);
                    spicelib::ERRINT(b"#", IOSTAT, ctx);
                    spicelib::SIGERR(b"SPICE(WRITEERROR)", ctx)?;
                    spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                    return Ok(());
                }

                spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);

                fstr::assign(&mut LINE, b" ");
                fstr::assign(fstr::substr_mut(&mut LINE, START..), &NUMSTR);

                spicelib::SUFFIX(b",", 0, &mut LINE);

                spicelib::WRITLN(&LINE, UNIT, ctx)?;

                I += m3__;
            }
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let internal_file = io::InternalFile::open(&mut NUMSTR);
            let mut writer = io::FormattedWriter::new(internal_file, None, b"(F20.1)")?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_f64(PSTART[NPARTS])?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(
                b"SCLK partition start # for file # could not be formatted; value = #; IOSTAT = #.",
                ctx,
            );
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRCH(b"#", FNAME, ctx);
            spicelib::ERRDP(b"#", PSTART[I], ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(WRITEERROR)", ctx)?;
            spicelib::CHKOUT(b"T_WCLK01", ctx)?;
            return Ok(());
        }

        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);

        fstr::assign(&mut LINE, b" ");
        fstr::assign(fstr::substr_mut(&mut LINE, START..), &NUMSTR);

        spicelib::SUFFIX(b")", 1, &mut LINE);

        spicelib::WRITLN(&LINE, UNIT, ctx)?;
    }

    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Write the partition stop data; one value only per line.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPARTS;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Partition stop readouts must be strictly greater than the
            // partition start values (this has been checked) and must not
            // exceed the product of the moduli.
            //
            if (PSTOP[I] > PRODM) {
                spicelib::SETMSG(
                    b"SCLK partition stop # for file # must be in range # : # but was #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", PSTART[I], ctx);
                spicelib::ERRDP(b"#", PRODM, ctx);
                spicelib::ERRDP(b"#", PSTOP[I], ctx);
                spicelib::SIGERR(b"SPICE(BADPARTITIONSTOP)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            //
            // Partition stop times must be integral.
            //
            if (PSTOP[I] != f64::round(PSTOP[I])) {
                spicelib::SETMSG(
                    b"SCLK partition stop # for file # must be integral but was #.",
                    ctx,
                );
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", PSTOP[I], ctx);
                spicelib::SIGERR(b"SPICE(BADPARTITIONSTOP)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    fstr::assign(&mut LINE, b"   SCLK_PARTITION_END_#          = ( #");
    spicelib::REPMI(&LINE.clone(), b"#", STRID, &mut LINE, ctx);

    //
    // Record the value start index in the line.
    //
    START = spicelib::LASTNB(&LINE);

    //
    // We must format partition bounds using a Fortran internal write
    // because the SPICE formatting routines can't handle 16
    // significant digits.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let internal_file = io::InternalFile::open(&mut NUMSTR);
        let mut writer = io::FormattedWriter::new(internal_file, None, b"(F20.1)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_f64(PSTOP[1])?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(
            b"SCLK partition stop # for file # could not be formatted; value = #; IOSTAT = #.",
            ctx,
        );
        spicelib::ERRINT(b"#", I, ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRDP(b"#", PSTOP[1], ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(WRITEERROR)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    spicelib::REPMC(&LINE.clone(), b"#", &NUMSTR, &mut LINE);

    if (NPARTS > 1) {
        spicelib::SUFFIX(b",", 0, &mut LINE);
    } else {
        spicelib::SUFFIX(b")", 1, &mut LINE);
    }

    spicelib::WRITLN(&LINE, UNIT, ctx)?;

    if (NPARTS > 1) {
        //
        // Write the rest of the partitions.
        //
        {
            let m1__: i32 = 2;
            let m2__: i32 = (NPARTS - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let internal_file = io::InternalFile::open(&mut NUMSTR);
                    let mut writer = io::FormattedWriter::new(internal_file, None, b"(F20.1)")?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_f64(PSTOP[I])?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    spicelib::SETMSG(b"SCLK partition stop # for file # could not be formatted; value = #; IOSTAT = #.", ctx);
                    spicelib::ERRINT(b"#", I, ctx);
                    spicelib::ERRCH(b"#", FNAME, ctx);
                    spicelib::ERRDP(b"#", PSTOP[I], ctx);
                    spicelib::ERRINT(b"#", IOSTAT, ctx);
                    spicelib::SIGERR(b"SPICE(WRITEERROR)", ctx)?;
                    spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                    return Ok(());
                }

                spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);

                fstr::assign(&mut LINE, b" ");
                fstr::assign(fstr::substr_mut(&mut LINE, START..), &NUMSTR);

                spicelib::SUFFIX(b",", 0, &mut LINE);

                spicelib::WRITLN(&LINE, UNIT, ctx)?;

                I += m3__;
            }
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let internal_file = io::InternalFile::open(&mut NUMSTR);
            let mut writer = io::FormattedWriter::new(internal_file, None, b"(F20.1)")?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_f64(PSTOP[NPARTS])?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(
                b"SCLK partition stop # for file # could not be formatted; value = #; IOSTAT = #.",
                ctx,
            );
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRCH(b"#", FNAME, ctx);
            spicelib::ERRDP(b"#", PSTOP[I], ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(WRITEERROR)", ctx)?;
            spicelib::CHKOUT(b"T_WCLK01", ctx)?;
            return Ok(());
        }

        spicelib::LJUST(&NUMSTR.clone(), &mut NUMSTR);

        fstr::assign(&mut LINE, b" ");
        fstr::assign(fstr::substr_mut(&mut LINE, START..), &NUMSTR);

        spicelib::SUFFIX(b")", 1, &mut LINE);

        spicelib::WRITLN(&LINE, UNIT, ctx)?;
    }

    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Prepare to write the coefficients.
    //
    //
    // Perform some simple checks on the coefficients.
    //
    if ((NCOEFF < 1) || (NCOEFF > MXCOEF)) {
        spicelib::SETMSG(
            b"SCLK coefficient record count for file # must be in range 1 : # but was #.",
            ctx,
        );
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRINT(b"#", MXCOEF, ctx);
        spicelib::ERRINT(b"#", NCOEFF, ctx);
        spicelib::SIGERR(b"SPICE(BADRECORDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    //
    // The first tick value must be non-negative; the tick values
    // must be strictly increasing.
    //
    if (COEFFS[[1, 1]] < 0.0) {
        spicelib::SETMSG(
            b"First tick coefficient  for file # must be non-negative but was #.",
            ctx,
        );
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRDP(b"#", COEFFS[[1, 1]], ctx);
        spicelib::SIGERR(b"SPICE(NEGATIVETICK)", ctx)?;
        spicelib::CHKOUT(b"T_WCLK01", ctx)?;
        return Ok(());
    }

    {
        let m1__: i32 = 2;
        let m2__: i32 = NCOEFF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (COEFFS[[1, I]] <= COEFFS[[1, (I - 1)]]) {
                spicelib::SETMSG(b"In file #, tick coefficient # at index # must be strictly less than coefficient # at index # but was not. ", ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", COEFFS[[1, (I - 1)]], ctx);
                spicelib::ERRINT(b"#", (I - 1), ctx);
                spicelib::ERRDP(b"#", COEFFS[[1, I]], ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(TICKSOUTOFORDER)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // The parallel time system values must be strictly increasing.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = NCOEFF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (COEFFS[[2, I]] <= COEFFS[[2, (I - 1)]]) {
                spicelib::SETMSG(b"In file #, parallel time coefficient # at index # must be strictly less than coefficient # at index # but was not. ", ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", COEFFS[[2, (I - 1)]], ctx);
                spicelib::ERRINT(b"#", (I - 1), ctx);
                spicelib::ERRDP(b"#", COEFFS[[2, I]], ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(TIMESOUTOFORDER)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // The SCLK rate values must be strictly positive.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NCOEFF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (COEFFS[[3, I]] <= 0.0) {
                spicelib::SETMSG(b"In file #, SCLK rate coefficient # at index # must be strictly positive but was not. ", ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", COEFFS[[3, (I - 1)]], ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(NONPOSITIVERATE)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Write the coefficient data; three values per line.
    // The first line contains no data.
    //
    fstr::assign(&mut LINE, b"   SCLK01_COEFFICIENTS_#         = (");
    spicelib::REPMI(&LINE.clone(), b"#", STRID, &mut LINE, ctx);

    spicelib::WRITLN(&LINE, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NCOEFF;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // We must format coefficients using a Fortran internal write
            // because the SPICE formatting routines can't handle 16
            // significant digits.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let internal_file = io::InternalFile::open(&mut LINE);
                let mut writer =
                    io::FormattedWriter::new(internal_file, None, b"(3X,F20.1,2E26.16)")?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    for J in intrinsics::range(1, 3, 1) {
                        writer.write_f64(COEFFS[[J, I]])?;
                    }
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                spicelib::SETMSG(b"Coefficient record # for file # could not be formatted; values = # # #; IOSTAT = #.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::ERRCH(b"#", FNAME, ctx);
                spicelib::ERRDP(b"#", COEFFS[[1, I]], ctx);
                spicelib::ERRDP(b"#", COEFFS[[2, I]], ctx);
                spicelib::ERRDP(b"#", COEFFS[[3, I]], ctx);
                spicelib::ERRINT(b"#", IOSTAT, ctx);
                spicelib::SIGERR(b"SPICE(WRITEERROR)", ctx)?;
                spicelib::CHKOUT(b"T_WCLK01", ctx)?;
                return Ok(());
            }

            spicelib::WRITLN(&LINE, UNIT, ctx)?;

            I += m3__;
        }
    }

    fstr::assign(&mut LINE, b"   )");

    spicelib::WRITLN(&LINE, UNIT, ctx)?;

    //
    // Write a final "begintext" marker.
    //
    spicelib::WRITLN(b" ", UNIT, ctx)?;
    let mut arg0 = vec![b' '; 80];
    BEGTXT(&mut arg0);
    spicelib::WRITLN(&arg0, UNIT, ctx)?;
    spicelib::WRITLN(b" ", UNIT, ctx)?;

    //
    // Finish the file.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    spicelib::CHKOUT(b"T_WCLK01", ctx)?;
    Ok(())
}
