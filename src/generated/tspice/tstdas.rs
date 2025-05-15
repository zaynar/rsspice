//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const SCHADD: i32 = 1;
pub const SCHCOD: i32 = 2;
pub const MAXCL: i32 = 20;
pub const MAXREC: i32 = 10000;
pub const MAXFIL: i32 = 5000;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const NWD: i32 = 128;
const NWI: i32 = 256;
const NWC: i32 = 1024;
const FILEX: i32 = 100000;
const RECX: i32 = 1000;

struct SaveVars {
    CREC: ActualCharArray,
    DREC: ActualArray2D<f64>,
    I: i32,
    IREC: ActualArray2D<i32>,
    J: i32,
    K: i32,
    NCL: StackArray<i32, 3>,
    NW: i32,
    NWREC: StackArray<i32, 3>,
    NWRITE: i32,
    NWRITN: StackArray<i32, 3>,
    RECNO: i32,
    REMAIN: i32,
    TYPE: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CREC = ActualCharArray::new(NWC, 1..=MAXCL);
        let mut DREC = ActualArray2D::<f64>::new(1..=NWD, 1..=MAXCL);
        let mut I: i32 = 0;
        let mut IREC = ActualArray2D::<i32>::new(1..=NWI, 1..=MAXCL);
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut NCL = StackArray::<i32, 3>::new(1..=3);
        let mut NW: i32 = 0;
        let mut NWREC = StackArray::<i32, 3>::new(1..=3);
        let mut NWRITE: i32 = 0;
        let mut NWRITN = StackArray::<i32, 3>::new(1..=3);
        let mut RECNO: i32 = 0;
        let mut REMAIN: i32 = 0;
        let mut TYPE: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(NWC), Val::I(NWD), Val::I(NWI)].into_iter();
            NWREC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CREC,
            DREC,
            I,
            IREC,
            J,
            K,
            NCL,
            NW,
            NWREC,
            NWRITE,
            NWRITN,
            RECNO,
            REMAIN,
            TYPE,
        }
    }
}

