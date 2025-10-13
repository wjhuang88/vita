package zone.hwj.vita.tools;

import java.lang.foreign.Arena;
import zone.hwj.vita.def.NativeBuffer;

public class SlicedBufferPool { // TODO: 目前是模拟逻辑，需提供真实的池化实现
    NativeBuffer acquire(long size) {
        var dataSeg = Arena.global().allocate(size);
        return NativeBuffer.from(Arena.global(), null, (int) size, dataSeg, false);
    }

    void release(NativeBuffer buffer) {
        // TODO
    }
}
