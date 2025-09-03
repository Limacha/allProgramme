#ifndef Function_H
#define Function_H

namespace function
{
    /* #region isUnsigned */

    // Par défaut : non défini -> toute instanciation interdite
    template <typename T>
    struct isUnsigned
    {
        static const bool value = false;
    };

    // Spécialisations explicites uniquement pour les unsigned
    template <>
    struct isUnsigned<unsigned char>
    {
        static const bool value = true;
    };
    template <>
    struct isUnsigned<unsigned short>
    {
        static const bool value = true;
    };
    template <>
    struct isUnsigned<unsigned int>
    {
        static const bool value = true;
    };
    template <>
    struct isUnsigned<unsigned long>
    {
        static const bool value = true;
    };
    template <>
    struct isUnsigned<unsigned long long>
    {
        static const bool value = true;
    };

    /* #endregion */

    /* #region isSignedNumber */

    template <typename T>
    struct isSignedNumber
    {
        static const bool value = false;
    };

    // Spécialisations explicites uniquement pour les signed nuùber
    template <>
    struct isSignedNumber<short>
    {
        static const bool value = true;
    };
    template <>
    struct isSignedNumber<int>
    {
        static const bool value = true;
    };
    template <>
    struct isSignedNumber<long>
    {
        static const bool value = true;
    };
    template <>
    struct isSignedNumber<long long>
    {
        static const bool value = true;
    };

    /* #endregion */

    /**
     * @brief obtient le nombre de char pour ecrire un nombre entier
     *
     * @param value la valeur dui va etre ecrite
     *
     * @return le nombre de char requis (- compris)
     */
    template <typename T>
    static unsigned short getNumberLength(T value);

    template <typename sizeType>
    static bool compareChar(const char *a, const char *b, sizeType size);

#include "Function.tpp" // inclusion de l'implémentation

} // namespace function
#endif