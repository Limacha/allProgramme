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
    <h1> 1 </h1>
    <p>hello world</p>

    <h1> 2 </h1>
    <?php for ($i = 1; $i <= 10; $i++) : ?>
        <P>le nombre est <?= $i ?> </p>
    <?php endfor ?>

    <h1> 3 </h1>
    <?php for ($i = 1; $i <= 10; $i++) : ?>
        <?php if ($i == 1 || $i == 2 || $i > 3) : ?>
            <P>le nombre est <?= $i ?> </p>
        <?php endif ?>
    <?php endfor ?>
    <h1> 4 </h1>
    <?php for ($i = 1; $i <= 10; $i++) : ?>
        <?php if ($i != 4 || $i != 5 || $i != 6 || $i != 7) : ?>
            <P>le nombre est <?= $i ?> </p>
        <?php endif ?>
    <?php endfor ?>
    <h1> 5 </h1>
    <p>La valeur absolu de -5 est <?php abs(-5) ?> <\p>
            <p>La valeur absolu de 10 est <?php abs(10) ?> <\p>

</body>

</html>