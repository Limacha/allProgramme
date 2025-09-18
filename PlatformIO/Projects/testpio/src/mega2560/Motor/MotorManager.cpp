#include "MotorManager.h"
namespace motor
{
    MotorManager::MotorManager()
    {
        motorRight.init();
        motorLeft.init();
        pinMode(PINControlMotor, OUTPUT);
    }

    MotorManager::~MotorManager()
    {
    }

    void MotorManager::ResetMove()
    {
        motorRight.reset();
        motorLeft.reset();

        digitalWrite(PINControlMotor, LOW);
    }

    void MotorManager::Move(bool directionRight, unsigned char speedRight, bool directionLeft, unsigned char speedLeft, unsigned int time)
    {
        // Clamp les vitesses
        auto clampSpeed = [&](unsigned char s) -> unsigned char
        {
            if (s > maxSpeed)
                return maxSpeed;
            if (s < minSpeed && s != 0)
                return minSpeed;
            return s;
        };
        speedRight = clampSpeed(speedRight);
        speedLeft = clampSpeed(speedLeft);

        if (!control)
        {
            ResetMove();
            return;
        }

        digitalWrite(PINControlMotor, HIGH);

        motorRight.setDirection(directionRight);
        motorRight.setSpeed(speedRight);

        motorLeft.setDirection(directionLeft);
        motorLeft.setSpeed(speedLeft);

        if (time > 0)
        {
            delay(time);
            ResetMove();
        }

        return;
    }

    void MotorManager::MoveByDirection(Direction direction, unsigned char speed, unsigned char relation, unsigned int time)
    {
        if (relation == 0)
            relation = 1; // éviter division par zéro

        if (relation < 0)
            relation = -relation;

        switch (direction)
        {
        case Direction::Forward:
            Move(true, speed, true, speed, time);
            break;
        case Direction::Backward:
            Move(false, speed, false, speed, time);
            break;
        case Direction::Left:
            Move(true, speed, false, speed, time);
            break;
        case Direction::Right:
            Move(false, speed, true, speed, time);
            break;
        case Direction::LeftForward:
            Move(true, speed, true, speed / relation, time);
            break;
        case Direction::LeftBackward:
            Move(false, speed, false, speed / relation, time);
            break;
        case Direction::RightForward:
            Move(true, speed, true, speed * relation, time);
            break;
        case Direction::RightBackward:
            Move(false, speed, false, speed * relation, time);
            break;
        case Direction::stop_it:
            ResetMove();
            break;

        default:
            ResetMove();
            break;
        }
    }

    void MotorManager::ChangeSpeed(unsigned char speed, unsigned char motor)
    {
        switch (motor)
        {
        case 0x01:
            motorRight.setSpeed(speed);
            return;
        case 0x02:
            motorLeft.setSpeed(speed);
            return;
        case 0x03:
            motorRight.setSpeed(speed);
            motorLeft.setSpeed(speed);
            return;

        default:
            return;
        }
    }

    void MotorManager::ChangeTankDirection(Direction direction)
    {
        MoveByDirection(direction, motorRight.Speed(), motorRight.Speed() / motorLeft.Speed());
    }

    void MotorManager::ChangeDirection(bool direction, unsigned char motor)
    {
        switch (motor)
        {
        case 0x01:
            motorRight.setSpeed(direction);
            return;
        case 0x02:
            motorLeft.setSpeed(direction);
            return;
        case 0x03:
            motorRight.setSpeed(direction);
            motorLeft.setSpeed(direction);
            return;

        default:
            return;
        }
    }

    unsigned char MotorManager::ManageMotor(unsigned short size, char *input)
    {
        if (size == 0 || input == nullptr)
            return 0x03;

        switch (static_cast<unsigned char>(input[0]))
        {
        case 0x01: // Reset
            ResetMove();
            return 0x00;

        case 0x02: // Move
            if (size >= 5)
            {
                bool directionRight = input[1];
                unsigned char speedRight = static_cast<unsigned char>(input[2]);
                bool directionLeft = input[3];
                unsigned char seepLeft = static_cast<unsigned char>(input[4]);
                unsigned int time = (size >= 6) ? static_cast<unsigned char>(input[5]) : 0;

                Move(directionRight, speedRight, directionLeft, seepLeft, time);
                return 0x00;
            }
            return 0x03;

        case 0x03: // MoveByDirection
            if (size >= 3)
            {
                unsigned int idx = static_cast<unsigned char>(input[1]);
                if (idx >= 0 && idx <= static_cast<unsigned int>(Direction::stop_it))
                {
                    unsigned char speed = static_cast<unsigned char>(input[2]);
                    unsigned char relation = (size >= 4) ? static_cast<unsigned char>(input[3]) : 1;
                    unsigned int time = (size >= 5) ? static_cast<unsigned char>(input[4]) : 0;

                    MoveByDirection(static_cast<Direction>(idx), speed, relation, time);
                    return 0x00;
                }
                return 0x04;
            }
            return 0x03;

        case 0x04: // change speed
            if (size >= 2)
            {
                ChangeSpeed(input[0], input[1]);
                return 0x00;
            }
            return 0x03;

        case 0x05: // change direction
            if (size >= 2)
            {
                ChangeDirection(input[0], input[1]);
                return 0x00;
            }
            return 0x03;

        case 0x06: // change tank direction
            if (size >= 1)
            {
                unsigned int idx = static_cast<unsigned char>(input[1]);
                if (idx >= 0 && idx <= static_cast<unsigned int>(Direction::stop_it))
                {
                    ChangeTankDirection(static_cast<Direction>(idx));
                    return 0x00;
                }
                return 0x04;
            }
            return 0x03;

        default:
            return 0x02;
        }
    }

} // namespace motor