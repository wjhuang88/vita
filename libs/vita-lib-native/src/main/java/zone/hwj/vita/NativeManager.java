package zone.hwj.vita;


import java.io.IOException;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.nio.charset.StandardCharsets;
import reactor.core.publisher.Flux;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.def.PathHolder;
import zone.hwj.vita.def.RequestHandleEntry;
import zone.hwj.vita.tools.IoUtils;

public class NativeManager {

    private static final MethodHandle registerRequestHandle = NativeTools.makeMethodHandle("register_request_handle", FunctionDescriptor.ofVoid(
            NativeBuffer.ADDRESS_LAYOUT,
            ValueLayout.ADDRESS)
    );

    private static final MethodHandle startServerHandle = NativeTools.makeMethodHandle("start_vita_server", FunctionDescriptor.ofVoid());

    public static NativeManager getInstance() {
        final class Holder {
            private static final NativeManager INSTANCE = new NativeManager();
        }

        return Holder.INSTANCE;
    }

    private NativeManager() {}

    public void startServer() {
        try(Arena arena = Arena.ofShared()) {
            registerRequestHandle.invokeExact(PathHolder.of(arena, "/123").getPtr(), RequestHandleEntry.createStartPtr(arena, is -> Flux.create(sink -> {
                try {
                    String resultStr = IoUtils.toUtf8String(is) + ", form java";
                    byte[] resultBytes = resultStr.getBytes(StandardCharsets.UTF_8);
                    MemorySegment resultSeg = arena.allocateFrom(ValueLayout.JAVA_BYTE, resultBytes);
                    NativeBuffer response = NativeBuffer.fromParts(arena, null, resultBytes.length, resultSeg, false);
                    sink.next(response);
                    sink.next(response);
                    sink.next(response);
                    sink.next(response);
                } catch (IOException e) {
                    sink.error(e);
                    return;
                }
                sink.complete();
            })));

            startServerHandle.invokeExact();
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }
}
