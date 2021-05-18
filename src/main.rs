use std::fs;
use std::env;
use std::str::FromStr;
use serde_derive::Deserialize;
use webbrowser;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time};
use clipboard_win::{formats, get_clipboard};
use enigo::*;

#[derive(Deserialize)]
struct Config {
    copy_mode: Option<bool>,
    config: Vec<Site>
}

#[derive(Deserialize)]
struct Site {
     front: String,
     keys: String,
     back: Option<String>
}

struct KeyEvent {
    keys: Vec<Keycode>,
    front: String,
    back: Option<String>
}

impl Default for KeyEvent {
    fn default () -> KeyEvent {
        KeyEvent{keys: Vec::new(), front: String::new(), back: Option::None}
    }
}

fn parse_keys(key_str: &String) -> Result<Vec<Keycode>, String>  {
    let mut key_events = Vec::new();
    for key in key_str.split("+") {
        match Keycode::from_str(key){
            Ok(k) => key_events.push(k),
            Err(e) => return Err(e),
        };
    }
    Ok(key_events)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("please pass the configuration file as a cmd line arg");
        return;
    }

    let mut copy_mode = false;
    let mut enigo = Enigo::new();
    
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("unable to read config file");
    let config: Config = toml::from_str(&contents.to_string()).unwrap();
    match config.copy_mode {
        Some(_) => {
            copy_mode = true;
            println!("Using copy mode")
        }
        _ => println!("Using regular mode")
    };

    let mut key_events = Vec::new();

    for site in config.config {
        let mut k = KeyEvent::default();
        k.front = site.front;
        k.back = site.back;
        match parse_keys(&site.keys){
            Ok(key_vec) => k.keys = key_vec,
            Err(_) => {
                println!("Failed to parse: {}", site.keys); 
                continue
                }
        };
        key_events.push(k);
    }

    if key_events.len() == 0 {
        println!("No valid key events. exiting");
        return;
    }

    let device_state = DeviceState::new();
    let ten_millis = time::Duration::from_millis(10);

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if !keys.is_empty(){
            // Need to optimize lookup by using efficient data structure
            for event in &key_events {
                let mut is_valid = true; 
                for key in &event.keys {
                    if !keys.contains(&key){
                        is_valid = false;
                        break;
                    }
                }
                if is_valid {
                    if copy_mode {
                        loop{
                            let k = device_state.get_keys();
                            if k.is_empty(){                                
                                enigo.key_sequence_parse("{+CTRL}c{-CTRL}");
                                break;
                            }
                        } 
                    }
                    let mut query = event.front.clone();
                    let content: String = get_clipboard(formats::Unicode).expect("To get clipboard");
                    query.push_str(&content);
                    if let Some(m) = &event.back {
                        query.push_str(m);
                    }
                    if webbrowser::open(&query).is_ok() {
                        println!("URL: {}", query);
                    }
                    thread::sleep(time::Duration::from_millis(1000));
                }
            }
        }
        thread::sleep(ten_millis);
    }
}
