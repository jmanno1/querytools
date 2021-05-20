mod parser;
use parser::parser::*;
use std::fs;
use std::env;
use webbrowser;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use enigo::*;

struct KeyEvent {
    keys: Vec<Keycode>,
    front: String,
    back: Option<String>,
    copy_mode: bool
}

impl Default for KeyEvent {
    fn default () -> KeyEvent {
        KeyEvent{keys: Vec::new(), front: String::new(), back: Option::None, copy_mode: false}
    }
}

fn populate_key_events(config: &Config) -> Vec<KeyEvent> {
    let mut key_events = Vec::new();
    for site in &config.config {
        let mut k = KeyEvent::default();
        k.front = site.front.clone();
        k.back = site.back.clone();
        k.copy_mode = site.copy_mode.unwrap_or(false);
        match parse_keys(&site.keys){
            Ok(key_vec) => k.keys = key_vec,
            Err(_) => {
                println!("Failed to parse: {}", site.keys); 
                continue
                }
        };
        key_events.push(k);
    }
    return key_events;
}

fn listen_for_events(key_events: &Vec<KeyEvent>, config: &Config) {
    let mut enigo = Enigo::new();
    let device_state = DeviceState::new();
    let ten_millis = time::Duration::from_millis(10);
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if !keys.is_empty(){
            // Need to optimize lookup by using efficient data structure
            for event in key_events {
                let mut is_valid = true; 
                for key in &event.keys {
                    if !keys.contains(&key){
                        is_valid = false;
                        break;
                    }
                }
                if is_valid {
                    if config.copy_mode.unwrap_or(false) || event.copy_mode {
                        loop{
                            let k = device_state.get_keys();
                            if k.is_empty(){                                
                                enigo.key_sequence_parse("{+CTRL}c{-CTRL}");
                                break;
                            }
                            thread::sleep(ten_millis);
                        }
                    }

                    let mut query = event.front.clone();
                    let content: String = ctx.get_contents().unwrap_or("".to_string());

                    query.push_str(&content);
                    if let Some(m) = &event.back {
                        query.push_str(m);
                    }

                    if query.len() > 2000 {
                        println!("URL length exceeds 2000 characters, aboritng query");
                        continue; 
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("please pass the configuration file as a cmd line arg");
        return;
    }

    let contents = fs::read_to_string(&args[1]).expect("unable to read config file");
    let config: Config = parse_config_file(&contents);

    let key_events = populate_key_events(&config);

    if key_events.len() == 0 {
        println!("No valid key events. exiting");
        return;
    }

    // Does not return
    listen_for_events(&key_events, &config);
}
