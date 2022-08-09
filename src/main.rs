use std::{collections::HashMap, env, process::exit};

fn main() {
    let icons = HashMap::from([
        ("ack", ""),
        ("atop", ""),
        ("bash", ""),
        ("bat", ""),
        ("cat", ""),
        ("clx", ""),
        ("cp", ""),
        ("curl", ""),
        ("docker", ""),
        ("docker-compose", ""),
        ("duplicate", ""),
        ("exa", "פּ"),
        ("fd", ""),
        ("find", ""),
        ("fish", ""),
        ("fzf", ""),
        ("gh", ""),
        ("git", ""),
        ("glow", ""),
        ("go", ""),
        ("grep", ""),
        ("htop", ""),
        ("http", ""),
        ("java", ""),
        ("lazygit", ""),
        ("less", ""),
        ("lf", "פּ"),
        ("ls", "פּ"),
        ("lua", ""),
        ("lynx", ""),
        ("man", ""),
        ("more", ""),
        ("mv", ""),
        ("nano", "פֿ"),
        ("nnn", "פּ"),
        ("node", ""),
        ("npm", ""),
        ("nvim", ""),
        ("pico", "פֿ"),
        ("pint", ""),
        ("python", ""),
        ("ranger", "פּ"),
        ("rg", ""),
        ("rm", " "),
        ("rsync", ""),
        ("ruby", ""),
        ("scp", ""),
        ("sleep", ""),
        ("spin", ""),
        ("ssh", ""),
        ("sudo", ""),
        ("tail", ""),
        ("tig", ""),
        ("tmux", "﬿"),
        ("top", ""),
        ("vi", ""),
        ("vim", ""),
        ("w3m", ""),
        ("wget", ""),
        ("yarn", ""),
        ("youtube-dl", ""),
        ("zsh", ""),
    ]);

    if let Some(input) = env::args().nth(1) {
        if let Some(icon) = icons.get(input.as_str()) {
            print!("{}", icon)
        }
        exit(0)
    }

    println!("Missing argument: program name");
    exit(1)
}
