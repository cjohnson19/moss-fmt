mod path_verifier;

extern crate clap;
use crate::path_verifier::PathVerifier;
use clap::{App, Arg};
use std::ffi::OsStr;
use std::fs;
use std::fs::{DirEntry, File};
use std::path::Path;
use zip::ZipArchive;

fn path_is_zip(entry: &DirEntry) -> Result<bool, std::io::Error> {
    let path = entry.path();
    Ok(path.is_file() && path.extension().unwrap_or(OsStr::new("")).eq("zip"))
}

fn collect_zips_from_dir(dir_name: &str) -> Vec<(String, ZipArchive<File>)> {
    let paths = fs::read_dir(dir_name).unwrap();
    let mut zips = Vec::new();
    for path in paths {
        let entry = path.unwrap();
        if path_is_zip(&entry).unwrap_or(false) {
            let file_name = entry
                .file_name()
                .to_str()
                .unwrap()
                .trim_end_matches(".zip")
                .to_string();
            let file = File::open(entry.path()).unwrap();
            let zip = zip::ZipArchive::new(file).unwrap();
            zips.push((file_name, zip));
        }
    }
    return zips;
}

fn extract_files(dir_name: &str, verifier: PathVerifier, output_dir: &str, verbose: bool) {
    let zip_archives = collect_zips_from_dir(dir_name);
    let base_output_path = Path::new(output_dir);
    for (zip_name, mut zip_archive) in zip_archives {
        for i in 0..zip_archive.len() {
            let mut search_file = zip_archive.by_index(i).unwrap();
            if !verifier.verify(&search_file.enclosed_name().unwrap()) {
                continue;
            }
            let search_file_name = search_file
                .enclosed_name()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            if verbose {
                println!(
                    "Found matching file {} in {}.zip",
                    search_file_name, zip_name
                );
            }
            let output_file_name = format!("{}-{}", zip_name, search_file_name);
            let output_file_path = base_output_path.join(output_file_name.clone());
            let mut output_file = File::create(output_file_path).unwrap();
            if verbose {
                println!("Copying file {} to {}", search_file_name, output_file_name);
            }
            std::io::copy(&mut search_file, &mut output_file).unwrap();
            if verbose {
                println!("Successfully copied file to {}\n", output_file_name);
            }
        }
    }
}

fn print_info(dir_name: &str, search_files: &Vec<&str>) {
    let n = search_files.len();
    if n == 1 {
        println!(
            "Searching in directory {} for {}...",
            dir_name,
            search_files.get(0).unwrap()
        );
    } else {
        println!("Searching in directory {} for {} files...", dir_name, n);
        for i in 0..n {
            println!("  {}: {}", i + 1, search_files.get(i).unwrap());
        }
    }
}

fn main() {
    let matches = App::new("Moss File Formatter")
        .version("1.0")
        .author("Chase Johnson <joh13266@umn.edu>")
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .value_name("dir")
                .help("The directory with all submissions (as zip files)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("output")
                .help("The directory to store all decompressed files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("filename")
                .value_name("file")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .help("The files to extract from the submission zips"),
        )
        .arg(
            Arg::with_name("filter-dir")
                .long("filter-dir")
                .value_name("filter-dir")
                .multiple(true)
                .takes_value(true)
                .help("Folder to exclude from searching"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .help("Output more information about operations being performed"),
        )
        .get_matches();

    let mut verifier = PathVerifier::default();
    for file_name in matches.values_of("file").unwrap() {
        verifier = verifier.add_search_file(file_name);
    }
    if matches.is_present("filter-dir") {
        for dir_name in matches.values_of("filter-dir").unwrap() {
            verifier = verifier.add_restricted_folder(dir_name);
        }
    }
    let dir_name = matches.value_of("dir").unwrap();
    let output_dir = matches.value_of("output").unwrap_or("./");
    let verbose = matches.is_present("verbosity");
    if verbose {
        let file_names: Vec<&str> = matches.values_of("file").unwrap().collect::<Vec<&str>>();
        print_info(dir_name, &file_names);
    }
    extract_files(dir_name, verifier, output_dir, verbose);
}
