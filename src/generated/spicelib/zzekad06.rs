//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
const ENCSIZ: i32 = 5;
const CPSIZE: i32 = 1014;
const CFPIDX: i32 = (CPSIZE + 1);
const CLCIDX: i32 = (CFPIDX + ENCSIZ);
const DPSIZE: i32 = 126;
const DFPIDX: i32 = (DPSIZE + 1);
const DLCIDX: i32 = (DFPIDX + 1);
const IPSIZE: i32 = 254;
const IFPIDX: i32 = (IPSIZE + 1);
const ILCIDX: i32 = (IFPIDX + 1);
const OLD: i32 = 1;
const UPDATE: i32 = (OLD + 1);
const NEW: i32 = (UPDATE + 1);
const DELOLD: i32 = (NEW + 1);
const DELNEW: i32 = (DELOLD + 1);
const DELUPD: i32 = (DELNEW + 1);
const STAIDX: i32 = 1;
const RCPIDX: i32 = (STAIDX + 1);
const DPTBAS: i32 = 2;
const MXRPSZ: i32 = 254;
const UNINIT: i32 = -1;
const NULL: i32 = (UNINIT - 1);
const NOBACK: i32 = (NULL - 1);
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const MAXPAD: i32 = 100;

struct SaveVars {
    PADBUF: Vec<u8>,
    COLIDX: i32,
    CVLEN: i32,
    DATPTR: i32,
    ELTIDX: i32,
    LASTW: i32,
    MBASE: i32,
    MNROOM: i32,
    N: i32,
    NCHRS: i32,
    NCOLS: i32,
    NLINKS: i32,
    NP: i32,
    NPAD: i32,
    NREC: i32,
    NWRITE: i32,
    P: i32,
    P2: i32,
    PADLEN: i32,
    PBASE: i32,
    POS: i32,
    PTRLOC: i32,
    RECNO: i32,
    REMAIN: i32,
    ROOM: i32,
    STRLEN: i32,
    WP: i32,
    FIRST: bool,
    FSTPAG: bool,
    PAD: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PADBUF = vec![b' '; MAXPAD as usize];
        let mut COLIDX: i32 = 0;
        let mut CVLEN: i32 = 0;
        let mut DATPTR: i32 = 0;
        let mut ELTIDX: i32 = 0;
        let mut LASTW: i32 = 0;
        let mut MBASE: i32 = 0;
        let mut MNROOM: i32 = 0;
        let mut N: i32 = 0;
        let mut NCHRS: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NLINKS: i32 = 0;
        let mut NP: i32 = 0;
        let mut NPAD: i32 = 0;
        let mut NREC: i32 = 0;
        let mut NWRITE: i32 = 0;
        let mut P: i32 = 0;
        let mut P2: i32 = 0;
        let mut PADLEN: i32 = 0;
        let mut PBASE: i32 = 0;
        let mut POS: i32 = 0;
        let mut PTRLOC: i32 = 0;
        let mut RECNO: i32 = 0;
        let mut REMAIN: i32 = 0;
        let mut ROOM: i32 = 0;
        let mut STRLEN: i32 = 0;
        let mut WP: i32 = 0;
        let mut FIRST: bool = false;
        let mut FSTPAG: bool = false;
        let mut PAD: bool = false;

        FIRST = true;

        Self {
            PADBUF,
            COLIDX,
            CVLEN,
            DATPTR,
            ELTIDX,
            LASTW,
            MBASE,
            MNROOM,
            N,
            NCHRS,
            NCOLS,
            NLINKS,
            NP,
            NPAD,
            NREC,
            NWRITE,
            P,
            P2,
            PADLEN,
            PBASE,
            POS,
            PTRLOC,
            RECNO,
            REMAIN,
            ROOM,
            STRLEN,
            WP,
            FIRST,
            FSTPAG,
            PAD,
        }
    }
}

