//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXNSRF: i32 = 2000;
const SFNMLN: i32 = 36;
const CTRSIZ: i32 = 2;
const NROOM: i32 = 2003;
const LBSNGL: i32 = -5;
const KVNNAM: &[u8] = b"NAIF_SURFACE_NAME";
const KVBNAM: &[u8] = b"NAIF_SURFACE_BODY";
const KVCNAM: &[u8] = b"NAIF_SURFACE_CODE";
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const NUMLEN: i32 = 11;
const NBDNAM: i32 = 3;

struct SaveVars {
    BODLST: ActualCharArray,
    BODNAM: ActualCharArray,
    BDNAM2: ActualCharArray,
    BODSTR: Vec<u8>,
    KERNAM: ActualCharArray,
    KRNAM2: ActualCharArray,
    LABEL: Vec<u8>,
    NORNAM: ActualCharArray,
    NUMSTR: Vec<u8>,
    SRFSTR: Vec<u8>,
    EXPSTR: Vec<u8>,
    BODYID: i32,
    I: i32,
    J: i32,
    K: i32,
    KERBID: ActualArray<i32>,
    KRBID2: ActualArray<i32>,
    KERSID: ActualArray<i32>,
    KRSID2: ActualArray<i32>,
    N: i32,
    NCASE: i32,
    NKVAR: i32,
    SIDHLS: ActualArray<i32>,
    SIDPOL: ActualArray<i32>,
    SIDIDX: ActualArray<i32>,
    SNMHLS: ActualArray<i32>,
    SNMPOL: ActualArray<i32>,
    SNMIDX: ActualArray<i32>,
    SURFID: i32,
    USRCTR: StackArray<i32, 2>,
    EXTKER: bool,
    FOUND: bool,
    ISNAME: bool,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BODLST = ActualCharArray::new(BDNMLN, 1..=NBDNAM);
        let mut BODNAM = ActualCharArray::new(BDNMLN, 1..=MXNSRF);
        let mut BDNAM2 = ActualCharArray::new(BDNMLN, 1..=(MXNSRF + 3));
        let mut BODSTR = vec![b' '; BDNMLN as usize];
        let mut KERNAM = ActualCharArray::new(SFNMLN, 1..=MXNSRF);
        let mut KRNAM2 = ActualCharArray::new(SFNMLN, 1..=(MXNSRF + 3));
        let mut LABEL = vec![b' '; LNSIZE as usize];
        let mut NORNAM = ActualCharArray::new(SFNMLN, 1..=MXNSRF);
        let mut NUMSTR = vec![b' '; NUMLEN as usize];
        let mut SRFSTR = vec![b' '; SFNMLN as usize];
        let mut EXPSTR = vec![b' '; SFNMLN as usize];
        let mut BODYID: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut KERBID = ActualArray::<i32>::new(1..=MXNSRF);
        let mut KRBID2 = ActualArray::<i32>::new(1..=(MXNSRF + 3));
        let mut KERSID = ActualArray::<i32>::new(1..=MXNSRF);
        let mut KRSID2 = ActualArray::<i32>::new(1..=(MXNSRF + 3));
        let mut N: i32 = 0;
        let mut NCASE: i32 = 0;
        let mut NKVAR: i32 = 0;
        let mut SIDHLS = ActualArray::<i32>::new(1..=NROOM);
        let mut SIDPOL = ActualArray::<i32>::new(LBSNGL..=NROOM);
        let mut SIDIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut SNMHLS = ActualArray::<i32>::new(1..=NROOM);
        let mut SNMPOL = ActualArray::<i32>::new(LBSNGL..=NROOM);
        let mut SNMIDX = ActualArray::<i32>::new(1..=NROOM);
        let mut SURFID: i32 = 0;
        let mut USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut EXTKER: bool = false;
        let mut FOUND: bool = false;
        let mut ISNAME: bool = false;
        let mut UPDATE: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Mars"), Val::C(b"Phobos"), Val::C(b"Moon")].into_iter();
            BODLST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BODLST,
            BODNAM,
            BDNAM2,
            BODSTR,
            KERNAM,
            KRNAM2,
            LABEL,
            NORNAM,
            NUMSTR,
            SRFSTR,
            EXPSTR,
            BODYID,
            I,
            J,
            K,
            KERBID,
            KRBID2,
            KERSID,
            KRSID2,
            N,
            NCASE,
            NKVAR,
            SIDHLS,
            SIDPOL,
            SIDIDX,
            SNMHLS,
            SNMPOL,
            SNMIDX,
            SURFID,
            USRCTR,
            EXTKER,
            FOUND,
            ISNAME,
            UPDATE,
        }
    }
}

