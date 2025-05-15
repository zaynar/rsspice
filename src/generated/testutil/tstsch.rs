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

//$Procedure  TSTSCH ( Produce EK table schemas for EK testing )
pub fn TSTSCH(
    TABLE: &[u8],
    MXROWS: i32,
    SEGTYP: &mut i32,
    NROWS: &mut i32,
    NCOLS: &mut i32,
    CNAMES: CharArrayMut,
    CCLASS: &mut [i32],
    DTYPES: &mut [i32],
    STLENS: &mut [i32],
    DIMS: &mut [i32],
    INDEXD: &mut [bool],
    NULLOK: &mut [bool],
    DECLS: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CNAMES = DummyCharArrayMut::new(CNAMES, None, 1..);
    let mut CCLASS = DummyArrayMut::new(CCLASS, 1..);
    let mut DTYPES = DummyArrayMut::new(DTYPES, 1..);
    let mut STLENS = DummyArrayMut::new(STLENS, 1..);
    let mut DIMS = DummyArrayMut::new(DIMS, 1..);
    let mut INDEXD = DummyArrayMut::new(INDEXD, 1..);
    let mut NULLOK = DummyArrayMut::new(NULLOK, 1..);
    let mut DECLS = DummyCharArrayMut::new(DECLS, None, 1..);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"TSTSCH", ctx)?;
    }

    //
    // Check MXROWS.
    //
    if (MXROWS < 10) {
        spicelib::SETMSG(b"Sorry, MXROWS was #; must be at least 10.", ctx);
        spicelib::ERRINT(b"#", MXROWS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"TSTSCH", ctx)?;
        return Ok(());
    }

    //
    // The table-specific assignments follow.  Declaration strings are
    // built at the end, since the logic is identical for each table.
    //
    if fstr::eq(TABLE, b"SCALAR_1") {
        *SEGTYP = 1;
        *NROWS = 10;
        *NCOLS = 8;

        fstr::assign(CNAMES.get_mut(1), b"TABLE_NAME");
        fstr::assign(CNAMES.get_mut(2), b"FILE_NO");
        fstr::assign(CNAMES.get_mut(3), b"SEGMENT_NO");
        fstr::assign(CNAMES.get_mut(4), b"ROW_NO");
        fstr::assign(CNAMES.get_mut(5), b"C_COL_1");
        fstr::assign(CNAMES.get_mut(6), b"D_COL_1");
        fstr::assign(CNAMES.get_mut(7), b"I_COL_1");
        fstr::assign(CNAMES.get_mut(8), b"T_COL_1");

        DTYPES[1] = CHR;
        DTYPES[2] = INT;
        DTYPES[3] = INT;
        DTYPES[4] = INT;
        DTYPES[5] = CHR;
        DTYPES[6] = DP;
        DTYPES[7] = INT;
        DTYPES[8] = TIME;

        spicelib::CLEARI(*NCOLS, STLENS.as_slice_mut());

        STLENS[1] = TNAMSZ;
        STLENS[5] = IFALSE;

        for I in 1..=*NCOLS {
            DIMS[I] = 1;
            INDEXD[I] = false;
            NULLOK[I] = false;
        }

        CCLASS[1] = 3;
        CCLASS[2] = 1;
        CCLASS[3] = 1;
        CCLASS[4] = 1;
        CCLASS[5] = 3;
        CCLASS[6] = 2;
        CCLASS[7] = 1;
        CCLASS[8] = 2;
    } else if fstr::eq(TABLE, b"SCALAR_2") {
        *SEGTYP = 1;
        *NROWS = MXROWS;
        *NCOLS = 19;

        fstr::assign(CNAMES.get_mut(1), b"TABLE_NAME");
        fstr::assign(CNAMES.get_mut(2), b"FILE_NO");
        fstr::assign(CNAMES.get_mut(3), b"SEGMENT_NO");
        fstr::assign(CNAMES.get_mut(4), b"ROW_NO");
        fstr::assign(CNAMES.get_mut(5), b"C_COL_1");
        fstr::assign(CNAMES.get_mut(6), b"C_COL_2");
        fstr::assign(CNAMES.get_mut(7), b"C_COL_3");
        fstr::assign(CNAMES.get_mut(8), b"C_COL_4");
        fstr::assign(CNAMES.get_mut(9), b"C_COL_5");
        fstr::assign(CNAMES.get_mut(10), b"C_COL_6");
        fstr::assign(CNAMES.get_mut(11), b"D_COL_1");
        fstr::assign(CNAMES.get_mut(12), b"D_COL_2");
        fstr::assign(CNAMES.get_mut(13), b"D_COL_3");
        fstr::assign(CNAMES.get_mut(14), b"I_COL_1");
        fstr::assign(CNAMES.get_mut(15), b"I_COL_2");
        fstr::assign(CNAMES.get_mut(16), b"I_COL_3");
        fstr::assign(CNAMES.get_mut(17), b"T_COL_1");
        fstr::assign(CNAMES.get_mut(18), b"T_COL_2");
        fstr::assign(CNAMES.get_mut(19), b"T_COL_3");

        DTYPES[1] = CHR;
        DTYPES[2] = INT;
        DTYPES[3] = INT;
        DTYPES[4] = INT;
        DTYPES[5] = CHR;
        DTYPES[6] = CHR;
        DTYPES[7] = CHR;
        DTYPES[8] = CHR;
        DTYPES[9] = CHR;
        DTYPES[10] = CHR;
        DTYPES[11] = DP;
        DTYPES[12] = DP;
        DTYPES[13] = DP;
        DTYPES[14] = INT;
        DTYPES[15] = INT;
        DTYPES[16] = INT;
        DTYPES[17] = TIME;
        DTYPES[18] = TIME;
        DTYPES[19] = TIME;

        spicelib::CLEARI(*NCOLS, STLENS.as_slice_mut());

        STLENS[1] = TNAMSZ;
        STLENS[5] = IFALSE;
        STLENS[6] = IFALSE;
        STLENS[7] = IFALSE;
        STLENS[8] = 20;
        STLENS[9] = 20;
        STLENS[10] = 20;

        for I in 1..=*NCOLS {
            DIMS[I] = 1;
            INDEXD[I] = true;
            NULLOK[I] = (I > 4);
        }

        CCLASS[1] = 3;
        CCLASS[2] = 1;
        CCLASS[3] = 1;
        CCLASS[4] = 1;
        CCLASS[5] = 3;
        CCLASS[6] = 3;
        CCLASS[7] = 3;
        CCLASS[8] = 3;
        CCLASS[9] = 3;
        CCLASS[10] = 3;
        CCLASS[11] = 2;
        CCLASS[12] = 2;
        CCLASS[13] = 2;
        CCLASS[14] = 1;
        CCLASS[15] = 1;
        CCLASS[16] = 1;
        CCLASS[17] = 2;
        CCLASS[18] = 2;
        CCLASS[19] = 2;
    } else if fstr::eq(TABLE, b"SCALAR_3") {
        *SEGTYP = 2;
        *NROWS = 10;
        *NCOLS = 8;

        fstr::assign(CNAMES.get_mut(1), b"TABLE_NAME");
        fstr::assign(CNAMES.get_mut(2), b"FILE_NO");
        fstr::assign(CNAMES.get_mut(3), b"SEGMENT_NO");
        fstr::assign(CNAMES.get_mut(4), b"ROW_NO");
        fstr::assign(CNAMES.get_mut(5), b"C_COL_1");
        fstr::assign(CNAMES.get_mut(6), b"D_COL_1");
        fstr::assign(CNAMES.get_mut(7), b"I_COL_1");
        fstr::assign(CNAMES.get_mut(8), b"T_COL_1");

        DTYPES[1] = CHR;
        DTYPES[2] = INT;
        DTYPES[3] = INT;
        DTYPES[4] = INT;
        DTYPES[5] = CHR;
        DTYPES[6] = DP;
        DTYPES[7] = INT;
        DTYPES[8] = TIME;

        spicelib::CLEARI(*NCOLS, STLENS.as_slice_mut());

        STLENS[1] = TNAMSZ;
        STLENS[5] = 20;

        for I in 1..=*NCOLS {
            DIMS[I] = 1;
            INDEXD[I] = false;
            NULLOK[I] = false;
        }

        CCLASS[1] = 9;
        CCLASS[2] = 7;
        CCLASS[3] = 7;
        CCLASS[4] = 7;
        CCLASS[5] = 9;
        CCLASS[6] = 8;
        CCLASS[7] = 7;
        CCLASS[8] = 8;
    } else if fstr::eq(TABLE, b"SCALAR_4") {
        *SEGTYP = 2;
        *NROWS = MXROWS;
        *NCOLS = 16;

        fstr::assign(CNAMES.get_mut(1), b"TABLE_NAME");
        fstr::assign(CNAMES.get_mut(2), b"FILE_NO");
        fstr::assign(CNAMES.get_mut(3), b"SEGMENT_NO");
        fstr::assign(CNAMES.get_mut(4), b"ROW_NO");
        fstr::assign(CNAMES.get_mut(5), b"C_COL_1");
        fstr::assign(CNAMES.get_mut(6), b"C_COL_2");
        fstr::assign(CNAMES.get_mut(7), b"C_COL_3");
        fstr::assign(CNAMES.get_mut(8), b"D_COL_1");
        fstr::assign(CNAMES.get_mut(9), b"D_COL_2");
        fstr::assign(CNAMES.get_mut(10), b"D_COL_3");
        fstr::assign(CNAMES.get_mut(11), b"I_COL_1");
        fstr::assign(CNAMES.get_mut(12), b"I_COL_2");
        fstr::assign(CNAMES.get_mut(13), b"I_COL_3");
        fstr::assign(CNAMES.get_mut(14), b"T_COL_1");
        fstr::assign(CNAMES.get_mut(15), b"T_COL_2");
        fstr::assign(CNAMES.get_mut(16), b"T_COL_3");

        DTYPES[1] = CHR;
        DTYPES[2] = INT;
        DTYPES[3] = INT;
        DTYPES[4] = INT;
        DTYPES[5] = CHR;
        DTYPES[6] = CHR;
        DTYPES[7] = CHR;
        DTYPES[8] = DP;
        DTYPES[9] = DP;
        DTYPES[10] = DP;
        DTYPES[11] = INT;
        DTYPES[12] = INT;
        DTYPES[13] = INT;
        DTYPES[14] = TIME;
        DTYPES[15] = TIME;
        DTYPES[16] = TIME;

        spicelib::CLEARI(*NCOLS, STLENS.as_slice_mut());

        STLENS[1] = TNAMSZ;
        STLENS[5] = 20;
        STLENS[6] = 20;
        STLENS[7] = 20;

        for I in 1..=*NCOLS {
            DIMS[I] = 1;
            INDEXD[I] = true;
            NULLOK[I] = (I > 4);
        }

        CCLASS[1] = 9;
        CCLASS[2] = 7;
        CCLASS[3] = 7;
        CCLASS[4] = 7;
        CCLASS[5] = 9;
        CCLASS[6] = 9;
        CCLASS[7] = 9;
        CCLASS[8] = 8;
        CCLASS[9] = 8;
        CCLASS[10] = 8;
        CCLASS[11] = 7;
        CCLASS[12] = 7;
        CCLASS[13] = 7;
        CCLASS[14] = 8;
        CCLASS[15] = 8;
        CCLASS[16] = 8;
    } else if fstr::eq(TABLE, b"VECTOR_1") {
        *SEGTYP = 1;
        *NROWS = 3;
        *NCOLS = 12;

        fstr::assign(CNAMES.get_mut(1), b"TABLE_NAME");
        fstr::assign(CNAMES.get_mut(2), b"FILE_NO");
        fstr::assign(CNAMES.get_mut(3), b"SEGMENT_NO");
        fstr::assign(CNAMES.get_mut(4), b"ROW_NO");
        fstr::assign(CNAMES.get_mut(5), b"C_COL_1");
        fstr::assign(CNAMES.get_mut(6), b"C_COL_2");
        fstr::assign(CNAMES.get_mut(7), b"D_COL_1");
        fstr::assign(CNAMES.get_mut(8), b"D_COL_2");
        fstr::assign(CNAMES.get_mut(9), b"I_COL_1");
        fstr::assign(CNAMES.get_mut(10), b"I_COL_2");
        fstr::assign(CNAMES.get_mut(11), b"T_COL_1");
        fstr::assign(CNAMES.get_mut(12), b"T_COL_2");

        DTYPES[1] = CHR;
        DTYPES[2] = INT;
        DTYPES[3] = INT;
        DTYPES[4] = INT;
        DTYPES[5] = CHR;
        DTYPES[6] = CHR;
        DTYPES[7] = DP;
        DTYPES[8] = DP;
        DTYPES[9] = INT;
        DTYPES[10] = INT;
        DTYPES[11] = TIME;
        DTYPES[12] = TIME;

        spicelib::CLEARI(*NCOLS, STLENS.as_slice_mut());

        STLENS[1] = TNAMSZ;
        STLENS[5] = 1024;
        STLENS[6] = 100;

        DIMS[1] = 1;
        DIMS[2] = 1;
        DIMS[3] = 1;
        DIMS[4] = 1;
        DIMS[5] = 3;
        DIMS[6] = IFALSE;
        DIMS[7] = 4;
        DIMS[8] = IFALSE;
        DIMS[9] = 5;
        DIMS[10] = IFALSE;
        DIMS[11] = 6;
        DIMS[12] = IFALSE;

        for I in 1..=*NCOLS {
            INDEXD[I] = false;
            NULLOK[I] = false;
        }

        CCLASS[1] = 3;
        CCLASS[2] = 1;
        CCLASS[3] = 1;
        CCLASS[4] = 1;
        CCLASS[5] = 6;
        CCLASS[6] = 6;
        CCLASS[7] = 5;
        CCLASS[8] = 5;
        CCLASS[9] = 4;
        CCLASS[10] = 4;
        CCLASS[11] = 5;
        CCLASS[12] = 5;
    } else if fstr::eq(TABLE, b"VECTOR_2") {
        *SEGTYP = 1;
        *NROWS = 3;
        *NCOLS = 20;

        fstr::assign(CNAMES.get_mut(1), b"TABLE_NAME");
        fstr::assign(CNAMES.get_mut(2), b"FILE_NO");
        fstr::assign(CNAMES.get_mut(3), b"SEGMENT_NO");
        fstr::assign(CNAMES.get_mut(4), b"ROW_NO");
        fstr::assign(CNAMES.get_mut(5), b"C_COL_1");
        fstr::assign(CNAMES.get_mut(6), b"C_COL_2");
        fstr::assign(CNAMES.get_mut(7), b"C_COL_3");
        fstr::assign(CNAMES.get_mut(8), b"C_COL_4");
        fstr::assign(CNAMES.get_mut(9), b"D_COL_1");
        fstr::assign(CNAMES.get_mut(10), b"D_COL_2");
        fstr::assign(CNAMES.get_mut(11), b"D_COL_3");
        fstr::assign(CNAMES.get_mut(12), b"D_COL_4");
        fstr::assign(CNAMES.get_mut(13), b"I_COL_1");
        fstr::assign(CNAMES.get_mut(14), b"I_COL_2");
        fstr::assign(CNAMES.get_mut(15), b"I_COL_3");
        fstr::assign(CNAMES.get_mut(16), b"I_COL_4");
        fstr::assign(CNAMES.get_mut(17), b"T_COL_1");
        fstr::assign(CNAMES.get_mut(18), b"T_COL_2");
        fstr::assign(CNAMES.get_mut(19), b"T_COL_3");
        fstr::assign(CNAMES.get_mut(20), b"T_COL_4");

        DTYPES[1] = CHR;
        DTYPES[2] = INT;
        DTYPES[3] = INT;
        DTYPES[4] = INT;
        DTYPES[5] = CHR;
        DTYPES[6] = CHR;
        DTYPES[7] = CHR;
        DTYPES[8] = CHR;
        DTYPES[9] = DP;
        DTYPES[10] = DP;
        DTYPES[11] = DP;
        DTYPES[12] = DP;
        DTYPES[13] = INT;
        DTYPES[14] = INT;
        DTYPES[15] = INT;
        DTYPES[16] = INT;
        DTYPES[17] = TIME;
        DTYPES[18] = TIME;
        DTYPES[19] = TIME;
        DTYPES[20] = TIME;

        spicelib::CLEARI(*NCOLS, STLENS.as_slice_mut());

        STLENS[1] = TNAMSZ;
        STLENS[5] = 1024;
        STLENS[6] = 1024;
        STLENS[7] = 1024;
        STLENS[8] = 1024;

        DIMS[1] = 1;
        DIMS[2] = 1;
        DIMS[3] = 1;
        DIMS[4] = 1;
        DIMS[5] = 3;
        DIMS[6] = 5;
        DIMS[7] = IFALSE;
        DIMS[8] = IFALSE;
        DIMS[9] = 4;
        DIMS[10] = 6;
        DIMS[11] = IFALSE;
        DIMS[12] = IFALSE;
        DIMS[13] = 5;
        DIMS[14] = 7;
        DIMS[15] = IFALSE;
        DIMS[16] = IFALSE;
        DIMS[17] = 6;
        DIMS[18] = 8;
        DIMS[19] = IFALSE;
        DIMS[20] = IFALSE;

        for I in 1..=*NCOLS {
            INDEXD[I] = (I <= 4);
            NULLOK[I] = (I > 4);
        }

        CCLASS[1] = 3;
        CCLASS[2] = 1;
        CCLASS[3] = 1;
        CCLASS[4] = 1;
        CCLASS[5] = 6;
        CCLASS[6] = 6;
        CCLASS[7] = 6;
        CCLASS[8] = 6;
        CCLASS[9] = 5;
        CCLASS[10] = 5;
        CCLASS[11] = 5;
        CCLASS[12] = 5;
        CCLASS[13] = 4;
        CCLASS[14] = 4;
        CCLASS[15] = 4;
        CCLASS[16] = 4;
        CCLASS[17] = 5;
        CCLASS[18] = 5;
        CCLASS[19] = 5;
        CCLASS[20] = 5;
    } else {
        spicelib::SETMSG(b"Table # does not exist.", ctx);
        spicelib::ERRCH(b"#", TABLE, ctx);
        spicelib::SIGERR(b"SPICE(NOSUCHTABLE)", ctx)?;
        spicelib::CHKOUT(b"TSTSCH", ctx)?;
        return Ok(());
    }

    //
    // Build the declaration strings for the columns.
    //
    for I in 1..=*NCOLS {
        //
        // Fill in the data type assigment.  For string-valued columns,
        // this includes a string length.
        //
        fstr::assign(DECLS.get_mut(I), b"DATATYPE = ");

        if (DTYPES[I] == CHR) {
            spicelib::SUFFIX(b"CHARACTER*(#)", 1, &mut DECLS[I]);

            if (STLENS[I] > 0) {
                spicelib::REPMI(&DECLS[I].to_vec(), b"#", STLENS[I], &mut DECLS[I], ctx);
            } else {
                spicelib::REPMC(&DECLS[I].to_vec(), b"#", b"*", &mut DECLS[I]);
            }
        } else if (DTYPES[I] == DP) {
            spicelib::SUFFIX(b"DOUBLE PRECISION", 1, &mut DECLS[I]);
        } else if (DTYPES[I] == INT) {
            spicelib::SUFFIX(b"INTEGER", 1, &mut DECLS[I]);
        } else {
            spicelib::SUFFIX(b"TIME", 1, &mut DECLS[I]);
        }

        //
        // If the dimension is not 1, add a dimension
        // specifier.
        //
        if (DIMS[I] > 1) {
            spicelib::SUFFIX(b", SIZE = #", 0, &mut DECLS[I]);

            spicelib::REPMI(&DECLS[I].to_vec(), b"#", DIMS[I], &mut DECLS[I], ctx);
        } else if (DIMS[I] == IFALSE) {
            //
            // This column has variable dimension.
            //
            spicelib::SUFFIX(b", SIZE = #", 0, &mut DECLS[I]);

            spicelib::REPMC(&DECLS[I].to_vec(), b"#", b"VARIABLE", &mut DECLS[I]);
        }

        //
        // If the column is indexed, add an index specifier.
        //
        if INDEXD[I] {
            spicelib::SUFFIX(b", INDEXED = TRUE", 0, &mut DECLS[I]);
        }

        //
        // If the column may contain nulls, add a null value specifier.
        //
        if NULLOK[I] {
            spicelib::SUFFIX(b", NULLS_OK = TRUE", 0, &mut DECLS[I]);
        }

        //
        // If the segment has type 2, the column must be a fixed count
        // column.  Add the fixed count specifier in this case.
        //
        if (*SEGTYP == 2) {
            spicelib::SUFFIX(b", FIXED_COUNT = TRUE", 0, &mut DECLS[I]);
        }
    }

    spicelib::CHKOUT(b"TSTSCH", ctx)?;
    Ok(())
}
