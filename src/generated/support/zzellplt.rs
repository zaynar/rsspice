//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZELLPLT ( Tessellate an ellipsoid with triangular plates )
pub fn ZZELLPLT(
    A: f64,
    B: f64,
    C: f64,
    NLON: i32,
    NLAT: i32,
    MAXV: i32,
    MAXP: i32,
    NV: &mut i32,
    VERTS: &mut [f64],
    NP: &mut i32,
    PLATES: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut VERTS = DummyArrayMut2D::new(VERTS, 1..=3, 1..);
    let mut PLATES = DummyArrayMut2D::new(PLATES, 1..=3, 1..);
    let mut DIR = StackArray::<f64, 3>::new(1..=3);
    let mut DLAT: f64 = 0.0;
    let mut DLON: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LEVEL: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut BIX: i32 = 0;
    let mut N: i32 = 0;
    let mut NNP: i32 = 0;
    let mut NSP: i32 = 0;
    let mut PIX: i32 = 0;
    let mut VIX: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZELLPLT", ctx)?;

    //
    // The semi-axes must have positive length.
    //
    if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        spicelib::SETMSG(b"Semi-axis lengths:  A = #, B = #, C = #. ", ctx);
        spicelib::ERRDP(b"#", A, ctx);
        spicelib::ERRDP(b"#", B, ctx);
        spicelib::ERRDP(b"#", C, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDAXISLENGTH)", ctx)?;
        spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
        return Ok(());
    }

    //
    // The longitude and latitude band counts must be realizable.
    //
    if (NLAT < 2) {
        spicelib::SETMSG(
            b"The latitude band count must be at least 2 but was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", NLAT, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
        return Ok(());
    }

    if (NLON < 3) {
        spicelib::SETMSG(
            b"The longitude band count must be at least 3 but was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", NLON, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
        return Ok(());
    }

    //
    // Compute the vertex and plate counts. Check against available
    // room.
    //
    //    Vertex count: there are NLAT-2 latitude bands, excluding
    //                  the polar caps. These are bounded by NLAT-1 rows
    //                  of vertices. Each row of vertices has NLON
    //                  members. The caps add two vertices.
    //
    //    Plate count:  each latitude band, excluding the polar caps,
    //                  contains 2*NLON plates. Each cap contains NLON
    //                  plates.
    //
    //
    *NV = ((NLON * (NLAT - 1)) + 2);

    *NP = ((2 * NLON) * (NLAT - 1));

    if (*NV > MAXV) {
        spicelib::SETMSG(
            b"The requested plate model requires # vertices but the maximum vertex count is #.",
            ctx,
        );
        spicelib::ERRINT(b"#", *NV, ctx);
        spicelib::ERRINT(b"#", MAXV, ctx);
        spicelib::SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
        return Ok(());
    }

    if (*NP > MAXP) {
        spicelib::SETMSG(
            b"The requested plate model requires # plates but the maximum plate count is #.",
            ctx,
        );
        spicelib::ERRINT(b"#", *NP, ctx);
        spicelib::ERRINT(b"#", MAXP, ctx);
        spicelib::SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
        return Ok(());
    }

    //
    // Create the vertex set. The north polar vertex is
    // at index 1; the south vertex is at index NV. It will
    // be convenient to make these the last two vertices.
    //
    spicelib::VPACK(0.0, 0.0, C, VERTS.subarray_mut([1, (*NV - 1)]));
    spicelib::VPACK(0.0, 0.0, -C, VERTS.subarray_mut([1, *NV]));

    //
    // The latitude bands are equally spaced in planetocentric
    // latitude.
    //
    DLAT = (spicelib::PI(ctx) / NLAT as f64);
    DLON = (((2 as f64) * spicelib::PI(ctx)) / NLON as f64);

    VIX = 1;

    for I in 1..=(NLAT - 1) {
        LAT = ((spicelib::PI(ctx) / 2 as f64) - ((I as f64) * DLAT));

        for J in 1..=NLON {
            LON = (((J - 1) as f64) * DLON);
            //
            // Create a unit direction vector for the current
            // vertex. Scale this vector to make it lie on the
            // ellipsoid's surface; the scaled vector is the
            // current vertex.
            //
            spicelib::LATREC(1.0, LON, LAT, DIR.as_slice_mut());

            LEVEL = ((f64::powi((DIR[1] / A), 2) + f64::powi((DIR[2] / B), 2))
                + f64::powi((DIR[3] / C), 2));

            S = (1.0 / f64::sqrt(LEVEL));

            spicelib::VSCL(S, DIR.as_slice(), VERTS.subarray_mut([1, VIX]));
            //
            // Next vertex.
            //
            VIX = (VIX + 1);
        }
    }

    //
    // Create the plates for the latitude bounds other than
    // those belonging to the caps.
    //
    // The first two inputs are the vertex row and column counts.
    // Next is a logical flag indicating whether longitude wrapping
    // should be used.
    //
    if (NLAT > 2) {
        ZZGRDPLT((NLAT - 1), NLON, true, &mut N, PLATES.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
            return Ok(());
        }
    }

    //
    // Add the north cap. This is a set of plates; the vertices
    // already have been computed.
    //
    PIX = ((*NP - (2 * NLON)) + 1);
    BIX = 0;

    ZZCAPPLT(
        NLON,
        true,
        true,
        BIX,
        (*NV - 1),
        &mut NNP,
        PLATES.subarray_mut([1, PIX]),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
        return Ok(());
    }

    //
    // Add the south cap.
    //
    PIX = (PIX + NLON);
    BIX = (*NV - (NLON + 2));

    ZZCAPPLT(
        NLON,
        false,
        true,
        BIX,
        *NV,
        &mut NSP,
        PLATES.subarray_mut([1, PIX]),
        ctx,
    )?;

    spicelib::CHKOUT(b"ZZELLPLT", ctx)?;
    Ok(())
}
