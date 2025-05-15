//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const STYLEN: i32 = 200;

//$ Disclaimer
//
//     THIS SOFTWARE AND ANY RELATED MATERIALS WERE CREATED BY THE
//     CALIFORNIA INSTITUTE OF TECHNOLOGY (CALTECH) UNDER A U.S.
//     GOVERNMENT CONTRACT WITH THE NATIONAL AERONAUTICS AND SPACE
//     ADMINISTRATION (NASA). THE SOFTWARE IS TECHNOLOGY AND SOFTWARE
//     PUBLICLY AVAILABLE UNDER U.S. EXPORT LAWS AND IS PROVIDED "AS-IS"
//     TO THE RECIPIENT WITHOUT WARRANTY OF ANY KIND, INCLUDING ANY
//     WARRANTIES OF PERFORMANCE OR MERCHANTABILITY OR FITNESS FOR A
//     PARTICULAR USE OR PURPOSE (AS SET FORTH IN UNITED STATES UCC
//     SECTIONS 2312-2313) OR FOR ANY PURPOSE WHATSOEVER, FOR THE
//     SOFTWARE AND RELATED MATERIALS, HOWEVER USED.
//
//     IN NO EVENT SHALL CALTECH, ITS JET PROPULSION LABORATORY, OR NASA
//     BE LIABLE FOR ANY DAMAGES AND/OR COSTS, INCLUDING, BUT NOT
//     LIMITED TO, INCIDENTAL OR CONSEQUENTIAL DAMAGES OF ANY KIND,
//     INCLUDING ECONOMIC DAMAGE OR INJURY TO PROPERTY AND LOST PROFITS,
//     REGARDLESS OF WHETHER CALTECH, JPL, OR NASA BE ADVISED, HAVE
//     REASON TO KNOW, OR, IN FACT, SHALL KNOW OF THE POSSIBILITY.
//
//     RECIPIENT BEARS ALL RISK RELATING TO QUALITY AND PERFORMANCE OF
//     THE SOFTWARE AND ANY RELATED MATERIALS, AND AGREES TO INDEMNIFY
//     CALTECH AND NASA FOR ALL THIRD-PARTY CLAIMS RESULTING FROM THE
//     ACTIONS OF RECIPIENT IN THE USE OF THE SOFTWARE.
//
pub fn FLGRPT(
    NITEMS: i32,
    NAMES: CharArray,
    VALUES: CharArray,
    MYIO: fn(&[u8], &mut Context) -> f2rust_std::Result<()>,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let NAMES = DummyCharArray::new(NAMES, None, 1..);
    let VALUES = DummyCharArray::new(VALUES, None, 1..);
    let mut J: i32 = 0;
    let mut L: i32 = 0;
    let mut WIDTH: i32 = 0;
    let mut FREE = StackArray::<bool, 129>::new(0..=128);
    let mut HARD = [b' '; 1 as usize];
    let mut LETTER = [b' '; 1 as usize];
    let mut STYLE = [b' '; STYLEN as usize];

    //
    // This routine takes an array of names and an array of associated
    // value strings and produces a flagged set of outputs.  This
    // routine signals no errors.
    //
    //
    // The routine MYIO is a routine that is supplied by the user
    // that can handle io of text lines without any action by the
    // routine that calls it.
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"FLGRPT", ctx)?;
    WIDTH = 0;

    for I in 1..=NITEMS {
        if (spicelib::RTRIM(&NAMES[I]) > WIDTH) {
            WIDTH = spicelib::RTRIM(&NAMES[I]);
        }
    }
    for I in 1..=NITEMS {
        {
            let m1__: i32 = 33;
            let m2__: i32 = 127;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FREE[J] = true;
                J += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = WIDTH;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FREE[intrinsics::ICHAR(fstr::substr(&NAMES[I], J..=J))] = false;
                J += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = intrinsics::LEN(&VALUES[1]);
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FREE[intrinsics::ICHAR(fstr::substr(&VALUES[I], J..=J))] = false;
                J += m3__;
            }
        }

        J = 33;
        while (!FREE[J] && (J < 127)) {
            J = (J + 1);
        }

        fstr::assign(&mut HARD, &intrinsics::CHAR(J));

        NSPMRG(&mut STYLE, ctx);

        spicelib::SUFFIX(b"HARDSPACE", 1, &mut STYLE);
        spicelib::SUFFIX(&HARD, 1, &mut STYLE);
        spicelib::SUFFIX(b"FLAG", 1, &mut STYLE);

        L = (spicelib::RTRIM(&STYLE) + 2);

        for K in 1..=WIDTH {
            fstr::assign(&mut LETTER, fstr::substr(NAMES.get(I), K..=K));

            if fstr::eq(&LETTER, b" ") {
                fstr::assign(fstr::substr_mut(&mut STYLE, L..=L), &HARD);
            } else {
                fstr::assign(fstr::substr_mut(&mut STYLE, L..=L), &LETTER);
            }

            L = (L + 1);
        }

        fstr::assign(fstr::substr_mut(&mut STYLE, L..=L), b":");
        L = (L + 1);
        fstr::assign(fstr::substr_mut(&mut STYLE, L..=L), &HARD);

        if (fstr::eq(NAMES.get(I), b" ") && fstr::eq(VALUES.get(I), b" ")) {
            fstr::assign(fstr::substr_mut(&mut STYLE, (L - 1)..=(L - 1)), &HARD);
            NICEPR_1(&HARD, fstr::substr(&STYLE, 1..=L), MYIO, ctx)?;
        } else if fstr::eq(VALUES.get(I), b" ") {
            fstr::assign(fstr::substr_mut(&mut STYLE, (L - 1)..=(L - 1)), &HARD);
            NICEPR_1(&HARD, fstr::substr(&STYLE, 1..=L), MYIO, ctx)?;
        } else {
            NICEPR_1(&VALUES[I], fstr::substr(&STYLE, 1..=L), MYIO, ctx)?;
        }
    }

    spicelib::CHKOUT(b"FLGRPT", ctx)?;
    Ok(())
}
