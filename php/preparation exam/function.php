<?php
function findNumber($number, $goodNum)
{
    $mess = "";
    if ($number == $goodNum) {
        $mess = "Bien jouer s'est le bon numero";
    } elseif ($number > $goodNum) {
        $mess = "Dommage trop grand";
    } else {
        $mess = "Dommage trop petit";
    }
    return $mess;
}

function algorithmeEuclide2($nbr1, $nbr2)
{
    $reste = $nbr2;
    while ($reste != 0) {
        $reste = $nbr1 % $nbr2;
        $nbr1 = $nbr2;
        $nbr2 = $reste;
    }
    return $nbr1;
}
function fonctionspecial($nombreDepart, $nombreElementsSouhaite)
{
    $suite = "" . $nombreDepart . " ";
    for ($i = 1; $i <= $nombreElementsSouhaite; $i++) {
        if ($nombreDepart < 5 && $nombreDepart % 3 != 0) {
            $nombreDepart = $nombreDepart * 5;
        } elseif ($nombreDepart > 5 && $nombreDepart < 10) {
            $nombreDepart = $nombreDepart / 6;
        } else {
            $nombreDepart = $nombreDepart * $nombreDepart;
        }
        $suite = $suite . $nombreDepart . " ";
    }
    return $suite;
}
function CalculeContrat($age, $obtenuPermis, $nbAccident, $nbAnneeAnciennete)
{
    if ($age < 25 && $obtenuPermis < 2) {
        if ($nbAccident == 0) {
            $contrat = "Rouge";
        } else {
            $contrat = "Refus";
        }
    } else if (($age < 25 && $obtenuPermis >= 2) || ($age >= 25 && $obtenuPermis < 2)) {
        if ($nbAccident == 0) {
            $contrat = "Orange";
        } else if ($nbAccident == 1) {
            $contrat = "Rouge";
        } else {
            $contrat = "Refus";
        }
    } else {
        if ($nbAccident == 0) {
            $contrat = "Vert";
        } else if ($nbAccident == 1) {
            $contrat = "Orange";
        } else if ($nbAccident == 2) {
            $contrat = "Rouge";
        } else {
            $contrat = "Refus";
        }
    }
    if ($nbAnneeAnciennete > 5) {
        // écriture façon if ; else
        if ($contrat == "Rouge") {
            $contrat = "Orange";
        } else if ($contrat == "Orange") {
            $contrat = "Vert";
        } else if ($contrat == "Vert") {
            $contrat = "Bleu";
        }
    }
    return $contrat;
}
