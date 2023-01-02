// using the question mark operator to handle Result statuses.
//'static str -> will return a static string
fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("Status text is too long!")
    }

    // the question mark operator was build to solve for this pattern
    // the match below can be replaced with a single line.
    /* let record = match save_to_database(text) {
        Ok(rec) => rec,
        Err(e) => return Err(e),
    }; */
    let record = save_to_database(text)?;

    Ok(record.id)
}

fn save_to_database(text: &str)  -> Result<StatusRecord, &'static str> {
    // fake db implementation that always returns an error
    Err("Database is not available!")
}

struct StatusRecord {
    id: i64,
    text: String,
    created_at: std::time::Instant,
}

fn main() {
    println!("{:?}", save_status("Boo!"));
}
