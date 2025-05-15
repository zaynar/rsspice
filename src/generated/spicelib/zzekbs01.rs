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
const CNAMSZ: i32 = 32;
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
const FPARSZ: i32 = 1;
const SGTIDX: i32 = 1;
const MXCLSG: i32 = 100;
const EPARCH: i32 = 1;
const EPNIPT: i32 = 5;
const EPPSZC: i32 = (EPARCH + 1);
const EPBASC: i32 = (EPPSZC + 1);
const EPNPC: i32 = (EPBASC + 1);
const EPNFPC: i32 = (EPNPC + 1);
const EPFPC: i32 = (EPNFPC + 1);
const EPPSZD: i32 = (EPPSZC + EPNIPT);
const EPBASD: i32 = (EPPSZD + 1);
const EPNPD: i32 = (EPBASD + 1);
const EPNFPD: i32 = (EPNPD + 1);
const EPFPD: i32 = (EPNFPD + 1);
const EPPSZI: i32 = (EPPSZD + EPNIPT);
const EPBASI: i32 = (EPPSZI + 1);
const EPNPI: i32 = (EPBASI + 1);
const EPNFPI: i32 = (EPNPI + 1);
const EPFPI: i32 = (EPNFPI + 1);
const EPMDSZ: i32 = (1 + (3 * EPNIPT));
const PGSIZC: i32 = 1024;
const PGSIZD: i32 = 128;
const PGSIZI: i32 = 256;
const PGBASC: i32 = 0;
const PGBASD: i32 = 0;
const PGBASI: i32 = 256;
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
const TNAMSZ: i32 = 64;
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBCELL: i32 = -5;
const MXSPEC: i32 = 512;
const NAMLIM: i32 = 32;

