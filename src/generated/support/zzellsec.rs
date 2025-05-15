//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TOL: f64 = 0.000000000001;

//$Procedure ZZELLSEC ( Tessellate an ellipsoid section with plates )
pub fn ZZELLSEC(
    A: f64,
    B: f64,
    C: f64,
    MINLON: f64,
    MAXLON: f64,
    MINLAT: f64,
    MAXLAT: f64,
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
    let mut LMXLON: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut BIX: i32 = 0;
    let mut LATLB: i32 = 0;
    let mut LATUB: i32 = 0;
    let mut N: i32 = 0;
    let mut NCOLS: i32 = 0;
    let mut NNP: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NSP: i32 = 0;
    let mut PIX: i32 = 0;
    let mut POLIDX: i32 = 0;
    let mut VIX: i32 = 0;
    let mut NCAP: bool = false;
    let mut SCAP: bool = false;
    let mut WRAP: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZELLSEC", ctx)?;

    //
    // The semi-axes must have positive length.
    //
    if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        spicelib::SETMSG(b"Semi-axis lengths:  A = #, B = #, C = #. ", ctx);
        spicelib::ERRDP(b"#", A, ctx);
        spicelib::ERRDP(b"#", B, ctx);
        spicelib::ERRDP(b"#", C, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDAXISLENGTH)", ctx)?;
        spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
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
        spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
        return Ok(());
    }

    if (NLON < 3) {
        spicelib::SETMSG(
            b"The longitude band count must be at least 3 but was #.",
            ctx,
        );
        spicelib::ERRINT(b"#", NLON, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
        return Ok(());
    }

    //
    // Decide whether we have two distinct longitude boundaries. First
    // create a local maximum longitude that's greater than the minimum
    // longitude. The logical variable WRAP is .TRUE. if and only if we
    // have 2*pi - TOL radians of longitude coverage, where TOL is a
    // small value.
    //
    if (MAXLON > MINLON) {
        LMXLON = MAXLON;
    } else {
        LMXLON = (MAXLON + ((2 as f64) * spicelib::PI(ctx)));
    }

    WRAP = ((LMXLON - MINLON) > (((2 as f64) * spicelib::PI(ctx)) - TOL));

    //
    // Decide whether we have north or south polar caps.
    //
    NCAP = (MAXLAT > ((spicelib::PI(ctx) / 2 as f64) - TOL));
    SCAP = (MINLAT < (-(spicelib::PI(ctx) / 2 as f64) + TOL));

    //
    // Compute the vertex counts.
    //
    if WRAP {
        //
        // Vertex count:  When both caps are present, there are NLAT-2
        //                latitude bands, excluding the polar caps. These
        //                are bounded by NLAT-1 rows of vertices. Each
        //                row of vertices has NLON members. The caps add
        //                two vertices.
        //
        if (NCAP && SCAP) {
            //
            // There are two polar caps.
            //
            *NV = ((NLON * (NLAT - 1)) + 2);
        } else if (NCAP || SCAP) {
            //
            // There's just one polar cap. Excluding the polar
            // vertex, there are NLAT rows of vertices.
            //
            *NV = ((NLON * NLAT) + 1);
        } else {
            //
            // No polar caps. There are NLAT+1 rows of vertices.
            //
            *NV = (NLON * (NLAT + 1));
        }
    } else {
        if (NCAP && SCAP) {
            //
            // There are two polar caps.
            //
            *NV = (((NLON + 1) * (NLAT - 1)) + 2);
        } else if (NCAP || SCAP) {
            //
            // There's just one polar cap. Excluding the polar
            // vertex, there are NLAT rows of vertices.
            //
            *NV = (((NLON + 1) * NLAT) + 1);
        } else {
            //
            // No polar caps. There are NLAT+1 rows of vertices.
            //
            *NV = ((NLON + 1) * (NLAT + 1));
        }
    }

    //
    // Compute the plate counts. These depend on the set of
    // polar caps.
    //
    //
    //    Plate count:   each latitude band, excluding the polar caps,
    //                   contains 2*NLON plates. Each cap contains NLON
    //                   plates.
    //
    if (NCAP && SCAP) {
        //
        // There are two polar caps.
        //
        *NP = ((2 * NLON) * (NLAT - 1));
    } else if (NCAP || SCAP) {
        //
        // There's just one polar cap. Excluding the polar
        // vertex, there are NLAT rows of vertices.
        //
        *NP = (NLON * ((2 * NLAT) - 1));
    } else {
        //
        // No polar caps. There are NLAT+1 rows of vertices.
        //
        *NP = ((2 * NLON) * NLAT);
    }

    if (*NV > MAXV) {
        spicelib::SETMSG(
            b"The requested plate model requires # vertices but the maximum vertex count is #.",
            ctx,
        );
        spicelib::ERRINT(b"#", *NV, ctx);
        spicelib::ERRINT(b"#", MAXV, ctx);
        spicelib::SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
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
        spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
        return Ok(());
    }

    //
    // Create the vertex set, excluding the polar caps.
    //
    // LATLB will be the index of the first vertex row, excluding
    // the caps. LATUB will be the index of the last vertex row,
    // excluding the caps.
    //
    if (NCAP && SCAP) {
        LATLB = 2;
        LATUB = NLAT;
    } else if NCAP {
        LATLB = 2;
        LATUB = (NLAT + 1);
    } else if SCAP {
        LATLB = 1;
        LATUB = NLAT;
    } else {
        LATLB = 1;
        LATUB = (NLAT + 1);
    }

    //
    // NCOLS is the number of columns of vertices.
    //
    if WRAP {
        NCOLS = NLON;
    } else {
        NCOLS = (NLON + 1);
    }

    //
    // The latitude bands are equally spaced in planetocentric latitude.
    //
    DLAT = ((MAXLAT - MINLAT) / NLAT as f64);
    DLON = ((LMXLON - MINLON) / NLON as f64);

    VIX = 1;

    for I in LATLB..=LATUB {
        LAT = (MAXLAT - (((I - 1) as f64) * DLAT));

        for J in 1..=NCOLS {
            LON = (MINLON + (((J - 1) as f64) * DLON));
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
    // Create the polar vertices if necessary.
    //
    if (NCAP && SCAP) {
        spicelib::VPACK(0.0, 0.0, C, VERTS.subarray_mut([1, (*NV - 1)]));
        spicelib::VPACK(0.0, 0.0, -C, VERTS.subarray_mut([1, *NV]));
    } else if NCAP {
        spicelib::VPACK(0.0, 0.0, C, VERTS.subarray_mut([1, *NV]));
    } else if SCAP {
        spicelib::VPACK(0.0, 0.0, -C, VERTS.subarray_mut([1, *NV]));
    }

    //
    // Create the plates for the latitude bounds other than
    // those belonging to the caps.
    //
    // The first two inputs are the vertex row and column counts.
    // Next is a logical flag indicating whether longitude wrapping
    // should be used.
    //
    if (LATUB > LATLB) {
        NROWS = ((LATUB - LATLB) + 1);

        ZZGRDPLT(NROWS, NCOLS, WRAP, &mut N, PLATES.as_slice_mut(), ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
            return Ok(());
        }
    } else {
        N = 0;
    }

    if NCAP {
        //
        // Add the north cap. This is a set of plates; the vertices
        // already have been computed.
        //
        // PIX is the index of the first cap plate. BIX is the
        // base (predecessor) index of the first vertex in the
        // first vertex row.
        //
        PIX = (N + 1);
        BIX = 0;

        //
        // POLIDX is the vertex index of the north polar vertex.
        //
        if SCAP {
            POLIDX = (*NV - 1);
        } else {
            POLIDX = *NV;
        }

        ZZCAPPLT(
            NCOLS,
            true,
            WRAP,
            BIX,
            POLIDX,
            &mut NNP,
            PLATES.subarray_mut([1, PIX]),
            ctx,
        )?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
            return Ok(());
        }
    }

    if SCAP {
        //
        // Add the south cap.
        //
        POLIDX = *NV;

        if NCAP {
            PIX = (PIX + NNP);
            BIX = (*NV - (NCOLS + 2));
        } else {
            PIX = (N + 1);
            BIX = (*NV - (NCOLS + 1));
        }

        ZZCAPPLT(
            NCOLS,
            false,
            WRAP,
            BIX,
            POLIDX,
            &mut NSP,
            PLATES.subarray_mut([1, PIX]),
            ctx,
        )?;
    }

    spicelib::CHKOUT(b"ZZELLSEC", ctx)?;
    Ok(())
}
