#define _GNU_SOURCE
#include <sys/syscall.h>
#include <sys/stat.h>
#include <linux/fb.h>
#include <linux/input.h>
#include <linux/fcntl.h>
#include <linux/types.h>
#include <linux/ioctl.h>
#include <linux/limits.h>
#include <sys/mman.h>
#include <unistd.h>

// === Dépendances du projet ===
#include "platform.h"
#include "../render.h"
#include "../input.h"

// =============================================================
// === VARIABLES GLOBALES ======================================
// =============================================================

static int fb_fd = -1;
static int input_fd = -1;
static unsigned int *fb_ptr = 0;
static struct fb_var_screeninfo vinfo;
static struct fb_fix_screeninfo finfo;
static unsigned long screensize = 0;

static unsigned int width = 0, height = 0;
static unsigned char running = 1;

// Simple spinlock (pas de pthread)
static volatile int event_lock = 0;

// =============================================================
// === STRUCTURES BAS NIVEAU ===================================
// =============================================================

struct linux_dirent64
{
    unsigned long long d_ino;
    long long d_off;
    unsigned short d_reclen;
    unsigned char d_type;
    char d_name[];
};

// =============================================================
// === SYSCALL WRAPPERS ========================================
// =============================================================

static inline long sys_open(const char *p, int f, int m)
{
    return syscall(SYS_open, p, f, m);
}
static inline long sys_close(int fd)
{
    return syscall(SYS_close, fd);
}
static inline long sys_read(int fd, void *b, unsigned long s)
{
    return syscall(SYS_read, fd, b, s);
}
static inline long sys_write(int fd, const void *b, unsigned long s)
{
    return syscall(SYS_write, fd, b, s);
}
static inline long sys_lseek(int fd, long o, int w)
{
    return syscall(SYS_lseek, fd, o, w);
}
static inline long sys_mmap(void *a, unsigned long l, int p, int f, int fd, long off)
{
    return (long)syscall(SYS_mmap, a, l, p, f, fd, off);
}
static inline long sys_munmap(void *a, unsigned long l)
{
    return syscall(SYS_munmap, a, l);
}
static inline long sys_ioctl(int fd, unsigned long req, void *arg)
{
    return syscall(SYS_ioctl, fd, req, arg);
}
static inline long sys_getcwd(char *buf, unsigned long size)
{
    return syscall(SYS_getcwd, buf, size);
}
static inline long sys_stat(const char *path, struct stat *st)
{
    return syscall(SYS_stat, path, st);
}
static inline long sys_mkdir(const char *path, int mode)
{
    return syscall(SYS_mkdir, path, mode);
}
static inline long sys_getdents64(int fd, void *dirp, unsigned int count)
{
    return syscall(SYS_getdents64, fd, dirp, count);
}

// =============================================================
// === FRAMEBUFFER RENDER ======================================
// =============================================================

void platformInitRender(int w, int h)
{
    (void)w;
    (void)h;

    fb_fd = sys_open("/dev/fb0", O_RDWR, 0);
    if (fb_fd < 0)
        return;

    sys_ioctl(fb_fd, FBIOGET_FSCREENINFO, &finfo);
    sys_ioctl(fb_fd, FBIOGET_VSCREENINFO, &vinfo);

    width = vinfo.xres;
    height = vinfo.yres;
    screensize = finfo.line_length * height;

    fb_ptr = (unsigned int *)sys_mmap(0, screensize,
                                      PROT_READ | PROT_WRITE, MAP_SHARED, fb_fd, 0);
    if ((long)fb_ptr < 0)
    {
        fb_ptr = 0;
        return;
    }

    // efface l’écran
    for (unsigned long i = 0; i < screensize / 4; i++)
        fb_ptr[i] = 0x00000000;
}

void platformRenderFrame(unsigned int *pixels)
{
    if (!fb_ptr || !pixels)
        return;

    for (unsigned int y = 0; y < height; y++)
    {
        unsigned int *dst = (unsigned int *)((char *)fb_ptr + y * finfo.line_length);
        unsigned int *src = pixels + y * width;
        for (unsigned int x = 0; x < width; x++)
            dst[x] = src[x];
    }
}

void platformShutdownRender(void)
{
    if (fb_ptr)
        sys_munmap(fb_ptr, screensize);
    if (fb_fd >= 0)
        sys_close(fb_fd);
}

// =============================================================
// === INPUT (clavier brut /dev/input/eventX) ==================
// =============================================================

void platformInitInput(void)
{
    input_fd = sys_open("/dev/input/event0", O_RDONLY | O_NONBLOCK, 0);
}

