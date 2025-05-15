//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const ROOM: i32 = 10;

//$Procedure      M2CHCK ( Meta-2, check a table of syntax definitions )
pub fn M2CHCK(
    STATMN: &mut [u8],
    SYNKEY: CharArray,
    SYNPTR: &[i32],
    SYNVAL: CharArrayMut,
    ERROR: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SYNKEY = DummyCharArray::new(SYNKEY, None, LBCELL..);
    let SYNPTR = DummyArray::new(SYNPTR, LBCELL..);
    let mut SYNVAL = DummyCharArrayMut::new(SYNVAL, None, LBCELL..);
    let mut ERROR = DummyCharArrayMut::new(ERROR, None, 1..);
    let mut BEST = StackArray::<i32, 16>::new(LBCELL..=ROOM);
    let mut SCORES = StackArray::<i32, 16>::new(LBCELL..=ROOM);
    let mut B: i32 = 0;
    let mut BST: i32 = 0;
    let mut CUTOFF: i32 = 0;
    let mut E: i32 = 0;
    let mut LOOKAT: i32 = 0;
    let mut MXSCOR: i32 = 0;
    let mut N: i32 = 0;
    let mut PTR: i32 = 0;
    let mut FOUND: bool = false;
    let mut UNKNWN: bool = false;
    let mut KEYWRD = [b' '; 32 as usize];
    let mut MSSG = [b' '; 160 as usize];

    //
    // Spicelib functions
    //

    //
    // Local Variables
    //

    if spicelib::RETURN(ctx) {
        fstr::assign(ERROR.get_mut(1), b"M2CHCK: The function RETURN was set to .TRUE. This situation is not supposed to happen.");
        return Ok(());
    }

    //
    // Initialize the cell BEST and SCORES.
    //
    spicelib::SSIZEI(ROOM, BEST.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(ROOM, SCORES.as_slice_mut(), ctx)?;

    //
    // Get the first word of the input string.
    //
    spicelib::FNDNWD(STATMN, 1, &mut B, &mut E);
    spicelib::UCASE(fstr::substr(STATMN, B..=E), &mut KEYWRD, ctx);

    //
    // Find the syntax templates that match the first word of the
    // command.
    //
    SYPTRC(
        &KEYWRD,
        SYNKEY.as_arg(),
        SYNPTR.as_slice(),
        SYNVAL.as_arg_mut(),
        &mut PTR,
        &mut N,
        &mut FOUND,
        ctx,
    )?;

    //
    // If we didn't find our word, then we look for a word that
    // comes close spelling-wise
    //
    if !FOUND {
        CUTOFF = 70;
        BESTWD(
            &KEYWRD,
            SYNKEY.as_arg(),
            CUTOFF,
            BEST.as_slice_mut(),
            SCORES.as_slice_mut(),
            &mut MSSG,
            ctx,
        )?;

        if (spicelib::CARDI(BEST.as_slice(), ctx)? == 0) {
            UNKNWN = true;
        } else if (SCORES[1] < 50) {
            UNKNWN = true;
        } else {
            UNKNWN = false;
        }

        if UNKNWN {
            fstr::assign(ERROR.get_mut(1), b"Sorry but I didn\'t recognize the word");
            spicelib::SUFFIX(&KEYWRD, 1, &mut ERROR[1]);
            spicelib::SUFFIX(
                b"as the beginning of any valid statement. ",
                1,
                &mut ERROR[1],
            );
            return Ok(());
        }

        //
        // Still here? fetch the set of likely syntax statements to check.
        //
        MXSCOR = 0;

        for I in 1..=spicelib::CARDI(BEST.as_slice(), ctx)? {
            if (SCORES[I] > MXSCOR) {
                MXSCOR = SCORES[I];
                LOOKAT = I;
            }
        }

        fstr::assign(&mut KEYWRD, SYNKEY.get(BEST[LOOKAT]));

        SYPTRC(
            &KEYWRD,
            SYNKEY.as_arg(),
            SYNPTR.as_slice(),
            SYNVAL.as_arg_mut(),
            &mut PTR,
            &mut N,
            &mut FOUND,
            ctx,
        )?;
    }

    //
    // Until we find out otherwise, we shall assume that we have
    // a syntactically correct input statement.
    //
    META_2(
        STATMN,
        SYNVAL.subarray(PTR).to_owned().as_arg(),
        N,
        &mut SYNVAL[LBCELL],
        &mut BST,
        ERROR.as_arg_mut(),
        ctx,
    )?;

    if fstr::ne(ERROR.get(1), b" ") {
        spicelib::PREFIX(b"M2CHCK:", 1, &mut ERROR[2]);
    }

    Ok(())
}
