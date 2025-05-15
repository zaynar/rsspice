#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use f2rust_std::{Context, Result};
    use tempfile::TempDir;

    use crate::generated::{testutil, tspice};

    const USE_VIRTUAL_FILESYSTEM: bool = true;

    #[allow(dead_code)]
    fn tspice_verbose<F>(testcase: F) -> Result<()>
    where
        F: FnOnce(&mut bool, &mut Context) -> Result<()>,
    {
        tspice_cfg(testcase, true)
    }

    fn tspice<F>(testcase: F) -> Result<()>
    where
        F: FnOnce(&mut bool, &mut Context) -> Result<()>,
    {
        tspice_cfg(testcase, false)
    }

    fn tspice_cfg<F>(testcase: F, verbose: bool) -> Result<()>
    where
        F: FnOnce(&mut bool, &mut Context) -> Result<()>,
    {
        let tmp; // declare in outer scope, so it's dropped after the test completes

        let mut ctx = if USE_VIRTUAL_FILESYSTEM {
            Context::with_file_manager(f2rust_std::io::VirtualFileManager::new())
        } else {
            tmp = TempDir::with_prefix("rsspice-")?;
            let cwd = tmp.path().to_path_buf();

            if verbose {
                println!("Temp path: {}", tmp.path().display());

                // Prevent TempDir deleting the path
                let _ = tmp.keep();
            }

            Context::with_file_manager(f2rust_std::io::FsFileManager::new(&cwd))
        };

        let mut cmline = if verbose {
            b"-C".to_vec()
        } else {
            b" ".to_vec()
        };
        testutil::TSETUP(
            &mut cmline,
            b"spice{0-9}{0-9}{0-9}{0-9}.log",
            b"RSSPICE 0.01",
            &mut ctx,
        )?;

        let mut ok = false;
        testcase(&mut ok, &mut ctx)?;

        testutil::TCLOSE(&mut ctx)?;
        drop(ctx);

        assert!(ok, "test case failed");

        Ok(())
    }

    #[test]
    fn F_AAAAPHSH() -> Result<()> {
        tspice(tspice::F_AAAAPHSH)
    }

    #[test]
    fn F_AB() -> Result<()> {
        tspice(tspice::F_AB)
    }

    #[test]
    fn F_ASNACSN() -> Result<()> {
        tspice(tspice::F_ASNACSN)
    }

    #[test]
    fn F_AZL() -> Result<()> {
        tspice(tspice::F_AZL)
    }

    #[test]
    fn F_AZLCPV() -> Result<()> {
        tspice(tspice::F_AZLCPV)
    }

    #[test]
    fn F_BADKPV() -> Result<()> {
        tspice(tspice::F_BADKPV)
    }

    #[test]
    fn F_BODCOD() -> Result<()> {
        tspice(tspice::F_BODCOD)
    }

    #[test]
    fn F_BODVAR() -> Result<()> {
        tspice(tspice::F_BODVAR)
    }

    #[test]
    fn F_CCIFRM() -> Result<()> {
        tspice(tspice::F_CCIFRM)
    }

    #[test]
    fn F_CHBIGR() -> Result<()> {
        tspice(tspice::F_CHBIGR)
    }

    #[test]
    fn F_CHBSHV() -> Result<()> {
        tspice(tspice::F_CHBSHV)
    }

    #[test]
    fn F_CHGIRF() -> Result<()> {
        tspice(tspice::F_CHGIRF)
    }

    #[test]
    fn F_CHGRFX() -> Result<()> {
        tspice(tspice::F_CHGRFX)
    }

    #[test]
    fn F_CK02() -> Result<()> {
        tspice(tspice::F_CK02)
    }

    #[test]
    fn F_CK03() -> Result<()> {
        tspice(tspice::F_CK03)
    }

    #[test]
    fn F_CK05() -> Result<()> {
        tspice(tspice::F_CK05)
    }

    #[test]
    fn F_CK06() -> Result<()> {
        tspice(tspice::F_CK06)
    }

    #[test]
    fn F_CKBSR() -> Result<()> {
        tspice(tspice::F_CKBSR)
    }

    #[test]
    fn F_CKCOV() -> Result<()> {
        tspice(tspice::F_CKCOV)
    }

    #[test]
    fn F_CKFROT() -> Result<()> {
        tspice(tspice::F_CKFROT)
    }

    #[test]
    fn F_CKFXFM() -> Result<()> {
        tspice(tspice::F_CKFXFM)
    }

    #[test]
    fn F_CKGP() -> Result<()> {
        tspice(tspice::F_CKGP)
    }

    #[test]
    fn F_CKMETA() -> Result<()> {
        tspice(tspice::F_CKMETA)
    }

    #[test]
    fn F_CKW01() -> Result<()> {
        tspice(tspice::F_CKW01)
    }

    #[test]
    fn F_CKW02() -> Result<()> {
        tspice(tspice::F_CKW02)
    }

    #[test]
    fn F_CKW03() -> Result<()> {
        tspice(tspice::F_CKW03)
    }

    #[test]
    fn F_CONVRT() -> Result<()> {
        tspice(tspice::F_CONVRT)
    }

    #[test]
    fn F_CRDCNV() -> Result<()> {
        tspice(tspice::F_CRDCNV)
    }

    #[test]
    fn F_CYIP() -> Result<()> {
        tspice(tspice::F_CYIP)
    }

    #[test]
    fn F_DAFAH() -> Result<()> {
        tspice(tspice::F_DAFAH)
    }

    #[test]
    fn F_DAFANA() -> Result<()> {
        tspice(tspice::F_DAFANA)
    }

    #[test]
    fn F_DAFNN() -> Result<()> {
        tspice(tspice::F_DAFNN)
    }

    #[test]
    fn F_DAS() -> Result<()> {
        tspice(tspice::F_DAS)
    }

    #[test]
    fn F_DASA2L() -> Result<()> {
        tspice(tspice::F_DASA2L)
    }

    #[test]
    fn F_DASCUD() -> Result<()> {
        tspice(tspice::F_DASCUD)
    }

    #[test]
    fn F_DASFR() -> Result<()> {
        tspice(tspice::F_DASFR)
    }

    #[test]
    fn F_DASMUL() -> Result<()> {
        tspice(tspice::F_DASMUL)
    }

    #[test]
    fn F_DDHCLS() -> Result<()> {
        tspice(tspice::F_DDHCLS)
    }

    #[test]
    fn F_DDHCLU() -> Result<()> {
        tspice(tspice::F_DDHCLU)
    }

    #[test]
    fn F_DDHF2H() -> Result<()> {
        tspice(tspice::F_DDHF2H)
    }

    #[test]
    fn F_DDHFNH() -> Result<()> {
        tspice(tspice::F_DDHFNH)
    }

    #[test]
    fn F_DDHFTSIZE() -> Result<()> {
        tspice(tspice::F_DDHFTSIZE)
    }

    #[test]
    fn F_DDHGSD() -> Result<()> {
        tspice(tspice::F_DDHGSD)
    }

    #[test]
    fn F_DDHGTU() -> Result<()> {
        tspice(tspice::F_DDHGTU)
    }

    #[test]
    fn F_DDHHLU() -> Result<()> {
        tspice(tspice::F_DDHHLU)
    }

    #[test]
    fn F_DDHISN() -> Result<()> {
        tspice(tspice::F_DDHISN)
    }

    #[test]
    fn F_DDHIVF() -> Result<()> {
        tspice(tspice::F_DDHIVF)
    }

    #[test]
    fn F_DDHLUH() -> Result<()> {
        tspice(tspice::F_DDHLUH)
    }

    #[test]
    fn F_DDHNFO() -> Result<()> {
        tspice(tspice::F_DDHNFO)
    }

    #[test]
    fn F_DDHOPN() -> Result<()> {
        tspice(tspice::F_DDHOPN)
    }

    #[test]
    fn F_DDHPPF() -> Result<()> {
        tspice(tspice::F_DDHPPF)
    }

    #[test]
    fn F_DDHRCM() -> Result<()> {
        tspice(tspice::F_DDHRCM)
    }

    #[test]
    fn F_DDHRMU() -> Result<()> {
        tspice(tspice::F_DDHRMU)
    }

    #[test]
    fn F_DDHUNL() -> Result<()> {
        tspice(tspice::F_DDHUNL)
    }

    #[test]
    fn F_DHFA() -> Result<()> {
        tspice(tspice::F_DHFA)
    }

    #[test]
    fn F_DLA() -> Result<()> {
        tspice(tspice::F_DLA)
    }

    #[test]
    fn F_DNEARP() -> Result<()> {
        tspice(tspice::F_DNEARP)
    }

    #[test]
    fn F_DPFMT() -> Result<()> {
        tspice(tspice::F_DPFMT)
    }

    #[test]
    fn F_DPSTRF() -> Result<()> {
        tspice(tspice::F_DPSTRF)
    }

    #[test]
    fn F_DSK02() -> Result<()> {
        tspice(tspice::F_DSK02)
    }

    #[test]
    fn F_DSKKPR() -> Result<()> {
        tspice(tspice::F_DSKKPR)
    }

    #[test]
    fn F_DSKMI2() -> Result<()> {
        tspice(tspice::F_DSKMI2)
    }

    #[test]
    fn F_DSKOBJ() -> Result<()> {
        tspice(tspice::F_DSKOBJ)
    }

    #[test]
    fn F_DSKRB2() -> Result<()> {
        tspice(tspice::F_DSKRB2)
    }

    #[test]
    fn F_DSKTOL() -> Result<()> {
        tspice(tspice::F_DSKTOL)
    }

    #[test]
    fn F_DSKW02() -> Result<()> {
        tspice(tspice::F_DSKW02)
    }

    #[test]
    fn F_DSKX02() -> Result<()> {
        tspice(tspice::F_DSKX02)
    }

    #[test]
    fn F_DSKXSI() -> Result<()> {
        tspice(tspice::F_DSKXSI)
    }

    #[test]
    fn F_DSKXV() -> Result<()> {
        tspice(tspice::F_DSKXV)
    }

    #[test]
    fn F_DVOPS() -> Result<()> {
        tspice(tspice::F_DVOPS)
    }

    #[test]
    fn F_DVSEP() -> Result<()> {
        tspice(tspice::F_DVSEP)
    }

    #[test]
    fn F_DWPOOL() -> Result<()> {
        tspice(tspice::F_DWPOOL)
    }

    #[test]
    fn F_DYN01() -> Result<()> {
        tspice(tspice::F_DYN01)
    }

    #[test]
    fn F_DYN02() -> Result<()> {
        tspice(tspice::F_DYN02)
    }

    #[test]
    fn F_DYN03() -> Result<()> {
        tspice(tspice::F_DYN03)
    }

    #[test]
    fn F_DYN04() -> Result<()> {
        tspice(tspice::F_DYN04)
    }

    #[test]
    fn F_DYN05() -> Result<()> {
        tspice(tspice::F_DYN05)
    }

    #[test]
    fn F_DYN06() -> Result<()> {
        tspice(tspice::F_DYN06)
    }

    #[test]
    fn F_DYN07() -> Result<()> {
        tspice(tspice::F_DYN07)
    }

    #[test]
    fn F_DYN08() -> Result<()> {
        tspice(tspice::F_DYN08)
    }

    #[test]
    fn F_EDNMPT() -> Result<()> {
        tspice(tspice::F_EDNMPT)
    }

    #[test]
    fn F_EDPNT() -> Result<()> {
        tspice(tspice::F_EDPNT)
    }

    #[test]
    fn F_EK01() -> Result<()> {
        tspice(tspice::F_EK01)
    }

    #[test]
    fn F_EK02() -> Result<()> {
        tspice(tspice::F_EK02)
    }

    #[test]
    fn F_EK03() -> Result<()> {
        tspice(tspice::F_EK03)
    }

    #[test]
    fn F_EK04() -> Result<()> {
        tspice(tspice::F_EK04)
    }

    #[test]
    fn F_EQNCPV() -> Result<()> {
        tspice(tspice::F_EQNCPV)
    }

    #[test]
    fn F_ET2LST() -> Result<()> {
        tspice(tspice::F_ET2LST)
    }

    #[test]
    fn F_ET2UTC() -> Result<()> {
        tspice(tspice::F_ET2UTC)
    }

    #[test]
    fn F_ETCAL() -> Result<()> {
        tspice(tspice::F_ETCAL)
    }

    #[test]
    fn F_EULER() -> Result<()> {
        tspice(tspice::F_EULER)
    }

    #[test]
    fn F_FNDCMP() -> Result<()> {
        tspice(tspice::F_FNDCMP)
    }

    #[test]
    fn F_FOVRAY() -> Result<()> {
        tspice(tspice::F_FOVRAY)
    }

    #[test]
    fn F_FOVTRG() -> Result<()> {
        tspice(tspice::F_FOVTRG)
    }

    #[test]
    fn F_FRAMEX() -> Result<()> {
        tspice(tspice::F_FRAMEX)
    }

    #[test]
    fn F_FRFTCH() -> Result<()> {
        tspice(tspice::F_FRFTCH)
    }

    #[test]
    fn F_FRMCHG() -> Result<()> {
        tspice(tspice::F_FRMCHG)
    }

    #[test]
    fn F_FRMGET() -> Result<()> {
        tspice(tspice::F_FRMGET)
    }

    #[test]
    fn F_GE01() -> Result<()> {
        tspice(tspice::F_GE01)
    }

    #[test]
    fn F_GETFOV() -> Result<()> {
        tspice(tspice::F_GETFOV)
    }

    #[test]
    fn F_GETFV2() -> Result<()> {
        tspice(tspice::F_GETFV2)
    }

    #[test]
    fn F_GFBAIL() -> Result<()> {
        tspice(tspice::F_GFBAIL)
    }

    #[test]
    fn F_GFBIRP() -> Result<()> {
        tspice(tspice::F_GFBIRP)
    }

    #[test]
    fn F_GFDIRP() -> Result<()> {
        tspice(tspice::F_GFDIRP)
    }

    #[test]
    fn F_GFDIST() -> Result<()> {
        tspice(tspice::F_GFDIST)
    }

    #[test]
    fn F_GFEVNT() -> Result<()> {
        tspice(tspice::F_GFEVNT)
    }

    #[test]
    fn F_GFFOVE() -> Result<()> {
        tspice(tspice::F_GFFOVE)
    }

    #[test]
    fn F_GFILUM() -> Result<()> {
        tspice(tspice::F_GFILUM)
    }

    #[test]
    fn F_GFLORP() -> Result<()> {
        tspice(tspice::F_GFLORP)
    }

    #[test]
    fn F_GFOCCE() -> Result<()> {
        tspice(tspice::F_GFOCCE)
    }

    #[test]
    fn F_GFOCLT() -> Result<()> {
        tspice(tspice::F_GFOCLT)
    }

    #[test]
    fn F_GFPA() -> Result<()> {
        tspice(tspice::F_GFPA)
    }

    #[test]
    fn F_GFPCRP() -> Result<()> {
        tspice(tspice::F_GFPCRP)
    }

    #[test]
    fn F_GFPOSC() -> Result<()> {
        tspice(tspice::F_GFPOSC)
    }

    #[test]
    fn F_GFREFN() -> Result<()> {
        tspice(tspice::F_GFREFN)
    }

    #[test]
    fn F_GFRFOV() -> Result<()> {
        tspice(tspice::F_GFRFOV)
    }

    #[test]
    fn F_GFRPRT() -> Result<()> {
        tspice(tspice::F_GFRPRT)
    }

    #[test]
    fn F_GFRR() -> Result<()> {
        tspice(tspice::F_GFRR)
    }

    #[test]
    fn F_GFSCRP() -> Result<()> {
        tspice(tspice::F_GFSCRP)
    }

    #[test]
    fn F_GFSEP() -> Result<()> {
        tspice(tspice::F_GFSEP)
    }

    #[test]
    fn F_GFSNTC() -> Result<()> {
        tspice(tspice::F_GFSNTC)
    }

    #[test]
    fn F_GFSPRP() -> Result<()> {
        tspice(tspice::F_GFSPRP)
    }

    #[test]
    fn F_GFSTEP() -> Result<()> {
        tspice(tspice::F_GFSTEP)
    }

    #[test]
    fn F_GFSUBC() -> Result<()> {
        tspice(tspice::F_GFSUBC)
    }

    #[test]
    fn F_GFTFOV() -> Result<()> {
        tspice(tspice::F_GFTFOV)
    }

    #[test]
    fn F_GFUDB() -> Result<()> {
        tspice(tspice::F_GFUDB)
    }

    #[test]
    fn F_GFUDS() -> Result<()> {
        tspice(tspice::F_GFUDS)
    }

    #[test]
    fn F_GFXCRP() -> Result<()> {
        tspice(tspice::F_GFXCRP)
    }

    #[test]
    fn F_HRMITE() -> Result<()> {
        tspice(tspice::F_HRMITE)
    }

    #[test]
    fn F_ILLUMF() -> Result<()> {
        tspice(tspice::F_ILLUMF)
    }

    #[test]
    fn F_ILLUMG() -> Result<()> {
        tspice(tspice::F_ILLUMG)
    }

    #[test]
    fn F_ILUMIN() -> Result<()> {
        tspice(tspice::F_ILUMIN)
    }

    #[test]
    fn F_INCNSG() -> Result<()> {
        tspice(tspice::F_INCNSG)
    }

    #[test]
    fn F_INELPL() -> Result<()> {
        tspice(tspice::F_INELPL)
    }

    #[test]
    fn F_INRYPL() -> Result<()> {
        tspice(tspice::F_INRYPL)
    }

    #[test]
    fn F_INSANG() -> Result<()> {
        tspice(tspice::F_INSANG)
    }

    #[test]
    fn F_INSERT() -> Result<()> {
        tspice(tspice::F_INSERT)
    }

    #[test]
    fn F_IOVCMP() -> Result<()> {
        tspice(tspice::F_IOVCMP)
    }

    #[test]
    fn F_JUL2GR() -> Result<()> {
        tspice(tspice::F_JUL2GR)
    }

    #[test]
    fn F_KEEPER() -> Result<()> {
        tspice(tspice::F_KEEPER)
    }

    #[test]
    fn F_KPBUG() -> Result<()> {
        tspice(tspice::F_KPBUG)
    }

    #[test]
    fn F_KPSOLV() -> Result<()> {
        tspice(tspice::F_KPSOLV)
    }

    #[test]
    fn F_LAGRNG() -> Result<()> {
        tspice(tspice::F_LAGRNG)
    }

    #[test]
    fn F_LATSRF() -> Result<()> {
        tspice(tspice::F_LATSRF)
    }

    #[test]
    fn F_LIMBPT() -> Result<()> {
        tspice(tspice::F_LIMBPT)
    }

    #[test]
    fn F_LOCATI() -> Result<()> {
        tspice(tspice::F_LOCATI)
    }

    #[test]
    fn F_LPARSE() -> Result<()> {
        tspice(tspice::F_LPARSE)
    }

    #[test]
    fn F_LS() -> Result<()> {
        tspice(tspice::F_LS)
    }

    #[test]
    fn F_LTIME() -> Result<()> {
        tspice(tspice::F_LTIME)
    }

    #[test]
    fn F_M2Q() -> Result<()> {
        tspice(tspice::F_M2Q)
    }

    #[test]
    fn F_MATRIX3() -> Result<()> {
        tspice(tspice::F_MATRIX3)
    }

    #[test]
    fn F_MATRIXG() -> Result<()> {
        tspice(tspice::F_MATRIXG)
    }

    #[test]
    fn F_MOVED() -> Result<()> {
        tspice(tspice::F_MOVED)
    }

    #[test]
    fn F_NEARPT() -> Result<()> {
        tspice(tspice::F_NEARPT)
    }

    #[test]
    fn F_NNEK01() -> Result<()> {
        tspice(tspice::F_NNEK01)
    }

    #[test]
    fn F_NNEK03() -> Result<()> {
        tspice(tspice::F_NNEK03)
    }

    #[test]
    fn F_NNEK04() -> Result<()> {
        tspice(tspice::F_NNEK04)
    }

    #[test]
    fn F_NPEDLN() -> Result<()> {
        tspice(tspice::F_NPEDLN)
    }

    #[test]
    fn F_NPELPT() -> Result<()> {
        tspice(tspice::F_NPELPT)
    }

    #[test]
    fn F_NPLNPT() -> Result<()> {
        tspice(tspice::F_NPLNPT)
    }

    #[test]
    fn F_NPSGPT() -> Result<()> {
        tspice(tspice::F_NPSGPT)
    }

    #[test]
    fn F_OCCULT() -> Result<()> {
        tspice(tspice::F_OCCULT)
    }

    #[test]
    fn F_OSCELT() -> Result<()> {
        tspice(tspice::F_OSCELT)
    }

    #[test]
    fn F_OSCLTX() -> Result<()> {
        tspice(tspice::F_OSCLTX)
    }

    #[test]
    fn F_PCK20() -> Result<()> {
        tspice(tspice::F_PCK20)
    }

    #[test]
    fn F_PCKBSR() -> Result<()> {
        tspice(tspice::F_PCKBSR)
    }

    #[test]
    fn F_PCKBUF() -> Result<()> {
        tspice(tspice::F_PCKBUF)
    }

    #[test]
    fn F_PCKCOV() -> Result<()> {
        tspice(tspice::F_PCKCOV)
    }

    #[test]
    fn F_PGR() -> Result<()> {
        tspice(tspice::F_PGR)
    }

    #[test]
    fn F_PHASEQ() -> Result<()> {
        tspice(tspice::F_PHASEQ)
    }

    #[test]
    fn F_PLN() -> Result<()> {
        tspice(tspice::F_PLN)
    }

    #[test]
    fn F_PLT() -> Result<()> {
        tspice(tspice::F_PLT)
    }

    #[test]
    fn F_PLTNP() -> Result<()> {
        tspice(tspice::F_PLTNP)
    }

    #[test]
    fn F_POLYDS() -> Result<()> {
        tspice(tspice::F_POLYDS)
    }

    #[test]
    fn F_POOL() -> Result<()> {
        tspice(tspice::F_POOL)
    }

    #[test]
    fn F_PRDFRM() -> Result<()> {
        tspice(tspice::F_PRDFRM)
    }

    #[test]
    fn F_PRJP() -> Result<()> {
        tspice(tspice::F_PRJP)
    }

    #[test]
    fn F_PXFORM() -> Result<()> {
        tspice(tspice::F_PXFORM)
    }

    #[test]
    fn F_PXFRM2() -> Result<()> {
        tspice(tspice::F_PXFRM2)
    }

    #[test]
    fn F_Q2M() -> Result<()> {
        tspice(tspice::F_Q2M)
    }

    #[test]
    fn F_QDERIV() -> Result<()> {
        tspice(tspice::F_QDERIV)
    }

    #[test]
    fn F_QUAT() -> Result<()> {
        tspice(tspice::F_QUAT)
    }

    #[test]
    fn F_RC2GRD() -> Result<()> {
        tspice(tspice::F_RC2GRD)
    }

    #[test]
    fn F_RDPCK() -> Result<()> {
        tspice(tspice::F_RDPCK)
    }

    #[test]
    fn F_RDPCKD() -> Result<()> {
        tspice(tspice::F_RDPCKD)
    }

    #[test]
    fn F_REFCHG() -> Result<()> {
        tspice(tspice::F_REFCHG)
    }

    #[test]
    fn F_REGLON() -> Result<()> {
        tspice(tspice::F_REGLON)
    }

    #[test]
    fn F_REPMX() -> Result<()> {
        tspice(tspice::F_REPMX)
    }

    #[test]
    fn F_ROTGET() -> Result<()> {
        tspice(tspice::F_ROTGET)
    }

    #[test]
    fn F_SAELGV() -> Result<()> {
        tspice(tspice::F_SAELGV)
    }

    #[test]
    fn F_SBF() -> Result<()> {
        tspice(tspice::F_SBF)
    }

    #[test]
    fn F_SCLK() -> Result<()> {
        tspice(tspice::F_SCLK)
    }

    #[test]
    fn F_SCLK0() -> Result<()> {
        tspice(tspice::F_SCLK0)
    }

    #[test]
    fn F_SCLK1() -> Result<()> {
        tspice(tspice::F_SCLK1)
    }

    #[test]
    fn F_SCTRAN() -> Result<()> {
        tspice(tspice::F_SCTRAN)
    }

    #[test]
    fn F_SEPOOL() -> Result<()> {
        tspice(tspice::F_SEPOOL)
    }

    #[test]
    fn F_SGMETA() -> Result<()> {
        tspice(tspice::F_SGMETA)
    }

    #[test]
    fn F_SHARPR() -> Result<()> {
        tspice(tspice::F_SHARPR)
    }

    #[test]
    fn F_SINCPT() -> Result<()> {
        tspice(tspice::F_SINCPT)
    }

    #[test]
    fn F_SLICE() -> Result<()> {
        tspice(tspice::F_SLICE)
    }

    #[test]
    fn F_SPK01() -> Result<()> {
        tspice(tspice::F_SPK01)
    }

    #[test]
    fn F_SPK02() -> Result<()> {
        tspice(tspice::F_SPK02)
    }

    #[test]
    fn F_SPK03() -> Result<()> {
        tspice(tspice::F_SPK03)
    }

    #[test]
    fn F_SPK05() -> Result<()> {
        tspice(tspice::F_SPK05)
    }

    #[test]
    fn F_SPK08() -> Result<()> {
        tspice(tspice::F_SPK08)
    }

    #[test]
    fn F_SPK09() -> Result<()> {
        tspice(tspice::F_SPK09)
    }

    #[test]
    fn F_SPK10() -> Result<()> {
        tspice(tspice::F_SPK10)
    }

    #[test]
    fn F_SPK12() -> Result<()> {
        tspice(tspice::F_SPK12)
    }

    #[test]
    fn F_SPK13() -> Result<()> {
        tspice(tspice::F_SPK13)
    }

    #[test]
    fn F_SPK14() -> Result<()> {
        tspice(tspice::F_SPK14)
    }

    #[test]
    fn F_SPK17() -> Result<()> {
        tspice(tspice::F_SPK17)
    }

    #[test]
    fn F_SPK18() -> Result<()> {
        tspice(tspice::F_SPK18)
    }

    #[test]
    fn F_SPK19() -> Result<()> {
        tspice(tspice::F_SPK19)
    }

    #[test]
    fn F_SPK20() -> Result<()> {
        tspice(tspice::F_SPK20)
    }

    #[test]
    fn F_SPK21() -> Result<()> {
        tspice(tspice::F_SPK21)
    }

    #[test]
    fn F_SPKAPO() -> Result<()> {
        tspice(tspice::F_SPKAPO)
    }

    #[test]
    fn F_SPKAPP() -> Result<()> {
        tspice(tspice::F_SPKAPP)
    }

    #[test]
    fn F_SPKBSR() -> Result<()> {
        tspice(tspice::F_SPKBSR)
    }

    #[test]
    fn F_SPKCOR() -> Result<()> {
        tspice(tspice::F_SPKCOR)
    }

    #[test]
    fn F_SPKCOV() -> Result<()> {
        tspice(tspice::F_SPKCOV)
    }

    #[test]
    fn F_SPKCPV() -> Result<()> {
        tspice(tspice::F_SPKCPV)
    }

    #[test]
    fn F_SPKE15() -> Result<()> {
        tspice(tspice::F_SPKE15)
    }

    #[test]
    fn F_SPKEZ() -> Result<()> {
        tspice(tspice::F_SPKEZ)
    }

    #[test]
    fn F_SPKEZP() -> Result<()> {
        tspice(tspice::F_SPKEZP)
    }

    #[test]
    fn F_SPKF15() -> Result<()> {
        tspice(tspice::F_SPKF15)
    }

    #[test]
    fn F_SPKGEO() -> Result<()> {
        tspice(tspice::F_SPKGEO)
    }

    #[test]
    fn F_SPKGPS() -> Result<()> {
        tspice(tspice::F_SPKGPS)
    }

    #[test]
    fn F_SPKGPX() -> Result<()> {
        tspice(tspice::F_SPKGPX)
    }

    #[test]
    fn F_SPKGXC() -> Result<()> {
        tspice(tspice::F_SPKGXC)
    }

    #[test]
    fn F_SPKPDS() -> Result<()> {
        tspice(tspice::F_SPKPDS)
    }

    #[test]
    fn F_SPKPVN() -> Result<()> {
        tspice(tspice::F_SPKPVN)
    }

    #[test]
    fn F_SPKS19() -> Result<()> {
        tspice(tspice::F_SPKS19)
    }

    #[test]
    fn F_SPKSPV() -> Result<()> {
        tspice(tspice::F_SPKSPV)
    }

    #[test]
    fn F_SPKW01() -> Result<()> {
        tspice(tspice::F_SPKW01)
    }

    #[test]
    fn F_SRFNRM() -> Result<()> {
        tspice(tspice::F_SRFNRM)
    }

    #[test]
    fn F_SRFTRN() -> Result<()> {
        tspice(tspice::F_SRFTRN)
    }

    #[test]
    fn F_SRFXPT() -> Result<()> {
        tspice(tspice::F_SRFXPT)
    }

    #[test]
    fn F_ST00() -> Result<()> {
        tspice(tspice::F_ST00)
    }

    #[test]
    fn F_STLABX() -> Result<()> {
        tspice(tspice::F_STLABX)
    }

    #[test]
    fn F_STMP03() -> Result<()> {
        tspice(tspice::F_STMP03)
    }

    #[test]
    fn F_STPOOL() -> Result<()> {
        tspice(tspice::F_STPOOL)
    }

    #[test]
    fn F_STR2ET() -> Result<()> {
        tspice(tspice::F_STR2ET)
    }

    #[test]
    fn F_STRING() -> Result<()> {
        tspice(tspice::F_STRING)
    }

    #[test]
    fn F_SUBPNT() -> Result<()> {
        tspice(tspice::F_SUBPNT)
    }

    #[test]
    fn F_SUBSLR() -> Result<()> {
        tspice(tspice::F_SUBSLR)
    }

    #[test]
    fn F_SURFNM() -> Result<()> {
        tspice(tspice::F_SURFNM)
    }

    #[test]
    fn F_SURFPV() -> Result<()> {
        tspice(tspice::F_SURFPV)
    }

    #[test]
    fn F_SWAPAC() -> Result<()> {
        tspice(tspice::F_SWAPAC)
    }

    #[test]
    fn F_SWAPAD() -> Result<()> {
        tspice(tspice::F_SWAPAD)
    }

    #[test]
    fn F_SWAPAI() -> Result<()> {
        tspice(tspice::F_SWAPAI)
    }

    #[test]
    fn F_SXFORM() -> Result<()> {
        tspice(tspice::F_SXFORM)
    }

    #[test]
    fn F_SYMTBC() -> Result<()> {
        tspice(tspice::F_SYMTBC)
    }

    #[test]
    fn F_SYMTBD() -> Result<()> {
        tspice(tspice::F_SYMTBD)
    }

    #[test]
    fn F_SYMTBI() -> Result<()> {
        tspice(tspice::F_SYMTBI)
    }

    #[test]
    fn F_T_URAND() -> Result<()> {
        tspice(tspice::F_T_URAND)
    }

    #[test]
    fn F_TABTXT() -> Result<()> {
        tspice(tspice::F_TABTXT)
    }

    #[test]
    fn F_TANGPT0() -> Result<()> {
        tspice(tspice::F_TANGPT0)
    }

    #[test]
    fn F_TANGPT1() -> Result<()> {
        tspice(tspice::F_TANGPT1)
    }

    #[test]
    fn F_TANGPT2() -> Result<()> {
        tspice(tspice::F_TANGPT2)
    }

    #[test]
    fn F_TANGPT3() -> Result<()> {
        tspice(tspice::F_TANGPT3)
    }

    #[test]
    fn F_TANGPTW() -> Result<()> {
        tspice(tspice::F_TANGPTW)
    }

    #[test]
    fn F_TCHECK() -> Result<()> {
        tspice(tspice::F_TCHECK)
    }

    #[test]
    fn F_TERM() -> Result<()> {
        tspice(tspice::F_TERM)
    }

    #[test]
    fn F_TERMPT() -> Result<()> {
        tspice(tspice::F_TERMPT)
    }

    #[test]
    fn F_TEXPYR() -> Result<()> {
        tspice(tspice::F_TEXPYR)
    }

    #[test]
    fn F_TIMCVR() -> Result<()> {
        tspice(tspice::F_TIMCVR)
    }

    #[test]
    fn F_TIMDEF() -> Result<()> {
        tspice(tspice::F_TIMDEF)
    }

    #[test]
    fn F_TIMOUT() -> Result<()> {
        tspice(tspice::F_TIMOUT)
    }

    #[test]
    fn F_TKFRAM() -> Result<()> {
        tspice(tspice::F_TKFRAM)
    }

    #[test]
    fn F_TLE() -> Result<()> {
        tspice(tspice::F_TLE)
    }

    #[test]
    fn F_TNCNSG() -> Result<()> {
        tspice(tspice::F_TNCNSG)
    }

    #[test]
    fn F_TPARSE() -> Result<()> {
        tspice(tspice::F_TPARSE)
    }

    #[test]
    fn F_TPARTV1() -> Result<()> {
        tspice(tspice::F_TPARTV1)
    }

    #[test]
    fn F_TPARTV2() -> Result<()> {
        tspice(tspice::F_TPARTV2)
    }

    #[test]
    fn F_TRGSEP() -> Result<()> {
        tspice(tspice::F_TRGSEP)
    }

    #[test]
    fn F_TSTCK3() -> Result<()> {
        tspice(tspice::F_TSTCK3)
    }

    #[test]
    fn F_TTRANS() -> Result<()> {
        tspice(tspice::F_TTRANS)
    }

    #[test]
    fn F_TWOVXF() -> Result<()> {
        tspice(tspice::F_TWOVXF)
    }

    #[test]
    fn F_UTC2ET() -> Result<()> {
        tspice(tspice::F_UTC2ET)
    }

    #[test]
    fn F_VECTOR3() -> Result<()> {
        tspice(tspice::F_VECTOR3)
    }

    #[test]
    fn F_VECTORG() -> Result<()> {
        tspice(tspice::F_VECTORG)
    }

    #[test]
    fn F_VOXEL() -> Result<()> {
        tspice(tspice::F_VOXEL)
    }

    #[test]
    fn F_VSTRNG() -> Result<()> {
        tspice(tspice::F_VSTRNG)
    }

    #[test]
    fn F_WIN() -> Result<()> {
        tspice(tspice::F_WIN)
    }

    #[test]
    fn F_XDDA() -> Result<()> {
        tspice(tspice::F_XDDA)
    }

    #[test]
    fn F_XFMSTA() -> Result<()> {
        tspice(tspice::F_XFMSTA)
    }

    #[test]
    fn F_XFNEUL() -> Result<()> {
        tspice(tspice::F_XFNEUL)
    }

    #[test]
    fn F_XFRAV() -> Result<()> {
        tspice(tspice::F_XFRAV)
    }

    #[test]
    fn F_XLATED() -> Result<()> {
        tspice(tspice::F_XLATED)
    }

    #[test]
    fn F_XLATEI() -> Result<()> {
        tspice(tspice::F_XLATEI)
    }

    #[test]
    fn F_ZZASC1() -> Result<()> {
        tspice(tspice::F_ZZASC1)
    }

    #[test]
    fn F_ZZASC2() -> Result<()> {
        tspice(tspice::F_ZZASC2)
    }

    #[test]
    fn F_ZZASRYEL() -> Result<()> {
        tspice(tspice::F_ZZASRYEL)
    }

    #[test]
    fn F_ZZBDIN() -> Result<()> {
        tspice(tspice::F_ZZBDIN)
    }

    #[test]
    fn F_ZZBDKR() -> Result<()> {
        tspice(tspice::F_ZZBDKR)
    }

    #[test]
    fn F_ZZBDTN() -> Result<()> {
        tspice(tspice::F_ZZBDTN)
    }

    #[test]
    fn F_ZZBDTRN() -> Result<()> {
        tspice(tspice::F_ZZBDTRN)
    }

    #[test]
    fn F_ZZBODS() -> Result<()> {
        tspice(tspice::F_ZZBODS)
    }

    #[test]
    fn F_ZZBODS2C() -> Result<()> {
        tspice(tspice::F_ZZBODS2C)
    }

    #[test]
    fn F_ZZBODVCD() -> Result<()> {
        tspice(tspice::F_ZZBODVCD)
    }

    #[test]
    fn F_ZZBQUAD() -> Result<()> {
        tspice(tspice::F_ZZBQUAD)
    }

    #[test]
    fn F_ZZCKE06() -> Result<()> {
        tspice(tspice::F_ZZCKE06)
    }

    #[test]
    fn F_ZZCNQUAD() -> Result<()> {
        tspice(tspice::F_ZZCNQUAD)
    }

    #[test]
    fn F_ZZCORSXF() -> Result<()> {
        tspice(tspice::F_ZZCORSXF)
    }

    #[test]
    fn F_ZZCTR() -> Result<()> {
        tspice(tspice::F_ZZCTR)
    }

    #[test]
    fn F_ZZCXBRUT() -> Result<()> {
        tspice(tspice::F_ZZCXBRUT)
    }

    #[test]
    fn F_ZZDASGRC() -> Result<()> {
        tspice(tspice::F_ZZDASGRC)
    }

    #[test]
    fn F_ZZDDHNFC() -> Result<()> {
        tspice(tspice::F_ZZDDHNFC)
    }

    #[test]
    fn F_ZZDGDR() -> Result<()> {
        tspice(tspice::F_ZZDGDR)
    }

    #[test]
    fn F_ZZDGFR() -> Result<()> {
        tspice(tspice::F_ZZDGFR)
    }

    #[test]
    fn F_ZZDGSR() -> Result<()> {
        tspice(tspice::F_ZZDGSR)
    }

    #[test]
    fn F_ZZDM() -> Result<()> {
        tspice(tspice::F_ZZDM)
    }

    #[test]
    fn F_ZZDSKBSR() -> Result<()> {
        tspice(tspice::F_ZZDSKBSR)
    }

    #[test]
    fn F_ZZDSKBUX() -> Result<()> {
        tspice(tspice::F_ZZDSKBUX)
    }

    #[test]
    fn F_ZZDSKSEL() -> Result<()> {
        tspice(tspice::F_ZZDSKSEL)
    }

    #[test]
    fn F_ZZDSKSGR() -> Result<()> {
        tspice(tspice::F_ZZDSKSGR)
    }

    #[test]
    fn F_ZZDSKSGX() -> Result<()> {
        tspice(tspice::F_ZZDSKSGX)
    }

    #[test]
    fn F_ZZDSKSPH() -> Result<()> {
        tspice(tspice::F_ZZDSKSPH)
    }

    #[test]
    fn F_ZZEDTMPT() -> Result<()> {
        tspice(tspice::F_ZZEDTMPT)
    }

    #[test]
    fn F_ZZELLBDS() -> Result<()> {
        tspice(tspice::F_ZZELLBDS)
    }

    #[test]
    fn F_ZZELNAXX() -> Result<()> {
        tspice(tspice::F_ZZELNAXX)
    }

    #[test]
    fn F_ZZELVUPY0() -> Result<()> {
        tspice(tspice::F_ZZELVUPY0)
    }

    #[test]
    fn F_ZZFDAT() -> Result<()> {
        tspice(tspice::F_ZZFDAT)
    }

    #[test]
    fn F_ZZFGEO() -> Result<()> {
        tspice(tspice::F_ZZFGEO)
    }

    #[test]
    fn F_ZZFOVAXI() -> Result<()> {
        tspice(tspice::F_ZZFOVAXI)
    }

    #[test]
    fn F_ZZGETELM() -> Result<()> {
        tspice(tspice::F_ZZGETELM)
    }

    #[test]
    fn F_ZZGFCOQ() -> Result<()> {
        tspice(tspice::F_ZZGFCOQ)
    }

    #[test]
    fn F_ZZGFCOST() -> Result<()> {
        tspice(tspice::F_ZZGFCOST)
    }

    #[test]
    fn F_ZZGFCOU() -> Result<()> {
        tspice(tspice::F_ZZGFCOU)
    }

    #[test]
    fn F_ZZGFCPRX() -> Result<()> {
        tspice(tspice::F_ZZGFCPRX)
    }

    #[test]
    fn F_ZZGFCSLV() -> Result<()> {
        tspice(tspice::F_ZZGFCSLV)
    }

    #[test]
    fn F_ZZGFDIQ() -> Result<()> {
        tspice(tspice::F_ZZGFDIQ)
    }

    #[test]
    fn F_ZZGFDIU() -> Result<()> {
        tspice(tspice::F_ZZGFDIU)
    }

    #[test]
    fn F_ZZGFFVU() -> Result<()> {
        tspice(tspice::F_ZZGFFVU)
    }

    #[test]
    fn F_ZZGFILU() -> Result<()> {
        tspice(tspice::F_ZZGFILU)
    }

    #[test]
    fn F_ZZGFLNG1() -> Result<()> {
        tspice(tspice::F_ZZGFLNG1)
    }

    #[test]
    fn F_ZZGFLNG2() -> Result<()> {
        tspice(tspice::F_ZZGFLNG2)
    }

    #[test]
    fn F_ZZGFLNG3() -> Result<()> {
        tspice(tspice::F_ZZGFLNG3)
    }

    #[test]
    fn F_ZZGFOCU() -> Result<()> {
        tspice(tspice::F_ZZGFOCU)
    }

    #[test]
    fn F_ZZGFPAU() -> Result<()> {
        tspice(tspice::F_ZZGFPAU)
    }

    #[test]
    fn F_ZZGFREL() -> Result<()> {
        tspice(tspice::F_ZZGFREL)
    }

    #[test]
    fn F_ZZGFRELX() -> Result<()> {
        tspice(tspice::F_ZZGFRELX)
    }

    #[test]
    fn F_ZZGFRPWK() -> Result<()> {
        tspice(tspice::F_ZZGFRPWK)
    }

    #[test]
    fn F_ZZGFRRU() -> Result<()> {
        tspice(tspice::F_ZZGFRRU)
    }

    #[test]
    fn F_ZZGFSOLV() -> Result<()> {
        tspice(tspice::F_ZZGFSOLV)
    }

    #[test]
    fn F_ZZGFSOLVX() -> Result<()> {
        tspice(tspice::F_ZZGFSOLVX)
    }

    #[test]
    fn F_ZZGFSPQ() -> Result<()> {
        tspice(tspice::F_ZZGFSPQ)
    }

    #[test]
    fn F_ZZGFSPU() -> Result<()> {
        tspice(tspice::F_ZZGFSPU)
    }

    #[test]
    fn F_ZZGFSSIN() -> Result<()> {
        tspice(tspice::F_ZZGFSSIN)
    }

    #[test]
    fn F_ZZGFSSOB() -> Result<()> {
        tspice(tspice::F_ZZGFSSOB)
    }

    #[test]
    fn F_ZZGFWSTS() -> Result<()> {
        tspice(tspice::F_ZZGFWSTS)
    }

    #[test]
    fn F_ZZHSC() -> Result<()> {
        tspice(tspice::F_ZZHSC)
    }

    #[test]
    fn F_ZZHSI() -> Result<()> {
        tspice(tspice::F_ZZHSI)
    }

    #[test]
    fn F_ZZHULLAX() -> Result<()> {
        tspice(tspice::F_ZZHULLAX)
    }

    #[test]
    fn F_ZZILUSTA() -> Result<()> {
        tspice(tspice::F_ZZILUSTA)
    }

    #[test]
    fn F_ZZINLAT() -> Result<()> {
        tspice(tspice::F_ZZINLAT)
    }

    #[test]
    fn F_ZZINLAT0() -> Result<()> {
        tspice(tspice::F_ZZINLAT0)
    }

    #[test]
    fn F_ZZINPDT() -> Result<()> {
        tspice(tspice::F_ZZINPDT)
    }

    #[test]
    fn F_ZZINPDT0() -> Result<()> {
        tspice(tspice::F_ZZINPDT0)
    }

    #[test]
    fn F_ZZINREC() -> Result<()> {
        tspice(tspice::F_ZZINREC)
    }

    #[test]
    fn F_ZZINVELT() -> Result<()> {
        tspice(tspice::F_ZZINVELT)
    }

    #[test]
    fn F_ZZLATBOX() -> Result<()> {
        tspice(tspice::F_ZZLATBOX)
    }

    #[test]
    fn F_ZZMKSPIN() -> Result<()> {
        tspice(tspice::F_ZZMKSPIN)
    }

    #[test]
    fn F_ZZMSXF() -> Result<()> {
        tspice(tspice::F_ZZMSXF)
    }

    #[test]
    fn F_ZZNAMFRM() -> Result<()> {
        tspice(tspice::F_ZZNAMFRM)
    }

    #[test]
    fn F_ZZNOFCON() -> Result<()> {
        tspice(tspice::F_ZZNOFCON)
    }

    #[test]
    fn F_ZZNRMLON() -> Result<()> {
        tspice(tspice::F_ZZNRMLON)
    }

    #[test]
    fn F_ZZOCCED() -> Result<()> {
        tspice(tspice::F_ZZOCCED)
    }

    #[test]
    fn F_ZZOCCED2() -> Result<()> {
        tspice(tspice::F_ZZOCCED2)
    }

    #[test]
    fn F_ZZOCCED3() -> Result<()> {
        tspice(tspice::F_ZZOCCED3)
    }

    #[test]
    fn F_ZZPDCMPL() -> Result<()> {
        tspice(tspice::F_ZZPDCMPL)
    }

    #[test]
    fn F_ZZPDPLTC() -> Result<()> {
        tspice(tspice::F_ZZPDPLTC)
    }

    #[test]
    fn F_ZZPDTBOX() -> Result<()> {
        tspice(tspice::F_ZZPDTBOX)
    }

    #[test]
    fn F_ZZPLAT() -> Result<()> {
        tspice(tspice::F_ZZPLAT)
    }

    #[test]
    fn F_ZZPTPL02() -> Result<()> {
        tspice(tspice::F_ZZPTPL02)
    }

    #[test]
    fn F_ZZRAYBOX() -> Result<()> {
        tspice(tspice::F_ZZRAYBOX)
    }

    #[test]
    fn F_ZZRECBOX() -> Result<()> {
        tspice(tspice::F_ZZRECBOX)
    }

    #[test]
    fn F_ZZRTNMAT() -> Result<()> {
        tspice(tspice::F_ZZRTNMAT)
    }

    #[test]
    fn F_ZZRXR() -> Result<()> {
        tspice(tspice::F_ZZRXR)
    }

    #[test]
    fn F_ZZRYTELT() -> Result<()> {
        tspice(tspice::F_ZZRYTELT)
    }

    #[test]
    fn F_ZZRYTLAT() -> Result<()> {
        tspice(tspice::F_ZZRYTLAT)
    }

    #[test]
    fn F_ZZRYTPDT() -> Result<()> {
        tspice(tspice::F_ZZRYTPDT)
    }

    #[test]
    fn F_ZZRYTREC() -> Result<()> {
        tspice(tspice::F_ZZRYTREC)
    }

    #[test]
    fn F_ZZSC01() -> Result<()> {
        tspice(tspice::F_ZZSC01)
    }

    #[test]
    fn F_ZZSEGBOX() -> Result<()> {
        tspice(tspice::F_ZZSEGBOX)
    }

    #[test]
    fn F_ZZSEPQ() -> Result<()> {
        tspice(tspice::F_ZZSEPQ)
    }

    #[test]
    fn F_ZZSFXCOR() -> Result<()> {
        tspice(tspice::F_ZZSFXCOR)
    }

    #[test]
    fn F_ZZSGLATX() -> Result<()> {
        tspice(tspice::F_ZZSGLATX)
    }

    #[test]
    fn F_ZZSINUTL() -> Result<()> {
        tspice(tspice::F_ZZSINUTL)
    }

    #[test]
    fn F_ZZSPKFUN() -> Result<()> {
        tspice(tspice::F_ZZSPKFUN)
    }

    #[test]
    fn F_ZZSTELAB() -> Result<()> {
        tspice(tspice::F_ZZSTELAB)
    }

    #[test]
    fn F_ZZSWFFET() -> Result<()> {
        tspice(tspice::F_ZZSWFFET)
    }

    #[test]
    fn F_ZZSWFXFM() -> Result<()> {
        tspice(tspice::F_ZZSWFXFM)
    }

    #[test]
    fn F_ZZTANSLV() -> Result<()> {
        tspice(tspice::F_ZZTANSLV)
    }

    #[test]
    fn F_ZZTANUTL() -> Result<()> {
        tspice(tspice::F_ZZTANUTL)
    }

    #[test]
    fn F_ZZTIME() -> Result<()> {
        tspice(tspice::F_ZZTIME)
    }

    #[test]
    fn F_ZZUTC() -> Result<()> {
        tspice(tspice::F_ZZUTC)
    }

    #[test]
    fn F_ZZVALCOR() -> Result<()> {
        tspice(tspice::F_ZZVALCOR)
    }

    #[test]
    fn F_ZZVRTPLT() -> Result<()> {
        tspice(tspice::F_ZZVRTPLT)
    }

    #[test]
    fn F_ZZWIND2D() -> Result<()> {
        tspice(tspice::F_ZZWIND2D)
    }
}
