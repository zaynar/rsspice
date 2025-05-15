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

struct SaveVars {
    CORLST: ActualCharArray,
    CONV: StackArray<bool, 15>,
    FIRST: bool,
    GEO: StackArray<bool, 15>,
    LT: StackArray<bool, 15>,
    REL: StackArray<bool, 15>,
    STL: StackArray<bool, 15>,
    XMIT: StackArray<bool, 15>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CORLST = ActualCharArray::new(CORLEN, 1..=NABCOR);
        let mut CONV = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut FIRST: bool = false;
        let mut GEO = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut LT = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut REL = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut STL = StackArray::<bool, 15>::new(1..=NABCOR);
        let mut XMIT = StackArray::<bool, 15>::new(1..=NABCOR);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CN"),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::C(b"CN+S"),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::C(b"LT"),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::C(b"LT+S"),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::C(b"NONE"),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::C(b"RL"),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::C(b"RL+S"),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::C(b"S"),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::C(b"XCN"),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::C(b"XCN+S"),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::C(b"XLT"),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::C(b"XLT+S"),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::C(b"XRL"),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::C(b"XRL+S"),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::L(false),
                Val::L(true),
                Val::L(true),
                Val::C(b"XS"),
                Val::L(false),
                Val::L(false),
                Val::L(true),
                Val::L(false),
                Val::L(true),
                Val::L(false),
            ]
            .into_iter();
            for I in intrinsics::range(1, NABCOR, 1) {
                fstr::assign(CORLST.get_mut(I), clist.next().unwrap().into_str());
                GEO[I] = clist.next().unwrap().into_bool();
                LT[I] = clist.next().unwrap().into_bool();
                STL[I] = clist.next().unwrap().into_bool();
                CONV[I] = clist.next().unwrap().into_bool();
                XMIT[I] = clist.next().unwrap().into_bool();
                REL[I] = clist.next().unwrap().into_bool();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;

        Self {
            CORLST,
            CONV,
            FIRST,
            GEO,
            LT,
            REL,
            STL,
            XMIT,
        }
    }
}

//$Procedure ZZPRSCOR ( Parse aberration correction )
pub fn ZZPRSCOR(ABCORR: &[u8], ATTBLK: &mut [bool], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ATTBLK = DummyArrayMut::new(ATTBLK, 1..);
    let mut TMPCOR = [b' '; CORLEN as usize];
    let mut LOC: i32 = 0;
    let mut ORDVEC = StackArray::<i32, 15>::new(1..=NABCOR);

    //
    // SPICELIB functions
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
    // It is recommended that, for maintainability, the correction
    // strings be kept in increasing order in this list.  However,
    // this routine does not rely on the strings being ordered
    // in this data statement:  the strings and associated values
    // are ordered at run time.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    if save.FIRST {
        //
        // The first time this routine is called, we sort the
        // aberration correction strings and the associated flag
        // lists.  This ensures we have an ordered list suitable
        // for a binary search.
        //
        // Find the sorted order of the aberration correction strings.
        //
        ORDERC(save.CORLST.as_arg(), NABCOR, ORDVEC.as_slice_mut());

        //
        // Put the aberration correction strings and the associated
        // arrays into increasing order.
        //
        REORDC(ORDVEC.as_slice_mut(), NABCOR, save.CORLST.as_arg_mut());

        REORDL(ORDVEC.as_slice_mut(), NABCOR, save.GEO.as_slice_mut());
        REORDL(ORDVEC.as_slice_mut(), NABCOR, save.LT.as_slice_mut());
        REORDL(ORDVEC.as_slice_mut(), NABCOR, save.STL.as_slice_mut());
        REORDL(ORDVEC.as_slice_mut(), NABCOR, save.CONV.as_slice_mut());
        REORDL(ORDVEC.as_slice_mut(), NABCOR, save.XMIT.as_slice_mut());
        REORDL(ORDVEC.as_slice_mut(), NABCOR, save.REL.as_slice_mut());

        save.FIRST = false;
    }

    //
    // Obtain a blank-free, upper-case copy of the aberration
    // correction string.
    //
    LJUCRS(0, ABCORR, &mut TMPCOR, ctx);

    //
    // Search the list for the aberration correction string.
    //
    LOC = BSRCHC(&TMPCOR, NABCOR, save.CORLST.as_arg());

    if (LOC == 0) {
        CHKIN(b"ZZPRSCOR", ctx)?;
        SETMSG(
            b"Aberration correction specification # is not recognized.",
            ctx,
        );
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"ZZPRSCOR", ctx)?;
        return Ok(());
    }

    //
    // Set the output flags.
    //
    ATTBLK[GEOIDX] = save.GEO[LOC];
    ATTBLK[LTIDX] = save.LT[LOC];
    ATTBLK[STLIDX] = save.STL[LOC];
    ATTBLK[CNVIDX] = save.CONV[LOC];
    ATTBLK[XMTIDX] = save.XMIT[LOC];
    ATTBLK[RELIDX] = save.REL[LOC];

    Ok(())
}
