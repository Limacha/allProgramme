#ifndef _WIN32
#include "gui.h"
#include <wayland-client.h>
#include <sys/mman.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>

static struct wl_display *display;
static struct wl_compositor *compositor;
static struct wl_registry *registry;
static struct wl_shm *shm;
static struct wl_surface *surface;
static struct wl_shm_pool *pool;
static struct wl_buffer *buffer;

static int width = 800, height = 480;
static void *data;
static int fd;
static size_t size;

static int create_shm_file(size_t size)
{
    char tmpl[] = "/tmp/minigui-XXXXXX";
    int fd = mkstemp(tmpl);
    unlink(tmpl);
    ftruncate(fd, size);
    return fd;
}

static void registry_add(void *data, struct wl_registry *reg, uint32_t id, const char *iface, uint32_t ver)
{
    if (!strcmp(iface, "wl_compositor"))
        compositor = wl_registry_bind(reg, id, &wl_compositor_interface, 1);
    else if (!strcmp(iface, "wl_shm"))
        shm = wl_registry_bind(reg, id, &wl_shm_interface, 1);
}

static const struct wl_registry_listener registry_listener = {
    .global = registry_add,
    .global_remove = NULL};

static void render()
{
    uint32_t *px = data;
    for (int y = 0; y < height; ++y)
        for (int x = 0; x < width; ++x)
            px[y * width + x] = 0xFFCCCCCC;
}

int gui_main_wayland(void)
{
    display = wl_display_connect(NULL);
    if (!display)
        return 1;

    registry = wl_display_get_registry(display);
    wl_registry_add_listener(registry, &registry_listener, NULL);
    wl_display_roundtrip(display);

    surface = wl_compositor_create_surface(compositor);

    int stride = width * 4;
    size = stride * height;
    fd = create_shm_file(size);
    data = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);

    pool = wl_shm_create_pool(shm, fd, size);
    buffer = wl_shm_pool_create_buffer(pool, 0, width, height, stride, WL_SHM_FORMAT_XRGB8888);

    render();

    wl_surface_attach(surface, buffer, 0, 0);
    wl_surface_commit(surface);

    while (wl_display_dispatch(display) != -1)
    {
    }

    return 0;
}
#endif