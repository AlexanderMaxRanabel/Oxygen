use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_nixos() -> bool {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("ID=nixos") {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let argument = &args[1].to_string();
        let installion_argument = args.get(2).map(String::clone).unwrap_or_default();


        if is_nixos() {
         match argument.as_str() {
             "install" => {
                let install_command = "nixos.".to_string() + &installion_argument;

                let output = Command::new("nix-env")
                .arg("-iA")
                .arg(install_command)
                .output()
                .expect("Failed to run command");

            if output.status.success() {
                println!("İnstalled wanted packages");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                 eprintln!("Failed to run command. Error: {}", stderr);
                }
            },

            "uc" => {
                let output = Command::new("nix-channel")
                 .arg("--update")
                 .output()
                 .expect("Failed to run command");

                if output.status.success() {
                    println!("Updated Channels");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to run command. Error: {}", stderr);
                }
            },

            "list" => {
               let output = Command::new("nix-env")
               .arg("-q")
               .output()
               .expect("Failed to run command");

             if output.status.success() {
                println!("Succesfully runned command");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to run command. error: {}", stderr);
                }
            },

            "clear" => {
                let output = Command::new("nix-collect-garbage")
                 .output()
                 .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

            "clear-generations" => {
                let output = Command::new("nix-collect-garbage")
                .arg("-d")
                .output()
                .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                 }
            },

            "delete" => {
                let output = Command::new("nix-env")
                    .arg("--uninstall")
                    .arg(installion_argument)
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

             "generations" => {
                let output = Command::new("nix-env")
                    .arg("--list-generations")
                    .output()
                    .expect("Failed to run command");
                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
               }
             },

             "rebuild" => {
                let output = Command::new("nixos-rebuild")
                    .arg("switch")
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    println!("Builded system");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error {}", stderr);
                }
             },

             "switch" =>  {
                 let output = Command::new("nix-env")
                     .arg("--switch-generation")
                     .arg(installion_argument.clone())
                     .output()
                     .expect("Failed to run command");

                if output.status.success() {
                    println!("Switched Generation to: {}", installion_argument);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error {}", stderr);
                }
             },

             "help" => {
                println!("install <package name>: installs the specified package");
                println!("delete <package name>: deletes specified package");
                println!("clear: Garbage collects Nix");
                println!("list: lists installed packages");
                println!("generations: lists generations");

             },

            "version" => {
                println!("0.1.1");
            },
            _ => {
                println!("Invalid Argument");
            }
         }
        } else {
        match argument.as_str() {
            "install" => {
                let install_command = "nixpkgs.".to_string() + &installion_argument;

                let output = Command::new("nix-env")
                .arg("-iA")
                .arg(install_command)
                .output()
                .expect("Failed to run command");

            if output.status.success() {
                println!("İnstalled wanted packages");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                 eprintln!("Failed to run command. Error: {}", stderr);
                }
            },

            "uc" => {
                let output = Command::new("nix-channel")
                 .arg("--update")
                 .output()
                 .expect("Failed to run command");

                if output.status.success() {
                    println!("Updated Channels");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to run command. Error: {}", stderr);
                }
            },

            "list" => {
               let output = Command::new("nix-env")
               .arg("-q")
               .output()
               .expect("Failed to run command");

             if output.status.success() {
                println!("{:#?}", output);
                println!("Succesfully runned command");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to run command. error: {}", stderr);
                }
            },

            "clear" => {
                let output = Command::new("nix-collect-garbage")
                 .output()
                 .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

            "delete" => {
                let output = Command::new("nix-env")
                    .arg("--uninstall")
                    .arg(installion_argument)
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

            "clear-generations" => {
                let output = Command::new("nix-collect-garbage")
                .arg("-d")
                .output()
                .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                 }
            },

            "generations" => {
                let output = Command::new("nix-env")
                    .arg("--list-generations")
                    .output()
                    .expect("Failed to run command");
                if output.status.success() {
                    println!("{:#?}", output);
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

            "install-nix-singular" => {
                let output = Command::new("sh")
                    .arg("<(curl -L https://nixos.org/nix/install)")
                    .arg("--no-daemon")
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

            "install-nix-multi" => {
                let output = Command::new("sh")
                    .arg("<(curl -L https://nixos.org/nix/install)")
                    .arg("--daemon")
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

            "switch" =>  {
                 let output = Command::new("nix-env")
                     .arg("--switch-generation")
                     .arg(installion_argument.clone())
                     .output()
                     .expect("Failed to run command");

                if output.status.success() {
                    println!("Switched Generation to: {}", installion_argument);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error {}", stderr);
                }
             },

            _ => {
                println!("Invalid Argument");
            }
        }
      }
    } else {
        println!("No argument provided.");
    }
}
