#include "CommandManager.h"

namespace command
{
    const unsigned int CommandManager::nbCommand = 1;
    const Command CommandManager::commands[nbCommand] = {
        {"dm", 2, invoke_Demo}};

    unsigned char CommandManager::executeCommand(unsigned short size, char *input)
    {
        invokerType func = nullptr;
        if (getInvoker(input, func))
        {
            return func(size, input);
        }
        return 0x02;
    }

    bool CommandManager::getInvoker(const char *prefix, invokerType &outInvoker)
    {
        for (unsigned int i = 0; i < nbCommand; i++)
        {
            if (compareChar(commands[i].prefix, prefix, commands[i].size))
            {
                outInvoker = commands[i].invoker;
                return true;
            }
        }
        return false;
    }

    bool CommandManager::hasCommand(char *prefix)
    {
        for (unsigned int i = 0; i < nbCommand; i++)
        {
            if (compareChar(commands[i].prefix, prefix, commands[i].size))
            {
                return true;
            }
        }
        return false;
    }
} // namespace command