use std::path::Path;

#[derive(Debug)]
enum Language {
    JS,
    GO,
    RUST,
    CPP,
    PYTHON,
    UNKNOWN,
}
fn detect_languages() -> Language {
    if Path::new("Cargo.toml").exists() {
        return Language::RUST;
    } else if Path::new("package.json").exists() {
        return Language::JS;
    } else if Path::new("go.mod").exists() {
        return Language::GO;
    } else if Path::new("requirements.txt").exists() {
        return Language::PYTHON;
    } else if Path::new("CMakeLists.txt").exists() {
        return Language::CPP;
    }
    return Language::UNKNOWN;
}
fn main() {
    let docs = match detect_languages(){
        Language::CPP => "https://cppreference.com/?__cf_chl_f_tk=Fex81taMt0rvdaZeeZspdSOwclS2gC3c6HtD37v.xwc-1782991617-1.0.1.1-DPy.rbz61vA8NK6jDRF2hQoQYicsaR2G55n1BP7tk.Q",
        Language::GO => "https://pkg.go.dev/",
        Language::PYTHON => "https://docs.python.org/3/",
        Language::JS => "https://developer.mozilla.org/en-US/docs/Web/JavaScript",
        Language::RUST => "https://doc.rust-lang.org/stable/std/",
        Language::UNKNOWN => {
            eprintln!("Could not confidently detect the project language.");
            return;
        }

    };

    println!("Opening docs: {}", docs);

    if let Err(e) = open::that(docs) {
        eprintln!("Failed to open browser: {}", e);
    }
}
