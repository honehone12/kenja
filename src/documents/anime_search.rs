use std::fmt::{
    Display, 
    Formatter, 
    Result as FmtResult
};
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::services::anime_search::{
    Candidate as CandidateMsg, 
    Parent as ParentMsg, 
    ItemId as ItemIdMsg,
    DocumentType as DocumentTypeMsg, 
    Rating as RatingMsg
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Rating {
    AllAges = 1,
    Hentai = 2
}

impl Display for Rating {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Rating::AllAges => f.write_str("all_ages"),
            Rating::Hentai => f.write_str("hentai"),
        }
    }
}

impl TryFrom<RatingMsg> for Rating {
    type Error = ();

    #[inline]
    fn try_from(value: RatingMsg) -> Result<Self, Self::Error> {
        match value {
            RatingMsg::Unspecified 
                => Err(()),
            RatingMsg::AllAges => Ok(Rating::AllAges),
            RatingMsg::Hentai => Ok(Rating::Hentai)
        }
    } 
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(i32)]
pub enum DocumentType {
    Unspecified = 0,
    Anime = 1,
    Character = 2
}

impl From<DocumentType> for DocumentTypeMsg {
    #[inline]
    fn from(value: DocumentType) -> Self {
        match value {
            DocumentType::Unspecified => DocumentTypeMsg::Unspecified,
            DocumentType::Anime => DocumentTypeMsg::Anime,
            DocumentType::Character => DocumentTypeMsg::Character,
        }
    }
}

impl From<DocumentType> for i32 {
    #[inline]
    fn from(value: DocumentType) -> Self {
        Into::<DocumentTypeMsg>::into(value).into()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemId {
    pub(crate) id: i64,
    pub(crate) document_type: DocumentType
}

impl From<ItemId> for ItemIdMsg {
    #[inline]
    fn from(value: ItemId) -> Self {
        ItemIdMsg { 
            id: value.id, 
            document_type: value.document_type.into()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parent {
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
pub struct Candidate {
    pub(crate) id: ItemId,
    pub(crate) parent: Option<Parent>,
    pub(crate) tags: Vec<String>,
    pub(crate) name: String,
    pub(crate) name_english: Option<String>,
    pub(crate) name_japanese: Option<String>
}

impl From<Candidate> for CandidateMsg {
    #[inline]
    fn from(value: Candidate) -> Self {
        CandidateMsg{
            id: Some(value.id.into()),
            parent: value.parent.map(|p| p.into()),
            tags: value.tags,
            name: value.name,
            name_english: value.name_english,
            name_japanese: value.name_japanese
        }
    }
}
