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
const VALLEN: i32 = 50;

struct SaveVars {
    CVALS: ActualCharArray,
    IVS: ActualCharArray,
    NAME: Vec<u8>,
    NIVS: ActualCharArray,
    NEWNAM: Vec<u8>,
    SYNAMS: ActualCharArray,
    XNAMES: ActualCharArray,
    SYVALS: ActualCharArray,
    VAL: Vec<u8>,
    XVAL: Vec<u8>,
    XVALS: ActualCharArray,
    TITLE: Vec<u8>,
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
        let mut CVALS = ActualCharArray::new(VALLEN, 1..=MAXVAL);
        let mut IVS = ActualCharArray::new(VALLEN, 1..=MAXVAL);
        let mut NAME = vec![b' '; NAMLEN as usize];
        let mut NIVS = ActualCharArray::new(VALLEN, 1..=MAXVAL);
        let mut NEWNAM = vec![b' '; NAMLEN as usize];
        let mut SYNAMS = ActualCharArray::new(NAMLEN, LBCELL..=MAXTAB);
        let mut XNAMES = ActualCharArray::new(NAMLEN, 1..=MAXTAB);
        let mut SYVALS = ActualCharArray::new(VALLEN, LBCELL..=MAXVAL);
        let mut VAL = vec![b' '; VALLEN as usize];
        let mut XVAL = vec![b' '; VALLEN as usize];
        let mut XVALS = ActualCharArray::new(VALLEN, 1..=MAXVAL);
        let mut TITLE = vec![b' '; MSGLEN as usize];
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
            CVALS,
            IVS,
            NAME,
            NIVS,
            NEWNAM,
            SYNAMS,
            XNAMES,
            SYVALS,
            VAL,
            XVAL,
            XVALS,
            TITLE,
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

//$Procedure F_SYMTBC ( Test SPICELIB character symbol table routines )
pub fn F_SYMTBC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_SYMTBC", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create arrays of values that take the place of the numeric values used in tests for the integer and d.p. symbol table routines.", ctx)?;

    //
    // We use the symbol name creation utility to create the values.
    //
    //
    // The array IVS contains values that replace integers.  Note
    // that these values are *not* integers in string form.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXVAL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, MAXVAL, &mut save.IVS[save.I], ctx)?;

            save.I += m3__;
        }
    }

    //
    // The array NIVS contains values whose order is the reverse of
    // the order of the elements of IVS.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXVAL;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = ((MAXVAL + 1) - save.I);

            fstr::assign(save.NIVS.get_mut(save.I), save.IVS.get(save.J));

            save.I += m3__;
        }
    }

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYSETC test:  Populate a simple symbol table with scalar-valued symbols.",
        ctx,
    )?;

    //
    // Initialize the symbol table.
    //
    spicelib::SSIZEC(MAXTAB, save.SYNAMS.as_arg_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI(MAXTAB, save.SYPTRS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEC(MAXVAL, save.SYVALS.as_arg_mut(), ctx)?;
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
            fstr::assign(save.XVALS.get_mut(save.I), save.NIVS.get(save.I));

            //
            // Associate the value NIVS(I) with the Ith symbol.  Add this
            // symbol to the table.
            //
            spicelib::SYSETC(
                &save.NAME,
                &save.NIVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETC test:  Populate a simple symbol table with scalar-valued symbols.  Insert symbols in reverse order.", ctx)?;

    //
    // Clear out the symbol table.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
            // Associate the value NIVS(J) with the Jth symbol.  Add this
            // symbol to the table.
            //
            spicelib::SYSETC(
                &save.XNAMES[save.J],
                &save.NIVS[save.J],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYPUTC test:  Populate a simple symbol table with scalar-valued symbols.",
        ctx,
    )?;

    //
    // This is essentially a reprise of the previous SYSETC test.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
            fstr::assign(save.XVALS.get_mut(save.I), save.NIVS.get(save.I));

            //
            // Associate the value NIVS(I) with the Ith symbol.  Add this
            // symbol to the table.
            //
            spicelib::SYPUTC(
                &save.NAME,
                save.XVALS.subarray(save.I),
                1,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        MAXTAB,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYFETC test:  Fetch names from symbol table with scalar-valued symbols.",
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

            spicelib::SYFETC(
                save.I,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
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

    spicelib::SYFETC(
        (MAXTAB + 1),
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_arg(),
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
        b"SYGETC test:  Fetch values from symbol table with scalar-valued symbols.",
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

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                CharArrayMut::from_mut(&mut save.VAL),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Value of symbol #");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            testutil::CHCKSC(&save.TITLE, &save.VAL, b"=", &save.XVALS[save.I], OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Also look for a symbol we know isn't there.
    //
    fstr::assign(&mut save.NAME, b"NOT_THERE");

    spicelib::SYGETC(
        &save.NAME,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_arg(),
        &mut save.NVALS,
        CharArrayMut::from_mut(&mut save.VAL),
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
        b"SYRENC test:  Rename symbols in a simple symbol table with scalar-valued symbols.",
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

            spicelib::SYGETC(
                &save.NAME,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                CharArrayMut::from_mut(&mut save.XVAL),
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
            spicelib::SYRENC(
                &save.NAME,
                &save.NEWNAM,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the old symbol is gone.
            //
            spicelib::SYGETC(
                &save.NAME,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                CharArrayMut::from_mut(&mut save.VAL),
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
            spicelib::SYGETC(
                &save.NEWNAM,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                CharArrayMut::from_mut(&mut save.VAL),
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
            testutil::CHCKSC(b"VAL", &save.VAL, b"=", &save.XVAL, OK, ctx)?;

            //
            // Change the name back to its original value.
            //
            spicelib::SYRENC(
                &save.NEWNAM,
                &save.NAME,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
        b"SYDELC test:  Delete symbols from symbol table with scalar-valued symbols.",
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

            spicelib::SYDELC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Make sure the symbol is gone.
            //
            fstr::assign(&mut save.TITLE, b"Was (deleted symbol) name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                CharArrayMut::from_mut(&mut save.VAL),
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

                    spicelib::SYGETC(
                        &save.XNAMES[save.J],
                        save.SYNAMS.as_arg(),
                        save.SYPTRS.as_slice(),
                        save.SYVALS.as_arg(),
                        &mut save.NVALS,
                        CharArrayMut::from_mut(&mut save.VAL),
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

                    fstr::assign(&mut save.TITLE, b"Remaining symbol # value");
                    spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

                    testutil::CHCKSC(&save.TITLE, &save.VAL, b"=", &save.XVALS[save.J], OK, ctx)?;

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
        b"SYPUTC test:  Create symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));
                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Add the symbol to the table.
            //
            spicelib::SYPUTC(
                &save.NAME,
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTC test:  Populate a simple symbol table with scalar-valued symbols.  Insert symbols in reverse order.", ctx)?;

    //
    // Clear out the symbol table.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

    spicelib::CLEARC(MAXVAL, save.XVALS.as_arg_mut());

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
            spicelib::SYPUTC(
                &save.XNAMES[save.I],
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYGETC test:  validate array-valued symbol table by fetching values associated with each symbol.", ctx)?;

    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol #");
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

                testutil::CHCKAC(
                    &save.TITLE,
                    save.CVALS.as_arg(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
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
    testutil::TCASE(b"SYNTHC test:  validate array-valued symbol table by fetching values associated with each symbol.", ctx)?;

    save.START = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(&mut save.TITLE, b"Was name # found?");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", save.I, &mut save.TITLE, ctx);

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                //
                // Get the size of the current symbol.
                //
                save.SIZE = spicelib::SYDIMC(
                    &save.XNAMES[save.I],
                    save.SYNAMS.as_arg(),
                    save.SYPTRS.as_slice(),
                    save.SYVALS.as_arg(),
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

                        spicelib::SYNTHC(
                            &save.XNAMES[save.I],
                            save.J,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_arg(),
                            &mut save.VAL,
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

                        testutil::CHCKSC(
                            &save.TITLE,
                            &save.VAL,
                            b"=",
                            &save.XVALS[((save.START + save.J) - 1)],
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

                spicelib::SYNTHC(
                    &save.XNAMES[save.I],
                    save.J,
                    save.SYNAMS.as_arg(),
                    save.SYPTRS.as_slice(),
                    save.SYVALS.as_arg(),
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
        b"SYENQC test:  Create symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));

                    spicelib::SYENQC(
                        &save.NAME,
                        &save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYPSHC test:  Create symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));

                    spicelib::SYPSHC(
                        &save.NAME,
                        &save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        save.XCARD,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPC test:  Create symbol table with array-valued symbols.  Then duplicate each symbol.", ctx)?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));

                    spicelib::SYENQC(
                        &save.NAME,
                        &save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_arg_mut(),
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

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
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

                testutil::CHCKAC(
                    &save.TITLE,
                    save.CVALS.as_arg(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
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

            spicelib::SYDUPC(
                &save.NAME,
                &save.NEWNAM,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
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

                testutil::CHCKAC(
                    &save.TITLE,
                    save.CVALS.as_arg(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
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

            spicelib::SYGETC(
                &save.NEWNAM,
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol # (2)");
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWNAM, &mut save.TITLE);

                testutil::CHCKAC(
                    &save.TITLE,
                    save.CVALS.as_arg(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
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
    testutil::TCASE(b"SYDUPC test:  Create symbol table with array-valued symbols.  Duplicate these symbols.  Negate the values of the original symbols.  Then duplicate the duplicate the symbols, overwriting the original symbols.", ctx)?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));

                    spicelib::SYENQC(
                        &save.NAME,
                        &save.XVALS[save.TO],
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_arg_mut(),
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

            spicelib::SYDUPC(
                &save.NAME,
                &save.NEWNAM,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // "Negate" the values of original symbols.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.UB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
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
                    spicelib::PREFIX(b"-", 0, &mut save.CVALS[save.J]);
                    save.J += m3__;
                }
            }

            spicelib::SYPUTC(
                &save.XNAMES[save.I],
                save.CVALS.as_arg(),
                save.NVALS,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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

            spicelib::SYDUPC(
                &save.NEWNAM,
                &save.NAME,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
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

                testutil::CHCKAC(
                    &save.TITLE,
                    save.CVALS.as_arg(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
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

            spicelib::SYGETC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
                &mut save.NVALS,
                save.CVALS.as_arg_mut(),
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.TITLE, b"Values of symbol # (2)");
                spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.NEWNAM, &mut save.TITLE);

                testutil::CHCKAC(
                    &save.TITLE,
                    save.CVALS.as_arg(),
                    b"=",
                    save.XVALS.subarray(save.START),
                    save.I,
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
    testutil::TCASE(b"SYORDC test:  Create symbol table with array-valued symbols; sort values associated with each symbol.", ctx)?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

    spicelib::CLEARC(MAXVAL, save.XVALS.as_arg_mut());

    //
    // Create the symbol table to be sorted.
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

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.VAL, save.IVS.get(save.TO));

                    spicelib::SYENQC(
                        &save.NAME,
                        &save.VAL,
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_arg_mut(),
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
    // Sort the symbol values.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = MAXTAB;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SYORDC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg_mut(),
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

                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));

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
    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        MAXTAB,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYDIMC test:  Find dimensions of symbols in symbol table with array-valued symbols.",
        ctx,
    )?;

    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

    spicelib::CLEARC(MAXVAL, save.XVALS.as_arg_mut());

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
                    fstr::assign(&mut save.VAL, save.IVS.get(save.TO));

                    spicelib::SYENQC(
                        &save.NAME,
                        &save.VAL,
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_arg_mut(),
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
            save.NVALS = spicelib::SYDIMC(
                &save.XNAMES[save.I],
                save.SYNAMS.as_arg(),
                save.SYPTRS.as_slice(),
                save.SYVALS.as_arg(),
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
        b"SYPOPC test:  Pop values from symbol table with array-valued symbols.",
        ctx,
    )?;

    //
    // Re-create the default array-valued symbol table.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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

            //
            // Set the values of the Ith symbol.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.I;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(save.CVALS.get_mut(save.J), save.IVS.get(save.TO));
                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Insert the Ith symbol.
            //
            spicelib::SYPUTC(
                &save.NAME,
                save.CVALS.as_arg(),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now test SYPOPC.
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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));

                    spicelib::SYPOPC(
                        &save.NAME,
                        save.SYNAMS.as_arg_mut(),
                        save.SYPTRS.as_slice_mut(),
                        save.SYVALS.as_arg_mut(),
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

                        testutil::CHCKSC(
                            &save.TITLE,
                            &save.VAL,
                            b"=",
                            &save.XVALS[save.TO],
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
            spicelib::SYPOPC(
                &save.NAME,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
        b"SYTRNC test:  Transpose values from symbol table with array-valued symbols.",
        ctx,
    )?;

    //
    // We'll create a small version of our array-valued symbol table.
    // Since we want to try all possible transpositions, there would
    // be an excessive number of calls to SYTRNC if we used the full
    // set of MAXTAB symbols.
    //
    // Create the symbol table now.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));
                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Insert the Ith symbol.
            //
            spicelib::SYPUTC(
                &save.NAME,
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
                        spicelib::SYTRNC(
                            &save.NAME,
                            save.J,
                            K,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_arg_mut(),
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        //
                        // Fetch the values of the modified symbol.
                        //
                        spicelib::SYGETC(
                            &save.NAME,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_arg(),
                            &mut save.NVALS,
                            save.CVALS.as_arg_mut(),
                            &mut save.FOUND,
                            ctx,
                        )?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSL(&save.TITLE, save.FOUND, true, OK, ctx)?;

                        if save.FOUND {
                            //
                            // If CVALS reflects the correct transposition,
                            // then we can swap elements J and K of CVALS
                            // to obtain the original symbol's values.
                            //
                            if (save.J != K) {
                                spicelib::SWAPC_ARRAY(
                                    save.CVALS.subscript(save.J),
                                    save.CVALS.subscript(K),
                                    save.CVALS.as_arg_mut(),
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

                            testutil::CHCKAC(
                                &save.TITLE,
                                save.CVALS.as_arg(),
                                b"=",
                                save.XVALS.subarray(save.START),
                                save.I,
                                OK,
                                ctx,
                            )?;
                        }

                        //
                        // Undo the transposition in the symbol table.
                        //
                        spicelib::SYTRNC(
                            &save.NAME,
                            save.J,
                            K,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_arg_mut(),
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
        b"SYSELC test:  Extract slice of values from symbol table with array-valued symbols.",
        ctx,
    )?;

    //
    // This approach used in this test case closely parallels that
    // of the SYTRNC test case above.
    //
    // We'll create a small version of our array-valued symbol table.
    // Since we want to try all possible slices, there would
    // be an excessive number of calls to SYTRNC if we used the full
    // set of MAXTAB symbols.
    //
    // Create the symbol table now.
    //
    spicelib::SCARDC(0, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SCARDC(0, save.SYVALS.as_arg_mut(), ctx)?;

    spicelib::CLEARC(MAXVAL, save.XVALS.as_arg_mut());

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
                    fstr::assign(save.XVALS.get_mut(save.TO), save.IVS.get(save.TO));
                    save.TO = (save.TO + 1);

                    save.J += m3__;
                }
            }

            //
            // Insert the Ith symbol.
            //
            spicelib::SYPUTC(
                &save.NAME,
                save.XVALS.subarray(save.START),
                save.I,
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
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
                        spicelib::SYSELC(
                            &save.NAME,
                            save.J,
                            K,
                            save.SYNAMS.as_arg(),
                            save.SYPTRS.as_slice(),
                            save.SYVALS.as_arg(),
                            save.CVALS.as_arg_mut(),
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

                            testutil::CHCKAC(
                                &save.TITLE,
                                save.CVALS.as_arg(),
                                b"=",
                                save.XVALS.subarray(((save.START + save.J) - 1)),
                                ((K - save.J) + 1),
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
    //     SYDELC:  No errors are detected by this routine.
    //
    //     SYDIMC:  No errors are detected by this routine.
    //
    //
    //     SYDUPC:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPC:  symbol name not present", ctx)?;

    spicelib::SYDUPC(
        b"NOSYMBOL",
        b"NOSYMBOL2",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOSUCHSYMBOL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPC:  name table overflow", ctx)?;

    //
    // We'll create a small symbol table so we can test the handling of
    // overflow conditions.
    //
    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((2 * save.TABSIZ), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC((2 * save.TABSIZ), save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYDUPC(
        &save.NAME,
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NAMETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPC:  pointer table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC((save.TABSIZ + 2), save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYDUPC(
        &save.NAME,
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(POINTERTABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYDUPC:  value table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(save.TABSIZ, save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYDUPC(
        &save.NAME,
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    //
    //     SYENQC:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYENQC:  value table overflow", ctx)?;

    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(save.TABSIZ, save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Enqueue a value onto the last symbol.
    //
    spicelib::SYENQC(
        &save.NAME,
        &save.NIVS[1],
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    //
    //
    //     SYFETC:  No errors are detected by this routine.
    //
    //     SYGETC:  No errors are detected by this routine.
    //
    //     SYNTHC:  No errors are detected by this routine.
    //
    //     SYORDC:  No errors are detected by this routine.
    //
    //     SYPOPC:  No errors are detected by this routine.
    //
    //
    //
    //     SYPSHC:
    //
    // --- Case: ------------------------------------------------------
    //
    //
    testutil::TCASE(b"SYPSHC:  value table overflow", ctx)?;

    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(save.TABSIZ, save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Push a value onto the last symbol.
    //
    spicelib::SYPSHC(
        &save.NAME,
        &save.NIVS[save.I],
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    //     SYPUTC:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTC:  name table overflow", ctx)?;

    //
    // We'll create a small symbol table so we can test the handling of
    // overflow conditions.
    //
    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((2 * save.TABSIZ), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC((2 * save.TABSIZ), save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTC(
        b"NEWNAME",
        save.IVS.subarray(save.I),
        1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NAMETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTC:  pointer table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC((save.TABSIZ + 2), save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTC(
        b"NEWNAME",
        save.IVS.subarray(save.I),
        1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(POINTERTABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTC:  value table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(save.TABSIZ, save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTC(
        b"NEWNAME",
        save.IVS.subarray(save.I),
        1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYPUTC: invalid value count", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC((save.TABSIZ + 1), save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYPUTC(
        b"NEWNAME",
        save.IVS.subarray(save.I),
        -1,
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDARGUMENT)", OK, ctx)?;

    //
    //     SYRENC:
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYRENC:  rename non-existent symbol", ctx)?;

    spicelib::SYRENC(
        b"NONNAME",
        b"NEWNAME",
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOSUCHSYMBOL)", OK, ctx)?;

    //
    //
    //     SYSELC:  No errors are detected by this routine.
    //
    //
    //     SYSETC:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETC:  name table overflow", ctx)?;

    //
    // We'll create a small symbol table so we can test the handling of
    // overflow conditions.
    //
    save.TABSIZ = 5;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((2 * save.TABSIZ), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC((2 * save.TABSIZ), save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYSETC(
        b"NEWNAME",
        &save.IVS[save.I],
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NAMETABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETC:  pointer table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC((save.TABSIZ + 2), save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYSETC(
        b"NEWNAME",
        &save.IVS[save.I],
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(POINTERTABLEFULL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYSETC:  value table overflow", ctx)?;

    spicelib::SSIZEC((save.TABSIZ + 1), save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI((save.TABSIZ + 1), save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(save.TABSIZ, save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    spicelib::SYSETC(
        b"NEWNAME",
        &save.IVS[save.I],
        save.SYNAMS.as_arg_mut(),
        save.SYPTRS.as_slice_mut(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUETABLEFULL)", OK, ctx)?;

    //     SYTRNC:
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYTRNC:  first index < 1", ctx)?;

    spicelib::SSIZEC(save.TABSIZ, save.SYNAMS.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(save.TABSIZ, save.SYPTRS.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(save.TABSIZ, save.SYVALS.as_arg_mut(), ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.TABSIZ;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            T_ITHSYM(save.I, save.TABSIZ, &mut save.NAME, ctx)?;

            spicelib::SYSETC(
                &save.NAME,
                &save.IVS[save.I],
                save.SYNAMS.as_arg_mut(),
                save.SYPTRS.as_slice_mut(),
                save.SYVALS.as_arg_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(save.XNAMES.get_mut(save.I), &save.NAME);
            save.XPTRS[save.I] = 1;
            fstr::assign(save.XVALS.get_mut(save.I), save.IVS.get(save.I));

            save.I += m3__;
        }
    }

    spicelib::SYTRNC(
        &save.NAME,
        -1,
        2,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYTRNC:  first index > second index", ctx)?;

    spicelib::SYTRNC(
        &save.NAME,
        2,
        1,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"SYTRNC:  second index > symbol size", ctx)?;

    spicelib::SYTRNC(
        &save.NAME,
        1,
        (save.TABSIZ + 1),
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_arg_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDINDEX)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SYTRNC:  non-error case: attempt transposition of values of non-existent symbol.",
        ctx,
    )?;

    spicelib::SYTRNC(
        b"NONAME",
        1,
        1,
        save.SYNAMS.as_arg(),
        save.SYPTRS.as_slice(),
        save.SYVALS.as_arg_mut(),
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
        spicelib::CARDC(save.SYVALS.as_arg(), ctx)?,
        b"=",
        save.TABSIZ,
        0,
        OK,
        ctx,
    )?;

    testutil::CHCKAC(
        b"SYVALS",
        save.SYVALS.subarray(1),
        b"=",
        save.XVALS.as_arg(),
        save.TABSIZ,
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
