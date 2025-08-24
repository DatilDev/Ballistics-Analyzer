package com.ballisticsanalyzer.mobile;

import android.os.Bundle;
import androidx.appcompat.app.AppCompatActivity;
import android.view.SurfaceView;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("ballistics_mobile");
    }

    private SurfaceView surfaceView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        // Initialize the Rust library
        NativeLib.init(getApplicationContext());
        
        // Create surface for egui rendering
        surfaceView = new SurfaceView(this);
        setContentView(surfaceView);
        
        // Start the Rust app
        NativeLib.startApp(surfaceView.getHolder().getSurface());
    }

    @Override
    protected void onResume() {
        super.onResume();
        NativeLib.resume();
    }

    @Override
    protected void onPause() {
        super.onPause();
        NativeLib.pause();
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        NativeLib.destroy();
    }
}