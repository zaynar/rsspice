//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNAMSZ: i32 = 32;
const MXCLSG: i32 = 100;
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const DECLEN: i32 = 200;
const IFNLEN: i32 = 60;
const NTABS: i32 = 6;
const MAXROW: i32 = 5000;
const MAXENT: i32 = 20;
const MAXFAT: i32 = 100;
const MAXFVL: i32 = (MAXFAT * MAXENT);
const CVALSZ: i32 = 100;
const MAXVAL: i32 = ((MAXROW * MAXENT) / 2);

struct SaveVars {
    CNAMES: ActualCharArray,
    CVALS: ActualCharArray,
    DECLS: ActualCharArray,
    FATBUF: ActualCharArray,
    IFNAME: Vec<u8>,
    TABLES: ActualCharArray,
    DVALS: ActualArray<f64>,
    TVALS: ActualArray<f64>,
    CCLASS: StackArray<i32, 100>,
    DIMS: StackArray<i32, 100>,
    DTYPES: StackArray<i32, 100>,
    ENTSZS: ActualArray<i32>,
    IVALS: ActualArray<i32>,
    LOC: i32,
    MAXSTL: i32,
    NCOLS: i32,
    NCOMCH: i32,
    NROWS: i32,
    P: i32,
    RCPTRS: ActualArray<i32>,
    SEGNO: i32,
    SEGTYP: i32,
    STLENS: StackArray<i32, 100>,
    WKINDX: ActualArray<i32>,
    INDEXD: StackArray<bool, 100>,
    NLFLGS: ActualArray<bool>,
    NULLOK: StackArray<bool, 100>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CNAMES = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
        let mut CVALS = ActualCharArray::new(CVALSZ, 1..=MAXVAL);
        let mut DECLS = ActualCharArray::new(DECLEN, 1..=MXCLSG);
        let mut FATBUF = ActualCharArray::new(MAXSTR, 1..=MAXFVL);
        let mut IFNAME = vec![b' '; IFNLEN as usize];
        let mut TABLES = ActualCharArray::new(TNAMSZ, 1..=NTABS);
        let mut DVALS = ActualArray::<f64>::new(1..=MAXVAL);
        let mut TVALS = ActualArray::<f64>::new(1..=MAXVAL);
        let mut CCLASS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut DIMS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut DTYPES = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut ENTSZS = ActualArray::<i32>::new(1..=MAXROW);
        let mut IVALS = ActualArray::<i32>::new(1..=MAXVAL);
        let mut LOC: i32 = 0;
        let mut MAXSTL: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NCOMCH: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut P: i32 = 0;
        let mut RCPTRS = ActualArray::<i32>::new(1..=MAXROW);
        let mut SEGNO: i32 = 0;
        let mut SEGTYP: i32 = 0;
        let mut STLENS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut WKINDX = ActualArray::<i32>::new(1..=MAXROW);
        let mut INDEXD = StackArray::<bool, 100>::new(1..=MXCLSG);
        let mut NLFLGS = ActualArray::<bool>::new(1..=MAXROW);
        let mut NULLOK = StackArray::<bool, 100>::new(1..=MXCLSG);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SCALAR_1"),
                Val::C(b"SCALAR_2"),
                Val::C(b"SCALAR_3"),
                Val::C(b"SCALAR_4"),
                Val::C(b"VECTOR_1"),
                Val::C(b"VECTOR_2"),
            ]
            .into_iter();
            TABLES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CNAMES,
            CVALS,
            DECLS,
            FATBUF,
            IFNAME,
            TABLES,
            DVALS,
            TVALS,
            CCLASS,
            DIMS,
            DTYPES,
            ENTSZS,
            IVALS,
            LOC,
            MAXSTL,
            NCOLS,
            NCOMCH,
            NROWS,
            P,
            RCPTRS,
            SEGNO,
            SEGTYP,
            STLENS,
            WKINDX,
            INDEXD,
            NLFLGS,
            NULLOK,
        }
    }
}

