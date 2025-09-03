#include "MouvementControl.h"

MouvementControl AppMotor;

void setup()
{
  pinMode(LED_BUILTIN, OUTPUT);
  AppMotor.Init();
  AppMotor.direction = MouvementControl::Forward;
  if (AppMotor.direction == MouvementControl::Forward)
  {
    AppMotor.Move(true, 100, true, 100, 1000);
  }
  // AppMotor.Test();
}

void loop()
{
  Blink();
}