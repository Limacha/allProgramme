#ifndef _MouvementControl_H_
#define _MouvementControl_H_

#include <Arduino.h>

/*Motor*/
class MouvementControl
{
public:
  const int maxSpeed = 255; // la vitesse max
  const int minSpeed = 100; // la vitesse min
  bool duration = true;     // si on le fais mais que sur une duree
  bool control = true;      // si on control le moteur
  enum Direction
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
  void Init();
  void Move(bool directionA, int speedA, bool directionB, int speedB, int time);
  void ResetMove();
  void Test();

private:
  const int PINSpeedMotorA = 5;  // vitesse motor droit
  const int PINSpeedMotorB = 6;  // vitesse motor gauche
  const int PINDirA = 8;         // direction gauche true avance
  const int PINDirB = 7;         // direction droit true avance
  const int PINControlMotor = 3; // si moteur roule ou pas
};
void Blink();
#endif