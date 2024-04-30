pub mod raw;
pub mod conventional;

pub trait InstructionStrategy {
	fn inject() -> &'static str;
}

