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

mod recipe;
pub mod register;

use tokio;
use std::env;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 3 
    {
        eprintln!("xpm: not enough arguments");
        eprintln!("usage: xpm [install | uninstall | build ] package(s)");
        exit(1);
    }

    args.remove(0);

    let option = args.remove(0);

    for package in args 
    {
        let recipe = recipe::grab_recipe(package.clone()).await?;
        let package_recipe = recipe[0].clone();

        if package_recipe["name"].is_badvalue() || package_recipe["name"].as_str().unwrap() != package
        {
            eprintln!("xpm: invalid recipe !");
            exit(1);
        }

        if vec!["install", "uninstall", "build"].iter().any(|&x| x.clone() == option)
        {
            if package_recipe[option.as_str()].is_badvalue() 
            {
                eprintln!("xpm: no recipe to {} {}", option, package);
                exit(1);
            }

            recipe::parse_recipe(package_recipe, option.clone()).await?;
        }        
        else 
        {
            eprintln!("xpm: invalid option -- {}", option);
            exit(1);
        }

    }

    Ok(())
}
