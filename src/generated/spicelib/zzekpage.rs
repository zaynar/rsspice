//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ARCHID: i32 = 8;
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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

struct SaveVars {
    CFILL: Vec<u8>,
    ENCPAG: Vec<u8>,
    DFILL: StackArray<f64, 128>,
    DPPTR: f64,
    ADDR: i32,
    E: i32,
    FORWRD: i32,
    FREEC: i32,
    FREED: i32,
    FREEI: i32,
    IFILL: StackArray<i32, 256>,
    L: i32,
    LASTC: i32,
    LASTD: i32,
    LASTI: i32,
    NFREEC: i32,
    NFREED: i32,
    NFREEI: i32,
    NPC: i32,
    NPD: i32,
    NPI: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CFILL = vec![b' '; PGSIZC as usize];
        let mut ENCPAG = vec![b' '; 5 as usize];
        let mut DFILL = StackArray::<f64, 128>::new(1..=PGSIZD);
        let mut DPPTR: f64 = 0.0;
        let mut ADDR: i32 = 0;
        let mut E: i32 = 0;
        let mut FORWRD: i32 = 0;
        let mut FREEC: i32 = 0;
        let mut FREED: i32 = 0;
        let mut FREEI: i32 = 0;
        let mut IFILL = StackArray::<i32, 256>::new(1..=PGSIZI);
        let mut L: i32 = 0;
        let mut LASTC: i32 = 0;
        let mut LASTD: i32 = 0;
        let mut LASTI: i32 = 0;
        let mut NFREEC: i32 = 0;
        let mut NFREED: i32 = 0;
        let mut NFREEI: i32 = 0;
        let mut NPC: i32 = 0;
        let mut NPD: i32 = 0;
        let mut NPI: i32 = 0;

        Self {
            CFILL,
            ENCPAG,
            DFILL,
            DPPTR,
            ADDR,
            E,
            FORWRD,
            FREEC,
            FREED,
            FREEI,
            IFILL,
            L,
            LASTC,
            LASTD,
            LASTI,
            NFREEC,
            NFREED,
            NFREEI,
            NPC,
            NPD,
            NPI,
        }
    }
}

//$Procedure  ZZEKPAGE ( Private: Manage EK DAS paging system )
pub fn ZZEKPAGE(
    HANDLE: i32,
    TYPE: i32,
    ADDRSS: i32,
    STAT: &[u8],
    P: i32,
    PAGEC: &[u8],
    PAGED: &[f64],
    PAGEI: &[i32],
    BASE: i32,
    VALUE: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    //
    // Note:  the integer fill buffer should be as large as the maximum
    // of the integer page size and the metadata area size.
    //

    //
    // Saved variables
    //

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    Ok(())
}

//$Procedure  ZZEKPGIN ( Private: Initialize DAS for paged access )
pub fn ZZEKPGIN(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    CHKIN(b"ZZEKPGIN", ctx)?;

    //
    // The file must be open for write access.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKPGIN", ctx)?;
        return Ok(());
    }

    //
    // Find out which addresses are already in use.  A file containing
    // data cannot be initialized.
    //
    DASLLA(
        HANDLE,
        &mut save.LASTC,
        &mut save.LASTD,
        &mut save.LASTI,
        ctx,
    )?;

    if (((save.LASTC > 0) || (save.LASTD > 0)) || (save.LASTI > 0)) {
        SETMSG(
            b"File # contains data; LASTC = #; LASTD = #; LASTI = #.",
            ctx,
        );
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", save.LASTC, ctx);
        ERRINT(b"#", save.LASTD, ctx);
        ERRINT(b"#", save.LASTI, ctx);
        SIGERR(b"SPICE(DASNOTEMPTY)", ctx)?;
        CHKOUT(b"ZZEKPGIN", ctx)?;
        return Ok(());
    }

    //
    // Initialize our fill buffers.
    //
    FILLC(b" ", 1, CharArrayMut::from_mut(&mut save.CFILL));
    FILLD(0.0, PGSIZD, save.DFILL.as_slice_mut());
    FILLI(0, PGSIZI, save.IFILL.as_slice_mut());

    //
    // Initialize enough integer addresses to hold the metadata area.
    //
    DASADI(HANDLE, PGBASI, save.IFILL.as_slice(), ctx)?;

    //
    // Set the architecture code.
    //
    DASUDI(HANDLE, EPARCH, EPARCH, &[ARCHID], ctx)?;

    //
    // Set the page sizes and base addresses.
    //
    DASUDI(HANDLE, EPPSZC, EPPSZC, &[PGSIZC], ctx)?;
    DASUDI(HANDLE, EPPSZD, EPPSZD, &[PGSIZD], ctx)?;
    DASUDI(HANDLE, EPPSZI, EPPSZI, &[PGSIZI], ctx)?;

    DASUDI(HANDLE, EPBASC, EPBASC, &[PGBASC], ctx)?;
    DASUDI(HANDLE, EPBASD, EPBASD, &[PGBASD], ctx)?;
    DASUDI(HANDLE, EPBASI, EPBASI, &[PGBASI], ctx)?;

    //
    // Since the integer fill value is zero, and since zero is
    // interpreted as null pointer, all pointers are initialized.
    //
    CHKOUT(b"ZZEKPGIN", ctx)?;
    Ok(())
}

