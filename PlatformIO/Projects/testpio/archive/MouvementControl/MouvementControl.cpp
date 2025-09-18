#include "MouvementControl.h"

void MouvementControl::Init()
{
  pinMode(PINSpeedMotorA, OUTPUT);
  pinMode(PINSpeedMotorB, OUTPUT);
  pinMode(PINDirA, OUTPUT);
  pinMode(PINDirB, OUTPUT);
  pinMode(PINControlMotor, OUTPUT);
}

void MouvementControl::Move(bool directionA, unsigned char speedA, bool directionB, unsigned char speedB, unsigned int time)
{
  if (speedA > maxSpeed)
  {
    speedA = maxSpeed;
  }
  if (speedB > maxSpeed)
  {
    speedB = maxSpeed;
  }

  if (speedA < minSpeed && speedA != 0)
  {
    speedA = minSpeed;
  }
  if (speedB < minSpeed && speedB != 0)
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

  if (time > 0)
  {
    delay(time);
    ResetMove();
  }

  return;
}

void MouvementControl::MoveByDirection(unsigned char speed, unsigned char relation, unsigned int time)
{
  switch (direction)
  {
  case MouvementControl::Direction::Forward:
    Move(true, speed, true, speed, time);
    break;
  case MouvementControl::Direction::Backward:
    Move(false, speed, false, speed, time);
    break;
  case MouvementControl::Direction::Left:
    Move(true, speed, false, speed, time);
    break;
  case MouvementControl::Direction::Right:
    Move(false, speed, true, speed, time);
    break;
  case MouvementControl::Direction::LeftForward:
    Move(true, speed, true, speed / relation, time);
    break;
  case MouvementControl::Direction::LeftBackward:
    Move(false, speed, false, speed / relation, time);
    break;
  case MouvementControl::Direction::RightForward:
    Move(true, speed / relation, true, speed, time);
    break;
  case MouvementControl::Direction::RightBackward:
    Move(false, speed / relation, false, speed, time);
    break;
  case MouvementControl::Direction::stop_it:
    ResetMove();
    break;

  default:
    ResetMove();
    break;
  }
}

void MouvementControl::ResetMove()
{
  digitalWrite(PINSpeedMotorA, 0);
  digitalWrite(PINSpeedMotorB, 0);

  digitalWrite(PINDirA, HIGH);
  digitalWrite(PINDirB, HIGH);

  digitalWrite(PINControlMotor, LOW);
}