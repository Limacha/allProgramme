#include "PrefixController.h"

namespace prefix
{

    PrefixController::PrefixController(PrefixManager &manager) : prefixManager(manager)
    {
    }

    PrefixController::~PrefixController()
    {
    }

    unsigned char PrefixController::callManager(char *prefix, unsigned short size, char *input)
    {
        managerType manager = nullptr;
        if (prefixManager.getPrefix(prefix, manager))
        {
            return manager(size, input);
        }
        return 0x01;
    }
} // namespace prefix
