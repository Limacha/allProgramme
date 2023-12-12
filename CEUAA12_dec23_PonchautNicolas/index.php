<!DOCTYPE html>
<html lang="fr">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Examen UAA12 - Ponchaut Nicolas</title>
    <link rel="icon" href="image/iconSiteWeb.png">
    <link rel="stylesheet" href="css/flex.css">
    <link rel="stylesheet" href="css/style.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@100&display=swap" rel="stylesheet">
</head>


<body>
    <header class="flex spaceAround">
        <a href="index.php" class="bouton">Home</a>
        <a href="formulaireMath.php" class="bouton">Tester l'application</a>
        <a href="#" class="bouton">Contact</a>
    </header>
    <div class="flex centerH" id="header2">
        <h1>Commander notre application astyCalc</h1>
    </div>
    <div class="flex centerH">
        <form action="index.php" method="GET" class="flex" id="middel">
            <div class="galImg flex column">
                <h2>glalerie</h2>
                <h2>image</h2>
                <div class="flex">
                    <img src="image/calc.png" alt="">
                    <img src="image/calc.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/calc.png" alt="">
                    <img src="image/calc.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/ops.png" alt="">
                    <img src="image/ops.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/ops.png" alt="">
                    <img src="image/ops.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/pi.png" alt="">
                    <img src="image/pi.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/pi.png" alt="">
                    <img src="image/pi.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/log.png" alt="">
                    <img src="image/log.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/log.png" alt="">
                    <img src="image/log.png" alt="">
                </div>
            </div>
            <div class="flex spaceAround">
                <fieldset>
                    <legend>Vos information</legend>
                    <div class="flex column spaceBetween inFieldset">
                        <div>
                            <h6>votre nom</h6>
                            <input type="text" id="surname" name="surname" required />
                        </div>
                        <div>
                            <h6>votre prenom</h6>
                            <input type="text" id="name" name="name" required />
                        </div>
                        <div>
                            <h6>votre email</h6>
                            <input type="email" id="email" name="email" required />
                        </div>
                    </div>


                </fieldset>
                <div class="flex column" id="divsend">
                    <button id="send" name="send">Envoyer</button>
                </div>
                <fieldset>
                    <legend>Parametrez votre commande</legend>
                    <div class="flex column spaceBetween inFieldset">
                        <h6>choissisez parmis les possibiliter</h6>
                        <select name="type" id="ype-select" required>
                            <optgroup label="ordinateur">
                                <option value="pc">pc</option>
                                <option value="mac">mac</option>
                            </optgroup>
                            <optgroup label="telephone">
                                <option value="andro">android</option>
                                <option value="appel">appel</option>
                            </optgroup>
                        </select>
                        <div>
                            <h6>votre date de naissance</h6>
                            <input type="date" id="date" name="date" required />
                        </div>

                        <div>
                            <h6>Facture</h6>
                            <input type="radio" id="mail" name="drone" value="mail" checked />
                            <label for="mail">par mail</label>
                            <br>
                            <input type="radio" id="paper" name="drone" value="paper" />
                            <label for="paper">par papier</label>
                        </div>
                    </div>
                </fieldset>
            </div>

            <div class="galImg flex column">
                <h2>glalerie</h2>
                <h2>image</h2>
                <div class="flex">
                    <img src="image/calc.png" alt="">
                    <img src="image/calc.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/calc.png" alt="">
                    <img src="image/calc.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/ops.png" alt="">
                    <img src="image/ops.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/ops.png" alt="">
                    <img src="image/ops.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/pi.png" alt="">
                    <img src="image/pi.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/pi.png" alt="">
                    <img src="image/pi.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/log.png" alt="">
                    <img src="image/log.png" alt="">
                </div>
                <div class="flex">
                    <img src="image/log.png" alt="">
                    <img src="image/log.png" alt="">
                </div>
            </div>
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