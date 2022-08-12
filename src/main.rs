#![allow(non_snake_case)]
use rand::Rng;
use tts_rust::{ GTTSClient, languages::Languages };
use std::{io::stdout, time::Duration};

use crossterm::{
    cursor::position,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
fn main() {
    println!("Hell&o, world!");

	let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
    };
    narrator.speak("5 plus -6");
}

enum ArithmeticOperators
{
	Addition(i16, i16),
	Multiplication(i16, i16),
	Division(i16, i16)
}

fn AskQuestion(operator : ArithmeticOperators, narrator : GTTSClient)
{
	let mut randGen = rand::thread_rng();
	match operator
	{ 
		ArithmeticOperators::Addition(numsMin, numsMax) => 
		{
			let numOne = randGen.gen_range(numsMin..numsMax);
			let numTwo = randGen.gen_range(numsMin..numsMax);
			
			if randGen.gen_range(0u8..1) == 0
			{
				narrator.speak(&format!("{numOne} plus {numTwo}"));
				let answer = numOne + numTwo;
			}
			else 
			{
				narrator.speak(&format!("{numOne} minus {numTwo}"));
				let answer = numOne - numTwo;
			}
			
			
		}
		_ => println!("Not implemented error"),
	}
	print_events();





}

fn print_events() -> Result<()> {
    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('`').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            /*if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }*/
        } else {
            // Timeout expired, no event for 1s
            println!(".\r");
        }
    }

    Ok(())
}