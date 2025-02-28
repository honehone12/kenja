use serde::{Serialize, Deserialize};
use crate::services::anime_search::{
    Rating as RatingMsg,
    Character as CharacterMsg,
    Anime as AnimeMsg,
    Candidate as CandidateMsg
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub(crate) enum Rating {
    AllAges,
    Hentai
}

impl Rating {
    #[inline]
    pub(crate) fn from_msg(msg: RatingMsg) -> Self {
        match msg {
            RatingMsg::Unspecified => Rating::AllAges,
            RatingMsg::AllAges => Rating::AllAges,
            RatingMsg::Hentai => Rating::Hentai
        }
    } 
}
 
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Character {
    pub(crate) name: Option<String>,
    pub(crate) name_kanji: Option<String>  
}

impl Character {
    #[inline]
    pub(crate) fn into_msg(self) -> CharacterMsg {
        CharacterMsg{
            name: self.name,
            name_japanese: self.name_kanji,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Anime {
    pub(crate) title: Option<String>,
    pub(crate) title_english: Option<String>,
    pub(crate) title_japanese: Option<String>
}

impl Anime {
    #[inline]
    pub(crate) fn into_msg(self) -> AnimeMsg {
        AnimeMsg{
            name: self.title,
            name_english: self.title_english,
            name_japanese: self.title_japanese,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Candidate {
    pub(crate) anime: Anime,
    pub(crate) characters: Vec<Character>
}

impl Candidate {
    #[inline]
    pub(crate) fn into_msg(self) -> CandidateMsg {
        let mut characters = vec![];
        for c in self.characters {
            characters.push(c.into_msg());
        }

        CandidateMsg{
            anime: Some(self.anime.into_msg()),
            characters,
        }
    }
}
