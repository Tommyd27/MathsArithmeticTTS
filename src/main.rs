#![allow(non_snake_case)]
use rand::Rng;
use tts_rust::{ GTTSClient, languages::Languages };
use std::{/*io::stdout,*/ time::Duration, fmt::format};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    execute,
    //terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
fn main() {
    println!("Hell&o, world!");

	let narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
    };
	let mut randGen = rand::thread_rng();
   	let mut operator = ArithmeticOperators::Addition(1, 1);
	while true
   	{
	match randGen.gen_range(0u8..2)
	{

		0 => operator = ArithmeticOperators::Addition(-50, 100),
		1 => operator = ArithmeticOperators::Addition(-3, 50),
		2 => operator = ArithmeticOperators::Addition(-3, 50),
		_ => ()
	}
	AskQuestion(operator, &narrator)
   	}
}
#[derive(Copy, Clone)]
enum ArithmeticOperators
{
	Addition(i16, i16),
	Multiplication(i16, i16),
	Division(i16, i16)
}

fn AskQuestion(operator : ArithmeticOperators, narrator : &GTTSClient)
{
	let mut randGen = rand::thread_rng();
	let mut answer : i16 = 0;
	match operator
	{ 
		ArithmeticOperators::Addition(numsMin, numsMax) => 
		{
			let numOne = randGen.gen_range(numsMin..numsMax);
			let numTwo = randGen.gen_range(numsMin..numsMax);
			
			if randGen.gen_range(0u8..1) == 0
			{
				narrator.speak(&format!("{numOne} plus {numTwo}"));
				answer = numOne + numTwo;
			}
			else 
			{
				narrator.speak(&format!("{numOne} minus {numTwo}"));
				answer = numOne - numTwo;
			}
			
			
		}
		ArithmeticOperators::Multiplication(numsMin, numsMax) =>
		{
			let numOne = randGen.gen_range(numsMin..numsMax);
			let numTwo = randGen.gen_range(numsMin..numsMax);
			narrator.speak(&format!("{numOne} times {numTwo}"));
			answer = numOne * numTwo;
		}
		ArithmeticOperators::Division(numsMin, numsMax) =>
		{
			let numOne = randGen.gen_range(numsMin..numsMax);
			let numTwo = randGen.gen_range(numsMin..numsMax);
			answer = numOne * numTwo;
			narrator.speak(&format!("{answer} divided by {numTwo}"));
			answer = numOne;
		}
		
		_ => println!("Not implemented error"),
	}
	waitForInput();
	narrator.speak(&format!("{answer}"))





}

fn waitForInput() -> Result<()> {
    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Up.into()) {
                break;
            }

            /*if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }*/
        } else {
            // Timeout expired, no event for 1s
            println!("Waiting\r");
        }
    }

    Ok(())
}