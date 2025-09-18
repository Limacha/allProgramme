#include "Motor.h"

namespace motor
{
    Motor::Motor(unsigned char pinSpeed, unsigned char pinDirection) : pinSpeed(pinSpeed), pinDirection(pinDirection)
    {
    }

    Motor::~Motor()
    {
    }

    void Motor::init()
    {
        MotorDriver::init(pinSpeed, pinDirection);
    }

    void Motor::setSpeed(unsigned char speed)
    {
        this->speed = speed;
        MotorDriver::setSpeed(pinSpeed, speed);
    }

    void Motor::setDirection(bool direction)
    {
        this->direction = direction;
        MotorDriver::setDirection(pinDirection, direction);
    }

    void Motor::reset()
    {
        this->speed = 0;
        this->direction = false;
        MotorDriver::reset(pinSpeed, pinDirection);
    }
} // namespace motor
