use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use crate::services::anime_search::{
    Rating as RatingMsg,
    Parent as ParentMsg,
    Candidate as CandidateMsg
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub(crate) enum Rating {
    AllAges,
    Hentai
}

impl Display for Rating {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Rating::AllAges => f.write_str("all_ages"),
            Rating::Hentai => f.write_str("hentai"),
        }
    }
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
pub(crate) struct Parent {
    pub(crate) name: String,
    pub(crate) name_japanese: Option<String>
}

impl Parent {
    #[inline]
    pub(crate) fn into_msg(self) -> ParentMsg {
        ParentMsg{
            name: self.name,
            name_japanese: self.name_japanese,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Candidate {
    pub(crate) url: String,
    pub(crate) parent: Option<Parent>,
    pub(crate) name: String,
    pub(crate) name_english: Option<String>,
    pub(crate) name_japanese: Option<String>
}

impl Candidate {
    #[inline]
    pub(crate) fn into_msg(self) -> CandidateMsg {
        CandidateMsg{
            url: self.url,
            parent: match self.parent {
                Some(p) => Some(p.into_msg()),
                None => None
            },
            name: self.name,
            name_english: self.name_english,
            name_japanese: self.name_japanese
        }
    }
}
