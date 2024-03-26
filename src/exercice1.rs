// Importation du module std::io pour la gestion des entrées/sorties
use std::io;

// Fonction principale
pub fn main() {
    // Déclaration des variables pour stocker les informations sur le livre
    let mut title = String::new(); // Titre du livre
    let mut author = String::new(); // Auteur du livre
    let mut genre = String::new(); // Genre du livre
    let mut year_of_publication = String::new(); // Année de publication du livre

    // Demander à l'utilisateur d'entrer le titre du livre
    println!("Entrer le titre du livre: ");
    // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `title`
    io::stdin().read_line(&mut title).expect("Failed to read line");

    // Demander à l'utilisateur d'entrer l'auteur du livre
    println!("Entrer le Auteur du livre: ");
    // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `author`
    io::stdin().read_line(&mut author).expect("Failed to read line");

    // Demander à l'utilisateur d'entrer le genre du livre
    println!("Entrer le genre du livre: ");
    // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `genre`
    io::stdin().read_line(&mut genre).expect("Failed to read line");

    // Demander à l'utilisateur d'entrer l'année de publication du livre
    println!("Entrer l'année de publication du livre: ");
    // Lire l'entrée de l'utilisateur et stocker la valeur dans la variable `year_of_publication`
    io::stdin().read_line(&mut year_of_publication).expect("Failed to read line");

    // Afficher les informations sur le livre saisies par l'utilisateur
    println!("Titre: {}", title.trim());
    println!("Auteur: {}", author.trim());
    println!("Genre: {}", genre.trim());
    println!("Année de publication: {}", year_of_publication.trim());
}
