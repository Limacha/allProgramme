#ifndef Command_H
#define Command_H

#include "mega2560/Motor/MotorManager.h"

namespace command
{
    unsigned char invoke_Demo(unsigned short size, char *input);
    unsigned char demoMotor(unsigned char nbDemo, unsigned char speed);
} // namespace command

#endif