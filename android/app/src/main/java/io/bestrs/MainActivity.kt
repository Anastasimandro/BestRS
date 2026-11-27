package io.bestrs

import android.app.NativeActivity

/**
 * Hosts the Rust entry point. The manifest's `android.app.lib_name` names
 * `libbestrs_android.so`; NativeActivity loads it and calls `android_main`.
 */
class MainActivity : NativeActivity()
