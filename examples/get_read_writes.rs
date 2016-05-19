extern crate xfs;

fn main() {
    match xfs::get() {
        Ok(stats) => {
            println!("XFS Stats");
            println!("=========");
            println!("Reads: {}", stats.read_write_stats.read);
            println!("Writes: {}", stats.read_write_stats.write);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
