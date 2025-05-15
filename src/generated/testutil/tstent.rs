//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const MAXESZ: i32 = 10;
const DECLEN: i32 = 200;

struct SaveVars {
    CNAMES: ActualCharArray,
    DECLS: ActualCharArray,
    PAD: Vec<u8>,
    COLIDX: i32,
    CCLASS: StackArray<i32, 100>,
    DIMS: StackArray<i32, 100>,
    DTYPES: StackArray<i32, 100>,
    BASVAL: i32,
    J: i32,
    NCOLS: i32,
    NROWS: i32,
    R: i32,
    SEGTYP: i32,
    STLENS: StackArray<i32, 100>,
    FIRST: bool,
    INDEXD: StackArray<bool, 100>,
    NULLOK: StackArray<bool, 100>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CNAMES = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
        let mut DECLS = ActualCharArray::new(DECLEN, 1..=MXCLSG);
        let mut PAD = vec![b' '; MAXSTR as usize];
        let mut COLIDX: i32 = 0;
        let mut CCLASS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut DIMS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut DTYPES = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut BASVAL: i32 = 0;
        let mut J: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut R: i32 = 0;
        let mut SEGTYP: i32 = 0;
        let mut STLENS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut FIRST: bool = false;
        let mut INDEXD = StackArray::<bool, 100>::new(1..=MXCLSG);
        let mut NULLOK = StackArray::<bool, 100>::new(1..=MXCLSG);

        FIRST = true;

        Self {
            CNAMES,
            DECLS,
            PAD,
            COLIDX,
            CCLASS,
            DIMS,
            DTYPES,
            BASVAL,
            J,
            NCOLS,
            NROWS,
            R,
            SEGTYP,
            STLENS,
            FIRST,
            INDEXD,
            NULLOK,
        }
    }
}

