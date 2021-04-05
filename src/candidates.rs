use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Candidates(i16);
impl Default for Candidates {
	fn default() -> Self {
		Self(0x1FF)
	}
}
impl Candidates {
	// v:    1 2 3 4  5  6  7   8   9
	// 1<<v: 1 2 4 8 16 32 64 128 256
	// %11:  1 2 4 8  5 10  9   7   3
	const DIGITS: [char; 11] = ['X', '1', '2', '9', '3', '5', 'X', '8', '4', '7', '6'];

	pub fn new() -> Self {
		Default::default()
	}

	pub fn is_valid(&self) -> bool {
		self.0 != 0
	}
	pub fn is_done(&self) -> bool {
		// if only one bit is 1
		(self.0 & (self.0 - 1)) == 0
	}
	pub fn len(&self) -> usize {
		// counting 1s' of the value
		let mut v = self.0;
		let mut count = 0;
		while v != 0 {
			v &= v - 1;
			count += 1;
		}
		count
	}
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
	pub fn iter(&self) -> Iter {
		Iter { v: *self }
	}
	pub fn pop(&mut self) -> Option<Candidates> {
		if self.0 == 0 {
			None
		} else {
			// extract the last 1
			let v = self.0 & -self.0;
			self.0 ^= v;
			Some(Self(v))
		}
	}
	pub fn lucky(&self) -> char {
		Self::DIGITS[(self.0 % 11) as usize]
	}

	pub fn substract(&mut self, cs: &Candidates) {
		self.0 &= !cs.0;
	}

	pub fn union(c1: Candidates, c2: Candidates, c3: Candidates) -> Self {
		Candidates(c1.0 | c2.0 | c3.0)
	}
}

pub struct Iter {
	v: Candidates,
}
impl Iterator for Iter {
	type Item = Candidates;
	fn next(&mut self) -> Option<Self::Item> {
		self.v.pop()
	}
}

use std::fmt::Write; // for `write_char`
impl fmt::Display for Candidates {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_done() {
			f.write_char(self.lucky())?;
		} else if self.is_valid() {
			let mut v = 1i16;
			for i in 1..=9 {
				if (v & self.0) != 0 {
					f.write_char((i + b'0') as char)?;
				}
				v *= 2;
			}
		} else {
			f.write_str("X")?;
		}
		Ok(())
	}
}
impl fmt::Debug for Candidates {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(&self, f)
	}
}
impl From<char> for Candidates {
	fn from(v: char) -> Self {
		if ('1'..='9').contains(&v) {
			Self(1 << (v as u8 - b'1'))
		} else {
			Default::default()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let cs = Candidates::new();
		assert_eq!("123456789", cs.to_string());
		assert_eq!(true, cs.is_valid());
		assert_eq!(false, cs.is_done());
		assert_eq!(9, cs.len());

		let cs = Candidates::from('5');
		assert_eq!("5", cs.to_string());
		assert_eq!(true, cs.is_valid());
		assert_eq!(true, cs.is_done());
		assert_eq!(1, cs.len());
		let a = cs
			.iter()
			.map(|c| c.to_string())
			.collect::<Vec<_>>()
			.join("");
		assert_eq!("5", a);

		let mut cs = Candidates::from('.');
		assert_eq!("123456789", cs.to_string());
		assert_eq!(9, cs.len());

		let p = cs.pop().unwrap();
		assert_eq!("1", p.to_string());
		assert_eq!('1', p.lucky());
		let a = cs
			.iter()
			.map(|c| c.to_string())
			.collect::<Vec<_>>()
			.join("");
		assert_eq!("23456789", a);
	}

	#[test]
	fn substract() {
		let mut cs = Candidates::new();
		let mut cs2 = Candidates::new();
		cs.substract(&cs2);
		assert_eq!("X", cs.to_string());
		assert_eq!(false, cs.is_valid());

		let mut cs = Candidates::new();
		cs2.pop();
		cs.substract(&cs2);
		assert_eq!("1", cs.to_string());
	}

	#[test]
	fn pop() {
		let mut cs = Candidates::new();
		assert_eq!("1", cs.pop().unwrap().to_string());
		assert_eq!("2", cs.pop().unwrap().to_string());
		assert_eq!("3", cs.pop().unwrap().to_string());
		assert_eq!("4", cs.pop().unwrap().to_string());
		assert_eq!("5", cs.pop().unwrap().to_string());
		assert_eq!("6", cs.pop().unwrap().to_string());
		assert_eq!("7", cs.pop().unwrap().to_string());
		assert_eq!("8", cs.pop().unwrap().to_string());
		assert_eq!("9", cs.pop().unwrap().to_string());
		assert_eq!(None, cs.pop());

		assert_eq!(false, cs.is_valid());
	}
}
