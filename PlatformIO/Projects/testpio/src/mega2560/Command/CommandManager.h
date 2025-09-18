#ifndef CommandManager_H
#define CommandManager_H

#include <Function.h>

#include "Command.h"

using namespace function;

namespace command
{
    typedef unsigned char (*invokerType)(unsigned short size, char *input);

    // Définir un type commun pour toutes les commandes
    struct Command
    {
        const char *prefix;
        unsigned char size;
        invokerType invoker;
    };

    class CommandManager
    {
    private:
        static const unsigned int nbCommand;
        static const Command commands[1];

    public:
        static bool getInvoker(const char *prefix, invokerType &manager);
        static bool hasCommand(char *prefix);
        static unsigned char executeCommand(unsigned short size, char *input);
    };

} // namespace command
#endif