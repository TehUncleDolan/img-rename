//! Page renamer.

// Lints {{{

#![deny(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    rustdoc::all,
    missing_crate_level_docs,
    missing_docs,
    unreachable_pub,
    unsafe_code,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    warnings,
    clippy::all,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::panic,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::verbose_file_reads,
    clippy::wrong_pub_self_convention,
    clippy::dbg_macro,
    clippy::let_underscore_must_use,
    clippy::todo,
    clippy::unwrap_used,
    clippy::use_debug
)]
#![allow(
    // The 90’s called and wanted their charset back :p
    clippy::non_ascii_literal,
    // For Kuchiki imports.
    clippy::wildcard_imports,
    // It's easily outdated and doesn't bring that much value.
    clippy::missing_errors_doc,
    // That's OK for this script.
    clippy::expect_used,
    clippy::print_stdout,
)]

// }}}

use anyhow::{anyhow, Context, Result};
use std::{env, ffi::OsStr, fs, path::Path};

fn main() -> Result<()> {
    let wd = env::current_dir().context("failed to get current directory")?;
    // Skip the binary name.
    for book in env::args().skip(1) {
        println!("Renaming page in {}...", book);

        env::set_current_dir(&book).with_context(|| format!("failed to move into {}", book))?;

        rename_pages().with_context(|| format!("failed to rename pages in {}", book))?;

        strip_bak_suffix().with_context(|| format!("failed to strip back suffix in {}", book))?;

        env::set_current_dir(&wd).context("failed to go back to the working directory")?;
    }

    Ok(())
}

/// Compute the right name for the page and rename it with a `.bak` suffix.
///
/// The `.bak` suffix avoir accidental overwrite on name conflicts.
fn rename_pages() -> Result<()> {
    let mut entries = fs::read_dir(".")
        .context("failed to list pages")?
        .collect::<Result<Vec<_>, _>>()
        .context("cannot access page")?;
    entries.sort_by_key(std::fs::DirEntry::path);

    let mut i = get_page_offset();
    for entry in entries {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        match get_extension_from_filename(&path) {
            Ok("jpg") | Ok("png") | Ok("webp") | Ok("jxl") => (),
            _ => continue,
        };

        match imagesize::size(&path) {
            Ok(dim) => {
                let ext = get_extension_from_filename(&path).expect("valid extension");

                let new_name = if dim.height > dim.width {
                    // SP
                    i += 1;
                    format!("{:03}.{}.bak", i - 1, ext)
                } else {
                    // DP
                    i += 2;
                    format!("{:03}-{:03}.{}.bak", i - 2, i - 1, ext)
                };

                rename(&path, &new_name)?;
            }
            Err(err) => eprintln!("Skip {}: {}", path.display(), err),
        }
    }

    Ok(())
}

fn strip_bak_suffix() -> Result<()> {
    for path in fs::read_dir(".").context("failed to list pages")? {
        let path = path.context("cannot access page")?.path();

        if !path.is_file() {
            continue;
        }
        match get_extension_from_filename(&path) {
            Ok("bak") => (),
            _ => continue,
        };

        let new_name = path.to_str().expect("valid UTF-8").trim_end_matches(".bak");
        rename(&path, &new_name)?;
    }

    Ok(())
}

fn get_page_offset() -> usize {
    const DEFAULT_PAGE_OFFSET: usize = 1;

    env::var("PAGE_OFFSET")
        .map(|val| val.parse::<usize>().unwrap_or(DEFAULT_PAGE_OFFSET))
        .unwrap_or(DEFAULT_PAGE_OFFSET)
}

fn get_extension_from_filename(filename: &Path) -> Result<&str> {
    filename
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| anyhow!("cannot get file extension for {}", filename.display()))
}

fn rename(from: &Path, to: &str) -> Result<()> {
    fs::rename(&from, &to).with_context(|| format!("failed to rename {} to {}", from.display(), to))
}
