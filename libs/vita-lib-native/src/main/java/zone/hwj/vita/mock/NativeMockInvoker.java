package zone.hwj.vita.mock;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.nio.charset.StandardCharsets;
import zone.hwj.vita.NativeCommon;

class NativeMockInvoker {

    private static final MethodHandle helloWorldHandle;
    private static final MethodHandle testAddIntHandle;

    private static final MethodHandle testStructHandle;
    private static final MethodHandle testStructPtrHandle;
    private static final MethodHandle testStructPtrFreeHandle;

    private static final MethodHandle testCreateBytesHandle;
    private static final MethodHandle testCreateBytesFreeHandle;

    private static final MethodHandle testCreateStringHandle;
    private static final MethodHandle testCreateStringFreeHandle;
    private static final MethodHandle testCreateStringWrapperFreeHandle;

    static {
        helloWorldHandle = NativeCommon.makeHandle("hello_world", FunctionDescriptor.ofVoid());
        testAddIntHandle = NativeCommon.makeHandle("test_add_int", FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT));

        testStructHandle = NativeCommon.makeHandle("test_struct", FunctionDescriptor.of(NativeTest.LAYOUT, ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_DOUBLE));
        testStructPtrHandle = NativeCommon.makeHandle("test_struct_pointer", FunctionDescriptor.of(ValueLayout.ADDRESS.withTargetLayout(NativeTest.LAYOUT), ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_DOUBLE));
        testStructPtrFreeHandle = NativeCommon.makeHandle("test_struct_pointer_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS.withTargetLayout(NativeTest.LAYOUT)));

        testCreateBytesHandle = NativeCommon.makeHandle("test_create_bytes", FunctionDescriptor.of(ValueLayout.ADDRESS));
        testCreateBytesFreeHandle = NativeCommon.makeHandle("test_create_bytes_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

        testCreateStringHandle = NativeCommon.makeHandle("test_create_string", FunctionDescriptor.of(ValueLayout.ADDRESS.withTargetLayout(
                NativeTestString.LAYOUT)));
        testCreateStringFreeHandle = NativeCommon.makeHandle("test_create_string_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.JAVA_INT));
        testCreateStringWrapperFreeHandle = NativeCommon.makeHandle("test_create_string_wrapper_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
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
            MemorySegment seg = ((MemorySegment) testCreateBytesHandle.invokeExact()).reinterpret(16, arena, ptr -> {
                try {
                    testCreateBytesFreeHandle.invokeExact(ptr);
                } catch (Throwable e) {
                    throw new IllegalStateException(e);
                }
            });
            byte[] result = new byte[16];
            MemorySegment.copy(seg, ValueLayout.JAVA_BYTE, 0, result, 0, 16);
            return result;
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }

    String testCreateString() {
        try(Arena arena = Arena.ofConfined()) {
            MemorySegment strSeg = ((MemorySegment) testCreateStringHandle.invokeExact()).reinterpret(arena, ptr -> {
                try {
                    testCreateStringWrapperFreeHandle.invokeExact(ptr);
                } catch (Throwable e) {
                    throw new IllegalStateException(e);
                }
            });
            NativeTestString strInst = new NativeTestString(strSeg, (ptr, size) -> {
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
