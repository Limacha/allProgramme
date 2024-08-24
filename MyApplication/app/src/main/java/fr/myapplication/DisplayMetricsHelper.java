package fr.myapplication;

import android.app.Activity;
import android.util.DisplayMetrics;

import com.bumptech.glide.Glide;
import com.bumptech.glide.load.engine.DiskCacheStrategy;

import java.util.concurrent.CountDownLatch;


public class DisplayMetricsHelper {
    public static int getScreenWidth(Activity activity) {
        DisplayMetrics displayMetrics = new DisplayMetrics();
        if (activity != null) {
            activity.getWindowManager().getDefaultDisplay().getMetrics(displayMetrics);
        }
        return displayMetrics.widthPixels;
    }

    public static int getScreenHeigt(Activity activity) {
        DisplayMetrics displayMetrics = new DisplayMetrics();
        if (activity != null) {
            activity.getWindowManager().getDefaultDisplay().getMetrics(displayMetrics);
        }
        return displayMetrics.heightPixels;
    }

    public static void getImageWidthAsync(Activity activity, OnImageWidthLoadedListener listener, String drawableResourceId) {
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    int width = Glide.with(activity)
                            .asBitmap()
                            .load(drawableResourceId)
                            .diskCacheStrategy(DiskCacheStrategy.ALL)
                            .submit()
                            .get()
                            .getWidth();
                    // Appel du listener avec la largeur de l'image
                    listener.onImageWidthLoaded(width);
                    System.out.println("----------------------------------------------------------------------------width " + width);
                } catch (Exception e) {
                    e.printStackTrace();
                    System.out.println("----------------------------------------------------------------------------err width");
                    listener.onImageWidthLoaded(0);
                }
            }
        }).start();
    }

    // Interface pour recevoir la largeur de l'image chargée
    public interface OnImageWidthLoadedListener {
        void onImageWidthLoaded(int width);
    }

    public static void getImageHeightAsync(Activity activity, OnImageHeightLoadedListener listener, String drawableResourceId) {
        new Thread(new Runnable() {
            @Override
            public void run() {
                try {
                    int height = Glide.with(activity)
                            .asBitmap()
                            .load(drawableResourceId)
                            .diskCacheStrategy(DiskCacheStrategy.ALL)
                            .submit()
                            .get()
                            .getHeight();
                    System.out.println("----------------------------------------------------------------------------Height");
                    // Appel du listener avec la largeur de l'image
                    listener.onImageHeightLoaded(height);
                    System.out.println("----------------------------------------------------------------------------Height " + height);
                } catch (Exception e) {
                    e.printStackTrace();
                    System.out.println("----------------------------------------------------------------------------err Height");
                    listener.onImageHeightLoaded(0);
                }
            }
        }).start();
    }

    // Interface pour recevoir la largeur de l'image chargée
    public interface OnImageHeightLoadedListener {
        void onImageHeightLoaded(int height);
    }

    public static int getHeightSize(Activity activity, String drawableRessourceId) {
        final int[] widthImage = {0};
        final int[] heightImage = {0};
        final int widthScreen;
        final int heightScreen;
        final CountDownLatch latch = new CountDownLatch(2);

        //#region taille image
        // Obtenir la largeur de l'image
        getImageWidthAsync(activity, new OnImageWidthLoadedListener() {
            @Override
            public void onImageWidthLoaded(int w) {
                widthImage[0] = w;
                latch.countDown();
            }
        }, drawableRessourceId);

        // Obtenir la hauteur de l'image
        getImageHeightAsync(activity, new OnImageHeightLoadedListener() {
            @Override
            public void onImageHeightLoaded(int h) {
                heightImage[0] = h;
                latch.countDown();
            }
        }, drawableRessourceId);

        try {
            // Attendre que les deux threads se terminent
            latch.await();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        System.out.println("-----------------------------------------------------Image width" + widthImage[0]);
        System.out.println("-----------------------------------------------------Image height" + heightImage[0]);
        System.out.println("-----------------------------------------------------Image" + (widthImage[0] + heightImage[0]));
        //#endregion

        //#region taille screen
        widthScreen = getScreenWidth(activity);
        heightScreen = getScreenHeigt(activity);
        System.out.println("-----------------------------------------------------Screen width" + widthScreen);
        System.out.println("-----------------------------------------------------Screen height" + heightScreen);
        System.out.println("-----------------------------------------------------Screen" + (widthScreen + heightScreen));
        //#endregion

        //#region calcul
        float newWidth;
        float newHeight;
        float factor;
        float newWidthImage = widthImage[0];
        float newHeightImage = heightImage[0];
        float newWidthScreen = widthScreen;
        float newHeightScreen = heightScreen;

        factor = newWidthScreen / newWidthImage;

        newWidth = newWidthImage * factor;
        newHeight = newHeightImage * factor;

        System.out.println("-----------------------------------------------------factor" + factor);
        System.out.println("-----------------------------------------------------new Screen width" + newWidth);
        System.out.println("-----------------------------------------------------new Screen height" + newHeight);
        //#endregion

        int newHeight1 = (int) newHeight;
        return newHeight1;
    }
}
