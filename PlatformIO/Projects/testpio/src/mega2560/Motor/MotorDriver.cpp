#include "MotorDriver.h"

namespace motor
{
    void MotorDriver::init(unsigned char pinSpeed, unsigned char pinDirection)
    {
        pinMode(pinSpeed, OUTPUT);
        pinMode(pinDirection, OUTPUT);
    }
    void MotorDriver::setSpeed(unsigned char pinSpeed, unsigned char speed)
    {
        analogWrite(pinSpeed, speed);
    }
    void MotorDriver::setDirection(unsigned char pinDirection, bool direction)
    {
        digitalWrite(pinDirection, direction);
    }
    void MotorDriver::reset(unsigned char pinSpeed, unsigned char pinDirection)
    {
        analogWrite(pinSpeed, 0);
        digitalWrite(pinDirection, false);
    }
} // namespace motor
