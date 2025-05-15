//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ADSCSZ: i32 = 6;
const ATTCLS: i32 = 1;
const ATTTYP: i32 = (ATTCLS + 1);
const ATTLEN: i32 = (ATTTYP + 1);
const ATTSIZ: i32 = (ATTLEN + 1);
const ATTIDX: i32 = (ATTSIZ + 1);
const ATTNFL: i32 = (ATTIDX + 1);
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
const CNAMSZ: i32 = 32;
const EQ: i32 = 1;
const GE: i32 = (EQ + 1);
const GT: i32 = (GE + 1);
const LE: i32 = (GT + 1);
const LT: i32 = (LE + 1);
const NE: i32 = (LT + 1);
const LIKE: i32 = (NE + 1);
const UNLIKE: i32 = (LIKE + 1);
const ISNULL: i32 = (UNLIKE + 1);
const NOTNUL: i32 = (ISNULL + 1);
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
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const EK1: &[u8] = b"test1.ek";
const DECLEN: i32 = 200;
const IFNLEN: i32 = 60;
const LNSIZE: i32 = 80;
const MAXENT: i32 = 20;
const MAXROW: i32 = 20;
const CVALSZ: i32 = (2 * MAXSTR);
const NTABS: i32 = 6;
const MAXVAL: i32 = MAXENT;

struct SaveVars {
    CDATA: ActualCharArray,
    CVAL: Vec<u8>,
    CNAMES: ActualCharArray,
    DECLS: ActualCharArray,
    TABNAM: Vec<u8>,
    DDATA: StackArray<f64, 20>,
    DVAL: f64,
    TDATA: StackArray<f64, 20>,
    CLDSC1: StackArray<i32, 11>,
    CLDSC3: StackArray<i32, 11>,
    CLDSC4: StackArray<i32, 11>,
    CLDSC5: StackArray<i32, 11>,
    DTYPE: i32,
    ELTIDX: i32,
    ENTSZS: StackArray<i32, 20>,
    HANDLE: i32,
    IDATA: StackArray<i32, 20>,
    IVAL: i32,
    NCOLS: i32,
    NROWS: i32,
    RCPTRS: StackArray<i32, 20>,
    RECPTR: i32,
    ROW: i32,
    SEGDSC: StackArray<i32, 24>,
    SEGNO: i32,
    WKINDX: StackArray<i32, 20>,
    MATCH: bool,
    NLFLGS: StackArray<bool, 20>,
    NULL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CDATA = ActualCharArray::new(CVALSZ, 1..=MAXROW);
        let mut CVAL = vec![b' '; CVALSZ as usize];
        let mut CNAMES = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
        let mut DECLS = ActualCharArray::new(DECLEN, 1..=MXCLSG);
        let mut TABNAM = vec![b' '; TNAMSZ as usize];
        let mut DDATA = StackArray::<f64, 20>::new(1..=MAXROW);
        let mut DVAL: f64 = 0.0;
        let mut TDATA = StackArray::<f64, 20>::new(1..=MAXROW);
        let mut CLDSC1 = StackArray::<i32, 11>::new(1..=CDSCSZ);
        let mut CLDSC3 = StackArray::<i32, 11>::new(1..=CDSCSZ);
        let mut CLDSC4 = StackArray::<i32, 11>::new(1..=CDSCSZ);
        let mut CLDSC5 = StackArray::<i32, 11>::new(1..=CDSCSZ);
        let mut DTYPE: i32 = 0;
        let mut ELTIDX: i32 = 0;
        let mut ENTSZS = StackArray::<i32, 20>::new(1..=MAXROW);
        let mut HANDLE: i32 = 0;
        let mut IDATA = StackArray::<i32, 20>::new(1..=MAXROW);
        let mut IVAL: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut RCPTRS = StackArray::<i32, 20>::new(1..=MAXROW);
        let mut RECPTR: i32 = 0;
        let mut ROW: i32 = 0;
        let mut SEGDSC = StackArray::<i32, 24>::new(1..=SDSCSZ);
        let mut SEGNO: i32 = 0;
        let mut WKINDX = StackArray::<i32, 20>::new(1..=MAXROW);
        let mut MATCH: bool = false;
        let mut NLFLGS = StackArray::<bool, 20>::new(1..=MAXROW);
        let mut NULL: bool = false;

