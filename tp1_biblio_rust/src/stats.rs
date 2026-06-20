use crate::book_management::{Book, Statut};

pub fn calculate_stats(books: &[Book]) -> (usize, u32, f32, usize, usize) {
	let total_books = books.len();
	let total_pages: u32 = books.iter().map(|book| book.pages as u32).sum();
	let average_pages = if total_books == 0 {
		0.0
	} else {
		total_pages as f32 / total_books as f32
	};

	let available_books = books
		.iter()
		.filter(|book| book.status == Statut::Disponible)
		.count();

	let borrowed_books = books
		.iter()
		.filter(|book| book.status == Statut::Emprunte)
		.count();

	(total_books, total_pages, average_pages, available_books, borrowed_books)
}
