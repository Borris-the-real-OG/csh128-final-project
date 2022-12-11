use core::panic;
use std::collections::HashMap;
use std::{env, fs};
use std::io::{self, Read, prelude::*};
use std::fs::File;
use std::path::{PathBuf, Path};
use std::process::{Command, Output};

use tectonic_bundles::cache::{CachingBundle, CacheBackend};
use tectonic_bundles::itar::IndexedTarBackend;
use tectonic_bundles::{cache::Cache, Bundle};
use tectonic_geturl::{null::NullBackend, GetUrlBackend};
use tectonic_io_base::digest::DigestData;
use tectonic_status_base::{StatusBackend, plain::PlainStatusBackend};
use tectonic_io_base::IoProvider;

/* NOTES
 * https://relay.fullyjustified.net/default_bundle_v{}.tar is the url format
 */
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn make_latex_document() {
    let latex = r#"
        \documentclass{article}
    \begin{document}
    Hello, world!
        \end{document}
    "#;

    //let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
    //println!("Output PDF size is {} bytes", pdf_data.len());
}

#[derive(Clone, Copy, Debug)]
pub struct bCachedFileInfo {
    /// The length of the file in bytes.
    ///
    /// This field isn't currently used, but seems handy to keep around.
    _length: u64,

    /// The digest of the file contents.
    ///
    /// This digest is used to locate the cached data on disk.
    digest: DigestData,
}

#[derive(Debug)]
pub struct NotActuallyCachingBundle<CB: CacheBackend> {
    /// The URL specifying where to start looking for the bundle data.
    ///
    /// The caching layer maintains two URLs: the "start" URL and the "resolved"
    /// URL. The goal here is to be able to store a single URL for fetching
    /// data, but maintain the capability to update the bundle data behind that
    /// URL. Requests to the start URL may get redirected (one or more times)
    /// until eventually we arrive at the "resolved" URL. While the redirection
    /// of the start URL might change, the contents of a resolved URL should
    /// never change once published.
    pub start_url: String,

    /// The "resolved" URL for the backing data.
    ///
    /// The bundle data located at this URL should never change.
    pub resolved_url: String,

    /// The cached value of the backend’s content digest.
    ///
    /// This is stored in a file at [`Self::digest_path`]. This value may be
    /// inaccurate, if the backing bundle has been updated (or if the cache is
    /// corrupt, etc.) and we haven't yet synchronized with the backend and
    /// discovered that fact.
    pub cached_digest: DigestData,

    /// Information about all of the files that have been cached locally.
    ///
    /// This maps filenames to summary information that can then be used to
    /// retrieve file data from [`Self::data_base`]. The contents are loaded
    /// from the manifest file if the cache is non-empty.
    pub contents: HashMap<String, bCachedFileInfo>,

    /// Information about all of the files known to the backend.
    ///
    /// This maps filenames to [`CacheBackend::FileInfo`] data that can be used
    /// to retrieve a file from the backend if needed.
    pub index: HashMap<String, CB::FileInfo>,

    /// If true, only use cached files -- never connect to the backend.
    ///
    /// This option can be useful if we are operating disconnected from the
    /// network (e.g., on an airplane). If you add a new figure to your
    /// document, the engine will inquire about several related files that it
    /// thinks might exist. Without this option, such an inquiry might require
    /// Tectonic to hit the network, when the user knows for sure that the
    /// bundle is not going to contain these files.
    only_cached: bool,

    /// The connection to the cache backend, maybe.
    ///
    /// This field will be `None` if there are locally cached data present and
    /// there has not yet been a need to connect to the backend. If it becomes
    /// necessary to "pull" and/or download a new file from the backend, this
    /// value will become `Some` — it represents something like an open network
    /// connection.
    backend: Option<CB>,

    /// The path to a file containing a cached copy of the backend's content
    /// digest.
    ///
    /// This file path is based on [`Self::start_url`].
    digest_path: PathBuf,

    /// A directory where we will save [`Self::resolved_url`].
    ///
    /// We need to cache `resolved_url` to enable the "quick check" backend
    /// reconnection path. The actual cache file path is based on the backend’s
    /// content digest.
    resolved_base: PathBuf,

    /// A directory where we will save the cache manifest.
    ///
    /// The manifest file contains information about the files that have
    /// actually been fetched from the backend and saved locally. The actual
    /// manifest file path is based on the backend’s content digest.
    manifest_path: PathBuf,

    /// A directory where we will save cached file data.
    ///
    /// This directory contains the actual cached file contents, in a directory
    /// structured based on the digest of each file’s content.
    data_base: PathBuf,
}

#[cfg(debug_assertions)]
fn view_cache_digests(cb: CachingBundle<IndexedTarBackend>) {
    let nacb: NotActuallyCachingBundle<IndexedTarBackend> = unsafe {
        std::mem::transmute(cb)
    };
    
    for (k, v) in nacb.contents {
        println!("{:?} -> {:?}", k, v);
    }
}

