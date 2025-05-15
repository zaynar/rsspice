//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 98;

//$Procedure ZZTSTSCLK (Create an SCLK file)
pub fn ZZTSTSCLK(SCLKNM: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut SCLKBF = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut UNIT: i32 = 0;
    let mut R: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Test Utility Functions
    //

    //
    // Local Variables.
    //

    //
    // The first order of business is to wipe out any existing
    // files with the same name.
    //
    KILFIL(SCLKNM, ctx)?;

    //
    // Fill up the buffer SCLKBF with the text that will make up
    // our test SCLK file.
    //

    fstr::assign(SCLKBF.get_mut(1), b"TEST SPICE SCLK Kernel ");
    fstr::assign(
        SCLKBF.get_mut(2),
        b"------------------------------------------- ------------- ",
    );
    fstr::assign(SCLKBF.get_mut(3), b" ");
    fstr::assign(
        SCLKBF.get_mut(4),
        b"This file contains the data necessary for converting from ",
    );
    fstr::assign(
        SCLKBF.get_mut(5),
        b"ET to ticks for the fictional spacecraft -9999.  It is produced ",
    );
    fstr::assign(
        SCLKBF.get_mut(6),
        b"by the Test Utility routine ZZTSTSCLK. ",
    );
    fstr::assign(SCLKBF.get_mut(7), b" ");
    fstr::assign(
        SCLKBF.get_mut(8),
        b"This SCLK kernel is intended to be used with the test CK file ",
    );
    fstr::assign(
        SCLKBF.get_mut(9),
        b"produced by that same routine.  The internal file name of the ",
    );
    fstr::assign(
        SCLKBF.get_mut(10),
        b"test C-Kernel is ZZ-TEST-CK-ZZ.  It contains a single type 03 ",
    );
    fstr::assign(SCLKBF.get_mut(11), b"segment. ");
    fstr::assign(SCLKBF.get_mut(12), b" ");
    fstr::assign(
        SCLKBF.get_mut(13),
        b"This fictional clock begins a 1 JAN 1980 00:00:00 ET and continues ",
    );
    fstr::assign(
        SCLKBF.get_mut(14),
        b"for 1 billion seconds.  (i.e. until 2011 SEP 9, 01:46:40.000 ET ). ",
    );
    fstr::assign(SCLKBF.get_mut(15), b" ");
    fstr::assign(
        SCLKBF.get_mut(16),
        b"This is intended for test purposes only and can be easily rebuilt ",
    );
    fstr::assign(SCLKBF.get_mut(17), b"by calling the routine ZZTSTSCLK. ");
    fstr::assign(SCLKBF.get_mut(18), b" ");
    fstr::assign(
        SCLKBF.get_mut(19),
        b"If you have any questions about this file that these comments don\'t ",
    );
    fstr::assign(SCLKBF.get_mut(20), b"answer, contact Bill Taber at NAIF. ");
    fstr::assign(SCLKBF.get_mut(21), b" ");
    fstr::assign(SCLKBF.get_mut(22), b"(818) 354-4279 ");
    fstr::assign(SCLKBF.get_mut(23), b"btaber@spice.jpl.nasa.gov ");
    fstr::assign(SCLKBF.get_mut(24), b" ");
    fstr::assign(SCLKBF.get_mut(25), b" ");
    fstr::assign(SCLKBF.get_mut(26), b" ");
    fstr::assign(SCLKBF.get_mut(27), b"Implementation notes ");
    fstr::assign(
        SCLKBF.get_mut(28),
        b"-------------------------------------------------------- ",
    );
    fstr::assign(SCLKBF.get_mut(29), b" ");
    fstr::assign(
        SCLKBF.get_mut(30),
        b"This SCLK file is constructed so that the valid SCLK strings ",
    );
    fstr::assign(
        SCLKBF.get_mut(31),
        b"are simply the number of TDB seconds that have passed ",
    );
    fstr::assign(
        SCLKBF.get_mut(32),
        b"since the Ephemeris epoch 1 Jan 1980 00:00:00 ",
    );
    fstr::assign(SCLKBF.get_mut(33), b" ");
    fstr::assign(
        SCLKBF.get_mut(34),
        b"So that 1/ 288929292.82017  simply represents the epoch that occurs ",
    );
    fstr::assign(
        SCLKBF.get_mut(35),
        b"288929292.82017 TDB seconds past the ET epoch 1 Jan 1980. ",
    );
    fstr::assign(SCLKBF.get_mut(36), b" ");
    fstr::assign(
        SCLKBF.get_mut(37),
        b"For all time, the clock runs at the same rate as TDB. There is only ",
    );
    fstr::assign(SCLKBF.get_mut(38), b"one partition for this clock. ");
    fstr::assign(SCLKBF.get_mut(39), b" ");
    fstr::assign(
        SCLKBF.get_mut(40),
        b"You must load this file into the kernel pool before using any of the ",
    );
    fstr::assign(
        SCLKBF.get_mut(41),
        b"SPICELIB SCLK routines. The code fragment ",
    );
    fstr::assign(SCLKBF.get_mut(42), b" ");
    fstr::assign(
        SCLKBF.get_mut(43),
        b"CALL FURNSH ( < name of this file > ) ",
    );
    fstr::assign(SCLKBF.get_mut(44), b" ");
    fstr::assign(
        SCLKBF.get_mut(45),
        b"performs this task. To convert between ET and UTC, you will also need ",
    );
    fstr::assign(
        SCLKBF.get_mut(46),
        b"to load a leapseconds kernel. The additional call to FURNSH, ",
    );
    fstr::assign(SCLKBF.get_mut(47), b" ");
    fstr::assign(
        SCLKBF.get_mut(48),
        b"CALL FURNSH ( < name of your leapsecond file > ) ",
    );
    fstr::assign(SCLKBF.get_mut(49), b" ");
    fstr::assign(
        SCLKBF.get_mut(50),
        b"will accomplish this. Note that you must supply the actual names of ",
    );
    fstr::assign(
        SCLKBF.get_mut(51),
        b"the files used on your system as arguments to FURNSH. Because the file ",
    );
    fstr::assign(
        SCLKBF.get_mut(52),
        b"names are system dependent, we do not list them here. ",
    );
    fstr::assign(SCLKBF.get_mut(53), b" ");
    fstr::assign(
        SCLKBF.get_mut(54),
        b"For more information, consult your SPICELIB required reading files. ",
    );
    fstr::assign(SCLKBF.get_mut(55), b"The following areas are covered: ");
    fstr::assign(SCLKBF.get_mut(56), b" ");
    fstr::assign(
        SCLKBF.get_mut(57),
        b"SCLK system                     SCLK required reading ",
    );
    fstr::assign(
        SCLKBF.get_mut(58),
        b"Time systems and conversion     TIME required reading ",
    );
    fstr::assign(
        SCLKBF.get_mut(59),
        b"Kernel pool                     KERNEL required reading ",
    );
    fstr::assign(SCLKBF.get_mut(60), b" ");
    fstr::assign(SCLKBF.get_mut(61), b" ");
    fstr::assign(SCLKBF.get_mut(62), b"Kernel data ");
    fstr::assign(
        SCLKBF.get_mut(63),
        b"-------------------------------------------------------- ",
    );
    fstr::assign(SCLKBF.get_mut(64), b" ");
    fstr::assign(SCLKBF.get_mut(65), b" ");
    BEGDAT(&mut SCLKBF[66]);
    fstr::assign(SCLKBF.get_mut(67), b" ");
    fstr::assign(SCLKBF.get_mut(68), b" ");
    fstr::assign(
        SCLKBF.get_mut(69),
        b"SCLK_KERNEL_ID                = ( @28-OCT-1994        ) ",
    );
    fstr::assign(SCLKBF.get_mut(70), b" ");
    fstr::assign(
        SCLKBF.get_mut(71),
        b"SCLK_DATA_TYPE_9              = ( 1 ) ",
    );
    fstr::assign(SCLKBF.get_mut(72), b" ");
    fstr::assign(
        SCLKBF.get_mut(73),
        b"SCLK01_TIME_SYSTEM_9          = ( 1 ) ",
    );
    fstr::assign(
        SCLKBF.get_mut(74),
        b"SCLK01_N_FIELDS_9             = ( 2 ) ",
    );
    fstr::assign(
        SCLKBF.get_mut(75),
        b"SCLK01_MODULI_9               = ( 1000000000     10000 ) ",
    );
    fstr::assign(
        SCLKBF.get_mut(76),
        b"SCLK01_OFFSETS_9              = ( 0         0 ) ",
    );
    fstr::assign(
        SCLKBF.get_mut(77),
        b"SCLK01_OUTPUT_DELIM_9         = ( 1 ) ",
    );
    fstr::assign(SCLKBF.get_mut(78), b" ");
    fstr::assign(
        SCLKBF.get_mut(79),
        b"SCLK_PARTITION_START_9        = ( 0.0000000000000E+00 ) ",
    );
    fstr::assign(
        SCLKBF.get_mut(80),
        b"SCLK_PARTITION_END_9          = ( 1.00000000E+14      ) ",
    );
    fstr::assign(
        SCLKBF.get_mut(81),
        b"SCLK01_COEFFICIENTS_9         = ( 0.00000000E+00 ",
    );
    fstr::assign(SCLKBF.get_mut(82), b"@01-JAN-1980-00:00:00.000 ");
    fstr::assign(SCLKBF.get_mut(83), b"1  ) ");
    fstr::assign(SCLKBF.get_mut(84), b" ");
    fstr::assign(SCLKBF.get_mut(85), b" ");
    fstr::assign(SCLKBF.get_mut(86), b"DELTET/DELTA_T_A       =   32.184 ");
    fstr::assign(SCLKBF.get_mut(87), b"DELTET/K               =    1.657D-3 ");
    fstr::assign(SCLKBF.get_mut(88), b"DELTET/EB              =    1.671D-2 ");
    fstr::assign(
        SCLKBF.get_mut(89),
        b"DELTET/M               = (  6.239996D0 1.99096871D-7 ) ",
    );
    fstr::assign(SCLKBF.get_mut(90), b" ");
    fstr::assign(SCLKBF.get_mut(91), b"CK_-9999_SCLK          =   -9 ");
    fstr::assign(SCLKBF.get_mut(92), b"CK_-9999_SPK           =   -9 ");
    fstr::assign(SCLKBF.get_mut(93), b" ");
    fstr::assign(SCLKBF.get_mut(94), b"CK_-10000_SCLK         =   -9 ");
    fstr::assign(SCLKBF.get_mut(95), b"CK_-10000_SPK          =   -9 ");
    fstr::assign(SCLKBF.get_mut(96), b" ");
    fstr::assign(SCLKBF.get_mut(97), b"CK_-10001_SCLK         =   -9 ");
    fstr::assign(SCLKBF.get_mut(98), b"CK_-10001_SPK          =   -9 ");

    //
    // Create the SCLK kernel.
    //
    spicelib::TXTOPN(SCLKNM, &mut UNIT, ctx)?;

    for I in 1..=NLINES {
        R = spicelib::RTRIM(&SCLKBF[I]);
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
            writer.start()?;
            writer.write_str(fstr::substr(SCLKBF.get(I), 1..=R))?;
            writer.finish()?;
        }
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    Ok(())
}
