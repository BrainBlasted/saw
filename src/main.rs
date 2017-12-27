//  A dead-simple wrapper around apt
//  Copyright (C) 2017 Christopher Davis
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate clap;
extern crate runas;

use runas::Command;
use clap::{App,Arg};

fn main() {
    let app = App::new("saw")
        .version("0.1.0")
        .author("Christopher Davis <brainblastedmods at gmail dot com>")
        .about("Simple wrapper around apt on Debian-based systems")
        .arg(Arg::with_name("COMMAND").required(true).index(1)
             .help("Subcommand to execute"))
        .arg(Arg::with_name("CMD-ARGS").index(2).multiple(true));
    let matches = app.get_matches();

    // If COMMAND is not provided, clap should handle it
    let cmd = matches.value_of("COMMAND").unwrap();
    let args = matches.values_of_lossy("CMD-ARGS");
    let mut run_cmd = Command::new("apt");
    run_cmd.arg(cmd);
    if args.is_some() {
        let args = args.unwrap();
        for arg in args.iter() {
            run_cmd.arg(arg);
        }
    }
    run_cmd.status().unwrap();

}
