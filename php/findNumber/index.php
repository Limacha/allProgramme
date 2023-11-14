<!DOCTYPE html>
<html lang="fr">
<?php
require_once "function.php";
$mess = "";
if (isset($_GET['number'])) {
    $number = (int)$_GET['number'];
    $mess = findNumber($number, 100);
}
?>

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Example : Ponchaut Nicolas</title>
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
                <label for="number"></label>
                <input type="number" id="number" name="number" min="0" max="1000" placeholder="0-1000" require>
            </div>
            <div class="flex flexEnd">
                <button id="send" name="send">Envoyer</button>
            </div>
        </form>
    </div>

    <div id="mess">
        <p><?= $mess ?></p>
    </div>
    <footer class="flex flexEnd">
        <p>Premier exercise GET 5TTI 2023</p>
    </footer>
</body>

</html>