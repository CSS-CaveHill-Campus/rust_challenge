mod red_button {

    use std::fs::File;
    use std::io::{self, Write};
    use std::process::Command;
    use reqwest::*;

    fn red_button() -> Result<()> {
        // Fetch the public IP address
        let response = reqwest::blocking::get("https://api.ipify.org").expect("failed to get response");
        let public_ip: String = response.text().expect("failed to parse response");

        // Write the IP address to a file
        let file_path = "public_ip.txt";
        let mut file = File::create(file_path).expect("failed to create file");
        writeln!(file, "Public IP Address: {}", public_ip).expect("failed to write to file");

        // Open the file in Notepad
        Command::new("notepad.exe")
            .arg(file_path)
            .spawn().expect("could not launch notepad");

        Ok(())
    }
}
