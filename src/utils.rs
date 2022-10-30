use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub type Pool = r2d2::Pool<redis::Client>;

pub fn generate_hashval() -> String {
    String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .collect::<Vec<u8>>()
            .as_slice()
            .to_ascii_lowercase(),
    )
    .unwrap()
}
