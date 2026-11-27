plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "io.bestrs"
    compileSdk = 36
    // Matches the NDK that CI installs and cargo-ndk uses. Gradle needs it to strip
    // the prebuilt Rust library; the Rust build itself is done by cargo-ndk.
    ndkVersion = "28.2.13676358"

    defaultConfig {
        applicationId = "io.bestrs"
        minSdk = 26
        targetSdk = 36
        versionCode = 20002
        versionName = "2.0.2"
        ndk { abiFilters += listOf("arm64-v8a", "x86_64") }
    }

    buildTypes {
        debug { isDebuggable = true }
        release {
            // The Rust .so is already stripped; there is no Java code to shrink yet.
            isMinifyEnabled = false
            isShrinkResources = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }

    packaging {
        // Required for 16 KB page sizes: native libraries stay uncompressed and aligned.
        jniLibs { useLegacyPackaging = false }
        resources { excludes += "/META-INF/{AL2.0,LGPL2.1}" }
    }
    buildFeatures { buildConfig = true }
}

dependencies {
    // Task 00 needs no AndroidX: MainActivity is the framework's NativeActivity.
    // Phase 4 adds androidx.media for NotificationCompat.MediaStyle.
}
