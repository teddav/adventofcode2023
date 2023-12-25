pub mod day14;
pub mod day9;
pub mod day7b;
pub mod day10;
pub mod day11;
pub mod day25;
pub mod day13a;
pub mod day20b;
pub mod day12a;
pub mod day21;
pub mod day15;
pub mod day8;
pub mod day3;
pub mod day6;
pub mod day2;
pub mod day18;
pub mod day5;
pub mod day1;
pub mod day19;
pub mod day4;
pub mod day20a;
pub mod day13b;
pub mod day12b;
pub mod day22;
pub mod day16;
pub mod day17;
pub mod day7a;

#[path = "day25.rs"]
pub mod latest;

pub const days_main: &[(&str, fn())] = &[
	("14", day14::main),
	("9", day9::main),
	("7b", day7b::main),
	("10", day10::main),
	("11", day11::main),
	("25", day25::main),
	("13a", day13a::main),
	("20b", day20b::main),
	("12a", day12a::main),
	("21", day21::main),
	("15", day15::main),
	("8", day8::main),
	("3", day3::main),
	("6", day6::main),
	("2", day2::main),
	("18", day18::main),
	("5", day5::main),
	("1", day1::main),
	("19", day19::main),
	("4", day4::main),
	("20a", day20a::main),
	("13b", day13b::main),
	("12b", day12b::main),
	("22", day22::main),
	("16", day16::main),
	("17", day17::main),
	("7a", day7a::main),
];