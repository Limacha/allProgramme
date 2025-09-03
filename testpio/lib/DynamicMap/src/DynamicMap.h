#ifndef DynamicMap_H
#define DynamicMap_H

// Comparateur par défaut
template <typename K>
struct DefaultKeyCompare
{
    bool operator()(const K &a, const K &b) const { return a == b; }
};

// Spécialisation pour const char*
template <>
struct DefaultKeyCompare<const char *>
{
    bool operator()(const char *a, const char *b) const
    {
        if (a == 0 || b == 0)
            return false;
        while (*a && *b)
        {
            if (*a != *b)
                return false;
            a++;
            b++;
        }
        return *a == *b;
    }
};

template <typename Key, typename Value, typename KeyCompare = DefaultKeyCompare<Key>>
class DynamicMap
{
    struct KeyVal
    {
        Key key;
        Value value;
    };

public:
    DynamicMap();
    DynamicMap(KeyVal *defaultTab, unsigned int size);
    ~DynamicMap();

    /**
     * @brief ajoute ou redefini la valeur lier a la key fournit
     *
     * @param key la key qui ssera lier a la valeur
     * @param value la valeur a stocker
     *
     * @return si on a reussit a ajouter la valeur
     */
    bool put(const Key key, const Value &value);

    /**
     * @brief retire la key et la valeur lier
     *
     * @param key la key a supprimer
     * @param secure si on change la valeur ou juste detruit le lien default: false
     *
     * @return si on a su retirer la valeur
     */
    bool remove(const Key key, bool secure = false);

    /**
     * @brief obtient la valeur assosier a la key
     *
     * @param key la key lier a la valeur
     * @param outValue la valeur sorti
     *
     * @return true si une valeur est trouver
     */
    bool get(const Key key, Value &outValue);

    /**
     * @brief obtien la capaciter actuel de la map
     *
     * @return renvoie la capaciter de la map
     */
    unsigned int getCapacity() { return capacity; }

    /**
     * @brief obtient le nombre de slot utiliser
     *
     * @return le nombre de slote utiliser
     */
    unsigned int getCount() { return count; }

    /**
     * @brief renvoie si la key existe dans la map
     *
     * @return true si exist
     */
    bool exist(Key key);

private:
    struct Entry
    {
        Key key{};
        Value value{};
        bool used = false;
    };
    Entry *entries = 0;
    unsigned int count = 0;
    unsigned int capacity = 0;
    KeyCompare comp;
};

#include "DynamicMap.tpp"
#endif