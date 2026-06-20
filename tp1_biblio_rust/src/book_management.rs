
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Genre {
    Roman,
    Informatique,
    Histoire,
    Science,
    Biographie,
    Autre,
}

impl Genre {
    pub fn from_choice(choice: &str) -> Option<Self> {
        match choice.trim() {
            "1" => Some(Self::Roman),
            "2" => Some(Self::Informatique),
            "3" => Some(Self::Histoire),
            "4" => Some(Self::Science),
            "5" => Some(Self::Biographie),
            "6" => Some(Self::Autre),
            _ => None,
        }
    }
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Roman => "Roman",
            Self::Informatique => "Informatique",
            Self::Histoire => "Histoire",
            Self::Science => "Science",
            Self::Biographie => "Biographie",
            Self::Autre => "Autre",
        };

        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statut {
    Disponible,
    Emprunte,
}

impl Statut {
    pub fn from_choice(choice: &str) -> Option<Self> {
        match choice.trim() {
            "1" => Some(Self::Disponible),
            "2" => Some(Self::Emprunte),
            _ => None,
        }
    }
}

impl fmt::Display for Statut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Disponible => "Disponible",
            Self::Emprunte => "Emprunté",
        };

        write!(f, "{}", label)
    }
}

pub trait Affichable {
    fn afficher(&self, index: usize);
}

#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u16,
    pub pages: u16,
    pub genre: Genre,
    pub status: Statut,
}

impl Affichable for Book {
    fn afficher(&self, index: usize) {
        println!(
            "{}. {}\nAuteur : {}\nAnnée : {}\nPages : {}\nGenre : {}\nStatut : {}",
            index, self.title, self.author, self.year, self.pages, self.genre, self.status
        );
    }
}

pub fn sample_books() -> Vec<Book> {
    vec![
        Book {
            title: "Le Petit Prince".to_string(),
            author: "Antoine de Saint-Exupéry".to_string(),
            year: 1943,
            pages: 96,
            genre: Genre::Roman,
            status: Statut::Disponible,
        },
        Book {
            title: "Programmation Rust".to_string(),
            author: "Équipe Rust".to_string(),
            year: 2021,
            pages: 320,
            genre: Genre::Informatique,
            status: Statut::Emprunte,
        },
        Book {
            title: "Histoire du monde".to_string(),
            author: "Jean Martin".to_string(),
            year: 2018,
            pages: 260,
            genre: Genre::Histoire,
            status: Statut::Disponible,
        },
    ]
}

pub fn add_book(books: &mut Vec<Book>, book: Book) {
    books.push(book);
}

pub fn update_book_status(books: &mut [Book], index: usize, new_status: Statut) -> Result<(), String> {
    if index >= books.len() {
        return Err("Index hors limites".to_string());
    }

    books[index].status = new_status;
    Ok(())
}

pub fn search_books_by_title<'a>(books: &'a [Book], query: &str) -> Vec<&'a Book> {
    let query = query.trim().to_lowercase();

    books
        .iter()
        .filter(|book| book.title.to_lowercase().contains(&query))
        .collect()
}

pub fn display_books(books: &[Book]) {
    if books.is_empty() {
        println!("Aucun livre dans la collection.");
        return;
    }

    println!("--- Liste des livres ---");
    for (index, book) in books.iter().enumerate() {
        book.afficher(index + 1);
        println!();
    }
}