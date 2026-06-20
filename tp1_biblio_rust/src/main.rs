mod book_management;
mod stats;
mod ui;

use crate::book_management::{add_book, display_books, sample_books, search_books_by_title, update_book_status, Book, Genre, Statut};
use crate::stats::calculate_stats;
use crate::ui::{display_genre_menu, display_menu, display_status_menu, get_user_input};

fn read_genre() -> Genre {
    loop {
        display_genre_menu();
        let choice = get_user_input("Votre choix de genre : ");

        match Genre::from_choice(&choice) {
            Some(genre) => return genre,
            None => println!("Choix de genre invalide."),
        }
    }
}

fn read_status() -> Statut {
    loop {
        display_status_menu();
        let choice = get_user_input("Votre choix de statut : ");

        match Statut::from_choice(&choice) {
            Some(status) => return status,
            None => println!("Choix de statut invalide."),
        }
    }
}

fn main() {
    let mut books = sample_books();

    println!("Bienvenue dans la bibliothèque!");
    loop {
        display_menu();
        let choice = get_user_input("Votre choix : ");

        match choice.as_str() {
            "1" => display_books(&books),
            "2" => {
                let title = get_user_input("Titre : ");
                let author = get_user_input("Auteur : ");
                let year = get_user_input("Année : ");
                let pages = get_user_input("Pages : ");

                let year = match year.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("Année invalide.");
                        continue;
                    }
                };

                let pages = match pages.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("Nombre de pages invalide.");
                        continue;
                    }
                };

                let genre = read_genre();
                let status = read_status();

                let book = Book {
                    title,
                    author,
                    year,
                    pages,
                    genre,
                    status,
                };

                add_book(&mut books, book);
                println!("Livre ajouté avec succès.");
            }
            "3" => {
                let query = get_user_input("Titre ou partie du titre : ");
                let matches = search_books_by_title(&books, &query);

                if matches.is_empty() {
                    println!("Aucun livre trouvé pour '{}'.", query);
                } else {
                    println!("--- Livres trouvés ---");
                    for (index, book) in matches.iter().enumerate() {
                        println!(
                            "{}. {} - {} ({}) | {} | {}",
                            index + 1,
                            book.title,
                            book.author,
                            book.year,
                            book.genre,
                            book.status
                        );
                    }
                }
            }
            "4" => {
                if books.is_empty() {
                    println!("Aucun livre à modifier.");
                    continue;
                }

                display_books(&books);
                let selection = get_user_input("Numéro du livre à modifier : ");

                let index = match selection.parse::<usize>() {
                    Ok(value) if value >= 1 && value <= books.len() => value - 1,
                    _ => {
                        eprintln!("Sélection invalide.");
                        continue;
                    }
                };

                let new_status = read_status();

                match update_book_status(&mut books, index, new_status) {
                    Ok(()) => println!("Statut mis à jour."),
                    Err(error) => eprintln!("Impossible de mettre à jour le statut: {}", error),
                }
            }
            "5" => {
                let (total_books, total_pages, average_pages, available_books, borrowed_books) = calculate_stats(&books);

                println!("--- Statistiques ---");
                println!("Nombre total de livres : {}", total_books);
                println!("Nombre total de pages : {}", total_pages);
                println!("Moyenne des pages : {:.2}", average_pages);
                println!("Livres disponibles : {}", available_books);
                println!("Livres empruntés : {}", borrowed_books);
            }
            "6" => {
                println!("Fin de l'application.");
                break;
            }
            _ => println!("Choix invalide. Veuillez réessayer."),
        }
    }
}
