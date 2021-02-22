
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ubi_reader/ubifs
// (c) 2013 Jason Pruitt (jrspruitt@gmail.com)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// pub mod settings;
use crate::settings;
use std::process;
use backtrace::Backtrace;

pub fn log(obj: &str, message: &str){
    unsafe {
        if settings::LOGGING_ON || settings::LOGGING_ON_VERBOSE {
            println!("{} {}", obj, message);
        }
    }
}

pub fn verbose_log(obj: &str, message: &str) {
    unsafe{
        if settings::LOGGING_ON_VERBOSE {
            log(obj, message);
        }
    }
}

// pub fn verbose_dispaly(displayable_obj: object){
//     if settings::LOGGING_ON_VERBOSE {
//         println!(displayable_obj.display("\t"))
//     }
// }

pub fn error(obj: &str, level: &str, message: &str){
    unsafe{
        if settings::ERROR_ACTION == "exit" {
            println!("{} {} {}", obj, level, message);
            if settings::FATAL_TRACEBACK == true {
                let bt = Backtrace::new();
                println!("{:?}", bt);
            }
            process::exit(1)
        }else{
            if level == "warn"{
                println!("{} {}: {}", obj, level, message);
            }else if level == "fatal" {
                println!("{:?} {} {}", obj, level, message);
                if settings::FATAL_TRACEBACK == true{
                    let bt = Backtrace::new();
                    println!("{:?}", bt);
                }
                process::exit(1);
            }else {
                println!("{:?} {}: {}", obj, level, message);
            }
        }
    }
}


