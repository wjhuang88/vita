package zone.hwj.vita;


import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.def.PathHolder;
import zone.hwj.vita.def.RequestHandleEntry;

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
            registerRequestHandle.invokeExact(PathHolder.of(arena, "/123").getPtr(), RequestHandleEntry.createStartPtr(arena));
            startServerHandle.invokeExact();
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }
}
