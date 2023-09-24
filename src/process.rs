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

use std::{ffi::OsStr, process::Command};

/// Extensions to [`Command`]
pub trait CommandExt {
    fn log_command(&mut self) -> &mut Self;
    fn execute(&mut self) -> anyhow::Result<&mut Self>;

    /// Adds an argument to pass to the program if `pred` is `true`.
    fn arg_if(&mut self, pred: bool, arg: impl AsRef<OsStr>) -> &mut Self;

    /// Adds arguments to pass to the program if `pred` is `true`.
    fn args_if(
        &mut self,
        pred: bool,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> &mut Self;
}

impl CommandExt for Command {
    fn log_command(&mut self) -> &mut Self {
        println!(
            "% {} {}",
            self.get_program().to_str().unwrap(),
            self.get_args()
                .collect::<Vec<&OsStr>>()
                .join(OsStr::new(" "))
                .to_str()
                .unwrap(),
        );
        self
    }

    fn execute(&mut self) -> anyhow::Result<&mut Self> {
        self.log_command().spawn()?.wait()?.exit_ok()?;
        Ok(self)
    }

    #[inline(always)]
    fn arg_if(&mut self, pred: bool, arg: impl AsRef<OsStr>) -> &mut Self {
        if pred {
            self.arg(arg);
        }
        self
    }

    #[inline(always)]
    fn args_if(
        &mut self,
        pred: bool,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> &mut Self {
        if pred {
            self.args(args);
        }
        self
    }
}
