

pub fn make_one_string(vec_string: Vec<String>) -> String {
    let mut ret = String::new();
    for chaine in vec_string.iter() {
        if chaine != "" {
            ret.push_str(&chaine);
            ret.push('\n');
        }

    }

    return ret
}
