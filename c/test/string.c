#include "string.h"
#include "memory.h"
#include "fonction.h"

String stringCreate(const char *initial)
{
    // set les valeurs en fonctions de l'initial
    String s;
    s.len = strLen(initial);
    s.cap = s.len + 1; // +1 pour le '\0'
    s.data = (char *)memoryMalloc(s.cap);
    if (s.data)
    {
        // copie le contenu
        for (unsigned int i = 0; i < s.len; i++)
            s.data[i] = initial[i];

        // fini la chaine
        s.data[s.len] = '\0';
    }
    return s;
}

void stringFree(String *s)
{
    // libere l'espace
    if (s->data)
        memoryFree(s->data);
    // reset les valeurs
    s->data = 0;
    s->len = 0;
    s->cap = 0;
}

void stringAppend(String *s, const char *text)
{
    unsigned int addLen = strLen(text); // obtien la taille sans \0
    if (addLen == 0)
        return;

    unsigned int newLen = s->len + addLen; // nouvelle taille sans \0

    // Si pas assez de place, réallouer
    if (newLen + 1 > s->cap)
    {
        // set des nouvelle valeur
        unsigned int newCap = newLen + 1;
        char *newData = (char *)memoryMalloc(newCap);
        if (!newData)
            return;

        // Copier l'ancien contenu
        for (unsigned int i = 0; i < s->len; i++)
            newData[i] = s->data[i];

        // Libérer ancien buffer
        memoryFree(s->data);
        s->data = newData;
        s->cap = newCap;
    }

    // Copier le nouveau texte
    for (unsigned int i = 0; i < addLen; i++)
        s->data[s->len + i] = text[i];

    s->len = newLen;
    s->data[s->len] = '\0';
}

String stringAdd(const String *sStart, const String *sAddon)
{
    String result;
    // calcul des nouvelles valeurs
    result.len = sStart->len + sAddon->len;
    result.cap = result.len + 1;
    result.data = (char *)memoryMalloc(result.cap);

    if (result.data)
    {
        // ajoute le string start au result
        unsigned int i = 0;
        for (; i < sStart->len; i++)
        {
            result.data[i] = sStart->data[i];
        }

        // ajoute l'addon au result
        for (unsigned int j = 0; j < sAddon->len; j++)
        {
            result.data[i + j] = sAddon->data[j];
        }
        // fini la chaine
        result.data[result.len] = '\0';
    }

    return result;
}

String stringSlice(const String *s, const unsigned int start, unsigned int end)
{
    // reset des valeurs a 0
    String result;
    result.len = 0;
    result.cap = 0;
    result.data = (void *)0; // null

    // verif reste au moins 1 char apres la coupe
    if (s->len > start + end)
    {
        // set les nouvelles valeurs
        result.len = s->len - start - end;
        result.cap = result.len + 1;
        result.data = (char *)memoryMalloc(result.cap);

        if (result.data)
        {
            // ajoute la tranche
            for (unsigned int i = 0; i < result.len; i++)
                result.data[i] = s->data[i + start];

            // fini la chaine
            result.data[result.len] = '\0';
        }
    }
    return result;
}
