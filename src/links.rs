pub struct Socials<'a> {
    pub social_vec: Vec<(&'a str, &'a str)>,
}

impl Socials<'_> {
    pub fn get_link_string(&mut self) -> String {
        self.social_vec.sort();
        let mut str = String::new();
        self.social_vec
            .iter()
            .for_each(|link| str.push_str(&format!("{}: {}\n", link.0, link.1)));
        str
    }
}
