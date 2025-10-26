use std::{ sync::{ atomic::{ AtomicU64, Ordering }, Once }, time::{ SystemTime, UNIX_EPOCH } };



const CONST_RAND_NUMBER:u64 = 0x243F6A8885A308D3;
const DEFAULT_SEED_ON_SEEDING_ERROR:u64 = 0xDA3E39CB94B95BDB;
const HASH_MIXING_GOLDEN_RATIO:u64 = 0x9E3779B97F4A7C15;



static INIT:Once = Once::new();
static VALUE_MODIFIER:AtomicU64 = AtomicU64::new(0);



/// One-time initialize the global state with a seed that mixes current time and a stack address.
fn init_state() {

	// First 'random' seed-part, current time.
	let current_time_nanoseconds:u64 = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0);

	// Second 'random' seed-part, address of random variable.
	let random_variable:u8 = 0u8;
	let random_variable_address:u64 = (&random_variable as *const u8 as usize) as u64;

	// Create a seed based on all 'random' seed parts.
	let seed:u64 = current_time_nanoseconds ^ random_variable_address.wrapping_mul(HASH_MIXING_GOLDEN_RATIO) ^ CONST_RAND_NUMBER;

	// Use the seed as the initial version of the value modifier.
	VALUE_MODIFIER.store(if seed == 0 { DEFAULT_SEED_ON_SEEDING_ERROR } else { seed }, Ordering::Relaxed);
}

/// Generate a random u64. Modifies the state to make sure the same number isn't returned again.
pub(crate) fn generate_random_u64() -> u64 {
	
	// Ensure the seed is created once.
	INIT.call_once(init_state);

	// Increment value modifier to next value.
	let old_value:u64 = VALUE_MODIFIER.fetch_add(HASH_MIXING_GOLDEN_RATIO, Ordering::Relaxed);

	// Random mixing of value.
	let mut generated_value:u64 = old_value.wrapping_add(HASH_MIXING_GOLDEN_RATIO);
	generated_value = generated_value.wrapping_add(HASH_MIXING_GOLDEN_RATIO); // ensure nontrivial mixing in case old was small
	generated_value = (generated_value ^ (generated_value >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
	generated_value = (generated_value ^ (generated_value >> 27)).wrapping_mul(0x94D049BB133111EB);
	generated_value ^ (generated_value >> 31)
}