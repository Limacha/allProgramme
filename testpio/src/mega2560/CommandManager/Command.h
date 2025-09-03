#ifndef Command_H
#define Command_H

#include "mega2560/MouvementControl/MouvementControl.h"

namespace command
{
    unsigned char invoke_Demo(unsigned short size, char *input);
    unsigned char demoMotor(unsigned char nbDemo, unsigned char speed);
} // namespace command

#endif