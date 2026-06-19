ADB=/home/bob/Android/Sdk/platform-tools/adb
$ADB install build/android/CrossPlatformApp.apk
$ADB logcat -c && $ADB logcat | grep -E "crossplatform|FATAL|AndroidRuntime|dlopen|__real|fopen|main"