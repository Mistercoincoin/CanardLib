// Importation du module std::io pour la gestion des entrées/sorties
use std::io;

// Fonction principale
pub fn main() {
    // Demander à l'utilisateur d'entrer les informations sur le livre
    println!("Entrez les informations sur le livre :");

    // Demander et lire le titre du livre
    println!("Titre du livre :");
    let mut title = String::new(); // Création d'une nouvelle chaîne de caractères pour stocker le titre
    io::stdin().read_line(&mut title).expect("Failed to read line"); // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `title`

    // Demander et lire l'auteur du livre
    println!("Auteur du livre :");
    let mut author = String::new(); // Création d'une nouvelle chaîne de caractères pour stocker l'auteur
    io::stdin().read_line(&mut author).expect("Failed to read line"); // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `author`

    // Demander et lire le genre du livre
    println!("Genre du livre :");
    let mut genre = String::new(); // Création d'une nouvelle chaîne de caractères pour stocker le genre
    io::stdin().read_line(&mut genre).expect("Failed to read line"); // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `genre`

    // Demander et lire l'année de publication du livre
    println!("Année de publication du livre :");
    let mut year_of_publication = String::new(); // Création d'une nouvelle chaîne de caractères pour stocker l'année de publication
    io::stdin().read_line(&mut year_of_publication).expect("Failed to read line"); // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `year_of_publication`

    // Créer un tuple avec les informations sur le livre
    let book_info = (title.trim().to_string(), author.trim().to_string(), genre.trim().to_string(), year_of_publication.trim().to_string());

    // Afficher les informations sur le livre saisies par l'utilisateur
    println!("Informations sur le livre :");
    println!("Titre: {}", book_info.0);
    println!("Auteur: {}", book_info.1);
    println!("Genre: {}", book_info.2);
    println!("Année de publication: {}", book_info.3);
}
