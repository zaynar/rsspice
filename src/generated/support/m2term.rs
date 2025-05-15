//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

struct SaveVars {
    MORE: bool,
    DOTHEN: bool,
    GROUP: bool,
    BEG: i32,
    NEXTT: i32,
    NEXTG: i32,
    NEXT: i32,
    MARK: i32,
    MRKEND: bool,
    B: i32,
    E: i32,
    COUNT: i32,
    ROOM: i32,
    D1: i32,
    D2: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MORE: bool = false;
        let mut DOTHEN: bool = false;
        let mut GROUP: bool = false;
        let mut BEG: i32 = 0;
        let mut NEXTT: i32 = 0;
        let mut NEXTG: i32 = 0;
        let mut NEXT: i32 = 0;
        let mut MARK: i32 = 0;
        let mut MRKEND: bool = false;
        let mut B: i32 = 0;
        let mut E: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut ROOM: i32 = 0;
        let mut D1: i32 = 0;
        let mut D2: i32 = 0;

        Self {
            MORE,
            DOTHEN,
            GROUP,
            BEG,
            NEXTT,
            NEXTG,
            NEXT,
            MARK,
            MRKEND,
            B,
            E,
            COUNT,
            ROOM,
            D1,
            D2,
        }
    }
}

//$Procedure      M2TERM (Find possible terminators of variable template)
pub fn M2TERM(
    TEMP: &[u8],
    TERMS: CharArrayMut,
    INDXES: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TERMS = DummyCharArrayMut::new(TERMS, None, LBCELL..);
    let mut INDXES = DummyArrayMut::new(INDXES, LBCELL..);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    save.ROOM = intrinsics::MIN0(&[
        spicelib::SIZEC(TERMS.as_arg(), ctx)?,
        spicelib::SIZEI(INDXES.as_slice(), ctx)?,
    ]);

    spicelib::SCARDC(0, TERMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(0, INDXES.as_slice_mut(), ctx)?;

    //
    // Just look through the string and located the appropriate keywords.
    // First see if there are any group templates.
    //
    save.BEG = 1;
    save.COUNT = 0;
    save.MORE = true;

    //
    // NEXT will point to the next group template so long as there are
    // more to find.
    //
    while save.MORE {
        save.NEXTG = UPTO(TEMP, b"){ ", save.BEG);
        save.NEXTT = UPTO(TEMP, b" @then ", save.BEG);

        if (save.NEXTG < save.NEXTT) {
            save.GROUP = true;
            save.DOTHEN = false;
            save.NEXT = save.NEXTG;
        } else if (save.NEXTT < save.NEXTG) {
            save.DOTHEN = true;
            save.GROUP = false;
            save.NEXT = save.NEXTT;
        } else {
            save.DOTHEN = false;
            save.GROUP = false;
            save.MORE = false;
        }

        if save.GROUP {
            //
            // Find the beginning of the range template and see if
            // it has the form (0:*).  If it has that form we will
            // not want to mark the end of the group when we finish
            // with it.
            //
            save.B = (spicelib::POSR(TEMP, b"(", save.NEXTG) + 1);
            save.MRKEND =
                (spicelib::NCPOS(TEMP, b"0", save.B) != spicelib::POS(TEMP, b":", save.B));

            //
            // Find the end of the next group template and set BEG
            //
            save.BEG = (spicelib::POS(TEMP, b"}", save.NEXTG) + 1);
            save.MARK = (save.BEG - 1);

            if (save.BEG == 1) {
                spicelib::CHKIN(b"M2TERM", ctx)?;
                spicelib::SETMSG(b"A switch was begun, but never ended.", ctx);
                spicelib::SIGERR(b"SPICE(META2DEFERR)", ctx)?;
                spicelib::CHKOUT(b"M2TERM", ctx)?;
                return Ok(());
            }

            //
            // Locate the first keyword of the group template.
            //
            spicelib::FNDNWD(TEMP, save.NEXTG, &mut save.B, &mut save.E);

            if (save.COUNT > save.ROOM) {
                spicelib::CHKIN(b"M2TERM", ctx)?;
                spicelib::SETMSG(b"There are too many possible terminating keywords. ", ctx);
                spicelib::SIGERR(b"SPICE(META2TOOMANYKEYS)", ctx)?;
                spicelib::CHKOUT(b"M2TERM", ctx)?;
                return Ok(());
            }

            save.COUNT = (save.COUNT + 1);
            fstr::assign(
                TERMS.get_mut(save.COUNT),
                fstr::substr(TEMP, save.B..=save.E),
            );
            INDXES[save.COUNT] = save.B;
            //
            // See if there are anymore simple templates in the this
            // group template ( they will all be preceeded by ' | '.
            //
            save.NEXTG = save.E;
            save.NEXTG = (spicelib::POS(fstr::substr(TEMP, 1..=save.BEG), b" | ", save.NEXT) + 2);

            while (save.NEXTG >= 3) {
                //
                // Locate the next keyword.
                //
                spicelib::FNDNWD(TEMP, save.NEXTG, &mut save.B, &mut save.E);

                //
                // Take care of any errors that might occur.
                //
                if (save.B == 0) {
                    spicelib::CHKIN(b"M2TERM", ctx)?;
                    spicelib::SETMSG(
                        b"An improperly composed META/2 switch was encountered.",
                        ctx,
                    );
                    spicelib::SIGERR(b"SPICE(META2DEFERR)", ctx)?;
                    spicelib::CHKOUT(b"M2TERM", ctx)?;
                    return Ok(());
                }

                if (save.COUNT >= save.ROOM) {
                    spicelib::CHKIN(b"M2TERM", ctx)?;
                    spicelib::SETMSG(b"There are too many possible terminating keywords. ", ctx);
                    spicelib::SIGERR(b"SPICE(META2TOOMANYKEYS)", ctx)?;
                    spicelib::CHKOUT(b"M2TERM", ctx)?;
                    return Ok(());
                }

                //
                // Put the keyword on the list and note its string position.
                //
                save.COUNT = (save.COUNT + 1);
                fstr::assign(
                    TERMS.get_mut(save.COUNT),
                    fstr::substr(TEMP, save.B..=save.E),
                );
                INDXES[save.COUNT] = save.B;
                save.NEXTG = save.E;
                save.NEXTG =
                    (spicelib::POS(fstr::substr(TEMP, 1..=save.BEG), b" | ", save.NEXTG) + 2);
            }

            //
            // If the group template just processed DID NOT have a range
            // template of the form (0:*%), put the marker '}' into the
            // list of keywords.
            //
            if save.MRKEND {
                save.COUNT = (save.COUNT + 1);
                fstr::assign(TERMS.get_mut(save.COUNT), b"}");
                INDXES[save.COUNT] = save.MARK;
            }

            //
            // We are out of initial keywords in the group. Get the next
            // word and see if it is a keyword or the beginning of
            // another group template.
            //
            spicelib::FNDNWD(TEMP, save.BEG, &mut save.B, &mut save.E);
        } else if save.DOTHEN {
            save.BEG = (save.NEXT + 6);
            spicelib::FNDNWD(TEMP, save.BEG, &mut save.B, &mut save.E);
        }

        if !save.MORE {

            //
            // Don't do anything, just get ready to drop through the loop.
            //
        } else if (save.B == 0) {
            //
            // We are out of template
            //
            save.MORE = false;

            spicelib::SCARDC(save.COUNT, TERMS.as_arg_mut(), ctx)?;
            spicelib::SCARDI(save.COUNT, INDXES.as_slice_mut(), ctx)?;
        } else if spicelib::MATCHW(fstr::substr(TEMP, save.B..=save.E), b"(%*:%*){", b"*", b"%") {

            //
            // Do nothing, this will all be taken care of later.
            //
        } else if fstr::eq(fstr::substr(TEMP, save.B..=save.E), b"@then") {
            //
            // Don't do anything, we'll get back to this in a moment.
            //
        } else if spicelib::MATCHW(
            fstr::substr(TEMP, save.B..=save.E),
            b"@then(%*)",
            b"*",
            b"%",
        ) {
            //
            // That's it.  I quit.
            //
            spicelib::SCARDC(save.COUNT, TERMS.as_arg_mut(), ctx)?;
            spicelib::SCARDI(save.COUNT, INDXES.as_slice_mut(), ctx)?;
            save.MORE = false;
        } else {
            //
            // Get rid of any beginning range template. (If there is a
            // range template we just dump the values into D1 and D2
            // and never use them.)
            //
            M2BEGR(TEMP, &mut save.B, save.E, &mut save.D1, &mut save.D2, ctx);

            if (save.B > save.E) {

                //
                // do nothing
                //
            } else if M2KEYW(fstr::substr(TEMP, save.B..=save.E), ctx) {
                save.COUNT = (save.COUNT + 1);
                fstr::assign(
                    TERMS.get_mut(save.COUNT),
                    fstr::substr(TEMP, save.B..=save.E),
                );
                INDXES[save.COUNT] = save.B;
                save.BEG = (save.E + 1);

                save.COUNT = (save.COUNT + 1);
                fstr::assign(TERMS.get_mut(save.COUNT), b"}");
                INDXES[save.COUNT] = save.BEG;
            }
        }

        save.GROUP = false;
        save.DOTHEN = false;
    }

    //
    // Set the cardinality and return
    //
    spicelib::SCARDC(save.COUNT, TERMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(save.COUNT, INDXES.as_slice_mut(), ctx)?;

    Ok(())
}
