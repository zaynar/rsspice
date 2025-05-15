//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MSGLEN: i32 = 500;
const MAXTAB: i32 = 100;
const MAXVAL: i32 = 10000;
const NAMLEN: i32 = 45;

struct SaveVars {
    NAME: Vec<u8>,
    NEWNAM: Vec<u8>,
    SYNAMS: ActualCharArray,
    XNAMES: ActualCharArray,
    TITLE: Vec<u8>,
    DVALS: ActualArray<f64>,
    SYVALS: ActualArray<f64>,
    VAL: f64,
    XVAL: f64,
    XVALS: ActualArray<f64>,
    I: i32,
    J: i32,
    NVALS: i32,
    SIZE: i32,
    START: i32,
    SYPTRS: StackArray<i32, 106>,
    TABSIZ: i32,
    TO: i32,
    UB: i32,
    XCARD: i32,
    XPTRS: StackArray<i32, 100>,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAME = vec![b' '; NAMLEN as usize];
        let mut NEWNAM = vec![b' '; NAMLEN as usize];
        let mut SYNAMS = ActualCharArray::new(NAMLEN, LBCELL..=MAXTAB);
        let mut XNAMES = ActualCharArray::new(NAMLEN, 1..=MAXTAB);
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut DVALS = ActualArray::<f64>::new(1..=MAXVAL);
        let mut SYVALS = ActualArray::<f64>::new(LBCELL..=MAXVAL);
        let mut VAL: f64 = 0.0;
        let mut XVAL: f64 = 0.0;
        let mut XVALS = ActualArray::<f64>::new(1..=MAXVAL);
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut NVALS: i32 = 0;
        let mut SIZE: i32 = 0;
        let mut START: i32 = 0;
        let mut SYPTRS = StackArray::<i32, 106>::new(LBCELL..=MAXTAB);
        let mut TABSIZ: i32 = 0;
        let mut TO: i32 = 0;
        let mut UB: i32 = 0;
        let mut XCARD: i32 = 0;
        let mut XPTRS = StackArray::<i32, 100>::new(1..=MAXTAB);
        let mut FOUND: bool = false;

        Self {
            NAME,
            NEWNAM,
            SYNAMS,
            XNAMES,
            TITLE,
            DVALS,
            SYVALS,
            VAL,
            XVAL,
            XVALS,
            I,
            J,
            NVALS,
            SIZE,
            START,
            SYPTRS,
            TABSIZ,
            TO,
            UB,
            XCARD,
            XPTRS,
            FOUND,
        }
    }
}

