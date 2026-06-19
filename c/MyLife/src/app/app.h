#ifndef APP_H
#define APP_H

#include <stdint.h> /* uint8_t, uint32_t, int32_t */
#include "../engine/engine.h"
#include "appStruct.h"
#include "appConst.h"

void app_init(void);
uint8_t app_should_run(void);
void app_begin_frame(void);
void app_update();
void app_draw(void);
void app_end_frame(void);
void app_close(void);
#endif
