//! Build the skill index from the installed game and report what came out.
//!
//!     cargo run --release --example skilldb
//!
//! The Rust parser has to agree with `tools/arz_probe.py`, which is the
//! reference the format was pinned down against. Run both after touching
//! either.

use std::path::PathBuf;

fn main() {
    let root = game_root().unwrap_or_else(|| {
        eprintln!("Grim Dawn not found; pass its directory as an argument");
        std::process::exit(1);
    });
    println!("game root: {}", root.display());

    let files = grimlua_core::db::locate(&root);
    println!("\narchives, in load order:");
    for a in &files.archives {
        println!("  {}", a.display());
    }
    println!("text:");
    for t in &files.text {
        println!("  {}", t.display());
    }

    let index = grimlua_core::db::build(&files);
    println!("\n{} skills in {} ms", index.skills.len(), index.built_ms);

    let castable = index.skills.iter().filter(|s| s.castable).count();
    println!("  castable {castable}, passive/other {}", index.skills.len() - castable);

    // The three skills Option A's mockup names, end to end.
    println!("\nthe skills the design mocks up:");
    for want in ["Blood of Dreeg", "Wendigo Totem", "Pneumatic Burst", "Mogdrogen's Pact"] {
        match index.skills.iter().find(|s| s.name == want) {
            Some(s) => println!(
                "  {:<18} {:<26} castable={} cd={:?} dur={:?}\n      {}",
                s.name, s.class, s.castable, s.cooldown, s.duration, s.path
            ),
            None => println!("  {want:<18} NOT FOUND"),
        }
    }

    println!("\nby mastery:");
    for m in 1..=10u32 {
        let n = index.skills.iter().filter(|s| s.mastery == Some(m)).count();
        let c = index
            .skills
            .iter()
            .filter(|s| s.mastery == Some(m) && s.castable)
            .count();
        if n > 0 {
            println!("  mastery {m:>2}: {n:>3} skills, {c:>3} castable");
        }
    }
    let items = index.skills.iter().filter(|s| s.mastery.is_none()).count();
    println!("  item/default: {items}");

    println!("\na sample of what the picker will show:");
    for s in index.castable().take(12) {
        println!("  [{}] {:<28} {}", s.mastery.map(|m| m.to_string()).unwrap_or("-".into()), s.name, s.class);
    }

    // Anything still showing a raw tag means the text archive did not cover it.
    let unresolved: Vec<&str> = index
        .skills
        .iter()
        .filter(|s| s.name.starts_with("tag"))
        .map(|s| s.name.as_str())
        .take(8)
        .collect();
    println!("\nunresolved name tags: {}", if unresolved.is_empty() {
        "none".to_string()
    } else {
        format!("{} e.g. {unresolved:?}", unresolved.len())
    });
}

fn game_root() -> Option<PathBuf> {
    if let Some(arg) = std::env::args().nth(1) {
        return Some(PathBuf::from(arg));
    }
    for base in [
        r"C:\Program Files (x86)\Steam\steamapps\common\Grim Dawn",
        r"C:\Program Files\Steam\steamapps\common\Grim Dawn",
    ] {
        let p = PathBuf::from(base);
        if p.join("database").join("database.arz").is_file() {
            return Some(p);
        }
    }
    None
}
