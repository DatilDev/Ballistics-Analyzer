// ballistics-mobile/android/app/src/main/java/com/ballistics/analyzer/MainActivity.kt
package com.ballistics.analyzer

import android.os.Bundle
import android.view.MotionEvent
import androidx.appcompat.app.AppCompatActivity
import android.opengl.GLSurfaceView
import javax.microedition.khronos.egl.EGLConfig
import javax.microedition.khronos.opengles.GL10
import android.content.SharedPreferences
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import android.Manifest
import com.karumi.dexter.Dexter
import com.karumi.dexter.MultiplePermissionsReport
import com.karumi.dexter.PermissionToken
import com.karumi.dexter.listener.PermissionRequest
import com.karumi.dexter.listener.multi.MultiplePermissionsListener

class MainActivity : AppCompatActivity() {
    
    private var appPtr: Long = 0
    private lateinit var glView: GLSurfaceView
    private lateinit var localPrefs: SharedPreferences
    private lateinit var encryptedPrefs: SharedPreferences
    
    companion object {
        init {
            System.loadLibrary("ballistics_mobile")
        }
        
        // Privacy constants
        const val PRIVACY_ACCEPTED_KEY = "privacy_policy_accepted"
        const val ANALYTICS_ENABLED_KEY = "analytics_enabled"
        const val DATA_SHARING_ENABLED_KEY = "data_sharing_enabled"
        const val LOCAL_STORAGE_ONLY = true
    }
    
    // Native methods
    private external fun initApp(storagePath: String): Long
    private external fun runFrame(appPtr: Long)
    private external fun touchEvent(appPtr: Long, x: Float, y: Float, eventType: Int)
    private external fun onResume(appPtr: Long)
    private external fun onPause(appPtr: Long)
    private external fun destroyApp(appPtr: Long)
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Initialize encrypted local storage
        initializeLocalStorage()
        
        // Show privacy policy on first launch
        if (!isPrivacyPolicyAccepted()) {
            showPrivacyPolicy()
        }
        
        // Initialize app with local storage path only
        val localStoragePath = getLocalStoragePath()
        appPtr = initApp(localStoragePath)
        
        // Setup OpenGL view
        glView = GLSurfaceView(this)
        glView.setEGLContextClientVersion(2)
        glView.setRenderer(BallisticsRenderer())
        
        setContentView(glView)
        
        // Request necessary permissions with explanation
        requestNecessaryPermissions()
        
