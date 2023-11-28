<!DOCTYPE html>
<html lang="fr">
<?php
require_once "function.php";
$mess = "";
if (isset($_GET['age']) && isset($_GET['obtenuPermis']) && isset($_GET['nbAccident']) && isset($_GET['nbAnneeAnciennete'])) {
    $age = (int)$_GET['age'];
    $obtenuPermis = (int)$_GET['obtenuPermis'];
    $nbAccident = (int)$_GET['nbAccident'];
    $nbAnneeAnciennete = (int)$_GET['nbAnneeAnciennete'];

    $mess = CalculeContrat($age, $obtenuPermis, $nbAccident, $nbAnneeAnciennete);
}
?>

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>prepa exam : Ponchaut Nicolas</title>
    <link rel="icon" href="./image/iconSiteWeb.png">
    <link rel="stylesheet" href="./css/flex.css">
    <link rel="stylesheet" href="./css/style.css">
</head>

<body>
    <header>
        <h1>Jouez avec nous!!!</h1>
    </header>
    <div class="flex centerH">
        <form action="GET" method="index.php" class="flex column">
            <h3>Devinez un nombre</h3>
            <div id="Inum">
                <label for="age">Quel âge avez-vous (entre 18 et 95) ?</label>
                <input type="age" id="age" name="age" min="18" max="95" placeholder="18-95" require>
            </div>
            <div id="Inum">
                <label for="obtenuPermis">Depuis combien de temps avez-vous le permis (mois) ?</label>
                <input type="obtenuPermis" id="obtenuPermis" name="obtenuPermis" min="0" require>
            </div>
            <div id="Inum">
                <label for="nbAccident">Combien d'accidents avez-vous déjà fait ?</label>
                <input type="nbAccident" id="nbAccident" name="nbAccident" min="0" require>
            </div>
            <div id="Inum">
                <label for="nbAnneeAnciennete">Depuis combien d'années êtes-vous assuré chez nous ?"</label>
                <input type="nbAnneeAnciennete" id="nbAnneeAnciennete" name="nbAnneeAnciennete" min="0" require>
            </div>
            <div class="flex flexEnd">
                <button id="send" name="send">Envoyer</button>
            </div>
        </form>
    </div>
    <?php
    if ($mess == "Rouge") : ?>
        <div id="rouge">
            <p><?= $mess ?></p>
        </div>
    <?php elseif ($mess == "Orange") : ?>
        <div id="orange">
            <p><?= $mess ?></p>
        </div>
    <?php elseif ($mess == "Vert") : ?>
        <div id="vert">
            <p><?= $mess ?></p>
        </div>
    <?php elseif ($mess == "Bleu") : ?>
        <div id="blue">
            <p><?= $mess ?></p>
        </div>
    <?php else : ?>
        <div id="none">
            <p>vous n'avez pas de contrat avec nous ou vous ne pouvez pas en avoir</p>
        </div>
    <?php endif ?>
    <footer class="flex flexEnd">
        <p>Premier exercise GET 5TTI 2023</p>
    </footer>
</body>

</html>