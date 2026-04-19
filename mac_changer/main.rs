//MAC changer program inspired by ZSecurity's Python program of the same name from their 
//"Learn Python and Ethical Hacking from Scratch" course on Udemy.com
//Working to comlete this with the addition of flags so it can be adjusted in the terminal
//like the original flow from ZSecurity.

use subprocess::Exec;
//use optparse

fn main() {
    //Interface to change and the address to change it to.
    let interface = "eth0";
    let new_mac = "00:11:22:33:44:55";

    println!("Changing MAC address for {}.", interface);

    let ch_mac_process = vec![Exec::cmd("sudo").args(&["ip", "link", "set", interface, "down"]),
        Exec::cmd("sudo").args(&["ip", "link", "set", interface, "address", new_mac]),
        Exec::cmd("sudo").args(&["ip", "link", "set", interface, "up"])
    ];

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
    println!("Changed {} MAC address to {}.", interface, new_mac);
}
