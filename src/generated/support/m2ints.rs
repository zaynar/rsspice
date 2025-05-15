//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      M2INTS (Meta 2 --- initialize syntax table)
pub fn M2INTS(
    NSYN: i32,
    SYNKEY: CharArrayMut,
    SYNPTR: &mut [i32],
    SYNVAL: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SYNKEY = DummyCharArrayMut::new(SYNKEY, None, LBCELL..);
    let mut SYNPTR = DummyArrayMut::new(SYNPTR, LBCELL..);
    let mut SYNVAL = DummyCharArrayMut::new(SYNVAL, None, LBCELL..);
    let mut KEYWRD = [b' '; 32 as usize];
    let mut LSTKEY = [b' '; 32 as usize];
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut PUT: i32 = 0;

    //
    // Spicelib functions.
    //

    //
    // Local variables.
    //

    //
    // Initialize the symbol table size attributes.
    //
    spicelib::SSIZEC(NSYN, SYNKEY.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(NSYN, SYNPTR.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(NSYN, SYNVAL.as_arg_mut(), ctx)?;

    //
    // Just in case, left justify everything in the values cell
    // and set all of the pointer values to 0.
    //
    for I in 1..=NSYN {
        spicelib::LJUST(&SYNVAL[I].to_vec(), &mut SYNVAL[I]);
        SYNPTR[I] = 0;
    }

    //
    // Turn the collection of syntax definitions into an array ordered
    // by initial keyword (minus any labels).
    //
    M2SHLL(NSYN, SYNVAL.subarray_mut(1));
    //
    // Remove any duplicates including a blank at the beginning if
    // there is one.
    //
    PUT = 0;
    fstr::assign(SYNVAL.get_mut(0), b" ");
    for I in 1..=NSYN {
        if fstr::ne(SYNVAL.get(I), SYNVAL.get((I - 1))) {
            PUT = (PUT + 1);
            let val = SYNVAL.get(I).to_vec();
            fstr::assign(SYNVAL.get_mut(PUT), &val);
        }
    }

    spicelib::SSIZEC(NSYN, SYNVAL.as_arg_mut(), ctx)?;
    spicelib::SCARDC(PUT, SYNVAL.as_arg_mut(), ctx)?;

    //
    // Now we will construct the symbol table to go with this collection
    // of syntax definitions.
    //
    fstr::assign(&mut LSTKEY, b" ");
    PUT = 0;

    for I in 1..=spicelib::CARDC(SYNVAL.as_arg(), ctx)? {
        //
        // Get the first word, and trim off any attached label.  Note that
        // since this is supposed to be a keyword, there are no range
        // templates or qualifiers attached.
        //
        spicelib::FNDNWD(&SYNVAL[I], 1, &mut B, &mut E);
        M2TRIM(fstr::substr(&SYNVAL[I], B..=E), &mut KEYWRD, ctx);
        spicelib::UCASE(&KEYWRD.clone(), &mut KEYWRD, ctx);

        //
        // If this is a new keyword, put it into the list of keywords and
        // change the last keyword.
        //
        if fstr::ne(&KEYWRD, &LSTKEY) {
            PUT = (PUT + 1);
            fstr::assign(SYNKEY.get_mut(PUT), &KEYWRD);
            fstr::assign(&mut LSTKEY, &KEYWRD);
        }

        //
        // Increment the value in the pointer array.
        //
        SYNPTR[PUT] = (SYNPTR[PUT] + 1);
    }

    //
    // Set the cardinality of the name and pointer cells.
    //
    spicelib::SCARDC(PUT, SYNKEY.as_arg_mut(), ctx)?;
    spicelib::SCARDI(PUT, SYNPTR.as_slice_mut(), ctx)?;

    //
    // Finally, blank out all of the non-used parts of the values cell.
    //
    for I in -5..=-2 {
        fstr::assign(SYNVAL.get_mut(I), b" ");
    }

    Ok(())
}