//$Procedure F_SYMTBD ( Test SPICELIB d.p. symbol table routines )
pub fn F_SYMTBD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //
    // To avoid memory problems under cygwin, save everything.
    //
    //

    //
    // Initial values
    //

    //
    // *********************************************************
    // Note:  the order of the test cases is significant in that
    // some cases depend on results from other cases.  Take care
    // to preserve side effects when modifying this code!
    // *********************************************************
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SYMTBD", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYSETD test:  Populate a simple symbol table with scalar-valued symbols.",
        ctx,
    )?;

    //
    // Initialize the symbol table.
    //
    spicelib::SSIZEC(MAXTAB, save.SYNAMS.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI(MAXTAB, save.SYPTRS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXVAL, save.SYVALS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = 1;
            save.XVALS[save.I] = -save.I as f64;

            //
            // Associate the value -I with the Ith symbol.  Add this
            // symbol to the table.
            //
            spicelib::SYSETD(
                &save.NAME,
                (-save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETD test:  Populate a simple symbol table with scalar-valued symbols.  Insert symbols in reverse order.", ctx)?;

    //
    // Clear out the symbol table.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    //
    // Use the symbol names and values from the previous test.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Let J be the index of the symbol to insert.
            //
            save.J = ((MAXTAB + 1) - save.I);
            //
            // Associate the value -J with the Jth symbol.  Add this
            // symbol to the table.
            //
            spicelib::SYSETD(
                &save.XNAMES[save.J],
                (-save.J as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // We should end up with the same symbol table as in the
    // previous test case.
    //
    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYPUTD test:  Populate a simple symbol table with scalar-valued symbols.",
        ctx,
    )?;

    //
    // This is essentially a reprise of the previous SYSETD test.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = 1;
            save.XVALS[save.I] = -save.I as f64;

            //
            // Associate the value -I with the Ith symbol.  Add this
            // symbol to the table.
            //
            spicelib::SYPUTD(
                &save.NAME,
                save.XVALS.subarray(save.I),
                1,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYFETD test:  Fetch names from symbol table with scalar-valued symbols.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYFETD(
                save.I,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NAME,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            testutil::CHCKSC(&save.TITLE, &save.NAME, b"=", &save.XNAMES[save.I], OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Also look for a symbol we know isn't there.
    //
    save.I = (MAXTAB + 1);
    fstr::assign(&mut save.TITLE, b"Was name # found?");
    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

    spicelib::SYFETD(
        (MAXTAB + 1),
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_slice(),
        &mut save.NAME,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(&save.TITLE, save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYGETD test:  Fetch values from symbol table with scalar-valued symbols.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                std::slice::from_mut(&mut save.VAL),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Value of symbol #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            testutil::CHCKSD(
                &save.TITLE,
                save.VAL,
                b"=",
                save.XVALS[save.I],
                0.0,
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Also look for a symbol we know isn't there.
    //
    fstr::assign(&mut save.NAME, b"NOT_THERE");

    spicelib::SYGETD(
        &save.NAME,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_slice(),
        &mut save.NVALS,
        std::slice::from_mut(&mut save.VAL),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.TITLE, b"Was name # found?");
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NAME, &mut save.TITLE);
    testutil::CHCKSL(&save.TITLE, save.FOUND, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYREND test:  Rename symbols in a simple symbol table with scalar-valued symbols.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Get the name and value of the ith symbol.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            spicelib::SYGETD(
                &save.NAME,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                std::slice::from_mut(&mut save.XVAL),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            //
            // Create the replacement name:
            //
            spicelib::REPLCH(&save.NAME, b"X", b"Y", &mut save.NEWNAM);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Change the name of the symbol.
            //
            spicelib::SYREND(
                &save.NAME,
                &save.NEWNAM,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the old symbol is gone.
            //
            spicelib::SYGETD(
                &save.NAME,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                std::slice::from_mut(&mut save.VAL),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NAME, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, false, OK, ctx)?;

            //
            // Get the value associated with the new name.
            //
            //
            // Get the name and value of the ith symbol.
            //
            spicelib::SYGETD(
                &save.NEWNAM,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                std::slice::from_mut(&mut save.VAL),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            //
            // Check the value.
            //
            testutil::CHCKSD(b"VAL", save.VAL, b"=", save.XVAL, 0.0, OK, ctx)?;

            //
            // Change the name back to its original value.
            //
            spicelib::SYREND(
                &save.NEWNAM,
                &save.NAME,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYDELD test:  Delete symbols from symbol table with scalar-valued symbols.",
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYDELD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the symbol is gone.
            //
            fstr::assign(&mut save.TITLE, b"Was (deleted symbol) name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                std::slice::from_mut(&mut save.VAL),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, false, OK, ctx)?;

            //
            // Validate the remaining symbol table.
            //
            {
                let m1__: i32 = (save.I + 1);
                let m2__: i32 = MAXTAB;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.TITLE, b"Was (remaining symbol) name # found?");
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

                    spicelib::SYGETD(
                        &save.XNAMES[save.J],
                        save.SYNAMS.as_arg(),
                        save.SYPTRS.as_slice(),
                        save.SYVALS.as_slice(),
                        &mut save.NVALS,
                        std::slice::from_mut(&mut save.VAL),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

                    fstr::assign(&mut save.TITLE, b"Remaining symbol # value");
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

                    testutil::CHCKSD(
                        &save.TITLE,
                        save.VAL,
                        b"=",
                        save.XVALS[save.J],
                        0.0,
                        OK,
                        ctx,
                    )?;

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    //     At this point, we need to work with symbol tables containing
    //     symbols having multiple values associated with them.  We'll
    //     build a symbol table whose nth symbol has n associated values.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYPUTD test:  Create symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    save.TO = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = save.I;
            save.START = save.TO;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XVALS[save.TO] = (-(MAXTAB * save.I) - save.J) as f64;
                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Add the symbol to the table.
            //
            spicelib::SYPUTD(
                &save.NAME,
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    save.XCARD = ((MAXTAB * (MAXTAB + 1)) / 2);

    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTD test:  Populate a simple symbol table with scalar-valued symbols.  Insert symbols in reverse order.", ctx)?;

    //
    // Clear out the symbol table.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    spicelib::CLEARD(MAXVAL, save.XVALS.as_slice_mut());

    //
    // Use the symbol names and values from the previous test.
    //
    save.SIZE = ((MAXTAB * (MAXTAB + 1)) / 2);

    save.START = ((save.SIZE - MAXTAB) + 1);

    {
        let m1__: i32 = MAXTAB;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Add the symbol to the table.
            //
            spicelib::SYPUTD(
                &save.XNAMES[save.I],
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;

            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.START = (save.START - (save.I - 1));

            save.I += m3__;
        }
    }

    //
    // We should end up with the same symbol table as in the
    // previous test case.
    //
    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    save.XCARD = ((MAXTAB * (MAXTAB + 1)) / 2);

    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYGETD test:  validate array-valued symbol table by fetching values associated with each symbol.", ctx)?;

    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol #");
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

                testutil::CHCKAD(
                    &save.TITLE,
                    save.DVALS.as_slice(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYNTHD test:  validate array-valued symbol table by fetching values associated with each symbol.", ctx)?;

    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                //
                // Get the size of the current symbol.
                //
                save.SIZE = spicelib::SYDIMD(
                    &save.XNAMES[save.I],
                    save.SYNAMS.as_arg(),
                    save.SYPTRS.as_slice(),
                    save.SYVALS.as_slice(),
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.SIZE;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        //
                        // Fetch each value.
                        //
                        fstr::assign(&mut save.TITLE, b"Value # of symbol #");
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
                        spicelib::REPMC(
                            &save.TITLE.to_vec(),
                            b"#",
                            &save.XNAMES[save.I],
                            &mut save.TITLE,
                        );
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        spicelib::SYNTHD(
                            &save.XNAMES[save.I],
                            save.J,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_slice(),
                            &mut save.VAL,
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

                        testutil::CHCKSD(
                            &save.TITLE,
                            save.VAL,
                            b"=",
                            save.XVALS[((save.START + save.J) - 1)],
                            0.0,
                            OK,
                            ctx,
                        )?;

                        save.J += m3__;
                    }
                }

                //
                // Try to fetch an element we know isn't there.
                //
                save.J = (save.SIZE + 1);

                fstr::assign(&mut save.TITLE, b"Value # of symbol #");
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.J, &mut save.TITLE, ctx);
                spicelib::REPMC(
                    &save.TITLE.to_vec(),
                    b"#",
                    &save.XNAMES[save.I],
                    &mut save.TITLE,
                );
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SYNTHD(
                    &save.XNAMES[save.I],
                    save.J,
                    save.SYNAMS.as_arg(),
                    save.SYPTRS.as_slice(),
                    save.SYVALS.as_slice(),
                    &mut save.VAL,
                    &mut save.FOUND,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSL(&save.TITLE, save.FOUND, false, OK, ctx)?;
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYENQD test:  Create symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    save.TO = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = save.I;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XVALS[save.TO] = (-(MAXTAB * save.I) - save.J) as f64;

                    spicelib::SYENQD(
                        &save.NAME,
                        save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    save.XCARD = ((MAXTAB * (MAXTAB + 1)) / 2);

    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYPSHD test:  Create symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    save.TO = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = save.I;
            save.TO = (save.TO + save.I);

            //
            // At the start of the loop, TO points to the element of XVALS
            // that will hold the last value associated with the Ith symbol.
            //
            {
                let m1__: i32 = save.I;
                let m2__: i32 = 1;
                let m3__: i32 = -1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XVALS[save.TO] = (-(MAXTAB * save.I) - save.J) as f64;

                    spicelib::SYPSHD(
                        &save.NAME,
                        save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.TO = (save.TO - 1);

                    save.J += m3__;
                }
            }

            save.TO = (save.TO + save.I);

            save.I += m3__;
        }
    }

    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    save.XCARD = ((MAXTAB * (MAXTAB + 1)) / 2);

    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPD test:  Create symbol table with array-valued symbols.  Then duplicate each symbol.", ctx)?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    //
    // We'll set the cardinality upper bound UB to MAXTAB/2, so
    // we'll have room for the duplicate symbols.
    //
    save.TO = 1;
    save.UB = (MAXTAB / 2);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, save.UB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = save.I;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XVALS[save.TO] = (-(save.UB * save.I) - save.J) as f64;

                    spicelib::SYENQD(
                        &save.NAME,
                        save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Validate the symbol table at this stage.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found? (0)");
            spicelib::REPMC(
                &save.TITLE.to_vec(),
                b"#",
                &save.XNAMES[save.I],
                &mut save.TITLE,
            );

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol # (0)");
                spicelib::REPMC(
                    &save.TITLE.to_vec(),
                    b"#",
                    &save.XNAMES[save.I],
                    &mut save.TITLE,
                );

                testutil::CHCKAD(
                    &save.TITLE,
                    save.DVALS.as_slice(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // Look up each symbol; add to the symbol table a duplicate symbol
    // with a new name.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, save.UB, &mut save.NAME, ctx)?;

            fstr::assign(&mut save.NEWNAM, &save.NAME);
            fstr::assign(fstr::substr_mut(&mut save.NEWNAM, 1..=2), b"2_");

            spicelib::SYDUPD(
                &save.NAME,
                &save.NEWNAM,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now validate the symbol table.  First make sure the original
    // symbols are intact.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found? (1)");
            spicelib::REPMC(
                &save.TITLE.to_vec(),
                b"#",
                &save.XNAMES[save.I],
                &mut save.TITLE,
            );

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol # (1)");
                spicelib::REPMC(
                    &save.TITLE.to_vec(),
                    b"#",
                    &save.XNAMES[save.I],
                    &mut save.TITLE,
                );

                testutil::CHCKAD(
                    &save.TITLE,
                    save.DVALS.as_slice(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // Now check the duplicate symbols.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.NEWNAM, save.XNAMES.get(save.I));
            fstr::assign(fstr::substr_mut(&mut save.NEWNAM, 1..=2), b"2_");

            fstr::assign(&mut save.TITLE, b"Was name # found? (2)");
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWNAM, &mut save.TITLE);

            spicelib::SYGETD(
                &save.NEWNAM,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol # (2)");
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWNAM, &mut save.TITLE);

                testutil::CHCKAD(
                    &save.TITLE,
                    save.DVALS.as_slice(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPD test:  Create symbol table with array-valued symbols.  Duplicate these symbols.  Negate the values of the original symbols.  Then duplicate the duplicate the symbols, overwriting the original symbols.", ctx)?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    //
    // We'll set the cardinality upper bound UB to MAXTAB/2, so
    // we'll have room for the duplicate symbols.
    //
    save.TO = 1;
    save.UB = (MAXTAB / 2);

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, save.UB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = save.I;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XVALS[save.TO] = (-(save.UB * save.I) - save.J) as f64;

                    spicelib::SYENQD(
                        &save.NAME,
                        save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Look up each symbol; add to the symbol table a duplicate symbol
    // with a new name.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, save.UB, &mut save.NAME, ctx)?;

            fstr::assign(&mut save.NEWNAM, &save.NAME);
            fstr::assign(fstr::substr_mut(&mut save.NEWNAM, 1..=2), b"2_");

            spicelib::SYDUPD(
                &save.NAME,
                &save.NEWNAM,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Negate the values of the symbols of the original symbols.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NVALS;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.DVALS[save.J] = -save.DVALS[save.J];
                    save.J += m3__;
                }
            }

            spicelib::SYPUTD(
                &save.XNAMES[save.I],
                save.DVALS.as_slice(),
                save.NVALS,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Duplicate the duplicate symbols, overwriting the originals.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, save.UB, &mut save.NAME, ctx)?;

            fstr::assign(&mut save.NEWNAM, &save.NAME);
            fstr::assign(fstr::substr_mut(&mut save.NEWNAM, 1..=2), b"2_");

            spicelib::SYDUPD(
                &save.NEWNAM,
                &save.NAME,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now validate the symbol table.  First make sure the
    // first set of duplicate symbols is intact.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.UB, &mut save.NAME, ctx)?;

            fstr::assign(&mut save.NEWNAM, &save.NAME);
            fstr::assign(fstr::substr_mut(&mut save.NEWNAM, 1..=2), b"2_");

            fstr::assign(&mut save.TITLE, b"Was name # found? (1)");
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWNAM, &mut save.TITLE);

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol # (1)");
                spicelib::REPMC(
                    &save.TITLE.to_vec(),
                    b"#",
                    &save.XNAMES[save.I],
                    &mut save.TITLE,
                );

                testutil::CHCKAD(
                    &save.TITLE,
                    save.DVALS.as_slice(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // Now check the duplicated duplicate symbols.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found? (2)");
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NAME, &mut save.TITLE);

            spicelib::SYGETD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                &mut save.NVALS,
                save.DVALS.as_slice_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol # (2)");
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWNAM, &mut save.TITLE);

                testutil::CHCKAD(
                    &save.TITLE,
                    save.DVALS.as_slice(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYORDD test:  Create symbol table with array-valued symbols; sort values associated with each symbol.", ctx)?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    spicelib::CLEARD(MAXVAL, save.XVALS.as_slice_mut());

    //
    // Create the symbol table to be sorted.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.VAL = (-(MAXTAB * save.I) - save.J) as f64;

                    spicelib::SYENQD(
                        &save.NAME,
                        save.VAL,
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Sort the symbol values.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SYORDD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Now we must validate the ordered symbol table.  Create
    // an array containing the expected values in the table.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Fill in the values for the Ith symbol in the
                    // opposite order from that in which they were inserted
                    // into the symbol table.
                    //
                    save.TO = ((save.START + save.I) - save.J);

                    save.XVALS[save.TO] = (-(MAXTAB * save.I) - save.J) as f64;

                    save.J += m3__;
                }
            }

            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // Check the symbol table values against the value array.
    //
    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        MAXTAB,
        0.0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYDIMD test:  Find dimensions of symbols in symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    spicelib::CLEARD(MAXVAL, save.XVALS.as_slice_mut());

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.VAL = (-(MAXTAB * save.I) - save.J) as f64;

                    spicelib::SYENQD(
                        &save.NAME,
                        save.VAL,
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_slice_mut(),
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.NVALS = spicelib::SYDIMD(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Dimension of symbol #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            testutil::CHCKSI(&save.TITLE, save.NVALS, b"=", save.I, 0, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYPOPD test:  Pop values from symbol table with array-valued symbols.",
        ctx,
    )?;

    //
    // Re-create the default array-valued symbol table.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            //
            // Set the values of the Ith symbol.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.DVALS[save.J] = (-(MAXTAB * save.I) - save.J) as f64;

                    save.J += m3__;
                }
            }

            //
            // Insert the Ith symbol.
            //
            spicelib::SYPUTD(
                &save.NAME,
                save.DVALS.as_slice(),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now test SYPOPD.
    //
    save.TO = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, MAXTAB, &mut save.NAME, ctx)?;

            save.XPTRS[save.I] = save.I;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Set the expected value.
                    //
                    save.XVALS[save.TO] = (-(MAXTAB * save.I) - save.J) as f64;

                    spicelib::SYPOPD(
                        &save.NAME,
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_slice_mut(),
                        &mut save.VAL,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.TITLE, b"Was value # of symbol name # found?");
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.J, &mut save.TITLE, ctx);
                    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NAME, &mut save.TITLE);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        fstr::assign(&mut save.TITLE, b"Symbol #: popped value at index #");
                        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NAME, &mut save.TITLE);
                        spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.J, &mut save.TITLE, ctx);

                        testutil::CHCKSD(
                            &save.TITLE,
                            save.VAL,
                            b"=",
                            save.XVALS[save.TO],
                            0.0,
                            OK,
                            ctx,
                        )?;
                    }

                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Try to pop a value from the now empty symbol.
            //
            spicelib::SYPOPD(
                &save.NAME,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                &mut save.VAL,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Was element popped from symbol #?");
            spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NAME, &mut save.TITLE);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, false, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYTRND test:  Transpose values from symbol table with array-valued symbols.",
        ctx,
    )?;

    //
    // We'll create a small version of our array-valued symbol table.
    // Since we want to try all possible transpositions, there would
    // be an excessive number of calls to SYTRND if we used the full
    // set of MAXTAB symbols.
    //
    // Create the symbol table now.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    save.TABSIZ = 5;

    save.TO = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            //
            // Set the values of the Ith symbol.
            //
            save.START = save.TO;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XVALS[save.TO] = (-(save.TABSIZ * save.I) - save.J) as f64;
                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Insert the Ith symbol.
            //
            spicelib::SYPUTD(
                &save.NAME,
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now try the transpositions.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SIZE = save.I;
            //
            // Try every possible transposition of the elements of the Ith
            // symbol, including those where the indices of the transposed
            // elements are not distinct.
            //
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            //
            // Transpose elements J and K of the Ith symbol, for all
            // applicable J, K.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.SIZE;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    for K in 1..=save.SIZE {
                        spicelib::SYTRND(
                            &save.NAME,
                            save.J,
                            K,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Fetch the values of the modified symbol.
                        //
                        spicelib::SYGETD(
                            &save.NAME,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_slice(),
                            &mut save.NVALS,
                            save.DVALS.as_slice_mut(),
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

                        if save.FOUND {
                            //
                            // If DVALS reflects the correct transposition,
                            // then we can swap elements J and K of DVALS
                            // to obtain the original symbol's values.
                            //
                            if (save.J != K) {
                                spicelib::SWAPD_ARRAY(
                                    save.DVALS.subscript(save.J),
                                    save.DVALS.subscript(K),
                                    save.DVALS.as_slice_mut(),
                                );
                            }

                            fstr::assign(&mut save.TITLE, b"Values of symbol #:  J = #; K = #");
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.NAME,
                                &mut save.TITLE,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.J,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);

                            testutil::CHCKAD(
                                &save.TITLE,
                                save.DVALS.as_slice(),
                                b"=",
                                save.XVALS.subarray(save.START),
                                save.I,
                                0.0,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Undo the transposition in the symbol table.
                        //
                        spicelib::SYTRND(
                            &save.NAME,
                            save.J,
                            K,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_slice_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                    }

                    save.J += m3__;
                }
            }

            //
            // Set the start index for the next symbol.
            //
            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYSELD test:  Extract slice of values from symbol table with array-valued symbols.",
        ctx,
    )?;

    //
    // This approach used in this test case closely parallels that
    // of the SYTRND test case above.
    //
    // We'll create a small version of our array-valued symbol table.
    // Since we want to try all possible slices, there would
    // be an excessive number of calls to SYTRND if we used the full
    // set of MAXTAB symbols.
    //
    // Create the symbol table now.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.SYVALS.as_slice_mut(), ctx)?;

    spicelib::CLEARD(MAXVAL, save.XVALS.as_slice_mut());

    save.TABSIZ = 5;

    save.TO = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Create the symbol name.
            //
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            //
            // Set the values of the Ith symbol.
            //
            save.START = save.TO;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.XVALS[save.TO] = (-(save.TABSIZ * save.I) - save.J) as f64;
                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Insert the Ith symbol.
            //
            spicelib::SYPUTD(
                &save.NAME,
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now try the slice extractions.
    //
    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SIZE = save.I;
            //
            // Try every possible slice of the elements of the Ith
            // symbol, including those where the endpoint indices of the
            // slice are not distinct.
            //
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            //
            // Extrace the slice ranging from elements J to K of the Ith
            // symbol, for all applicable J, K.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.SIZE;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    for K in 1..=save.SIZE {
                        //
                        // Fetch the slice from J to K.  We expect to find a
                        // result only if K >= J.
                        //
                        spicelib::SYSELD(
                            &save.NAME,
                            save.J,
                            K,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_slice(),
                            save.DVALS.as_slice_mut(),
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        if (K >= save.J) {
                            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;
                        } else {
                            testutil::CHCKSL(&save.TITLE, save.FOUND, false, OK, ctx)?;
                        }

                        if save.FOUND {
                            fstr::assign(&mut save.TITLE, b"Slice #(#:#)");
                            spicelib::REPMC(
                                &save.TITLE.to_vec(),
                                b"#",
                                &save.NAME,
                                &mut save.TITLE,
                            );
                            spicelib::REPMI(
                                &save.TITLE.to_vec(),
                                b"#",
                                save.J,
                                &mut save.TITLE,
                                ctx,
                            );
                            spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);

                            testutil::CHCKAD(
                                &save.TITLE,
                                save.DVALS.as_slice(),
                                b"=",
                                save.XVALS.subarray(((save.START + save.J) - 1)),
                                ((K - save.J) + 1),
                                0.0,
                                OK,
                                ctx,
                            )?;
                        }
                    }

                    save.J += m3__;
                }
            }

            //
            // Set the start index for the next symbol.
            //
            save.START = (save.START + save.I);

            save.I += m3__;
        }
    }

    //
    //     Now for some error handling tests.
    //
    //
    //     SYDELD:  No errors are detected by this routine.
    //
    //     SYDIMD:  No errors are detected by this routine.
    //
    //
    //     SYDUPD:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPD:  symbol name not present", ctx)?;

    spicelib::SYDUPD(
        b"NOSYMBOL",
        b"NOSYMBOL2",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOSUCHSYMBOL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPD:  name table overflow", ctx)?;

    //
    // We'll create a small symbol table so we can test the handling of
    // overflow conditions.
    //
    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((2 * save.TABSIZ), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * save.TABSIZ), save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYDUPD(
        &save.NAME,
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NAMETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPD:  pointer table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED((save.TABSIZ + 2), save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYDUPD(
        &save.NAME,
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(POINTERTABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPD:  value table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED(save.TABSIZ, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYDUPD(
        &save.NAME,
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    //
    //     SYENQD:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYENQD:  value table overflow", ctx)?;

    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED(save.TABSIZ, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Enqueue a value onto the last symbol.
    //
    spicelib::SYENQD(
        &save.NAME,
        (-1 as f64),
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    //
    //
    //     SYFETD:  No errors are detected by this routine.
    //
    //     SYGETD:  No errors are detected by this routine.
    //
    //     SYNTHD:  No errors are detected by this routine.
    //
    //     SYORDD:  No errors are detected by this routine.
    //
    //     SYPOPD:  No errors are detected by this routine.
    //
    //
    //
    //     SYPSHD:
    //
    // --- Case: ------------------------------------------------------
    //
    //
    testutil::TCASE(b"SYPSHD:  value table overflow", ctx)?;

    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED(save.TABSIZ, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Push a value onto the last symbol.
    //
    spicelib::SYPSHD(
        &save.NAME,
        (-1 as f64),
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    //     SYPUTD:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTD:  name table overflow", ctx)?;

    //
    // We'll create a small symbol table so we can test the handling of
    // overflow conditions.
    //
    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((2 * save.TABSIZ), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * save.TABSIZ), save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTD(
        b"NEWNAME",
        &[(save.I as f64)],
        1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NAMETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTD:  pointer table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED((save.TABSIZ + 2), save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTD(
        b"NEWNAME",
        &[(save.I as f64)],
        1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(POINTERTABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTD:  value table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED(save.TABSIZ, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTD(
        b"NEWNAME",
        &[(save.I as f64)],
        1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTD: invalid value count", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED((save.TABSIZ + 1), save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTD(
        b"NEWNAME",
        &[(save.I as f64)],
        -1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDARGUMENT)", OK, ctx)?;

    //
    //     SYREND:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYREND:  rename non-existent symbol", ctx)?;

    spicelib::SYREND(
        b"NONNAME",
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOSUCHSYMBOL)", OK, ctx)?;

    //
    //
    //     SYSELD:  No errors are detected by this routine.
    //
    //
    //     SYSETD:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETD:  name table overflow", ctx)?;

    //
    // We'll create a small symbol table so we can test the handling of
    // overflow conditions.
    //
    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((2 * save.TABSIZ), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * save.TABSIZ), save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYSETD(
        b"NEWNAME",
        (save.I as f64),
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NAMETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETD:  pointer table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED((save.TABSIZ + 2), save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYSETD(
        b"NEWNAME",
        (save.I as f64),
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(POINTERTABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETD:  value table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED(save.TABSIZ, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYSETD(
        b"NEWNAME",
        (save.I as f64),
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //     SYTRND:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYTRND:  first index < 1", ctx)?;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZED(save.TABSIZ, save.SYVALS.as_slice_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETD(
                &save.NAME,
                (save.I as f64),
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = 1;
            save.XVALS[save.I] = save.I as f64;

            save.I += m3__;
        }
    }

    spicelib::SYTRND(
        &save.NAME,
        -1,
        2,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYTRND:  first index > second index", ctx)?;

    spicelib::SYTRND(
        &save.NAME,
        2,
        1,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYTRND:  second index > symbol size", ctx)?;

    spicelib::SYTRND(
        &save.NAME,
        1,
        (save.TABSIZ + 1),
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYTRND:  non-error case: attempt transposition of values of non-existent symbol.",
        ctx,
    )?;

    spicelib::SYTRND(
        b"NONAME",
        1,
        1,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure the symbol table wasn't touched.
    //
    // Check the structure of the cells we've populated.
    //
    // The symbol name cell first:
    //
    testutil::CHCKSI(
        b"Card(SYNAMS)",
        spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?,
        b"=",
        save.TABSIZ,
        0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(save.SYNAMS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Name #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);
            testutil::CHCKSC(
                &save.TITLE,
                &save.SYNAMS[save.I],
                b"=",
                &save.XNAMES[save.I],
                OK,
                ctx,
            )?;

            save.I += m3__;
        }
    }

    //
    // Then the "pointer" cell.  The pointers are actually element
    // counts.
    //
    testutil::CHCKSI(
        b"Card(SYPTRS)",
        spicelib::CARDI(save.SYPTRS.as_slice(), ctx)?,
        b"=",
        save.TABSIZ,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAI(
        b"SYPTRS",
        save.SYPTRS.subarray(1),
        b"=",
        save.XPTRS.as_slice(),
        save.TABSIZ,
        OK,
        ctx,
    )?;

    //
    // Then the value cell.
    //
    testutil::CHCKSI(
        b"Card(SYVALS)",
        spicelib::CARDD(save.SYVALS.as_slice(), ctx)?,
        b"=",
        save.TABSIZ,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_slice(),
        save.TABSIZ,
        0.0,
        OK,
        ctx,
    )?;

    //
    // That's all, folks!
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