//$Procedure  TSTENT ( Produce EK column entries for EK testing )
pub fn TSTENT(
    FILENO: i32,
    TABLE: &[u8],
    SEGNO: i32,
    COLUMN: &[u8],
    ROWNO: i32,
    NMAX: i32,
    NELTS: &mut i32,
    CVALS: CharArrayMut,
    DVALS: &mut [f64],
    IVALS: &mut [i32],
    TVALS: &mut [f64],
    ISNULL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CVALS = DummyCharArrayMut::new(CVALS, None, 1..);
    let mut DVALS = DummyArrayMut::new(DVALS, 1..);
    let mut IVALS = DummyArrayMut::new(IVALS, 1..);
    let mut TVALS = DummyArrayMut::new(TVALS, 1..);

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
    // Save all.
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"TSTENT", ctx)?;
    }

    if save.FIRST {
        //
        // Initialize the pad string for long character data values.
        //
        for I in 1..=MAXSTR {
            fstr::assign(fstr::substr_mut(&mut save.PAD, I..=I), b"X");
        }

        fstr::assign(fstr::substr_mut(&mut save.PAD, 96..=100), b" 100>");
        fstr::assign(
            fstr::substr_mut(&mut save.PAD, (MAXSTR - 6)..=MAXSTR),
            b" 1024>",
        );

        save.FIRST = false;
    }

    //
    // Look up the schema for the indicated table.  This gives us
    // dimension information.  The value of "MXROWS" is irrelevant;
    // set it to the minimum allowed value.
    //
    TSTSCH(
        TABLE,
        10,
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
    // Get the index of the requested column within the CNAMES array.
    //
    save.COLIDX = spicelib::ISRCHC(COLUMN, save.NCOLS, save.CNAMES.as_arg());

    if (save.COLIDX == 0) {
        spicelib::SETMSG(b"Column # does not exist in table #.", ctx);
        spicelib::ERRCH(b"#", COLUMN, ctx);
        spicelib::ERRCH(b"#", TABLE, ctx);
        spicelib::SIGERR(b"SPICE(NOSUCHCOLUMN)", ctx)?;
        spicelib::CHKOUT(b"TSTENT", ctx)?;
        return Ok(());
    }

    //
    // Factor out the boiler-plate responses.  Set values for
    // the TABLE_NAME, FILE_NO, SEGMENT_NO and ROW_NO columns right here.
    //
    if (((((((fstr::eq(TABLE, b"SCALAR_1") || fstr::eq(TABLE, b"SCALAR_2"))
        || fstr::eq(TABLE, b"SCALAR_3"))
        || fstr::eq(TABLE, b"SCALAR_4"))
        || fstr::eq(TABLE, b"VECTOR_1"))
        || fstr::eq(TABLE, b"VECTOR_2"))
        || fstr::eq(TABLE, b"EMPTY_1"))
        || fstr::eq(TABLE, b"EMPTY_2"))
    {
        if fstr::eq(COLUMN, b"TABLE_NAME") {
            *NELTS = 1;
            fstr::assign(CVALS.get_mut(1), TABLE);
            *ISNULL = false;

            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        } else if fstr::eq(COLUMN, b"FILE_NO") {
            *NELTS = 1;
            IVALS[1] = FILENO;
            *ISNULL = false;

            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        } else if fstr::eq(COLUMN, b"SEGMENT_NO") {
            *NELTS = 1;
            IVALS[1] = SEGNO;
            *ISNULL = false;

            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        } else if fstr::eq(COLUMN, b"ROW_NO") {
            *NELTS = 1;
            IVALS[1] = ROWNO;
            *ISNULL = false;

            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        }
    }

    if fstr::eq(TABLE, b"SCALAR_1") {
        save.BASVAL = ((1000000 * SEGNO) + ROWNO);

        if fstr::eq(COLUMN, b"C_COL_1") {
            *NELTS = 1;
            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#");
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"D_COL_1") {
            *NELTS = 1;
            DVALS[1] = (save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"I_COL_1") {
            *NELTS = 1;
            IVALS[1] = save.BASVAL;
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"T_COL_1") {
            *NELTS = 1;
            TVALS[1] = (save.BASVAL as f64);
            *ISNULL = false;
        } else {
            spicelib::SETMSG(b"Column # does not exist in table SCALAR_1.", ctx);
            spicelib::ERRCH(b"#", COLUMN, ctx);
            spicelib::SIGERR(b"SPICE(NOSUCHCOLUMN)", ctx)?;
            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(TABLE, b"SCALAR_2") {
        save.BASVAL = ((1000000 * SEGNO) + ROWNO);

        if fstr::eq(COLUMN, b"C_COL_1") {
            *NELTS = 1;

            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#");
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"C_COL_2") {
            *NELTS = 1;

            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#_");
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);

            save.J = intrinsics::MOD(ROWNO, 100);

            for I in 1..=save.J {
                spicelib::SUFFIX(b"X", 0, &mut CVALS[1]);
            }

            //
            // Create two rows that are identical up through the first
            // 32 characters.  This tests some of the logic in ZZEKJSRT.
            //
            if ((ROWNO == 16) || (ROWNO == 18)) {
                for I in 1..=32 {
                    spicelib::PREFIX(b"X", 0, &mut CVALS[1]);
                }
            }

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"C_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"C_COL_4") {
            *NELTS = 1;

            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#_");
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);

            save.J = intrinsics::MOD(ROWNO, 100);

            for I in 1..=save.J {
                spicelib::SUFFIX(b"X", 0, &mut CVALS[1]);
            }

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"C_COL_5") {
            *NELTS = 1;
            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#_");
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"C_COL_6") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"D_COL_1") {
            *NELTS = 1;
            DVALS[1] = (-save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"D_COL_2") {
            *NELTS = 1;
            DVALS[1] = (-save.BASVAL as f64);
            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"D_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"I_COL_1") {
            *NELTS = 1;
            IVALS[1] = save.BASVAL;
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"I_COL_2") {
            *NELTS = 1;
            IVALS[1] = -save.BASVAL;
            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"I_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"T_COL_1") {
            *NELTS = 1;
            TVALS[1] = (-save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"T_COL_2") {
            *NELTS = 1;
            TVALS[1] = (-save.BASVAL as f64);
            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"T_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else {
            spicelib::SETMSG(b"Column # does not exist in table SCALAR_2.", ctx);
            spicelib::ERRCH(b"#", COLUMN, ctx);
            spicelib::SIGERR(b"SPICE(NOSUCHCOLUMN)", ctx)?;
            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(TABLE, b"SCALAR_3") {
        save.BASVAL = ((1000000 * SEGNO) + ROWNO);

        if fstr::eq(COLUMN, b"C_COL_1") {
            *NELTS = 1;

            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#_");
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"D_COL_1") {
            *NELTS = 1;
            DVALS[1] = (save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"I_COL_1") {
            *NELTS = 1;
            IVALS[1] = save.BASVAL;
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"T_COL_1") {
            *NELTS = 1;
            TVALS[1] = (save.BASVAL as f64);
            *ISNULL = false;
        } else {
            spicelib::SETMSG(b"Column # does not exist in table SCALAR_3.", ctx);
            spicelib::ERRCH(b"#", COLUMN, ctx);
            spicelib::SIGERR(b"SPICE(NOSUCHCOLUMN)", ctx)?;
            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(TABLE, b"SCALAR_4") {
        save.BASVAL = ((1000000 * SEGNO) + ROWNO);

        if fstr::eq(COLUMN, b"C_COL_1") {
            *NELTS = 1;

            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#_");

            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"C_COL_2") {
            *NELTS = 1;

            fstr::assign(CVALS.get_mut(1), b"SEG_#_#_ROW_#_");
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", SEGNO, &mut CVALS[1], ctx);
            spicelib::REPMC(&CVALS[1].to_vec(), b"#", COLUMN, &mut CVALS[1]);
            spicelib::REPMI(&CVALS[1].to_vec(), b"#", ROWNO, &mut CVALS[1], ctx);

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"C_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"D_COL_1") {
            *NELTS = 1;
            DVALS[1] = (save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"D_COL_2") {
            *NELTS = 1;
            DVALS[1] = (-save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"D_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"I_COL_1") {
            *NELTS = 1;
            IVALS[1] = save.BASVAL;
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"I_COL_2") {
            *NELTS = 1;
            IVALS[1] = -save.BASVAL;
            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"I_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"T_COL_1") {
            *NELTS = 1;
            TVALS[1] = (save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"T_COL_2") {
            *NELTS = 1;
            TVALS[1] = (-save.BASVAL as f64);
            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"T_COL_3") {
            *NELTS = 1;
            *ISNULL = true;
        } else {
            spicelib::SETMSG(b"Column # does not exist in table SCALAR_4.", ctx);
            spicelib::ERRCH(b"#", COLUMN, ctx);
            spicelib::SIGERR(b"SPICE(NOSUCHCOLUMN)", ctx)?;
            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(TABLE, b"VECTOR_1") {
        save.BASVAL = ((1000000 * SEGNO) + (100 * ROWNO));

        if fstr::eq(COLUMN, b"C_COL_1") {
            *NELTS = 3;

            for I in 1..=*NELTS {
                fstr::assign(CVALS.get_mut(I), b"SEG_#_#_ROW_#_ELT_#_");
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", SEGNO, &mut CVALS[I], ctx);
                spicelib::REPMC(&CVALS[I].to_vec(), b"#", COLUMN, &mut CVALS[I]);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", ROWNO, &mut CVALS[I], ctx);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", I, &mut CVALS[I], ctx);

                save.R = spicelib::RTRIM(&CVALS[I]);

                fstr::assign(
                    fstr::substr_mut(CVALS.get_mut(I), (save.R + 3)..),
                    fstr::substr(&save.PAD, (save.R + 3)..),
                );
            }

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"C_COL_2") {
            *NELTS = (1 + intrinsics::MOD(ROWNO, 10));

            for I in 1..=*NELTS {
                fstr::assign(CVALS.get_mut(I), b"SEG_#_#_ROW_#_ELT_#_");
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", SEGNO, &mut CVALS[I], ctx);
                spicelib::REPMC(&CVALS[I].to_vec(), b"#", COLUMN, &mut CVALS[I]);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", ROWNO, &mut CVALS[I], ctx);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", I, &mut CVALS[I], ctx);

                save.R = spicelib::RTRIM(&CVALS[I]);

                fstr::assign(
                    fstr::substr_mut(CVALS.get_mut(I), (save.R + 3)..),
                    fstr::substr(&save.PAD, (save.R + 3)..),
                );
            }

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"D_COL_1") {
            *NELTS = 4;

            for I in 1..=*NELTS {
                DVALS[I] = (save.BASVAL + I) as f64;
            }

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"D_COL_2") {
            *NELTS = (1 + intrinsics::MOD(ROWNO, 11));

            for I in 1..=*NELTS {
                DVALS[I] = (save.BASVAL + I) as f64;
            }

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"I_COL_1") {
            *NELTS = 5;

            for I in 1..=*NELTS {
                IVALS[I] = (save.BASVAL + I);
            }

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"I_COL_2") {
            *NELTS = (1 + intrinsics::MOD(ROWNO, 12));

            for I in 1..=*NELTS {
                IVALS[I] = (save.BASVAL + I);
            }

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"T_COL_1") {
            *NELTS = 6;

            for I in 1..=*NELTS {
                TVALS[I] = (save.BASVAL + I) as f64;
            }

            *ISNULL = false;
        } else if fstr::eq(COLUMN, b"T_COL_2") {
            *NELTS = (1 + intrinsics::MOD(ROWNO, 11));

            for I in 1..=*NELTS {
                TVALS[I] = (save.BASVAL + I) as f64;
            }

            *ISNULL = false;
        } else {
            spicelib::SETMSG(b"Column # does not exist in table VECTOR_1.", ctx);
            spicelib::ERRCH(b"#", COLUMN, ctx);
            spicelib::SIGERR(b"SPICE(NOSUCHCOLUMN)", ctx)?;
            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(TABLE, b"VECTOR_2") {
        save.BASVAL = ((1000000 * SEGNO) + (100 * ROWNO));

        if fstr::eq(COLUMN, b"C_COL_1") {
            *NELTS = 3;

            for I in 1..=*NELTS {
                fstr::assign(CVALS.get_mut(I), b"SEG_#_#_ROW_#_ELT_#_");
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", SEGNO, &mut CVALS[I], ctx);
                spicelib::REPMC(&CVALS[I].to_vec(), b"#", COLUMN, &mut CVALS[I]);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", ROWNO, &mut CVALS[I], ctx);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", I, &mut CVALS[I], ctx);

                save.R = spicelib::RTRIM(&CVALS[I]);

                fstr::assign(
                    fstr::substr_mut(CVALS.get_mut(I), (save.R + 3)..),
                    fstr::substr(&save.PAD, (save.R + 3)..),
                );
            }

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"C_COL_2") {
            *NELTS = 5;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"C_COL_3") {
            *ISNULL = spicelib::ODD(ROWNO);

            if *ISNULL {
                *NELTS = 1;
            } else {
                *NELTS = (1 + intrinsics::MOD(ROWNO, 10));
            }

            for I in 1..=*NELTS {
                fstr::assign(CVALS.get_mut(I), b"SEG_#_#_ROW_#_ELT_#_");
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", SEGNO, &mut CVALS[I], ctx);
                spicelib::REPMC(&CVALS[I].to_vec(), b"#", COLUMN, &mut CVALS[I]);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", ROWNO, &mut CVALS[I], ctx);
                spicelib::REPMI(&CVALS[I].to_vec(), b"#", I, &mut CVALS[I], ctx);

                save.R = spicelib::RTRIM(&CVALS[I]);

                fstr::assign(
                    fstr::substr_mut(CVALS.get_mut(I), (save.R + 3)..),
                    fstr::substr(&save.PAD, (save.R + 3)..),
                );
            }
        } else if fstr::eq(COLUMN, b"C_COL_4") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"D_COL_1") {
            *NELTS = 4;

            for I in 1..=*NELTS {
                DVALS[I] = (save.BASVAL + I) as f64;
            }

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"D_COL_2") {
            *NELTS = 6;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"D_COL_3") {
            *ISNULL = spicelib::ODD(ROWNO);

            if *ISNULL {
                *NELTS = 1;
            } else {
                *NELTS = (1 + intrinsics::MOD(ROWNO, 11));
            }

            for I in 1..=*NELTS {
                DVALS[I] = (save.BASVAL + I) as f64;
            }
        } else if fstr::eq(COLUMN, b"D_COL_4") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"I_COL_1") {
            *NELTS = 5;

            for I in 1..=*NELTS {
                IVALS[I] = (save.BASVAL + I);
            }

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"I_COL_2") {
            *NELTS = 7;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"I_COL_3") {
            *ISNULL = spicelib::ODD(ROWNO);

            if *ISNULL {
                *NELTS = 1;
            } else {
                *NELTS = (1 + intrinsics::MOD(ROWNO, 12));
            }

            for I in 1..=*NELTS {
                IVALS[I] = (save.BASVAL + I);
            }
        } else if fstr::eq(COLUMN, b"I_COL_4") {
            *NELTS = 1;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"T_COL_1") {
            *NELTS = 6;

            for I in 1..=*NELTS {
                TVALS[I] = (save.BASVAL + I) as f64;
            }

            *ISNULL = spicelib::ODD(ROWNO);
        } else if fstr::eq(COLUMN, b"T_COL_2") {
            *NELTS = 8;
            *ISNULL = true;
        } else if fstr::eq(COLUMN, b"T_COL_3") {
            *ISNULL = spicelib::ODD(ROWNO);

            if *ISNULL {
                *NELTS = 1;
            } else {
                *NELTS = (1 + intrinsics::MOD(ROWNO, 11));
            }

            for I in 1..=*NELTS {
                TVALS[I] = (save.BASVAL + I) as f64;
            }
        } else if fstr::eq(COLUMN, b"T_COL_4") {
            *NELTS = 1;
            *ISNULL = true;
        } else {
            spicelib::SETMSG(b"Column # does not exist in table VECTOR_2.", ctx);
            spicelib::ERRCH(b"#", COLUMN, ctx);
            spicelib::SIGERR(b"SPICE(NOSUCHCOLUMN)", ctx)?;
            spicelib::CHKOUT(b"TSTENT", ctx)?;
            return Ok(());
        }
    } else {
        spicelib::SETMSG(b"Table # does not exist.", ctx);
        spicelib::ERRCH(b"#", TABLE, ctx);
        spicelib::SIGERR(b"SPICE(NOSUCHTABLE)", ctx)?;
        spicelib::CHKOUT(b"TSTENT", ctx)?;
        return Ok(());
    }

    //
    // Make some final adjustments:  for fixed-length character columns,
    // truncate the non-blank portion of the column entries at the
    // declared column string length.
    //
    if (save.DTYPES[save.COLIDX] == CHR) {
        save.R = save.STLENS[save.COLIDX];

        if (save.R == IFALSE) {
            if (intrinsics::LEN(&CVALS[1]) > 100) {
                for I in 1..=*NELTS {
                    fstr::assign(fstr::substr_mut(CVALS.get_mut(I), 101..), b" ");
                }
            }
        } else {
            if (intrinsics::LEN(&CVALS[1]) > save.R) {
                for I in 1..=*NELTS {
                    fstr::assign(fstr::substr_mut(CVALS.get_mut(I), (save.R + 1)..), b" ");
                }
            }
        }
    }

    spicelib::CHKOUT(b"TSTENT", ctx)?;
    Ok(())
}
