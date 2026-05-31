use mail_builder::{MessageBuilder, headers::address::Address};
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a collection of your messages
    let mut messages = Vec::new();

    let msg1 = MessageBuilder::new()
        .from(Address::new_address("Alice".into(), "alice@example.com"))
        .to(Address::new_address("Bob".into(), "bob@example.com"))
        .subject("First Message")
        .text_body("Hello Bob, this is message one.");
    messages.push(("alice@example.com", msg1));

    let msg2 = MessageBuilder::new()
        .from(Address::new_address("Bob".into(), "bob@example.com"))
        .to(Address::new_address("Alice".into(), "alice@example.com"))
        .subject("Second Message")
        .text_body("Hey Alice, this is message two!");
    messages.push(("bob@example.com", msg2));

    // 2. Open or create your mbox target file in append mode
    let mut mbox_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("test1.mbox")?;

    // 3. Serialize and append each message with the required mbox separator
    for (sender, builder) in messages {
        // Generate a valid mbox timestamp (simplified to local/system or fake time)
        // Format: Day Mon DD HH:MM:SS YYYY
        let mbox_timestamp = "Sun May 31 10:06:00 2026";

        // Write the critical "From " envelope delimiter line
        writeln!(mbox_file, "From {} {}", sender, mbox_timestamp)?;

        // Pipe the mail-builder byte output directly into your file stream
        builder.write_to(&mut mbox_file)?;

        // Ensure there is an extra trailing newline following the message block
        writeln!(mbox_file)?;
    }

    Ok(())
}
