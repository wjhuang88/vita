package zone.hwj.vita;

import fr.stardustenterprises.yanl.NativeLoader;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.nio.charset.StandardCharsets;
import zone.hwj.vita.def.NativeString;
import zone.hwj.vita.def.NativeTest;

public class NativeManager {

    private static final MethodHandle helloWorldHandle;
    private static final MethodHandle testAddIntHandle;

    private static final MethodHandle testStructHandle;
    private static final MethodHandle testStructPtrHandle;
    private static final MethodHandle testStructPtrFreeHandle;

    private static final MethodHandle testCreateBytesHandle;
    private static final MethodHandle testCreateStringHandle;
    private static final MethodHandle testCreateStringFreeHandle;

    static {
        new NativeLoader.Builder().build().loadLibrary("vita", false);
        SymbolLookup lookup = SymbolLookup.loaderLookup();
        Linker linker = Linker.nativeLinker();

        helloWorldHandle = linker.downcallHandle(lookup.findOrThrow("hello_world"), FunctionDescriptor.ofVoid());
        testAddIntHandle = linker.downcallHandle(lookup.findOrThrow("test_add_int"), FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT));

        testStructHandle = linker.downcallHandle(lookup.findOrThrow("test_struct"), FunctionDescriptor.of(NativeTest.LAYOUT, ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_DOUBLE));
        testStructPtrHandle = linker.downcallHandle(lookup.findOrThrow("test_struct_pointer"), FunctionDescriptor.of(ValueLayout.ADDRESS.withTargetLayout(NativeTest.LAYOUT), ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_DOUBLE));
        testStructPtrFreeHandle = linker.downcallHandle(lookup.findOrThrow("test_struct_pointer_free"), FunctionDescriptor.ofVoid(ValueLayout.ADDRESS.withTargetLayout(NativeTest.LAYOUT)));

        testCreateBytesHandle = linker.downcallHandle(lookup.findOrThrow("test_create_bytes"), FunctionDescriptor.of(ValueLayout.ADDRESS));
        testCreateStringHandle = linker.downcallHandle(lookup.findOrThrow("test_create_string"), FunctionDescriptor.of(ValueLayout.ADDRESS.withTargetLayout(NativeString.LAYOUT)));
        testCreateStringFreeHandle = linker.downcallHandle(lookup.findOrThrow("test_create_string_free"), FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.JAVA_INT));
    }

    private NativeManager() { }

    public static NativeManager getInstance() {
        final class Holder {
            private static final NativeManager INSTANCE = new NativeManager();
        }

        return Holder.INSTANCE;
    }

    void helloWorld() {
        try {
            helloWorldHandle.invokeExact();
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    int testAddInt(int a, int b) {
        try {
            return (int) testAddIntHandle.invokeExact(a, b);
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    NativeTest testStruct(int a, long b, float c, double d) {
        try(Arena arena = Arena.ofConfined()) {
            MemorySegment structSeg = (MemorySegment) testStructHandle.invokeExact((SegmentAllocator) arena, a, b, c, d);
            return new NativeTest(structSeg);
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }

    NativeTest testStructPtr(int a, long b, float c, double d) {
        try(Arena arena = Arena.ofConfined()) {
            MemorySegment structSeg = ((MemorySegment) testStructPtrHandle.invokeExact(a, b, c, d)).reinterpret(arena, ptr -> {
                try {
                    testStructPtrFreeHandle.invokeExact(ptr);
                } catch (Throwable e) {
                    throw new IllegalStateException(e);
                }
            });
            return new NativeTest(structSeg);
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }

    byte[] testCreateBytes() {
        try(Arena arena = Arena.ofConfined()) {
            MemorySegment seg = ((MemorySegment) testCreateBytesHandle.invokeExact()).reinterpret(16, arena, null);
            byte[] result = new byte[16];
            MemorySegment.copy(seg, ValueLayout.JAVA_BYTE, 0, result, 0, 16);
            return result;
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }

    String testCreateString() {
        try(Arena arena = Arena.ofConfined()) {
            MemorySegment strSeg = ((MemorySegment) testCreateStringHandle.invokeExact()).reinterpret(arena, null);
            NativeString strInst = new NativeString(strSeg, (ptr, size) -> {
                try {
                    testCreateStringFreeHandle.invokeExact(ptr, (int) size);
                } catch (Throwable e) {
                    throw new IllegalStateException(e);
                }
            });
            return new String(strInst.getContent(arena), StandardCharsets.UTF_8);
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }
}
