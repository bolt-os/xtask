/*
 * Copyright (c) 2023 xvanc and contributors
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its contributors
 *    may be used to endorse or promote products derived from this software without
 *    specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

use std::{borrow::Cow, io, path::Path};

/// Copy files
///
/// If `to` is a directory, a new file is created with the same name as `from`.
///
/// # Errors
///
/// See [`std::fs::copy()`](std::fs::copy#errors).
///
/// # Panics
///
/// If `to` is a directory and [`from.file_name()`](method@Path::file_name) returns `None`.
#[inline]
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    fn _copy(from: &Path, to: &Path) -> io::Result<()> {
        println!(
            "% copy: {} -> {}{}",
            from.display(),
            to.display(),
            if to.is_dir() { "/" } else { "" },
        );
        if to.is_dir() {
            let mut to = to.to_owned();
            to.push(from.file_name().unwrap());
            std::fs::copy(from, to)?;
        } else {
            std::fs::copy(from, to)?;
        }
        Ok(())
    }
    _copy(from.as_ref(), to.as_ref())
}

/// Recursively create a directory and its parents
///
/// If `path` is not a directory, the parent is used instead.
///
/// # Errors
///
/// See [`std::fs::create_dir_all()`](std::fs::create_dir_all#errors).
///
/// # Panics
///
/// If `path` is not a directory and [`.parent()`](method@Path::parent) returns `None`.
#[inline]
pub fn make_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fn _make_dir(path: &Path) -> io::Result<()> {
        println!(
            "% make_dir: {}{}",
            path.display(),
            if path.is_dir() { "/" } else { "" },
        );
        std::fs::create_dir_all(path)
    }
    _make_dir(path.as_ref())
}

#[inline]
pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(
    original: P,
    link: Q,
    force: bool,
) -> io::Result<()> {
    fn _symlink(original: &Path, link: &Path, force: bool) -> io::Result<()> {
        println!(
            "% symlink: {} -> {}{}",
            original.display(),
            link.display(),
            if link.is_dir() { "/" } else { "" },
        );

        let link = if link.is_dir() {
            let mut path = link.to_path_buf();
            path.push(link.file_name().unwrap());
            Cow::Owned(path)
        } else {
            Cow::Borrowed(link)
        };

        if force && link.exists() {
            assert!(link.is_symlink());
            std::fs::remove_file(&link)?;
        }

        #[cfg(unix)]
        std::os::unix::fs::symlink(original, link)?;

        #[cfg(windows)]
        if original.is_dir() {
            std::os::windows::fs::symlink_dir(original, link)?;
        } else {
            std::os::windows::fs::symlink_file(original, link)?;
        }

        #[cfg(not(any(unix, windows)))]
        compile_error!("unsupported host platform");

        Ok(())
    }
    _symlink(original.as_ref(), link.as_ref(), force)
}
