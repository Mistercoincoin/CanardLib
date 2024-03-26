use std::io;

// Définition de la structure Livre
struct Livre {
    titre: String,
    auteur: String,
    genre: String,
    annee_publication: String,
}

// Implémentation de méthodes pour la structure Livre
impl Livre {
    // Constructeur de Livre
    fn new(titre: String, auteur: String, genre: String, annee_publication: String) -> Livre {
        Livre { titre, auteur, genre, annee_publication }
    }

    // Méthode pour afficher les informations sur le livre
    fn afficher_informations(&self) {
        println!("Informations sur le livre :");
        println!("Titre: {}", self.titre);
        println!("Auteur: {}", self.auteur);
        println!("Genre: {}", self.genre);
        println!("Année de publication: {}", self.annee_publication);
    }
}

pub fn main() {
    // Demander à l'utilisateur d'entrer les informations sur le livre
    println!("Entrez les informations sur le livre :");

    // Demander et lire le titre du livre
    println!("Titre du livre :");
    let mut titre = String::new();
    io::stdin().read_line(&mut titre).expect("Failed to read line");

    // Demander et lire l'auteur du livre
    println!("Auteur du livre :");
    let mut auteur = String::new();
    io::stdin().read_line(&mut auteur).expect("Failed to read line");

    // Demander et lire le genre du livre
    println!("Genre du livre :");
    let mut genre = String::new();
    io::stdin().read_line(&mut genre).expect("Failed to read line");

    // Demander et lire l'année de publication du livre
    println!("Année de publication du livre :");
    let mut annee_publication = String::new();
    io::stdin().read_line(&mut annee_publication).expect("Failed to read line");

    // Créer un objet Livre avec les informations saisies par l'utilisateur
    let livre = Livre::new(titre.trim().to_string(), auteur.trim().to_string(), genre.trim().to_string(), annee_publication.trim().to_string());

    // Afficher les informations sur le livre
    livre.afficher_informations();
}
