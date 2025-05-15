//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const WDSIZE: i32 = 32;
const NUFILS: i32 = 5;
const MAXTST: i32 = 100;
const BIGSIZ: i32 = 1000;

struct SaveVars {
    CVALS: ActualCharArray,
    BIGNAM: Vec<u8>,
    EXPFIL: ActualCharArray,
    EXPSRC: ActualCharArray,
    FILE: Vec<u8>,
    SOURCE: Vec<u8>,
    UFILES: ActualCharArray,
    ZZLEAP: ActualCharArray,
    ZZTST1: ActualCharArray,
    ZZTST2: ActualCharArray,
    ZZTST3: ActualCharArray,
    ZZTST4: ActualCharArray,
    ZZTST6: ActualCharArray,
    CMP: ActualCharArray,
    DTTYPE: Vec<u8>,
    EXPTYP: ActualCharArray,
    FILTYP: Vec<u8>,
    B: i32,
    COUNT: i32,
    E: i32,
    HANDLE: i32,
    HANSET: StackArray<i32, 16>,
    K: i32,
    N: i32,
    NMETA: i32,
    EXPCNT: i32,
    FOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CVALS = ActualCharArray::new(LNSIZE, 1..=1);
        let mut BIGNAM = vec![b' '; BIGSIZ as usize];
        let mut EXPFIL = ActualCharArray::new(LNSIZE, 1..=20);
        let mut EXPSRC = ActualCharArray::new(LNSIZE, 1..=20);
        let mut FILE = vec![b' '; LNSIZE as usize];
        let mut SOURCE = vec![b' '; LNSIZE as usize];
        let mut UFILES = ActualCharArray::new(LNSIZE, 1..=NUFILS);
        let mut ZZLEAP = ActualCharArray::new(LNSIZE, 1..=29);
        let mut ZZTST1 = ActualCharArray::new(LNSIZE, 1..=3);
        let mut ZZTST2 = ActualCharArray::new(LNSIZE, 1..=3);
        let mut ZZTST3 = ActualCharArray::new(LNSIZE, 1..=3);
        let mut ZZTST4 = ActualCharArray::new(LNSIZE, 1..=9);
        let mut ZZTST6 = ActualCharArray::new(LNSIZE, 1..=MAXTST);
        let mut CMP = ActualCharArray::new(WDSIZE, 1..=20);
        let mut DTTYPE = vec![b' '; WDSIZE as usize];
        let mut EXPTYP = ActualCharArray::new(WDSIZE, 1..=20);
        let mut FILTYP = vec![b' '; WDSIZE as usize];
        let mut B: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut E: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut HANSET = StackArray::<i32, 16>::new(LBCELL..=10);
        let mut K: i32 = 0;
        let mut N: i32 = 0;
        let mut NMETA: i32 = 0;
        let mut EXPCNT: i32 = 0;
        let mut FOUND: bool = false;

        Self {
            CVALS,
            BIGNAM,
            EXPFIL,
            EXPSRC,
            FILE,
            SOURCE,
            UFILES,
            ZZLEAP,
            ZZTST1,
            ZZTST2,
            ZZTST3,
            ZZTST4,
            ZZTST6,
            CMP,
            DTTYPE,
            EXPTYP,
            FILTYP,
            B,
            COUNT,
            E,
            HANDLE,
            HANSET,
            K,
            N,
            NMETA,
            EXPCNT,
            FOUND,
        }
    }
}

