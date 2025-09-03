#include "MouvementControl.h"

void MouvementControl::Init()
{
  pinMode(PINSpeedMotorA, OUTPUT);
  pinMode(PINSpeedMotorB, OUTPUT);
  pinMode(PINDirA, OUTPUT);
  pinMode(PINDirB, OUTPUT);
  pinMode(PINControlMotor, OUTPUT);
}
void MouvementControl::Move(bool directionA, int speedA, bool directionB, int speedB, int time)
{
  if (speedA > maxSpeed)
  {
    speedA = maxSpeed;
  }
  if (speedB > maxSpeed)
  {
    speedB = maxSpeed;
  }

  if (speedA < minSpeed)
  {
    speedA = minSpeed;
  }
  if (speedB < minSpeed)
  {
    speedB = minSpeed;
  }

  if (control)
  {
    digitalWrite(PINControlMotor, HIGH);
    if (directionA)
    {
      digitalWrite(PINDirA, HIGH);
      analogWrite(PINSpeedMotorA, speedA);
    }
    else
    {
      digitalWrite(PINDirA, LOW);
      analogWrite(PINSpeedMotorA, speedA);
    }
    if (directionB)
    {
      digitalWrite(PINDirB, HIGH);
      analogWrite(PINSpeedMotorB, speedB);
    }
    else
    {
      digitalWrite(PINDirB, LOW);
      analogWrite(PINSpeedMotorB, speedB);
    }
  }
  else
  {
    ResetMove();
    return;
  }

  if(time > 0)
  {    
    delay(time);
    ResetMove();
  }
  return;
}

void MouvementControl::Test()
{
  ResetMove();
}

void MouvementControl::ResetMove()
{
  digitalWrite(PINSpeedMotorA, 0);
  digitalWrite(PINSpeedMotorB, 0);
  
  digitalWrite(PINDirA, HIGH);
  digitalWrite(PINDirB, HIGH);

  digitalWrite(PINControlMotor, LOW);
}

void Blink()
{
  digitalWrite(LED_BUILTIN, HIGH);
  delay(1000);
  digitalWrite(LED_BUILTIN, LOW);
  delay(1000);
}