//$Procedure  TSTEK ( Produce EK column entries for EK testing )
pub fn TSTEK(
    FILE: &[u8],
    FILENO: i32,
    MXROWS: i32,
    LOAD: bool,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
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
    // Save ALL variables to prevent TSPICE_C crashing on cygwin.
    //

    //
    // Initial values
    //

    //
    // Use discovery check-in.
    //

    //
    // Delete the old version of the file, if it exists.
    //
    if spicelib::EXISTS(FILE, ctx)? {
        spicelib::DELFIL(FILE, ctx)?;
    }

    //
    // Set up the number of reserved comment characters and the internal
    // file name.
    //
    save.NCOMCH = (1024 * FILENO);
    fstr::assign(&mut save.IFNAME, b"EK TEST FILE #*");
    spicelib::REPMI(&save.IFNAME.to_vec(), b"*", FILENO, &mut save.IFNAME, ctx);

    //
    // Open a new E-kernel.
    //
    spicelib::EKOPN(FILE, &save.IFNAME, save.NCOMCH, HANDLE, ctx)?;

    //
    // For each table:
    //
    for TABNO in 1..=NTABS {
        //
        // Look up the schema for this table.
        //
        TSTSCH(
            &save.TABLES[TABNO],
            MXROWS,
            &mut save.SEGTYP,
            &mut save.NROWS,
            &mut save.NCOLS,
            save.CNAMES.as_arg_mut(),
            save.CCLASS.as_slice_mut(),
            save.DTYPES.as_slice_mut(),
            save.STLENS.as_slice_mut(),
            save.DIMS.as_slice_mut(),
            save.INDEXD.as_slice_mut(),
            save.NULLOK.as_slice_mut(),
            save.DECLS.as_arg_mut(),
            ctx,
        )?;

        //
        // Make sure we're not getting more rows than we expected.
        //
        spicelib::MAXAI(
            save.STLENS.as_slice(),
            save.NCOLS,
            &mut save.MAXSTL,
            &mut save.LOC,
        );

        if (save.NROWS > MAXROW) {
            spicelib::ERRACT(b"SET", &mut b"ABORT".clone(), ctx)?;
            spicelib::ERRDEV(b"SET", &mut b"SCREEN".clone(), ctx)?;
            spicelib::ERRPRT(b"SET", &mut b"ALL".clone(), ctx)?;

            spicelib::CHKIN(b"TSTEK", ctx)?;
            spicelib::SETMSG(b"Oops! Max number of rows that can be handled by this routine is #; number handed back by TSTSCH is #.", ctx);
            spicelib::ERRINT(b"#", MAXROW, ctx);
            spicelib::ERRINT(b"#", save.NROWS, ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"TSTEK", ctx)?;
        } else if ((save.NROWS > MAXFAT) && (save.MAXSTL > CVALSZ)) {
            spicelib::ERRACT(b"SET", &mut b"ABORT".clone(), ctx)?;
            spicelib::ERRDEV(b"SET", &mut b"SCREEN".clone(), ctx)?;
            spicelib::ERRPRT(b"SET", &mut b"ALL".clone(), ctx)?;

            spicelib::CHKIN(b"TSTEK", ctx)?;
            spicelib::SETMSG(b"Oops! Max number of long rows that can be handled by this routine is #; number handed back by TSTSCH is #.", ctx);
            spicelib::ERRINT(b"#", MAXFAT, ctx);
            spicelib::ERRINT(b"#", save.NROWS, ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"TSTEK", ctx)?;
        }

        //
        // Initiate a fast load.
        //
        spicelib::EKIFLD(
            *HANDLE,
            &save.TABLES[TABNO],
            save.NCOLS,
            save.NROWS,
            save.CNAMES.as_arg(),
            save.DECLS.as_arg(),
            &mut save.SEGNO,
            save.RCPTRS.as_slice_mut(),
            ctx,
        )?;

        //
        // Load the columns one by one. Each column and associated
        // arrays must be filled in first.  Obtain data, entry sizes,
        // and null flags from TSTENT.
        //
        for COLNO in 1..=save.NCOLS {
            //
            // Initialize the data pointer to point to the first free
            // slot.
            //
            save.P = 1;

            for I in 1..=save.NROWS {
                if (save.STLENS[COLNO] > CVALSZ) {
                    //
                    // We're dealing with some very long strings.  Use
                    // the fat string buffer.
                    //
                    TSTENT(
                        FILENO,
                        &save.TABLES[TABNO],
                        save.SEGNO,
                        &save.CNAMES[COLNO],
                        I,
                        MAXENT,
                        &mut save.ENTSZS[I],
                        save.FATBUF.subarray_mut(save.P),
                        save.DVALS.subarray_mut(save.P),
                        save.IVALS.subarray_mut(save.P),
                        save.TVALS.subarray_mut(save.P),
                        &mut save.NLFLGS[I],
                        ctx,
                    )?;
                } else {
                    //
                    // Use the normal buffers.
                    //
                    TSTENT(
                        FILENO,
                        &save.TABLES[TABNO],
                        save.SEGNO,
                        &save.CNAMES[COLNO],
                        I,
                        MAXENT,
                        &mut save.ENTSZS[I],
                        save.CVALS.subarray_mut(save.P),
                        save.DVALS.subarray_mut(save.P),
                        save.IVALS.subarray_mut(save.P),
                        save.TVALS.subarray_mut(save.P),
                        &mut save.NLFLGS[I],
                        ctx,
                    )?;
                }

                //
                // Advance the data pointer by the number of elements in
                // the current entry.
                //
                save.P = (save.P + save.ENTSZS[I]);
            }

            //
            // The column is ready to be added.  Choose the addition
            // routine based on the column's data type.
            //
            if (save.DTYPES[COLNO] == CHR) {
                if (save.STLENS[COLNO] > CVALSZ) {
                    spicelib::EKACLC(
                        *HANDLE,
                        save.SEGNO,
                        &save.CNAMES[COLNO],
                        save.FATBUF.as_arg(),
                        save.ENTSZS.as_slice(),
                        save.NLFLGS.as_slice(),
                        save.RCPTRS.as_slice(),
                        save.WKINDX.as_slice_mut(),
                        ctx,
                    )?;
                } else {
                    spicelib::EKACLC(
                        *HANDLE,
                        save.SEGNO,
                        &save.CNAMES[COLNO],
                        save.CVALS.as_arg(),
                        save.ENTSZS.as_slice(),
                        save.NLFLGS.as_slice(),
                        save.RCPTRS.as_slice(),
                        save.WKINDX.as_slice_mut(),
                        ctx,
                    )?;
                }
            } else if (save.DTYPES[COLNO] == DP) {
                spicelib::EKACLD(
                    *HANDLE,
                    save.SEGNO,
                    &save.CNAMES[COLNO],
                    save.DVALS.as_slice(),
                    save.ENTSZS.as_slice(),
                    save.NLFLGS.as_slice(),
                    save.RCPTRS.as_slice(),
                    save.WKINDX.as_slice_mut(),
                    ctx,
                )?;
            } else if (save.DTYPES[COLNO] == INT) {
                spicelib::EKACLI(
                    *HANDLE,
                    save.SEGNO,
                    &save.CNAMES[COLNO],
                    save.IVALS.as_slice(),
                    save.ENTSZS.as_slice(),
                    save.NLFLGS.as_slice(),
                    save.RCPTRS.as_slice(),
                    save.WKINDX.as_slice_mut(),
                    ctx,
                )?;
            } else {
                //
                // The data type is TIME.  Use the DP column add routine.
                //
                spicelib::EKACLD(
                    *HANDLE,
                    save.SEGNO,
                    &save.CNAMES[COLNO],
                    save.TVALS.as_slice(),
                    save.ENTSZS.as_slice(),
                    save.NLFLGS.as_slice(),
                    save.RCPTRS.as_slice(),
                    save.WKINDX.as_slice_mut(),
                    ctx,
                )?;
            }
        }

        //
        // Finish the fast load for this table.
        //
        spicelib::EKFFLD(*HANDLE, save.SEGNO, save.RCPTRS.as_slice(), ctx)?;
    }

    //
    // Close the EK file.
    //
    spicelib::EKCLS(*HANDLE, ctx)?;

    //
    // Load the file if commanded to do so.
    //
    if LOAD {
        spicelib::EKLEF(FILE, HANDLE, ctx)?;
    }

    //
    // Register this file with the FILREG so that it can be
    // removed when the current test family is done with its
    // task.
    //
    TFILES(FILE, ctx);

    Ok(())
}