//$Procedure     ZZEKAD06 ( EK, add data to class 6 column )
pub fn ZZEKAD06(
    HANDLE: i32,
    SEGDSC: &mut [i32],
    COLDSC: &[i32],
    RECPTR: i32,
    NVALS: i32,
    CVALS: CharArray,
    ISNULL: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SEGDSC = DummyArrayMut::new(SEGDSC, 1..);
    let COLDSC = DummyArray::new(COLDSC, 1..);
    let CVALS = DummyCharArray::new(CVALS, None, 1..);

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
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
    // Initial values
    //

    //
    // Use discovery check-in.
    //
    if save.FIRST {
        fstr::assign(&mut save.PADBUF, b" ");
        save.FIRST = false;
    }

    //
    // Make sure the record exists.
    //
    save.NREC = SEGDSC[NRIDX];
    save.COLIDX = COLDSC[ORDIDX];

    //
    // Make sure the column exists.
    //
    save.NCOLS = SEGDSC[NCIDX];

    if ((save.COLIDX < 1) || (save.COLIDX > save.NCOLS)) {
        CHKIN(b"ZZEKAD06", ctx)?;
        SETMSG(b"Column index = #; valid range is 1:#.", ctx);
        ERRINT(b"#", save.COLIDX, ctx);
        ERRINT(b"#", save.NREC, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKAD06", ctx)?;
        return Ok(());
    }

    //
    // If the value is null, make sure that nulls are permitted
    // in this column.
    //
    if (ISNULL && (COLDSC[NFLIDX] != ITRUE)) {
        save.RECNO = ZZEKRP2N(HANDLE, SEGDSC[SNOIDX], RECPTR, ctx)?;

        CHKIN(b"ZZEKAD06", ctx)?;
        SETMSG(b"Column having index # in segment # does not allow nulls, but a null value was supplied for the element in record #.", ctx);
        ERRINT(b"#", save.COLIDX, ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", save.RECNO, ctx);
        SIGERR(b"SPICE(BADATTRIBUTE)", ctx)?;
        CHKOUT(b"ZZEKAD06", ctx)?;
        return Ok(());
    }

    //
    // Check NVALS.  If the column has fixed-size entries, NVALS must
    // match the declared entry size.  In all cases, NVALS must be
    // positive.
    //
    if (NVALS < 1) {
        CHKIN(b"ZZEKAD06", ctx)?;
        SETMSG(
            b"COLIDX = #;  segment = #; NVALS = #;  NVALS must be positive ",
            ctx,
        );
        ERRINT(b"#", save.COLIDX, ctx);
        ERRINT(b"#", SEGDSC[SNOIDX], ctx);
        ERRINT(b"#", NVALS, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKAD06", ctx)?;
        return Ok(());
    }

    if (COLDSC[SIZIDX] != IFALSE) {
        if (NVALS != COLDSC[SIZIDX]) {
            CHKIN(b"ZZEKAD06", ctx)?;
            SETMSG(
                b"COLIDX = #;  segment = #; NVALS = #; declared entry size = #.  Sizes must match.",
                ctx,
            );
            ERRINT(b"#", save.COLIDX, ctx);
            ERRINT(b"#", SEGDSC[SNOIDX], ctx);
            ERRINT(b"#", NVALS, ctx);
            ERRINT(b"#", COLDSC[SIZIDX], ctx);
            SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
            CHKOUT(b"ZZEKAD06", ctx)?;
            return Ok(());
        }
    }

    //
    // Compute the data pointer location.
    //
    save.PTRLOC = ((RECPTR + DPTBAS) + save.COLIDX);

    if ISNULL {
        //
        // All we need do is set the data pointer.  The segment's
        // metadata are not affected.
        //
        DASUDI(HANDLE, save.PTRLOC, save.PTRLOC, &[NULL], ctx)?;
    } else {
        //
        // Decide now whether we will need to pad the input entry
        // elements with trailing blanks, and if so how much padding
        // we'll need.
        //
        save.STRLEN = COLDSC[LENIDX];
        save.CVLEN = intrinsics::LEN(&CVALS[1]);
        save.PAD = (save.CVLEN < save.STRLEN);

        if save.PAD {
            save.PADLEN = (save.STRLEN - save.CVLEN);
        }

        save.LASTW = SEGDSC[LCWIDX];
        save.ROOM = (CPSIZE - save.LASTW);
        save.FSTPAG = true;

        //
        // Initialize the page base and target data pointer, if possible.
        // If the current page is full, these functions will be performed
        // below in the code section in which a new page is allocated.
        //
        if (save.LASTW < CPSIZE) {
            save.P = SEGDSC[LCPIDX];

            ZZEKPGBS(CHR, save.P, &mut save.PBASE, ctx)?;

            save.DATPTR = ((save.PBASE + save.LASTW) + 1);
        }

        save.ELTIDX = 1;

        while ((save.ELTIDX <= NVALS) && !FAILED(ctx)) {
            //
            // Write out the element having index ELTIDX.
            //
            save.POS = 0;
            save.REMAIN = save.STRLEN;

            while (save.REMAIN > 0) {
                //
                // Decide where to write the data values.  In order to write
                // a new entry, we require enough room for the count
                // and at least one character of data.
                //
                if save.FSTPAG {
                    save.MNROOM = (1 + ENCSIZ);
                } else {
                    save.MNROOM = 1;
                }

                if (save.ROOM >= save.MNROOM) {
                    //
                    // There's room in the current page.  If this is the
                    // first page this entry is written on, set the data
                    // pointer and count.  Write as much of the value as
                    // possible to the current page.
                    //
                    if save.FSTPAG {
                        DASUDI(HANDLE, save.PTRLOC, save.PTRLOC, &[save.DATPTR], ctx)?;
                        ZZEKSEI(HANDLE, save.DATPTR, NVALS, ctx)?;

                        save.ROOM = (save.ROOM - ENCSIZ);
                        save.DATPTR = (save.DATPTR + ENCSIZ);

                        //
                        // The first page containing some or all of the data
                        // item gains a link.
                        //
                        ZZEKGLNK(HANDLE, CHR, save.P, &mut save.NLINKS, ctx)?;
                        ZZEKSLNK(HANDLE, CHR, save.P, (save.NLINKS + 1), ctx)?;
                    }

                    //
                    // Write the characters we can fit onto the current page.
                    //
                    save.NWRITE = intrinsics::MIN0(&[save.REMAIN, save.ROOM]);
                    save.N = save.NWRITE;

                    while (save.N > 0) {
                        if (save.POS < save.CVLEN) {
                            //
                            // Take data from the input string CVALS(ELTIDX).
                            //
                            save.NCHRS = intrinsics::MIN0(&[save.N, (save.CVLEN - save.POS)]);

                            DASUDC(
                                HANDLE,
                                save.DATPTR,
                                ((save.DATPTR + save.NCHRS) - 1),
                                (save.POS + 1),
                                (save.POS + save.NCHRS),
                                CVALS.subarray(save.ELTIDX),
                                ctx,
                            )?;

                            save.N = (save.N - save.NCHRS);
                            save.POS = (save.POS + save.NCHRS);
                            save.DATPTR = (save.DATPTR + save.NCHRS);
                        } else if save.PAD {
                            //
                            // We must add trailing blanks to the column
                            // entry at this point.
                            //
                            save.NPAD = intrinsics::MIN0(&[save.N, save.PADLEN]);
                            save.NP = save.NPAD;

                            while (save.NP > 0) {
                                save.WP = intrinsics::MIN0(&[save.NP, MAXPAD]);

                                DASUDC(
                                    HANDLE,
                                    save.DATPTR,
                                    ((save.DATPTR + save.WP) - 1),
                                    1,
                                    save.WP,
                                    CharArray::from_ref(&save.PADBUF),
                                    ctx,
                                )?;

                                save.NP = (save.NP - save.WP);
                                save.DATPTR = (save.DATPTR + save.WP);
                            }

                            save.N = (save.N - save.NPAD);
                            save.POS = (save.POS + save.NPAD);
                        }
                    }
                    //
                    // We've written all we can to the current page.
                    //
                    save.REMAIN = (save.REMAIN - save.NWRITE);
                    save.ROOM = (save.ROOM - save.NWRITE);

                    //
                    // The last character word in use must be updated.
                    // Account for the count, if this is the first page on
                    // which the current entry is written.
                    //
                    if save.FSTPAG {
                        save.LASTW = ((save.LASTW + ENCSIZ) + save.NWRITE);
                        SEGDSC[LCWIDX] = save.LASTW;
                        save.FSTPAG = false;
                    } else {
                        save.LASTW = (save.LASTW + save.NWRITE);
                        SEGDSC[LCWIDX] = save.LASTW;
                    }
                } else {
                    //
                    // Allocate a character data page.  If this is not the
                    // first data page written to, link the previous page to
                    // the current one.
                    //
                    ZZEKAPS(
                        HANDLE,
                        SEGDSC.as_slice(),
                        CHR,
                        false,
                        &mut save.P2,
                        &mut save.PBASE,
                        ctx,
                    )?;

                    if !save.FSTPAG {
                        ZZEKSFWD(HANDLE, CHR, save.P, save.P2, ctx)?;
                    }

                    save.P = save.P2;
                    save.LASTW = 0;
                    SEGDSC[LCPIDX] = save.P;
                    SEGDSC[LCWIDX] = save.LASTW;

                    save.ROOM = CPSIZE;
                    save.DATPTR = (save.PBASE + 1);

                    //
                    // Set the link count.  If this is the first page
                    // onto which the input column entry is written,
                    // just zero out the count; the count will be set above.
                    // Additional pages get one link.
                    //
                    if save.FSTPAG {
                        save.NLINKS = 0;
                    } else {
                        save.NLINKS = 1;
                    }

                    ZZEKSLNK(HANDLE, CHR, save.P, save.NLINKS, ctx)?;
                }
            }
            //
            // We've written out the current element.
            //

            save.ELTIDX = (save.ELTIDX + 1);
        }
    }

    //
    // Write out the updated segment descriptor.
    //
    save.MBASE = SEGDSC[IMDIDX];

    DASUDI(
        HANDLE,
        (save.MBASE + 1),
        (save.MBASE + SDSCSZ),
        SEGDSC.as_slice(),
        ctx,
    )?;

    //
    // Class 6 columns are not indexed, so we need not update any
    // index to account for the new element.
    //

    Ok(())
}
