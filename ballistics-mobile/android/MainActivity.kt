package com.ballistics.analyzer

import android.os.Bundle
import android.view.MotionEvent
import androidx.appcompat.app.AppCompatActivity
import android.opengl.GLSurfaceView
import javax.microedition.khronos.egl.EGLConfig
import javax.microedition.khronos.opengles.GL10

class MainActivity : AppCompatActivity() {
    
    private var appPtr: Long = 0
    private lateinit var glView: GLSurfaceView
    
    companion object {
        init {
            System.loadLibrary("ballistics_mobile")
        }
    }
    
    // Native methods
    private external fun initApp(assetsPath: String): Long
    private external fun runFrame(appPtr: Long)
    private external fun touchEvent(appPtr: Long, x: Float, y: Float, eventType: Int)
    private external fun onResume(appPtr: Long)
    private external fun onPause(appPtr: Long)
    private external fun destroyApp(appPtr: Long)
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Initialize Rust app
        val assetsPath = filesDir.absolutePath
        appPtr = initApp(assetsPath)
        
        // Setup OpenGL view
        glView = GLSurfaceView(this)
        glView.setEGLContextClientVersion(2)
        glView.setRenderer(BallisticsRenderer())
        
        setContentView(glView)
        
        // Initialize services
        LocationService.init(this)
        CameraService.init(this)
        ShareService.init(this)
    }
    
    override fun onTouchEvent(event: MotionEvent): Boolean {
        val eventType = when (event.action) {
            MotionEvent.ACTION_DOWN -> 0
            MotionEvent.ACTION_MOVE -> 1
            MotionEvent.ACTION_UP -> 2
            MotionEvent.ACTION_CANCEL -> 3
            else -> return false
        }
        
        touchEvent(appPtr, event.x, event.y, eventType)
        return true
    }
    
    override fun onResume() {
        super.onResume()
        glView.onResume()
        onResume(appPtr)
    }
    
    override fun onPause() {
        super.onPause()
        glView.onPause()
        onPause(appPtr)
    }
    
    override fun onDestroy() {
        super.onDestroy()
        destroyApp(appPtr)
    }
    
    inner class BallisticsRenderer : GLSurfaceView.Renderer {
        override fun onSurfaceCreated(gl: GL10?, config: EGLConfig?) {
            // Initialize OpenGL
        }
        
        override fun onSurfaceChanged(gl: GL10?, width: Int, height: Int) {
            gl?.glViewport(0, 0, width, height)
        }
        
        override fun onDrawFrame(gl: GL10?) {
            runFrame(appPtr)
        }
    }
}

// Location Service
object LocationService {
    private lateinit var context: MainActivity
    
    fun init(ctx: MainActivity) {
        context = ctx
    }
    
    @JvmStatic
    fun startLocationUpdates() {
        // Implement location updates
    }
    
    @JvmStatic
    fun stopLocationUpdates() {
        // Stop location updates
    }
}

// Camera Service
object CameraService {
    private lateinit var context: MainActivity
    
    fun init(ctx: MainActivity) {
        context = ctx
    }
    
    @JvmStatic
    fun takePhoto() {
        // Launch camera intent
    }
}

// Share Service
object ShareService {
    private lateinit var context: MainActivity
    
    fun init(ctx: MainActivity) {
        context = ctx
    }
    
    @JvmStatic
    fun shareText(text: String, title: String) {
        val intent = android.content.Intent().apply {
            action = android.content.Intent.ACTION_SEND
            type = "text/plain"
            putExtra(android.content.Intent.EXTRA_TEXT, text)
            putExtra(android.content.Intent.EXTRA_TITLE, title)
        }
        context.startActivity(android.content.Intent.createChooser(intent, title))
    }
}