#include "Command.h"

namespace command
{
    motor::MotorManager motorManager;
    unsigned char invoke_Demo(unsigned short size, char *input)
    {
        return demoMotor(1, 100);
    }
    unsigned char demoMotor(unsigned char nbDemo, unsigned char speed)
    {
        motorManager.Move(true, 100, true, 100, 1000);
        return 0x00;
    }
} // namespace command
