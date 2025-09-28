pub mod character_filter;
pub mod model;
pub mod pre_tokenizer;
pub mod text_analyzer;
pub mod token_filter;
pub mod tokenizer;
pub mod utils;

::pgrx::pg_module_magic!();

#[cfg(not(all(target_endian = "little", target_pointer_width = "64")))]
compile_error!("Target is not supported.");

#[cfg(not(any(
    feature = "pg13",
    feature = "pg14",
    feature = "pg15",
    feature = "pg16",
    feature = "pg17",
    feature = "pg18"
)))]
compile_error!("PostgreSQL version must be selected.");

#[pgrx::pg_guard]
extern "C-unwind" fn _PG_init() {
    if unsafe { pgrx::pg_sys::IsUnderPostmaster } {
        pgrx::error!("pg_tokenizer must be loaded via shared_preload_libraries.");
    }

    pre_tokenizer::init();
    model::init();
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![r#"search_path = '"$user", public, tokenizer_catalog'"#]
    }
}
