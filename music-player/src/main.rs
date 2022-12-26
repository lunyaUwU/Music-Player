
use std::io::{stdout, stdin,Write};
use std::io::BufReader;
use rodio::Sink;
extern crate mp3_metadata;

fn print(input: String){
    print!("{}",input);
    let _flush = stdout().flush();

}
fn prompt(input: &str) -> String{
    print(input.to_owned());
    let stdin = stdin(); // We get `Stdin` here.
    let mut input = String::new();
    stdin.read_line(&mut input).expect("failed to read from stdin");
    input = input.trim().to_string();
    return input
}
fn main(){
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let mut show_stats = true;
    let mut current_artist = "".to_string();
    let mut current_title = "".to_string();
    let mut color = "35";
    loop {
        if show_stats {
            println!("\x1B[2J\x1B[1;1H");
            println!("\x1b[1;{}mCurrently Playing: \x1b[0m {} - {}",color, current_artist, current_title);
            println!("\x1b[1;{}mVolume: \x1b[0m{}",color, Sink::volume(&sink));
            if Sink::is_paused(&sink) {
                println!("\x1b[1;{}mStatus: \x1b[0mPaused",color);
            }
            else if Sink::empty(&sink) {
                println!("\x1b[1;{}mStatus: \x1b[0mStopped",color);
            }
            else {
                println!("\x1b[1;{}mStatus: \x1b[0mPlaying",color);
            }
            if Sink::speed(&sink) != 1.0 {
                println!("\x1b[1;{}mSpeed: \x1b[0m{}",color, Sink::speed(&sink));
            }
        }
        show_stats = true;
        let input = prompt("Enter a command: ");

        if input.starts_with("play"){
            let mut file_path = input.replace("play ", "");
            if file_path == "play" {
                file_path = prompt("Enter a file name: ");
            }
            let file_path_2 = file_path.clone();  // store the value of filePath in a separate variable

            // Use the `match` statement to handle the Result returned by File::open()
            match std::fs::File::open(file_path) {
                Ok(file) => {
                    Sink::set_volume(&sink, 0.0 );
                    Sink::set_speed(&sink, 1000.0);
                    Sink::sleep_until_end(&sink);
                    Sink::set_volume(&sink, 1.0);
                    Sink::set_speed(&sink, 1.0);
                    Sink::append(&sink, rodio::Decoder::new(BufReader::new(file)).unwrap());
                    let meta = mp3_metadata::read_from_file(file_path_2.clone()).expect("File error");
                    if let Some(tag) = meta.tag {
                        current_artist = tag.artist.to_string();
                        current_title = tag.title.to_string();
                    }
                    else{
                        print!("No metadata found for file: {} (using file name instead)\r", file_path_2);
                        current_artist = "unknown".to_string();
                        current_title = file_path_2.to_string();
                    }


                },
                Err(error) => {
                    // Handle the error here
                    println!("Error opening file: {}", error);
                }
            }
 
        }
        else if input.starts_with("add Song") || input.starts_with("add"){
            let mut file_path = input.replace("add Song ", "");
            if file_path == "add Song" {
                file_path = prompt("Enter a file name: ");
            }
            match std::fs::File::open(file_path) {
                Ok(file) => {
                    Sink::append(&sink, rodio::Decoder::new(BufReader::new(file)).unwrap());
                },
                Err(error) => {
                    // Handle the error here
                    println!("Error opening file: {}", error);
                }
            };
        }
        else if input == "pause" {
            Sink::pause(&sink);
        }
        else if input == "resume" {
            Sink::play(&sink);
        }
        else if input == "stop" {
            Sink::stop(&sink);
        }
        else if input.starts_with("volume") || input.starts_with("vol") {
            //strip the command from the input  
            let mut volume = input.replace("volume ", "");
            volume = volume.replace("vol ", "");
            //check if the string has a number
            if volume.parse::<f32>().is_ok() {
                let volume = volume.parse::<f32>().unwrap();
                Sink::set_volume(&sink, volume);
            }
            else {
                let volume = prompt("Enter a volume: ");
                let volume = volume.parse::<f32>().unwrap();
                Sink::set_volume(&sink, volume);
            }   
        }
        else if input.starts_with("speed") {
            let speed = input.replace("speed ", "");
            if speed.parse::<f32>().is_ok() {
                let speed = speed.parse::<f32>().unwrap();
                Sink::set_speed(&sink, speed);
            }
            else {
                let speed = prompt("Enter a speed: ");
                let speed = speed.parse::<f32>().unwrap();
                Sink::set_speed(&sink, speed);
            }
        }
        else if input == "skip" {
            Sink::set_volume(&sink, 0.0);
            Sink::set_speed(&sink, 5000.0);
            Sink::sleep_until_end(&sink);
            Sink::set_volume(&sink, 1.0);
            Sink::set_speed(&sink, 1.0);


        }
        else if input.starts_with("theme") {
            let color_string:&str = &input.replace("theme ", "");
            if color_string == "theme" {
                let color_string_2:&str = &prompt("Enter a color: ");
                            
                match color_string_2{
                    "red" => color = "31",
                    "green" => color = "32",
                    "yellow" => color = "33",
                    "blue" => color = "34",
                    "magenta" => color = "35",
                    "cyan" => color = "36",
                    "white" => color = "37",
                    _ => color = "35",
                }
            }
            else {
                match color_string{
                    "red" => color = "31",
                    "green" => color = "32",
                    "yellow" => color = "33",
                    "blue" => color = "34",
                    "magenta" => color = "35",
                    "cyan" => color = "36",
                    "white" => color = "37",
                    _ => color = "35",
                }
            }

        }
        else if input == "help" {
            println!("Commands:");
            println!("\x1b[1;{}mplay         - \x1b[0mplays a file",color);
            println!("\x1b[1;{}mpause        - \x1b[0mpauses the current song",color);
            println!("\x1b[1;{}mresume       - \x1b[0mresumes the current song",color);
            println!("\x1b[1;{}mstop         - \x1b[0mbreaks everything",color);
            println!("\x1b[1;{}mvolume \x1b[34m[\x1b[35mvol\x1b[34m]\x1b[{}m - \x1b[0msets the volume",color,color);
            println!("\x1b[1;{}mspeed        - \x1b[0msets the speed", color);
            println!("\x1b[1;{}mskip         - \x1b[0mskips the current song",color);
            println!("\x1b[1;{}mtheme        - \x1b[0mlet you change the color",color);
            println!("\x1b[1;{}mexit         - \x1b[0mexits the program",color);
            show_stats = false;
        }
        else if input == "stats" {
            if show_stats {
                show_stats = false;
            }
            else {
                show_stats = true;
            }
        }

        else if input == "exit" {
            break;
        }

    }
}