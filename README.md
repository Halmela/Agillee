# Agillee

## Sovellus

Tietokanta, johon tallennetaan esineitä, ja missä kyseiset esineet ovat.
Jokaisella esineellä on
- vanhempi
- kuvaus
- mikä taho sen omistaa
- mikä taho on sen valmistanut

Mahdollisesti esineille voisi pistää myös esineen sijainnin vanhempansa sisällä.

Vanhemmuudet voidaan käydä läpi kutsumalla aina vanhemman vanhempaa jne. kunnes vanhempi on tyhjä.

Jos esineestä halutaan lapsi, voidaan etsiä tietokannasta jokainen esine, jonka vanhempi_id on esineen id.

Jos esine halutaan poistaa, etsitään samalla kaikki esineet, joiden vanhempi_id on kyseinen esine.
Käyttäjältä kysytään poistetaanko esineen sisällä olleet esineet, vaihdetaanko niiden vanhempi poistettavan vanhemmaksi, vai pistetäänkö ne johonkin muuhun laatikkoon.

## Asennus

Asenna Docker ja Docker Compose, ja suorita `docker-compose up --build`.
