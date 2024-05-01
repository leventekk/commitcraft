pub mod conventional;
pub mod raw;

pub trait InstructionStrategy {
	fn inject() -> &'static str;
}
