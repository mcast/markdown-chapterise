use std::path::{Path, PathBuf};
use std::fs::{ File, rename };
use std::io::Write;
use std::io::{Result, Error, ErrorKind};
use std::cell::RefCell;
use std::ops::DerefMut;


// #[derive(Debug)] // XXX: not for File
pub struct MarkdownOut {
    /// Directory into which we write
    outdir: PathBuf,
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

impl MarkdownOut {
    pub fn new(outdir: &Path, leafname: &str) -> MarkdownOut {
        _new(outdir.to_path_buf(), 0, leafname)
    }
    pub fn append(&self, data: String) -> Result<()> {
        let mut fhput = self.outfh.borrow_mut();
        match *fhput {
            Some(ref f) => {
                println!("{}", data); // XXX: bogus.  have problems writing a `mut f`
                // f.write_all(data.as_bytes()),
                Ok(())
            },
            None => self.gone(),
        }
    }
    pub fn next(&self, leafname: &str) -> MarkdownOut {
        let n = self.filenum + 1;
        _new(self.outdir, n, leafname)
    }
    pub fn close(&self) -> Result<()> {
        let mut fhput = self.outfh.borrow_mut();
        match *fhput {
            Some(_) => {
                try!(rename(self.tmppath.as_path(), self.outpath.as_path()));
                *fhput = None;
                Ok(())
            },
            None => self.gone(),
        }
    }
    fn gone(&self) -> Result<()> {
        let msg = "File was already closed and renamed";
        Err( Error::new(ErrorKind::AlreadyExists, msg) )
    }
}


fn _new(outdir: PathBuf, filenum: u32, leafname: &str) -> MarkdownOut {
    let outpath = mkout(&outdir, filenum, leafname, false);
    let tmppath = mkout(&outdir, filenum, leafname, true);
    let f =  File::create(outpath.as_path()).unwrap();
    let new = MarkdownOut {
        outdir: outdir,
        filenum: filenum,
        outpath: outpath,
        tmppath: tmppath,
        outfh: RefCell::new(Some(f)),
    };
    new
}

fn mkout(outdir: &PathBuf, filenum: u32, leafname: &str, is_tmp: bool) -> PathBuf {
    let mut outpath = outdir.clone();
    let sfx = if is_tmp {
        "+" // XXX: insecure tmpfile.  tmpnam is unstable & libc; rand is elsewhere
    } else {
        ""
    };
    outpath.push(format!("{:02}_{}{}", filenum, leafname, sfx));
    outpath
}
