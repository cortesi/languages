use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LanguageYaml {
    #[serde(rename = "type")]
    language_type: String,
    color: Option<String>,
    aliases: Option<Vec<String>>, 
    extensions: Option<Vec<String>>, 
    interpreters: Option<Vec<String>>, 
    group: Option<String>,
    tm_scope: Option<String>,
    ace_mode: Option<String>,
    codemirror_mode: Option<String>,
    codemirror_mime_type: Option<String>,
    language_id: u32,
}

fn optional_string(opt: &Option<String>) -> String {
    match opt {
        Some(s) => format!("Some({:?})", s),
        None => "None".to_string(),
    }
}

fn optional_vec(opt: &Option<Vec<String>>) -> String {
    match opt {
        Some(vec) => {
            let items: Vec<String> = vec.iter().map(|s| format!("{:?}", s)).collect();
            format!("Some(&[{}])", items.join(", "))
        }
        None => "None".to_string(),
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let yaml_content = fs::read_to_string(Path::new("languages.yml"))
        .expect("languages.yml missing; run download_languages.sh");
    let map: BTreeMap<String, LanguageYaml> = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");

    let mut generated = String::new();
    generated.push_str("use std::collections::HashMap;\nuse once_cell::sync::Lazy;\n\n");

    generated.push_str("#[derive(Debug, PartialEq, Eq, Clone)]\n");
    generated.push_str("pub struct Language {\n");
    generated.push_str("    pub name: &'static str,\n");
    generated.push_str("    pub language_type: &'static str,\n");
    generated.push_str("    pub color: Option<&'static str>,\n");
    generated.push_str("    pub aliases: Option<&'static [&'static str]>,\n");
    generated.push_str("    pub extensions: Option<&'static [&'static str]>,\n");
    generated.push_str("    pub interpreters: Option<&'static [&'static str]>,\n");
    generated.push_str("    pub group: Option<&'static str>,\n");
    generated.push_str("    pub tm_scope: Option<&'static str>,\n");
    generated.push_str("    pub ace_mode: Option<&'static str>,\n");
    generated.push_str("    pub codemirror_mode: Option<&'static str>,\n");
    generated.push_str("    pub codemirror_mime_type: Option<&'static str>,\n");
    generated.push_str("    pub language_id: u32,\n");
    generated.push_str("}\n\n");

    generated.push_str("pub const ALL_LANGUAGES: &[Language] = &[\n");
    let mut entries = Vec::new();
    for (name, lang) in &map {
        let mut entry = String::new();
        entry.push_str("    Language {\n");
        entry.push_str(&format!("        name: {:?},\n", name));
        entry.push_str(&format!("        language_type: {:?},\n", lang.language_type));
        entry.push_str(&format!("        color: {},\n", optional_string(&lang.color)));
        entry.push_str(&format!("        aliases: {},\n", optional_vec(&lang.aliases)));
        entry.push_str(&format!("        extensions: {},\n", optional_vec(&lang.extensions)));
        entry.push_str(&format!("        interpreters: {},\n", optional_vec(&lang.interpreters)));
        entry.push_str(&format!("        group: {},\n", optional_string(&lang.group)));
        entry.push_str(&format!("        tm_scope: {},\n", optional_string(&lang.tm_scope)));
        entry.push_str(&format!("        ace_mode: {},\n", optional_string(&lang.ace_mode)));
        entry.push_str(&format!("        codemirror_mode: {},\n", optional_string(&lang.codemirror_mode)));
        entry.push_str(&format!("        codemirror_mime_type: {},\n", optional_string(&lang.codemirror_mime_type)));
        entry.push_str(&format!("        language_id: {},\n", lang.language_id));
        entry.push_str("    }\n");
        entries.push(entry);
    }
    generated.push_str(&entries.join(",\n"));
    generated.push_str("];\n\n");

    // NAME_MAP
    let mut name_map_entries = String::new();
    {
        let mut seen = std::collections::HashSet::new();
        for (index, (name, lang)) in map.iter().enumerate() {
            let key = name.to_lowercase();
            if seen.insert(key.clone()) {
                name_map_entries.push_str(&format!("    m.insert({:?}, &ALL_LANGUAGES[{}]);\n", key, index));
            }
            if let Some(aliases) = &lang.aliases {
                for alias in aliases {
                    let key = alias.to_lowercase();
                    if seen.insert(key.clone()) {
                        name_map_entries.push_str(&format!("    m.insert({:?}, &ALL_LANGUAGES[{}]);\n", key, index));
                    }
                }
            }
        }
    }
    generated.push_str("pub static NAME_MAP: Lazy<HashMap<&'static str, &'static Language>> = Lazy::new(|| {\n");
    generated.push_str("    let mut m: HashMap<&'static str, &'static Language> = HashMap::new();\n");
    generated.push_str(&name_map_entries);
    generated.push_str("    m\n});\n\n");

    // EXTENSION_MAP
    let mut ext_map_entries = String::new();
    {
        use std::collections::BTreeMap;
        let mut ext_map: BTreeMap<String, (usize, bool)> = BTreeMap::new();
        for (index, (name, lang)) in map.iter().enumerate() {
            if let Some(exts) = &lang.extensions {
                for ext in exts {
                    let key = ext.trim_start_matches('.').to_lowercase();
                    let alias_match = lang.aliases.as_ref().map_or(false, |a| a.iter().any(|al| al.eq_ignore_ascii_case(&key))) || name.eq_ignore_ascii_case(&key);
                    match ext_map.get(&key) {
                        Some(&(_, existing_match)) => {
                            if alias_match && !existing_match {
                                ext_map.insert(key.clone(), (index, alias_match));
                            }
                        }
                        None => {
                            ext_map.insert(key.clone(), (index, alias_match));
                        }
                    }
                }
            }
        }
        for (key, (idx, _)) in ext_map {
            ext_map_entries.push_str(&format!("    m.insert({:?}, &ALL_LANGUAGES[{}]);\n", key, idx));
        }
    }
    generated.push_str("pub static EXTENSION_MAP: Lazy<HashMap<&'static str, &'static Language>> = Lazy::new(|| {\n");
    generated.push_str("    let mut m: HashMap<&'static str, &'static Language> = HashMap::new();\n");
    generated.push_str(&ext_map_entries);
    generated.push_str("    m\n});\n\n");

    // CODEMIRROR_MODE_MAP
    let mut mode_map_entries = String::new();
    {
        use std::collections::BTreeMap;
        let mut mode_map: BTreeMap<String, (usize, bool)> = BTreeMap::new();
        for (index, (name, lang)) in map.iter().enumerate() {
            if let Some(mode) = &lang.codemirror_mode {
                let key = mode.to_lowercase();
                let preferred = name.to_lowercase() == key;
                match mode_map.get(&key) {
                    Some(&(_, existing_pref)) => {
                        if preferred && !existing_pref {
                            mode_map.insert(key.clone(), (index, preferred));
                        }
                    }
                    None => {
                        mode_map.insert(key.clone(), (index, preferred));
                    }
                }
            }
        }
        for (key, (idx, _)) in mode_map {
            mode_map_entries.push_str(&format!("    m.insert({:?}, &ALL_LANGUAGES[{}]);\n", key, idx));
        }
    }
    generated.push_str("pub static CODEMIRROR_MODE_MAP: Lazy<HashMap<&'static str, &'static Language>> = Lazy::new(|| {\n");
    generated.push_str("    let mut m: HashMap<&'static str, &'static Language> = HashMap::new();\n");
    generated.push_str(&mode_map_entries);
    generated.push_str("    m\n});\n");

    fs::write(Path::new(&out_dir).join("generated.rs"), generated).unwrap();
}

