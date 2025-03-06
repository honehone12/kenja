pub mod mongo;
pub mod atlas;

const SEARCH_DATABASE: &str = "anime";
const SEARCH_COLLECTION: &str = "flat_ani_chara";
const FORBIDDEN: [char; 12]  = [
    '$', 
    '.', ',', 
    '{', '}', 
    '[', ']', 
    '(', ')',
    ':', ';', 
    '/'
];
