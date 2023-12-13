use aoc_lib::input_reader;
use std::env;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut run_all = false;

    if args.len() > 1 && (args[1] == "a" || args[1] == "all") {
        run_all = true;
    }
    day12::run(input("12"));
    if run_all {
        day1::run(input("1"));
        day2::run(input("2"));
        day3::run(input("3"));
        day4::run(input("4"));
        day5::run(input("5"));
        day6::run();
        day7::run(input("7"));
        day8::run(input("8"));
        day9::run(input("9"));
        day10::run(input("10"));
        day11::run(input("11"));
        //day13::run(input("13"));
        //day14::run(input("14"));
        //day15::run(input("15"));
        //day16::run(input("16"));
        //day17::run(input("17"));
        //day18::run(input("18"));
        //day19::run(input("19"));
        //day20::run();
        //day21::run();
        //day22::run();
        //day23::run(input("23"));
        //day24::run(input("24"));
        //day25::run();
    }
}

fn input(day: &str) -> String {
    input_reader::get_input("2023", day, "./cookie.txt")
}
