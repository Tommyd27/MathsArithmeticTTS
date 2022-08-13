#![allow(non_snake_case)]
use rand::Rng;
use tts_rust::{ GTTSClient, languages::Languages };
use std::time::Duration;
use std::thread::sleep;


use winapi::{self, um::winuser::GetKeyState};
fn main() {
    println!("Hell&o, world!");

	let narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
    };
	let mut randGen = rand::thread_rng();
   	let mut operator = ArithmeticOperators::Addition(1, 1);
	loop 
   	{
		match randGen.gen_range(0u8..3)
		{

			0 => operator = ArithmeticOperators::Addition(-50, 100),
			1 => operator = ArithmeticOperators::Multiplication(-3, 50),
			2 => operator = ArithmeticOperators::Division(-3, 50),
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
			
			if randGen.gen_range(0u8..2) == 0
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
			let numTwo = randGen.gen_range(-9..10);
			answer = numOne * numTwo;
			narrator.speak(&format!("{answer} divided by {numTwo}"));
			answer = numOne;
		}
		
		_ => println!("Not implemented error"),
	}
	WaitForInput();
	narrator.speak(&format!("{answer}"));
	WaitForInput();





}

fn WaitForInput()
{
	unsafe
	{
		let mut state = GetKeyState(0x26);
		while state != -127 && state != -128
		{
			sleep(Duration::new(0, 12500000));
			state = GetKeyState(0x26);
		}
	}
	
}