//$Procedure      F_KEEPER ( Test the entry points of KEEPER )
pub fn F_KEEPER(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_KEEPER", ctx)?;

    testutil::TCASE(b"KEEPER test setup", ctx)?;

    //
    // Set up the text for the various text kernels we shall
    // be loading via FURNSH.
    //
    testutil::BEGDAT(&mut save.ZZLEAP[1]);
    fstr::assign(save.ZZLEAP.get_mut(2), b"DELTET/DELTA_T_A =   32.184");
    fstr::assign(save.ZZLEAP.get_mut(3), b"DELTET/K         =    1.657D-3");
    fstr::assign(save.ZZLEAP.get_mut(4), b"DELTET/EB        =    1.671D-2");
    fstr::assign(
        save.ZZLEAP.get_mut(5),
        b"DELTET/M         = (  6.239996D0   1.99096871D-7 )",
    );
    fstr::assign(save.ZZLEAP.get_mut(6), b" ");
    fstr::assign(
        save.ZZLEAP.get_mut(7),
        b"DELTET/DELTA_AT  = ( 10,   @1972-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(8),
        b"                     11,   @1972-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(9),
        b"                     12,   @1973-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(10),
        b"                     13,   @1974-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(11),
        b"                     14,   @1975-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(12),
        b"                     15,   @1976-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(13),
        b"                     16,   @1977-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(14),
        b"                     17,   @1978-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(15),
        b"                     18,   @1979-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(16),
        b"                     19,   @1980-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(17),
        b"                     20,   @1981-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(18),
        b"                     21,   @1982-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(19),
        b"                     22,   @1983-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(20),
        b"                     23,   @1985-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(21),
        b"                     24,   @1988-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(22),
        b"                     25,   @1990-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(23),
        b"                     26,   @1991-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(24),
        b"                     27,   @1992-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(25),
        b"                     28,   @1993-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(26),
        b"                     29,   @1994-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(27),
        b"                     30,   @1996-JAN-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(28),
        b"                     31,   @1997-JUL-1",
    );
    fstr::assign(
        save.ZZLEAP.get_mut(29),
        b"                     32,   @1999-JAN-1 )",
    );

    testutil::BEGDAT(&mut save.ZZTST1[1]);
    fstr::assign(save.ZZTST1.get_mut(2), b"ZZTST1_NUMBER += 1");
    fstr::assign(save.ZZTST1.get_mut(3), b"ZZTST1_STRING = \'1\'");

    testutil::BEGDAT(&mut save.ZZTST2[1]);
    fstr::assign(save.ZZTST2.get_mut(2), b"ZZTST1_NUMBER += 2");
    fstr::assign(save.ZZTST2.get_mut(3), b"ZZTST2_STRING = \'2\'");

    testutil::BEGDAT(&mut save.ZZTST3[1]);
    fstr::assign(save.ZZTST3.get_mut(2), b"ZZTST1_NUMBER += 3");
    fstr::assign(save.ZZTST3.get_mut(3), b"ZZTST3_STRING  = \'3\'");
    //
    // The first meta-text kernel.
    //
    testutil::BEGDAT(&mut save.ZZTST4[1]);
    fstr::assign(
        save.ZZTST4.get_mut(2),
        b"KERNELS_TO_LOAD = ( \'zz3spk$SPK\',",
    );
    fstr::assign(save.ZZTST4.get_mut(3), b"                    \'zzck2$CK\',");
    fstr::assign(
        save.ZZTST4.get_mut(4),
        b"                    \'zzsclk2.ker\',",
    );
    fstr::assign(
        save.ZZTST4.get_mut(5),
        b"                    \'zztst3$TXT\' )",
    );
    fstr::assign(
        save.ZZTST4.get_mut(6),
        b"PATH_SYMBOLS = ( \'SPK\',  \'CK\',   \'TXT\' )",
    );
    fstr::assign(
        save.ZZTST4.get_mut(7),
        b"PATH_VALUES  = ( \'.bsp\', \'.bc\', \'.txt\' )",
    );

    //
    // Wipe out any existing test kernels.  (There shouldn't
    // be any, but just in case.)
    //
    testutil::KILFIL(b"zz1spk.bsp", ctx)?;
    testutil::KILFIL(b"zz2spk.bsp", ctx)?;
    testutil::KILFIL(b"zz3spk.bsp", ctx)?;
    testutil::KILFIL(b"zzck1.bc", ctx)?;
    testutil::KILFIL(b"zzck2.bc", ctx)?;
    testutil::KILFIL(b"zzsclk1.ker", ctx)?;
    testutil::KILFIL(b"zzsclk2.ker", ctx)?;
    testutil::KILFIL(b"zzleaps.ker", ctx)?;
    testutil::KILFIL(b"zztst1.txt", ctx)?;
    testutil::KILFIL(b"zztst2.txt", ctx)?;
    testutil::KILFIL(b"zztst3.txt", ctx)?;
    testutil::KILFIL(b"zztst4.txt", ctx)?;
    testutil::KILFIL(b"zztst5.txt", ctx)?;

    testutil::KILFIL(b"zztstek.be", ctx)?;

    //
    // Create all of the test kernels we shall need.
    //
    testutil::TSTSPK(b"zz1spk.bsp", false, &mut save.HANDLE, ctx)?;
    testutil::TSTSPK(b"zz2spk.bsp", false, &mut save.HANDLE, ctx)?;
    testutil::TSTSPK(b"zz3spk.bsp", false, &mut save.HANDLE, ctx)?;
    testutil::TSTCK3(
        b"zzck1.bc",
        b"zzsclk1.ker",
        false,
        false,
        true,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::TSTCK3(
        b"zzck2.bc",
        b"zzsclk2.ker",
        false,
        false,
        true,
        &mut save.HANDLE,
        ctx,
    )?;
    testutil::TSTSPK(b"zztstek.be", false, &mut save.HANDLE, ctx)?;

    testutil::TSTTXT(b"zzleaps.ker", save.ZZLEAP.as_arg(), 29, false, true, ctx)?;
    testutil::TSTTXT(b"zztst1.txt", save.ZZTST1.as_arg(), 3, false, true, ctx)?;
    testutil::TSTTXT(b"zztst2.txt", save.ZZTST2.as_arg(), 3, false, true, ctx)?;
    testutil::TSTTXT(b"zztst3.txt", save.ZZTST3.as_arg(), 3, false, true, ctx)?;
    testutil::TSTTXT(b"zztst4.txt", save.ZZTST4.as_arg(), 7, false, true, ctx)?;
    testutil::TSTTXT(b"zztst5.txt", save.ZZTST4.as_arg(), 7, false, true, ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Initialization Check.", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Error: input file name too long.", ctx)?;

    fstr::assign(&mut save.BIGNAM, b"BIG_FILE_NAME");

    for I in 14..=(BIGSIZ - 1) {
        fstr::assign(fstr::substr_mut(&mut save.BIGNAM, I..=I), b"X");
    }

    fstr::assign(fstr::substr_mut(&mut save.BIGNAM, BIGSIZ..=BIGSIZ), b"@");

    spicelib::FURNSH(&save.BIGNAM, ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILENAMETOOLONG)", OK, ctx)?;

    testutil::TCASE(b"Error: KERNELS_TO_LOAD file name too long.", ctx)?;

    testutil::BEGDAT(&mut save.ZZTST6[1]);
    fstr::assign(save.ZZTST6.get_mut(2), b"KERNELS_TO_LOAD = (");

    //
    // Put 50 characters of BIGNAM on each of 20 lines in
    // the meta-kernel.
    //
    save.B = 1;
    save.E = 50;

    for I in 1..=19 {
        fstr::assign(
            save.ZZTST6.get_mut((2 + I)),
            &fstr::concat(
                &fstr::concat(b"\'", fstr::substr(&save.BIGNAM, save.B..=save.E)),
                b"+\'",
            ),
        );

        save.B = (save.B + 50);
        save.E = (save.E + 50);
    }

    //
    // The last line gets a closing parenthesis.
    //
    fstr::assign(
        save.ZZTST6.get_mut(22),
        &fstr::concat(
            &fstr::concat(b"\'", fstr::substr(&save.BIGNAM, save.B..=save.E)),
            b"\' )",
        ),
    );

    testutil::TSTTXT(b"zztst6.txt", save.ZZTST6.as_arg(), 22, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILENAMETOOLONG)", OK, ctx)?;

    //
    // The meta-kernel is still loaded; unload it so the
    // subsequent tests are done on a clean system.
    //
    spicelib::UNLOAD(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Error: path name too long.", ctx)?;

    spicelib::CLEARC(MAXTST, save.ZZTST6.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::BEGDAT(&mut save.ZZTST6[1]);
    fstr::assign(save.ZZTST6.get_mut(2), b"PATH_VALUES = ( \'Z\' ");

    //
    // Put 50 characters of BIGNAM on each of 20 lines in
    // the meta-kernel.
    //
    save.B = 1;
    save.E = 50;

    for I in 1..=19 {
        fstr::assign(
            save.ZZTST6.get_mut((2 + I)),
            &fstr::concat(
                &fstr::concat(b"\'", fstr::substr(&save.BIGNAM, save.B..=save.E)),
                b"+\'",
            ),
        );

        save.B = (save.B + 50);
        save.E = (save.E + 50);
    }

    //
    // The last line gets a closing parenthesis.
    //
    fstr::assign(
        save.ZZTST6.get_mut(22),
        &fstr::concat(
            &fstr::concat(b"\'", fstr::substr(&save.BIGNAM, save.B..=save.E)),
            b"\' )",
        ),
    );

    //
    // Fill in the rest of the meta-kernel.
    //
    fstr::assign(
        save.ZZTST6.get_mut(23),
        b"PATH_SYMBOLS    = ( \'B\'  \'A\' )",
    );

    fstr::assign(
        save.ZZTST6.get_mut(24),
        b"KERNELS_TO_LOAD = \'$A/spud.txt\' ",
    );

    // DO I = 1, 24
    //    CALL TOSTDO ( ZZTST6(I) )
    // END DO

    testutil::TSTTXT(b"zztst6.txt", save.ZZTST6.as_arg(), 24, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(true, b"SPICE(PATHTOOLONG)", OK, ctx)?;

    //
    // The meta-kernel is still loaded; unload it so the
    // subsequent tests are done on a clean system.
    //
    spicelib::UNLOAD(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Error: combined path and file name too long.", ctx)?;

    spicelib::CLEARC(MAXTST, save.ZZTST6.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::BEGDAT(&mut save.ZZTST6[1]);
    fstr::assign(save.ZZTST6.get_mut(2), b"PATH_VALUES = (");

    //
    // Put 50 characters of BIGNAM on each of 20 lines in
    // the meta-kernel.
    //
    save.B = 1;
    save.E = 50;

    for I in 1..=4 {
        fstr::assign(
            save.ZZTST6.get_mut((2 + I)),
            &fstr::concat(
                &fstr::concat(b"\'", fstr::substr(&save.BIGNAM, save.B..=save.E)),
                b"+\'",
            ),
        );

        save.B = (save.B + 50);
        save.E = (save.E + 50);
    }

    //
    // The last line gets a closing parenthesis.
    //
    fstr::assign(
        save.ZZTST6.get_mut(7),
        &fstr::concat(
            &fstr::concat(b"\'", fstr::substr(&save.BIGNAM, save.B..=save.E)),
            b"\' )",
        ),
    );

    //
    // Fill in the rest of the meta-kernel.
    //
    fstr::assign(save.ZZTST6.get_mut(8), b"PATH_SYMBOLS    = \'A\' ");

    fstr::assign(
        save.ZZTST6.get_mut(9),
        b"KERNELS_TO_LOAD = \'$A/spud.txt\' ",
    );

    // DO I = 1, 9
    //    CALL TOSTDO ( ZZTST6(I) )
    // END DO

    testutil::TSTTXT(b"zztst6.txt", save.ZZTST6.as_arg(), 9, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(true, b"SPICE(FILENAMETOOLONG)", OK, ctx)?;

    //
    // The meta-kernel is still loaded; unload it so the
    // subsequent tests are done on a clean system.
    //
    spicelib::UNLOAD(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Load file with multiple symbol substitutions.", ctx)?;

    spicelib::CLEARC(MAXTST, save.ZZTST6.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::BEGDAT(&mut save.ZZTST6[1]);
    fstr::assign(
        save.ZZTST6.get_mut(2),
        b"PATH_SYMBOLS    = ( \'ABC\'  \'WXYZ\' )",
    );
    fstr::assign(
        save.ZZTST6.get_mut(3),
        b"PATH_VALUES     = ( \'zz\'   \'spk\'  )",
    );
    fstr::assign(
        save.ZZTST6.get_mut(4),
        b"KERNELS_TO_LOAD = \'$ABC1$WXYZ.bsp\' ",
    );

    testutil::TSTTXT(b"zztst6.txt", save.ZZTST6.as_arg(), 4, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // DO I = 1, 4
    //    CALL TOSTDO ( ZZTST6(I) )
    // END DO

    spicelib::FURNSH(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check loaded file counts.
    //
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check loaded file counts again.
    //
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Load file with continued path value.", ctx)?;

    spicelib::CLEARC(MAXTST, save.ZZTST6.as_arg_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::BEGDAT(&mut save.ZZTST6[1]);
    fstr::assign(save.ZZTST6.get_mut(2), b"PATH_SYMBOLS    = ( \'ABC\' )");
    fstr::assign(
        save.ZZTST6.get_mut(3),
        b"PATH_VALUES     = ( \'zz+\' \'2+\'  \'spk\'  )",
    );
    fstr::assign(save.ZZTST6.get_mut(4), b"KERNELS_TO_LOAD = \'$ABC.bsp\' ");

    testutil::TSTTXT(b"zztst6.txt", save.ZZTST6.as_arg(), 4, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // DO I = 1, 4
    //    CALL TOSTDO ( ZZTST6(I) )
    // END DO

    spicelib::FURNSH(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check loaded file counts.
    //
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::UNLOAD(b"zztst6.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check loaded file counts again.
    //
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Check Initial Values.", ctx)?;

    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"PCK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"PCKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"SPK CK PCK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Check that we can furnish all kernels directly.", ctx)?;

    spicelib::FURNSH(b"zz1spk.bsp", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::FURNSH(b"zz2spk.bsp", ctx)?;
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TESTCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::FURNSH(b"zzck1.bc", ctx)?;
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"APKCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DAFHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DAFCOUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;

    spicelib::FURNSH(b"zzsclk1.ker", ctx)?;
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::FURNSH(b"zztstek.be", ctx)?;
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 5, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DAFHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DASCOUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;

    //
    // Now fetch files to see if we can get the correct
    // information about them.
    //
    fstr::assign(save.EXPFIL.get_mut(1), b"zz1spk.bsp");
    fstr::assign(save.EXPFIL.get_mut(2), b"zz2spk.bsp");
    fstr::assign(save.EXPFIL.get_mut(3), b"zzck1.bc");
    fstr::assign(save.EXPFIL.get_mut(4), b"zzsclk1.ker");
    fstr::assign(save.EXPFIL.get_mut(5), b"zztstek.be");

    fstr::assign(save.EXPTYP.get_mut(1), b"SPK");
    fstr::assign(save.EXPTYP.get_mut(2), b"SPK");
    fstr::assign(save.EXPTYP.get_mut(3), b"CK");
    fstr::assign(save.EXPTYP.get_mut(4), b"TEXT");
    fstr::assign(save.EXPTYP.get_mut(5), b"SPK");

    fstr::assign(save.CMP.get_mut(1), b"!=");
    fstr::assign(save.CMP.get_mut(2), b"!=");
    fstr::assign(save.CMP.get_mut(3), b"!=");
    fstr::assign(save.CMP.get_mut(4), b"=");
    fstr::assign(save.CMP.get_mut(5), b"!=");

    for I in 1..=5 {
        spicelib::KDATA(
            I,
            b"ALL",
            &mut save.FILE,
            &mut save.FILTYP,
            &mut save.SOURCE,
            &mut save.HANDLE,
            &mut save.FOUND,
            ctx,
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[I], OK, ctx)?;
        testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[I], OK, ctx)?;
        testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", b" ", OK, ctx)?;

        testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[I], 0, 0, OK, ctx)?;
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    }

    testutil::TCASE(b"Check that Meta-Kernels load successfully.", ctx)?;

    spicelib::FURNSH(b"zztst4.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 10, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 6, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;
    spicelib::KTOTAL(b"META", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"METACOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    fstr::assign(save.EXPFIL.get_mut(1), b"zz1spk.bsp");
    fstr::assign(save.EXPFIL.get_mut(2), b"zz2spk.bsp");
    fstr::assign(save.EXPFIL.get_mut(3), b"zzck1.bc");
    fstr::assign(save.EXPFIL.get_mut(4), b"zzsclk1.ker");
    fstr::assign(save.EXPFIL.get_mut(5), b"zztstek.be");
    fstr::assign(save.EXPFIL.get_mut(6), b"zztst4.txt");
    fstr::assign(save.EXPFIL.get_mut(7), b"zz3spk.bsp");
    fstr::assign(save.EXPFIL.get_mut(8), b"zzck2.bc");
    fstr::assign(save.EXPFIL.get_mut(9), b"zzsclk2.ker");
    fstr::assign(save.EXPFIL.get_mut(10), b"zztst3.txt");

    fstr::assign(save.EXPTYP.get_mut(1), b"SPK");
    fstr::assign(save.EXPTYP.get_mut(2), b"SPK");
    fstr::assign(save.EXPTYP.get_mut(3), b"CK");
    fstr::assign(save.EXPTYP.get_mut(4), b"TEXT");
    fstr::assign(save.EXPTYP.get_mut(5), b"SPK");
    fstr::assign(save.EXPTYP.get_mut(6), b"META");
    fstr::assign(save.EXPTYP.get_mut(7), b"SPK");
    fstr::assign(save.EXPTYP.get_mut(8), b"CK");
    fstr::assign(save.EXPTYP.get_mut(9), b"TEXT");
    fstr::assign(save.EXPTYP.get_mut(10), b"TEXT");

    fstr::assign(save.CMP.get_mut(1), b"!=");
    fstr::assign(save.CMP.get_mut(2), b"!=");
    fstr::assign(save.CMP.get_mut(3), b"!=");
    fstr::assign(save.CMP.get_mut(4), b"=");
    fstr::assign(save.CMP.get_mut(5), b"!=");
    fstr::assign(save.CMP.get_mut(6), b"=");
    fstr::assign(save.CMP.get_mut(7), b"!=");
    fstr::assign(save.CMP.get_mut(8), b"!=");
    fstr::assign(save.CMP.get_mut(9), b"=");
    fstr::assign(save.CMP.get_mut(10), b"=");

    fstr::assign(save.EXPSRC.get_mut(1), b" ");
    fstr::assign(save.EXPSRC.get_mut(2), b" ");
    fstr::assign(save.EXPSRC.get_mut(3), b" ");
    fstr::assign(save.EXPSRC.get_mut(4), b" ");
    fstr::assign(save.EXPSRC.get_mut(5), b" ");
    fstr::assign(save.EXPSRC.get_mut(6), b" ");
    fstr::assign(save.EXPSRC.get_mut(7), b"zztst4.txt");
    fstr::assign(save.EXPSRC.get_mut(8), b"zztst4.txt");
    fstr::assign(save.EXPSRC.get_mut(9), b"zztst4.txt");
    fstr::assign(save.EXPSRC.get_mut(10), b"zztst4.txt");

    for I in 1..=10 {
        spicelib::KDATA(
            I,
            b"ALL",
            &mut save.FILE,
            &mut save.FILTYP,
            &mut save.SOURCE,
            &mut save.HANDLE,
            &mut save.FOUND,
            ctx,
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[I], OK, ctx)?;
        testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[I], OK, ctx)?;
        testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[I], OK, ctx)?;

        testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[I], 0, 0, OK, ctx)?;
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    }

    spicelib::KDATA(
        4,
        b"SPK CK",
        &mut save.FILE,
        &mut save.FILTYP,
        &mut save.SOURCE,
        &mut save.HANDLE,
        &mut save.FOUND,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[5], OK, ctx)?;
    testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[5], OK, ctx)?;
    testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[5], OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[5], 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::KDATA(
        2,
        b"TEXT",
        &mut save.FILE,
        &mut save.FILTYP,
        &mut save.SOURCE,
        &mut save.HANDLE,
        &mut save.FOUND,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[9], OK, ctx)?;
    testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[9], OK, ctx)?;
    testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[9], OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[9], 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::TCASE(
        b"Check that files loaded after a meta-kernel are loaded correctly.",
        ctx,
    )?;

    spicelib::FURNSH(b"zzleaps.ker", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(b"zztst1.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(b"zztst2.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.EXPFIL.get_mut(11), b"zzleaps.ker");
    fstr::assign(save.EXPFIL.get_mut(12), b"zztst1.txt");
    fstr::assign(save.EXPFIL.get_mut(13), b"zztst2.txt");

    fstr::assign(save.EXPTYP.get_mut(11), b"TEXT");
    fstr::assign(save.EXPTYP.get_mut(12), b"TEXT");
    fstr::assign(save.EXPTYP.get_mut(13), b"TEXT");

    fstr::assign(save.CMP.get_mut(11), b"=");
    fstr::assign(save.CMP.get_mut(12), b"=");
    fstr::assign(save.CMP.get_mut(13), b"=");

    fstr::assign(save.EXPSRC.get_mut(11), b" ");
    fstr::assign(save.EXPSRC.get_mut(12), b" ");
    fstr::assign(save.EXPSRC.get_mut(13), b" ");

    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 13, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 6, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 6, 0, OK, ctx)?;
    spicelib::KTOTAL(b"META", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"METACOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DAFHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DAFCOUNT", save.COUNT, b"=", 6, 0, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DASHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DASCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    for I in 1..=13 {
        spicelib::KDATA(
            I,
            b"ALL",
            &mut save.FILE,
            &mut save.FILTYP,
            &mut save.SOURCE,
            &mut save.HANDLE,
            &mut save.FOUND,
            ctx,
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[I], OK, ctx)?;
        testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[I], OK, ctx)?;
        testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[I], OK, ctx)?;

        testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[I], 0, 0, OK, ctx)?;
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    }

    spicelib::KDATA(
        4,
        b"SPK CK",
        &mut save.FILE,
        &mut save.FILTYP,
        &mut save.SOURCE,
        &mut save.HANDLE,
        &mut save.FOUND,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[5], OK, ctx)?;
    testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[5], OK, ctx)?;
    testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[5], OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[5], 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    spicelib::KDATA(
        2,
        b"TEXT",
        &mut save.FILE,
        &mut save.FILTYP,
        &mut save.SOURCE,
        &mut save.HANDLE,
        &mut save.FOUND,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[9], OK, ctx)?;
    testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[9], OK, ctx)?;
    testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[9], OK, ctx)?;
    testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[9], 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

    testutil::TCASE(b"See if data is actually present in the kernel pool. ", ctx)?;

    spicelib::DTPOOL(
        b"ZZTST1_NUMBER",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", save.N, b"=", 3, 0, OK, ctx)?;
    testutil::CHCKSC(b"DTTYPE", &save.DTTYPE, b"=", b"N", OK, ctx)?;

    spicelib::DTPOOL(
        b"ZZTST1_STRING",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSC(b"DTTYPE", &save.DTTYPE, b"=", b"C", OK, ctx)?;

    spicelib::DTPOOL(
        b"KERNELS_TO_LOAD",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DTPOOL(
        b"PATH_SYMBOLS",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DTPOOL(
        b"PATH_VALUES",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that we can fetch information about files by name. ",
        ctx,
    )?;

    for I in 1..=13 {
        spicelib::KINFO(
            &save.EXPFIL[I],
            &mut save.FILTYP,
            &mut save.SOURCE,
            &mut save.HANDLE,
            &mut save.FOUND,
            ctx,
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
        testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[I], OK, ctx)?;
        testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[I], OK, ctx)?;

        testutil::CHCKSI(b"HANDLE", save.HANDLE, &save.CMP[I], 0, 0, OK, ctx)?;
    }

    spicelib::KINFO(
        b"SPUD",
        &mut save.FILTYP,
        &mut save.SOURCE,
        &mut save.HANDLE,
        &mut save.FOUND,
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(b"See if we can successfully unload a kernel.", ctx)?;

    spicelib::UNLOAD(b"zztst2.txt", ctx)?;

    spicelib::DTPOOL(
        b"ZZTST1_NUMBER",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", save.N, b"=", 2, 0, OK, ctx)?;
    testutil::CHCKSC(b"DTTYPE", &save.DTTYPE, b"=", b"N", OK, ctx)?;

    spicelib::DTPOOL(
        b"ZZTST3_STRING",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSC(b"DTTYPE", &save.DTTYPE, b"=", b"C", OK, ctx)?;

    spicelib::DTPOOL(
        b"KERNELS_TO_LOAD",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DTPOOL(
        b"PATH_SYMBOLS",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DTPOOL(
        b"PATH_VALUES",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 12, 0, OK, ctx)?;

    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 6, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 5, 0, OK, ctx)?;
    spicelib::KTOTAL(b"META", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"METACOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    testutil::TCASE(b"See if we can successfully unload a meta-kernel.", ctx)?;

    spicelib::UNLOAD(b"zztst4.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 7, 0, OK, ctx)?;
    spicelib::KTOTAL(b"SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"SPKCOUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"CKCOUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;
    spicelib::KTOTAL(b"CK PCK SPK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"3COUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;
    spicelib::KTOTAL(b"TEXT", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"TEXTCOUNT", save.COUNT, b"=", 3, 0, OK, ctx)?;
    spicelib::KTOTAL(b"META", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"METACOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;
    spicelib::KTOTAL(b"EK", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"EKCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(
        b"ZZTST1_NUMBER",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"N", save.N, b"=", 1, 0, OK, ctx)?;
    testutil::CHCKSC(b"DTTYPE", &save.DTTYPE, b"=", b"N", OK, ctx)?;

    spicelib::DTPOOL(
        b"ZZTST3_STRING",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DAFHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DAFCOUNT", save.COUNT, b"=", 4, 0, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DASHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DASCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(
        b"KERNELS_TO_LOAD",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DTPOOL(
        b"PATH_SYMBOLS",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DTPOOL(
        b"PATH_VALUES",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    testutil::TCASE(b"Unload all of the remaining files", ctx)?;

    spicelib::UNLOAD(b"zz1spk.bsp", ctx)?;
    spicelib::UNLOAD(b"zz2spk.bsp", ctx)?;
    spicelib::UNLOAD(b"zzck1.bc", ctx)?;
    spicelib::UNLOAD(b"zztstek.be", ctx)?;
    spicelib::UNLOAD(b"zzsclk1.ker", ctx)?;
    spicelib::UNLOAD(b"zzleaps.ker", ctx)?;
    spicelib::UNLOAD(b"zztst1.txt", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKSI(b"ALLCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::DTPOOL(
        b"ZZTST1_NUMBER",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::DTPOOL(
        b"ZZTST3_STRING",
        &mut save.FOUND,
        &mut save.N,
        &mut save.DTTYPE,
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DAFHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DAFCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    spicelib::SSIZEI(10, save.HANSET.as_slice_mut(), ctx)?;
    spicelib::DASHOF(save.HANSET.as_slice_mut(), ctx)?;

    save.COUNT = spicelib::CARDI(save.HANSET.as_slice(), ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"DASCOUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    //
    // Test proper updating of source pointers when kernels are
    // deleted.
    //
    testutil::TCASE(b"Load meta-kernel, then individual files, then another meta-kernel.  Unload the individual files loaded initially.  Verify entries for files referenced by the meta-kernel.", ctx)?;

    //
    // Here's the first meta-kernel:
    //
    spicelib::FURNSH(b"zztst4.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.UFILES.get_mut(1), b"zz1spk.bsp");
    fstr::assign(save.UFILES.get_mut(2), b"zz2spk.bsp");
    fstr::assign(save.UFILES.get_mut(3), b"zzck1.bc");
    fstr::assign(save.UFILES.get_mut(4), b"zztstek.be");
    fstr::assign(save.UFILES.get_mut(5), b"zzleaps.ker");

    for I in 1..=NUFILS {
        spicelib::FURNSH(&save.UFILES[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Here's the second meta-kernel:
    //
    spicelib::FURNSH(b"zztst5.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a local database of information
    // on loaded files.
    //
    spicelib::KTOTAL(b"ALL", &mut save.EXPCNT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=save.EXPCNT {
        spicelib::KDATA(
            I,
            b"ALL",
            &mut save.EXPFIL[I],
            &mut save.EXPTYP[I],
            &mut save.EXPSRC[I],
            &mut save.HANDLE,
            &mut save.FOUND,
            ctx,
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;
    }

    //
    // Let NMETA be the number of database entries corresponding
    // to the first meta-kernel.
    //
    save.NMETA = 5;

    for I in 1..=NUFILS {
        //
        // Unload the Ith file preceding the second meta-kernel.
        //
        spicelib::UNLOAD(&save.UFILES[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the database.
        //
        spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", (save.EXPCNT - I), 0, OK, ctx)?;

        for J in 1..=save.COUNT {
            spicelib::KDATA(
                J,
                b"ALL",
                &mut save.FILE,
                &mut save.FILTYP,
                &mut save.SOURCE,
                &mut save.HANDLE,
                &mut save.FOUND,
                ctx,
            );

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

            //
            // Compute K, the index in the original database
            // of the data for the Jth file.  The first
            // meta-kernel and entries for files it references
            // are unaffected by the UNLOADs.  Later files
            // have their data shifted as the unloads occur.
            //

            if (J <= save.NMETA) {
                save.K = J;
            } else {
                save.K = (J + I);
            }

            testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[save.K], OK, ctx)?;
            testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[save.K], OK, ctx)?;
            testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[save.K], OK, ctx)?;
        }
    }

    testutil::TCASE(
        b"Continue the previous test, this time unloading the first meta-kernel.",
        ctx,
    )?;

    //
    // Unload the first meta-kernel.
    //
    spicelib::UNLOAD(b"zztst4.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the database.  At this point all that
    // should be left are entries for the second
    // meta-kernel and its references.
    //
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"COUNT",
        save.COUNT,
        b"=",
        ((save.EXPCNT - save.NMETA) - NUFILS),
        0,
        OK,
        ctx,
    )?;

    for J in 1..=save.COUNT {
        spicelib::KDATA(
            J,
            b"ALL",
            &mut save.FILE,
            &mut save.FILTYP,
            &mut save.SOURCE,
            &mut save.HANDLE,
            &mut save.FOUND,
            ctx,
        );

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", save.FOUND, true, OK, ctx)?;

        //
        // Compute K, the index in the original database
        // of the data for the Jth file.  The first
        // meta-kernel and entries for files it references
        // are unaffected by the UNLOADs.  Later files
        // have their data shifted as the unloads occur.
        //
        save.K = ((J + save.NMETA) + NUFILS);

        testutil::CHCKSC(b"FILE", &save.FILE, b"=", &save.EXPFIL[save.K], OK, ctx)?;
        testutil::CHCKSC(b"FILTYP", &save.FILTYP, b"=", &save.EXPTYP[save.K], OK, ctx)?;
        testutil::CHCKSC(b"SOURCE", &save.SOURCE, b"=", &save.EXPSRC[save.K], OK, ctx)?;
    }

    //
    // Test KCLEAR.
    //
    testutil::TCASE(b"Load meta-kernel, then individual files, then another meta-kernel.  Unload all files using KCLEAR.", ctx)?;

    //
    // Here's the first meta-kernel:
    //
    spicelib::FURNSH(b"zztst4.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(save.UFILES.get_mut(1), b"zz1spk.bsp");
    fstr::assign(save.UFILES.get_mut(2), b"zz2spk.bsp");
    fstr::assign(save.UFILES.get_mut(3), b"zzck1.bc");
    fstr::assign(save.UFILES.get_mut(4), b"zztstek.be");
    fstr::assign(save.UFILES.get_mut(5), b"zzleaps.ker");

    for I in 1..=NUFILS {
        spicelib::FURNSH(&save.UFILES[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Here's the second meta-kernel:
    //
    spicelib::FURNSH(b"zztst5.txt", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Clear out the keeper system:
    //
    spicelib::KCLEAR(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the database entry count:  the database should
    // be empty.
    //
    spicelib::KTOTAL(b"ALL", &mut save.COUNT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 0, 0, OK, ctx)?;

    //
    // Make sure the kernel pool is empty.
    //
    spicelib::GNPOOL(
        b"*",
        1,
        1,
        &mut save.N,
        save.CVALS.as_arg_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", save.FOUND, false, OK, ctx)?;

    //
    // This is the end of the KCLEAR test.
    //

    //
    // Cleanup any debris left around from our test files.
    //

    spicelib::CLPOOL(ctx)?;

    testutil::KILFIL(b"zz1spk.bsp", ctx)?;
    testutil::KILFIL(b"zz2spk.bsp", ctx)?;
    testutil::KILFIL(b"zz3spk.bsp", ctx)?;
    testutil::KILFIL(b"zzck1.bc", ctx)?;
    testutil::KILFIL(b"zzck2.bc", ctx)?;
    testutil::KILFIL(b"zzsclk1.ker", ctx)?;
    testutil::KILFIL(b"zzsclk2.ker", ctx)?;
    testutil::KILFIL(b"zzleaps.ker", ctx)?;
    testutil::KILFIL(b"zztst1.txt", ctx)?;
    testutil::KILFIL(b"zztst2.txt", ctx)?;
    testutil::KILFIL(b"zztst3.txt", ctx)?;
    testutil::KILFIL(b"zztst4.txt", ctx)?;
    testutil::KILFIL(b"zztst5.txt", ctx)?;
    testutil::KILFIL(b"zztst6.txt", ctx)?;
    testutil::KILFIL(b"zztstek.be", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
