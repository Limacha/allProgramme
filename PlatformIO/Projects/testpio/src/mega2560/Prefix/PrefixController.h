#ifndef PrefixController_H
#define PrefixController_H
#include "PrefixManager.h"
namespace prefix
{
    class PrefixController
    {
    private:
        PrefixManager &prefixManager;

    public:
        PrefixController(PrefixManager &manager);
        ~PrefixController();
        /**
         * @brief appel le manager lier au prefix et execute se qui est demande
         *
         * @param prefix le prefix(3 char) lier au manager a appeller
         * @param size la taille de l'entre sans le prefix et la taille
         * @param input le reste de l'entrer sans le prefix
         *
         * @return 0x00 si aucun probleme, 0x01 si pas de manager trouver, reste en fonction du manager
         */
        unsigned char callManager(char *prefix, unsigned short size, char *input);
    };

} // namespace prefix
#endif