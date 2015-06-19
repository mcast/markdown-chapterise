use std::path::{Path, PathBuf};
use std::fs::{ File, rename };
use std::io::{Result, Error, ErrorKind};
use std::cell::RefCell;

// #[derive(Debug)] // XXX: not for File
pub struct MarkdownOut<'a> {
    /// Directory into which we write
    outdir: &'a Path,
    /// Serial number prefix for output files
    filenum: u32,
    /// Final name for the File after closing
    outpath: PathBuf,
    /// Name at which File is created
    tmppath: PathBuf,
    /// The current output
    outfh: RefCell<Option<File>>, // XXX: mutability needed to close File, better way?
}
// XXX: some pattern or crate, for (final Path, tmp Path, File being written; move on close) ?

impl<'a> MarkdownOut<'a> {
    pub fn new(outdir: &'a Path, leafname: &str) -> MarkdownOut<'a> {
        let outpath = mkout(outdir, 0, leafname, false);
        let tmppath = mkout(outdir, 0, leafname, true);
        let f =  File::create(outpath.as_path()).unwrap();
        let new = MarkdownOut {
            outdir: outdir,
            filenum: 0,
            outpath: outpath,
            tmppath: tmppath,
            outfh: RefCell::new(Some(f)),
        };
        new
    }
    pub fn close(&self) -> Result<()> {
        let mut fhput = self.outfh.borrow_mut();
        match *fhput {
            Some(_) => {
                try!(rename(self.tmppath.as_path(), self.outpath.as_path()));
                *fhput = None;
                Ok(())
            },
            None => {
                let msg = "File was already closed and renamed";
                Err( Error::new(ErrorKind::AlreadyExists, msg) )
            },
        }
    }
}


fn mkout(outdir: &Path, filenum: u32, leafname: &str, is_tmp: bool) -> PathBuf {
    let mut outpath = outdir.to_path_buf();
    let sfx = if is_tmp {
        "+" // XXX: insecure tmpfile.  tmpnam is unstable & libc; rand is elsewhere
    } else {
        ""
    };
    outpath.push(format!("{:02}_{}{}", filenum, leafname, sfx));
    outpath
}
