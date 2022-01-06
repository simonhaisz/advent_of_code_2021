use std::collections::HashMap;

pub fn dice_states() -> HashMap<u16, u64> {
	let mut states = HashMap::new();
	
	states.insert(3, 1);
	states.insert(4, 3);
	states.insert(5, 6);
	states.insert(6, 7);
	states.insert(7, 6);
	states.insert(8, 3);
	states.insert(9, 1);

	states
}