//$Procedure  ZZEKPGAN ( Private: EK, allocate new page )
pub fn ZZEKPGAN(
    HANDLE: i32,
    TYPE: i32,
    P: &mut i32,
    BASE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    CHKIN(b"ZZEKPGAN", ctx)?;

    //
    // Validate the file.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKPGAN", ctx)?;
        return Ok(());
    }

    if (TYPE == CHR) {
        //
        // The new page follows the last character address.
        //
        DASADC(
            HANDLE,
            PGSIZC,
            1,
            PGSIZC,
            CharArray::from_ref(&save.CFILL),
            ctx,
        )?;

        //
        // Update the character page count.
        //
        DASRDI(
            HANDLE,
            EPNPC,
            EPNPC,
            std::slice::from_mut(&mut save.NPC),
            ctx,
        )?;
        DASUDI(HANDLE, EPNPC, EPNPC, &[(save.NPC + 1)], ctx)?;

        //
        // Set the page number and base address.
        //
        *P = (save.NPC + 1);
        *BASE = (save.NPC * PGSIZC);
    } else if (TYPE == DP) {
        DASADD(HANDLE, PGSIZD, save.DFILL.as_slice(), ctx)?;

        DASRDI(
            HANDLE,
            EPNPD,
            EPNPD,
            std::slice::from_mut(&mut save.NPD),
            ctx,
        )?;
        DASUDI(HANDLE, EPNPD, EPNPD, &[(save.NPD + 1)], ctx)?;

        *P = (save.NPD + 1);
        *BASE = (save.NPD * PGSIZD);
    } else if (TYPE == INT) {
        DASADI(HANDLE, PGSIZI, save.IFILL.as_slice(), ctx)?;

        DASRDI(
            HANDLE,
            EPNPI,
            EPNPI,
            std::slice::from_mut(&mut save.NPI),
            ctx,
        )?;
        DASUDI(HANDLE, EPNPI, EPNPI, &[(save.NPI + 1)], ctx)?;

        *P = (save.NPI + 1);
        *BASE = ((save.NPI * PGSIZI) + PGBASI);
    } else {
        SETMSG(b"The data type code # was not recognized.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKPGAN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKPGAN", ctx)?;
    Ok(())
}

