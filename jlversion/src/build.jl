using AppBundler

import Pkg.BinaryPlatforms: Linux, MacOS, Windows

APP_NAME = "easyjs"

APP_DIR = dirname(@__DIR__)
BUILD_DIR = APP_DIR * "/bin"

println("$BUILD_DIR/$APP_NAME")

AppBundler.bundle_app(MacOS(:x86_64), APP_DIR, "$BUILD_DIR/$APP_NAME-x64.app")
AppBundler.bundle_app(MacOS(:aarch64), APP_DIR, "$BUILD_DIR/$APP_NAME-arm64.app")

AppBundler.bundle_app(Linux(:x86_64), APP_DIR, "$BUILD_DIR/$APP_NAME-x64.snap")
AppBundler.bundle_app(Linux(:aarch64), APP_DIR, "$BUILD_DIR/$APP_NAME-arm64.snap")

AppBundler.bundle_app(Windows(:x86_64), APP_DIR, "$BUILD_DIR/$APP_NAME-win64.zip")