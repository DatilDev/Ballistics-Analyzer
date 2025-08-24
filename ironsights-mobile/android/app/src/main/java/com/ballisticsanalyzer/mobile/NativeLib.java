package com.ballisticsanalyzer.mobile;

import android.content.Context;
import android.view.Surface;

public class NativeLib {
    // Native methods implemented in Rust
    public static native void init(Context context);
    public static native void startApp(Surface surface);
    public static native void resume();
    public static native void pause();
    public static native void destroy();
    public static native void onTouch(float x, float y, int action);
}