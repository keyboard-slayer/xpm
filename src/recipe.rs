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

use std::io;
use std::env;
use std::process::exit;
use async_recursion::async_recursion;

use crate::register::read_from_register;

use hyper::{Client, body::HttpBody};
use yaml_rust::{YamlLoader, yaml::Yaml};

const SERVER: &str = "http://localhost:8000";

pub async fn grab_recipe(package: String) -> Result<Vec<Yaml>, Box<dyn std::error::Error + Send + Sync>>
{
    let client = Client::new();
    let url: String = format!("{}/{}.yml", SERVER, package);
    let uri = url.parse()?;
    let mut resp = client.get(uri).await?;

    if resp.status() != 200 
    {
        eprintln!("Couldn't get {}'s recipe from {}", package, SERVER);
        exit(1);
    }

    if let Some(chunk) = resp.body_mut().data().await
    {
        let yml = String::from_utf8(chunk.unwrap().to_vec()).unwrap();

        match YamlLoader::load_from_str(yml.as_str())
        {
            Ok(doc) => {
                return Ok(doc);
            }

            Err(p) => {
                eprintln!("{}", p);
                exit(1);
            }
        }
    }
    else 
    {
        eprintln!("Couldn't get {}'s recipe from {}", package, SERVER);
        exit(1);
    }
}

#[async_recursion]
pub async fn parse_recipe(recipe: Yaml, option: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    read_from_register(String::from(recipe["name"].as_str().unwrap()));

    match option.as_str()
    {
        "build" => {
            if !recipe["build"]["deps"].is_badvalue()
            {
                for deps in recipe["build"]["deps"].clone()
                {
                    let dep_recipe = grab_recipe(deps.as_str().unwrap().to_string()).await?;
                    parse_recipe(dep_recipe[0].clone(), String::from("install")).await?;
                }
            }

            else
            {
                eprintln!("xpm: no recipe to build {}", recipe["name"].as_str().unwrap());
                exit(1);
            }
        }

        "install" => {
            if recipe["binaries"].is_badvalue()
            {
                let mut choice = String::new();
                
                println!("No binaries are available for {}", recipe["name"].as_str().unwrap());
                println!("Do you want to build it first ? [Y/n]\n");

                io::stdin().read_line(&mut choice).ok();
                println!("");
                choice = String::from(choice.trim_end());

                if choice == "" || choice == "y" || choice == "Y"
                {
                    parse_recipe(recipe, String::from("build")).await?;
                }
                else 
                {
                    eprintln!("xpm: installation aborted");
                    exit(1);
                }
            }
        }

        "uninstall" => {}

        _ => {}
    }

    Ok(())
}