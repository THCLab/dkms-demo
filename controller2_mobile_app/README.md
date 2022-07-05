# Running

On github this is the folder containing the mobile app in Flutter.

Scan the QR code below to visit a downable `apk` file:

![image](https://user-images.githubusercontent.com/83274413/176196207-3110f5cd-ceb6-456c-bfae-60a6d726ff3a.png)

And install the app on your Android device. Make sure to give it the permission to use camera (it is necessary to scan QR codes inside). Also, make sure you have a screen lock* set up.

* *A `screen lock`is to ensure your phone's security, you can set it up as a password, PIN, or Pattern. That way, even if someone gets their hands on your phone, they won't be able to access it. From Settings, swipe to and tap Lock screen. Then, tap Screen lock type and select your desired type of lock.*

## Flow of the install

*Mind you, the description has been made using an Android Phone Samsung Galaxy A50, Android version 11.*

1. The android device might warn you that there's a security risk when saving an `apk` file from unverivied source on your device. Ignore this, our `apk` file is safe.
2. Look for the recently saved fie in your download directory and tap it to install.
3. You'll get the message (paraphrasing!)

``` For security reasons your device has not granted access```
``` to install unknown apps from this unknown source```

4. Follow the button/ link to the Setting and switch on ```allow from this source```
5. Tap 'back' and now your able to install *dkms_demo*.
6. Open it and CONFIRM the button *Scan* IS THERE underneath the sentence '1. Scan for watcher oobi:', but DON'T USE IT YET.

Platforms DKMS-DEMO has been tested on:

* Android 6 (Successfully installed and opened but stopped working) ❌
* Android 11 (pending) ✔️
* Android 10 ✔️
* Android 9 ✔️

## Compiling

### Finding the right Android Studio version

For installing Android Studio, visit [the official IDE website](https://developer.android.com/android-studio/download) and download the latest version. Then click on the downloaded file (for example .exe on Windows) and follow the on screen instructions for installation.

### How to find, install and configure the right version of Flutter
To install Flutter on your device, please follow the [official tutorial](https://docs.flutter.dev/get-started/install?gclid=CjwKCAjwwo-WBhAMEiwAV4dybVFx2We9iyyzFqm0U0ox1NpsXLkVvVOCUsKO9PyTfiIoZcpaf7z7vBoC_GIQAvD_BwE&gclsrc=aw.ds) provided by Google, as it is the most explicit one. It also contains a description of installing Android Studio and configuring a mobile device to work along. The Dart SDK is provided with Flutter SDK.

### Running the `.dart` file in Android Studio
1. Open the `controller2_mobile_app` directory in Android Studio using `File -> Open` menu.
2. Wait for the files to stop indexing and run `flutter pub get` in Android Studio terminal to get all dependencies. 
3. In the `project` tab on the left side of the screen find `controller2_mobile_app/lib/` folder and open the `main.dart` file
4. Run the `main` function by clicking the two green arrows on the left side of the function name.

## Building apk

1. Open the `controller2_mobile_app` directory in Android Studio using `File -> Open` menu.
2. Run in Android Studio terminal `flutter build apk --split-per-abi`
3. Find the suitable file for your device in `build/app/outputs/flutter-apk`. For most cases it will be `app-arm64-v8a-release`.
