
use std::io::{stdout, stdin,Write};
use std::fs::File;
use std::io::BufReader;
use rodio::source;
use rodio::{Decoder, OutputStream, source::Source};
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

    let mut currently_playing: String = "".to_string();
    let mut current_artist = "".to_string();
    let mut current_title = "".to_string();

    loop {
        println!("\x1B[2J\x1B[1;1H");
        println!("\x1b[1;35mCurrently Playing: \x1b[0m {} - {}", current_artist, current_title);
        println!("\x1b[1;35mVolume: \x1b[0m{}", Sink::volume(&sink));
        if Sink::is_paused(&sink) {
            println!("\x1b[1;35mStatus: \x1b[0mPaused");
        }
        else if Sink::empty(&sink) {
            println!("\x1b[1;35mStatus: \x1b[0mStopped");
        }
        else {
            println!("\x1b[1;35mStatus: \x1b[0mPlaying");
        }
        if Sink::speed(&sink) != 1.0 {
            println!("\x1b[1;35mSpeed: \x1b[0m{}", Sink::speed(&sink));
        }
        let input = prompt("Enter a command: ");
        if input == "play"{
            let filePath:String = prompt("Enter a file name: ");
            let file_path = filePath.clone();  // store the value of filePath in a separate variable

            // Use the `match` statement to handle the Result returned by File::open()
            match std::fs::File::open(filePath) {
                Ok(file) => {
                    Sink::set_volume(&sink, 0.0);
                    Sink::set_speed(&sink, 10000.0);
                    Sink::sleep_until_end(&sink);
                    Sink::set_volume(&sink, 1.0);
                    Sink::set_speed(&sink, 1.0);
                    Sink::append(&sink, rodio::Decoder::new(BufReader::new(file)).unwrap());
                    let meta = mp3_metadata::read_from_file(file_path.clone()).expect("File error");
                    if let Some(tag) = meta.tag {
                        current_artist = tag.artist.to_string();
                        current_title = tag.title.to_string();
                    }
                    else{
                        current_artist = "unknown".to_string();
                        current_title = file_path.to_string();
                    }


                },
                Err(error) => {
                    // Handle the error here
                    println!("Error opening file: {}", error);
                }
            }
 
        }
        else if input == "pause" {
            Sink::pause(&sink);
        }
        else if input == "resume" {
            Sink::play(&sink);
        }
        else if input == "stop" {
            Sink::stop(&sink);
            currently_playing = "".to_string();
        }
        else if input == "volume" {
            let volume = prompt("Enter a volume: ");
            let volume = volume.parse::<f32>().unwrap();
            Sink::set_volume(&sink, volume);
        }
        else if input == "speed" {
            let speed = prompt("Enter a speed: ");
            let speed = speed.parse::<f32>().unwrap();
            Sink::set_speed(&sink, speed);
        }
        else if input == "skip" {
            Sink::set_volume(&sink, 0.0);
            Sink::set_speed(&sink, 5000.0);
            Sink::sleep_until_end(&sink);
            Sink::set_volume(&sink, 1.0);
            Sink::set_speed(&sink, 1.0);


        }

        else if input == "exit" {
            break;
        }

    }
}