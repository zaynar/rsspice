//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure       SYPTRC ( Symbol table, fetch pointers, generic )
pub fn SYPTRC(
    NAME: &[u8],
    SYMNAM: CharArray,
    SYMPTR: &[i32],
    SYMVAL: CharArrayMut,
    PTR: &mut i32,
    N: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SYMNAM = DummyCharArray::new(SYMNAM, None, LBCELL..);
    let SYMPTR = DummyArray::new(SYMPTR, LBCELL..);
    let mut SYMVAL = DummyCharArrayMut::new(SYMVAL, None, LBCELL..);
    let mut LOC: i32 = 0;
    let mut NUMBER: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"SYPTRC", ctx)?;
    }
    //
    // We don't use the values of the symbol table in this routine
    // but it is passed for the sake of uniformity in the symbol
    // table routine calling sequences.  However, some compilers
    // generate warnings if a variable isn't used.  So we touch
    // the values cell to fake out the compiler.
    //
    spicelib::TOUCHC(
        &SYMVAL[LBCELL].to_vec(),
        fstr::substr_mut(&mut SYMVAL[LBCELL], 1..=1),
    );

    //
    // Now for the real work of this routine.
    //
    NUMBER = spicelib::CARDC(SYMNAM.as_arg(), ctx)?;
    LOC = spicelib::BSRCHC(NAME, NUMBER, SYMNAM.subarray(1));

    if (LOC == 0) {
        *FOUND = false;
        *PTR = 0;
        *N = 0;
        spicelib::CHKOUT(b"SYPTRC", ctx)?;
        return Ok(());
    }

    *PTR = (spicelib::SUMAI(SYMPTR.subarray(1), (LOC - 1)) + 1);
    *N = SYMPTR[LOC];
    *FOUND = true;

    spicelib::CHKOUT(b"SYPTRC", ctx)?;

    Ok(())
}