        Self {
            CDATA,
            CVAL,
            CNAMES,
            DECLS,
            TABNAM,
            DDATA,
            DVAL,
            TDATA,
            CLDSC1,
            CLDSC3,
            CLDSC4,
            CLDSC5,
            DTYPE,
            ELTIDX,
            ENTSZS,
            HANDLE,
            IDATA,
            IVAL,
            NCOLS,
            NROWS,
            RCPTRS,
            RECPTR,
            ROW,
            SEGDSC,
            SEGNO,
            WKINDX,
            MATCH,
            NLFLGS,
            NULL,
        }
    }
}

//$Procedure F_EK04 ( EK tests, subset 4 )
pub fn F_EK04(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    // CHARACTER*(*)         EK2
    // PARAMETER           ( EK2    = 'test2.ek' )

    //
    // Local Variables
    //

    //
    // Saved variables
    //
    //
    // To avoid portability problems, make all data static.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_EK04", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Setup:  create a custom EK.", ctx)?;
    //
    // We need an EK having columns containing scalar strings
    // longer than MAXSTR.
    //
    // Open a new EK.
    //
    spicelib::EKOPN(EK1, EK1, 0, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Since many EK routines don't return upon entry,
    // exit here if we couldn't open the file.
    //
    if !*OK {
        testutil::T_SUCCESS(OK, ctx);
        return Ok(());
    }

    //
    // Start building a segment by creating a schema for an EK table.
    //
    fstr::assign(&mut save.TABNAM, b"TAB1");

    save.NCOLS = 5;
    save.NROWS = 5;

    fstr::assign(save.CNAMES.get_mut(1), b"CCOL1");
    fstr::assign(save.CNAMES.get_mut(2), b"CCOL2");
    fstr::assign(save.CNAMES.get_mut(3), b"ICOL1");
    fstr::assign(save.CNAMES.get_mut(4), b"DCOL1");
    fstr::assign(save.CNAMES.get_mut(5), b"TCOL1");

    fstr::assign(
        save.DECLS.get_mut(1),
        b"DATATYPE = CHARACTER*(*), INDEXED  = TRUE, NULLS_OK = TRUE",
    );

    let val = save.DECLS.get(1).to_vec();
    fstr::assign(save.DECLS.get_mut(2), &val);

    fstr::assign(
        save.DECLS.get_mut(3),
        b"DATATYPE = INTEGER, INDEXED  = TRUE, NULLS_OK = TRUE",
    );

    fstr::assign(
        save.DECLS.get_mut(4),
        b"DATATYPE = DOUBLE PRECISION, INDEXED  = TRUE, NULLS_OK = TRUE",
    );

    fstr::assign(
        save.DECLS.get_mut(5),
        b"DATATYPE = TIME, INDEXED  = TRUE, NULLS_OK = TRUE",
    );

    //
    // Initiate a fast load.
    //
    spicelib::EKIFLD(
        save.HANDLE,
        &save.TABNAM,
        save.NCOLS,
        save.NROWS,
        save.CNAMES.as_arg(),
        save.DECLS.as_arg(),
        &mut save.SEGNO,
        save.RCPTRS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the data for the first column. We need strings
    // longer than MAXSTR, and we want both strings that
    // are identical up to their first MAXSTR characters, and
    // strings that differ in their first MAXSTR characters.
    //
    for I in 1..=save.NROWS {
        fstr::assign(save.CDATA.get_mut(I), b" ");
    }

    for I in 1..=(MAXSTR - 1) {
        fstr::assign(fstr::substr_mut(save.CDATA.get_mut(1), I..=I), b"X");
        fstr::assign(fstr::substr_mut(save.CDATA.get_mut(2), I..=I), b"X");
        fstr::assign(fstr::substr_mut(save.CDATA.get_mut(3), I..=I), b"X");
        fstr::assign(fstr::substr_mut(save.CDATA.get_mut(4), I..=I), b"X");
    }

    fstr::assign(
        fstr::substr_mut(save.CDATA.get_mut(1), MAXSTR..=(MAXSTR + 2)),
        b"ZAB",
    );
    fstr::assign(
        fstr::substr_mut(save.CDATA.get_mut(2), MAXSTR..=(MAXSTR + 2)),
        b"ZAC",
    );
    fstr::assign(
        fstr::substr_mut(save.CDATA.get_mut(3), MAXSTR..=(MAXSTR + 2)),
        b"WAB",
    );
    fstr::assign(
        fstr::substr_mut(save.CDATA.get_mut(4), MAXSTR..=(MAXSTR + 2)),
        b"VAB",
    );
    fstr::assign(
        fstr::substr_mut(save.CDATA.get_mut(5), MAXSTR..=(MAXSTR + 2)),
        b"VAB",
    );

    //
    // Add the first column to the segment.
    //
    spicelib::FILLI(1, MAXROW, save.ENTSZS.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=MAXROW {
        save.NLFLGS[I] = false;
    }

    //
    // Make the last entry null.
    //
    save.NLFLGS[save.NROWS] = true;

    spicelib::EKACLC(
        save.HANDLE,
        save.SEGNO,
        &save.CNAMES[1],
        save.CDATA.as_arg(),
        save.ENTSZS.as_slice(),
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Give the second column the same data as the first.
    //
    spicelib::EKACLC(
        save.HANDLE,
        save.SEGNO,
        &save.CNAMES[2],
        save.CDATA.as_arg(),
        save.ENTSZS.as_slice(),
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create data for and load the integer column.
    //
    for I in 1..=save.NROWS {
        save.IDATA[I] = (100 * I);
    }

    spicelib::EKACLI(
        save.HANDLE,
        save.SEGNO,
        &save.CNAMES[3],
        save.IDATA.as_slice(),
        save.ENTSZS.as_slice(),
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create data for and load the d.p. column.
    //
    for I in 1..=save.NROWS {
        save.DDATA[I] = ((100 * I) as f64);
    }

    spicelib::EKACLD(
        save.HANDLE,
        save.SEGNO,
        &save.CNAMES[4],
        save.DDATA.as_slice(),
        save.ENTSZS.as_slice(),
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create data for and load the time column.
    //
    for I in 1..=save.NROWS {
        save.TDATA[I] = ((I as f64) * 86400.0);
    }

    spicelib::EKACLD(
        save.HANDLE,
        save.SEGNO,
        &save.CNAMES[5],
        save.TDATA.as_slice(),
        save.ENTSZS.as_slice(),
        save.NLFLGS.as_slice(),
        save.RCPTRS.as_slice(),
        save.WKINDX.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now that we've finished adding columns to the segment,
    // finish the load.
    //
    spicelib::EKFFLD(save.HANDLE, save.SEGNO, save.RCPTRS.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close the EK.
    //
    spicelib::EKCLS(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load the EK for read access. We'll use the handle, so
    // it's easier to use the load entry point from the query
    // manager.
    //
    spicelib::EKLEF(EK1, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on long strings.
    //
    testutil::TCASE(
        b"ZZEKSCMP: compare strings that are identical up to length MAXSTR.",
        ctx,
    )?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the first column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[1],
        save.CLDSC1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 1 to the second entry.
    // Since these entries agree up to the first MAXSTR characters,
    // we expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = CHR;
    fstr::assign(&mut save.CVAL, save.CDATA.get(2));
    save.DVAL = 0.0;
    save.IVAL = 0;
    save.NULL = false;
    save.ROW = 1;
    //
    // For class three columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD03 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 1.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (0)", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (0)", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on long strings.
    //
    testutil::TCASE(
        b"ZZEKSCMP: compare a string element than is less than a given scalar value.",
        ctx,
    )?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the first column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[1],
        save.CLDSC1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 1 to the second entry.
    // Since these entries agree up to the first MAXSTR characters,
    // we expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = CHR;
    fstr::assign(&mut save.CVAL, save.CDATA.get(1));
    save.DVAL = 0.0;
    save.IVAL = 0;
    save.NULL = false;
    save.ROW = 4;
    //
    // For class three columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD03 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 4.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XV",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (0)", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (0)", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XV",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on long strings.
    //
    testutil::TCASE(
        b"ZZEKSCMP: compare a string element than is greater than a given scalar value.",
        ctx,
    )?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the first column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[1],
        save.CLDSC1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 1 to the second entry.
    // Since these entries agree up to the first MAXSTR characters,
    // we expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = CHR;
    fstr::assign(&mut save.CVAL, save.CDATA.get(4));
    save.DVAL = 0.0;
    save.IVAL = 0;
    save.NULL = false;
    save.ROW = 1;
    //
    // For class three columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD03 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 1.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (0)", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (0)", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on long strings.
    //
    testutil::TCASE(
        b"ZZEKSCMP: compare a non-null string element to a null value.",
        ctx,
    )?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the first column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[1],
        save.CLDSC1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 1 to the second entry.
    // Since these entries agree up to the first MAXSTR characters,
    // we expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = CHR;
    fstr::assign(&mut save.CVAL, save.CDATA.get(4));
    save.DVAL = 0.0;
    save.IVAL = 0;
    save.NULL = true;
    save.ROW = 1;
    //
    // For class three columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD03 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 1.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (0)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (0)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on long strings.
    //
    testutil::TCASE(
        b"ZZEKSCMP: compare a null string element to a null value.",
        ctx,
    )?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the first column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[1],
        save.CLDSC1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 1 to the second entry.
    // Since these entries agree up to the first MAXSTR characters,
    // we expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = CHR;
    fstr::assign(&mut save.CVAL, save.CDATA.get(5));
    save.DVAL = 0.0;
    save.IVAL = 0;
    save.NULL = true;
    save.ROW = 5;
    //
    // For class three columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD03 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 5.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (0)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (0)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on long strings.
    //
    testutil::TCASE(
        b"ZZEKSCMP: compare a null string element to a non-null value.",
        ctx,
    )?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the first column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[1],
        save.CLDSC1.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 1 to the second entry.
    // Since these entries agree up to the first MAXSTR characters,
    // we expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = CHR;
    fstr::assign(&mut save.CVAL, save.CDATA.get(4));
    save.DVAL = 0.0;
    save.IVAL = 0;
    save.NULL = false;
    save.ROW = 5;
    //
    // For class three columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD03 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 5.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (0)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*X",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (0)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        UNLIKE,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        b"*XZ",
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOT LIKE MATCH (1)", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC1.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, false, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on integers.
    //
    testutil::TCASE(b"ZZEKSCMP: compare matching integers.", ctx)?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the third column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[3],
        save.CLDSC3.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 1 to the first entry.
    // We expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = INT;
    fstr::assign(&mut save.CVAL, b" ");
    save.DVAL = 0.0;
    save.IVAL = save.IDATA[1];
    save.NULL = false;
    save.ROW = 1;
    //
    // For class one columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD01 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 1.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC3.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC3.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC3.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC3.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC3.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on d.p numbers.
    //
    testutil::TCASE(b"ZZEKSCMP: compare matching d.p. numbers.", ctx)?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the fourth column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[4],
        save.CLDSC4.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 4 to the first entry.
    // We expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = DP;
    fstr::assign(&mut save.CVAL, b" ");
    save.DVAL = save.DDATA[1];
    save.IVAL = 0;
    save.NULL = false;
    save.ROW = 1;
    //
    // For class two columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD02 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 1.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC4.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC4.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC4.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC4.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC4.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, true, OK, ctx)?;

    // --- Case: ------------------------------------------------------
    //

    //
    // Test operation of ZZEKSCMP on times.
    //
    testutil::TCASE(b"ZZEKSCMP: compare matching times.", ctx)?;

    //
    // Get the segment descriptor for the first segment of EK1.
    //
    spicelib::ZZEKSDSC(save.HANDLE, 1, save.SEGDSC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the column descriptor for the fifth column.
    //
    spicelib::ZZEKCDSC(
        save.HANDLE,
        save.SEGDSC.as_slice(),
        &save.CNAMES[5],
        save.CLDSC5.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the first entry of column 5 to the first entry.
    // We expect equality.
    //
    save.ELTIDX = 1;
    save.DTYPE = TIME;
    fstr::assign(&mut save.CVAL, b" ");
    save.DVAL = save.TDATA[1];
    save.IVAL = 0;
    save.NULL = false;
    save.ROW = 1;
    //
    // For class two columns, the "row" input required by ZZEKSCMP
    // is actually a pointer to the base of a data structure called an
    // "EK record pointer." (See usage in ZZEKRD02 for an example;
    // also see ekrecptr.inc.) We need to obtain this record pointer
    // for the row of interest: row 1.
    //
    spicelib::ZZEKTRDP(
        save.HANDLE,
        save.SEGDSC[RTIDX],
        save.ROW,
        &mut save.RECPTR,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        EQ,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC5.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"EQ MATCH", save.MATCH, true, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        LT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC5.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"LT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        GT,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC5.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"GT MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        ISNULL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC5.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"ISNULL MATCH", save.MATCH, false, OK, ctx)?;

    save.MATCH = spicelib::ZZEKSCMP(
        NOTNUL,
        save.HANDLE,
        save.SEGDSC.as_slice(),
        save.CLDSC5.as_slice(),
        save.RECPTR,
        save.ELTIDX,
        save.DTYPE,
        &save.CVAL,
        save.DVAL,
        save.IVAL,
        save.NULL,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"NOTNUL MATCH", save.MATCH, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //

    //
    // Unload the EKs and delete the files.
    //
    testutil::TCASE(b"Unload EKs from query system.", ctx)?;

    spicelib::EKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(EK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
