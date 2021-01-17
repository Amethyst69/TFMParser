use std::collections::HashMap;
use std::process::Command;
use std::path::Path;
use regex::Regex;
use std::fs;


fn main() {
    download_swf();
    let dump = read_dump();
    let functions = extract_functions(&dump);

    // Search Transformice's encryption keys
    let version = search_version(&dump);
    let connection_key = search_connection_key(&dump);
    let auth_key = search_auth_key(&dump, &functions);

    // Display encryption keys
    println!("\nVersion: {}", version);
    println!("Connection key: {}", connection_key);
    println!("Authentication key: {}", auth_key);

    println!("\nCleaning up the files ...");
    clean_up();
}

fn download_swf() {
    let mut cmd = Command::new("wget");
    cmd.arg("http://www.transformice.com/Transformice.swf");

    println!("Downloading the SWF ...");

    match cmd.output() {
        Ok(_) => {
            println!("Downloaded the SWF ...")
        }
        Err(_) => {
            println!("Couldn't download the SWF ... Did you download WGET?")
        }
    }
}

fn read_dump() -> Vec<String> {
    Command::new(r"Tools\dumper")
        .args(&["Transformice.swf", "tfm.swf"])
        .output()
        .expect("Some error occurred ...\nExit status 1.");

    let cmd = Command::new(r"Tools\swfdump")
        .args(&["-a", "tfm.swf"])
        .output()
        .expect("Some error occurred ...\nExit status 1.");

    String::from_utf8(cmd.stdout)
        .unwrap()
        .split("\r\n")
        .map(|x| x.to_owned())
        .collect()
}

fn clean_up() {
    for file in &["Transformice.swf", "tfm.swf"] {
        if Path::new(file).exists() {
            fs::remove_file(file)
                .expect("Couldn't delete the file {file}");
        }
    }
}

fn extract_functions(dump: &Vec<String>) -> HashMap<&str, i32> {
    let mut functions = HashMap::<&str, i32>::new();
    let patt = Regex::new("push(int|byte|short)").unwrap();
    for mut i in 0..dump.len() {
        if dump[i].contains("method <q>[public]::int") && dump[i].contains("0 params") {
            let mut value = 0;
            let start = i;
            
            while !dump[i].contains("returnvalue") {
                if patt.is_match(&dump[i]) {
                    let n = dump[i].split(" ").last().unwrap().parse::<i32>().unwrap();
                    value += n;
                }
                i += 1;
            }

            let funcname = dump[start]
                .split("::").last().unwrap()
                .split("=").nth(0).unwrap();

            functions.insert(funcname, value);
        }
    }
    functions
}

fn search_version(dump: &Vec<String>) -> i32 {
    let patt = Regex::new(r"int = (\d+)").unwrap();
    for line in dump {
        if line.contains("int") {
            let v = patt.captures(&line).unwrap();
            return v[1].parse().unwrap();
        }
    }
    0
}

fn search_connection_key(dump: &Vec<String>) -> String {
    for mut i in 0..dump.len() {
        if dump[i].contains("getscopeobject 1") && dump[i + 1].contains("getslot 7") && dump[i + 2].contains("getlocal_0") {
            while !dump[i].contains("system::Capabilities") {
                if dump[i].contains("getlex") && dump[i + 1].contains("getproperty") {
                    if dump[i + 2].contains("getscopeobject 1") || dump[i + 2].contains("callproperty") {
                        let varname = dump[i + 1].split("::").last().unwrap().to_string();
                        for line in dump {
                            if line.contains(&varname) && line.contains("String") {
                                let ckey = line.split("=").last().unwrap().trim().to_string();
                                return ckey;
                            }
                        }
                    }
                }
                i += 1;
            }
        }
    }
    "".to_string()
}

fn search_auth_key(dump: &Vec<String>, functions: &HashMap<&str, i32>) -> i32 {
    let mut auth_key = 0;
    let patt = Regex::new(r"::([\\0-9]+), 0 params$").unwrap();
    
    for mut i in 0..dump.len() {
        if dump[i].contains("getlocal_0") {
            if dump[i + 2].contains("convert_i") && dump[i + 3].contains("setlocal_1") {
                while !dump[i].contains("returnvalue") {

                    if dump[i].contains("bitxor") {
                        if dump[i - 1].contains("callproperty") {
                            let funcname = &patt.captures(&dump[i - 1]).unwrap()[1];
                            auth_key ^= functions.get(funcname).unwrap();
                        }
                    }
                    else if dump[i].contains("lshift") {
                        if dump[i - 1].contains("callproperty") {
                            let funcname = &patt.captures(&dump[i - 1]).unwrap()[1];
                            auth_key ^= 1 << functions.get(funcname).unwrap();
                        }
                    }
                    i += 1;
                }
                break;
            }
        }
    }
    auth_key
}