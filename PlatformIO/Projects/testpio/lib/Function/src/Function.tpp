template <typename T>
static unsigned short getNumberLength(T value)
{
    unsigned short len = 0;

    // Cas zéro
    if (value == 0)
        return 1;

    // Gestion du signe (uniquement si type signé)
    if (value < 0)
    {
        len++; // pour '-'

        // On convertit en valeur positive "manuellement"
        // ATTENTION : si T est signé, on force vers plus grand
        unsigned long long uval = static_cast<unsigned long long>(-(value + 1)) + 1; //(-(value + 1)) + 1 evite l'overflow

        while (uval > 0)
        {
            uval /= 10;
            len++;
        }
        return len;
    }

    // Partie positive
    unsigned long long uval = static_cast<unsigned long long>(value);
    while (uval > 0)
    {
        uval /= 10;
        len++;
    }

    return len;
}

template <typename sizeType>
static bool compareChar(const char *a, const char *b, sizeType size)
{
    // creation d'un tableau taille -1 si pas unsigned donc error
    // char check[isUnsigned<sizeType>::value ? 1 : -1];
    // (void)check; // éviter warning unused
    static_assert(
        (isUnsigned<sizeType>::value || isSignedNumber<sizeType>::value),
        "compareChar: sizeType must be an unsigned type!");

    for (sizeType i = 0; i < size; i++)
    {
        if (a[i] != b[i])
            return false;
    }
    return true;
}