<?php
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
    $suite = "" + $nombreDepart + "";
    for ($i = 1; $i <= $nombreElementsSouhaite; $i++) {
        if ($nombreDepart < 5 && $nombreDepart % 3 != 0) {
            $nombreDepart = $nombreDepart * 5;
        } elseif ($nombreDepart > 5 && $nombreDepart < 10) {
            $nombreDepart = $nombreDepart / 6;
        } else {
            $nombreDepart = $nombreDepart * $nombreDepart;
        }
        $suite = $suite + $nombreDepart + "";
    }
    return $suite;
}
