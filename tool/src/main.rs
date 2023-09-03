use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
};

use gray_matter::{engine::YAML, Matter};
use regex::{Captures, Regex};
use serde::Deserialize;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    generate_readme()?;

    generate_lang("en")?;
    generate_lang("nl")?;

    generate_latex_from_md()?;
    generate_latex_from_yml()?;
    generate_latex_private()?;

    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FrontMatter {
    Project { title: String, url: String },
}

fn generate_latex_from_md() -> anyhow::Result<()> {
    let matter = Matter::<YAML>::new();

    let prefix = Path::new("../markdown");

    for entry in WalkDir::new(prefix).into_iter() {
        let entry = entry?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        let mut file = File::open(entry.path()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        println!("Converting '{entry:?}' into LaTeX");

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

        let path = entry.path().strip_prefix(prefix)?.with_extension("md.tex");
        let path = Path::new("../latex").join(path);
        create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Entry {
    name: String,
    description: String,
    at: String,
    start: isize,
    end: Option<isize>,
}

fn generate_latex_from_yml() -> anyhow::Result<()> {
    let prefix = Path::new("../yaml");

    for entry in WalkDir::new(prefix).into_iter() {
        let entry = entry?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        let mut file = File::open(entry.path()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        println!("Converting '{entry:?}' into LaTeX");

        let data: Vec<Entry> = serde_yaml::from_str(&contents)?;

        let mut content = String::new();
        for entry in data {
            let date = if entry.end.is_none() {
                format!("{} -- \\Now", entry.start)
            } else if entry.start == entry.end.unwrap() {
                format!("{}", entry.start)
            } else {
                format!("{} -- {}", entry.start, entry.end.unwrap())
            };

            content = format!(
                "{content}\\entry\n\t{{{date}}}\n\t{{{}}}\n\t{{{}}}\n\t{{{}}}\n",
                entry.name, entry.at, entry.description
            );
        }

        let path = entry.path().strip_prefix(prefix)?.with_extension("yml.tex");
        let path = Path::new("../latex").join(path);
        create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

fn generate_latex_private() -> anyhow::Result<()> {
    let private = format!(
        "\\newcommand{{\\City}}{{{}}}\n\\newcommand{{\\Phone}}{{{}}}\n\\newcommand{{\\Email}}{{{}}}\n",
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

    let matter = Matter::<YAML>::new();

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

fn generate_lang(lang: &str) -> anyhow::Result<()> {
    let main = Path::new("../main.tex");

    let mut file = File::open(main)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents = contents.replace("{{lang}}", lang);

    let path = main.with_extension(format!("{lang}.tex"));
    println!("Generating '{path:?}'");
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes()).unwrap();

    Ok(())
}
