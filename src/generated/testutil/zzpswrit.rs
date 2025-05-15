//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZPSWRIT ( Plate set, write to file )
pub fn ZZPSWRIT(UNIT: i32, V1: &[f64], P1: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let V1 = DummyArray::new(V1, LBCELL..);
    let P1 = DummyArray::new(P1, LBCELL..);
    let mut CP1: i32 = 0;
    let mut CV1: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut J: i32 = 0;
    let mut NP: i32 = 0;
    let mut NV: i32 = 0;

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
    spicelib::CHKIN(b"ZZPSWRIT", ctx)?;

    NV = (spicelib::CARDD(V1.as_slice(), ctx)? / 3);
    NP = (spicelib::CARDI(P1.as_slice(), ctx)? / 3);

    //
    // Make sure the cardinality of the input vertex set is a
    // multiple of 3.
    //
    CV1 = spicelib::CARDD(V1.as_slice(), ctx)?;
    NV = (CV1 / 3);

    if ((NV * 3) != CV1) {
        spicelib::SETMSG(
            b"Input vertex set cardinality # is not a multiple of 3.",
            ctx,
        );
        spicelib::ERRINT(b"#", CV1, ctx);
        spicelib::SIGERR(b"SPICE(BADVERTEXARRAY)", ctx)?;
        spicelib::CHKOUT(b"ZZPSWRIT", ctx)?;
        return Ok(());
    }

    //
    // Make sure the cardinality of the input plate set is a
    // multiple of 3.
    //
    CP1 = spicelib::CARDI(P1.as_slice(), ctx)?;
    NP = (CP1 / 3);

    if ((NP * 3) != CP1) {
        spicelib::SETMSG(
            b"Input plate set cardinality # is not a multiple of 3.",
            ctx,
        );
        spicelib::ERRINT(b"#", CP1, ctx);
        spicelib::SIGERR(b"SPICE(BADPLATEXARRAY)", ctx)?;
        spicelib::CHKOUT(b"ZZPSWRIT", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,I10)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_i32(NV)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(b"Write of NV failed; IOSTAT = #.", ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        spicelib::CHKOUT(b"ZZPSWRIT", ctx)?;
        return Ok(());
    }

    for I in 1..=NV {
        J = (3 * (I - 1));

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer =
                io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,I11,3E26.17)")?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_i32(I)?;
                for K in intrinsics::range(1, 3, 1) {
                    writer.write_f64(V1[(J + K)])?;
                }
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Write of vertex # failed; IOSTAT = #.", ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            spicelib::CHKOUT(b"ZZPSWRIT", ctx)?;
            return Ok(());
        }
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,I10)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_i32(NP)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(b"Write of NP failed; IOSTAT = #.", ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        spicelib::CHKOUT(b"ZZPSWRIT", ctx)?;
        return Ok(());
    }

    for I in 1..=NP {
        J = (3 * (I - 1));

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::FormattedWriter::new(ctx.io_unit(UNIT)?, None, b"(1X,4I10)")?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_i32(I)?;
                for K in intrinsics::range(1, 3, 1) {
                    writer.write_i32(P1[(J + K)])?;
                }
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Write of plate # failed; IOSTAT = #.", ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            spicelib::CHKOUT(b"ZZPSWRIT", ctx)?;
            return Ok(());
        }
    }

    spicelib::CHKOUT(b"ZZPSWRIT", ctx)?;
    Ok(())
}
