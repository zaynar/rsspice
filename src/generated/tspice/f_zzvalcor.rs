//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const LNSIZE: i32 = 80;
const MAXCOR: i32 = 9;

struct SaveVars {
    CORLST: ActualCharArray,
    QNAME: Vec<u8>,
    ATTBLK: StackArray<bool, 6>,
    XATBLK: StackArray<bool, 6>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CORLST = ActualCharArray::new(LNSIZE, 1..=MAXCOR);
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut ATTBLK = StackArray::<bool, 6>::new(1..=ABATSZ);
        let mut XATBLK = StackArray::<bool, 6>::new(1..=ABATSZ);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"None"),
                Val::C(b"lT"),
                Val::C(b"Xlt"),
                Val::C(b"lt + s"),
                Val::C(b"XLt + S"),
                Val::C(b"Cn"),
                Val::C(b"xCn"),
                Val::C(b"cN  +  s"),
                Val::C(b"xCN +  S"),
            ]
            .into_iter();
            CORLST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CORLST,
            QNAME,
            ATTBLK,
            XATBLK,
        }
    }
}

//$Procedure      F_ZZVALCOR ( Test ZZVALCOR )
pub fn F_ZZVALCOR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Saved everything
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZVALCOR", ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Reject stellar aberration correction without light time correction",
        ctx,
    )?;

    spicelib::ZZVALCOR(b"S", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b" s", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b"+s", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b" + s", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b" + S", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Reject relativistic light time correction", ctx)?;

    spicelib::ZZVALCOR(b"RLT", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b" rlT", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b"xRLT", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b" XrlT", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b"RLT + S", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::ZZVALCOR(b"xRLT + s", save.ATTBLK.as_slice_mut(), ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Check attribute block for each normal aberration correction flag.",
        ctx,
    )?;

    for I in 1..=MAXCOR {
        //
        // Parse the correction.
        //
        spicelib::ZZVALCOR(&save.CORLST[I], save.ATTBLK.as_slice_mut(), ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set up the expected attribute block.
        //
        for J in 1..=ABATSZ {
            save.XATBLK[J] = false;
        }

        //
        // Set the expected geometric element.
        //
        save.XATBLK[GEOIDX] = spicelib::MATCHI(&save.CORLST[I], b"*NONE*", b"*", b"?", ctx);

        //
        // Set the expected transmission element.
        //
        save.XATBLK[XMTIDX] = spicelib::MATCHI(&save.CORLST[I], b"*X*", b"*", b"?", ctx);

        //
        // Set the expected stellar aberration element.
        //
        save.XATBLK[STLIDX] = spicelib::MATCHI(&save.CORLST[I], b"*+*S*", b"*", b"?", ctx);

        //
        // Set the expected relativistic correction element.
        //
        save.XATBLK[RELIDX] = spicelib::MATCHI(&save.CORLST[I], b"*RLT*", b"*", b"?", ctx);

        //
        // Set the expected light time and converted Newtonian
        // elements.
        //
        if spicelib::MATCHI(&save.CORLST[I], b"*LT*", b"*", b"?", ctx) {
            //
            // The light time element should be set; the converged
            // Newtonian element should not.
            //
            save.XATBLK[LTIDX] = true;
            save.XATBLK[CNVIDX] = false;
        } else if spicelib::MATCHI(&save.CORLST[I], b"*CN*", b"*", b"?", ctx) {
            save.XATBLK[LTIDX] = true;
            save.XATBLK[CNVIDX] = true;
        } else {
            save.XATBLK[LTIDX] = false;
            save.XATBLK[CNVIDX] = false;
        }

        //
        // Check each element of the attribute block.
        //
        for J in 1..=ABATSZ {
            fstr::assign(&mut save.QNAME, save.CORLST.get(I));
            spicelib::SUFFIX(b"/", 0, &mut save.QNAME);
            spicelib::SUFFIX(b"attblk elt. #", 1, &mut save.QNAME);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.QNAME.to_vec(), b"#", J, &mut save.QNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.QNAME, save.ATTBLK[J], save.XATBLK[J], OK, ctx)?;
        }
    }

    Ok(())
}
