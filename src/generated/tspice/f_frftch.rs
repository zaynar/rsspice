//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NINERT: i32 = 21;
const NNINRT: i32 = 124;
const LBCELL: i32 = -5;
const FRNMLN: i32 = 32;
const KVNMLN: i32 = 32;
const BUFSIZ: i32 = 5000;
const MAXBLT: i32 = (NINERT + NNINRT);
const MAXFRM: i32 = MAXBLT;
const LNSIZE: i32 = 400;
const NCL: i32 = 6;
const LBPOOL: i32 = -5;
const MAXBFR: i32 = (MAXBLT + 1);

struct SaveVars {
    FRNAME: Vec<u8>,
    KVNAME: Vec<u8>,
    NAMES: ActualCharArray,
    TITLE: Vec<u8>,
    BIGN: i32,
    CENTRD: StackArray<i32, 145>,
    CLSIDX: i32,
    CTRS: StackArray<i32, 145>,
    FRCLSS: i32,
    FRCLID: i32,
    FRCODE: i32,
    FRCENT: i32,
    FRMID: i32,
    I: i32,
    IDCDES: StackArray<i32, 145>,
    IDSET: ActualArray<i32>,
    IDLIST: ActualArray<i32>,
    N: i32,
    NCLASS: StackArray<i32, 6>,
    NCOUNT: i32,
    TYPES: StackArray<i32, 145>,
    TYPIDS: StackArray<i32, 145>,
    XIDSET: ActualArray<i32>,
    BNMLST: StackArray<i32, 146>,
    BNMPOL: StackArray<i32, 152>,
    BNMNMS: ActualCharArray,
    BNMIDX: StackArray<i32, 146>,
    BIDLST: StackArray<i32, 146>,
    BIDPOL: StackArray<i32, 152>,
    BIDIDS: StackArray<i32, 146>,
    BIDIDX: StackArray<i32, 146>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRNAME = vec![b' '; FRNMLN as usize];
        let mut KVNAME = vec![b' '; KVNMLN as usize];
        let mut NAMES = ActualCharArray::new(FRNMLN, 1..=MAXBLT);
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut BIGN: i32 = 0;
        let mut CENTRD = StackArray::<i32, 145>::new(1..=MAXBLT);
        let mut CLSIDX: i32 = 0;
        let mut CTRS = StackArray::<i32, 145>::new(1..=MAXBLT);
        let mut FRCLSS: i32 = 0;
        let mut FRCLID: i32 = 0;
        let mut FRCODE: i32 = 0;
        let mut FRCENT: i32 = 0;
        let mut FRMID: i32 = 0;
        let mut I: i32 = 0;
        let mut IDCDES = StackArray::<i32, 145>::new(1..=MAXBLT);
        let mut IDSET = ActualArray::<i32>::new(LBCELL..=BUFSIZ);
        let mut IDLIST = ActualArray::<i32>::new(1..=BUFSIZ);
        let mut N: i32 = 0;
        let mut NCLASS = StackArray::<i32, 6>::new(1..=NCL);
        let mut NCOUNT: i32 = 0;
        let mut TYPES = StackArray::<i32, 145>::new(1..=MAXBLT);
        let mut TYPIDS = StackArray::<i32, 145>::new(1..=MAXBLT);
        let mut XIDSET = ActualArray::<i32>::new(LBCELL..=BUFSIZ);
        let mut BNMLST = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BNMPOL = StackArray::<i32, 152>::new(LBPOOL..=MAXBFR);
        let mut BNMNMS = ActualCharArray::new(FRNMLN, 1..=MAXBFR);
        let mut BNMIDX = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BIDLST = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BIDPOL = StackArray::<i32, 152>::new(LBPOOL..=MAXBFR);
        let mut BIDIDS = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BIDIDX = StackArray::<i32, 146>::new(1..=MAXBFR);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(0),
                Val::I(5),
                Val::I(6),
                Val::I(7),
                Val::I(8),
                Val::I(2),
            ]
            .into_iter();
            NCLASS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FRNAME,
            KVNAME,
            NAMES,
            TITLE,
            BIGN,
            CENTRD,
            CLSIDX,
            CTRS,
            FRCLSS,
            FRCLID,
            FRCODE,
            FRCENT,
            FRMID,
            I,
            IDCDES,
            IDSET,
            IDLIST,
            N,
            NCLASS,
            NCOUNT,
            TYPES,
            TYPIDS,
            XIDSET,
            BNMLST,
            BNMPOL,
            BNMNMS,
            BNMIDX,
            BIDLST,
            BIDPOL,
            BIDIDS,
            BIDIDX,
        }
    }
}

