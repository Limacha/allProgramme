#include "memory.h"
#include "platform/platform.h"

void *memoryMalloc(unsigned int size)
{
    return platformMemoryAlloc(size);
}

void memoryFree(void *ptr)
{
    platformMemoryFree(ptr);
}