//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DELIM: &[u8] = b" /,=:";
const DQUOTE: &[u8] = b"\"";

//$Procedure ZZLEXMET ( Scan method string )
pub fn ZZLEXMET(
    METHOD: &[u8],
    MAXN: i32,
    N: &mut i32,
    BEGS: &mut [i32],
    ENDS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BEGS = DummyArrayMut::new(BEGS, 1..);
    let mut ENDS = DummyArrayMut::new(ENDS, 1..);
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut EQ: i32 = 0;
    let mut I: i32 = 0;
    let mut LOC: i32 = 0;
    let mut NCHAR: i32 = 0;
    let mut QPOS: i32 = 0;
    let mut R: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut TKE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Note the leading blank in DELIM below; this
    // indicates that the blank character is a delimiter.
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZLEXMET", ctx)?;

    if fstr::eq(METHOD, b" ") {
        //
        // No tokens here.
        //
        *N = 0;

        CHKOUT(b"ZZLEXMET", ctx)?;
        return Ok(());
    }

    *N = 0;
    ROOM = MAXN;
    R = RTRIM(METHOD);
    I = 1;

    while (I <= R) {
        //
        // Look ahead in the input string for the start of a
        // quoted string.
        //
        QPOS = CPOS(fstr::substr(METHOD, I..), DQUOTE, 1);
        B = I;

        if (QPOS == 0) {
            //
            // There are no quoted string tokens in the input string
            // from index I onward.
            //
            E = R;
        } else {
            E = ((I + QPOS) - 2);
        }

        if (B <= E) {
            //
            // Locate any tokens between indices B and E.
            //
            I = B;

            while (I <= E) {
                //
                // Find the next delimiter in the substring
                // from indices I : E.
                //
                LOC = CPOS(fstr::substr(METHOD, I..=E), DELIM, 1);

                if (LOC == 1) {
                    //
                    // There is a delimiter character at index I in METHOD.
                    //
                    TKE = ((I - 1) + LOC);
                } else if (LOC > 1) {
                    //
                    // There is a delimiter character at index
                    //
                    //    I - 1 + LOC
                    //
                    // in METHOD.
                    //
                    if fstr::ne(fstr::substr(METHOD, I..=((I + LOC) - 2)), b" ") {
                        //
                        // There's a token preceding the delimiter.
                        //
                        TKE = ((I - 2) + LOC);
                    } else {
                        //
                        // The delimiter is all we've got.
                        //
                        TKE = ((I - 1) + LOC);
                    }
                } else {
                    //
                    // The token, if any, ends at the end of
                    // substring we're considering.
                    //
                    TKE = E;
                }

                //
                // There is a token, which may be blank, between
                // indices I and TKE. We don't return blank tokens
                // in the output array.
                //
                if fstr::ne(fstr::substr(METHOD, I..=TKE), b" ") {
                    if (ROOM > 0) {
                        *N = (*N + 1);
                        ROOM = (ROOM - 1);
                        BEGS[*N] = ((LTRIM(fstr::substr(METHOD, I..=TKE)) + I) - 1);
                        ENDS[*N] = ((RTRIM(fstr::substr(METHOD, I..=TKE)) + I) - 1);
                    } else {
                        SETMSG(b"Need more room in output arrays. Token count = #; substring indices = #:#; substring = #.", ctx);
                        ERRINT(b"#", *N, ctx);
                        ERRINT(b"#", I, ctx);
                        ERRINT(b"#", TKE, ctx);
                        SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
                        CHKOUT(b"ZZLEXMET", ctx)?;
                        return Ok(());
                    }
                }

                I = (TKE + 1);
            }
        }

        if (E < R) {
            //
            // We expect to find at least one quoted string starting
            // at index E + 1.
            //
            I = (E + 1);

            LXQSTR(fstr::substr(METHOD, I..), DQUOTE, 1, &mut EQ, &mut NCHAR);

            if (NCHAR > 0) {
                if (ROOM > 0) {
                    *N = (*N + 1);
                    ROOM = (ROOM - 1);
                    BEGS[*N] = I;
                    ENDS[*N] = ((I - 1) + EQ);
                } else {
                    SETMSG(b"Need more room in output arrays. Token count = #; substring indices = #:#; substring = #.", ctx);
                    ERRINT(b"#", *N, ctx);
                    ERRINT(b"#", I, ctx);
                    ERRINT(b"#", TKE, ctx);
                    ERRCH(b"#", fstr::substr(METHOD, I..=TKE), ctx);
                    SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
                    CHKOUT(b"ZZLEXMET", ctx)?;
                    return Ok(());
                }
            } else {
                //
                // We have a syntax error in the input string.
                //
                SETMSG(
                    b"Invalid quoted string found starting at index #. Substring is #.",
                    ctx,
                );
                ERRINT(b"#", I, ctx);
                ERRCH(b"#", fstr::substr(METHOD, I..), ctx);
                SIGERR(b"SPICE(SYNTAXERROR)", ctx)?;
                CHKOUT(b"ZZLEXMET", ctx)?;
                return Ok(());
            }

            I = (ENDS[*N] + 1);
        }
        //
        // The index I has been moved forward by at least one position.
        //
    }

    CHKOUT(b"ZZLEXMET", ctx)?;
    Ok(())
}
