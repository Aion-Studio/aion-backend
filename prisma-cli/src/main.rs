use std::fs;

fn main() {
    prisma_client_rust_cli::run();

    // Read the file into a String
    let path = "src/prisma.rs";
    let content = fs::read_to_string(&path).expect("Failed to read the file");

    // Replace the path in the include_str! macro
    let modified_content = content.replace(
        r#"include_str!("/Users/marko/idle-rpg/aion_server/prisma/schema.prisma")"#,
        r#"include_str!("../prisma/schema.prisma")"#,
    );

    // Write the modified content back to the file
    fs::write(&path, modified_content).expect("Failed to write to the file");
}