fn fetch_cache_digest(cb: CachingBundle<IndexedTarBackend>, tect_dir: &PathBuf) -> Vec<(String, String)> {
    // this is the one piece of unsafe code in the program, because it relies on rustc to give both
    // structs the same layout, which it isn't guaranteed to do if they're not marked `repr(C)`.
    let nacb: NotActuallyCachingBundle<IndexedTarBackend> = unsafe {
        std::mem::transmute(cb)
    };
    
    nacb.contents.into_iter().map(|(k, v)| {
        (k, v.digest.create_two_part_path(&tect_dir).expect("Error: unable to fetch digest file").file_name().unwrap().to_os_string().into_string().unwrap())
    }).collect()
}

fn main2() -> io::Result<()> {
    let cache = Cache::get_user_default().unwrap();
    println!("Cache info: {:?}", cache.root());
    let mut backend = PlainStatusBackend::default();
    // version number only checks if it's above 32. idk man /shrug
    let mut files = tectonic_bundles::get_fallback_bundle(33, true, &mut backend).expect("error getting fallback bundle");
    // ok, I'm pretty sure we work our way from fallback
    // WE HAVE THE STRING REPRESENTING THE LATEX ASSET (i.e. "lmroman-10-regular")
    //cache.open(tectonic_bundles::get_fallback_bundle_url(tectonic_engine_xetex::FORMAT_SERIAL), true, );
    // ok, I think I get it. 'status' is like stderr, so we can have multiple error streams
    view_cache_digests(files);
    /*

    let chonky_boi = files.all_files(&mut backend).expect("can't get the files of the bundle");
    println!("{} files in the chonky bundle", &chonky_boi.len());

    let mut lmroman = match (&mut files).input_open_name("lmroman10-regular.otf", &mut backend) {
        tectonic_io_base::OpenResult::Ok(s) => s,
        tectonic_io_base::OpenResult::NotAvailable => panic!("file not available"),
        tectonic_io_base::OpenResult::Err(d) => panic!("{}", d.to_string())
    };

    let mut fout = File::create("wow.otf")?;
    let barray = &lmroman.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    fout.write_all(barray)?;
    
    // ok, here's the question
    // it /might/ be faster to just lookup the hash in manifests, and get the corresponding file.
    // The thing is, this version is technically more cross-platform
    */
    Ok(())
}

fn tex_setup() {
    // "Tectonic" dir
    let cache_dir = Cache::get_user_default().unwrap();
    let cache = cache_dir.root();
    let mut lsr = cache.to_path_buf(); lsr.push("ls-R");
    let mut aliases = cache.to_path_buf(); aliases.push("aliases");
    let mut config = cache.to_path_buf(); config.push("texmf.cnf");

    // should update if lsr is old; check file age?
    if !lsr.exists() {
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && ls -LAR ./ > ls-R", lsr.to_str().unwrap()))
            .output()
            .expect("Error: cannot create ls-R database");
    }
    if !aliases.exists() {
        let mut backend = PlainStatusBackend::default();
        let files = tectonic_bundles::get_fallback_bundle(33, true, &mut backend).expect("error getting fallback bundle");

        let mut f = File::create(aliases).expect("Error: unable to create aliases file");
        fetch_cache_digest(files, &cache.to_path_buf()).iter().for_each(|(dig, name)| {
            writeln!(f, "{}, {}", dig, name).expect("Error: cannot write to aliases file");
        });
    }

    if !config.exists() {
        let data = format!(r#"
            TEXINPUTS={d}//
            TEXMFDBS={d}
            TEXMF={d}
            TEXFONTMAPS={d}//
            TEXFONTS={d}//
            "#, d=cache.to_str().unwrap());
        fs::write(config, data).expect("Error: couldn't write to texmf.cnf");
    }
}

// I would have very much liked to have done this using the tectonic bundle instead of calling the
// executable, but unfortunately they make it very difficult to get an XDV using library functions.
fn compile_tex(f: &Path) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(format!("tectonic -X compile {} --outdir {} --outfmt xdv", f.to_str().expect("Error: file name invalid"), env::temp_dir().to_str().expect("Error: /tmp weirdness")))
        .output()
        .expect("Error: tectonic failed to run")
}

fn create_svg(f: PathBuf, tect_f: PathBuf) -> Output {
    Command::new("sh")
        .env("TEXMFCNF", tect_f.to_str().unwrap())
        .arg("-c")
        .arg(format!("dvisvgm {} --output {} -n", f.to_str().expect("Error: file name invalid"), env::current_dir().unwrap().to_str().expect("Error: PWD weirdness")))
        .output()
        .expect("Error: dvisvgm failed to run")
}


fn main() -> io::Result<()> {
    // assume dvisvgm and Tectonic are on the machine, enforce using build script
    let file = PathBuf::from(env::args().nth(2).expect("Error: please input your TeX filename"));
    if !file.exists() {
        panic!("Error: {} not found.", file.to_str().unwrap());
    }

    let cache = Cache::get_user_default().unwrap();
    tex_setup();

    let _tex_output = compile_tex(&file);

    let mut xdv_path = env::temp_dir();
    xdv_path.push(file.file_stem().unwrap());
    xdv_path.push(".xdv");

    let _svg_output = create_svg(xdv_path, cache.root().to_path_buf());

    println!("Successfully wrote {}.svg", file.file_stem().unwrap().to_str().unwrap());

    Ok(())
}