//$Procedure  ZZEKPGAL ( Private: EK, allocate page )
pub fn ZZEKPGAL(
    HANDLE: i32,
    TYPE: i32,
    P: &mut i32,
    BASE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    CHKIN(b"ZZEKPGAL", ctx)?;

    //
    // Validate the file.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKPGAL", ctx)?;
        return Ok(());
    }

    if (TYPE == CHR) {
        //
        // If the character free list is non-empty, take a page from
        // that list.
        //
        DASRDI(
            HANDLE,
            EPFPC,
            EPFPC,
            std::slice::from_mut(&mut save.FREEC),
            ctx,
        )?;

        if (save.FREEC > 0) {
            //
            // We'll return the first free page.
            //
            *P = save.FREEC;
            //
            // The new head of the list is the successor of FREEC, if
            // any.  Obtain the forward pointer from the page.
            //
            save.ADDR = (((save.FREEC - 1) * PGSIZC) + 1);

            DASRDC(
                HANDLE,
                save.ADDR,
                (save.ADDR + 4),
                1,
                5,
                CharArrayMut::from_mut(&mut save.ENCPAG),
                ctx,
            )?;
            PRTDEC(&save.ENCPAG, &mut save.FORWRD, ctx)?;

            save.FREEC = save.FORWRD;

            //
            // Decrement the free page count, and write the free pointer
            // back to the file.
            //
            DASRDI(
                HANDLE,
                EPNFPC,
                EPNFPC,
                std::slice::from_mut(&mut save.NFREEC),
                ctx,
            )?;
            DASUDI(HANDLE, EPNFPC, EPNFPC, &[(save.NFREEC - 1)], ctx)?;
            DASUDI(HANDLE, EPFPC, EPFPC, &[save.FREEC], ctx)?;

            //
            // Set base address.
            //
            *BASE = ((*P - 1) * PGSIZC);
        } else {
            //
            // The new page follows the last character address.
            //
            DASADC(
                HANDLE,
                PGSIZC,
                1,
                PGSIZC,
                CharArray::from_ref(&save.CFILL),
                ctx,
            )?;

            //
            // Update the character page count.
            //
            DASRDI(
                HANDLE,
                EPNPC,
                EPNPC,
                std::slice::from_mut(&mut save.NPC),
                ctx,
            )?;
            DASUDI(HANDLE, EPNPC, EPNPC, &[(save.NPC + 1)], ctx)?;

            //
            // Set the page number and base address.
            //
            *P = (save.NPC + 1);
            *BASE = (save.NPC * PGSIZC);
        }
    } else if (TYPE == DP) {
        //
        // If the d.p. free list is non-empty, take a page from
        // that list.
        //
        DASRDI(
            HANDLE,
            EPFPD,
            EPFPD,
            std::slice::from_mut(&mut save.FREED),
            ctx,
        )?;

        if (save.FREED > 0) {
            //
            // We'll return the first free page.
            //
            *P = save.FREED;
            //
            // The new head of the list is the successor of FREED, if
            // any.  Obtain the forward pointer from the page.
            //
            save.ADDR = (((save.FREED - 1) * PGSIZD) + 1);

            DASRDD(
                HANDLE,
                save.ADDR,
                save.ADDR,
                std::slice::from_mut(&mut save.DPPTR),
                ctx,
            )?;
            save.FREED = intrinsics::IDNINT(save.DPPTR);

            //
            // Decrement the free page count, and write the free pointer
            // back to the file.
            //
            DASRDI(
                HANDLE,
                EPNFPD,
                EPNFPD,
                std::slice::from_mut(&mut save.NFREED),
                ctx,
            )?;
            DASUDI(HANDLE, EPNFPD, EPNFPD, &[(save.NFREED - 1)], ctx)?;
            DASUDI(HANDLE, EPFPD, EPFPD, &[save.FREED], ctx)?;

            //
            // Set base address.
            //
            *BASE = ((*P - 1) * PGSIZD);
        } else {
            //
            // The new page follows the last d.p. address.
            //
            DASADD(HANDLE, PGSIZD, save.DFILL.as_slice(), ctx)?;

            //
            // Update the d.p. page count.
            //
            DASRDI(
                HANDLE,
                EPNPD,
                EPNPD,
                std::slice::from_mut(&mut save.NPD),
                ctx,
            )?;
            DASUDI(HANDLE, EPNPD, EPNPD, &[(save.NPD + 1)], ctx)?;

            //
            // Set the page number and base address.
            //
            *P = (save.NPD + 1);
            *BASE = (save.NPD * PGSIZD);
        }
    } else if (TYPE == INT) {
        //
        // If the integer free list is non-empty, take a page from
        // that list.
        //
        DASRDI(
            HANDLE,
            EPFPI,
            EPFPI,
            std::slice::from_mut(&mut save.FREEI),
            ctx,
        )?;

        if (save.FREEI > 0) {
            //
            // We'll return the first free page.
            //
            *P = save.FREEI;
            //
            // The new head of the list is the successor of FREEI, if
            // any.  Obtain the forward pointer from the page.
            //
            save.ADDR = ((((save.FREEI - 1) * PGSIZI) + PGBASI) + 1);

            DASRDI(
                HANDLE,
                save.ADDR,
                save.ADDR,
                std::slice::from_mut(&mut save.FREEI),
                ctx,
            )?;

            //
            // Decrement the free page count, and write the free pointer
            // back to the file.
            //
            DASRDI(
                HANDLE,
                EPNFPI,
                EPNFPI,
                std::slice::from_mut(&mut save.NFREEI),
                ctx,
            )?;
            DASUDI(HANDLE, EPNFPI, EPNFPI, &[(save.NFREEI - 1)], ctx)?;
            DASUDI(HANDLE, EPFPI, EPFPI, &[save.FREEI], ctx)?;

            //
            // Set base address.
            //
            *BASE = (((*P - 1) * PGSIZI) + PGBASI);
        } else {
            DASADI(HANDLE, PGSIZI, save.IFILL.as_slice(), ctx)?;

            DASRDI(
                HANDLE,
                EPNPI,
                EPNPI,
                std::slice::from_mut(&mut save.NPI),
                ctx,
            )?;
            DASUDI(HANDLE, EPNPI, EPNPI, &[(save.NPI + 1)], ctx)?;

            *P = (save.NPI + 1);
            *BASE = ((save.NPI * PGSIZI) + PGBASI);
        }
    } else {
        SETMSG(b"The data type code # was not recognized.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKPGAL", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKPGAL", ctx)?;
    Ok(())
}

//$Procedure  ZZEKPGFR ( Private: EK, free page )
pub fn ZZEKPGFR(HANDLE: i32, TYPE: i32, P: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    CHKIN(b"ZZEKPGFR", ctx)?;

    //
    // Check the file.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZEKPGFR", ctx)?;
        return Ok(());
    }

    if (TYPE == CHR) {
        //
        // Validate the page number.  Find out how many pages are
        // out there.
        //
        DASRDI(
            HANDLE,
            EPNPC,
            EPNPC,
            std::slice::from_mut(&mut save.NPC),
            ctx,
        )?;

        if ((P < 1) || (P > save.NPC)) {
            SETMSG(
                b"Attempt to free non-existent CHR page. Page number = #; valid range is 1:#",
                ctx,
            );
            ERRINT(b"#", P, ctx);
            ERRINT(b"#", save.NPC, ctx);
            SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
            CHKOUT(b"ZZEKPGFR", ctx)?;
            return Ok(());
        }

        //
        // Get the current character free pointer and free page count.
        //
        DASRDI(
            HANDLE,
            EPFPC,
            EPFPC,
            std::slice::from_mut(&mut save.FREEC),
            ctx,
        )?;
        DASRDI(
            HANDLE,
            EPNFPC,
            EPNFPC,
            std::slice::from_mut(&mut save.NFREEC),
            ctx,
        )?;

        //
        // Insert into the freed page a pointer to the head of the
        // free list.
        //
        PRTENC(save.FREEC, &mut save.ENCPAG, ctx)?;
        save.ADDR = (((P - 1) * PGSIZC) + 1);

        DASUDC(
            HANDLE,
            save.ADDR,
            (save.ADDR + 4),
            1,
            5,
            CharArray::from_ref(&save.ENCPAG),
            ctx,
        )?;

        //
        // Update the current character free pointer and free page count.
        //
        DASUDI(HANDLE, EPFPC, EPFPC, &[P], ctx)?;
        DASUDI(HANDLE, EPNFPC, EPNFPC, &[(save.NFREEC + 1)], ctx)?;
    } else if (TYPE == DP) {
        //
        // Validate the page number.  Find out how many pages are
        // out there.
        //
        DASRDI(
            HANDLE,
            EPNPD,
            EPNPD,
            std::slice::from_mut(&mut save.NPD),
            ctx,
        )?;

        if ((P < 1) || (P > save.NPD)) {
            SETMSG(
                b"Attempt to free non-existent DP page. Page number = #; valid range is 1:#",
                ctx,
            );
            ERRINT(b"#", P, ctx);
            ERRINT(b"#", save.NPD, ctx);
            SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
            CHKOUT(b"ZZEKPGFR", ctx)?;
            return Ok(());
        }

        //
        // Get the current d.p. free pointer and free page count.
        //
        DASRDI(
            HANDLE,
            EPFPD,
            EPFPD,
            std::slice::from_mut(&mut save.FREED),
            ctx,
        )?;
        DASRDI(
            HANDLE,
            EPNFPD,
            EPNFPD,
            std::slice::from_mut(&mut save.NFREED),
            ctx,
        )?;

        //
        // Insert into the freed page a pointer to the head of the
        // free list.
        //
        save.ADDR = (((P - 1) * PGSIZD) + 1);

        DASUDD(HANDLE, save.ADDR, save.ADDR, &[(save.FREED as f64)], ctx)?;

        //
        // Update the current d.p. free pointer and free page count.
        //
        DASUDI(HANDLE, EPFPD, EPFPD, &[P], ctx)?;
        DASUDI(HANDLE, EPNFPD, EPNFPD, &[(save.NFREED + 1)], ctx)?;
    } else if (TYPE == INT) {
        //
        // Validate the page number.  Find out how many pages are
        // out there.
        //
        DASRDI(
            HANDLE,
            EPNPI,
            EPNPI,
            std::slice::from_mut(&mut save.NPI),
            ctx,
        )?;

        if ((P < 1) || (P > save.NPI)) {
            SETMSG(
                b"Attempt to free non-existent INT page. Page number = #; valid range is 1:#",
                ctx,
            );
            ERRINT(b"#", P, ctx);
            ERRINT(b"#", save.NPI, ctx);
            SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
            CHKOUT(b"ZZEKPGFR", ctx)?;
            return Ok(());
        }

        //
        // Get the current integer free pointer and free page count.
        //
        DASRDI(
            HANDLE,
            EPFPI,
            EPFPI,
            std::slice::from_mut(&mut save.FREEI),
            ctx,
        )?;
        DASRDI(
            HANDLE,
            EPNFPI,
            EPNFPI,
            std::slice::from_mut(&mut save.NFREEI),
            ctx,
        )?;

        //
        // Insert into the freed page a pointer to the head of the
        // free list.
        //
        save.ADDR = ((((P - 1) * PGSIZI) + PGBASI) + 1);

        DASUDI(HANDLE, save.ADDR, save.ADDR, &[save.FREEI], ctx)?;

        //
        // Update the current integer free pointer and free page count.
        //
        DASUDI(HANDLE, EPFPI, EPFPI, &[P], ctx)?;
        DASUDI(HANDLE, EPNFPI, EPNFPI, &[(save.NFREEI + 1)], ctx)?;
    } else {
        SETMSG(b"The data type code # was not recognized.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKPGFR", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKPGFR", ctx)?;
    Ok(())
}

//$Procedure  ZZEKPGRC ( Private: EK, read character page )
pub fn ZZEKPGRC(
    HANDLE: i32,
    P: i32,
    PAGEC: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Use discovery check-in.
    //
    //
    // Find out how many character pages are in use.
    //
    DASRDI(
        HANDLE,
        EPNPC,
        EPNPC,
        std::slice::from_mut(&mut save.NPC),
        ctx,
    )?;

    if ((P < 1) || (P > save.NPC)) {
        CHKIN(b"ZZEKPGRC", ctx)?;
        SETMSG(b"CHR page = #; valid range is [1:#]", ctx);
        ERRINT(b"#", P, ctx);
        ERRINT(b"#", save.NPC, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKPGRC", ctx)?;
        return Ok(());
    }

    save.L = intrinsics::LEN(PAGEC);
    save.E = intrinsics::MIN0(&[save.L, PGSIZC]);

    save.ADDR = (((P - 1) * PGSIZC) + 1);

    DASRDC(
        HANDLE,
        save.ADDR,
        ((save.ADDR + PGSIZC) - 1),
        1,
        save.E,
        CharArrayMut::from_mut(PAGEC),
        ctx,
    )?;

    if (save.L > save.E) {
        fstr::assign(fstr::substr_mut(PAGEC, (save.E + 1)..), b" ");
    }

    Ok(())
}

//$Procedure  ZZEKPGRD ( Private: EK, read d.p. page )
pub fn ZZEKPGRD(
    HANDLE: i32,
    P: i32,
    PAGED: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PAGED = DummyArrayMut::new(PAGED, 1..);

    //
    // Use discovery check-in.
    //
    //
    // Find out how many d.p. pages are in use.
    //
    DASRDI(
        HANDLE,
        EPNPD,
        EPNPD,
        std::slice::from_mut(&mut save.NPD),
        ctx,
    )?;

    if ((P < 1) || (P > save.NPD)) {
        CHKIN(b"ZZEKPGRD", ctx)?;
        SETMSG(b"DP page = #; valid range is [1:#]", ctx);
        ERRINT(b"#", P, ctx);
        ERRINT(b"#", save.NPD, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKPGRD", ctx)?;
        return Ok(());
    }

    save.ADDR = (((P - 1) * PGSIZD) + 1);

    DASRDD(
        HANDLE,
        save.ADDR,
        ((save.ADDR + PGSIZD) - 1),
        PAGED.as_slice_mut(),
        ctx,
    )?;

    Ok(())
}

//$Procedure  ZZEKPGRI ( Private: EK, read integer page )
pub fn ZZEKPGRI(
    HANDLE: i32,
    P: i32,
    PAGEI: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut PAGEI = DummyArrayMut::new(PAGEI, 1..);

    //
    // Use discovery check-in.
    //
    //
    // Find out how many integer pages are in use.
    //
    DASRDI(
        HANDLE,
        EPNPI,
        EPNPI,
        std::slice::from_mut(&mut save.NPI),
        ctx,
    )?;

    if ((P < 1) || (P > save.NPI)) {
        CHKIN(b"ZZEKPGRI", ctx)?;
        SETMSG(b"INT page = #; valid range is [1:#]", ctx);
        ERRINT(b"#", P, ctx);
        ERRINT(b"#", save.NPI, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKPGRI", ctx)?;
        return Ok(());
    }

    save.ADDR = ((PGBASI + ((P - 1) * PGSIZI)) + 1);

    DASRDI(
        HANDLE,
        save.ADDR,
        ((save.ADDR + PGSIZI) - 1),
        PAGEI.as_slice_mut(),
        ctx,
    )?;

    Ok(())
}

//$Procedure  ZZEKPGWC ( Private: EK, write character page )
pub fn ZZEKPGWC(HANDLE: i32, P: i32, PAGEC: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Use discovery check-in.
    //
    // Validate the file.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Find out how many character pages are in use.
    //
    DASRDI(
        HANDLE,
        EPNPC,
        EPNPC,
        std::slice::from_mut(&mut save.NPC),
        ctx,
    )?;

    if ((P < 1) || (P > save.NPC)) {
        CHKIN(b"ZZEKPGWC", ctx)?;
        SETMSG(b"CHR page = #; valid range is [1:#]", ctx);
        ERRINT(b"#", P, ctx);
        ERRINT(b"#", save.NPC, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKPGWC", ctx)?;
        return Ok(());
    }

    save.L = intrinsics::LEN(PAGEC);

    if (save.L < PGSIZC) {
        CHKIN(b"ZZEKPGWC", ctx)?;
        SETMSG(b"Input CHR page size = #; valid size is [#:]", ctx);
        ERRINT(b"#", save.L, ctx);
        ERRINT(b"#", PGSIZC, ctx);
        SIGERR(b"SPICE(STRINGTOOSHORT)", ctx)?;
        CHKOUT(b"ZZEKPGWC", ctx)?;
        return Ok(());
    }

    save.ADDR = (((P - 1) * PGSIZC) + 1);

    DASUDC(
        HANDLE,
        save.ADDR,
        ((save.ADDR + PGSIZC) - 1),
        1,
        PGSIZC,
        CharArray::from_ref(PAGEC),
        ctx,
    )?;

    Ok(())
}

//$Procedure  ZZEKPGWD ( Private: EK, write d.p. page )
pub fn ZZEKPGWD(HANDLE: i32, P: i32, PAGED: &[f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let PAGED = DummyArray::new(PAGED, 1..);

    //
    // Use discovery check-in.
    //
    //
    // Validate the file.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Find out how many d.p. pages are in use.
    //
    DASRDI(
        HANDLE,
        EPNPD,
        EPNPD,
        std::slice::from_mut(&mut save.NPD),
        ctx,
    )?;

    if ((P < 1) || (P > save.NPD)) {
        CHKIN(b"ZZEKPGWD", ctx)?;
        SETMSG(b"DP page = #; valid range is [1:#]", ctx);
        ERRINT(b"#", P, ctx);
        ERRINT(b"#", save.NPD, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKPGWD", ctx)?;
        return Ok(());
    }

    save.ADDR = (((P - 1) * PGSIZD) + 1);

    DASUDD(
        HANDLE,
        save.ADDR,
        ((save.ADDR + PGSIZD) - 1),
        PAGED.as_slice(),
        ctx,
    )?;

    Ok(())
}

//$Procedure  ZZEKPGWI ( Private: EK, write integer page )
pub fn ZZEKPGWI(HANDLE: i32, P: i32, PAGEI: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let PAGEI = DummyArray::new(PAGEI, 1..);

    //
    // Use discovery check-in.
    //
    // Validate the file.
    //
    ZZEKPGCH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Find out how many integer pages are in use.
    //
    DASRDI(
        HANDLE,
        EPNPI,
        EPNPI,
        std::slice::from_mut(&mut save.NPI),
        ctx,
    )?;

    if ((P < 1) || (P > save.NPI)) {
        CHKIN(b"ZZEKPGWI", ctx)?;
        SETMSG(b"INT page = #; valid range is [1:#]", ctx);
        ERRINT(b"#", P, ctx);
        ERRINT(b"#", save.NPI, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKPGWI", ctx)?;
        return Ok(());
    }

    save.ADDR = ((PGBASI + ((P - 1) * PGSIZI)) + 1);

    DASUDI(
        HANDLE,
        save.ADDR,
        ((save.ADDR + PGSIZI) - 1),
        PAGEI.as_slice(),
        ctx,
    )?;

    Ok(())
}

//$Procedure  ZZEKPGBS ( Private: EK, map page to base address )
pub fn ZZEKPGBS(TYPE: i32, P: i32, BASE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    if (TYPE == CHR) {
        *BASE = ((P - 1) * PGSIZC);
    } else if (TYPE == DP) {
        *BASE = ((P - 1) * PGSIZD);
    } else if (TYPE == INT) {
        *BASE = (((P - 1) * PGSIZI) + PGBASI);
    } else {
        CHKIN(b"ZZEKPGBS", ctx)?;
        SETMSG(b"The data type code # was not recognized.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKPGBS", ctx)?;
        return Ok(());
    }

    Ok(())
}

//$Procedure  ZZEKPGPG ( Private: EK, map address to page )
pub fn ZZEKPGPG(
    TYPE: i32,
    ADDRSS: i32,
    P: &mut i32,
    BASE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    if (TYPE == CHR) {
        *P = (((ADDRSS + PGSIZC) - 1) / PGSIZC);
        *BASE = ((*P - 1) * PGSIZC);
    } else if (TYPE == DP) {
        *P = (((ADDRSS + PGSIZD) - 1) / PGSIZD);
        *BASE = ((*P - 1) * PGSIZD);
    } else if (TYPE == INT) {
        *P = ((((ADDRSS - PGBASI) + PGSIZI) - 1) / PGSIZI);
        *BASE = (((*P - 1) * PGSIZI) + PGBASI);
    } else {
        CHKIN(b"ZZEKPGPG", ctx)?;
        SETMSG(b"The data type code # was not recognized.", ctx);
        ERRINT(b"#", TYPE, ctx);
        SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
        CHKOUT(b"ZZEKPGPG", ctx)?;
        return Ok(());
    }

    Ok(())
}

//$Procedure  ZZEKPGST ( Private: EK, return paging statistics )
pub fn ZZEKPGST(
    HANDLE: i32,
    STAT: &[u8],
    VALUE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    CHKIN(b"ZZEKPGST", ctx)?;

    if EQSTR(STAT, b"N_C_ALLOC") {
        DASRDI(HANDLE, EPNPC, EPNPC, std::slice::from_mut(VALUE), ctx)?;
    } else if EQSTR(STAT, b"N_D_ALLOC") {
        DASRDI(HANDLE, EPNPD, EPNPD, std::slice::from_mut(VALUE), ctx)?;
    } else if EQSTR(STAT, b"N_I_ALLOC") {
        DASRDI(HANDLE, EPNPI, EPNPI, std::slice::from_mut(VALUE), ctx)?;
    } else if EQSTR(STAT, b"N_C_FREE") {
        DASRDI(HANDLE, EPNFPC, EPNFPC, std::slice::from_mut(VALUE), ctx)?;
    } else if EQSTR(STAT, b"N_D_FREE") {
        DASRDI(HANDLE, EPNFPD, EPNFPD, std::slice::from_mut(VALUE), ctx)?;
    } else if EQSTR(STAT, b"N_I_FREE") {
        DASRDI(HANDLE, EPNFPI, EPNFPI, std::slice::from_mut(VALUE), ctx)?;
    } else {
        SETMSG(b"Statistic # is not supported.", ctx);
        ERRCH(b"#", STAT, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"ZZEKPGST", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZEKPGST", ctx)?;
    Ok(())
}
