use std::collections::HashMap;

pub fn get_link_string() -> String{
    let social_links: HashMap<&str, &str> = HashMap::from([
        ("Mastodon", "https://tech.lgbt/@bird"), 
        ("Twitter", "https://twitter.com/unitybirb"),
        ("Tumblr",  "https://unity-birdposts.tumblr.com"),
        ("Cohost", "https://cohost.org/unitybirb")
    ]);
    let mut str =  String::new();
    social_links.iter().for_each(|link| str.push_str(&format!("{}: {}\n", link.0, link.1)));
    str
}