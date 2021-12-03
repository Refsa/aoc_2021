use crate::Runner;
use byteorder::*;
use std::rc::Rc;

pub struct AOC3 {}

impl AOC3 {
	pub fn parse(input: &Vec<String>) -> (usize, Vec<u32>) {
		(
			input[0].len(),
			input
				.iter()
				.map(|e| u32::from_str_radix(&e.chars().collect::<String>(), 2).unwrap())
				.collect(),
		)
	}
}

impl Runner for AOC3 {
	fn run_p1(&self, input: &std::vec::Vec<std::string::String>) -> usize {
		let (len, input) = Self::parse(input);
		let input = Rc::new(input);

		let cnt = input.iter().fold(vec![0isize; len], |mut acc, e| {
			for i in 0..len {
				if bit_set(*e, i as u32) {
					acc[i] += 1;
				} else {
					acc[i] -= 1;
				}
			}
			acc
		});

		let mut gamma = 0isize;
		let mut epsilon = 0isize;

		for i in 0..len {
			let e = cnt[i];
			if e > 0 {
				gamma |= 1 << i;
			} else {
				epsilon |= 1 << i;
			}
		}

		(gamma * epsilon) as usize
	}

	fn run_p2(&self, input: &std::vec::Vec<std::string::String>) -> usize {
		let (len, input) = Self::parse(input);

		let mut oxy_i = input.iter().collect::<Vec<&u32>>();
		for bc in (0..len).rev() {
			let cnt = count_bits_p(&oxy_i, bc as u32);
			let c = if cnt >= 0 { 1 } else { 0 };
			
			for i in (0..oxy_i.len()).rev() {
				let set = btoi(bit_set(*oxy_i[i], bc as u32));

				if c != set {
					oxy_i.remove(i);
				}

				if oxy_i.len() == 1 {
					break;
				}
			}

			if oxy_i.len() == 1 {
				break;
			}
		}
		let oxy = oxy_i[0];
		// println!("{} - {:?}", oxy_i.len(), oxy);

		let mut oxy_i = input.iter().collect::<Vec<&u32>>();
		for bc in (0..len).rev() {
			let cnt = count_bits_p(&oxy_i, bc as u32);
			let c = if cnt >= 0 { 1 } else { 0 };
			
			for i in (0..oxy_i.len()).rev() {
				let set = btoi(bit_set(*oxy_i[i], bc as u32));

				if c == set {
					oxy_i.remove(i);
				}

				if oxy_i.len() == 1 {
					break;
				}
			}

			if oxy_i.len() == 1 {
				break;
			}
		}
		let c02 = oxy_i[0];
		// println!("{} - {:?}", oxy_i.len(), oxy);

		(oxy * c02) as usize
	}
}

fn count_bits_p(input: &Vec<&u32>, pos: u32) -> isize {
	let cnt = input.iter().fold((0isize, 0isize), |acc, &&e| {
		if bit_set(e, pos) {
			(acc.0 + 1, acc.1)
		} else {
			(acc.0, acc.1 + 1)
		}
	});
	cnt.0 - cnt.1
}

fn btoi(val: bool) -> isize {
	if val {
		1
	} else {
		0
	}
}

fn bit_set(value: u32, bit: u32) -> bool {
	value & (1 << bit) != 0
}

mod tests {
	#[test]
	pub fn bit_set() {
		let value = 45u32;

		assert_eq!(true, super::bit_set(value, 0u32));
		assert_eq!(true, super::bit_set(value, 2u32));
		assert_eq!(true, super::bit_set(value, 3u32));
		assert_eq!(true, super::bit_set(value, 5u32));
	}
}
