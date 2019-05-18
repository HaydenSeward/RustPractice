use std::io;
use std::process::Command;

fn main() {

    let mut input = String::new();

        
     loop {
        
        io::stdin().read_line(&mut input).expect("You Are A Fail");
        
        if input.to_string().trim() == "q" {
    
            break;          

        } else if input.to_string().trim() == "l" {

            Command::new("pkill -u")    
                .arg("denard")
                .spawn()
                .expect("Logout Failed");

        } else if input.to_string().trim() == "s" {

            Command::new("shutdown")
                .arg("-h now")
                .spawn()
                .expect("Shutdown Failed");
        }

    }

}
