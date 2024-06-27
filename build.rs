#![allow(unused)]

use std::{ cmp::Ordering, fmt::Write, fs::File, io::{ Read, Seek, Write as ioWrite } };

use indoc::indoc;

extern crate indoc;
extern crate anyhow;
extern crate prost_build;
extern crate protobuf_src;

#[derive(Debug)]
enum Comet {
    Login,
    Gate,
    Scene,
}

fn get_progress_for(comet: Comet) -> anyhow::Result<(Comet, Vec<Vec<String>>)> {
    let module = match comet {
        Comet::Login => "src/server/login/mod.rs",
        Comet::Gate => "src/server/gate/mod.rs",
        Comet::Scene => "src/server/scene/mod.rs",
    };

    let mut content = String::new();
    File::open(module)?.read_to_string(&mut content)?;

    let match_branch = content.splitn(2, "match para_cmd {").skip(1).take(1).next().unwrap();
    let match_arms = match_branch
        .splitn(2, "_ => unreachable!()")
        .take(1)
        .next()
        .unwrap()
        .lines()
        .filter_map(|x| {
            if x.trim().is_empty() || x.trim().starts_with("//") { None } else { Some(x.trim()) }
        })
        .map(|x|
            x
                .trim_end_matches(",")
                .split("=>")
                .map(|x| x.trim().to_owned())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    Ok((comet, match_arms))
}

#[rustfmt::skip]
fn progress_to_tables(progress: Vec<(Comet, Vec<Vec<String>>)>) -> String {
    let mut tables = String::new();

    for comet in progress {
        let (comet, mut row) = comet;
        row.sort_by(|a, b| {
            let result = if b[1] == "todo!()" { 0 } else { 1 } - if a[1] == "todo!()" { 0 } else { 1 };
            
            match result {
                0 => Ordering::Equal,
                -1 => Ordering::Less,
                1 => Ordering::Greater,
                _ => unreachable!()
            }
        });

        let completion = row
            .iter()
            .filter(|x| x[1] != "todo!()")
            .count();

        let mut row_table = String::new();
        row_table.write_str(
            indoc! {"
                | Method | Implemented |
                | --------- | ------- |
            "}
        );

        for method in &row {
            row_table.write_str(&format!(
                "| {} | {} |\n",
                method[0],
                if method[1] == "todo!()" { "❌" } else { "✅" }
            ));
        }

        row_table.write_str("\n\n");
        tables.write_str(
            &format!(
                indoc! {"
                    ### Comet::{:?} ({:.0}% done)

                    {}
                "},
                comet,
                ((completion as f32) / (row.len() as f32)) * 100.0,
                row_table
            )
        );
    }

    tables
}

fn add_progress_to_readme() -> anyhow::Result<()> {
    let readme = File::options().read(true).write(true).open("README.md");

    // We don't want the code to fail compilation just because the README cannot be opened.
    if readme.is_err() {
        return Ok(());
    }

    let mut readme = readme.unwrap();
    let mut buffer = String::new();

    let _ = readme.read_to_string(&mut buffer);
    let _ = readme.seek(std::io::SeekFrom::Start(0))?;

    let parts: Vec<&str> = buffer.splitn(2, "<!-- progress-start -->").collect();
    let (prefix, buffer) = (parts[0], parts[1]);

    let parts: Vec<&str> = buffer.splitn(2, "<!-- progress-end -->").collect();
    let suffix = parts[1];

    let login = get_progress_for(Comet::Login)?;
    let gate = get_progress_for(Comet::Gate)?;
    let scene = get_progress_for(Comet::Scene)?;

    readme.write(prefix.to_string().as_bytes());
    readme.write(b"<!-- progress-start -->\n");
    readme.write(b"## Progress\n\n");
    readme.write(progress_to_tables(vec![login, gate, scene]).as_bytes());
    readme.write(b"\n<!-- progress-end -->\n");
    readme.write(suffix.as_bytes());

    Ok(())
}

fn main() {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    prost_build
        ::compile_protos(
            &[
                "src/proto/cometGate.proto",
                "src/proto/cometLogin.proto",
                "src/proto/cometScene.proto",
            ],
            &["src/"]
        )
        .unwrap();

    add_progress_to_readme().unwrap();
}
