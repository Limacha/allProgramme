#include "Command.h"

namespace command
{
    MouvementControl mouvementControl;
    unsigned char invoke_Demo(unsigned short size, char *input)
    {
        return demoMotor(1, 100);
    }
    unsigned char demoMotor(unsigned char nbDemo, unsigned char speed)
    {
        mouvementControl.direction = MouvementControl::Direction::Backward;
        mouvementControl.MoveByDirection(100, 2, 1000);
        return 0x00;
    }
} // namespace command
