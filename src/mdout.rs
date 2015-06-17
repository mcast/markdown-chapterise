use std::path::{Path, PathBuf};
use std::fs::File;

// #[derive(Debug)] // XXX: not for File
pub struct MarkdownOut<'a> {
    outdir: &'a Path,
    filenum: u32,
    outpath: PathBuf,
    outfh: File,
}

impl<'a> MarkdownOut<'a> {
    pub fn new(outdir: &'a Path, leafname: &str) -> MarkdownOut<'a> {
        let mut outpath = outdir.to_path_buf();
        outpath.push(leafname);
        let f =  File::create(outpath.as_path()).unwrap();
        let new = MarkdownOut { outdir: outdir, filenum: 0, outpath: outpath, outfh: f };
        new
    }
}

