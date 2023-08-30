use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
};

use gray_matter::{engine::TOML, Matter};
use regex::{Captures, Regex};
use serde::Deserialize;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum FrontMatter {
    Project { title: String, url: String },
}

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    generate_latex()?;

    generate_readme()?;

    Ok(())
}

fn generate_latex() -> anyhow::Result<()> {
    let matter = Matter::<TOML>::new();

    let prefix = Path::new("../markdown");

    for entry in WalkDir::new(prefix).into_iter() {
        let entry = entry?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        println!("{entry:#?}");

        let mut file = File::open(entry.path()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let result = matter.parse(&contents);
        let content = format!(
            "\\begin{{markdown}}\n{}\n\\end{{markdown}}\n",
            result.content
        );
        let contents = if let Some(data) = result.data {
            let front_matter: FrontMatter = data.deserialize().expect("Invalid front matter");

            match front_matter {
                FrontMatter::Project { title, url } => {
                    format!(
                        "\\cvsect{{{title}}}\n\n{content}\n\n\\vspace{{3pt}}\n\n\\rurl{{{url}}}\n"
                    )
                }
            }
        } else {
            content
        };

        let path = entry.path().strip_prefix(prefix)?.with_extension("tex");
        let path = Path::new("../latex").join(path);
        create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;
    }

    let private = format!(
        "\\newcommand{{\\city}}{{{}}}\n\\newcommand{{\\phone}}{{{}}}\n\\newcommand{{\\email}}{{{}}}\n",
        std::env::var("CITY")?,
        std::env::var("PHONE")?,
        std::env::var("EMAIL")?,
    );
    File::create("../latex/private.tex")?.write_all(private.as_bytes())?;

    Ok(())
}

fn generate_readme() -> anyhow::Result<()> {
    let readme = "../markdown/README.md";
    let base_path = Path::new(readme).parent().expect("File should have parent");

    let mut file = File::open(readme)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let matter = Matter::<TOML>::new();

    let re = Regex::new(r"\#\{(.*)\}").expect("Regex should be valid");
    let contents = re.replace_all(&contents, |caps: &Captures| {
        let path = caps.get(1).expect("Capture group should exist").as_str();
        println!("Including '{path}' in README.md");

        let mut file = File::open(base_path.join(path)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let result = matter.parse(&contents);
        if let Some(data) = result.data {
            let front_matter: FrontMatter = data.deserialize().expect("Invalid front matter");

            match front_matter {
                FrontMatter::Project { title, url } => {
                    format!("### [{title}](https://{url})\n{}\n", result.content)
                }
            }
        } else {
            format!("{}\n", result.content)
        }
    });

    let mut file = File::create("../README.md")?;
    file.write_all(contents.as_bytes()).unwrap();

    Ok(())
}
