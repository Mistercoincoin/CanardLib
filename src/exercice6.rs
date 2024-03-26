use std::io;

pub fn main() {
    loop {
        // Déclaration des variables pour stocker les informations sur le livre
        let mut titre = String::new();
        let mut auteur = String::new();
        let mut genre = String::new();
        let mut annee_publication = String::new();

        // Boucle while pour demander à l'utilisateur de saisir les informations sur le livre
        while titre.trim().is_empty() {
            // Demander et lire le titre du livre
            println!("Entrez le titre du livre (ou laissez vide pour terminer) :");
            titre.clear(); // Effacer le contenu précédent
            io::stdin().read_line(&mut titre).expect("Erreur lors de la lecture de l'entrée");

            // Vérifier si l'utilisateur a entré un titre vide
            if titre.trim().is_empty() {
                break; // Sortir de la boucle si le titre est vide
            }

            // Demander et lire l'auteur du livre
            println!("Entrez l'auteur du livre :");
            auteur.clear(); // Effacer le contenu précédent
            io::stdin().read_line(&mut auteur).expect("Erreur lors de la lecture de l'entrée");

            // Demander et lire le genre du livre
            println!("Entrez le genre du livre :");
            genre.clear(); // Effacer le contenu précédent
            io::stdin().read_line(&mut genre).expect("Erreur lors de la lecture de l'entrée");

            // Demander et lire l'année de publication du livre
            println!("Entrez l'année de publication du livre :");
            annee_publication.clear(); // Effacer le contenu précédent
            io::stdin().read_line(&mut annee_publication).expect("Erreur lors de la lecture de l'entrée");

            // Afficher les informations sur le livre saisies par l'utilisateur
            println!("Informations sur le livre :");
            println!("Titre: {}", titre.trim());
            println!("Auteur: {}", auteur.trim());
            println!("Genre: {}", genre.trim());
            println!("Année de publication: {}", annee_publication.trim());
        }

        // Demander à l'utilisateur s'il souhaite ajouter un autre livre
        println!("Voulez-vous ajouter un autre livre ? (oui/non)");
        let mut reponse = String::new();
        io::stdin().read_line(&mut reponse).expect("Erreur lors de la lecture de l'entrée");
        let reponse = reponse.trim().to_lowercase();

        if reponse != "oui" {
            break; // Sortir de la boucle principale si l'utilisateur répond autre chose que "oui"
        }
    }

    println!("Fin du programme.");
}