//$Procedure      ZZEKBS01 ( EK, begin segment, type 1 )
pub fn ZZEKBS01(
    HANDLE: i32,
    TABNAM: &[u8],
    NCOLS: i32,
    CNAMES: CharArray,
    CDSCRS: &mut [i32],
    SEGNO: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CNAMES = DummyCharArray::new(CNAMES, None, 1..);
    let mut CDSCRS = DummyArrayMut2D::new(CDSCRS, 1..=CDSCSZ, 1..);
    let mut CPAGE = [b' '; PGSIZC as usize];
    let mut TMPCNM = [b' '; CNAMSZ as usize];
    let mut TMPTNM = [b' '; TNAMSZ as usize];
    let mut BASE: i32 = 0;
    let mut CBASE: i32 = 0;
    let mut CP1: i32 = 0;
    let mut CP: i32 = 0;
    let mut CPAGNO: i32 = 0;
    let mut CPT: i32 = 0;
    let mut DPT: i32 = 0;
    let mut DSCBAS: i32 = 0;
    let mut IPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut IPAGNO: i32 = 0;
    let mut IPT: i32 = 0;
    let mut METASZ: i32 = 0;
    let mut NAMBAS: i32 = 0;
    let mut NCPAGE: i32 = 0;
    let mut NIPAGE: i32 = 0;
    let mut P: i32 = 0;
    let mut P1: i32 = 0;
    let mut P1BASE: i32 = 0;
    let mut REC: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut SGTREE: i32 = 0;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKBS01", ctx)?;
    }

    //
    // Before trying to actually write anything, do every error
    // check we can.
    //
    // Is this file handle valid--is the file open for paged write
    // access?  Signal an error if not.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKBS01", ctx)?;
        return Ok(());
    }

    //
    // The metadata layout has the following form:
    //
    //    +------------------------------------------+
    //    |                                          |
    //    |            segment descriptor            |
    //    |                                          |
    //    +------------------------------------------+
    //    |            column descriptor 1           |
    //    +------------------------------------------+
    //    |            column descriptor 2           |
    //    +------------------------------------------+
    //                          .
    //                          .
    //                          .
    //    +------------------------------------------+
    //    |           column descriptor m            |
    //    +------------------------------------------+
    //
    // The column descriptors may span multiple pages, but they
    // always occupy contiguous DAS integer addresses.
    //
    // In addition, the metadata area includes a character page
    // that contains the segment's table name and the table's
    // column names.
    //
    // Calculate the number of contiguous integer pages we'll need.
    // This value is a function of the number of columns.
    //
    METASZ = (SDSCSZ + (NCOLS * CDSCSZ));

    NIPAGE = (((METASZ + PGSIZI) - 1) / PGSIZI);
    //
    // Allocate NIPAGE new integer pages.  Insisting on new pages
    // enforces contiguity.  Also allocate one character page, which
    // need not be new.
    //
    ZZEKPGAN(HANDLE, INT, &mut P1, &mut P1BASE, ctx)?;

    for I in 2..=NIPAGE {
        ZZEKPGAN(HANDLE, INT, &mut P, &mut BASE, ctx)?;
    }

    //
    // Calculate the number of contiguous character pages we'll need.
    //
    NCPAGE = ((((TNAMSZ + (NCOLS * CNAMSZ)) + PGSIZC) - 1) / PGSIZC);

    ZZEKPGAN(HANDLE, CHR, &mut CP1, &mut CBASE, ctx)?;

    for I in 2..=NCPAGE {
        ZZEKPGAN(HANDLE, CHR, &mut P, &mut BASE, ctx)?;
    }

    //
    // Initialize the record tree.
    //
    ZZEKTRIT(HANDLE, &mut REC, ctx)?;

    //
    // On the third day of Christmas, we initialized three data page
    // trees:  one for each data type.
    //
    ZZEKTRIT(HANDLE, &mut CPT, ctx)?;
    ZZEKTRIT(HANDLE, &mut DPT, ctx)?;
    ZZEKTRIT(HANDLE, &mut IPT, ctx)?;

    //
    // Prepare the contents of the first integer page:  initialize
    // everything other than the column descriptors.
    //
    // The last data word in use for each data type is initialized
    // to indicate that no room is left in the current page.  This
    // forces allocation of a new page when data must be added.  The
    // `last word' counts of each type for both the data and modified
    // record trees are initialized in this fashion.
    //
    CLEARI(PGSIZI, IPAGE.as_slice_mut());
    //
    // The value at index EKTIDX is the segment type.
    //
    IPAGE[EKTIDX] = 1;
    IPAGE[SNOIDX] = (EKNSEG(HANDLE, ctx)? + 1);
    IPAGE[IMDIDX] = P1BASE;
    IPAGE[TNMIDX] = CBASE;
    IPAGE[NCIDX] = NCOLS;
    IPAGE[NRIDX] = 0;
    IPAGE[RTIDX] = REC;
    IPAGE[CPTIDX] = CPT;
    IPAGE[DPTIDX] = DPT;
    IPAGE[IPTIDX] = IPT;
    IPAGE[MFLIDX] = ITRUE;
    IPAGE[IFLIDX] = IFALSE;
    IPAGE[SHDIDX] = IFALSE;
    IPAGE[CFHIDX] = 0;
    IPAGE[CSNIDX] = 0;
    IPAGE[LCPIDX] = 0;
    IPAGE[LDPIDX] = 0;
    IPAGE[LIPIDX] = 0;
    IPAGE[LCWIDX] = CPSIZE;
    IPAGE[LDWIDX] = DPSIZE;
    IPAGE[LIWIDX] = IPSIZE;
    IPAGE[NMLIDX] = (CBASE + TNAMSZ);

    //
    // Initialize the character metadata page:  fill in the table name.
    // The table name gets converted to upper case and is left justified.
    //
    fstr::assign(&mut CPAGE, b" ");

    LJUST(TABNAM, &mut TMPTNM);
    UCASE(&TMPTNM.clone(), &mut TMPTNM, ctx);

    fstr::assign(fstr::substr_mut(&mut CPAGE, 1..=TNAMSZ), &TMPTNM);

    //
    // Now for the column-specific tasks.  We write out a descriptor for
    // each column.  At the same time, we write out the column's name.
    //
    IPAGNO = 1;
    CPAGNO = 1;
    P = P1;
    CP = CP1;

    for I in 1..=NCOLS {
        //
        // Insert the column's ordinal position in the segment into
        // the column's descriptor.
        //
        CDSCRS[[ORDIDX, I]] = I;

        //
        // Write the Ith column name into the character metdata page.  (We
        // know the name is non-blank.)  Blank-pad the name on the right,
        // up to a length of CNAMSZ characters, if necessary.  Convert the
        // name to upper case as well.
        //
        UCASE(&CNAMES[I], &mut TMPCNM, ctx);

        NAMBAS = ((TNAMSZ + ((I - 1) * CNAMSZ)) - ((CPAGNO - 1) * PGSIZC));
        ROOM = (PGSIZC - NAMBAS);

        if (CNAMSZ <= ROOM) {
            fstr::assign(
                fstr::substr_mut(&mut CPAGE, (NAMBAS + 1)..=(NAMBAS + CNAMSZ)),
                &TMPCNM,
            );

            //
            // Fill the column name's base address into the descriptor.
            //
            CDSCRS[[NAMIDX, I]] = ((CBASE + ((CPAGNO - 1) * PGSIZC)) + NAMBAS);
        } else {
            //
            // Some or all of the column name will overflow onto the next
            // page.
            //
            if (ROOM > 0) {
                fstr::assign(
                    fstr::substr_mut(&mut CPAGE, (NAMBAS + 1)..=(NAMBAS + ROOM)),
                    fstr::substr(&TMPCNM, 1..=ROOM),
                );

                CDSCRS[[NAMIDX, I]] = ((CBASE + ((CPAGNO - 1) * PGSIZC)) + NAMBAS);
            } else {
                CDSCRS[[NAMIDX, I]] = (CBASE + (CPAGNO * PGSIZC));
            }

            //
            // Write out the page we just filled up.
            //
            ZZEKPGWC(HANDLE, CP, &CPAGE, ctx)?;

            //
            // The next character page will hold the overflow.  The next
            // page is the successor of page CP, since we allocated
            // consecutive character pages.
            //
            CP = (CP + 1);
            CPAGNO = (CPAGNO + 1);
            fstr::assign(&mut CPAGE, fstr::substr(&TMPCNM, (ROOM + 1)..));
        }

        //
        // If the column is indexed, create a new index for this column.
        // Currently, data type 1 indexes are in vogue.  Set the
        // descriptor to indicate the data type and to point to the index.
        //
        if (CDSCRS[[IXTIDX, I]] != IFALSE) {
            //
            // ZZEKCIX1 will update the descriptor to indicate the type and
            // location of the new index.
            //
            ZZEKCIX1(HANDLE, CDSCRS.subarray_mut([1, I]), ctx)?;
        }

        //
        // Add the column descriptor to the metadata page, if the
        // descriptor will fit.  We may need to allocate another page
        // to hold the descriptor.
        //
        DSCBAS = ((SDSCSZ + ((I - 1) * CDSCSZ)) - ((IPAGNO - 1) * PGSIZI));
        ROOM = (PGSIZI - DSCBAS);

        if (CDSCSZ <= ROOM) {
            //
            // The whole descriptor fits in the current page.
            //
            MOVEI(
                CDSCRS.subarray([1, I]),
                CDSCSZ,
                IPAGE.subarray_mut((DSCBAS + 1)),
            );
        } else {
            //
            // Some or all of the descriptor will overflow onto the next
            // page.
            //
            if (ROOM > 0) {
                MOVEI(
                    CDSCRS.subarray([1, I]),
                    ROOM,
                    IPAGE.subarray_mut((DSCBAS + 1)),
                );
            }

            //
            // Write out the page we just filled up.
            //
            ZZEKPGWI(HANDLE, P, IPAGE.as_slice(), ctx)?;

            //
            // The next integer page will hold the overflow.  The next page
            // is the successor of page P, since we allocated consecutive
            // pages.
            //
            P = (P + 1);
            IPAGNO = (IPAGNO + 1);

            CLEARI(PGSIZI, IPAGE.as_slice_mut());
            MOVEI(
                CDSCRS.subarray([(ROOM + 1), I]),
                (CDSCSZ - ROOM),
                IPAGE.as_slice_mut(),
            );
        }

        //
        // If we encountered a DAS error, leave now.
        //
        if FAILED(ctx) {
            CHKOUT(b"ZZEKBS01", ctx)?;
            return Ok(());
        }
    }

    //
    // Write out the last integer metadata page, and write out the
    // character metadata page.
    //
    ZZEKPGWI(HANDLE, P, IPAGE.as_slice(), ctx)?;
    ZZEKPGWC(HANDLE, CP, &CPAGE, ctx)?;

    //
    // At this point, the segment's metadata is filled in.  We must
    // update the file's segment list information to account for this
    // segment.  All we need do is add a new entry to the file's
    // segment pointer tree.  First, look up the tree.
    //
    ZZEKPGBS(INT, 1, &mut BASE, ctx)?;

    DASRDI(
        HANDLE,
        (BASE + SGTIDX),
        (BASE + SGTIDX),
        std::slice::from_mut(&mut SGTREE),
        ctx,
    )?;

    //
    // Append the head node of this segment at the end of the segment
    // tree.  The tree will point to the first integer metadata page of
    // the new segment.
    //
    ZZEKTRAP(HANDLE, SGTREE, P1, SEGNO, ctx)?;

    CHKOUT(b"ZZEKBS01", ctx)?;
    Ok(())
}