//$Procedure      TSTDAS ( Test DAS address calculation routine )
pub fn TSTDAS(
    FILE: &[u8],
    FTYPE: &[u8],
    NCOMRC: i32,
    FILENO: i32,
    NCLUST: i32,
    CLTYPS: &[i32],
    CLNWDS: &[i32],
    SGRGAT: bool,
    FCLOSE: bool,
    NSCHEM: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CLTYPS = DummyArray::new(CLTYPS, 1..);
    let CLNWDS = DummyArray::new(CLNWDS, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    // These parameters are named to enhance ease of maintenance of
    // the code; the values should not be changed.

    //
    // File number factor used for creating data:
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

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"TSTDAS", ctx)?;

    if (FILENO > MAXFIL) {
        spicelib::SETMSG(b"FILENO = #; max allowed value is #.", ctx);
        spicelib::ERRINT(b"#", FILENO, ctx);
        spicelib::ERRINT(b"#", MAXFIL, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"TSTDAS", ctx)?;
        return Ok(());
    }

    //
    // Check cluster data types and sizes.
    //
    spicelib::CLEARI(3, save.NCL.as_slice_mut());

    for CLIDX in intrinsics::range(NCLUST, 1, -1) {
        save.TYPE = CLTYPS[CLIDX];
        //
        // Check the data type before using it as an array index.
        //
        if ((save.TYPE < CHR) || (save.TYPE > INT)) {
            spicelib::SETMSG(b"Invalid data type code # found for cluster #.", ctx);
            spicelib::ERRINT(b"#", save.TYPE, ctx);
            spicelib::ERRINT(b"#", CLIDX, ctx);
            spicelib::SIGERR(b"SPICE(BADDATATYPE)", ctx)?;
            spicelib::CHKOUT(b"TSTDAS", ctx)?;
            return Ok(());
        }

        //
        // Check the cluster size. The upper bound check is
        // loose, but it may catch uninitialized junk.
        //
        save.I = CLNWDS[CLIDX];

        if ((save.I < 1) || (save.I > (MAXCL * save.NWREC[save.TYPE]))) {
            spicelib::SETMSG(b"Invalid cluster word count # found for cluster #.", ctx);
            spicelib::ERRINT(b"#", save.I, ctx);
            spicelib::ERRINT(b"#", CLIDX, ctx);
            spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
            spicelib::CHKOUT(b"TSTDAS", ctx)?;
            return Ok(());
        }

        //
        // The cluster must be "full" unless this is
        // the final cluster of this data type. Check it.
        //
        save.NCL[save.TYPE] = (save.NCL[save.TYPE] + 1);

        save.NW = save.NWREC[save.TYPE];

        if (((save.I / save.NW) * save.NW) != save.I) {
            //
            // The word count is not an integral multiple
            // of the number of words of TYPE that fit in a
            // record.
            //
            if (save.NCL[save.TYPE] > 1) {
                //
                // We're searching backward in the cluster lists,
                // so a cluster count greater than 1 means we're
                // not looking at the last cluster of data type
                // TYPE.
                //
                spicelib::SETMSG(b"Partial cluster found prior to last cluster of data type #. Cluster index is #. Cluster word count is #.", ctx);
                spicelib::ERRINT(b"#", save.TYPE, ctx);
                spicelib::ERRINT(b"#", CLIDX, ctx);
                spicelib::ERRINT(b"#", save.I, ctx);
                spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
                spicelib::CHKOUT(b"TSTDAS", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Open the file.
    //
    spicelib::DASONW(FILE, FTYPE, FILE, NCOMRC, HANDLE, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"TSTDAS", ctx)?;
        return Ok(());
    }

    //
    // Write data to the file.
    //
    spicelib::CLEARI(3, save.NWRITN.as_slice_mut());

    save.RECNO = 1;

    for CLIDX in 1..=NCLUST {
        if (save.RECNO >= MAXREC) {
            spicelib::SETMSG(b"RECNO = #; max allowed value is #.", ctx);
            spicelib::ERRINT(b"#", save.RECNO, ctx);
            spicelib::ERRINT(b"#", MAXREC, ctx);
            spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            spicelib::CHKOUT(b"TSTDAS", ctx)?;
            return Ok(());
        }

        save.TYPE = CLTYPS[CLIDX];
        save.NW = save.NWREC[save.TYPE];

        //
        // Create data current cluster, one record at a time. Write
        // the entire cluster using a single call.
        //
        if (save.TYPE == INT) {
            //
            // Populate a buffer with data of the indicated type.
            //
            save.REMAIN = CLNWDS[CLIDX];
            save.J = 1;

            while (save.REMAIN > 0) {
                save.NWRITE = intrinsics::MIN0(&[save.NW, save.REMAIN]);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.NWRITE;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if (NSCHEM == SCHADD) {
                            save.IREC[[save.I, save.J]] = (save.I + save.NWRITN[save.TYPE]);
                        } else {
                            save.IREC[[save.I, save.J]] =
                                (((FILENO * FILEX) + (save.RECNO * RECX)) + save.I);
                        }

                        save.I += m3__;
                    }
                }

                save.NWRITN[save.TYPE] = (save.NWRITN[save.TYPE] + save.NWRITE);

                save.J = (save.J + 1);
                save.RECNO = (save.RECNO + 1);
                save.REMAIN = (save.REMAIN - save.NWRITE);
            }

            spicelib::DASADI(*HANDLE, CLNWDS[CLIDX], save.IREC.as_slice(), ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"TSTDAS", ctx)?;
                return Ok(());
            }
        } else if (save.TYPE == DP) {
            save.REMAIN = CLNWDS[CLIDX];
            save.J = 1;

            while (save.REMAIN > 0) {
                save.NWRITE = intrinsics::MIN0(&[save.NW, save.REMAIN]);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.NWRITE;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if (NSCHEM == SCHADD) {
                            save.DREC[[save.I, save.J]] =
                                ((save.I as f64) + save.NWRITN[save.TYPE] as f64);
                        } else {
                            save.DREC[[save.I, save.J]] = ((((FILENO * FILEX) + (save.RECNO * RECX))
                                as f64)
                                + (save.I as f64));
                        }

                        save.I += m3__;
                    }
                }

                save.NWRITN[save.TYPE] = (save.NWRITN[save.TYPE] + save.NWRITE);

                save.J = (save.J + 1);
                save.RECNO = (save.RECNO + 1);
                save.REMAIN = (save.REMAIN - save.NWRITE);
            }

            spicelib::DASADD(*HANDLE, CLNWDS[CLIDX], save.DREC.as_slice(), ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"TSTDAS", ctx)?;
                return Ok(());
            }
        } else if (save.TYPE == CHR) {
            save.REMAIN = CLNWDS[CLIDX];
            save.J = 1;

            while (save.REMAIN > 0) {
                save.NWRITE = intrinsics::MIN0(&[save.NW, save.REMAIN]);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.NWRITE;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if (NSCHEM == SCHADD) {
                            save.K = (save.I + save.NWRITN[save.TYPE]);
                        } else {
                            save.K = (((FILENO * FILEX) + (save.RECNO * RECX)) + save.I);
                        }

                        fstr::assign(
                            fstr::substr_mut(save.CREC.get_mut(save.J), save.I..=save.I),
                            &intrinsics::CHAR(intrinsics::MOD(save.K, 128)),
                        );

                        save.I += m3__;
                    }
                }

                save.NWRITN[save.TYPE] = (save.NWRITN[save.TYPE] + save.NWRITE);

                save.J = (save.J + 1);
                save.RECNO = (save.RECNO + 1);
                save.REMAIN = (save.REMAIN - save.NWRITE);
            }

            spicelib::DASADC(*HANDLE, CLNWDS[CLIDX], 1, NWC, save.CREC.as_arg(), ctx)?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"TSTDAS", ctx)?;
                return Ok(());
            }
        } else {
            spicelib::SETMSG(b"Bad data type # found for cluster #.", ctx);
            spicelib::ERRINT(b"#", save.TYPE, ctx);
            spicelib::ERRINT(b"#", CLIDX, ctx);
            spicelib::SIGERR(b"SPICE(BADDATATYPE)", ctx)?;
            spicelib::CHKOUT(b"TSTDAS", ctx)?;
            return Ok(());
        }
    }

    if FCLOSE {
        //
        // Close the file. Segregate the records if so commanded.
        //
        if SGRGAT {
            spicelib::DASCLS(*HANDLE, ctx)?;
        } else {
            spicelib::DASWBR(*HANDLE, ctx)?;
            spicelib::DASLLC(*HANDLE, ctx)?;
        }
    }

    spicelib::CHKOUT(b"TSTDAS", ctx)?;
    Ok(())
}
