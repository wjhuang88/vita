package zone.hwj.vita.def;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodHandles.Lookup;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.nio.charset.StandardCharsets;
import java.util.UUID;
import zone.hwj.vita.NativeTools;

public class RequestHandleEntry {
    public static final MemoryLayout LAYOUT = MemoryLayout.structLayout(
            ValueLayout.ADDRESS.withName("push_handle"),
            ValueLayout.ADDRESS.withName("end_handle")
    ).withName("RequestHandleEntry");

    public static final MemoryLayout ADDRESS_LAYOUT = ValueLayout.ADDRESS.withTargetLayout(LAYOUT);

    private static final VarHandle PUSH_VAR = NativeTools.makeVarHandle(LAYOUT, "push_handle");
    private static final VarHandle END_VAR = NativeTools.makeVarHandle(LAYOUT, "end_handle");

    private static final MethodHandle START_METHOD;
    private static final MethodHandle PUSH_METHOD;
    private static final MethodHandle END_METHOD;

    static {
        try {
            Lookup lookup = MethodHandles.lookup();
            START_METHOD = lookup.findStatic(
                    RequestHandleEntry.class, "invokeStart", MethodType.methodType(MemorySegment.class, Arena.class, MemorySegment.class));
            PUSH_METHOD = lookup.findVirtual(
                    RequestHandleEntry.class, "invokePush", MethodType.methodType(void.class, Arena.class, MemorySegment.class));
            END_METHOD = lookup.findVirtual(RequestHandleEntry.class, "invokeEnd", MethodType.methodType(void.class, Arena.class, MemorySegment.class));
        } catch (NoSuchMethodException | IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }

    private final String id;
    private final MemorySegment sender;

    // 在rust端每个请求创建一个实例，不在java端创建实例
    private RequestHandleEntry(MemorySegment sender) {
        this.id = UUID.randomUUID().toString();
        this.sender = sender;
    }

    public static MemorySegment createStartPtr(Arena arena) {
        MethodHandle cb = MethodHandles.insertArguments(START_METHOD, 0, arena);
        FunctionDescriptor fc = FunctionDescriptor.of(ADDRESS_LAYOUT, ValueLayout.ADDRESS);
        return NativeTools.makeCallback(arena, cb, fc);
    }

    private static MemorySegment invokeStart(Arena arena, MemorySegment rev) {
        return new RequestHandleEntry(rev).allocatePtr(arena);
    }

    private void invokePush(Arena arena, MemorySegment chunk) {
        System.out.println("Push method invoked: " + id);
    }

    private void invokeEnd(Arena arena, MemorySegment responseCall) {
        MethodHandle responseInvoke = NativeTools.makeMethodHandle(responseCall,
                FunctionDescriptor.ofVoid(NativeBuffer.ADDRESS_LAYOUT, ValueLayout.ADDRESS));
        try {
            byte[] resultBytes = ("test response: " + id).getBytes(StandardCharsets.UTF_8);
            MemorySegment resultSeg = arena.allocateFrom(ValueLayout.JAVA_BYTE, resultBytes);
            MemorySegment response = NativeBuffer.fromParts(arena, null, resultBytes.length, resultSeg, false).getPtr();
            responseInvoke.invokeExact(response, sender);
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
        System.out.println("End method invoked: " + id);
    }

    private MemorySegment allocatePtr(Arena arena) {
        MemorySegment pushPtr = getMethodPtr(arena, PUSH_METHOD, FunctionDescriptor.ofVoid(NativeBuffer.ADDRESS_LAYOUT));
        MemorySegment endPtr = getMethodPtr(arena, END_METHOD, FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
        MemorySegment allocated = arena.allocate(LAYOUT);
        PUSH_VAR.set(allocated, pushPtr);
        END_VAR.set(allocated, endPtr);
        return allocated;
    }

    private MemorySegment getMethodPtr(Arena arena, MethodHandle target, FunctionDescriptor fc) {
        MethodHandle cb = MethodHandles.insertArguments(target, 0, this, arena);
        return NativeTools.makeCallback(arena, cb, fc);
    }
}
