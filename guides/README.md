# Structure

- main => Porte d'entrée des requêtes permet de mapper un endpoint à une fonction


Chaque fonction majeure sera mise dans un module (AkA un dossier), chaque module comportera au moins deux fichiers : ``module_requests.rs`` et ``module_client.rs``.  
- ``module_requests`` définiras tous les endpoints, les fonctions qui seront appelés par le main en fonction du besoin. 
- ``module_client`` quand a lui comportera toutes les fonctions liés au fonctionnement profond des endpoints
