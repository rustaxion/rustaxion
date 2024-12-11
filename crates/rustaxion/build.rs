#![allow(unused)]

use std::{cmp::Ordering, fmt::Write, fs};

extern crate anyhow;
extern crate prost_build;
extern crate protobuf_src;

fn main() {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let proto = prost_build::compile_protos(
        &[
            "src/proto/cometGate.proto",
            "src/proto/cometLogin.proto",
            "src/proto/cometScene.proto",
        ],
        &["src/backend/"],
    );

    match proto {
        Ok(_) => {}
        Err(err) => {
            let inner = err.into_inner().unwrap();
            eprintln!("Failed to compile proto files: {}", inner);
            std::process::exit(1);
        }
    };

    add_progress_to_readme();
}

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

    let content = fs::read_to_string(module)?;

    let match_branch = content
        .splitn(2, "match para_cmd {")
        .skip(1)
        .take(1)
        .next()
        .unwrap();
    let match_arms = match_branch
        .splitn(2, "_ => unreachable!()")
        .take(1)
        .next()
        .unwrap()
        .lines()
        .filter_map(|x| {
            if x.trim().is_empty() || x.trim().starts_with("//") {
                None
            } else {
                Some(x.trim())
            }
        })
        .map(|x| {
            x.trim_end_matches(",")
                .split("=>")
                .map(|x| x.trim().to_owned())
                .collect::<Vec<_>>()
        })
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
            indoc::indoc! {"
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

        tables.write_str(
            &format!(
                indoc::indoc! {"
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
    let buffer = fs::read_to_string("../../README.md")?;

    let start = "<!-- progress-start -->";
    let end = "<!-- progress-end -->";

    let prefix = &buffer[..buffer.find(start).unwrap()];
    let suffix = &buffer[buffer.find("<!-- progress-end -->").unwrap() + end.len()..];

    let login = get_progress_for(Comet::Login).unwrap_or((Comet::Login, vec![]));
    let gate = get_progress_for(Comet::Gate).unwrap_or((Comet::Gate, vec![]));
    let scene = get_progress_for(Comet::Scene).unwrap_or((Comet::Scene, vec![]));

    let mut output = String::new();

    output.write_str(prefix);
    output.write_str("<!-- progress-start -->\n## Progress\n\n");
    output.write_str(progress_to_tables(vec![login, gate, scene]).as_str());
    output.write_str("<!-- progress-end -->");
    output.write_str(suffix);

    fs::write("../../README.md", output)?;

    Ok(())
}
