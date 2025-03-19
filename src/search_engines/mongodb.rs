pub mod mongo;
pub mod atlas;

const SEARCH_DATABASE: &str = "anime_search";
const FORBIDDEN: [char; 12]  = [
    '$', 
    '.', ',', 
    '{', '}', 
    '[', ']', 
    '(', ')',
    ':', ';', 
    '/'
];
