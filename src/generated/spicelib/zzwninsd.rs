//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const LBCELL: i32 = -5;

//$Procedure ZZWNINSD ( Insert an interval into a DP window )
pub fn ZZWNINSD(
    LEFT: f64,
    RIGHT: f64,
    CONTEXT: &[u8],
    WINDOW: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut WINDOW = DummyArrayMut::new(WINDOW, LBCELL..);
    let mut SIZE: i32 = 0;
    let mut CARD: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut MSG = [b' '; LMSGLN as usize];

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Local parameters
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZWNINSD", ctx)?;
    }

    //
    // Get the size and cardinality of the window.
    //
    SIZE = SIZED(WINDOW.as_slice(), ctx)?;
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Let's try the easy cases first. No input interval? No change.
    // Signal that an error has occurred and set the error message.
    //
    if (LEFT > RIGHT) {
        fstr::assign(
            &mut MSG,
            b"Left endpoint greater-than right. Left endpoint was #1. Right endpoint was #2.",
        );

        let val = &fstr::concat(
            &fstr::concat(fstr::substr(&MSG, 1..=LASTNB(&MSG)), b" "),
            fstr::substr(CONTEXT, 1..=LASTNB(CONTEXT)),
        )
        .to_vec();
        fstr::assign(&mut MSG, &val);

        SETMSG(&MSG, ctx);
        ERRDP(b"#1", LEFT, ctx);
        ERRDP(b"#2", RIGHT, ctx);
        SIGERR(b"SPICE(BADENDPOINTS)", ctx)?;

        CHKOUT(b"ZZWNINSD", ctx)?;
        return Ok(());
    } else if ((CARD == 0) || (LEFT > WINDOW[CARD])) {
        //
        // Empty window? Input interval later than the end of the window?
        // Just insert the interval, if there's room.
        //
        if (SIZE >= (CARD + 2)) {
            SCARDD((CARD + 2), WINDOW.as_slice_mut(), ctx)?;
            WINDOW[(CARD + 1)] = LEFT;
            WINDOW[(CARD + 2)] = RIGHT;
        } else {
            fstr::assign(&mut MSG, b"Window has size, #1, cardinality #2. Cannot insert an additional interval into the window.");

            let val = &fstr::concat(
                &fstr::concat(fstr::substr(&MSG, 1..=LASTNB(&MSG)), b" "),
                fstr::substr(CONTEXT, 1..=LASTNB(CONTEXT)),
            )
            .to_vec();
            fstr::assign(&mut MSG, &val);

            SETMSG(&MSG, ctx);
            ERRINT(b"#1", SIZE, ctx);
            ERRINT(b"#2", CARD, ctx);
            SIGERR(b"SPICE(WINDOWEXCESS)", ctx)?;
        }

        CHKOUT(b"ZZWNINSD", ctx)?;
        return Ok(());
    }

    //
    // Now on to the tougher cases.
    //
    // Skip intervals which lie completely to the left of the input
    // interval. (The index I will always point to the right endpoint
    // of an interval).
    //
    I = 2;

    while ((I <= CARD) && (WINDOW[I] < LEFT)) {
        I = (I + 2);
    }

    //
    // There are three ways this can go. The new interval can:
    //
    //    1) lie entirely between the previous interval and the next.
    //
    //    2) overlap the next interval, but no others.
    //
    //    3) overlap more than one interval.
    //
    // Only the first case can possibly cause an overflow, since the
    // other two cases require existing intervals to be merged.
    //

    //
    // Case (1). If there's room, move succeeding intervals back and
    // insert the new one. If there isn't room, signal an error.
    //
    if (RIGHT < WINDOW[(I - 1)]) {
        if (SIZE >= (CARD + 2)) {
            {
                let m1__: i32 = CARD;
                let m2__: i32 = (I - 1);
                let m3__: i32 = -1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    WINDOW[(J + 2)] = WINDOW[J];
                    J += m3__;
                }
            }

            SCARDD((CARD + 2), WINDOW.as_slice_mut(), ctx)?;
            WINDOW[(I - 1)] = LEFT;
            WINDOW[I] = RIGHT;
        } else {
            fstr::assign(&mut MSG, b"Window has size, #1, cardinality #2. Cannot insert an additional interval into the window. The new interval lies entirely between the previous interval and thenext.");

            let val = &fstr::concat(
                &fstr::concat(fstr::substr(&MSG, 1..=LASTNB(&MSG)), b" "),
                fstr::substr(CONTEXT, 1..=LASTNB(CONTEXT)),
            )
            .to_vec();
            fstr::assign(&mut MSG, &val);

            SETMSG(&MSG, ctx);
            ERRINT(b"#1", SIZE, ctx);
            ERRINT(b"#2", CARD, ctx);
            SIGERR(b"SPICE(WINDOWEXCESS)", ctx)?;

            CHKOUT(b"ZZWNINSD", ctx)?;
            return Ok(());
        }

    //
    // Cases (2) and (3).
    //
    } else {
        //
        // The left and right endpoints of the new interval may or
        // may not replace the left and right endpoints of the existing
        // interval.
        //
        WINDOW[(I - 1)] = intrinsics::DMIN1(&[LEFT, WINDOW[(I - 1)]]);
        WINDOW[I] = intrinsics::DMAX1(&[RIGHT, WINDOW[I]]);

        //
        // Skip any intervals contained in the one we modified.
        // (Like I, J always points to the right endpoint of an
        // interval.)
        //
        J = (I + 2);

        while ((J <= CARD) && (WINDOW[J] <= WINDOW[I])) {
            J = (J + 2);
        }

        //
        // If the modified interval extends into the next interval,
        // merge the two. (The modified interval grows to the right.)
        //
        if ((J <= CARD) && (WINDOW[I] >= WINDOW[(J - 1)])) {
            WINDOW[I] = WINDOW[J];
            J = (J + 2);
        }

        //
        // Move the rest of the intervals forward to take up the
        // spaces left by the absorbed intervals.
        //
        while (J <= CARD) {
            I = (I + 2);
            WINDOW[(I - 1)] = WINDOW[(J - 1)];
            WINDOW[I] = WINDOW[J];
            J = (J + 2);
        }

        SCARDD(I, WINDOW.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"ZZWNINSD", ctx)?;

    Ok(())
}
