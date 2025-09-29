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
import reactor.core.publisher.Flux;
import zone.hwj.vita.NativeTools;
import zone.hwj.vita.api.RequestBodyHandler;

public class RequestHandleEntry {
    public static final MemoryLayout LAYOUT = MemoryLayout.structLayout(
            ValueLayout.ADDRESS.withName("push_handle"),
            ValueLayout.ADDRESS.withName("end_request_handle"),
            ValueLayout.ADDRESS.withName("close_handle")
    ).withName("RequestHandleEntry");

    public static final MemoryLayout ADDRESS_LAYOUT = ValueLayout.ADDRESS.withTargetLayout(LAYOUT);

    private static final VarHandle PUSH_VAR = NativeTools.makeVarHandle(LAYOUT, "push_handle");
    private static final VarHandle END_VAR = NativeTools.makeVarHandle(LAYOUT, "end_request_handle");
    private static final VarHandle CLOSE_VAR = NativeTools.makeVarHandle(LAYOUT, "close_handle");

    private static final MethodHandle START_METHOD;
    private static final MethodHandle PUSH_METHOD;
    private static final MethodHandle END_METHOD;
    private static final MethodHandle CLOSE_METHOD;

    static {
        try {
            Lookup lookup = MethodHandles.lookup();
            START_METHOD = lookup.findStatic(
                    RequestHandleEntry.class, "invokeStart", MethodType.methodType(MemorySegment.class, RequestBodyHandler.class, MemorySegment.class));
            PUSH_METHOD = lookup.findVirtual(
                    RequestHandleEntry.class, "invokePush", MethodType.methodType(void.class, MemorySegment.class));
            END_METHOD = lookup.findVirtual(RequestHandleEntry.class, "invokeEnd", MethodType.methodType(void.class, MemorySegment.class, MemorySegment.class));
            CLOSE_METHOD = lookup.findVirtual(RequestHandleEntry.class, "invokeClose", MethodType.methodType(void.class));
        } catch (NoSuchMethodException | IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }

    private final NativeBufferGroup requestBuffer = NativeBufferGroup.of();

    // 从rust端传入的sender指针，用来在回调中回传给rust端
    private final MemorySegment sender;

    private final RequestBodyHandler bodyHandler;

    private final Arena arena;

    // 在rust端每个请求创建一个实例，不在java端创建实例
    private RequestHandleEntry(MemorySegment sender, RequestBodyHandler bodyHandler) {
        this.sender = sender;
        this.bodyHandler = bodyHandler;
        this.arena = Arena.ofShared();
    }

    public static MemorySegment createStartPtr(Arena globalArena, RequestBodyHandler bodyHandler) {
        FunctionDescriptor fc = FunctionDescriptor.of(ADDRESS_LAYOUT, ValueLayout.ADDRESS);
        MethodHandle cb = MethodHandles.insertArguments(START_METHOD, 0, bodyHandler);
        return NativeTools.makeCallback(globalArena, cb, fc);
    }

    private static MemorySegment invokeStart(RequestBodyHandler bodyHandler, MemorySegment sender) {
        return new RequestHandleEntry(sender, bodyHandler).allocatePtr();
    }

    private void invokePush(MemorySegment chunk) {
        requestBuffer.appendBuffer(new NativeBuffer(arena, chunk));
    }

    private void invokeEnd(MemorySegment responseCall, MemorySegment endResponseCall) {
        Thread.ofVirtual().name("vita-worker-", 0).start(() -> {

            // extern "system" fn send_response_ptr(resp_ptr: *mut JBuffer, send: *mut Sender<*mut JBuffer>)
            MethodHandle responseInvoke = NativeTools.makeMethodHandle(responseCall,
                    FunctionDescriptor.ofVoid(NativeBuffer.ADDRESS_LAYOUT, ValueLayout.ADDRESS));

            // extern "system" fn send_end_response_ptr(send: *mut Sender<*mut JBuffer>)
            MethodHandle endResponseInvoke = NativeTools.makeMethodHandle(endResponseCall,
                    FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

            try {
                Flux<NativeBuffer> responseBody = bodyHandler.apply(this.requestBuffer);
                responseBody.subscribe(buffer -> {
                    try {
                        responseInvoke.invokeExact(buffer.getPtr(), sender);
                    } catch (Throwable e) {
                        throw new RuntimeException(e);
                    }
                }, _ -> {}, () -> {
                    try {
                        endResponseInvoke.invokeExact(sender);
                    } catch (Throwable e) {
                        throw new RuntimeException(e);
                    }
                });
            } catch (Throwable e) {
                throw new IllegalStateException(e);
            }
        });
    }

    private void invokeClose() {
        arena.close();
    }

    private MemorySegment allocatePtr() {
        MemorySegment allocated = arena.allocate(LAYOUT);
        MemorySegment pushPtr = getMethodPtr(PUSH_METHOD, FunctionDescriptor.ofVoid(NativeBuffer.ADDRESS_LAYOUT));
        MemorySegment endPtr = getMethodPtr(END_METHOD, FunctionDescriptor.ofVoid(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
        MemorySegment closePtr = getMethodPtr(CLOSE_METHOD, FunctionDescriptor.ofVoid());
        PUSH_VAR.set(allocated, pushPtr);
        END_VAR.set(allocated, endPtr);
        CLOSE_VAR.set(allocated, closePtr);
        return allocated;
    }

    private MemorySegment getMethodPtr(MethodHandle target, FunctionDescriptor fc) {
        MethodHandle cb = MethodHandles.insertArguments(target, 0, this);
        return NativeTools.makeCallback(arena, cb, fc);
    }
}
