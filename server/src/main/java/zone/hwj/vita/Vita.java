package zone.hwj.vita;


import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import zone.hwj.vita.api.Routes;
import zone.hwj.vita.api.Routes.Route;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.def.PathHolder;
import zone.hwj.vita.def.RequestHandleEntry;

public class Vita {

    // pub extern "system" fn register_request_handle(path_buf: *const JBuffer, start_handle: StartHandle)
    private static final MethodHandle registerRequestHandle = NativeTools.makeMethodHandle("register_request_handle", FunctionDescriptor.ofVoid(
            NativeBuffer.ADDRESS_LAYOUT,
            ValueLayout.ADDRESS)
    );

    // pub extern "system" fn start_vita_server()
    private static final MethodHandle startServerHandle = NativeTools.makeMethodHandle("start_vita_server", FunctionDescriptor.ofVoid());

    public static Vita getInstance() {
        final class Holder {
            private static final Vita INSTANCE = new Vita();
        }

        return Holder.INSTANCE;
    }

    private Vita() {}

    public void startServer(Routes routes) {
        try(Arena arena = Arena.ofShared()) {
            for (Route route : routes.routes()) {
                MemorySegment pathPtr = PathHolder.of(arena, route.pattern()).getPtr();
                MemorySegment startPtr = RequestHandleEntry.createStartPtr(arena, route.handler());
                registerRequestHandle.invokeExact(pathPtr, startPtr);
            }

            startServerHandle.invokeExact();
        } catch (Throwable e) {
            throw new IllegalStateException(e);
        }
    }
}
