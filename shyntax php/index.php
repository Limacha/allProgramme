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

<body>
    <?php
    echo "<h1> 1 </h1>";
    echo "<p>hello world</p>";

    echo "<h1> 2 </h1>";
    for ($i = 1; $i <= 10; $i++) {
        echo "<P>le nombre est" . $i . "</p>";
    }

    echo "<h1> 3 </h1>";
    for ($i = 1; $i <= 10; $i++) {
        if ($i == 1 || $i == 2 || $i > 3)
            echo "<P>le nombre est" . $i . "</p>";
    }

    echo "<h1> 4 </h1>";
    for ($i = 1; $i <= 10; $i++) {
        if ($i != 4 || $i != 5 || $i != 6 || $i != 7)
            echo "<P>le nombre est" . $i . "</p>";
    }

    echo "<h1> 5 </h1>";
    echo "<p>La valeur absolu de -5 est " . abs(-5);
    echo "<p>La valeur absolu de 10 est " . abs(10);
    ?>
</body>

</html>