use std::io::{self, Write};

pub fn display_menu() {
    println!("=== Gestionnaire de bibliothèque ===");
    println!("1. Afficher tous les livres");
    println!("2. Ajouter un livre");
    println!("3. Rechercher un livre par titre");
    println!("4. Modifier le statut d’un livre");
    println!("5. Afficher les statistiques");
    println!("6. Quitter");
}

pub fn display_genre_menu() {
    println!("Choisissez un genre :");
    println!("1. Roman");
    println!("2. Informatique");
    println!("3. Histoire");
    println!("4. Science");
    println!("5. Biographie");
    println!("6. Autre");
}

pub fn display_status_menu() {
    println!("Choisissez un statut :");
    println!("1. Disponible");
    println!("2. Emprunté");
}

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read user input");

    input.trim().to_string()
}


