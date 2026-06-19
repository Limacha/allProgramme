clear
$env:JAVA_HOME = "D:\jdk-25.0.3+9"
D:\rust\windows\cargo\bin\cargo.exe clean
D:\rust\windows\cargo\bin\cargo.exe apk build --release --lib -p MyRustLife
copy D:\github\allProgramme\rust\target\release\apk\MyRustyLife.apk D:\github\allProgramme\rust\target\release\apk\MyRustyLife.zip          
tar -tf D:\github\allProgramme\rust\target\release\apk\MyRustyLife.zip  
D:\android\platform-tools\adb.exe install D:\github\allProgramme\rust\target\release\apk\MyRustyLife.apk 
D:\android\platform-tools\adb.exe logcat -c
D:\android\platform-tools\adb.exe shell am start -n com.arflaka.MyRustyLife/android.app.NativeActivity
D:\android\platform-tools\adb.exe logcat -d > crash_full.txt


clear
$env:JAVA_HOME = "D:\jdk-25.0.3+9"
D:\rust\windows\cargo\bin\cargo.exe clean
D:\rust\windows\cargo\bin\cargo.exe apk build --lib -p MyRustLife
copy D:\github\allProgramme\rust\target\debug\apk\MyRustyLife.apk D:\github\allProgramme\rust\target\debug\apk\MyRustyLife.zip          
tar -tf D:\github\allProgramme\rust\target\debug\apk\MyRustyLife.zip  
D:\android\platform-tools\adb.exe install D:\github\allProgramme\rust\target\debug\apk\MyRustyLife.apk 
D:\android\platform-tools\adb.exe logcat -c
D:\android\platform-tools\adb.exe shell am start -n com.arflaka.MyRustyLife/android.app.NativeActivity
D:\android\platform-tools\adb.exe logcat -d > crash_full.txt