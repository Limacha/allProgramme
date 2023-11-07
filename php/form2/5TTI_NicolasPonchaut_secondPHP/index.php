<!DOCTYPE html>
<html lang="fr">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Example : Ponchaut Nicolas</title>
    <link rel="icon" href="./image/iconSiteWeb.png">
    <link rel="stylesheet" href="./css/flex.css">
    <link rel="stylesheet" href="./css/style.css">
</head>

<body class="flex column centerV">
    <header>
        <h1>Un super formulaire</h1>
    </header>
    <form action="" method="">
        <fieldset>
            <legend>donnés personnel</legend>
            <div>
                <label for="name">votre prenom</label>
                <input type="text" id="name" name="name" require>
            </div>
            <div>
                <label for="surname">votre nom</label>
                <input type="text" id="surname" name="surname" require>
            </div>
            <div>
                <label for="date">votre date de naissance</label>
                <input type="date" id="date" name="date" require>
            </div>
        </fieldset>
        <fieldset>
            <legend>les nouveaux input</legend>
            <div><label for="URL">votre URL</label>
                <input type="url" id="URL" name="URL" require>
            </div>
            <div>
                <label for="phone">votre numero de telephone</label>
                <input type="tel" id="phone" name="phone" require>
            </div>
            <div>
                <label for="appreciation">votre appreciation (1 - 20)</label>
                <input type="range" id="appreciation" name="appreciation" min="1" max="20" value="10" require>
            </div>
            <div>
                <label for="color">Votre couleur preferer</label>
                <input type="color" id="color" name="color" require>
            </div>
            <div>
                <label for="search">Votre resherche preferer</label>
                <input type="search" id="search" name="search" require>
            </div>
            <div>
                <label for="fichier">Choisi un fichier</label>
                <input type="file" id="fichier" name="fichier" require>
            </div>
            <div>
                <label for="time">Choisi une heure</label>
                <input type="time" id="time" name="time" require>
            </div>
            <div>
                <label for="month">Choisi un mois</label>
                <input type="month" id="month" name="month" require>
            </div>
            <div>
                <label for="week">Choisi une semaine</label>
                <input type="week" id="week" name="week" require>
            </div>
            <div class="flex">
                <label for="explaine">Vos explication</label>
                <textarea name="explaine" id="explaine" cols="30" rows="10"></textarea>
            </div>
            <div>
                <label for="restart">Votre resherche</label>
                <button id="restart" name="restart">Reinitialiser</button>
            </div>
        </fieldset>
        <fieldset>
            <legend>Bouton envoyer</legend>
            <button id="send" name="send">Envoyer</button>
        </fieldset>
    </form>

</body>

</html>