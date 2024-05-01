pub mod conventional;
pub mod raw;

pub trait InstructionStrategy {
	fn inject(with_description: &bool) -> String;
}
