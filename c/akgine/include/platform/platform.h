#ifndef PLATFORM_H
#define PLATFORM_H

#if defined(_WIN32) || defined(_WIN64)
#define OS_WINDOWS
#else
#define OS_LINUX
#endif

#endif
