# ballistics-mobile/android/app/proguard-rules.pro

# Remove all analytics and tracking code
-assumenosideeffects class android.util.Log {
    public static boolean isLoggable(java.lang.String, int);
    public static int v(...);
    public static int i(...);
    public static int w(...);
    public static int d(...);
    public static int e(...);
}

# Remove any Firebase/Google Analytics if accidentally included
-assumenosideeffects class com.google.firebase.analytics.FirebaseAnalytics {
    *;
}

-assumenosideeffects class com.google.android.gms.analytics.** {
    *;
}

-assumenosideeffects class com.crashlytics.** {
    *;
}

# Remove advertising ID access
-assumenosideeffects class com.google.android.gms.ads.identifier.AdvertisingIdClient {
    *;
}

-assumenosideeffects class com.google.android.gms.ads.identifier.AdvertisingIdClient$Info {
    *;
}

# Keep only essential app classes
-keep class com.ballistics.analyzer.** { *; }
-keep class ballistics_core.** { *; }

# Keep native methods for Rust integration
-keepclasseswithmembernames class * {
    native <methods>;
}

# SQLite for local storage
-keep class org.sqlite.** { *; }
-keep class org.sqlite.database.** { *; }

# Room database for local storage
-keep class androidx.room.** { *; }
-keep @androidx.room.Database class * { *; }
-keep @androidx.room.Entity class * { *; }
-keep @androidx.room.Dao class * { *; }

# Security crypto for local encryption
-keep class androidx.security.crypto.** { *; }

# Remove network tracking
-assumenosideeffects class okhttp3.logging.HttpLoggingInterceptor {
    *;
}

# Strip debug information in release
-dontobfuscate
-optimizations !code/simplification/arithmetic,!field/*,!class/merging/*
-optimizationpasses 5
-allowaccessmodification