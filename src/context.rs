// Copyright 2016 Joe Wilm
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::io::{self, Write};
use terminfo::{capability as cap, Database};

use rand::{self, Rng};

use result::Result;

pub struct Context<'a, W: Write + 'a> {
    pub out: &'a mut W,
    pub db: &'a Database,
    pub buf: Vec<u8>,
    pub rng: rand::ThreadRng,
}

impl<'a, W: Write + 'a> Context<'a, W> {
    pub fn smcup(&mut self) -> Result<usize> {
        let smcup = expand!(self.db.get::<cap::EnterCaMode>().unwrap().as_ref())?;
        self.write_all(&smcup)?;
        Ok(smcup.len())
    }

    pub fn rmcup(&mut self) -> Result<usize> {
        let rmcup = expand!(self.db.get::<cap::ExitCaMode>().unwrap().as_ref())?;
        self.write_all(&rmcup)?;
        Ok(rmcup.len())
    }

    pub fn cup(&mut self, line: u16, col: u16) -> Result<usize> {
        let cup = expand!(self.db.get::<cap::CursorAddress>().unwrap().as_ref(); line, col)?;
        self.write_all(&cup)?;
        Ok(cup.len())
    }

    pub fn write_ascii(&mut self, count: usize) -> Result<usize> {
        self.buf.clear();
        for _ in 0..count {
            self.buf.push(self.rng.gen_range(32, 127));
        }

        self.out.write_all(&self.buf)?;
        Ok(count)
    }

    pub fn setaf(&mut self, v: u16) -> Result<usize> {
        let setaf = expand!(
            self.db.get::<cap::SetAForeground>().unwrap().as_ref(); v
        )?;
        self.write_all(&setaf)?;
        Ok(setaf.len())
    }

    pub fn sgr0(&mut self) -> Result<usize> {
        let sgr0 = expand!(self.db.get::<cap::ExitAttributeMode>().unwrap().as_ref())?;
        self.write_all(&sgr0)?;
        Ok(sgr0.len())
    }

    pub fn csr(&mut self, top: u16, bottom: u16) -> Result<usize> {
        let csr = expand!(
            self.db.get::<cap::ChangeScrollRegion>().unwrap().as_ref();
            top, bottom
        )?;
        self.write_all(&csr)?;
        Ok(csr.len())
    }
}

impl<'a, W: Write + 'a> Write for Context<'a, W> {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.out.write(bytes)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}
