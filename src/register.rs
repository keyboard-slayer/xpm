/**
 * Copyright (C) 2021 KeyboardSlayer (Jordan Dalcq)
 * 
 * This file is part of xpm.
 * 
 * xpm is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * xpm is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with xpm.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::env;
use std::process::exit;

pub fn read_from_register(key: String) -> Option<String> 
{
    let mut appdata: String = String::from("");

    match std::env::consts::OS
    {
        "windows" => {
            appdata = format!("{}\\xpm", env::var_os("APPDATA").unwrap().to_str().unwrap());
        }

        "linux" => {
            appdata = format!("{}\\config\\xpm", env::var_os("APPDATA").unwrap().to_str().unwrap());
        }

        _ => {
            eprintln!("xpm: your platform is not compatible yet");
            exit(1);
        }
    } 

    Some(String::from("DEBUG"))
}