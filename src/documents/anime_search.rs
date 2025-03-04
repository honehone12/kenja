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

impl From<RatingMsg> for Rating {
    #[inline]
    fn from(value: RatingMsg) -> Self {
        match value {
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

impl From<Parent> for ParentMsg {
    #[inline]
    fn from(value: Parent) -> Self {
        ParentMsg{
            name: value.name,
            name_japanese: value.name_japanese,
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

impl From<Candidate> for CandidateMsg {
    #[inline]
    fn from(value: Candidate) -> Self {
        CandidateMsg{
            url: value.url,
            parent: value.parent.map(|p| p.into()),
            name: value.name,
            name_english: value.name_english,
            name_japanese: value.name_japanese
        }
    }
}
