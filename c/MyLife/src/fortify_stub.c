#include <stdio.h>
/* NDK r27 avec FORTIFY génère des références à __real_fopen.
   On le définit ici comme simple wrapper de fopen. */
FILE *__real_fopen(const char *path, const char *mode)
{
    return fopen(path, mode);
}
FILE *__real_fopen64(const char *path, const char *mode)
{
    return fopen(path, mode);
}