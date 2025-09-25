package zone.hwj.vita.mock;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SegmentAllocator;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.nio.charset.StandardCharsets;
import zone.hwj.vita.NativeTools;
import zone.hwj.vita.def.NativeBuffer;

class NativeMockInvoker {

    private static final MethodHandle helloWorldHandle;
    private static final MethodHandle testAddIntHandle;

    private static final MethodHandle testStructHandle;
    private static final MethodHandle testStructPtrHandle;
    private static final MethodHandle testStructPtrFreeHandle;

    private static final MethodHandle testCreateBytesHandle;
    private static final MethodHandle testCreateBytesFreeHandle;

    private static final MethodHandle testCreateStringHandle;

    private static final MethodHandle testInvokeCallbackHandle;

    static {
        helloWorldHandle = NativeTools.makeMethodHandle("hello_world", FunctionDescriptor.ofVoid());
        testAddIntHandle = NativeTools.makeMethodHandle("test_add_int", FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT));

        testStructHandle = NativeTools.makeMethodHandle("test_struct", FunctionDescriptor.of(NativeTest.LAYOUT, ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_DOUBLE));
        testStructPtrHandle = NativeTools.makeMethodHandle("test_struct_pointer", FunctionDescriptor.of(ValueLayout.ADDRESS.withTargetLayout(NativeTest.LAYOUT), ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_DOUBLE));
        testStructPtrFreeHandle = NativeTools.makeMethodHandle("test_struct_pointer_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS.withTargetLayout(NativeTest.LAYOUT)));

        testCreateBytesHandle = NativeTools.makeMethodHandle("test_create_bytes", FunctionDescriptor.of(ValueLayout.ADDRESS));
        testCreateBytesFreeHandle = NativeTools.makeMethodHandle("test_create_bytes_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

        testCreateStringHandle = NativeTools.makeMethodHandle("test_create_string", FunctionDescriptor.of(ValueLayout.ADDRESS.withTargetLayout(
                NativeBuffer.LAYOUT), ValueLayout.ADDRESS, ValueLayout.JAVA_INT));

        testInvokeCallbackHandle = NativeTools.makeMethodHandle("test_invoke_callback", FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS));
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

    String testCreateString(String name) {
        try(Arena arena = Arena.ofConfined()) {
            byte[] nameBytes = name.getBytes(StandardCharsets.UTF_8);
            final MemorySegment nameSeg = arena.allocateFrom(ValueLayout.JAVA_BYTE, nameBytes);
            final MemorySegment bufSeg = ((MemorySegment) testCreateStringHandle.invokeExact(nameSeg, nameBytes.length));
            System.out.println("java got: " + bufSeg.address());
            NativeBuffer strInst = new NativeBuffer(arena, bufSeg);
            System.out.println("ID addr in java: " + strInst.getRequestIdPtr().address());
            return new String(strInst.copyContent(), StandardCharsets.UTF_8);
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }

    static class CallbackHolder {

        private String content;

        public boolean testCallback(MemorySegment segment) {
            try(Arena arena = Arena.ofConfined()) {
                NativeBuffer strInst = new NativeBuffer(arena, segment);
                System.out.println("id: " + strInst.getRequestId());
                System.out.println("call back received: " + strInst.getPtr().address());
                content = new String(strInst.copyContent(), StandardCharsets.UTF_8);
            }
            return true;
        }
    }

    String testCallbackInvoke() {
        try {
            CallbackHolder holder = new CallbackHolder();
            MethodHandle target = MethodHandles.lookup().findVirtual(CallbackHolder.class, "testCallback", MethodType.methodType(boolean.class, MemorySegment.class));
            MethodHandle cb = MethodHandles.insertArguments(target, 0, holder);
            FunctionDescriptor fc = FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN, NativeBuffer.ADDRESS_LAYOUT);
            MemorySegment cbPtr = NativeTools.makeCallback(cb, fc);
            boolean result = (boolean) testInvokeCallbackHandle.invokeExact(cbPtr);
            if (result) {
                return holder.content;
            }
            return "Error";
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }
}
