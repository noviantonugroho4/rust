# rust

# build development
cargo run

# build production
cargo build --release

# run test
cargo test
cargo test nama_test_function -- --exact
cargo test -- --nocapture



# integer type
signed = bisa negatif (i64)
unisign = positif saja (u64)
float = (f32)


# management memory
stack (tumpukan = lifo) & heap (string pakai clone untuk copy value)

# error handling
- unrecoverable error
harus di fix / lebih baik matikan aplikasi (panic)