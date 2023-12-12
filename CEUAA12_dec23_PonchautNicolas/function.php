<?php
function NatureTriangle($cA, $cB, $cC)
{
    $message = "";
    if ($cA == $cB && $cB == $cC) {
        $message = "le triangle est equilateral";
    } else {
        if ($cA >= $cB && $cA >= $cC) {
            $pg = $cA;
            $c2 = $cB;
            $c3 = $cC;
        } else {
            if ($cB >= $cA && $cB >= $cC) {
                $pg = $cB;
                $c2 = $cA;
                $c3 = $cC;
            } else {
                $pg = $cC;
                $c2 = $cA;
                $c3 = $cB;
            }
        }
        if ($pg < $c2 + $c3) {
            if (round(pow($pg, 2)) == round(pow($cB, 2)) + round(pow($cC, 2))) {
                if ($cB == $cC) {
                    $message = "un triangle isocèle rectangle";
                } else {
                    $message = "un triangle rectangle";
                }
            } elseif ($cA == $cB || $cA == $cC || $cC == $cB) {
                $message = "un triangle isocèle";
            } else {
                $message = "un triangle quelconque";
            }
        } else {
            $message = "les dimension sont incorrect";
        }
    }
    return $message;
}
