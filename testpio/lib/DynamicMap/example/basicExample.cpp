#include "../src/DynamicMap.h"

int main()
{
    DynamicMap<const char *, int> map;
    map.put("test", 42);
    map.get("test");
    map.put("test", 95);
    map.get("test");
    map.remove("test");
    map.get("test");
}