        // Initialize local-only services
        LocationService.init(this, localOnly = true)
        CameraService.init(this, localOnly = true)
        ShareService.init(this, localOnly = true)
    }
    
    private fun initializeLocalStorage() {
        // Create encrypted shared preferences for sensitive data
        val masterKey = MasterKey.Builder(this)
            .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
            .build()
        
        encryptedPrefs = EncryptedSharedPreferences.create(
            this,
            "ballistics_secure_prefs",
            masterKey,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
        )
        
        // Regular preferences for non-sensitive settings
        localPrefs = getSharedPreferences("ballistics_prefs", MODE_PRIVATE)
    }
    
    private fun getLocalStoragePath(): String {
        // Use internal storage - completely private to the app
        return filesDir.absolutePath
    }
    
    private fun isPrivacyPolicyAccepted(): Boolean {
        return localPrefs.getBoolean(PRIVACY_ACCEPTED_KEY, false)
    }
    
    private fun showPrivacyPolicy() {
        AlertDialog.Builder(this)
            .setTitle("Privacy Policy")
            .setMessage("""
                Ballistics Analyzer - Privacy First
                
                Your Privacy is Protected:
                • All data is stored locally on your device
                • No analytics or tracking by default
                • No data is sent to any servers
                • Network access is only for optional Nostr sharing
                • You have complete control over your data
                
                Optional Features:
                • Location: Used only for environmental calculations
                • Camera: Used only to attach photos to calculations
                • Bluetooth: Used only for hardware device connections
                
                We never collect:
                • Personal information
                • Usage statistics
                • Device identifiers
                • Advertising IDs
                
                By using this app, you acknowledge that all data remains on your device.
            """.trimIndent())
            .setPositiveButton("I Agree") { _, _ ->
                localPrefs.edit()
                    .putBoolean(PRIVACY_ACCEPTED_KEY, true)
                    .putBoolean(ANALYTICS_ENABLED_KEY, false) // Disabled by default
                    .putBoolean(DATA_SHARING_ENABLED_KEY, false) // Disabled by default
                    .apply()
            }
            .setNegativeButton("Exit") { _, _ ->
                finish()
            }
            .setCancelable(false)
            .show()
    }
    
    private fun requestNecessaryPermissions() {
        Dexter.withContext(this)
            .withPermissions(
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.CAMERA,
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.BLUETOOTH_SCAN
            )
            .withListener(object : MultiplePermissionsListener {
                override fun onPermissionsChecked(report: MultiplePermissionsReport) {
                    // All permissions are optional - app works without them
                    if (report.areAllPermissionsGranted()) {
                        showToast("Permissions granted for enhanced features")
                    } else {
                        showToast("App will work with limited features")
                    }
                }
                
                override fun onPermissionRationaleShouldBeShown(
                    permissions: List<PermissionRequest>,
                    token: PermissionToken
                ) {
                    AlertDialog.Builder(this@MainActivity)
                        .setTitle("Optional Permissions")
                        .setMessage("""
                            These permissions enhance app functionality:
                            
                            • Location: Environmental data for calculations
                            • Camera: Attach photos to calculations
                            • Bluetooth: Connect to rangefinders/weather meters
                            
                            All features work offline. No data leaves your device.
                        """.trimIndent())
                        .setPositiveButton("Grant") { _, _ -> token.continuePermissionRequest() }
                        .setNegativeButton("Skip") { _, _ -> token.cancelPermissionRequest() }
                        .show()
                }
            })
            .check()
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
        
        // Save state locally
        saveLocalState()
    }
    
    override fun onDestroy() {
        super.onDestroy()
        saveLocalState()
        destroyApp(appPtr)
    }
    
    private fun saveLocalState() {
        // All data saved locally - nothing sent to servers
        // State is automatically saved by the Rust library to local SQLite
    }
    
    private fun showToast(message: String) {
        Toast.makeText(this, message, Toast.LENGTH_SHORT).show()
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

// Local-only Location Service
object LocationService {
    private lateinit var context: MainActivity
    private var localOnly: Boolean = true
    
    fun init(ctx: MainActivity, localOnly: Boolean = true) {
        context = ctx
        this.localOnly = localOnly
    }
    
    @JvmStatic
    fun startLocationUpdates() {
        // Location data stays on device - never transmitted
        // Used only for environmental calculations
    }
    
    @JvmStatic
    fun stopLocationUpdates() {
        // Stop location updates
    }
}

// Local-only Camera Service
object CameraService {
    private lateinit var context: MainActivity
    private var localOnly: Boolean = true
    
    fun init(ctx: MainActivity, localOnly: Boolean = true) {
        context = ctx
        this.localOnly = localOnly
    }
    
    @JvmStatic
    fun takePhoto() {
        // Photos saved to internal storage only
        // Never uploaded or transmitted
    }
}

// Local-only Share Service
object ShareService {
    private lateinit var context: MainActivity
    private var localOnly: Boolean = true
    
    fun init(ctx: MainActivity, localOnly: Boolean = true) {
        context = ctx
        this.localOnly = localOnly
    }
    
    @JvmStatic
    fun shareText(text: String, title: String) {
        // Share via Android's built-in share sheet
        // User chooses where to share - no automatic uploads
        val intent = android.content.Intent().apply {
            action = android.content.Intent.ACTION_SEND
            type = "text/plain"
            putExtra(android.content.Intent.EXTRA_TEXT, text)
            putExtra(android.content.Intent.EXTRA_TITLE, title)
        }
        context.startActivity(android.content.Intent.createChooser(intent, title))
    }
}