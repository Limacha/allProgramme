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
public:
    DynamicMap();
    ~DynamicMap() { delete[] entries; }

    bool put(const Key &key, const Value &value);
    bool remove(const Key &key, bool secure = false);
    Value get(const Key &key, const Value &defaultValue = Value());
    int getCapacity() { return capacity; }
    int getCount() { return count; }

private:
    struct Entry
    {
        Key key{};
        Value value{};
        bool used = false;
    };
    Entry *entries = 0;
    int count = 0;
    int capacity = 0;
    KeyCompare comp;
};

#include "DynamicMap.tpp"
#endif