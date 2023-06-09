#if os(iOS)
public typealias NativeApp = IosApplication
#else
public typealias NativeApp = MacosApplication
#endif

// Converts data to a pointer
// Would be a macro, if Swift had macros :|
func ptr<T: AnyObject>(_ data: T) -> UnsafeMutableRawPointer {
    return Unmanaged.passRetained(data).toOpaque()
}

@_cdecl("new_app")
public func newApp() -> UnsafeMutableRawPointer {
    return ptr(NativeApp.init())
}
