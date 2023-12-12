<!DOCTYPE html>
<html lang="fr">
<?php
require_once "function.php";
$mess = "";
if (isset($_GET['cA']) && isset($_GET['cB']) && isset($_GET['cC'])) {
    $cA = (int)$_GET['cA'];
    $cB = (int)$_GET['cB'];
    $cC = (int)$_GET['cC'];

    $mess = NatureTriangle($cA, $cB, $cC);
}
?>

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Examen UAA12 - Ponchaut Nicolas</title>
    <link rel="icon" href="image/iconSiteWeb.png">
    <link rel="stylesheet" href="css/flex.css">
    <link rel="stylesheet" href="css/appli.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@100&display=swap" rel="stylesheet">
</head>

<body>
    <header class="flex spaceAround">
        <a href="index.php" class="bouton">Home</a>
        <a href="appli.php" class="bouton">Tester l'application</a>
        <a href="#" class="bouton">Contact</a>
    </header>
    <div class="flex centerH" id="header2">
        <h1>Testez notre calculateur de triangle</h1>
    </div>
    <div class="flex centerH">
        <form action="formulaireMath.php" method="GET" id="middel">
            <div class="flex centerV">
                <fieldset>
                    <legend>Vos information</legend>
                    <div class="flex column spaceBetween inFieldset">
                        <div>
                            <h6>coté A</h6>
                            <input type="number" id="cA" name="cA" min="1" placeholder="coté A" required>
                        </div>
                        <div>
                            <h6>coté B</h6>
                            <input type="number" id="cB" name="cB" min="1" placeholder="coté B" required>
                        </div>
                        <div>
                            <h6>coté C</h6>

                            <input type="number" id="cC" name="cC" min="1" placeholder="coté C" required>
                        </div>
                    </div>


                </fieldset>

                <p id="answer"><?= $mess ?></p>

            </div>
            <button id="send" name="send">Envoyer</button>

        </form>
    </div>

    <footer class="flex spaceBetween">
        <div class="buttonEnd">
            <p>Examen decembre 2023</p>
            <p>UAA12</p>
        </div>
        <div class="buttonEnd">
            <p>5TTI</p>
        </div>
    </footer>
</body>

</html>