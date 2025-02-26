use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Character {
    pub(crate) name: Option<String>,
    pub(crate) name_kanji: Option<String>  
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Anime {
    pub(crate) title: Option<String>,
    pub(crate) title_english: Option<String>,
    pub(crate) title_japanese: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AnimeSearchCandidate {
    pub(crate) anime: Anime,
    pub(crate) characters: Vec<Character>
}
