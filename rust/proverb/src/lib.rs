pub fn build_proverb(list: &[&str]) -> String {
    let mut res = Vec::new();
    if list.len() > 0 {
        for i in 0..(list.len() - 1) {
            res.push(format!(
                "For want of a {} the {} was lost.",
                list[i],
                list[i + 1]
            ));
        }
        res.push(format!("And all for the want of a {}.", list[0]));
        res.join("\n")
    } else {
        String::new()
    }
}