//$Procedure      F_FRFTCH ( Family of tests for frame fetch APIs )
pub fn F_FRFTCH(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Built-in frame hashes returned by ZZFDAT.
    //

    //
    // Saved variables
    //
    // Save all variables to avoid CSPICE stack overflow
    // problems.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_FRFTCH", ctx)?;

    //**********************************************************************
    //
    //
    //     BLTFRM test cases
    //
    //
    //**********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"BLTFRM test setup: get data for built-in frames.", ctx)?;

    //
    // Fetch built-in frame attributes.
    //
    save.NCOUNT = (NINERT + NNINRT);

    spicelib::ZZFDAT(
        save.NCOUNT,
        MAXBFR,
        save.NAMES.as_arg_mut(),
        save.IDCDES.as_slice_mut(),
        save.CTRS.as_slice_mut(),
        save.TYPES.as_slice_mut(),
        save.TYPIDS.as_slice_mut(),
        save.CENTRD.as_slice_mut(),
        save.BNMLST.as_slice_mut(),
        save.BNMPOL.as_slice_mut(),
        save.BNMNMS.as_arg_mut(),
        save.BNMIDX.as_slice_mut(),
        save.BIDLST.as_slice_mut(),
        save.BIDPOL.as_slice_mut(),
        save.BIDIDS.as_slice_mut(),
        save.BIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize sets.
    //
    spicelib::SSIZEI(MAXBLT, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZEI(MAXBLT, save.XIDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    //
    //     Check the ability of BLTFRM to fetch all IDs of built-in
    //     frames of a given class.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(
                &mut save.TITLE,
                b"BLTFRM test: fetch class <class> built-in frames.",
            );
            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"<class>",
                save.CLSIDX,
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            //---- Case -------------------------------------------------------------
            //
            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Empty the sets.
            //
            spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SCARDI(0, save.XIDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Get the expected result set.
            //
            save.N = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCOUNT;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (save.TYPES[save.I] == save.CLSIDX) {
                        spicelib::APPNDI(save.IDCDES[save.I], save.XIDSET.as_slice_mut(), ctx)?;
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        save.N = (save.N + 1);
                    }

                    save.I += m3__;
                }
            }

            spicelib::VALIDI(MAXBLT, save.N, save.XIDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Now fetch from BLTFRM the built-in frames of class CLSIDX.
            //
            spicelib::BLTFRM(save.CLSIDX, save.IDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check set cardinality first.
            //
            testutil::CHCKSI(
                b"Card(IDSET)",
                spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
                b"=",
                spicelib::CARDI(save.XIDSET.as_slice(), ctx)?,
                0,
                OK,
                ctx,
            )?;

            if *OK {
                //
                // Check set contents.
                //
                testutil::CHCKAI(
                    b"IDSET",
                    save.IDSET.as_slice(),
                    b"=",
                    save.XIDSET.as_slice(),
                    spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
                    OK,
                    ctx,
                )?;
            }

            save.CLSIDX += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //
    //     Check the ability of BLTFRM to fetch all IDs of built-in
    //     frames.
    //
    fstr::assign(
        &mut save.TITLE,
        b"BLTFRM test: fetch IDs of all built-in frames.",
    );

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Empty the sets.
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDI(0, save.XIDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the expected result set.
    //
    save.N = save.NCOUNT;

    spicelib::MOVEI(
        save.IDCDES.as_slice(),
        save.NCOUNT,
        save.XIDSET.subarray_mut(1),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::VALIDI(MAXBLT, save.N, save.XIDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now fetch the inertial, built-in frames from BLTFRM.
    //
    spicelib::BLTFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check set cardinality first.
    //
    testutil::CHCKSI(
        b"Card(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        spicelib::CARDI(save.XIDSET.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    if *OK {
        //
        // Check set contents.
        //
        testutil::CHCKAI(
            b"IDSET",
            save.IDSET.as_slice(),
            b"=",
            save.XIDSET.as_slice(),
            spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
            OK,
            ctx,
        )?;
    }

    //
    //
    // BLTFRM error cases
    //
    //

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"BLTFRM: bad frame class.", ctx)?;

    //
    // Class 0:
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BLTFRM(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMECLASS)", OK, ctx)?;

    //
    // Class -2:
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BLTFRM(-2, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMECLASS)", OK, ctx)?;

    //
    // Class 7:
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::BLTFRM(7, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMECLASS)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"BLTFRM: output set too small.", ctx)?;

    //
    // Set the frame ID set size to 3.
    //
    spicelib::SSIZEI(3, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try to fetch the built-in inertial frames.
    //
    spicelib::BLTFRM(INERTL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(SETTOOSMALL)", OK, ctx)?;

    //**********************************************************************
    //
    //
    //     KPLFRM test cases
    //
    //
    //**********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: setup: insert frame specs into pool.", ctx)?;

    //
    // Create a set of frame specifications and insert these into
    // the kernel pool. Frame classes 2-6 will be represented in
    // this set.
    //

    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
                    spicelib::REPMI(
                        &save.FRNAME.to_vec(),
                        b"<class>",
                        save.CLSIDX,
                        &mut save.FRNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);
                    //
                    // Insert the frame ID assignment into the pool.
                    //
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Create a frame name assignment and insert this
                    // into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<ID>_NAME");
                    spicelib::REPMI(
                        &save.KVNAME.to_vec(),
                        b"<ID>",
                        save.FRMID,
                        &mut save.KVNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.FRNAME), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame class into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>_CLASS");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCLSS = save.CLSIDX;
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCLSS], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame class ID into the pool. Make
                    // the class ID the negative of the ID.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>_CLASS_ID");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCLID = -save.FRMID;
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCLID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame center into the pool. Make
                    // the center 10x the ID.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>_CENTER");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCENT = (10 * save.FRMID);
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCENT], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //
    //     Check the ability of KPLFRM to fetch all IDs of kernel pool
    //     frames of a given class.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(
                &mut save.TITLE,
                b"KPLFRM test: fetch class <class> kernel pool frames.",
            );
            spicelib::REPMI(
                &save.TITLE.to_vec(),
                b"<class>",
                save.CLSIDX,
                &mut save.TITLE,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            //---- Case -------------------------------------------------------------
            //
            testutil::TCASE(&save.TITLE, ctx)?;

            //
            // Initialize the sets.
            //
            spicelib::SSIZEI(MAXFRM, save.IDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SSIZEI(MAXFRM, save.XIDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Get the expected result set.
            //
            save.N = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.FRCODE = ((100000 * save.CLSIDX) + save.I);

                    spicelib::APPNDI(save.FRCODE, save.XIDSET.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.N = (save.N + 1);

                    save.I += m3__;
                }
            }

            spicelib::VALIDI(MAXFRM, save.N, save.XIDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Now fetch from KPLFRM the IDs of the kernel pool frames
            // of class CLSIDX.
            //
            spicelib::KPLFRM(save.CLSIDX, save.IDSET.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check set cardinality first.
            //
            testutil::CHCKSI(
                b"Card(IDSET)",
                spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
                b"=",
                spicelib::CARDI(save.XIDSET.as_slice(), ctx)?,
                0,
                OK,
                ctx,
            )?;

            if *OK {
                //
                // Check set contents.
                //
                testutil::CHCKAI(
                    b"IDSET",
                    save.IDSET.as_slice(),
                    b"=",
                    save.XIDSET.as_slice(),
                    spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
                    OK,
                    ctx,
                )?;
            }

            save.CLSIDX += m3__;
        }
    }

    //
    //---- Case -------------------------------------------------------------
    //
    //
    //     Check the ability of KPLFRM to fetch all IDs of kernel pool
    //     frames.
    //
    testutil::TCASE(b"KPLFRM: fetch IDs of all loaded frames.", ctx)?;

    //
    // Empty the sets.
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDI(0, save.XIDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Form a set containing the IDs of all loaded frames.
    //
    save.N = 0;

    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);

                    spicelib::APPNDI(save.FRMID, save.XIDSET.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.N = (save.N + 1);

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    spicelib::VALIDI(MAXFRM, save.N, save.XIDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now fetch from KPLFRM the IDs of the kernel pool frames.
    //
    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check set cardinality first.
    //
    testutil::CHCKSI(
        b"Card(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        spicelib::CARDI(save.XIDSET.as_slice(), ctx)?,
        0,
        OK,
        ctx,
    )?;

    if *OK {
        //
        // Check set contents.
        //
        testutil::CHCKAI(
            b"IDSET",
            save.IDSET.as_slice(),
            b"=",
            save.XIDSET.as_slice(),
            spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
            OK,
            ctx,
        )?;
    }

    //
    // Non-error exceptions:
    //

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: make sure no error is signaled on an attempt to fetch IDs of class 1 frames. The returned set should be empty.", ctx)?;

    spicelib::SSIZEI(MAXFRM, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(INERTL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check set cardinality.
    //
    testutil::CHCKSI(
        b"Card(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: make sure no error is signaled and no frames are found if only FRAME_<name> assignments are present.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of frame specifications and insert these into
    // the kernel pool. Frame classes 2-6 will be represented in
    // this set.
    //

    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
                    spicelib::REPMI(
                        &save.FRNAME.to_vec(),
                        b"<class>",
                        save.CLSIDX,
                        &mut save.FRNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);
                    //
                    // Insert the frame ID assignment into the pool.
                    //
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    //
    // Fetch all of the frames. We shouldn't find any.
    //
    spicelib::SSIZEI(MAXFRM, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"CARDI(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: make sure no error is signaled and no frames are found if only FRAME_<name> and FRAME_<ID code>_NAME assignments are present.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of frame specifications and insert these into
    // the kernel pool. Frame classes 2-6 will be represented in
    // this set.
    //

    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
                    spicelib::REPMI(
                        &save.FRNAME.to_vec(),
                        b"<class>",
                        save.CLSIDX,
                        &mut save.FRNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);
                    //
                    // Insert the frame ID assignment into the pool.
                    //
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Create a frame name assignment and insert this
                    // into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<ID>_NAME");
                    spicelib::REPMI(
                        &save.KVNAME.to_vec(),
                        b"<ID>",
                        save.FRMID,
                        &mut save.KVNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.FRNAME), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    //
    // Fetch all of the frames. We shouldn't find any.
    //
    spicelib::SSIZEI(MAXFRM, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"CARDI(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: make sure no error is signaled and no frames are found if only FRAME_<name> and FRAME_<name>_CLASS assignments are present.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of frame specifications and insert these into
    // the kernel pool. Frame classes 2-6 will be represented in
    // this set.
    //

    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
                    spicelib::REPMI(
                        &save.FRNAME.to_vec(),
                        b"<class>",
                        save.CLSIDX,
                        &mut save.FRNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);
                    //
                    // Insert the frame ID assignment into the pool.
                    //
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame class into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>_CLASS");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCLSS = save.CLSIDX;
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCLSS], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    //
    // Fetch all of the frames. We shouldn't find any.
    //
    spicelib::SSIZEI(MAXFRM, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"CARDI(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: make sure no error is signaled and no frames are found if only FRAME_<name> and FRAME_<ID code>_CLASS assignments are present.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of frame specifications and insert these into
    // the kernel pool. Frame classes 2-6 will be represented in
    // this set.
    //

    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
                    spicelib::REPMI(
                        &save.FRNAME.to_vec(),
                        b"<class>",
                        save.CLSIDX,
                        &mut save.FRNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);
                    //
                    // Insert the frame ID assignment into the pool.
                    //
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame class into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<code>_CLASS");
                    spicelib::REPMI(
                        &save.KVNAME.to_vec(),
                        b"<code>",
                        save.FRMID,
                        &mut save.KVNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCLSS = save.CLSIDX;
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCLSS], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    //
    // Fetch all of the frames. We shouldn't find any.
    //
    spicelib::SSIZEI(MAXFRM, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"CARDI(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: make sure no error is signaled and no frames are found if FRAME_<name> and FRAME_<ID code>_NAME assignments are present and the frame names don\'t match.", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a set of frame specifications and insert these into
    // the kernel pool. Frame classes 2-6 will be represented in
    // this set.
    //

    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
                    spicelib::REPMI(
                        &save.FRNAME.to_vec(),
                        b"<class>",
                        save.CLSIDX,
                        &mut save.FRNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);
                    //
                    // Insert the frame ID assignment into the pool.
                    //
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert a non-matching frame name into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<code>_NAME");
                    spicelib::REPMI(
                        &save.KVNAME.to_vec(),
                        b"<code>",
                        save.FRMID,
                        &mut save.KVNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(b"XXX"), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    //
    // Fetch all of the frames. We shouldn't find any.
    //
    spicelib::SSIZEI(MAXFRM, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"CARDI(IDSET)",
        spicelib::CARDI(save.IDSET.as_slice(), ctx)?,
        b"=",
        0,
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Check frame buffer handling for large frame counts.", ctx)?;

    spicelib::SSIZEI(BUFSIZ, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Insert a large number of frame specs into the kernel pool.
    // The specs don't have to be complete---only the parts checked
    // by KPLFRM are needed.
    //
    save.BIGN = BUFSIZ;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.BIGN;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.CLSIDX = (intrinsics::MOD(save.I, 3) + 1);

            fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
            spicelib::REPMI(
                &save.FRNAME.to_vec(),
                b"<class>",
                save.CLSIDX,
                &mut save.FRNAME,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
            spicelib::REPMC(
                &save.KVNAME.to_vec(),
                b"<name>",
                &save.FRNAME,
                &mut save.KVNAME,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create an ID code for this frame.
            //
            save.FRMID = ((100000 * save.CLSIDX) + save.I);
            //
            // Insert the frame ID assignment into the pool.
            //
            spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Store the frame ID we just created.
            //
            save.IDLIST[save.I] = save.FRMID;

            //
            // Insert the assignment of the frame name into
            // the kernel pool.
            //
            fstr::assign(&mut save.KVNAME, b"FRAME_<code>_NAME");
            spicelib::REPMI(
                &save.KVNAME.to_vec(),
                b"<code>",
                save.FRMID,
                &mut save.KVNAME,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.FRNAME), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Insert the frame's class into the pool.
            //
            fstr::assign(&mut save.KVNAME, b"FRAME_<code>_CLASS");
            spicelib::REPMI(
                &save.KVNAME.to_vec(),
                b"<code>",
                save.FRMID,
                &mut save.KVNAME,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PIPOOL(&save.KVNAME, 1, &[save.CLSIDX], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Now fetch the frame names.
    //
    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.I = spicelib::CARDI(save.IDSET.as_slice(), ctx)?;

    testutil::CHCKSI(b"IDSET card", save.I, b"=", save.BIGN, 0, OK, ctx)?;

    //
    // Sort the stored ID codes.
    //
    spicelib::SHELLI(save.BIGN, save.IDLIST.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure that the stored ID set matches the
    // set returned by KPLFRM.
    //
    testutil::CHCKAI(
        b"IDSET",
        save.IDSET.subarray(1),
        b"=",
        save.IDLIST.as_slice(),
        save.BIGN,
        OK,
        ctx,
    )?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Setup: restore frame specs.", ctx)?;

    //
    // Restore the frame specifications for use in later test cases.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        save.CLSIDX = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NCLASS[save.CLSIDX];
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(&mut save.FRNAME, b"FRCLASS_<class>_FRAME_<n>");
                    spicelib::REPMI(
                        &save.FRNAME.to_vec(),
                        b"<class>",
                        save.CLSIDX,
                        &mut save.FRNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::REPMI(&save.FRNAME.to_vec(), b"<n>", save.I, &mut save.FRNAME, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    //
                    // Create an ID code for this frame.
                    //
                    save.FRMID = ((100000 * save.CLSIDX) + save.I);
                    //
                    // Insert the frame ID assignment into the pool.
                    //
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRMID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Create a frame name assignment and insert this
                    // into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<ID>_NAME");
                    spicelib::REPMI(
                        &save.KVNAME.to_vec(),
                        b"<ID>",
                        save.FRMID,
                        &mut save.KVNAME,
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::PCPOOL(&save.KVNAME, 1, CharArray::from_ref(&save.FRNAME), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame class into the pool.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>_CLASS");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCLSS = save.CLSIDX;
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCLSS], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame class ID into the pool. Make
                    // the class ID the negative of the ID.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>_CLASS_ID");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCLID = -save.FRMID;
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCLID], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    //
                    // Insert the frame center into the pool. Make
                    // the center 10x the ID.
                    //
                    fstr::assign(&mut save.KVNAME, b"FRAME_<name>_CENTER");
                    spicelib::REPMC(
                        &save.KVNAME.to_vec(),
                        b"<name>",
                        &save.FRNAME,
                        &mut save.KVNAME,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.FRCENT = (10 * save.FRMID);
                    spicelib::PIPOOL(&save.KVNAME, 1, &[save.FRCENT], ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.I += m3__;
                }
            }

            save.CLSIDX += m3__;
        }
    }

    //
    //     KPLFRM error cases
    //
    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: bad frame class.", ctx)?;

    //
    // Class 0:
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMECLASS)", OK, ctx)?;

    //
    // Class -2:
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(-2, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMECLASS)", OK, ctx)?;

    //
    // Class 7:
    //
    spicelib::SCARDI(0, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KPLFRM(7, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADFRAMECLASS)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"KPLFRM: output set too small.", ctx)?;

    //
    // Set the frame ID set size to 3.
    //
    spicelib::SSIZEI(3, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Try to fetch all kernel pool frames.
    //
    spicelib::KPLFRM(ALL, save.IDSET.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(SETTOOSMALL)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
