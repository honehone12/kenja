pub mod mongo;
pub mod atlas;

const FORBIDDEN: [char; 12]  = [
    '$', 
    '.', ',', 
    '{', '}', 
    '[', ']', 
    '(', ')',
    ':', ';', 
    '/'
];
