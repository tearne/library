use std::path::PathBuf;

use color_eyre::Result;
use dialoguer::{Confirm, theme::ColorfulTheme};
use regex::Regex;
use color_eyre::eyre::eyre;

fn main() -> Result<()> {
    let key_opt = find_private_key()?;

    match key_opt {
        Some(path) => println!("You have a key at {}", &path.to_string_lossy()),
        None => {
            println!("You don't seem to have a key.");
            
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Want me to make one?")
                .default(false)
                .interact()
                .unwrap()
            {
                println!("Looks like you want to continue");
            } else {
                println!("nevermind then :(");
            }
            
        },
    };

    Ok(())
}

fn find_private_key() -> Result<Option<PathBuf>> {
    let re = Regex::new(r"^C:[\w\\]+.ssh\\id_\w+$")?;

    let ssh_dir = dirs::home_dir().expect("Failed to find home dir").join(".ssh");

    if !ssh_dir.exists() {
        Ok(None)
    } else {
        let f = std::fs::read_dir(ssh_dir).unwrap().collect::<Vec<_>>();
        let mut matches: Vec<PathBuf> = f.iter().filter_map(|pr|{
            let path_buf = pr.as_ref().unwrap().path();
            if re.is_match(&path_buf.to_string_lossy()) { 
                Some(path_buf) 
            }
            else { None }
        }).collect();


        let num_matches = matches.len();
        if num_matches == 0 {
            Ok(None)
        } else if num_matches == 1 {
            Ok(Some(matches.swap_remove(0)))
        } else {
            Err(eyre!("More than one ssh key available: {:?}", matches))
        }
    }
}
