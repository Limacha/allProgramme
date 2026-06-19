#include "app/app.h"

int main(void)
{

   // #region 1 --- initialisation ---
   // init the app
   app_init();
   // #endregion

   // #region 2 --- main loop ---
   while (app_should_run())
   {
      // catch all event and get the delta-time
      app_begin_frame();

      // update the game
      app_update();

      // create the buffer
      app_draw();

      // show the buffer at screen
      app_end_frame();
   }
   // #endregion

   // #region  3 --- clean ---

   // free the ressource used
   app_close();

   // #endregion

   return 0;
}