void platformProcessInput(void)
{
    if (input_fd < 0)
        return;

    struct input_event ev;
    long n = sys_read(input_fd, &ev, sizeof(ev));
    if (n != sizeof(ev))
        return;

    Event e;
    e.type = EVENT_NONE;

    if (ev.type == EV_KEY)
    {
        e.key.keycode = ev.code;
        e.key.repeat = 0;
        e.key.extended = 0;
        e.key.pressed_before = 0;
        e.key.shift = e.key.ctrl = e.key.alt = e.key.win = 0;
        e.type = (ev.value ? EVENT_KEY_DOWN : EVENT_KEY_UP);
        platformPushEvent(e);
    }
}

// =============================================================
// === FILESYSTEM MINIMAL ======================================
// =============================================================

unsigned short platformGetCurrentPath(char *path)
{
    long len = sys_getcwd(path, MAX_PATH_LENGHT);
    return (len > 0) ? (unsigned short)len : 0;
}

unsigned char platformWriteFile(const char *filename, const char *data, unsigned long size)
{
    int fd = sys_open(filename, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd < 0)
        return 0;
    sys_write(fd, data, size);
    sys_close(fd);
    return 1;
}

unsigned char platformAppendToFile(const char *filename, const char *data)
{
    int fd = sys_open(filename, O_WRONLY | O_CREAT | O_APPEND, 0644);
    if (fd < 0)
        return 0;

    unsigned long len = 0;
    while (data[len])
        len++;

    sys_write(fd, data, len);
    sys_close(fd);
    return 1;
}

void *platformReadFileBinary(const char *path, unsigned long *out_size)
{
    struct stat st;
    if (sys_stat(path, &st) != 0)
        return 0;

    int fd = sys_open(path, O_RDONLY, 0);
    if (fd < 0)
        return 0;

    void *buf = (void *)sys_mmap(0, st.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    *out_size = st.st_size;
    sys_close(fd);
    return buf;
}

void platformFreeFileBinary(void *buffer)
{
    (void)buffer; // rien à faire
}

unsigned char platformFileExists(const char *path)
{
    struct stat st;
    return (sys_stat(path, &st) == 0 && (st.st_mode & S_IFREG));
}

unsigned char platformDirExists(const char *path)
{
    struct stat st;
    return (sys_stat(path, &st) == 0 && (st.st_mode & S_IFDIR));
}

unsigned char platformPathExists(const char *path)
{
    struct stat st;
    return (sys_stat(path, &st) == 0);
}

unsigned char platformCreateDir(const char *path)
{
    return (sys_mkdir(path, 0755) == 0);
}

// =============================================================
// === LISTAGE DE RÉPERTOIRE ===================================
// =============================================================

char **platformListDir(const char *path, unsigned int *outCount)
{
    static char *names[256];
    *outCount = 0;

    int fd = sys_open(path, O_RDONLY | O_DIRECTORY, 0);
    if (fd < 0)
        return names;

    char buf[1024];
    long nread;
    while ((nread = sys_getdents64(fd, buf, sizeof(buf))) > 0)
    {
        int bpos = 0;
        while (bpos < nread)
        {
            struct linux_dirent64 *d = (struct linux_dirent64 *)(buf + bpos);
            if (*outCount < 256)
                names[*outCount] = d->d_name;
            (*outCount)++;
            bpos += d->d_reclen;
        }
    }

    sys_close(fd);
    return names;
}

// =============================================================
// === EVENT QUEUE =============================================
// =============================================================

static void lock(void)
{
    while (__sync_lock_test_and_set(&event_lock, 1))
        ;
}
static void unlock(void)
{
    __sync_lock_release(&event_lock);
}

unsigned char platformPushEvent(Event event)
{
    lock();
    int next = (eventQueue.end + 1) % MAX_EVENTS;
    if (next == eventQueue.start)
        eventQueue.start = (eventQueue.start + 1) % MAX_EVENTS;
    eventQueue.queue[eventQueue.end] = event;
    eventQueue.end = next;
    unlock();
    return 1;
}

unsigned char platformGetEvent(Event *out)
{
    lock();
    if (eventQueue.start == eventQueue.end)
    {
        unlock();
        return 0;
    }
    *out = eventQueue.queue[eventQueue.start];
    eventQueue.start = (eventQueue.start + 1) % MAX_EVENTS;
    unlock();
    return 1;
}

void platformInitEventQueue()
{
    event_lock = 0;
}

// =============================================================
// === DATE / HEURE ============================================
// =============================================================

void platformGetDate(unsigned short *year, unsigned short *month, unsigned short *day)
{
    struct timespec ts;
    syscall(SYS_clock_gettime, 0, &ts);
    if (year)
        *year = 1970;
    if (month)
        *month = 1;
    if (day)
        *day = 1 + ts.tv_sec / 86400;
}

void platformGetTime(unsigned short *hour, unsigned short *minute, unsigned short *second)
{
    struct timespec ts;
    syscall(SYS_clock_gettime, 0, &ts);
    unsigned long t = ts.tv_sec % 86400;
    if (hour)
        *hour = t / 3600;
    if (minute)
        *minute = (t % 3600) / 60;
    if (second)
        *second = t % 60;
}
