#ifndef MouvementControl_H
#define MouvementControl_H

#include <Arduino.h>

/*control des mouvements du robot*/
class MouvementControl
{
public:
  const int maxSpeed = 255; // la vitesse max
  const int minSpeed = 50;  // la vitesse min
  bool duration = true;     // si on le fais mais que sur une duree
  bool control = true;      // si on control le moteur
  enum class Direction      // direction emprunter
  {
    Forward,       //(1)
    Backward,      //(2)
    Left,          //(3)
    Right,         //(4)
    LeftForward,   //(5)
    LeftBackward,  //(6)
    RightForward,  //(7)
    RightBackward, //(8)
    stop_it        //(9)
  };
  Direction direction;
  /**
   * @brief initialize tout les port
   *
   * - PINSpeedMotorA/B
   *
   * - PINDirA/B
   *
   * - PINControlMotor
   */
  void Init();
  /**
   * @brief Contrôle le mouvement des moteurs pendant une durée donnée.
   *
   * - moteura = droite
   *
   * - moteurb = gauche
   *
   * @param directionA  Direction du moteur A (true = avant, false = arrière)
   * @param speedA      Vitesse du moteur A (0; 50 à 255)
   * @param directionB  Direction du moteur B (true = avant, false = arrière)
   * @param speedB      Vitesse du moteur B (0; 50 à 255)
   * @param time        Durée du mouvement en millisecondes default: 0 -> infini
   */
  void Move(bool directionA, unsigned char speedA, bool directionB, unsigned char speedB, unsigned int time = 0);
  /**
   * @brief deplacement en fonction de la direction
   *
   * @param speed Vitesse par default des moteurs ->  /raport si deplacement + rotation
   * @param relation Raport entre les vitesses des moteurs
   * @param time Durée du mouvement en millisecondes default: 0 -> infini
   */
  void MoveByDirection(unsigned char speed, unsigned char relation = 2, unsigned int time = 0);
  /**
   * @brief reset tout les ports sur leur valeur par default et desactive le control
   *
   * - Speed = 0
   *
   * - Dir = HIGH
   */
  void ResetMove();

private:
  const int PINSpeedMotorA = 5;  // vitesse motor droit
  const int PINSpeedMotorB = 6;  // vitesse motor gauche
  const int PINDirA = 7;         // direction droite true avance
  const int PINDirB = 8;         // direction gauche true avance
  const int PINControlMotor = 3; // si moteur roule ou pas
};
#endif