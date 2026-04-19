//MAC changer program inspired by ZSecurity's Python program of the same name from their 
//"Learn Python and Ethical Hacking from Scratch" course on Udemy.com
//Working to comlete this with the addition of flags so it can be adjusted in the terminal
//like the original flow from ZSecurity.

use subprocess::Exec;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Interface to change
    #[arg(short, long)]
    interface: String,

    ///New MAC address to assign
    #[arg(short, long)]
    maddr: String,
}

fn main() {
    //Interface to change and the address to change it to.
    let argus = Args::parse();

    println!("Changing MAC address for {}.", &argus.interface);

    //Commands to execute, take down the interface, change the MAC and bring it back up
    let ch_mac_process = vec![Exec::cmd("sudo").args(&["ip", "link", "set", &argus.interface, "down"]),
        Exec::cmd("sudo").args(&["ip", "link", "set", &argus.interface, "address", &argus.maddr]),
        Exec::cmd("sudo").args(&["ip", "link", "set", &argus.interface, "up"])
    ];
    
    //Iterate through the commands, and return status
    for (i, cmd) in ch_mac_process.into_iter().enumerate() {
        match cmd.join() {
            Ok(status) if status.success() => {
                println!("Attempting step {}...", i + 1);
            }
            Ok(status) => {
                eprintln!("Step {} failed with code{}.", i + 1, status);
                return;
            }
            Err(e) => {
                eprintln!("Failed to execute: {}.", e);
                return;
            }
        }
    }

    println!("Changed {} MAC address to {}.", &argus.interface, &argus.maddr);
}
