<!DOCTYPE html>
<html lang="fr">
<?php
$nd = 5;
$nt  = 8;
$nbr1  = 21;
$nbr2  = 15;
$phrase = "La reussite passe par une etude et un entrainement regulier et serieux";
require_once "function.php";
?>

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>first function : Ponchaut Nicolas</title>
    <link rel="icon" href="./image/iconSiteWeb.png">
    <link rel="stylesheet" href="./css/flex.css">
    <link rel="stylesheet" href="./css/style.css">
    <h1>Apprendre les fonctions en php</h1>
</head>

<body>
    <div>
        <h1>Teston les appel de fonctions</h1>
        <p>Je veux de belles fonctions php (séparer analyse et affichagedqns votre fichier)</p>
        <h3>Premiere fonction</h3>
        <p>Voisi une suite tous a fait farfelue (pour un nombre de depart <?= $nd ?> et un nombre d'éléments de <?= $nt ?>): <?= fonctionspecial($nd, $nt) ?> </p>
        <h3>Calcul du PGCD</h3>
        <p>Le PGCD entre <?= $nbr1 ?> et <?= $nbr2 ?> vaut <?= algorithmeEuclide2($nbr1, $nbr2) ?></p>

        <h1>Afficher proprementdu code</h1>
        <p>On ne creez pas de fonction mais on ecrit proprement la boucle php dans l'html <br>
            (On souhaite afficher la derniere lettre de chaque mot dans une liste à puces. On considère que chaque mot est suivi d'un espace sauf le dernier)</p>
        <p>Dans la variable $phrase "<?= $phrase ?>". La derniere lettre de chaque mot est</p>
    </div>
</body>

</html>