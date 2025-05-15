//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    PLACE: i32,
    B: i32,
    E: i32,
    BPAD: i32,
    FILL: Vec<u8>,
    PAD: i32,
    EMARK: Vec<u8>,
    BMARK: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PLACE: i32 = 0;
        let mut B: i32 = 0;
        let mut E: i32 = 0;
        let mut BPAD: i32 = 0;
        let mut FILL = vec![b' '; 80];
        let mut PAD: i32 = 0;
        let mut EMARK = vec![b' '; 16];
        let mut BMARK = vec![b' '; 16];

        fstr::assign(&mut FILL, b" ");
        PAD = 1;
        fstr::assign(&mut BMARK, b".....<");
        fstr::assign(&mut EMARK, b">.....");

        Self {
            PLACE,
            B,
            E,
            BPAD,
            FILL,
            PAD,
            EMARK,
            BMARK,
        }
    }
}

//$Procedure      M2DIAG ( META/2 diagnostics formatting utility. )
pub fn M2DIAG(
    FILLER: &[u8],
    BEGMRK: &[u8],
    ENDMRK: &[u8],
    STRING: &[u8],
    SB: i32,
    SE: i32,
    MESSGE: &[u8],
) {

    //
    // Entry points
    //
    // M2MARK
    // M2SERR
    //
    //
    // SPICELIB functions
    //
    //
    // Local variables
    //
}

//$Procedure M2SERR ( Set the META/2 error markers )
pub fn M2SERR(FILLER: &[u8], BEGMRK: &[u8], ENDMRK: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.PAD = intrinsics::MIN0(&[80, intrinsics::LEN(FILLER)]);
    fstr::assign(&mut save.BMARK, BEGMRK);
    fstr::assign(&mut save.EMARK, ENDMRK);
    fstr::assign(&mut save.FILL, FILLER);
}

//$Procedure      M2MARK (META/2 Error Marking Utility)
pub fn M2MARK(STRING: &[u8], SB: i32, SE: i32, MESSGE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // The end of MESSGE looks like
    //
    //     . . . xxx  xxxxxx
    //                          ^
    //                          |
    //                          PLACE = LASTNB(CAUSE)+PAD
    //
    //
    // After suffixing STRING to CAUSE with one space
    // it will look like:
    //
    //
    //    . . . xx x  xxxxxx     string beginning
    //                           ^
    //                           |
    //                           PLACE + 1
    //
    // and the beginning and end  of the marked string
    // will be at PLACE + SB and PLACE+SE respectively.
    //

    save.B = spicelib::LASTNB(&save.BMARK);
    save.E = spicelib::LASTNB(&save.EMARK);
    save.BPAD = (spicelib::LASTNB(MESSGE) + 1);

    if (save.PAD < 1) {
        save.PLACE = spicelib::LASTNB(MESSGE);
    } else {
        save.PLACE = (spicelib::LASTNB(MESSGE) + save.PAD);
        spicelib::SUFFIX(STRING, save.PAD, MESSGE);
        fstr::assign(
            fstr::substr_mut(MESSGE, save.BPAD..=save.PLACE),
            fstr::substr(&save.FILL, 1..=save.PAD),
        );
    }

    if (save.E > 0) {
        spicelib::ZZINSSUB(
            &MESSGE.to_vec(),
            fstr::substr(&save.EMARK, 1..=save.E),
            ((save.PLACE + SE) + 1),
            MESSGE,
        );
    }

    if (save.B > 0) {
        spicelib::ZZINSSUB(
            &MESSGE.to_vec(),
            fstr::substr(&save.BMARK, 1..=save.B),
            (save.PLACE + SB),
            MESSGE,
        );
    }
}