//$Procedure      F_SRFTRN ( Test surface name/ID mapping routines )
pub fn F_SRFTRN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_SRFTRN", ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create kernel variables", ctx)?;

    //
    // Create a set of variables with multiple bodies for a given
    // surface name.
    //
    save.NKVAR = 1998;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 3;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.K = ((save.I - 1) + save.J);

                    save.KERSID[save.K] = save.K;

                    fstr::assign(save.BODNAM.get_mut(save.K), save.BODLST.get(save.J));

                    spicelib::BODS2C(
                        &save.BODNAM[save.K],
                        &mut save.KERBID[save.K],
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

                    {
                        use f2rust_std::{
                            data::Val,
                            io::{self, Writer},
                        };

                        let internal_file = io::InternalFile::open(&mut save.NUMSTR);
                        let mut writer = io::FormattedWriter::new(internal_file, None, b"(I4.4)")?;
                        writer.start()?;
                        writer.write_i32(save.I)?;
                        writer.finish()?;
                    }

                    fstr::assign(save.KERNAM.get_mut(save.K), b"Surface # name");

                    spicelib::REPMC(
                        &save.KERNAM[save.K].to_vec(),
                        b"#",
                        &save.NUMSTR,
                        &mut save.KERNAM[save.K],
                    );

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // >>> We'll avoid overwriting
    //
    //        KERNAM
    //        KERBID
    //        KERSID
    //        BODNAM
    //
    //     in the rest of this routine. We'll use the variables
    //
    //        KRNAM2
    //        KRBID2
    //        KRSID2
    //        BDNAM2
    //
    //     for temporary kernel variable values.
    //

    //***********************************************************************
    //
    //     ZZSRFTRK tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZSRFTRK: check update status before any other ZZSRFTRN entry point has been initialized.", ctx)?;

    spicelib::ZZCTRUIN(save.USRCTR.as_slice_mut(), ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFTRK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE 0", save.UPDATE, true, OK, ctx)?;

    //
    // Make a second check.
    //
    spicelib::ZZSRFTRK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE 1", save.UPDATE, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZSRFTRK: check update status after a map update.", ctx)?;

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFTRK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE 0", save.UPDATE, true, OK, ctx)?;

    //
    // Make a second check.
    //
    spicelib::ZZSRFTRK(save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"UPDATE 1", save.UPDATE, false, OK, ctx)?;

    //***********************************************************************
    //
    //     ZZSRFKER tests
    //
    //***********************************************************************

    //
    // Most of the functionality of ZZSRFKER will be exercised by
    // tests of higher-level routines. We'll test the error handling
    // here.
    //

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZSRFKER: check EXTKER when map is not present", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFKER(
        save.KERNAM.as_arg_mut(),
        save.NORNAM.as_arg_mut(),
        save.KERSID.as_slice_mut(),
        save.KERBID.as_slice_mut(),
        &mut save.EXTKER,
        &mut save.N,
        save.SNMHLS.as_slice_mut(),
        save.SNMPOL.as_slice_mut(),
        save.SNMIDX.as_slice_mut(),
        save.SIDHLS.as_slice_mut(),
        save.SIDPOL.as_slice_mut(),
        save.SIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"EXTKER", save.EXTKER, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZSRFKER: array size mismatch", ctx)?;

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, 10, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFKER(
        save.KERNAM.as_arg_mut(),
        save.NORNAM.as_arg_mut(),
        save.KERSID.as_slice_mut(),
        save.KERBID.as_slice_mut(),
        &mut save.EXTKER,
        &mut save.N,
        save.SNMHLS.as_slice_mut(),
        save.SNMPOL.as_slice_mut(),
        save.SNMIDX.as_slice_mut(),
        save.SIDHLS.as_slice_mut(),
        save.SIDPOL.as_slice_mut(),
        save.SIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ARRAYSIZEMISMATCH)", OK, ctx)?;

    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, 10, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFKER(
        save.KERNAM.as_arg_mut(),
        save.NORNAM.as_arg_mut(),
        save.KERSID.as_slice_mut(),
        save.KERBID.as_slice_mut(),
        &mut save.EXTKER,
        &mut save.N,
        save.SNMHLS.as_slice_mut(),
        save.SNMPOL.as_slice_mut(),
        save.SNMIDX.as_slice_mut(),
        save.SIDHLS.as_slice_mut(),
        save.SIDPOL.as_slice_mut(),
        save.SIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ARRAYSIZEMISMATCH)", OK, ctx)?;

    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, 10, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFKER(
        save.KERNAM.as_arg_mut(),
        save.NORNAM.as_arg_mut(),
        save.KERSID.as_slice_mut(),
        save.KERBID.as_slice_mut(),
        &mut save.EXTKER,
        &mut save.N,
        save.SNMHLS.as_slice_mut(),
        save.SNMPOL.as_slice_mut(),
        save.SNMIDX.as_slice_mut(),
        save.SIDHLS.as_slice_mut(),
        save.SIDPOL.as_slice_mut(),
        save.SIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ARRAYSIZEMISMATCH)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZSRFKER: bad kernel variable data type", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVBNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PCPOOL(KVCNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVNNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFKER(
        save.KERNAM.as_arg_mut(),
        save.NORNAM.as_arg_mut(),
        save.KERSID.as_slice_mut(),
        save.KERBID.as_slice_mut(),
        &mut save.EXTKER,
        &mut save.N,
        save.SNMHLS.as_slice_mut(),
        save.SNMPOL.as_slice_mut(),
        save.SNMIDX.as_slice_mut(),
        save.SIDHLS.as_slice_mut(),
        save.SIDPOL.as_slice_mut(),
        save.SIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZSRFKER: arrays too large", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(save.KRNAM2.get_mut(save.I), save.KERNAM.get(save.I));
            save.KRBID2[save.I] = save.KERBID[save.I];
            save.KRSID2[save.I] = save.KERSID[save.I];

            save.I += m3__;
        }
    }

    fstr::assign(save.KRNAM2.get_mut((save.NKVAR + 1)), b"EXTRA 1");
    fstr::assign(save.KRNAM2.get_mut((save.NKVAR + 2)), b"EXTRA 2");
    fstr::assign(save.KRNAM2.get_mut((save.NKVAR + 3)), b"EXTRA 3");

    save.KRSID2[(save.NKVAR + 1)] = 1;
    save.KRSID2[(save.NKVAR + 1)] = 2;
    save.KRSID2[(save.NKVAR + 1)] = 3;

    save.KRBID2[(save.NKVAR + 1)] = -1;
    save.KRBID2[(save.NKVAR + 1)] = -2;
    save.KRBID2[(save.NKVAR + 1)] = -3;

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, (save.NKVAR + 3), save.KRNAM2.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, (save.NKVAR + 3), save.KRBID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, (save.NKVAR + 3), save.KRSID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFKER(
        save.KRNAM2.as_arg_mut(),
        save.NORNAM.as_arg_mut(),
        save.KRSID2.as_slice_mut(),
        save.KRBID2.as_slice_mut(),
        &mut save.EXTKER,
        &mut save.N,
        save.SNMHLS.as_slice_mut(),
        save.SNMPOL.as_slice_mut(),
        save.SNMIDX.as_slice_mut(),
        save.SIDHLS.as_slice_mut(),
        save.SIDPOL.as_slice_mut(),
        save.SIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(TOOMANYSURFACES)", OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"ZZSRFKER: blank surface name", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Install variables in the kernel pool.
    //
    fstr::assign(save.KRNAM2.get_mut(1000), b" ");

    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KRNAM2.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KRBID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KRSID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ZZSRFKER(
        save.KRNAM2.as_arg_mut(),
        save.NORNAM.as_arg_mut(),
        save.KRSID2.as_slice_mut(),
        save.KRBID2.as_slice_mut(),
        &mut save.EXTKER,
        &mut save.N,
        save.SNMHLS.as_slice_mut(),
        save.SNMPOL.as_slice_mut(),
        save.SNMIDX.as_slice_mut(),
        save.SIDHLS.as_slice_mut(),
        save.SIDPOL.as_slice_mut(),
        save.SIDIDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BLANKNAMEASSIGNED)", OK, ctx)?;

    //***********************************************************************
    //
    //     SRFS2C tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFS2C: map names to codes", ctx)?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFS2C(
                &save.KERNAM[save.I],
                &save.BODNAM[save.I],
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf ID #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.SURFID,
                    b"=",
                    save.KERSID[save.I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"SRFS2C: map integer surface and body strings to codes",
        ctx,
    )?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR(save.KERSID[save.I], &mut save.SRFSTR, ctx);
            spicelib::INTSTR(save.KERBID[save.I], &mut save.BODSTR, ctx);

            spicelib::SRFS2C(
                &save.SRFSTR,
                &save.BODSTR,
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf ID #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.SURFID,
                    b"=",
                    save.KERSID[save.I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFS2C: surface name not valid; body name valid", ctx)?;

    fstr::assign(&mut save.SRFSTR, b"ZZZ");

    spicelib::INTSTR(save.KERBID[1], &mut save.BODSTR, ctx);

    spicelib::SRFS2C(
        &save.SRFSTR,
        &save.BODSTR,
        &mut save.SURFID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFS2C: surface name valid; body name not valid", ctx)?;

    fstr::assign(&mut save.BODSTR, b"ZZZ");

    spicelib::SRFS2C(
        &save.KERNAM[1],
        &save.BODSTR,
        &mut save.SURFID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"SRFS2C: clear pool, make sure translations are not performed.",
        ctx,
    )?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFS2C(
                &save.KERNAM[save.I],
                &save.BODNAM[save.I],
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFS2C: map names to codes (change variables)", ctx)?;

    //
    // Create variables having reversed order of elements; also
    // negate surface IDs.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = ((save.NKVAR + 1) - save.I);

            fstr::assign(
                save.KRNAM2.get_mut(save.J),
                &fstr::concat(
                    fstr::substr(save.KERNAM.get(save.I), 1..=(SFNMLN - 2)),
                    b"ZZ",
                ),
            );
            save.KRBID2[save.J] = save.KERBID[save.I];
            fstr::assign(save.BDNAM2.get_mut(save.J), save.BODNAM.get(save.I));
            save.KRSID2[save.J] = -save.KERSID[save.I];

            save.I += m3__;
        }
    }

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KRNAM2.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KRBID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KRSID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFS2C(
                &save.KRNAM2[save.I],
                &save.BDNAM2[save.I],
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf ID #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.SURFID,
                    b"=",
                    save.KRSID2[save.I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //***********************************************************************
    //
    //     SRFSCC tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFSCC: map names to codes", ctx)?;

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFSCC(
                &save.KERNAM[save.I],
                save.KERBID[save.I],
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf ID #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.SURFID,
                    b"=",
                    save.KERSID[save.I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFSCC: map integer surface strings to codes", ctx)?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR(save.KERSID[save.I], &mut save.SRFSTR, ctx);

            spicelib::SRFSCC(
                &save.SRFSTR,
                save.KERBID[save.I],
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf ID #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.SURFID,
                    b"=",
                    save.KERSID[save.I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFSCC: surface name not valid; body code valid", ctx)?;

    fstr::assign(&mut save.SRFSTR, b"ZZZ");

    spicelib::SRFSCC(
        &save.SRFSTR,
        save.KERBID[1],
        &mut save.SURFID,
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFSCC: surface name valid; body code not valid", ctx)?;

    spicelib::SRFSCC(&save.KERNAM[1], -1, &mut save.SURFID, &mut save.FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(
        b"SRFSCC: clear pool, make sure translations are not performed.",
        ctx,
    )?;

    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFSCC(
                &save.KERNAM[save.I],
                save.KERBID[save.I],
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

            save.I += m3__;
        }
    }

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //***********************************************************************
    //
    //     SRFC2S tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFC2S: map codes to names", ctx)?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFC2S(
                save.KERSID[save.I],
                save.KERBID[save.I],
                &mut save.SRFSTR,
                &mut save.ISNAME,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"ISNAME #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.ISNAME, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf name #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSC(
                    &save.LABEL,
                    &save.SRFSTR,
                    b"=",
                    &save.KERNAM[save.I],
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFCSS: surface code valid; body code not valid", ctx)?;

    save.BODYID = -3;

    spicelib::SRFC2S(
        save.KERSID[1],
        save.BODYID,
        &mut save.SRFSTR,
        &mut save.ISNAME,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(&save.LABEL, save.ISNAME, false, OK, ctx)?;

    spicelib::INTSTR(save.KERSID[1], &mut save.EXPSTR, ctx);

    fstr::assign(&mut save.LABEL, b"Surf name #");
    spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(&save.LABEL, &save.SRFSTR, b"=", &save.EXPSTR, OK, ctx)?;

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFC2S: map names to codes (change variables)", ctx)?;

    //
    // Create variables having reversed order of elements; also
    // negate surface IDs.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.J = ((save.NKVAR + 1) - save.I);

            fstr::assign(
                save.KRNAM2.get_mut(save.J),
                &fstr::concat(
                    fstr::substr(save.KERNAM.get(save.I), 1..=(SFNMLN - 2)),
                    b"ZZ",
                ),
            );
            save.KRBID2[save.J] = save.KERBID[save.I];
            fstr::assign(save.BDNAM2.get_mut(save.J), save.BODNAM.get(save.I));
            save.KRSID2[save.J] = -save.KERSID[save.I];

            save.I += m3__;
        }
    }

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KRNAM2.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KRBID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KRSID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFC2S(
                save.KRSID2[save.I],
                save.KRBID2[save.I],
                &mut save.SRFSTR,
                &mut save.ISNAME,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"ISNAME #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.ISNAME, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf name #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSC(
                    &save.LABEL,
                    &save.SRFSTR,
                    b"=",
                    &save.KRNAM2[save.I],
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //***********************************************************************
    //
    //     SRFCSS tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFCSS: map codes to names", ctx)?;

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KERNAM.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KERBID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KERSID.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::SRFCSS(
                save.KERSID[save.I],
                &save.BODNAM[save.I],
                &mut save.SRFSTR,
                &mut save.ISNAME,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"ISNAME #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.ISNAME, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf name #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSC(
                    &save.LABEL,
                    &save.SRFSTR,
                    b"=",
                    &save.KERNAM[save.I],
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFCSS: map codes to names using body strings", ctx)?;

    save.NCASE = save.NKVAR;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NCASE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::INTSTR(save.KERBID[save.I], &mut save.BODSTR, ctx);

            spicelib::SRFCSS(
                save.KERSID[save.I],
                &save.BODSTR,
                &mut save.SRFSTR,
                &mut save.ISNAME,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"ISNAME #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.ISNAME, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf name #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSC(
                    &save.LABEL,
                    &save.SRFSTR,
                    b"=",
                    &save.KERNAM[save.I],
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"SRFCSS: surface code valid; body string not valid", ctx)?;

    fstr::assign(&mut save.BODSTR, b"ZZZ");

    spicelib::SRFCSS(
        save.KERSID[1],
        &save.BODSTR,
        &mut save.SRFSTR,
        &mut save.ISNAME,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(&save.LABEL, save.ISNAME, false, OK, ctx)?;

    spicelib::INTSTR(save.KERSID[1], &mut save.EXPSTR, ctx);

    fstr::assign(&mut save.LABEL, b"Surf name #");
    spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(&save.LABEL, &save.SRFSTR, b"=", &save.EXPSTR, OK, ctx)?;

    //***********************************************************************
    //
    //     ZZSRFN2C, ZZSRFC2N tests
    //
    //***********************************************************************

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check mapping of alias surface names to codes", ctx)?;

    //
    // Create a mapping such that, for each surface ID and body,
    // there are three names.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 3;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.K = ((save.I - 1) + save.J);

                    fstr::assign(save.KRNAM2.get_mut(save.K), save.KERNAM.get(save.K));

                    spicelib::REPMC(
                        &save.KRNAM2[save.K].to_vec(),
                        b"name",
                        b"name #",
                        &mut save.KRNAM2[save.K],
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    spicelib::REPMI(
                        &save.KRNAM2[save.K].to_vec(),
                        b"#",
                        save.J,
                        &mut save.KRNAM2[save.K],
                        ctx,
                    );
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    save.KRBID2[save.K] = save.KERBID[save.I];
                    save.KRSID2[save.K] = save.KERSID[save.I];
                    fstr::assign(save.BDNAM2.get_mut(save.K), save.BODNAM.get(save.I));

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KRNAM2.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KRBID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KRSID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZSRFN2C(
                &save.KRNAM2[save.I],
                save.KRBID2[save.I],
                &mut save.SURFID,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf ID #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSI(
                    &save.LABEL,
                    save.SURFID,
                    b"=",
                    save.KRSID2[save.I],
                    0,
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check mapping of codes to aliased surface names", ctx)?;

    //
    // Use the mapping from the previous test case.
    //
    // Make sure the highest-priority name is selected.
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 3;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            spicelib::ZZSRFC2N(
                save.KRSID2[save.I],
                save.KRBID2[save.I],
                &mut save.SRFSTR,
                &mut save.FOUND,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.LABEL, b"FOUND #");
            spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

            if save.FOUND {
                fstr::assign(&mut save.LABEL, b"Surf string #");
                spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Note the index used in KRNAM2...
                //
                testutil::CHCKSC(
                    &save.LABEL,
                    &save.SRFSTR,
                    b"=",
                    &save.KRNAM2[(save.I + 2)],
                    OK,
                    ctx,
                )?;
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check masking of surface names associated with multiple codes when mapping names to codes", ctx)?;

    //
    // Create a mapping such that, for each surface name and body,
    // there are three surface IDs.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 3;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.K = ((save.I - 1) + save.J);

                    fstr::assign(save.KRNAM2.get_mut(save.K), save.KERNAM.get(save.I));
                    save.KRBID2[save.K] = save.KERBID[save.I];
                    save.KRSID2[save.K] = save.KERSID[save.K];
                    fstr::assign(save.BDNAM2.get_mut(save.K), save.BODNAM.get(save.I));

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // Install variables in the kernel pool.
    //
    spicelib::PCPOOL(KVNNAM, save.NKVAR, save.KRNAM2.as_arg(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVBNAM, save.NKVAR, save.KRBID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PIPOOL(KVCNAM, save.NKVAR, save.KRSID2.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 3;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // The mapping in this direction need be performed for
            // only one value of J.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = 1;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.K = ((save.I - 1) + save.J);

                    spicelib::ZZSRFN2C(
                        &save.KRNAM2[save.I],
                        save.KRBID2[save.I],
                        &mut save.SURFID,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"FOUND #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        fstr::assign(&mut save.LABEL, b"Surf ID #");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        //
                        // Note index of KRSID2.
                        //
                        testutil::CHCKSI(
                            &save.LABEL,
                            save.SURFID,
                            b"=",
                            save.KRSID2[(save.I + 2)],
                            0,
                            OK,
                            ctx,
                        )?;
                    }

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    //
    // --- Case --------------------------------------------------------
    //
    testutil::TCASE(b"Check masking of surface names associated with multiple codes when mapping codes to names", ctx)?;

    //
    // Use the mapping defined in the previous test case.
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NKVAR;
        let m3__: i32 = 3;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // The mapping in this direction can be performed for
            // only J = 3.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = 2;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.K = ((save.I - 1) + save.J);

                    spicelib::ZZSRFC2N(
                        save.KRSID2[save.K],
                        save.KRBID2[save.K],
                        &mut save.SRFSTR,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"FOUND #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.I, &mut save.LABEL, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(&save.LABEL, save.FOUND, false, OK, ctx)?;

                    save.J += m3__;
                }
            }

            {
                let m1__: i32 = 3;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    save.K = ((save.I - 1) + save.J);

                    spicelib::ZZSRFC2N(
                        save.KRSID2[save.K],
                        save.KRBID2[save.K],
                        &mut save.SRFSTR,
                        &mut save.FOUND,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    fstr::assign(&mut save.LABEL, b"FOUND #");
                    spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.K, &mut save.LABEL, ctx);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    testutil::CHCKSL(&save.LABEL, save.FOUND, true, OK, ctx)?;

                    if save.FOUND {
                        fstr::assign(&mut save.LABEL, b"Surf name #");
                        spicelib::REPMI(&save.LABEL.to_vec(), b"#", save.K, &mut save.LABEL, ctx);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        testutil::CHCKSC(
                            &save.LABEL,
                            &save.SRFSTR,
                            b"=",
                            &save.KRNAM2[save.K],
                            OK,
                            ctx,
                        )?;
                    }

                    save.J += m3__;
                }
            }

            save.I += m3__;
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
