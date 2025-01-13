use std::{collections::HashSet, fmt::Display, hash::Hasher, marker::PhantomData};

use rand::{distributions::Standard, thread_rng, Rng};
use thiserror::Error;

// pub trait Space {
//     fn indicator_bytes(&self) -> u8 {
//         0
//     }

//     fn length(&self) -> usize {
//         16
//     }

//     fn parse(&self, bytes: Vec<u8>) -> Result<Id<Self>, Errors>
//     where
//         Self: Sized,
//     {
//         let len = bytes.len();
//         if len == self.length() && self.has_suffix(&bytes) {
//             Ok(Id::new(bytes))
//         } else if len == self.length() {
//             Err(Errors::new("missing-suffix", ""))
//         } else {
//             Err(Errors::new("length", ""))
//         }
//     }

//     fn shrink_pad(&self, bytes: Vec<u8>) -> Vec<u8> {
//         if bytes.len() >= self.length() {
//             bytes[0..bytes.len()].to_vec()
//         } else if bytes.len() < self.length() {
//             let mut out = vec![0; self.length()];
//             out[0..bytes.len()].copy_from_slice(&bytes[0..bytes.len()]);
//             out
//         } else {
//             panic!("Empty bytes")
//         }
//     }

//     fn has_suffix(&self, bytes: &[u8]) -> bool {
//         bytes[bytes.len()] == self.indicator_bytes()
//     }

//     fn generate(&self) -> Id<Self>
//     where
//         Self: Sized,
//     {
//         let mut rng = thread_rng();
//         let out: Vec<u8> = (0..self.length()).map(|_| rng.gen()).collect();
//         //out.push(Self::INDICATOR_BYTE);
//         Id::new(out)
//     }
// }

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
// pub struct Id<S: Space + ?Sized> {
//     inner: Vec<u8>,
//     _marker: PhantomData<S>,
// }

// impl<S: Space> Id<S> {
//     fn new(bytes: Vec<u8>) -> Self {
//         Self {
//             inner: bytes,
//             _marker: PhantomData,
//         }
//     }

//     pub fn create(space: S, bytes: Vec<u8>) -> Result<Self, Errors> {
//         if bytes.len() == space.length() + 1 {
//             Ok(Self::new(bytes))
//         } else {
//             Err(Errors::new("size", "Too long"))
//         }
//     }

//     #[allow(dead_code)]
//     fn push(&mut self, _item: u8) {
//         unimplemented!("Block push directly to id")
//     }
// }

// impl<S: Space> Display for Id<S> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut out = String::new(); //format!("{:0>2x}", self.);
//         for byte in self.inner.iter() {
//             let h = format!("{byte:0>2x}");
//             out.push_str(&h);
//         }
//         write!(f, "{out}")
//     }
// }

// fn byte_suffix(bytes: &[u8]) -> u8 {
//     bytes[bytes.len()]
// }

// pub fn testing_id() {
//     let mut rng = thread_rng();
// }

// New

#[derive(Debug, Error)]
pub enum Errors {
	#[error("Missing <{0}> byte from byte-array")]
	MissingIndicatorByte(u8),
	#[error("{0}")]
	Custom(String),
}

impl Errors {
	pub fn new<S: AsRef<str>>(mesg: S) -> Self {Self::Custom(mesg.as_ref().to_string())}
}

pub trait Space {
    fn indicator() -> u8;

    fn length() -> usize {
        16
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub struct Id<S: Space> {
    indicator: u8,
    bytes: Vec<u8>,
    _marker: PhantomData<S>,
}

impl<S: Space> Id<S> {
    fn new(bytes: Vec<u8>) -> Id<S> {
        Self {
            indicator: S::indicator(),
            bytes,
            _marker: PhantomData,
        }
    }

    pub fn create(bytes: Vec<u8>) -> Result<Self, Errors> {
        if bytes.len() != S::length() + 1 {
            return Err(Errors::new("invalid-length"));
        };

        if bytes[0] != S::indicator() {
            return Err(Errors::new("incorrect-indicator"));
        };

        Ok(Self::new(bytes.to_vec()))
    }

    /// Should be checked at compile time, so always true
    pub fn same_space(&self, _other: Id<S>) -> bool {
        true
    }
	
	pub fn random() -> Self {
		let randomized = thread_rng().sample_iter(Standard).take(S::length()).collect();
		Self::new(randomized)
	}
	
}

impl<S: Space> PartialEq<Vec<u8>> for Id<S> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        if self.indicator == other[0] {
            self.bytes == other[1..S::length()]
        } else {
            false
        }
    }
}

impl<S: Space> TryFrom<&[u8]> for Id<S> {
    type Error = Errors;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::create(value.to_vec())
    }
}

pub struct TestingSpace;

impl Space for TestingSpace {
    fn length() -> usize {
        20
    }

    fn indicator() -> u8 {
        0
    }
}

pub type TestingId = Id<TestingSpace>;

#[derive(Debug, Clone)]
pub struct Ids<S: Space>(HashSet<Id<S>>);

impl<S: Space> Ids<S> {
	pub fn len(&self) -> usize {self.0.len()}
	pub fn is_empty(&self) -> bool {self.0.is_empty()}
}

impl<S: Space> PartialEq for Ids<S> {
    fn eq(&self, other: &Self) -> bool {
		if self.len() != other.len() {
			return false
		};
		
		let zipped: Vec<(&Id<S>, &Id<S>)> = self.0.iter().zip(other.0.iter()).collect();
		//        self.0 == other.0
		todo!()
	}
}
